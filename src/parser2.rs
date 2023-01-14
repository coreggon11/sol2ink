// MIT License

// Copyright (c) 2022 727.ventures

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
use solang_parser::{
    parse,
    pt::{
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
        StructDefinition,
        Type as SolangType,
        VariableAttribute,
        VariableDefinition,
        Visibility,
    },
};

pub enum ParserOutput {
    Contract(Contract),
    Interface(Interface),
    Library(Library),
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

pub fn parse_file(content: &String) -> Result<Vec<ParserOutput>, ParserError> {
    let token_tree = parse(&content, 0).map_err(|_| ParserError::FileCorrupted)?;

    let mut output = Vec::new();
    let source_unit = token_tree.0;
    let _comments = token_tree.1;

    for source_unit_part in source_unit.0.iter() {
        match &source_unit_part {
            SourceUnitPart::ContractDefinition(contract) => {
                output.push(handle_contract_definition(contract)?);
            }
            SourceUnitPart::ImportDirective(_) => println!("import"),
            SourceUnitPart::PragmaDirective(..) => {}
            _ => println!("Found a source unit outside of contract"),
        }
    }

    Ok(output)
}

fn handle_contract_definition(
    contract_definition: &ContractDefinition,
) -> Result<ParserOutput, ParserError> {
    match contract_definition.ty {
        ContractTy::Abstract(_) => {
            unimplemented!(
                "Abstract contract can not be instantiated so we only create impl and trait for it"
            )
        }
        ContractTy::Contract(_) => Ok(ParserOutput::Contract(parse_contract(contract_definition)?)),
        ContractTy::Library(_) => unimplemented!(),
        ContractTy::Interface(_) => unimplemented!(),
    }
}

fn parse_contract(contract_definition: &ContractDefinition) -> Result<Contract, ParserError> {
    let name = parse_identifier(&contract_definition.name);

    let mut structs: Vec<Struct> = Default::default();
    let mut events: Vec<Event> = Default::default();
    let mut enums: Vec<Enum> = Default::default();
    let mut fields: Vec<ContractField> = Default::default();
    let mut functions: Vec<Function> = Default::default();
    let mut constructor: Function = Default::default();
    let mut modifiers: Vec<Function> = Default::default();

    for part in contract_definition.parts.iter() {
        match part {
            ContractPart::Annotation(_) => println!("Anottation: {part:?}"),
            ContractPart::StructDefinition(struct_definition) => {
                let parsed_struct = parse_struct(struct_definition)?;
                structs.push(parsed_struct);
            }
            ContractPart::EventDefinition(event_definition) => {
                let parsed_event = parse_event(event_definition)?;
                events.push(parsed_event);
            }
            ContractPart::EnumDefinition(enum_definition) => {
                let parsed_enum = parse_enum(enum_definition)?;
                enums.push(parsed_enum);
            }
            ContractPart::ErrorDefinition(_) => {}
            ContractPart::VariableDefinition(variable_definition) => {
                let parsed_field = parse_storage_field(variable_definition)?;
                fields.push(parsed_field);
            }
            ContractPart::FunctionDefinition(function_definition) => {
                let parsed_function = parse_function(function_definition)?;
                match function_definition.ty {
                    FunctionTy::Constructor => constructor = parsed_function,
                    FunctionTy::Modifier => modifiers.push(parsed_function),
                    _ => functions.push(parsed_function),
                }
            }
            ContractPart::TypeDefinition(_) => {}
            ContractPart::Using(_) => {}
            ContractPart::StraySemicolon(_) => {}
        }
    }
    // TODO
    // parent = contract_definition.base
    // pub constructor: Function,
    // pub functions: Vec<Function>,
    // pub imports: HashSet<String>,
    // pub contract_doc: Vec<String>,
    // pub modifiers: Vec<Modifier>,

    Ok(Contract {
        name,
        structs,
        events,
        enums,
        fields,
        functions,
        constructor,
        ..Default::default()
    })
}

fn parse_struct(struct_definition: &StructDefinition) -> Result<Struct, ParserError> {
    let name = parse_identifier(&struct_definition.name);

    let fields: Vec<StructField> = struct_definition
        .fields
        .iter()
        .map(|variable_declaration| {
            Some(StructField {
                name: parse_identifier(&variable_declaration.name),
                field_type: parse_type(&variable_declaration.ty).ok()?,
                comments: Default::default(),
            })
        })
        .filter(|maybe| maybe.is_some())
        .map(|option| option.unwrap())
        .collect();
    Ok(Struct {
        name: name.to_string(),
        fields,
        comments: Default::default(),
    })
}

fn parse_event(event_definition: &EventDefinition) -> Result<Event, ParserError> {
    let name = parse_identifier(&event_definition.name);

    let fields: Vec<EventField> = event_definition
        .fields
        .iter()
        .map(|variable_declaration| {
            Some(EventField {
                name: parse_identifier(&variable_declaration.name),
                field_type: parse_type(&variable_declaration.ty).ok()?,
                indexed: variable_declaration.indexed,
                comments: Default::default(),
            })
        })
        .filter(|maybe| maybe.is_some())
        .map(|option| option.unwrap())
        .collect();
    Ok(Event {
        name: name.to_string(),
        fields,
        comments: Default::default(),
    })
}

fn parse_enum(event_definition: &EnumDefinition) -> Result<Enum, ParserError> {
    let name = parse_identifier(&event_definition.name);

    let values: Vec<EnumField> = event_definition
        .values
        .iter()
        .map(|enum_value| {
            Some(EnumField {
                name: parse_identifier(enum_value),
                comments: Default::default(),
            })
        })
        .filter(|maybe| maybe.is_some())
        .map(|option| option.unwrap())
        .collect();
    Ok(Enum {
        name,
        values,
        comments: Default::default(),
    })
}

fn parse_storage_field(
    variable_definition: &VariableDefinition,
) -> Result<ContractField, ParserError> {
    let field_type = parse_type(&variable_definition.ty)?;
    let name = parse_identifier(&variable_definition.name);
    let constant = variable_definition.attrs.iter().any(|item| {
        match item {
            VariableAttribute::Constant(_) => true,
            _ => false,
        }
    });
    let public = variable_definition.attrs.iter().any(|item| {
        match item {
            VariableAttribute::Visibility(Visibility::External(_))
            | VariableAttribute::Visibility(Visibility::Public(_)) => true,
            _ => false,
        }
    });
    let initial_value = None; // TODO
    let comments = Vec::default();
    Ok(ContractField {
        field_type,
        name,
        initial_value,
        constant,
        public,
        comments,
    })
}

fn parse_function(function_definition: &FunctionDefinition) -> Result<Function, ParserError> {
    let name = parse_identifier(&function_definition.name);
    let params = function_definition
        .params
        .iter()
        .map(|item| item.1.clone().unwrap())
        .map(|param| {
            let name = parse_identifier(&param.name);
            let param_type = parse_type(&param.ty).ok()?;
            Some(FunctionParam { name, param_type })
        })
        .filter(|maybe| maybe.is_some())
        .map(|option| option.unwrap())
        .collect();
    let external = function_definition.attributes.iter().any(|attribute| {
        match attribute {
            FunctionAttribute::Visibility(Visibility::External(_))
            | FunctionAttribute::Visibility(Visibility::Public(_)) => true,
            _ => false,
        }
    });
    let view = function_definition.attributes.iter().any(|attribute| {
        match attribute {
            FunctionAttribute::Mutability(Mutability::Pure(_))
            | FunctionAttribute::Mutability(Mutability::View(_)) => true,
            _ => false,
        }
    });
    let payable = function_definition.attributes.iter().any(|attribute| {
        match attribute {
            FunctionAttribute::Mutability(Mutability::Payable(_)) => true,
            _ => false,
        }
    });
    let return_params = function_definition
        .returns
        .iter()
        .map(|item| item.1.clone().unwrap())
        .map(|param| {
            let name = parse_identifier(&param.name);
            let param_type = parse_type(&param.ty).ok()?;
            Some(FunctionParam { name, param_type })
        })
        .filter(|maybe| maybe.is_some())
        .map(|option| option.unwrap())
        .collect();
    // TODO
    let _modifiers = function_definition
        .attributes
        .iter()
        .filter(|&attribute| {
            match attribute {
                FunctionAttribute::BaseOrModifier(..) => true,
                _ => false,
            }
        })
        .map(|modifier| {
            if let FunctionAttribute::BaseOrModifier(_, base) = modifier {
                let _name = parse_identifier_path(&base.name);
                let _args = ();
                // TODO
            } else {
                panic!("This can not happen due to filter before");
            }
        });

    let header = FunctionHeader {
        name,
        params,
        external,
        view,
        payable,
        return_params,
        ..Default::default()
    };

    // TODO: body
    return Ok(Function {
        header,
        body: Vec::default(),
    })
}

fn parse_identifier(variable_declaration: &Option<Identifier>) -> String {
    match variable_declaration {
        Some(identifier) => identifier.name.clone(),
        None => String::from("_"),
    }
}

fn parse_identifier_path(identifier_path: &IdentifierPath) -> String {
    identifier_path
        .identifiers
        .iter()
        .map(|identifier| identifier.name.clone())
        .collect::<Vec<String>>()
        .join(".")
        .to_string()
}

fn parse_type(ty: &SolangExpression) -> Result<Type, ParserError> {
    match &ty {
        SolangExpression::Type(_, SolangType::Mapping(_, key_type, value_type)) => {
            let mut parsed_key_types = vec![parse_type(key_type)?];
            let mut value_type_now = value_type.as_ref();
            while let SolangExpression::Type(
                _,
                SolangType::Mapping(_, key_type_value, value_type_value),
            ) = value_type_now
            {
                parsed_key_types.push(parse_type(&key_type_value)?);
                value_type_now = value_type_value;
            }
            let parsed_value_type = parse_type(&value_type_now)?;
            Ok(Type::Mapping(parsed_key_types, Box::new(parsed_value_type)))
        }
        SolangExpression::Type(_, solidity_type) => Ok(convert_solidity_type(solidity_type)),
        SolangExpression::Variable(identifier) => Ok(Type::Variable(identifier.name.clone())),
        _ => Err(ParserError::IncorrectTypeOfVariable),
    }
}

fn convert_solidity_type(solidity_type: &SolangType) -> Type {
    match solidity_type {
        SolangType::Address | SolangType::AddressPayable => Type::AccountId,
        SolangType::Bool => Type::Bool,
        SolangType::String => Type::String,
        SolangType::Int(original_bytes) => Type::Int(convert_int_bytes(original_bytes)),
        SolangType::Uint(original_bytes) => Type::Uint(convert_int_bytes(original_bytes)),
        SolangType::Bytes(length) => Type::Bytes(*length),
        SolangType::DynamicBytes => Type::DynamicBytes,
        _ => Type::None,
    }
}

fn convert_int_bytes(original_bytes: &u16) -> u16 {
    match *original_bytes {
        i if i <= 8 => 8,
        i if i <= 16 => 16,
        i if i <= 32 => 32,
        i if i <= 64 => 64,
        _ => 128,
    }
}
