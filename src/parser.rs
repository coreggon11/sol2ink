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
        Comment as SolangComment,
        ContractDefinition,
        ContractPart,
        ContractTy,
        EnumDefinition,
        EventDefinition,
        Expression as SolangExpression,
        FunctionAttribute,
        FunctionDefinition,
        FunctionTy,
        Identifier,
        IdentifierPath,
        Mutability,
        SourceUnitPart,
        Statement as SolangStatement,
        StructDefinition,
        Type as SolangType,
        Unit,
        VariableAttribute,
        VariableDefinition,
        Visibility,
    },
};
use std::{
    collections::{
        HashMap,
        HashSet,
        VecDeque,
    },
    hash::Hash,
};

#[derive(Clone, Hash)]
pub enum ParserOutput {
    Contract(String, Contract),
    Interface(String, Interface),
    Library(String, Library),
    None,
}

#[derive(Debug, Eq, PartialEq)]
pub enum ParserError {
    FileError(String),
    FileCorrupted,

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

    /// Removes slashes and asterisks from a comment and returns it as a String
    ///
    /// `original` the original comment
    pub fn filter_comment(&self, original: &SolangComment) -> String {
        match original {
            SolangComment::Line(_, content) => content.trim()[2..].to_owned(),
            SolangComment::Block(_, content) => {
                let trimmed = content.trim();
                trimmed[2..trimmed.len() - 2].trim().to_owned()
            }
            SolangComment::DocLine(_, content) => content[3..].trim().to_owned(),
            SolangComment::DocBlock(_, content) => {
                let trimmed = content.trim();
                trimmed[3..trimmed.len() - 2]
                    .trim()
                    .split('\n')
                    .map(|str| &str.trim()[1..])
                    .collect::<Vec<&str>>()
                    .join("\n")
            }
        }
    }

    /// Parses a fil and returns the vec of ParserOutput or a ParserError
    ///
    /// `content` the content of a solidity file
    pub fn parse_file(&mut self, content: &str) -> Result<Vec<ParserOutput>, ParserError> {
        let token_tree = parse(content, 0).map_err(|_| ParserError::FileCorrupted)?;

        let mut output = Vec::new();
        let source_unit = token_tree.0;
        let comments = token_tree.1;

        comments.iter().for_each(|comment| {
            match comment {
                SolangComment::Line(loc, _)
                | SolangComment::Block(loc, _)
                | SolangComment::DocLine(loc, _)
                | SolangComment::DocBlock(loc, _) => {
                    self.comments
                        .insert(loc.end(), self.filter_comment(comment));
                }
            }
        });

        for source_unit_part in source_unit.0.iter() {
            match &source_unit_part {
                SourceUnitPart::ContractDefinition(contract) => {
                    output.push(self.handle_contract_definition(contract)?);
                }
                SourceUnitPart::ImportDirective(_) | SourceUnitPart::PragmaDirective(..) => {}
                _ => println!("Found a source unit outside of contract"),
            }
        }

        Ok(output)
    }

    /// Retrieves the comments from the Parser's RB Tree from the beginning until a key
    ///
    /// `until` key value where we stop iterating over the tree
    fn get_comments(&mut self, until: usize) -> Vec<String> {
        let mut comments = Vec::default();
        let mut removed = Vec::default();
        for node in self.comments.iter() {
            if *node.0 > until {
                break
            }
            comments.push(node.1.clone());
            removed.push(*node.0);
        }
        removed.iter().for_each(|key| {
            self.comments.remove(key);
        });
        comments
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
            ContractTy::Abstract(loc) | ContractTy::Contract(loc) => {
                let comments = self.get_comments(loc.end());
                let parsed_contract = self.parse_contract(contract_definition, &comments)?;
                let contract =
                    ParserOutput::Contract(parsed_contract.name.to_case(Snake), parsed_contract);
                Ok(contract)
            }
            ContractTy::Library(loc) => {
                let comments = self.get_comments(loc.end());
                let parsed_library = self.parse_library(contract_definition, &comments)?;
                let library =
                    ParserOutput::Library(parsed_library.name.to_case(Snake), parsed_library);
                Ok(library)
            }
            ContractTy::Interface(loc) => {
                let comments = self.get_comments(loc.end());
                let parsed_trait = self.parse_interface(contract_definition, &comments)?;
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
        comments: &[String],
    ) -> Result<Contract, ParserError> {
        let name = self.parse_identifier(&contract_definition.name);
        let base = contract_definition
            .base
            .iter()
            .map(|base| self.parse_identifier_path(&base.name))
            .collect();

        let mut structs: Vec<Struct> = Default::default();
        let mut events: Vec<Event> = Default::default();
        let mut enums: Vec<Enum> = Default::default();
        let mut fields: Vec<ContractField> = Default::default();
        let mut functions: Vec<Function> = Default::default();
        let mut constructor: Function = Default::default();
        let mut modifiers: Vec<Function> = Default::default();

        // first we register all members of the contract
        for part in contract_definition.parts.iter() {
            match part {
                ContractPart::VariableDefinition(variable_definition) => {
                    let field_type = self.parse_type(&variable_definition.ty)?;
                    let name = self.parse_identifier(&variable_definition.name);
                    let constant = variable_definition
                        .attrs
                        .iter()
                        .any(|item| matches!(item, VariableAttribute::Constant(_)));
                    self.members_map.insert(
                        name,
                        if constant {
                            MemberType::Constant
                        } else {
                            MemberType::Variable(Box::new(field_type))
                        },
                    );
                }
                ContractPart::FunctionDefinition(function_definition) => {
                    let fn_name = self.parse_identifier(&function_definition.name);
                    let external = function_definition.attributes.iter().any(|attribute| {
                        matches!(
                            attribute,
                            FunctionAttribute::Visibility(Visibility::External(_))
                                | FunctionAttribute::Visibility(Visibility::Public(_))
                        )
                    });
                    match function_definition.ty {
                        FunctionTy::Function => {
                            self.members_map.insert(
                                fn_name.clone(),
                                if external {
                                    MemberType::Function
                                } else {
                                    MemberType::FunctionPrivate
                                },
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
                ContractPart::Annotation(_) => println!("Anottation: {part:?}"),
                ContractPart::VariableDefinition(variable_definition) => {
                    let parsed_field = self.parse_storage_field(variable_definition)?;
                    fields.push(parsed_field);
                }
                ContractPart::ErrorDefinition(_) => {}
                ContractPart::FunctionDefinition(function_definition) => {
                    let parsed_function = self.parse_function(function_definition)?;
                    match function_definition.ty {
                        FunctionTy::Constructor => constructor = parsed_function,
                        FunctionTy::Modifier => modifiers.push(parsed_function),
                        FunctionTy::Function => functions.push(parsed_function),
                        _ => (),
                    }
                }
                ContractPart::StructDefinition(struct_definition) => {
                    let parsed_struct = self.parse_struct(struct_definition)?;
                    structs.push(parsed_struct);
                }
                ContractPart::EventDefinition(event_definition) => {
                    let parsed_event = self.parse_event(event_definition)?;
                    events.push(parsed_event);
                }
                ContractPart::EnumDefinition(enum_definition) => {
                    let parsed_enum = self.parse_enum(enum_definition)?;
                    enums.push(parsed_enum);
                }
                ContractPart::TypeDefinition(_) => {}
                ContractPart::Using(_) => {}
                ContractPart::StraySemicolon(_) => {}
            }
        }

        Ok(Contract {
            name,
            structs,
            events,
            enums,
            fields,
            functions,
            constructor,
            modifiers,
            imports: self.imports.clone(),
            contract_doc: comments.to_vec(),
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
        comments: &[String],
    ) -> Result<Interface, ParserError> {
        let name = self.parse_identifier(&contract_definition.name);

        let mut structs: Vec<Struct> = Default::default();
        let mut events: Vec<Event> = Default::default();
        let mut enums: Vec<Enum> = Default::default();
        let mut function_headers: Vec<FunctionHeader> = Default::default();

        for part in contract_definition.parts.iter() {
            match part {
                ContractPart::Annotation(_) => println!("Anottation: {part:?}"),
                ContractPart::StructDefinition(struct_definition) => {
                    let parsed_struct = self.parse_struct(struct_definition)?;
                    structs.push(parsed_struct);
                }
                ContractPart::EventDefinition(event_definition) => {
                    let parsed_event = self.parse_event(event_definition)?;
                    events.push(parsed_event);
                }
                ContractPart::EnumDefinition(enum_definition) => {
                    let parsed_enum = self.parse_enum(enum_definition)?;
                    enums.push(parsed_enum);
                }
                ContractPart::ErrorDefinition(_) => {}
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
            events,
            enums,
            structs,
            function_headers,
            imports: self.imports.clone(),
            comments: comments.to_vec(),
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
        comments: &[String],
    ) -> Result<Library, ParserError> {
        let name = self.parse_identifier(&contract_definition.name);

        let mut fields: Vec<ContractField> = Default::default();
        let mut events: Vec<Event> = Default::default();
        let mut enums: Vec<Enum> = Default::default();
        let mut structs: Vec<Struct> = Default::default();
        let mut functions: Vec<Function> = Default::default();

        // first we register all members of the contract
        for part in contract_definition.parts.iter() {
            if let ContractPart::FunctionDefinition(function_definition) = part {
                let fn_name = self.parse_identifier(&function_definition.name);
                let external = function_definition.attributes.iter().any(|attribute| {
                    matches!(
                        attribute,
                        FunctionAttribute::Visibility(Visibility::External(_))
                            | FunctionAttribute::Visibility(Visibility::Public(_))
                    )
                });
                self.members_map.insert(
                    fn_name.clone(),
                    if external {
                        MemberType::Function
                    } else {
                        MemberType::FunctionPrivate
                    },
                );
            }
        }

        for part in contract_definition.parts.iter() {
            match part {
                ContractPart::Annotation(_) => println!("Anottation: {part:?}"),
                ContractPart::StructDefinition(struct_definition) => {
                    let parsed_struct = self.parse_struct(struct_definition)?;
                    structs.push(parsed_struct);
                }
                ContractPart::EventDefinition(event_definition) => {
                    let parsed_event = self.parse_event(event_definition)?;
                    events.push(parsed_event);
                }
                ContractPart::EnumDefinition(enum_definition) => {
                    let parsed_enum = self.parse_enum(enum_definition)?;
                    enums.push(parsed_enum);
                }
                ContractPart::ErrorDefinition(_) => {}
                ContractPart::FunctionDefinition(function_definition) => {
                    if function_definition.ty == FunctionTy::Function {
                        let parsed_function = self.parse_function(function_definition)?;
                        functions.push(parsed_function)
                    }
                }
                ContractPart::TypeDefinition(_) => {}
                ContractPart::Using(_) => {}
                ContractPart::StraySemicolon(_) => {}
                ContractPart::VariableDefinition(variable_definition) => {
                    let parsed_field = self.parse_storage_field(variable_definition)?;
                    self.members_map.insert(
                        parsed_field.name.clone(),
                        MemberType::Variable(Box::new(parsed_field.field_type.clone())),
                    );
                    fields.push(parsed_field);
                }
            }
        }

        Ok(Library {
            name,
            fields,
            events,
            enums,
            structs,
            functions,
            imports: self.imports.clone(),
            libraray_doc: comments.to_vec(),
        })
    }

    /// Parses a Solang struct definition to Sol2Ink struct definition
    ///
    /// `struct_definition` the Solang struct definition
    ///
    /// Returns the parsed `Struct` struct
    fn parse_struct(
        &mut self,
        struct_definition: &StructDefinition,
    ) -> Result<Struct, ParserError> {
        let name = self.parse_identifier(&struct_definition.name);

        let comments = self.get_comments(struct_definition.loc.end());

        let fields: Vec<StructField> = struct_definition
            .fields
            .iter()
            .filter_map(|variable_declaration| {
                let name = self.parse_identifier(&variable_declaration.name);
                let ty = self.parse_type(&variable_declaration.ty).ok()?;
                self.members_map
                    .insert(name.clone(), MemberType::None(Box::new(ty.clone())));
                let struct_field = StructField {
                    name,
                    field_type: ty,
                    comments: Default::default(),
                };
                Some(struct_field)
            })
            .collect();

        let parsed_struct = Struct {
            name,
            fields,
            comments,
        };

        Ok(parsed_struct)
    }

    /// Parses a Solang event definition to Sol2Ink evet definition
    ///
    /// `event_definition` the Solang event definition
    ///
    /// Returns the parsed `Event` struct
    fn parse_event(&mut self, event_definition: &EventDefinition) -> Result<Event, ParserError> {
        let name = self.parse_identifier(&event_definition.name);

        let comments = self.get_comments(event_definition.loc.end());
        let fields: Vec<EventField> = event_definition
            .fields
            .iter()
            .filter_map(|variable_declaration| {
                let event_field_name = self.parse_identifier(&variable_declaration.name);
                let event_field = EventField {
                    name: if event_field_name == "_" {
                        String::from("anonymous")
                    } else {
                        event_field_name
                    },
                    field_type: self.parse_type(&variable_declaration.ty).ok()?,
                    indexed: variable_declaration.indexed,
                    comments: self.get_comments(variable_declaration.loc.end()),
                };
                Some(event_field)
            })
            .collect();

        let parsed_event = Event {
            name,
            fields,
            comments,
        };

        Ok(parsed_event)
    }

    /// Parses a Solang enum definition to Sol2Ink enum definition
    ///
    /// `enum_definition` the Solang enum definition
    ///
    /// Returns the parsed `Enum` struct
    fn parse_enum(&mut self, enum_definition: &EnumDefinition) -> Result<Enum, ParserError> {
        let name = self.parse_identifier(&enum_definition.name);

        let comments = self.get_comments(enum_definition.loc.end());
        let values: Vec<EnumValue> = enum_definition
            .values
            .iter()
            .map(|enum_value| {
                EnumValue {
                    name: self.parse_identifier(enum_value),
                    comments: if let Some(value) = enum_value {
                        self.get_comments(value.loc.end())
                    } else {
                        Vec::default()
                    },
                }
            })
            .collect();

        let parsed_enum = Enum {
            name,
            values,
            comments,
        };

        Ok(parsed_enum)
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
        let field_type = self.parse_type(&variable_definition.ty)?;
        let name = self.parse_identifier(&variable_definition.name);
        let constant = variable_definition
            .attrs
            .iter()
            .any(|item| matches!(item, VariableAttribute::Constant(_)));
        let public = variable_definition.attrs.iter().any(|item| {
            matches!(
                item,
                VariableAttribute::Visibility(Visibility::External(_))
                    | VariableAttribute::Visibility(Visibility::Public(_))
            )
        });
        let initial_value = variable_definition.initializer.as_ref().map(|expression| {
            self.parse_expression(expression, VariableAccessLocation::Constructor)
        });
        let comments = self.get_comments(variable_definition.loc.end());
        let contract_field = ContractField {
            field_type,
            name,
            initial_value,
            constant,
            public,
            comments,
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
        let mut invalid_modifiers = HashMap::new();
        for modifier in header.invalid_modifiers.clone() {
            match modifier {
                Expression::InvalidModifier(name, _) => {
                    if let Some(function) = self.modifiers_map.clone().get(&name) {
                        invalid_modifiers.insert(
                            (header.name.clone(), name),
                            Function {
                                header: self.parse_function_header(function),
                                body: if let Some(body) = &function.body {
                                    Some(self.parse_statement(
                                        body,
                                        self.parse_variable_access_location(function_definition),
                                    )?)
                                } else {
                                    None
                                },
                                ..Default::default()
                            },
                        );
                    }
                }
                _ => unreachable!("Only invalid modifiers are allowed here"),
            }
        }

        let body = if let Some(statement) = &function_definition.body {
            Some(self.parse_statement(
                statement,
                self.parse_variable_access_location(function_definition),
            )?)
        } else {
            None
        };

        Ok(Function {
            header,
            body,
            invalid_modifiers,
        })
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
        let params = function_definition
            .params
            .iter()
            .map(|item| item.1.clone().unwrap())
            .filter_map(|param| {
                let name = self.parse_identifier(&param.name);
                let param_type = self.parse_type(&param.ty).ok()?;
                Some(FunctionParam { name, param_type })
            })
            .collect();
        let all_modifiers: Vec<Expression> = function_definition
            .attributes
            .iter()
            .filter(|&attribute| matches!(attribute, FunctionAttribute::BaseOrModifier(..)))
            .map(|modifier| {
                if let FunctionAttribute::BaseOrModifier(_, base) = modifier {
                    let parsed_name = self.parse_identifier_path(&base.name);
                    let parsed_args = if let Some(args) = &base.args {
                        self.parse_expression_vec(args, VariableAccessLocation::Any)
                    } else {
                        Vec::default()
                    };
                    if parsed_args.iter().any(function_call_in_expression) {
                        Expression::InvalidModifier(parsed_name, parsed_args)
                    } else {
                        Expression::Modifier(parsed_name, parsed_args)
                    }
                } else {
                    unreachable!("The vec was filtered before");
                }
            })
            .collect();
        let modifiers = all_modifiers
            .iter()
            .filter(|expression| matches!(expression, Expression::Modifier(..)))
            .cloned()
            .collect();
        let invalid_modifiers = all_modifiers
            .iter()
            .filter(|expression| matches!(expression, Expression::InvalidModifier(..)))
            .cloned()
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
        let return_params = function_definition
            .returns
            .iter()
            .map(|item| item.1.clone().unwrap())
            .filter_map(|param| {
                let name = self.parse_identifier(&param.name);
                let param_type = self.parse_type(&param.ty).ok()?;
                Some(FunctionParam { name, param_type })
            })
            .collect();

        FunctionHeader {
            name,
            params,
            external,
            view,
            payable,
            return_params,
            modifiers,
            invalid_modifiers,
            comments: self.get_comments(function_definition.loc.end()),
        }
    }

    /// Parses from where a variable will be accessed
    /// This affects if we call `Self::var`, `T::var` or `self.var`
    ///
    /// `function_definition` the Solang function definition
    ///
    /// Returns the corresponding `VariableAccessLocation` enum variant
    fn parse_variable_access_location(
        &self,
        function_definition: &FunctionDefinition,
    ) -> VariableAccessLocation {
        match function_definition.ty {
            FunctionTy::Constructor => VariableAccessLocation::Constructor,
            FunctionTy::Modifier => VariableAccessLocation::Modifier,
            _ => VariableAccessLocation::Any,
        }
    }

    /// Parses a Solang statement enum variant to Sol2Ink statement enum variant
    ///
    /// `statement` the original Solang statement enum variant
    /// `location` the location where the statement is [being called](fn@parse_variable_access_location)
    ///
    /// Returns the parsed `Statement` enum variant
    fn parse_statement(
        &mut self,
        statement: &SolangStatement,
        location: VariableAccessLocation,
    ) -> Result<Statement, ParserError> {
        Ok(match statement {
            SolangStatement::Block {
                loc: _,
                unchecked,
                statements,
            } => {
                let parsed_statements = statements
                    .iter()
                    .map(|statement| self.parse_statement(statement, location.clone()))
                    .map(|result| result.unwrap())
                    .collect::<Vec<_>>();
                if *unchecked {
                    Statement::UncheckedBlock(parsed_statements)
                } else {
                    Statement::Block(parsed_statements)
                }
            }
            SolangStatement::Assembly {
                loc: _,
                dialect: _,
                flags: _,
                block: _,
            } => Statement::Assembly,
            SolangStatement::Args(_, _) => {
                println!("{statement:?}");
                todo!()
            }
            SolangStatement::If(_, expression, if_true, if_false) => {
                let parsed_expression = self.parse_expression(expression, location.clone());
                let parsed_if_true = Box::new(self.parse_statement(if_true, location.clone())?);
                let parsed_if_false = if_false
                    .as_ref()
                    .map(|statement| self.parse_statement(statement, location.clone()))
                    .map(|result| Box::new(result.unwrap()));
                Statement::If(parsed_expression, parsed_if_true, parsed_if_false)
            }
            SolangStatement::While(_, expression, statement) => {
                let parsed_expression = self.parse_expression(expression, location.clone());
                let parsed_statement = Box::new(self.parse_statement(statement, location)?);
                Statement::While(parsed_expression, parsed_statement)
            }
            SolangStatement::Expression(_, expression) => {
                let parsed_expression = self.parse_expression(expression, location);
                Statement::Expression(parsed_expression)
            }
            SolangStatement::VariableDefinition(_, declaration, initial_value_maybe) => {
                let parsed_name = self.parse_identifier(&declaration.name).to_case(Snake);
                let parsed_type = Box::new(self.parse_type(&declaration.ty)?);
                let parsed_declaration = Expression::VariableDeclaration(parsed_type, parsed_name);
                let parsed_initial_value = initial_value_maybe
                    .as_ref()
                    .map(|expression| self.parse_expression(expression, location));

                Statement::VariableDefinition(parsed_declaration, parsed_initial_value)
            }
            SolangStatement::For(_, variable_definition, condition, on_pass, body) => {
                let parsed_variable_definition = variable_definition
                    .as_ref()
                    .map(|statement| self.parse_statement(statement, location.clone()))
                    .map(|result| Box::new(result.unwrap()));
                let parsed_condition = condition
                    .as_ref()
                    .map(|expression| self.parse_expression(expression, location.clone()));
                let parsed_on_pass = on_pass
                    .as_ref()
                    .map(|statement| self.parse_statement(statement, location.clone()))
                    .map(|result| Box::new(result.unwrap()));
                let parsed_body = body
                    .as_ref()
                    .map(|statement| self.parse_statement(statement, location))
                    .map(|result| Box::new(result.unwrap()));

                Statement::For(
                    parsed_variable_definition,
                    parsed_condition,
                    parsed_on_pass,
                    parsed_body,
                )
            }
            SolangStatement::DoWhile(_, body, condition) => {
                let parsed_condition = self.parse_expression(condition, location.clone());
                let parsed_body = Box::new(self.parse_statement(body, location)?);
                Statement::DoWhile(parsed_body, parsed_condition)
            }
            SolangStatement::Continue(_) => Statement::Continue,
            SolangStatement::Break(_) => Statement::Break,
            SolangStatement::Return(_, expression) => {
                let parsed_expression = expression
                    .as_ref()
                    .map(|expression| self.parse_expression(expression, location));
                Statement::Return(parsed_expression)
            }
            SolangStatement::Revert(_, identifier_path, args) => {
                let identifier_path = identifier_path
                    .as_ref()
                    .map(|identifier_path| self.parse_identifier_path(identifier_path))
                    .unwrap_or(String::from("_"));
                let parsed_args = self.parse_expression_vec(args, location);
                Statement::Revert(identifier_path, parsed_args)
            }
            SolangStatement::RevertNamedArgs(_, _, _) => todo!(),
            SolangStatement::Emit(_, expression) => {
                let parsed_expression = self.parse_expression(expression, location);
                Statement::Emit(parsed_expression)
            }
            SolangStatement::Try(_, expression, _, _) => {
                let parsed_expression = self.parse_expression(expression, location);
                Statement::Try(parsed_expression)
            }
            SolangStatement::Error(_) => todo!(),
        })
    }

    /// Parses a Solang expression enum variant to Sol2Ink expression enum variant
    ///
    /// `expression` the original Solang expression enum variant
    /// `location` the location where the expression is [being called](fn@parse_variable_access_location)
    ///
    /// Returns the parsed `Expression` enum variant
    fn parse_expression(
        &mut self,
        expression: &SolangExpression,
        location: VariableAccessLocation,
    ) -> Expression {
        macro_rules! maybe_boxed_expression {
            ($to_declare:ident,$to_parse:expr) => {
                let $to_declare = $to_parse.as_ref().map(|expression| {
                    Box::new(self.parse_expression(&expression, location.clone()))
                });
            };
        }

        macro_rules! boxed_expression {
            ($to_declare:ident,$to_parse:expr) => {
                let $to_declare = Box::new(self.parse_expression($to_parse, location.clone()));
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
                        self.array_subscript_to_mapping_subscript(array, index_maybe, location)
                    }
                    SolangExpression::MemberAccess(_, expression, _) => {
                        let parsed_expresion = self.parse_expression(&expression, location.clone());
                        match parsed_expresion {
                            Expression::MappingSubscript(..) => {
                                self.array_subscript_to_mapping_subscript(
                                    array,
                                    index_maybe,
                                    location,
                                )
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
                                    MemberType::Variable(variable_type)
                                    | MemberType::None(variable_type) => {
                                        match *variable_type.clone() {
                                            Type::Mapping(_, _) => {
                                                self.array_subscript_to_mapping_subscript(
                                                    array,
                                                    index_maybe,
                                                    location,
                                                )
                                            }
                                            _ => {
                                                boxed_expression!(parsed_array, array);
                                                maybe_boxed_expression!(
                                                    parsed_index_maybe,
                                                    index_maybe
                                                );
                                                Expression::ArraySubscript(
                                                    parsed_array,
                                                    parsed_index_maybe,
                                                )
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
                let parsed_args = self.parse_expression_vec(args, location.clone());

                if let SolangExpression::FunctionCallBlock(_, function, parameters) =
                    *function.clone()
                {
                    if let SolangStatement::Args(_, arguments) = *parameters.clone() {
                        let value_argument = arguments
                            .iter()
                            .map(|argument| {
                                let parsed_argument =
                                    self.parse_expression(&argument.expr, location.clone());
                                let parsed_name =
                                    self.parse_identifier(&Some(argument.name.clone()));
                                (parsed_name, parsed_argument)
                            })
                            .filter(|(name, _)| name == "value")
                            .nth(0)
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
                        _ => ()
                    }
                    Expression::FunctionCall(parsed_function, parsed_args, None)
                }
            }
            SolangExpression::FunctionCallBlock(_, _, _) => Expression::None,
            SolangExpression::NamedFunctionCall(_, expression, arguments) => {
                boxed_expression!(parsed_expression, expression);
                if let Expression::Variable(_, MemberType::Function, _) = *parsed_expression.clone()
                {
                    let parsed_arguments = arguments
                        .iter()
                        .map(|argument| self.parse_expression(&argument.expr, location.clone()))
                        .collect();
                    Expression::FunctionCall(parsed_expression, parsed_arguments, None)
                } else {
                    let parsed_arguments = arguments
                        .iter()
                        .map(|argument| {
                            let parsed_argument =
                                self.parse_expression(&argument.expr, location.clone());
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
            SolangExpression::AddressLiteral(_, _) => todo!(),
            SolangExpression::Variable(identifier) => {
                let parsed_identifier = self.parse_identifier(&Some(identifier.clone()));
                if parsed_identifier == "_" {
                    return Expression::ModifierBody
                }
                let none = MemberType::None(Box::new(Type::None));
                let member_type = self.members_map.get(&parsed_identifier).unwrap_or(&none);
                Expression::Variable(parsed_identifier, member_type.clone(), location)
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
                        Expression::Variable(
                            name,
                            MemberType::None(Box::new(Type::None)),
                            location.clone(),
                        )
                    })
                    .collect();
                Expression::List(list)
            }
            SolangExpression::ArrayLiteral(_, content) => {
                let list = self.parse_expression_vec(content, location);
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
            SolangExpression::This(_) => Expression::This(location),
        }
    }

    /// Parses multiple Solang expression enum variants to Sol2Ink expression enum variants
    ///
    /// `expressions` the original Solang expression enum variants
    /// `location` the location where the expression is [being called](fn@parse_variable_access_location)
    ///
    /// Returns the vec of parsed `Expression` enum variant
    fn parse_expression_vec(
        &mut self,
        expressions: &[SolangExpression],
        location: VariableAccessLocation,
    ) -> Vec<Expression> {
        expressions
            .iter()
            .map(|expression| self.parse_expression(expression, location.clone()))
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
        location: VariableAccessLocation,
    ) -> Expression {
        let mut parsed_indices = VecDeque::default();
        parsed_indices.push_back(
            self.parse_expression(&index_maybe.as_ref().unwrap().clone(), location.clone()),
        );
        let mut array_now = array.clone();
        while let SolangExpression::ArraySubscript(_, array, index_maybe) = array_now {
            parsed_indices
                .push_back(self.parse_expression(&index_maybe.unwrap(), location.clone()));
            array_now = *array.clone();
        }
        let mut vec_indices = Vec::default();
        while !parsed_indices.is_empty() {
            vec_indices.push(parsed_indices.pop_back().unwrap());
        }

        let parsed_array = Box::new(self.parse_expression(&array_now, location));
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
            .join("::")
    }

    /// Converts a Solang `Expression::Type` enum variant to Sol2Ink `Type` enum variant
    /// We do this to convert some Solidity specific types into ink! specific types
    ///
    /// `ty` the origina Solang `Expression::Type` enum variant
    ///
    /// Returns the parsed `Type` enum variant
    fn parse_type(&mut self, ty: &SolangExpression) -> Result<Type, ParserError> {
        match &ty {
            SolangExpression::Type(_, SolangType::Mapping(_, key_type, value_type)) => {
                let mut parsed_key_types = vec![self.parse_type(key_type)?];
                let mut value_type_now = value_type.as_ref();
                while let SolangExpression::Type(
                    _,
                    SolangType::Mapping(_, key_type_value, value_type_value),
                ) = value_type_now
                {
                    parsed_key_types.push(self.parse_type(key_type_value)?);
                    value_type_now = value_type_value;
                }
                let parsed_value_type = self.parse_type(value_type_now)?;
                self.imports.insert(Import::Mapping);
                Ok(Type::Mapping(parsed_key_types, Box::new(parsed_value_type)))
            }
            SolangExpression::Type(_, solidity_type) => {
                let converted_type = self.convert_solidity_type(solidity_type);
                match converted_type {
                    Type::Array(..) => self.imports.insert(Import::Vec),
                    Type::AccountId => self.imports.insert(Import::AccountId),
                    Type::String => self.imports.insert(Import::String),
                    Type::DynamicBytes => self.imports.insert(Import::Vec),
                    Type::Mapping(_, _) => self.imports.insert(Import::Mapping),
                    _ => true,
                };
                Ok(converted_type)
            }
            SolangExpression::Variable(identifier) => Ok(Type::Variable(identifier.name.clone())),
            SolangExpression::ArraySubscript(_, ty, expression_maybe) => {
                let parsed_type = Box::new(self.parse_type(ty)?);
                let parsed_expression = expression_maybe
                    .as_ref()
                    .map(|option| self.parse_expression(option, VariableAccessLocation::Any));
                Ok(Type::Array(parsed_type, parsed_expression))
            }
            SolangExpression::MemberAccess(_, from, identifier) => {
                let parsed_expression = self.parse_expression(from, VariableAccessLocation::Any);
                let parsed_identifier = self.parse_identifier(&Some(identifier.clone()));
                Ok(Type::MemberAccess(parsed_expression, parsed_identifier))
            }
            _ => {
                println!("{ty:?}");
                Err(ParserError::IncorrectTypeOfVariable)
            }
        }
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

fn function_call_in_expression(expresion: &Expression) -> bool {
    match expresion {
        Expression::Add(expr1, expr2)
        | Expression::And(expr1, expr2)
        | Expression::Assign(expr1, expr2)
        | Expression::AssignAdd(expr1, expr2)
        | Expression::AssignDivide(expr1, expr2)
        | Expression::AssignModulo(expr1, expr2)
        | Expression::AssignMultiply(expr1, expr2)
        | Expression::AssignSubtract(expr1, expr2)
        | Expression::Divide(expr1, expr2)
        | Expression::Equal(expr1, expr2)
        | Expression::Less(expr1, expr2)
        | Expression::LessEqual(expr1, expr2)
        | Expression::Modulo(expr1, expr2)
        | Expression::More(expr1, expr2)
        | Expression::MoreEqual(expr1, expr2)
        | Expression::Multiply(expr1, expr2)
        | Expression::NotEqual(expr1, expr2)
        | Expression::Or(expr1, expr2)
        | Expression::Subtract(expr1, expr2)
        | Expression::ShiftLeft(expr1, expr2)
        | Expression::ShiftRight(expr1, expr2)
        | Expression::BitwiseAnd(expr1, expr2)
        | Expression::BitwiseXor(expr1, expr2)
        | Expression::BitwiseOr(expr1, expr2)
        | Expression::AssignOr(expr1, expr2)
        | Expression::AssignAnd(expr1, expr2)
        | Expression::AssignXor(expr1, expr2)
        | Expression::AssignShiftLeft(expr1, expr2)
        | Expression::AssignShiftRight(expr1, expr2)
        | Expression::Power(expr1, expr2) => {
            function_call_in_expression(expr1) || function_call_in_expression(expr2)
        }
        Expression::List(list) => {
            list.iter()
                .map(function_call_in_expression)
                .any(|output| output)
        }
        Expression::MappingSubscript(expr, list) => {
            list.iter()
                .map(function_call_in_expression)
                .any(|output| output)
                || function_call_in_expression(expr)
        }
        Expression::ArraySubscript(expr1, expr2) => {
            function_call_in_expression(expr1)
                || expr2
                    .as_ref()
                    .map_or(false, |expression| function_call_in_expression(expression))
        }
        Expression::MemberAccess(expr, _) => function_call_in_expression(expr),
        Expression::New(expr)
        | Expression::Not(expr)
        | Expression::Parenthesis(expr)
        | Expression::PostDecrement(expr)
        | Expression::PostIncrement(expr)
        | Expression::PreDecrement(expr)
        | Expression::PreIncrement(expr) => function_call_in_expression(expr),
        Expression::Ternary(expr1, expr2, expr3) => {
            function_call_in_expression(expr1)
                || function_call_in_expression(expr2)
                || function_call_in_expression(expr3)
        }
        Expression::NamedFunctionCall(..) | Expression::FunctionCall(..) => true,
        Expression::Modifier(_, list) => {
            list.iter()
                .map(function_call_in_expression)
                .any(|output| output)
        }
        _ => false,
    }
}
