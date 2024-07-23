use crate::structures::*;
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
        StorageLocation,
        StructDefinition,
        VariableAttribute,
        VariableDefinition,
        Visibility,
        YulExpression,
        YulStatement,
    },
};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum ParserOutput {
    Contract(String, Contract),
    Interface(String, Interface),
    Library(String, Contract),
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
    // mapping function => struct return
    storage_pointers: &'a mut HashMap<String, String>,
    // Contract.Struct => Struct
    structs: &'a mut HashMap<String, StructDefinition>,
    // declaration => Contract.Struct
    local_storage_pointers: &'a mut HashMap<String, String>,
    // depth => [declaration_0, declaration_1 ...]
    local_storage_pointers_declared: &'a mut HashMap<u8, Vec<String>>,
    // Contract.Struct => [field_0, field_1 ...]
    storage_access: &'a mut HashMap<String, Vec<String>>,
    current_depth: u8,
    current_contract: String,
}

impl<'a> Parser<'a> {
    pub fn new(
        members_map: &'a mut HashMap<String, MemberType>,
        modifiers_map: &'a mut HashMap<String, FunctionDefinition>,
        storage_pointers: &'a mut HashMap<String, String>,
        structs: &'a mut HashMap<String, StructDefinition>,
        local_variables: &'a mut HashMap<String, String>,
        local_variables_declared: &'a mut HashMap<u8, Vec<String>>,
        storage_access: &'a mut HashMap<String, Vec<String>>,
    ) -> Self {
        Parser {
            members_map,
            modifiers_map,
            storage_pointers,
            structs,
            local_storage_pointers: local_variables,
            local_storage_pointers_declared: local_variables_declared,
            storage_access,
            current_depth: 0,
            current_contract: String::new(),
        }
    }

    pub fn clear(&mut self) {
        self.members_map.clear();
        self.modifiers_map.clear();
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
            if let SourceUnitPart::ContractDefinition(contract) = source_unit_part {
                output.push(self.handle_contract_definition(contract)?);
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
                let parsed_library = self.parse_contract(contract_definition)?;
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

    pub fn extract_storage_pointers(&mut self, content: &str) -> Result<(), ParserError> {
        let token_tree = parse(content, 0).map_err(|errors| {
            ParserError::FileCorrupted(errors.iter().map(|error| error.message.clone()).collect())
        })?;

        let source_unit = token_tree.0;

        for source_unit_part in source_unit.0.iter() {
            if let SourceUnitPart::ContractDefinition(contract_definition) = source_unit_part {
                let contract_name = self.parse_identifier(&contract_definition.name.clone());

                for part in contract_definition.parts.iter() {
                    if let ContractPart::FunctionDefinition(function_definition) = part {
                        // if function returns a storage pointer
                        let return_param = function_definition
                            .returns
                            .iter()
                            .filter_map(|tuple| tuple.1.clone())
                            .filter(|param| param.storage.is_some())
                            .find(|param| {
                                matches!(
                                    param.storage.clone().unwrap(),
                                    StorageLocation::Storage(_)
                                )
                            });
                        if let Some(return_param) = return_param {
                            let function_header = self.parse_function_header(function_definition);

                            if let SolangExpression::Variable(ident) = return_param.ty {
                                let parsed_ident = self.parse_identifier(&Some(ident));
                                self.storage_pointers.insert(
                                    format!("{contract_name}_{}", function_header.name),
                                    format!("{contract_name}_{parsed_ident}"),
                                );
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub fn extract_all_structs(&mut self, content: &str) -> Result<(), ParserError> {
        let token_tree = parse(content, 0).map_err(|errors| {
            ParserError::FileCorrupted(errors.iter().map(|error| error.message.clone()).collect())
        })?;

        let source_unit = token_tree.0;

        for source_unit_part in source_unit.0.iter() {
            if let SourceUnitPart::ContractDefinition(contract_definition) = source_unit_part {
                let contract_name = self.parse_identifier(&contract_definition.name);

                // first we need to know functions that exist
                for part in contract_definition.parts.iter() {
                    if let ContractPart::StructDefinition(struct_definition) = part {
                        // we will save struct definitions as structs might be used as storage containers
                        let name = self.parse_identifier(&struct_definition.name);
                        // panic if unnamed struct
                        self.structs.insert(
                            format!("{}_{}", contract_name, name.clone()),
                            *struct_definition.clone(),
                        );
                    }
                }
            }
        }

        Ok(())
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
        let contract_name = self.parse_identifier(&contract_definition.name);

        self.current_contract = contract_name.clone();

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
                    self.members_map.insert(
                        name.clone(),
                        MemberType::StorageField(contract_name.clone()),
                    );
                }
                ContractPart::FunctionDefinition(function_definition) => {
                    // @todo function might take storage as input param
                    let fn_name = self.parse_identifier(&function_definition.name);
                    match function_definition.ty {
                        FunctionTy::Function => {
                            let function_header = self.parse_function_header(function_definition);
                            self.members_map.insert(
                                fn_name.clone(),
                                MemberType::Function(function_header, contract_name.clone()),
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

        let slots = self
            .storage_access
            .iter()
            .map(|entry| StorageSlot::new(entry.0.clone(), entry.1.clone()))
            .collect();

        Ok(Contract {
            name: contract_name,
            fields,
            slots,
            functions,
            constructor,
            modifiers,
            base,
            is_abstract: matches!(contract_definition.ty, ContractTy::Abstract(_)),
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
            if let ContractPart::FunctionDefinition(function_definition) = part {
                if function_definition.ty == FunctionTy::Function {
                    let header = self.parse_function_header(function_definition);
                    function_headers.push(header);
                }
            }
        }

        Ok(Interface {
            name,
            function_headers,
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

        // function can have storage parameters, we will save them to stack
        let storage_params_declared = function_definition
            .params
            .iter()
            .filter_map(|tuple| tuple.1.clone())
            .filter(|param| param.storage.is_some() && param.name.is_some())
            .filter(|param| matches!(param.storage.clone().unwrap(), StorageLocation::Storage(_)))
            .filter_map(|param| {
                // map to Option<(param_name, param_type)>
                let raw_expression = param.ty;
                let param_type = match raw_expression {
                    SolangExpression::MemberAccess(_, left, right) => {
                        // left should be a variable
                        // @todo handle cases when it is member access etc...
                        let parsed_right = self.parse_identifier(&Some(right));
                        if let SolangExpression::Variable(identifier) = *left.clone() {
                            let parsed_identifier = self.parse_identifier(&Some(identifier));
                            format!("{parsed_identifier}_{parsed_right}")
                        } else {
                            return None
                        }
                    }
                    SolangExpression::Variable(identifier) => {
                        let parsed_identifier = self.parse_identifier(&Some(identifier));
                        format!("{}_{parsed_identifier}", self.current_contract)
                    }
                    _ => return None,
                };
                let parsed_param_name = self.parse_identifier(&param.name);

                Some((parsed_param_name, param_type))
            })
            .collect::<Vec<_>>();

        let local_storage_pointers_declared = storage_params_declared
            .iter()
            .map(|(param_name, _)| param_name.clone())
            .collect();

        for (param_name, param_type) in storage_params_declared {
            self.local_storage_pointers.insert(param_name, param_type);
        }
        self.local_storage_pointers_declared
            .insert(0, local_storage_pointers_declared);

        let calls = if let Some(statement) = &function_definition.body {
            self.parse_statement(statement)?
        } else {
            Vec::default()
        };

        self.local_storage_pointers.clear();
        self.local_storage_pointers_declared.clear();

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
                self.current_depth += 1;
                let out = statements
                    .iter()
                    .flat_map(|statement| self.parse_statement(statement).unwrap_or_default())
                    .collect::<Vec<_>>();
                if let Some(local_storage_pointers_declared) = self
                    .local_storage_pointers_declared
                    .get(&self.current_depth)
                {
                    for local_storage_pointer_declared in local_storage_pointers_declared {
                        self.local_storage_pointers
                            .remove(local_storage_pointer_declared);
                    }
                    self.local_storage_pointers_declared
                        .remove(&self.current_depth);
                }
                self.current_depth -= 1;
                out
            }
            SolangStatement::Assembly {
                loc: _,
                dialect: _,
                flags: _,
                block,
            } => {
                block
                    .statements
                    .iter()
                    .flat_map(|statement| self.parse_yul_statement(&statement.clone()))
                    .collect()
            }
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
            SolangStatement::VariableDefinition(_, definition, initial_value_maybe) => {
                if definition
                    .storage
                    .clone()
                    .filter(|storage| matches!(storage, StorageLocation::Storage(_)))
                    .is_some()
                {
                    let mut local_storage_pointers_declared = self
                        .local_storage_pointers_declared
                        .get(&self.current_depth)
                        .unwrap_or(&vec![])
                        .clone();
                    match definition.ty.clone() {
                        // @todo handle cases where we store mpping, structs etc.
                        SolangExpression::Variable(variable_type) => {
                            let parsed_variable_type = self.parse_identifier(&Some(variable_type));
                            let full_variable_type =
                                format!("{}_{}", self.current_contract, parsed_variable_type);
                            let variable_name = self.parse_identifier(&definition.name.clone());
                            self.local_storage_pointers
                                .insert(variable_name.clone(), full_variable_type);
                            local_storage_pointers_declared.push(variable_name);
                        }
                        SolangExpression::MemberAccess(_, left, right) => {
                            if let SolangExpression::Variable(left_ident) = *left.clone() {
                                let parsed_left = self.parse_identifier(&Some(left_ident));
                                let parsed_right = self.parse_identifier(&Some(right));
                                let variable_name = self.parse_identifier(&definition.name.clone());
                                self.local_storage_pointers.insert(
                                    variable_name.clone(),
                                    format!("{parsed_left}_{parsed_right}"),
                                );
                                local_storage_pointers_declared.push(variable_name);
                            }
                        }
                        _ => (),
                    };
                    self.local_storage_pointers_declared
                        .insert(self.current_depth, local_storage_pointers_declared);
                }

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

    fn parse_yul_statement(&self, yul_statement: &YulStatement) -> Vec<Call> {
        match yul_statement {
            YulStatement::Assign(_, yul_expressions, yul_expression) => {
                let mut expressions = yul_expressions
                    .iter()
                    .flat_map(|yul_expression| self.parse_yul_expression(&yul_expression.clone()))
                    .collect::<Vec<_>>();
                let expression = self.parse_yul_expression(&yul_expression.clone());

                expressions.extend(expression);

                expressions
            }
            YulStatement::If(_, yul_expression, yul_block) => {
                let mut yul_expression = self.parse_yul_expression(&yul_expression.clone());
                let yul_block = yul_block
                    .statements
                    .iter()
                    .flat_map(|statement| self.parse_yul_statement(&statement.clone()))
                    .collect::<Vec<_>>();

                yul_expression.extend(yul_block);

                yul_expression
            }
            YulStatement::For(yul_for) => {
                let mut init_block = yul_for
                    .init_block
                    .statements
                    .iter()
                    .flat_map(|statement| self.parse_yul_statement(&statement.clone()))
                    .collect::<Vec<_>>();

                let expression = self.parse_yul_expression(&yul_for.condition.clone());

                let post_block = yul_for
                    .post_block
                    .statements
                    .iter()
                    .flat_map(|statement| self.parse_yul_statement(&statement.clone()))
                    .collect::<Vec<_>>();

                let execution_block = yul_for
                    .execution_block
                    .statements
                    .iter()
                    .flat_map(|statement| self.parse_yul_statement(&statement.clone()))
                    .collect::<Vec<_>>();

                init_block.extend(expression);
                init_block.extend(post_block);
                init_block.extend(execution_block);

                init_block
            }
            YulStatement::Switch(yul_switch) => {
                let mut condition = self.parse_yul_expression(&yul_switch.condition.clone());
                let cases = yul_switch
                    .cases
                    .iter()
                    .flat_map(|case| {
                        match case {
                            solang_parser::pt::YulSwitchOptions::Case(
                                _,
                                yul_expression,
                                yul_block,
                            ) => {
                                let mut yul_expression =
                                    self.parse_yul_expression(&yul_expression.clone());

                                let yul_block = yul_block
                                    .statements
                                    .iter()
                                    .flat_map(|statement| {
                                        self.parse_yul_statement(&statement.clone())
                                    })
                                    .collect::<Vec<_>>();

                                yul_expression.extend(yul_block);

                                yul_expression
                            }
                            solang_parser::pt::YulSwitchOptions::Default(_, yul_block) => {
                                yul_block
                                    .statements
                                    .iter()
                                    .flat_map(|statement| {
                                        self.parse_yul_statement(&statement.clone())
                                    })
                                    .collect::<Vec<_>>()
                            }
                        }
                    })
                    .collect::<Vec<_>>();
                let default = yul_switch
                    .default
                    .clone()
                    .map(|case| {
                        match case {
                            solang_parser::pt::YulSwitchOptions::Case(
                                _,
                                yul_expression,
                                yul_block,
                            ) => {
                                let mut yul_expression =
                                    self.parse_yul_expression(&yul_expression.clone());

                                let yul_block = yul_block
                                    .statements
                                    .iter()
                                    .flat_map(|statement| {
                                        self.parse_yul_statement(&statement.clone())
                                    })
                                    .collect::<Vec<_>>();

                                yul_expression.extend(yul_block);

                                yul_expression
                            }
                            solang_parser::pt::YulSwitchOptions::Default(_, yul_block) => {
                                yul_block
                                    .statements
                                    .iter()
                                    .flat_map(|statement| {
                                        self.parse_yul_statement(&statement.clone())
                                    })
                                    .collect::<Vec<_>>()
                            }
                        }
                    })
                    .unwrap_or_default();

                condition.extend(cases);
                condition.extend(default);

                condition
            }
            YulStatement::Block(yul_block) => {
                yul_block
                    .statements
                    .iter()
                    .flat_map(|statement| self.parse_yul_statement(&statement.clone()))
                    .collect()
            }
            _ => Vec::default(),
        }
    }

    fn parse_yul_expression(&self, yul_expression: &YulExpression) -> Vec<Call> {
        match yul_expression {
            YulExpression::BoolLiteral(..)
            | YulExpression::NumberLiteral(..)
            | YulExpression::HexNumberLiteral(..)
            | YulExpression::HexStringLiteral(..)
            | YulExpression::StringLiteral(..) => Vec::default(),
            YulExpression::Variable(identifier) => {
                let parsed_identifier = self.parse_identifier(&Some(identifier.clone()));
                if let Some(member) = self.members_map.get(&parsed_identifier) {
                    match member {
                        MemberType::StorageField(contract_name) => {
                            vec![Call::ReadStorage(
                                CallType::CallingStorage,
                                contract_name.clone(),
                                parsed_identifier,
                            )]
                        }
                        MemberType::Function(function_header, contract_name) => {
                            let call_type = CallType::CallingFunction;

                            if function_header.view {
                                vec![Call::Read(
                                    call_type,
                                    contract_name.clone(),
                                    parsed_identifier,
                                )]
                            } else {
                                vec![Call::Write(
                                    call_type,
                                    contract_name.clone(),
                                    parsed_identifier,
                                )]
                            }
                        }
                    }
                } else {
                    Vec::default()
                }
            }
            YulExpression::FunctionCall(yul_function_call) => {
                yul_function_call
                    .arguments
                    .iter()
                    .flat_map(|arg| self.parse_yul_expression(&arg.clone()))
                    .collect()
            }
            YulExpression::SuffixAccess(_, expression, _) => {
                self.parse_yul_expression(&expression.as_ref().clone())
            }
        }
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
            | SolangExpression::PreIncrement(_, expression)
            | SolangExpression::PreDecrement(_, expression)
            | SolangExpression::Delete(_, expression) => {
                boxed_expression!(parsed_expression, expression)
                    .iter()
                    .map(|call| {
                        match call.clone() {
                            Call::Read(call_type, contract, read) => {
                                Call::Read(call_type, contract, read)
                            }
                            Call::ReadStorage(call_type, contract, read) => {
                                Call::WriteStorage(call_type, contract, read)
                            }
                            Call::Write(..) | Call::WriteStorage(..) => call.clone(),
                            _ => unreachable!("This should be unreachable"),
                        }
                    })
                    .collect()
            }
            SolangExpression::New(_, expression)
            | SolangExpression::Parenthesis(_, expression)
            | SolangExpression::Not(_, expression)
            | SolangExpression::BitwiseNot(_, expression)
            | SolangExpression::UnaryPlus(_, expression)
            | SolangExpression::Negate(_, expression) => {
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
            SolangExpression::MemberAccess(_, left, right) => {
                let mut expressions = boxed_expression!(parsed_expression, left);
                let parsed_right = self.parse_identifier(&Some(right.clone()));
                let mut success = false;

                if let SolangExpression::Variable(left_ident) = *left.clone() {
                    let parsed_left = self.parse_identifier(&Some(left_ident.clone()));

                    if let Some(storage_pointer) = self.local_storage_pointers.get(&parsed_left) {
                        // @todo right can be lib function of this struct

                        if self.structs.get(storage_pointer).is_some() {
                            expressions.extend(vec![Call::ReadStorage(
                                CallType::CallingStoragePointer,
                                storage_pointer.clone(),
                                parsed_right.clone(),
                            )]);
                            success = true;

                            let current_storage_maybe =
                                self.storage_access.get(&storage_pointer.clone());

                            if let Some(current_storage) = current_storage_maybe {
                                let mut new_storage_access = vec![parsed_right.clone()];
                                new_storage_access.extend(current_storage.clone());
                                self.storage_access
                                    .insert(storage_pointer.clone(), new_storage_access);
                            } else {
                                self.storage_access
                                    .insert(storage_pointer.clone(), vec![parsed_right.clone()]);
                            }
                        }
                    }
                }

                if !success {
                    if let Some(member_type) = self.members_map.get(&parsed_right) {
                        match member_type {
                            MemberType::StorageField(contract_name) => {
                                expressions.extend(vec![Call::ReadStorage(
                                    CallType::CallingStorage,
                                    contract_name.clone(),
                                    parsed_right,
                                )])
                            }
                            MemberType::Function(function_header, contract_name) => {
                                let call_type = CallType::CallingFunction;

                                if function_header.view {
                                    expressions.extend(vec![Call::Read(
                                        call_type,
                                        contract_name.clone(),
                                        parsed_right,
                                    )])
                                } else {
                                    expressions.extend(vec![Call::Write(
                                        call_type,
                                        contract_name.clone(),
                                        parsed_right,
                                    )])
                                }
                            }
                        }
                    }
                }

                expressions
            }
            SolangExpression::FunctionCall(_, function, args) => {
                // First we will handle case when we call a Library function of a storage pointer struct

                if let SolangExpression::MemberAccess(_, left, right) = *function.clone() {
                    // if on the left we have a variable definition
                    if let SolangExpression::Variable(left_ident) = *left.clone() {
                        let parsed_left = self.parse_identifier(&Some(left_ident.clone()));
                        // if on the left we have a storage pointer
                        if let Some(storage_pointer) =
                            self.local_storage_pointers.get(&parsed_left).cloned()
                        {
                            // on the right side we have the function name
                            let parsed_right = self.parse_identifier(&Some(right.clone()));

                            // we have the Library function
                            let mut parsed_args = self.parse_expression_vec(args);
                            parsed_args.push(Call::Library(storage_pointer.clone(), parsed_right));

                            return parsed_args
                        }
                    }
                }

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

                // if left is a storage field we are updating storage
                uno = uno
                    .iter()
                    .map(|call| {
                        match call.clone() {
                            Call::Read(call_type, contract, read) => {
                                Call::Read(call_type, contract, read)
                            }
                            Call::ReadStorage(call_type, contract, read) => {
                                Call::WriteStorage(call_type, contract, read)
                            }
                            Call::Write(..) | Call::WriteStorage(..) => call.clone(),
                            _ => unreachable!("Should be unreachable"),
                        }
                    })
                    .collect();

                uno.extend(dos);

                uno
            }
            SolangExpression::Variable(identifier) => {
                let parsed_identifier = self.parse_identifier(&Some(identifier.clone()));

                if let Some(member_type) = self.members_map.get(&parsed_identifier) {
                    match member_type {
                        MemberType::StorageField(contract_name) => {
                            vec![Call::ReadStorage(
                                CallType::CallingStorage,
                                contract_name.clone(),
                                parsed_identifier,
                            )]
                        }
                        MemberType::Function(function_header, contract_name) => {
                            if function_header.view {
                                vec![Call::Read(
                                    CallType::CallingFunction,
                                    contract_name.clone(),
                                    parsed_identifier,
                                )]
                            } else {
                                vec![Call::Write(
                                    CallType::CallingFunction,
                                    contract_name.clone(),
                                    parsed_identifier,
                                )]
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
        let mut storage_pointers = HashMap::new();
        let mut structs = HashMap::new();
        let mut local_variables = HashMap::new();
        let mut local_variables_declared = HashMap::new();
        let mut storage_access = HashMap::new();

        let mut $parser = Parser::new(
            &mut fields_map,
            &mut modifier_map,
            &mut storage_pointers,
            &mut structs,
            &mut local_variables,
            &mut local_variables_declared,
            &mut storage_access,
        );
    };
}
