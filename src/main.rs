#![feature(string_remove_matches)]
#![feature(if_let_guard)]

extern crate core;

pub mod cli;
pub mod file_utils;
pub mod parser;
pub mod poseidon;
pub mod structures;

use file_utils::get_solidity_files_from_directory;
use parser::Parser;

use crate::{
    cli::{
        cli,
        CliInput,
    },
    parser::{
        ParserError,
        ParserOutput,
    },
};
use std::collections::{
    HashMap,
    HashSet,
};

/// main function
fn main() {
    let args = cli();

    let files = args.files.unwrap_or_else(|| {
        eprintln!("No files provided");
        std::process::exit(1);
    });

    if files.is_empty() {
        eprintln!("No files provided");
        std::process::exit(1);
    }

    for file in files {
        match file {
            CliInput::SolidityFile(file) => {
                match run(&[file.clone()]) {
                    Ok(_) => {
                        println!("Successfully parsed {file}");
                    }
                    Err(err) => {
                        eprintln!("error: {err:?}");
                        std::process::exit(1);
                    }
                }
            }
            CliInput::Directory(dir) => {
                let paths = get_solidity_files_from_directory(&dir)
                    .unwrap_or_else(|err| panic!("error: {err:?}"));

                match run(&paths) {
                    Ok(_) => {}
                    Err(err) => {
                        eprintln!("error: {err:?}");
                        std::process::exit(1);
                    }
                }
            }
        }
    }
}

/// Runs the parser on the selected contracts
///
/// `home` the home directory of a single file, or the directory we are parsing
/// `path` the paths to the files we want to parse
fn run(path: &[String]) -> Result<(), ParserError> {
    initialize_parser!(parser);

    let mut to_proccess_vec = Vec::default();
    let mut to_proccess_map = HashMap::new();
    let mut outputs = HashMap::new();
    let mut processed_map = HashMap::new();
    let mut processed_vec = Vec::default();

    for file in path {
        let content = file_utils::read_file(file)?;
        let output = parser.parse_file(&content)?;

        for parser_output in output {
            match parser_output.clone() {
                ParserOutput::Contract(name, _)
                | ParserOutput::Interface(name, _)
                | ParserOutput::Library(name, _) => {
                    to_proccess_map.insert(name.clone(), ());
                    to_proccess_vec.push(name.clone());
                    outputs.insert(name.clone(), parser_output.clone());
                }
                ParserOutput::None => (),
            }
        }

        parser.clear();
    }

    let mut index = 0;
    while !to_proccess_vec.is_empty() {
        let to_proccess = to_proccess_vec.get(index).unwrap();
        let parser_output = outputs.get(to_proccess).unwrap();

        // go through bases and check if processed

        match parser_output {
            ParserOutput::Contract(name, contract) => {
                let mut new_contract = contract.clone();
                let mut processed = true;

                for base in contract.base.clone() {
                    // if the base needs to be processed process it
                    if to_proccess_map.contains_key(&base) {
                        index += 1;
                        if index == to_proccess_vec.len() {
                            index = 0
                        }
                        processed = false;
                        break;
                    }
                    // else we can add its functions to the contract
                    if let Some(ParserOutput::Contract(_, contract)) = outputs.get(&base) {
                        new_contract.fields.append(&mut contract.fields.clone());
                        new_contract
                            .functions
                            .append(&mut contract.functions.clone());
                        new_contract
                            .modifiers
                            .append(&mut contract.modifiers.clone());
                    }
                }
                if !processed {
                    continue
                }
                if !contract.is_abstract {
                    processed_vec.push(new_contract.clone());
                }
                processed_map.insert(name.clone(), contract.clone());
                to_proccess_vec.remove(index);
                to_proccess_map.remove(&name.clone());
                outputs.insert(
                    name.clone(),
                    ParserOutput::Contract(name.clone(), new_contract),
                );
                if index == to_proccess_vec.len() {
                    index = 0;
                }
            }
            ParserOutput::Interface(name, _interface) => {
                //@todo dont care for now
                to_proccess_vec.remove(index);
                to_proccess_map.remove(&name.clone());
            }
            ParserOutput::Library(name, _library) => {
                //@todo dont care for now
                to_proccess_vec.remove(index);
                to_proccess_map.remove(&name.clone());
            }
            _ => (),
        }
    }

    // now we pass processed vec to assembler

    let output = poseidon::generate_mermaid(processed_vec);
    file_utils::write_mermaid(output)?;

    Ok(())
}
