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
use convert_case::{
    Case::Snake,
    Casing,
};
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
        Type as SolangType,
        Unit,
        VariableAttribute,
        VariableDefinition,
        Visibility,
    },
};
use std::collections::{
    HashMap,
    HashSet,
    VecDeque,
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
                    ParserOutput::Contract(parsed_contract.name.to_case(Snake), parsed_contract);
                Ok(contract)
            }
            ContractTy::Library(_) => {
                let parsed_library = self.parse_library(contract_definition)?;
                let library =
                    ParserOutput::Library(parsed_library.name.to_case(Snake), parsed_library);
                Ok(library)
            }
            ContractTy::Interface(_) => {
                let parsed_trait = self.parse_interface(contract_definition)?;
                let interface =
                    ParserOutput::Interface(parsed_trait.name.to_case(Snake), parsed_trait);
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
                        continue
                    }
                    self.members_map.insert(name, MemberType::StorageField);
                }
                ContractPart::FunctionDefinition(function_definition) => {
                    let fn_name = self.parse_identifier(&function_definition.name);
                    match function_definition.ty {
                        FunctionTy::Function => {
                            let function_header = self.parse_function_header(function_definition);
                            self.members_map
                                .insert(fn_name.clone(), MemberType::Function(function_header));
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
                ContractPart::TypeDefinition(_) => {}
                ContractPart::Using(_) => {}
                ContractPart::StraySemicolon(_) => {}
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
                        self.members_map
                            .insert(fn_name.clone(), MemberType::Function(function_header));
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
        let initial_value = variable_definition
            .initializer
            .as_ref()
            .map(|expression| self.parse_expression(expression));
        let contract_field = ContractField {
            name,
            initial_value,
        };

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
                // @todo expression must return call
                let parsed_expression = self.parse_expression(expression);
                let parsed_if_true = self.parse_statement(if_true)?;
                let parsed_if_false = if_false
                    .as_ref()
                    .map(|statement| self.parse_statement(statement).unwrap_or_default())
                    .unwrap_or_default();
                todo!("Expression must return call")
            }
            SolangStatement::While(_, expression, statement) => {
                let parsed_expression = self.parse_expression(expression);
                let parsed_statement = self.parse_statement(statement)?;
                todo!("Expression must return call")
            }
            SolangStatement::Expression(_, expression) => {
                let parsed_expression = self.parse_expression(expression);
                todo!("Expression must return call")
            }
            SolangStatement::VariableDefinition(_, declaration, initial_value_maybe) => {
                let parsed_name = self.parse_identifier(&declaration.name).to_case(Snake);
                let parsed_declaration = Expression::VariableDeclaration(parsed_name);
                let parsed_initial_value = initial_value_maybe
                    .as_ref()
                    .map(|expression| self.parse_expression(expression));

                todo!("Expression must return call")
            }
            SolangStatement::For(_, variable_definition, condition, on_pass, body) => {
                let parsed_variable_definition = variable_definition
                    .as_ref()
                    .map(|statement| self.parse_statement(statement))
                    .map(|result| Box::new(result.unwrap()));
                let parsed_condition = condition
                    .as_ref()
                    .map(|expression| self.parse_expression(expression));
                let parsed_on_pass = on_pass
                    .as_ref()
                    .map(|statement| self.parse_statement(statement))
                    .map(|result| Box::new(result.unwrap()));
                let parsed_body = body
                    .as_ref()
                    .map(|statement| self.parse_statement(statement))
                    .map(|result| Box::new(result.unwrap()));

                todo!("Expression must return call")
            }
            SolangStatement::DoWhile(_, body, condition) => {
                let parsed_condition = self.parse_expression(condition);
                let parsed_body = Box::new(self.parse_statement(body)?);
                todo!("Expression must return call")
            }
            SolangStatement::Return(_, expression) => {
                let parsed_expression = expression
                    .as_ref()
                    .map(|expression| self.parse_expression(expression));
                todo!("Expression must return call")
            }
            SolangStatement::Try(_, expression, _, _) => {
                let parsed_expression = self.parse_expression(expression);
                todo!("Expression must return call")
            }
            _ => Vec::default(),
        })
    }

    /// Parses a Solang expression enum variant to Sol2Ink expression enum variant
    ///
    /// `expression` the original Solang expression enum variant
    /// `location` the location where the expression is [being called](fn@parse_variable_access_location)
    ///
    /// Returns the parsed `Expression` enum variant
    fn parse_expression(&mut self, expression: &SolangExpression) -> Expression {
        macro_rules! maybe_boxed_expression {
            ($to_declare:ident,$to_parse:expr) => {
                let $to_declare = $to_parse
                    .as_ref()
                    .map(|expression| Box::new(self.parse_expression(&expression)));
            };
        }

        macro_rules! boxed_expression {
            ($to_declare:ident,$to_parse:expr) => {
                let $to_declare = Box::new(self.parse_expression($to_parse));
            };
        }

        match expression {
            SolangExpression::PostIncrement(_, expression) => {
                boxed_expression!(parsed_expression, expression);
                Expression::PostIncrement(parsed_expression)
            }
            SolangExpression::PostDecrement(_, expression) => {
                boxed_expression!(parsed_expression, expression);
                Expression::PostDecrement(parsed_expression)
            }
            SolangExpression::New(_, expression) => {
                boxed_expression!(parsed_expression, expression);
                Expression::New(parsed_expression)
            }
            SolangExpression::ArraySubscript(_, array, index_maybe) => {
                match *array.clone() {
                    SolangExpression::ArraySubscript(..) => {
                        self.array_subscript_to_mapping_subscript(array, index_maybe)
                    }
                    SolangExpression::MemberAccess(_, expression, _) => {
                        let parsed_expresion = self.parse_expression(&expression);
                        match parsed_expresion {
                            Expression::MappingSubscript(..) => {
                                self.array_subscript_to_mapping_subscript(array, index_maybe)
                            }
                            _ => {
                                boxed_expression!(parsed_array, array);
                                maybe_boxed_expression!(parsed_index_maybe, index_maybe);
                                Expression::ArraySubscript(parsed_array, parsed_index_maybe)
                            }
                        }
                    }
                    SolangExpression::Variable(identifier) => {
                        let parsed_identifier = self.parse_identifier(&Some(identifier));
                        match self.members_map.get(&parsed_identifier) {
                            Some(ty) => {
                                match ty {
                                    MemberType::StorageField => {
                                        todo!()
                                    }
                                    _ => {
                                        boxed_expression!(parsed_array, array);
                                        maybe_boxed_expression!(parsed_index_maybe, index_maybe);
                                        Expression::ArraySubscript(parsed_array, parsed_index_maybe)
                                    }
                                }
                            }
                            None => {
                                boxed_expression!(parsed_array, array);
                                maybe_boxed_expression!(parsed_index_maybe, index_maybe);
                                Expression::ArraySubscript(parsed_array, parsed_index_maybe)
                            }
                        }
                    }
                    _ => {
                        boxed_expression!(parsed_array, array);
                        maybe_boxed_expression!(parsed_index_maybe, index_maybe);
                        Expression::ArraySubscript(parsed_array, parsed_index_maybe)
                    }
                }
            }
            SolangExpression::ArraySlice(_, exp, left, right) => {
                boxed_expression!(parsed_exp, exp);
                maybe_boxed_expression!(parsed_left, left);
                maybe_boxed_expression!(parsed_right, right);

                Expression::ArraySlice(parsed_exp, parsed_left, parsed_right)
            }
            SolangExpression::Parenthesis(_, expression) => {
                boxed_expression!(parsed_expression, expression);
                Expression::Parenthesis(parsed_expression)
            }
            SolangExpression::MemberAccess(_, expression, identifier) => {
                boxed_expression!(parsed_expression, expression);
                let parsed_identifier = self.parse_identifier(&Some(identifier.clone()));
                Expression::MemberAccess(parsed_expression, parsed_identifier)
            }
            SolangExpression::FunctionCall(_, function, args) => {
                let parsed_args = self.parse_expression_vec(args);

                if let SolangExpression::FunctionCallBlock(_, function, parameters) =
                    *function.clone()
                {
                    if let SolangStatement::Args(_, arguments) = *parameters {
                        let value_argument = arguments
                            .iter()
                            .map(|argument| {
                                let parsed_argument = self.parse_expression(&argument.expr);
                                let parsed_name =
                                    self.parse_identifier(&Some(argument.name.clone()));
                                (parsed_name, parsed_argument)
                            })
                            .find(|(name, _)| name == "value")
                            .map(|option| Box::new(option.1));
                        boxed_expression!(parsed_function, &function);
                        return Expression::FunctionCall(
                            parsed_function,
                            parsed_args,
                            value_argument,
                        )
                    }
                    unreachable!("Only function is allowed here!");
                } else {
                    boxed_expression!(parsed_function, function);
                    match *parsed_function.clone() {
                        Expression::Type(ty) if let Type::AccountId = *ty.clone() => {
                            if parsed_args.len() > 1 {
                                unreachable!("Multiple parameters were provided to `address` call")
                            }
                            let account_id = &parsed_args[0];
                            match account_id {
                                Expression::NumberLiteral(number) if number == "0" => {
                                    self.imports.insert(Import::ZeroAddress);
                                }
                                _ => (),
                            }
                        }
                        _ => (),
                    }
                    Expression::FunctionCall(parsed_function, parsed_args, None)
                }
            }
            SolangExpression::FunctionCallBlock(_, _, _) => Expression::None,
            SolangExpression::NamedFunctionCall(_, expression, arguments) => {
                boxed_expression!(parsed_expression, expression);
                if let Expression::Variable(_, Some(MemberType::Function(_))) =
                    *parsed_expression.clone()
                {
                    let parsed_arguments = arguments
                        .iter()
                        .map(|argument| self.parse_expression(&argument.expr))
                        .collect();
                    Expression::FunctionCall(parsed_expression, parsed_arguments, None)
                } else {
                    let parsed_arguments = arguments
                        .iter()
                        .map(|argument| {
                            let parsed_argument = self.parse_expression(&argument.expr);
                            let parsed_name = self.parse_identifier(&Some(argument.name.clone()));
                            (parsed_name, parsed_argument)
                        })
                        .collect();
                    Expression::NamedFunctionCall(parsed_expression, parsed_arguments)
                }
            }
            SolangExpression::Not(_, expression) => {
                boxed_expression!(parsed_expression, expression);
                Expression::Not(parsed_expression)
            }
            SolangExpression::Complement(_, exp) => {
                boxed_expression!(parsed_expression, exp);
                Expression::Not(parsed_expression)
            }
            SolangExpression::Delete(_, expression) => {
                boxed_expression!(parsed_expression, expression);
                Expression::Delete(parsed_expression)
            }
            SolangExpression::PreIncrement(_, expression) => {
                boxed_expression!(parsed_expression, expression);
                Expression::PreIncrement(parsed_expression)
            }
            SolangExpression::PreDecrement(_, expression) => {
                boxed_expression!(parsed_expression, expression);
                Expression::PreDecrement(parsed_expression)
            }
            SolangExpression::UnaryPlus(_, exp) => {
                boxed_expression!(parsed_expression, exp);
                Expression::UnaryPlus(parsed_expression)
            }
            SolangExpression::UnaryMinus(_, exp) => {
                boxed_expression!(parsed_expression, exp);
                Expression::UnaryMinus(parsed_expression)
            }
            SolangExpression::Power(_, left, right) => {
                boxed_expression!(parsed_left, left);
                boxed_expression!(parsed_right, right);
                Expression::Power(parsed_left, parsed_right)
            }
            SolangExpression::Multiply(_, left, right) => {
                boxed_expression!(parsed_left, left);
                boxed_expression!(parsed_right, right);
                Expression::Multiply(parsed_left, parsed_right)
            }
            SolangExpression::Divide(_, left, right) => {
                boxed_expression!(parsed_left, left);
                boxed_expression!(parsed_right, right);
                Expression::Divide(parsed_left, parsed_right)
            }
            SolangExpression::Modulo(_, left, right) => {
                boxed_expression!(parsed_left, left);
                boxed_expression!(parsed_right, right);
                Expression::Modulo(parsed_left, parsed_right)
            }
            SolangExpression::Add(_, left, right) => {
                boxed_expression!(parsed_left, left);
                boxed_expression!(parsed_right, right);
                Expression::Add(parsed_left, parsed_right)
            }
            SolangExpression::Subtract(_, left, right) => {
                boxed_expression!(parsed_left, left);
                boxed_expression!(parsed_right, right);
                Expression::Subtract(parsed_left, parsed_right)
            }
            SolangExpression::ShiftLeft(_, left, right) => {
                boxed_expression!(parsed_left, left);
                boxed_expression!(parsed_right, right);
                Expression::ShiftLeft(parsed_left, parsed_right)
            }
            SolangExpression::ShiftRight(_, left, right) => {
                boxed_expression!(parsed_left, left);
                boxed_expression!(parsed_right, right);
                Expression::ShiftRight(parsed_left, parsed_right)
            }
            SolangExpression::BitwiseAnd(_, left, right) => {
                boxed_expression!(parsed_left, left);
                boxed_expression!(parsed_right, right);
                Expression::BitwiseAnd(parsed_left, parsed_right)
            }
            SolangExpression::BitwiseXor(_, left, right) => {
                boxed_expression!(parsed_left, left);
                boxed_expression!(parsed_right, right);
                Expression::BitwiseXor(parsed_left, parsed_right)
            }
            SolangExpression::BitwiseOr(_, left, right) => {
                boxed_expression!(parsed_left, left);
                boxed_expression!(parsed_right, right);
                Expression::BitwiseOr(parsed_left, parsed_right)
            }
            SolangExpression::Less(_, left, right) => {
                boxed_expression!(parsed_left, left);
                boxed_expression!(parsed_right, right);
                Expression::Less(parsed_left, parsed_right)
            }
            SolangExpression::More(_, left, right) => {
                boxed_expression!(parsed_left, left);
                boxed_expression!(parsed_right, right);
                Expression::More(parsed_left, parsed_right)
            }
            SolangExpression::LessEqual(_, left, right) => {
                boxed_expression!(parsed_left, left);
                boxed_expression!(parsed_right, right);
                Expression::LessEqual(parsed_left, parsed_right)
            }
            SolangExpression::MoreEqual(_, left, right) => {
                boxed_expression!(parsed_left, left);
                boxed_expression!(parsed_right, right);
                Expression::MoreEqual(parsed_left, parsed_right)
            }
            SolangExpression::Equal(_, left, right) => {
                boxed_expression!(parsed_left, left);
                boxed_expression!(parsed_right, right);
                Expression::Equal(parsed_left, parsed_right)
            }
            SolangExpression::NotEqual(_, left, right) => {
                boxed_expression!(parsed_left, left);
                boxed_expression!(parsed_right, right);
                Expression::NotEqual(parsed_left, parsed_right)
            }
            SolangExpression::And(_, left, right) => {
                boxed_expression!(parsed_left, left);
                boxed_expression!(parsed_right, right);
                Expression::And(parsed_left, parsed_right)
            }
            SolangExpression::Or(_, left, right) => {
                boxed_expression!(parsed_left, left);
                boxed_expression!(parsed_right, right);
                Expression::Or(parsed_left, parsed_right)
            }
            SolangExpression::ConditionalOperator(_, condition, if_true, if_false) => {
                boxed_expression!(parsed_condition, condition);
                boxed_expression!(parsed_if_true, if_true);
                boxed_expression!(parsed_if_false, if_false);
                Expression::Ternary(parsed_condition, parsed_if_true, parsed_if_false)
            }
            SolangExpression::Assign(_, left, right) => {
                boxed_expression!(parsed_left, left);
                boxed_expression!(parsed_right, right);
                Expression::Assign(parsed_left, parsed_right)
            }
            SolangExpression::AssignOr(_, left, right) => {
                boxed_expression!(parsed_left, left);
                boxed_expression!(parsed_right, right);
                Expression::AssignOr(parsed_left, parsed_right)
            }
            SolangExpression::AssignAnd(_, left, right) => {
                boxed_expression!(parsed_left, left);
                boxed_expression!(parsed_right, right);
                Expression::AssignAnd(parsed_left, parsed_right)
            }
            SolangExpression::AssignXor(_, left, right) => {
                boxed_expression!(parsed_left, left);
                boxed_expression!(parsed_right, right);
                Expression::AssignXor(parsed_left, parsed_right)
            }
            SolangExpression::AssignShiftLeft(_, left, right) => {
                boxed_expression!(parsed_left, left);
                boxed_expression!(parsed_right, right);
                Expression::AssignShiftLeft(parsed_left, parsed_right)
            }
            SolangExpression::AssignShiftRight(_, left, right) => {
                boxed_expression!(parsed_left, left);
                boxed_expression!(parsed_right, right);
                Expression::AssignShiftRight(parsed_left, parsed_right)
            }
            SolangExpression::AssignAdd(_, left, right) => {
                boxed_expression!(parsed_left, left);
                boxed_expression!(parsed_right, right);
                Expression::AssignAdd(parsed_left, parsed_right)
            }
            SolangExpression::AssignSubtract(_, left, right) => {
                boxed_expression!(parsed_left, left);
                boxed_expression!(parsed_right, right);
                Expression::AssignSubtract(parsed_left, parsed_right)
            }
            SolangExpression::AssignMultiply(_, left, right) => {
                boxed_expression!(parsed_left, left);
                boxed_expression!(parsed_right, right);
                Expression::AssignMultiply(parsed_left, parsed_right)
            }
            SolangExpression::AssignDivide(_, left, right) => {
                boxed_expression!(parsed_left, left);
                boxed_expression!(parsed_right, right);
                Expression::AssignDivide(parsed_left, parsed_right)
            }
            SolangExpression::AssignModulo(_, left, right) => {
                boxed_expression!(parsed_left, left);
                boxed_expression!(parsed_right, right);
                Expression::AssignModulo(parsed_left, parsed_right)
            }
            SolangExpression::BoolLiteral(_, value) => Expression::BoolLiteral(*value),
            SolangExpression::NumberLiteral(_, literal, exponent) => {
                if !exponent.is_empty() {
                    let literal = literal.parse::<i128>().unwrap();
                    let exponent = exponent.parse::<u32>().unwrap();
                    let out = literal * 10_i128.pow(exponent);
                    Expression::NumberLiteral(out.to_string())
                } else {
                    Expression::NumberLiteral(literal.clone())
                }
            }
            SolangExpression::RationalNumberLiteral(_, integer_exp, float_exp, exp) => {
                let float_len = float_exp.len();
                let integer_exp = integer_exp.parse::<f64>().unwrap();
                let float_exp = float_exp.parse::<f64>().unwrap_or(0_f64);
                let exp = exp.parse::<i32>().unwrap_or(0);

                let result =
                    (integer_exp + float_exp * 10_f64.powi(-(float_len as i32))) * 10_f64.powi(exp);

                Expression::NumberLiteral(result.to_string())
            }
            SolangExpression::HexNumberLiteral(_, hex_number) => {
                Expression::HexLiteral(hex_number.clone())
            }
            SolangExpression::StringLiteral(strings) => {
                let parsed_strings = strings
                    .iter()
                    .map(|string_literal| string_literal.string.clone())
                    .collect();
                Expression::StringLiteral(parsed_strings)
            }
            SolangExpression::Type(_, solidity_type) => {
                let parsed_type = Box::new(self.convert_solidity_type(solidity_type));
                Expression::Type(parsed_type)
            }
            SolangExpression::HexLiteral(hex_vec) => {
                let literal = hex_vec
                    .iter()
                    .map(|hex| hex.hex.clone())
                    .collect::<Vec<_>>()
                    .join("");
                Expression::HexLiteral(literal)
            }
            SolangExpression::AddressLiteral(_, literal) => Expression::HexLiteral(literal.clone()),
            SolangExpression::Variable(identifier) => {
                let parsed_identifier = self.parse_identifier(&Some(identifier.clone()));
                if parsed_identifier == "_" {
                    return Expression::ModifierBody
                }
                let member_type = self.members_map.get(&parsed_identifier);
                Expression::Variable(parsed_identifier, member_type.cloned())
            }
            SolangExpression::List(_, parameters) => {
                let list = parameters
                    .iter()
                    .map(|tuple| tuple.1.clone())
                    .map(|parameter_maybe| {
                        let name = self.parse_identifier(
                            &parameter_maybe
                                .map(|parameter| parameter.name)
                                .unwrap_or(None),
                        );
                        Expression::Variable(name, None)
                    })
                    .collect();
                Expression::List(list)
            }
            SolangExpression::ArrayLiteral(_, content) => {
                let list = self.parse_expression_vec(content);
                Expression::ArrayLiteral(list)
            }
            SolangExpression::Unit(_, exp, unit) => {
                let constant: i128 = match unit {
                    Unit::Seconds(_) => 1,
                    Unit::Minutes(_) => 60,
                    Unit::Hours(_) => 3600,
                    Unit::Days(_) => 86400,
                    Unit::Weeks(_) => 604800,
                    Unit::Wei(_) => 1,
                    Unit::Gwei(_) => 1_000_000_000,
                    Unit::Ether(_) => 1_000_000_000_000_000_000,
                };

                boxed_expression!(parsed_exp, exp);

                Expression::Unit(parsed_exp, constant)
            }
            SolangExpression::This(_) => Expression::This(),
        }
    }

    /// Parses multiple Solang expression enum variants to Sol2Ink expression enum variants
    ///
    /// `expressions` the original Solang expression enum variants
    /// `location` the location where the expression is [being called](fn@parse_variable_access_location)
    ///
    /// Returns the vec of parsed `Expression` enum variant
    fn parse_expression_vec(&mut self, expressions: &[SolangExpression]) -> Vec<Expression> {
        expressions
            .iter()
            .map(|expression| self.parse_expression(expression))
            .collect()
    }

    /// Converts a Solang `Expression::ArraySubscript` enum variant to Sol2Ink `Expression::MappingSubscript` enum variant
    /// We do this to differentiate between arrays and mappings
    ///
    /// `array` the array we want to convert to mapping
    /// `index_maybe` the index we were accessing within the ArraySubscript
    /// `expression` the original Solang expression enum variant
    /// `location` the location where the expression is [being called](fn@parse_variable_access_location)
    ///
    /// Returns the parsed `Expression` enum variant
    fn array_subscript_to_mapping_subscript(
        &mut self,
        array: &SolangExpression,
        index_maybe: &Option<Box<SolangExpression>>,
    ) -> Expression {
        let mut parsed_indices = VecDeque::default();
        parsed_indices.push_back(self.parse_expression(&index_maybe.as_ref().unwrap().clone()));
        let mut array_now = array.clone();
        while let SolangExpression::ArraySubscript(_, array, index_maybe) = array_now {
            parsed_indices.push_back(self.parse_expression(&index_maybe.unwrap()));
            array_now = *array.clone();
        }
        let mut vec_indices = Vec::default();
        while !parsed_indices.is_empty() {
            vec_indices.push(parsed_indices.pop_back().unwrap());
        }

        let parsed_array = Box::new(self.parse_expression(&array_now));
        Expression::MappingSubscript(parsed_array, vec_indices)
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

    /// Converts a Solang `Type` enum variant to Sol2Ink `Type` enum variant
    /// We do this to convert some Solidity specific types into ink! specific types
    ///
    /// `ty` the origina Solang `Type` enum variant
    ///
    /// Returns the parsed `Type` enum variant
    fn convert_solidity_type(&self, solidity_type: &SolangType) -> Type {
        match solidity_type {
            SolangType::Address | SolangType::AddressPayable => Type::AccountId,
            SolangType::Bool => Type::Bool,
            SolangType::String => Type::String,
            SolangType::Int(original_bytes) => Type::Int(self.convert_int_bits(original_bytes)),
            SolangType::Uint(original_bytes) => Type::Uint(self.convert_int_bits(original_bytes)),
            SolangType::Bytes(length) => Type::Bytes(*length),
            SolangType::DynamicBytes => Type::DynamicBytes,
            _ => Type::None,
        }
    }

    /// Converts a Solidity integer size into rust integer size
    /// Possible variants in Rust are 8, 16, 32, 64 and 128, we choose the nearest possible to fit
    /// If the original size is greater than 128 bits, we make it 128 bits
    ///
    /// `original_bits` the origina size of the integer
    ///
    /// Returns the converted integer size
    fn convert_int_bits(&self, original_bits: &u16) -> u16 {
        match *original_bits {
            i if i <= 8 => 8,
            i if i <= 16 => 16,
            i if i <= 32 => 32,
            i if i <= 64 => 64,
            _ => 128,
        }
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
