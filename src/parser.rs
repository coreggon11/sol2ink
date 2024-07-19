// MIT License

// Copyright (c) 2022 Supercolony

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use crate::structures::*;
use rbtree::RBTree;
use solang_parser::{
    parse,
    pt::{
        ContractDefinition,
        ContractPart,
        ContractTy,
        Expression as SolangExpression,
        FunctionAttribute,
        FunctionDefinition,
        FunctionTy,
        Identifier,
        IdentifierPath,
        Mutability,
        SourceUnitPart,
        Statement as SolangStatement,
        VariableAttribute,
        VariableDefinition,
        Visibility,
    },
};
use std::collections::{
    HashMap,
    HashSet,
};

#[derive(Clone, Debug)]
pub enum ParserOutput {
    Contract(String, Contract),
    Interface(String, Interface),
    Library(String, Library),
    None,
}

#[derive(Debug, Eq, PartialEq)]
pub enum ParserError {
    FileError(String),
    FileCorrupted(Vec<String>),

    ContractNameNotFound,
    StructNameNotFound,
    EventNameNotFound,
    VariableNameNotFound,
    EnumValueNotDefined,

    IncorrectTypeOfVariable,
}

impl From<std::io::Error> for ParserError {
    fn from(error: std::io::Error) -> Self {
        ParserError::FileError(error.to_string())
    }
}

pub struct Parser<'a> {
    members_map: &'a mut HashMap<String, MemberType>,
    modifiers_map: &'a mut HashMap<String, FunctionDefinition>,
    imports: &'a mut HashSet<Import>,
    comments: &'a mut RBTree<usize, String>,
}

impl<'a> Parser<'a> {
    pub fn new(
        members_map: &'a mut HashMap<String, MemberType>,
        modifiers_map: &'a mut HashMap<String, FunctionDefinition>,
        imports: &'a mut HashSet<Import>,
        comments: &'a mut RBTree<usize, String>,
    ) -> Self {
        Parser {
            members_map,
            modifiers_map,
            imports,
            comments,
        }
    }

    pub fn clear(&mut self) {
        self.members_map.clear();
        self.modifiers_map.clear();
        self.imports.clear();
        self.comments.clear();
    }

    /// Parses a fil and returns the vec of ParserOutput or a ParserError
    ///
    /// `content` the content of a solidity file
    pub fn parse_file(&mut self, content: &str) -> Result<Vec<ParserOutput>, ParserError> {
        let token_tree = parse(content, 0).map_err(|errors| {
            ParserError::FileCorrupted(errors.iter().map(|error| error.message.clone()).collect())
        })?;

        let mut output = Vec::new();
        let source_unit = token_tree.0;

        for source_unit_part in source_unit.0.iter() {
            match &source_unit_part {
                SourceUnitPart::ContractDefinition(contract) => {
                    output.push(self.handle_contract_definition(contract)?);
                }
                _ => (),
            }
        }

        Ok(output)
    }

    /// Parses a contract definition and returns a ParserOutput
    ///
    /// `contract_definition` the Solang contract definition
    ///
    /// Returns `ParserOutput::Contract` if we are parsing a contract
    /// Returns `ParserOutput::Library` if we are parsing a libraray
    /// Returns `ParserOutput::Interface` if we are parsing an interface
    fn handle_contract_definition(
        &mut self,
        contract_definition: &ContractDefinition,
    ) -> Result<ParserOutput, ParserError> {
        match contract_definition.ty {
            ContractTy::Abstract(_) | ContractTy::Contract(_) => {
                let parsed_contract = self.parse_contract(contract_definition)?;
                let contract =
                    ParserOutput::Contract(parsed_contract.name.clone(), parsed_contract);
                Ok(contract)
            }
            ContractTy::Library(_) => {
                let parsed_library = self.parse_library(contract_definition)?;
                let library = ParserOutput::Library(parsed_library.name.clone(), parsed_library);
                Ok(library)
            }
            ContractTy::Interface(_) => {
                let parsed_trait = self.parse_interface(contract_definition)?;
                let interface = ParserOutput::Interface(parsed_trait.name.clone(), parsed_trait);
                Ok(interface)
            }
        }
    }

    /// Parses a contract
    ///
    /// `contract_definition` the Solang contract definition
    /// `comments` the documentation of the contract
    ///
    /// Returns the parsed contract
    fn parse_contract(
        &mut self,
        contract_definition: &ContractDefinition,
    ) -> Result<Contract, ParserError> {
        let name = self.parse_identifier(&contract_definition.name);
        let base = contract_definition
            .base
            .iter()
            .map(|base| self.parse_identifier_path(&base.name))
            .collect();

        let mut fields: Vec<ContractField> = Default::default();
        let mut functions: Vec<Function> = Default::default();
        let mut constructor: Function = Default::default();
        let mut modifiers: Vec<Function> = Default::default();

        // first we need to know functions and storage fields that exist
        for part in contract_definition.parts.iter() {
            match part {
                ContractPart::VariableDefinition(variable_definition) => {
                    let name = self.parse_identifier(&variable_definition.name);
                    if variable_definition.attrs.iter().any(|item| {
                        matches!(
                            item,
                            VariableAttribute::Constant(_) | VariableAttribute::Immutable(_)
                        )
                    }) {
                        // we do not care about consants and immutables as they do not change state of the contract so we skip
                        // @todo we do care about them as they could hide the type of contract we want to call later
                        continue
                    }
                    self.members_map
                        .insert(name.clone(), MemberType::StorageField(name.clone()));
                }
                ContractPart::FunctionDefinition(function_definition) => {
                    let fn_name = self.parse_identifier(&function_definition.name);
                    match function_definition.ty {
                        FunctionTy::Function => {
                            let function_header = self.parse_function_header(function_definition);
                            self.members_map.insert(
                                fn_name.clone(),
                                MemberType::Function(function_header, name.clone()),
                            );
                        }
                        FunctionTy::Modifier => {
                            self.modifiers_map
                                .insert(fn_name.clone(), *function_definition.clone());
                        }
                        _ => (),
                    }
                }
                _ => (),
            }
        }

        for part in contract_definition.parts.iter() {
            match part {
                ContractPart::VariableDefinition(variable_definition) => {
                    if variable_definition.attrs.iter().any(|item| {
                        matches!(
                            item,
                            VariableAttribute::Constant(_) | VariableAttribute::Immutable(_)
                        )
                    }) {
                        // we do not care about consants and immutables as they do not change state of the contract so we skip
                        continue
                    }
                    let parsed_field = self.parse_storage_field(variable_definition)?;
                    fields.push(parsed_field);
                }
                ContractPart::FunctionDefinition(function_definition) => {
                    let parsed_function = self.parse_function(function_definition)?;
                    match function_definition.ty {
                        FunctionTy::Constructor => constructor = parsed_function,
                        FunctionTy::Modifier => modifiers.push(parsed_function),
                        _ => functions.push(parsed_function),
                    }
                }
                _ => {}
            }
        }

        Ok(Contract {
            name,
            fields,
            functions,
            constructor,
            modifiers,
            base,
            is_abstract: match contract_definition.ty {
                ContractTy::Abstract(_) => true,
                _ => false,
            },
        })
    }

    /// Parses an interface
    ///
    /// `contract_definition` the Solang interface definition
    /// `comments` the documentation of the interface
    ///
    /// Returns the parsed interface as an ink! trait definition
    fn parse_interface(
        &mut self,
        contract_definition: &ContractDefinition,
    ) -> Result<Interface, ParserError> {
        let name = self.parse_identifier(&contract_definition.name);

        let mut function_headers: Vec<FunctionHeader> = Default::default();

        for part in contract_definition.parts.iter() {
            match part {
                ContractPart::FunctionDefinition(function_definition) => {
                    if function_definition.ty == FunctionTy::Function {
                        let header = self.parse_function_header(function_definition);
                        function_headers.push(header);
                    }
                }
                _ => {}
            }
        }

        Ok(Interface {
            name,
            function_headers,
            imports: self.imports.clone(),
        })
    }

    /// Parses a library
    ///
    /// `contract_definition` the Solang library definition
    /// `comments` the documentation of the library
    ///
    /// Returns the parsed library as a plain rust file
    fn parse_library(
        &mut self,
        contract_definition: &ContractDefinition,
    ) -> Result<Library, ParserError> {
        let name = self.parse_identifier(&contract_definition.name);

        let mut functions: Vec<Function> = Default::default();

        // first we register all members of the contract
        for part in contract_definition.parts.iter() {
            if let ContractPart::FunctionDefinition(function_definition) = part {
                let fn_name = self.parse_identifier(&function_definition.name);
                match function_definition.ty {
                    FunctionTy::Function => {
                        let function_header = self.parse_function_header(function_definition);
                        self.members_map.insert(
                            fn_name.clone(),
                            MemberType::Function(function_header, name.clone()),
                        );
                    }
                    FunctionTy::Modifier => {
                        self.modifiers_map
                            .insert(fn_name.clone(), *function_definition.clone());
                    }
                    _ => (),
                }
            }
        }

        for part in contract_definition.parts.iter() {
            match part {
                ContractPart::FunctionDefinition(function_definition) => {
                    if function_definition.ty == FunctionTy::Function {
                        let parsed_function = self.parse_function(function_definition)?;
                        functions.push(parsed_function)
                    }
                }
                _ => {}
            }
        }

        Ok(Library {
            name,
            functions,
            imports: self.imports.clone(),
        })
    }

    /// Parses a Solang storage variable definition to Sol2Ink contract field definition
    ///
    /// `variable_definition` the Solang variable definition
    ///
    /// Returns the parsed `ContractField` struct
    fn parse_storage_field(
        &mut self,
        variable_definition: &VariableDefinition,
    ) -> Result<ContractField, ParserError> {
        let name = self.parse_identifier(&variable_definition.name);
        let contract_field = ContractField { name };

        Ok(contract_field)
    }

    /// Parses a Solang function definition to Sol2Ink function definition
    ///
    /// `function_definition` the Solang function definition
    ///
    /// Returns the parsed `Function` struct
    fn parse_function(
        &mut self,
        function_definition: &FunctionDefinition,
    ) -> Result<Function, ParserError> {
        let header = self.parse_function_header(function_definition);
        let calls = if let Some(statement) = &function_definition.body {
            self.parse_statement(statement)?
        } else {
            Vec::default()
        };

        Ok(Function { header, calls })
    }

    /// Parses a Sol2Ink function header definition from Solang function definition
    ///
    /// `function_definition` the Solang function definition
    ///
    /// Returns the parsed `FunctionHeader` struct
    fn parse_function_header(
        &mut self,
        function_definition: &FunctionDefinition,
    ) -> FunctionHeader {
        let name = self.parse_identifier(&function_definition.name);
        let modifiers: Vec<Expression> = function_definition
            .attributes
            .iter()
            .filter(|&attribute| matches!(attribute, FunctionAttribute::BaseOrModifier(..)))
            .map(|modifier| {
                if let FunctionAttribute::BaseOrModifier(_, base) = modifier {
                    let parsed_name = self.parse_identifier_path(&base.name);
                    let parsed_args = if let Some(args) = &base.args {
                        self.parse_expression_vec(args)
                    } else {
                        Vec::default()
                    };

                    Expression::Modifier(parsed_name, parsed_args)
                } else {
                    unreachable!("The vec was filtered before");
                }
            })
            .collect();
        let external = function_definition.attributes.iter().any(|attribute| {
            matches!(
                attribute,
                FunctionAttribute::Visibility(Visibility::External(_))
                    | FunctionAttribute::Visibility(Visibility::Public(_))
            )
        });
        let view = function_definition.attributes.iter().any(|attribute| {
            matches!(
                attribute,
                FunctionAttribute::Mutability(Mutability::Pure(_))
                    | FunctionAttribute::Mutability(Mutability::View(_))
            )
        });
        let payable = function_definition.attributes.iter().any(|attribute| {
            matches!(
                attribute,
                FunctionAttribute::Mutability(Mutability::Payable(_))
            )
        });

        FunctionHeader {
            name,
            external,
            view,
            payable,
            modifiers,
        }
    }

    /// Parses a Solang statement enum variant to Sol2Ink statement enum variant
    ///
    /// `statement` the original Solang statement enum variant
    /// `location` the location where the statement is [being called](fn@parse_variable_access_location)
    ///
    /// Returns the parsed `Statement` enum variant
    fn parse_statement(&mut self, statement: &SolangStatement) -> Result<Vec<Call>, ParserError> {
        Ok(match statement {
            SolangStatement::Block {
                loc: _,
                unchecked: _,
                statements,
            } => {
                statements
                    .iter()
                    .flat_map(|statement| self.parse_statement(statement).unwrap_or_default())
                    .collect::<Vec<_>>()
            }
            SolangStatement::Assembly {
                loc: _,
                dialect: _,
                flags: _,
                block: _,
            } => todo!("Assembly not done yet!"),
            SolangStatement::If(_, expression, if_true, if_false) => {
                let mut parsed_expression = self.parse_expression(expression);
                let parsed_if_true = self.parse_statement(if_true)?;
                let parsed_if_false = if_false
                    .as_ref()
                    .map(|statement| self.parse_statement(statement).unwrap_or_default())
                    .unwrap_or_default();

                parsed_expression.extend(parsed_if_true);
                parsed_expression.extend(parsed_if_false);

                parsed_expression
            }
            SolangStatement::While(_, expression, statement) => {
                let mut parsed_expression = self.parse_expression(expression);
                let parsed_statement = self.parse_statement(statement)?;

                parsed_expression.extend(parsed_statement);

                parsed_expression
            }
            SolangStatement::Expression(_, expression) => self.parse_expression(expression),
            SolangStatement::VariableDefinition(_, _, initial_value_maybe) => {
                initial_value_maybe
                    .as_ref()
                    .map(|expression| self.parse_expression(expression))
                    .unwrap_or_default()
            }
            SolangStatement::For(_, variable_definition, condition, on_pass, body) => {
                let mut parsed_variable_definition = variable_definition
                    .as_ref()
                    .map(|statement| self.parse_statement(statement).unwrap_or_default())
                    .unwrap_or_default();
                let parsed_condition = condition
                    .as_ref()
                    .map(|expression| self.parse_expression(expression))
                    .unwrap_or_default();
                let parsed_on_pass = on_pass
                    .as_ref()
                    .map(|statement| self.parse_statement(statement))
                    .map(|result| result.unwrap())
                    .unwrap_or_default();
                let parsed_body = body
                    .as_ref()
                    .map(|statement| self.parse_statement(statement))
                    .map(|result| result.unwrap())
                    .unwrap_or_default();

                parsed_variable_definition.extend(parsed_condition);
                parsed_variable_definition.extend(parsed_on_pass);
                parsed_variable_definition.extend(parsed_body);

                parsed_variable_definition
            }
            SolangStatement::DoWhile(_, body, condition) => {
                let mut parsed_condition = self.parse_expression(condition);
                let parsed_body = self.parse_statement(body)?;

                parsed_condition.extend(parsed_body);

                parsed_condition
            }
            SolangStatement::Return(_, expression) => {
                expression
                    .as_ref()
                    .map(|expression| self.parse_expression(expression))
                    .unwrap_or_default()
            }
            SolangStatement::Try(_, expression, _, _) => self.parse_expression(expression),
            _ => Vec::default(),
        })
    }

    /// Parses a Solang expression enum variant to Sol2Ink expression enum variant
    ///
    /// `expression` the original Solang expression enum variant
    /// `location` the location where the expression is [being called](fn@parse_variable_access_location)
    ///
    /// Returns the parsed `Expression` enum variant
    fn parse_expression(&mut self, expression: &SolangExpression) -> Vec<Call> {
        macro_rules! maybe_boxed_expression {
            ($to_declare:ident,$to_parse:expr) => {
                $to_parse
                    .as_ref()
                    .map(|expression| self.parse_expression(&expression))
            };
        }

        macro_rules! boxed_expression {
            ($to_declare:ident,$to_parse:expr) => {
                self.parse_expression($to_parse)
            };
        }

        match expression {
            SolangExpression::PostIncrement(_, expression)
            | SolangExpression::PostDecrement(_, expression)
            | SolangExpression::New(_, expression)
            | SolangExpression::Parenthesis(_, expression)
            | SolangExpression::Not(_, expression)
            | SolangExpression::Complement(_, expression)
            | SolangExpression::PreIncrement(_, expression)
            | SolangExpression::PreDecrement(_, expression)
            | SolangExpression::UnaryPlus(_, expression)
            | SolangExpression::UnaryMinus(_, expression)
            | SolangExpression::Delete(_, expression) => {
                boxed_expression!(parsed_expression, expression)
            }

            SolangExpression::Power(_, left, right)
            | SolangExpression::Multiply(_, left, right)
            | SolangExpression::Divide(_, left, right)
            | SolangExpression::Modulo(_, left, right)
            | SolangExpression::Add(_, left, right)
            | SolangExpression::Subtract(_, left, right)
            | SolangExpression::ShiftLeft(_, left, right)
            | SolangExpression::ShiftRight(_, left, right)
            | SolangExpression::BitwiseAnd(_, left, right)
            | SolangExpression::BitwiseXor(_, left, right)
            | SolangExpression::BitwiseOr(_, left, right)
            | SolangExpression::Less(_, left, right)
            | SolangExpression::More(_, left, right)
            | SolangExpression::LessEqual(_, left, right)
            | SolangExpression::MoreEqual(_, left, right)
            | SolangExpression::Equal(_, left, right)
            | SolangExpression::NotEqual(_, left, right)
            | SolangExpression::And(_, left, right)
            | SolangExpression::Or(_, left, right) => {
                let mut left = boxed_expression!(parsed_left, left);
                let right = boxed_expression!(parsed_right, right);

                left.extend(right);

                left
            }

            SolangExpression::ArraySubscript(_, array, index_maybe) => {
                let mut left = boxed_expression!(parsed_left, array);
                let right = maybe_boxed_expression!(parsed_right, index_maybe);

                left.extend(right.unwrap_or_default());

                left
            }
            SolangExpression::ArraySlice(_, exp, left, right) => {
                let mut exp = boxed_expression!(parsed_exp, exp);
                let left = maybe_boxed_expression!(parsed_left, left);
                let right = maybe_boxed_expression!(parsed_right, right);

                exp.extend(left.unwrap_or_default());
                exp.extend(right.unwrap_or_default());

                exp
            }
            SolangExpression::MemberAccess(_, expression, identifier) => {
                let mut expression = boxed_expression!(parsed_expression, expression);
                let parsed_identifier = self.parse_identifier(&Some(identifier.clone()));

                if let Some(member_type) = self.members_map.get(&parsed_identifier) {
                    match member_type {
                        MemberType::StorageField(contract_name) => {
                            expression.extend(vec![Call::ReadStorage(format!(
                                "s_{contract_name}_{parsed_identifier}"
                            ))])
                        }
                        MemberType::Function(function_header, contract_name) => {
                            if function_header.view {
                                expression.extend(vec![Call::Read(format!(
                                    "f_{contract_name}_{parsed_identifier}"
                                ))])
                            } else {
                                expression.extend(vec![Call::Write(format!(
                                    "f_{contract_name}_{parsed_identifier}"
                                ))])
                            }
                        }
                    }
                }

                expression
            }
            SolangExpression::FunctionCall(_, function, args) => {
                let mut parsed_args = self.parse_expression_vec(args);
                let parsed_function = self.parse_expression(function.as_ref());

                parsed_args.extend(parsed_function);

                parsed_args
            }
            SolangExpression::FunctionCallBlock(_, expression, statement) => {
                let mut parsed_args = self.parse_expression(expression);
                let parsed_function = self.parse_statement(statement).unwrap_or_default();

                parsed_args.extend(parsed_function);

                parsed_args
            }
            SolangExpression::NamedFunctionCall(_, expression, arguments) => {
                let mut expression = boxed_expression!(parsed_expression, expression);
                let args = arguments
                    .iter()
                    .flat_map(|arg| self.parse_expression(&arg.expr))
                    .collect::<Vec<_>>();

                expression.extend(args);

                expression
            }
            SolangExpression::ConditionalOperator(_, condition, if_true, if_false) => {
                let mut uno = boxed_expression!(parsed_condition, condition);
                let dos = boxed_expression!(parsed_if_true, if_true);
                let cuatro = boxed_expression!(parsed_if_false, if_false);

                uno.extend(dos);
                uno.extend(cuatro);

                uno
            }
            SolangExpression::Assign(_, left, right)
            | SolangExpression::AssignOr(_, left, right)
            | SolangExpression::AssignAnd(_, left, right)
            | SolangExpression::AssignXor(_, left, right)
            | SolangExpression::AssignShiftLeft(_, left, right)
            | SolangExpression::AssignShiftRight(_, left, right)
            | SolangExpression::AssignAdd(_, left, right)
            | SolangExpression::AssignSubtract(_, left, right)
            | SolangExpression::AssignMultiply(_, left, right)
            | SolangExpression::AssignDivide(_, left, right)
            | SolangExpression::AssignModulo(_, left, right) => {
                let mut uno = boxed_expression!(parsed_condition, left);
                let dos = boxed_expression!(parsed_if_true, right);

                uno.extend(dos);

                uno
            }
            SolangExpression::Variable(identifier) => {
                let parsed_identifier = self.parse_identifier(&Some(identifier.clone()));

                if let Some(member_type) = self.members_map.get(&parsed_identifier) {
                    match member_type {
                        MemberType::StorageField(contract_name) => {
                            vec![Call::ReadStorage(format!(
                                "s_{contract_name}_{parsed_identifier}"
                            ))]
                        }
                        MemberType::Function(function_header, contract_name) => {
                            if function_header.view {
                                vec![Call::Read(format!("f_{contract_name}_{parsed_identifier}"))]
                            } else {
                                vec![Call::Write(format!(
                                    "f_{contract_name}_{parsed_identifier}"
                                ))]
                            }
                        }
                    }
                } else {
                    Vec::default()
                }
            }
            SolangExpression::ArrayLiteral(_, content) => self.parse_expression_vec(content),
            _ => Vec::default(),
        }
    }

    /// Parses multiple Solang expression enum variants to Sol2Ink expression enum variants
    ///
    /// `expressions` the original Solang expression enum variants
    /// `location` the location where the expression is [being called](fn@parse_variable_access_location)
    ///
    /// Returns the vec of parsed `Expression` enum variant
    fn parse_expression_vec(&mut self, expressions: &[SolangExpression]) -> Vec<Call> {
        expressions
            .iter()
            .flat_map(|expression| self.parse_expression(expression))
            .collect()
    }

    /// Parses a Solang `IdentifierPath` struct to String
    ///
    /// `identifier_path` the original Solang identifier
    ///
    /// Returns the parsed `String`
    fn parse_identifier_path(&self, identifier_path: &IdentifierPath) -> String {
        identifier_path
            .identifiers
            .iter()
            .map(|identifier| identifier.name.clone())
            .collect::<Vec<String>>()
            .join(".")
    }

    /// Parses a Solang `Identifier` struct to String
    ///
    /// `identifier` the original Solang identifier
    ///
    /// Returns the parsed `String`
    fn parse_identifier(&self, identifier: &Option<Identifier>) -> String {
        match identifier {
            Some(identifier) => identifier.name.clone(),
            None => String::from("_"),
        }
    }
}

#[macro_export]
macro_rules! initialize_parser {
    ($parser: ident) => {
        let mut fields_map = HashMap::new();
        let mut modifier_map = HashMap::new();
        let mut imports = HashSet::new();
        let mut comments = RBTree::new();

        let mut $parser = Parser::new(
            &mut fields_map,
            &mut modifier_map,
            &mut imports,
            &mut comments,
        );
    };
}
