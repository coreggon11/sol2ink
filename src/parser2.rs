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
        EventDefinition,
        Expression as SolangExpression,
        SourceUnitPart,
        StructDefinition,
        Type,
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
    let name = contract_definition
        .name
        .as_ref()
        .ok_or(ParserError::ContractNameNotFound)?
        .name
        .clone();

    let mut structs: Vec<Struct> = Default::default();
    let mut events: Vec<Event> = Default::default();

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
            ContractPart::EnumDefinition(_) => {}
            ContractPart::ErrorDefinition(_) => {}
            ContractPart::VariableDefinition(_) => {}
            ContractPart::FunctionDefinition(_) => {}
            ContractPart::TypeDefinition(_) => {}
            ContractPart::Using(_) => {}
            ContractPart::StraySemicolon(_) => {}
        }
    }
    // TODO
    // parent = contract_definition.base
    // pub fields: Vec<ContractField>,
    // pub constructor: Function,
    // pub events: Vec<Event>,
    // pub enums: Vec<Enum>,
    // pub structs: Vec<Struct>,
    // pub functions: Vec<Function>,
    // pub imports: HashSet<String>,
    // pub contract_doc: Vec<String>,
    // pub modifiers: Vec<Modifier>,

    Ok(Contract {
        name,
        structs,
        events,
        ..Default::default()
    })
}

fn parse_struct(struct_definition: &StructDefinition) -> Result<Struct, ParserError> {
    let name = &struct_definition
        .name
        .as_ref()
        .ok_or(ParserError::StructNameNotFound)?
        .name;

    let fields: Vec<StructField> = struct_definition
        .fields
        .iter()
        .map(|variable_declaration| {
            let field_type = match &variable_declaration.ty {
                SolangExpression::Type(_, solidity_type) => {
                    Ok(convert_solidity_type(solidity_type))
                }
                SolangExpression::Variable(identifier) => Ok(identifier.name.clone()),
                _ => Err(ParserError::IncorrectTypeOfVariable),
            }
            .ok()?;
            Some(StructField {
                name: variable_declaration
                    .name
                    .as_ref()
                    .ok_or(ParserError::VariableNameNotFound)
                    .ok()?
                    .name
                    .clone(),
                field_type,
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
    let name = &event_definition
        .name
        .as_ref()
        .ok_or(ParserError::EventNameNotFound)?
        .name;

    let fields: Vec<EventField> = event_definition
        .fields
        .iter()
        .map(|variable_declaration| {
            let field_type = match &variable_declaration.ty {
                SolangExpression::Type(_, solidity_type) => {
                    Ok(convert_solidity_type(solidity_type))
                }
                SolangExpression::Variable(identifier) => Ok(identifier.name.clone()),
                _ => Err(ParserError::IncorrectTypeOfVariable),
            }
            .ok()?;
            Some(EventField {
                name: variable_declaration
                    .name
                    .as_ref()
                    .ok_or(ParserError::VariableNameNotFound)
                    .ok()?
                    .name
                    .clone(),
                field_type,
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

fn convert_solidity_type(solidity_type: &Type) -> String {
    match solidity_type {
        Type::Address | Type::AddressPayable => String::from("AccountId"),
        Type::Bool => String::from("bool"),
        Type::String => String::from("String"),
        Type::Int(original_bytes) => {
            let bytes = convert_int_bytes(original_bytes);
            format!("i{bytes}")
        }
        Type::Uint(original_bytes) => {
            let bytes = convert_int_bytes(original_bytes);
            format!("u{bytes}")
        }
        Type::Bytes(bytes) => format!("[u8 ; {bytes} ]"),
        Type::DynamicBytes => String::from("Vec<u8>"),
        Type::Mapping(_, _key_type, _value_type) => todo!(),
        _ => String::default(),
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
