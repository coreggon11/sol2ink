#![feature(string_remove_matches)]
#![feature(if_let_guard)]

extern crate core;

pub mod cli;
pub mod file_utils;
pub mod parser;
pub mod poseidon;
pub mod structures;

use cli::SwitchFlag;
use file_utils::get_solidity_files_from_directory;
use parser::Parser;
use structures::{
    Call,
    CallType,
};

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
use std::collections::HashMap;

/// main function
fn main() {
    let args = cli();

    let inputs = args.inputs.unwrap_or_else(|| {
        eprintln!("No files provided");
        std::process::exit(1);
    });

    if inputs.is_empty() {
        eprintln!("No files provided");
        std::process::exit(1);
    }

    let mut filtered = Vec::default();
    let mut omitted = Vec::default();
    let mut current_flag = SwitchFlag::None;
    let mut omit_read_storage = false;

    for input in inputs.clone() {
        match input {
            CliInput::SwitchFlag(switch_flag) => {
                match switch_flag {
                    SwitchFlag::OmitReadStorage => omit_read_storage = true,
                    _ => current_flag = switch_flag,
                }
            }
            CliInput::SpecificContract(contract) => {
                match current_flag {
                    SwitchFlag::SpecifyContract => filtered.push(contract),
                    SwitchFlag::OmitContract => omitted.push(contract),
                    _ => (),
                }
            }
            _ => (),
        }
    }

    for input in inputs {
        match input {
            CliInput::SolidityFile(file) => {
                match run(
                    &[file.clone()],
                    &filtered.clone(),
                    &omitted.clone(),
                    omit_read_storage,
                ) {
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

                match run(
                    &paths,
                    &filtered.clone(),
                    &omitted.clone(),
                    omit_read_storage,
                ) {
                    Ok(_) => {}
                    Err(err) => {
                        eprintln!("error: {err:?}");
                        std::process::exit(1);
                    }
                }
            }
            _ => (),
        }
    }
}

/// Runs the parser on the selected contracts
///
/// `home` the home directory of a single file, or the directory we are parsing
/// `path` the paths to the files we want to parse
fn run(
    path: &[String],
    contracts: &[String],
    omitted: &[String],
    omit_read_storage: bool,
) -> Result<(), ParserError> {
    initialize_parser!(parser);

    for file in path {
        let content = file_utils::read_file(file)?;

        parser.extract_all_structs(&content)?;
    }

    for file in path {
        let content = file_utils::read_file(file)?;

        parser.extract_storage_pointers(&content)?;
    }

    let mut to_proccess_vec = Vec::default();
    let mut to_proccess_map = HashMap::new();
    let mut outputs = HashMap::new();
    let mut processed_map = HashMap::new();
    let mut processed_vec = Vec::default();
    let mut slots_map: HashMap<String, Vec<String>> = HashMap::new();

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

                        for new_slot in new_contract.slots.clone() {
                            if let Some(fields) = slots_map.get(&new_slot.name) {
                                let mut current_fields = new_slot.fields.clone();
                                current_fields.extend(fields.clone());
                                current_fields.sort();
                                current_fields.dedup();

                                slots_map.insert(new_slot.name.clone(), current_fields);
                            } else {
                                slots_map.insert(new_slot.name.clone(), new_slot.fields);
                            }
                        }

                        let mut new_functions = contract
                            .functions
                            .iter()
                            .map(|function| {
                                let mut new_function = function.clone();

                                new_function.calls = function
                                    .calls
                                    .iter()
                                    .map(|call| {
                                        match call {
                                            Call::Read(call_type, _, _)
                                            | Call::ReadStorage(call_type, _, _)
                                            | Call::WriteStorage(call_type, _, _)
                                            | Call::Write(call_type, _, _) => {
                                                if let CallType::CallingStoragePointer = call_type {
                                                    call.clone()
                                                } else {
                                                    call.change_contract(&new_contract.name.clone())
                                                }
                                            }
                                            Call::Library(_, _) => call.clone(),
                                        }
                                    })
                                    .collect();
                                new_function
                            })
                            .collect();

                        new_contract.functions.append(&mut new_functions);
                        new_contract
                            .modifiers
                            .append(&mut contract.modifiers.clone());
                    }
                }

                if !processed {
                    continue
                }

                // go through Library calls of each of the functions and remap it
                let mut new_functions = Vec::default();

                for function in new_contract.functions.clone() {
                    let mut new_calls = Vec::default();
                    for call in function.calls.clone() {
                        if let Call::Library(library_struct_name, library_function) = call.clone() {
                            // @todo we are optimistic here
                            let library_name =
                                library_struct_name.split('_').next().unwrap_or_default();
                            if processed_map.contains_key(library_name) {
                                processed = false;
                                break
                            }
                            if let Some(ParserOutput::Library(_, contract)) =
                                outputs.get(library_name)
                            {
                                // find the function we are looking for
                                let calls = contract
                                    .functions
                                    .iter()
                                    .filter(|function| function.header.name == library_function)
                                    .flat_map(|function| function.calls.clone())
                                    .collect::<Vec<_>>();

                                // calls might work some storage slots that are not saved yet

                                new_calls.extend(calls);
                            }
                        } else {
                            new_calls.push(call);
                        }
                    }

                    if !processed {
                        break
                    }

                    let mut new_function = function.clone();
                    new_function.calls = new_calls;
                    new_functions.push(new_function);
                }

                if !processed {
                    continue
                }

                new_contract.functions = new_functions;

                if !omitted.contains(&new_contract.name)
                    && (contracts.is_empty() || contracts.contains(&new_contract.name))
                {
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
                if index == to_proccess_vec.len() {
                    index = 0;
                }
            }
            ParserOutput::Library(name, library) => {
                // @todo for now we are only using library functions which manipulate with storage
                // e.g. they take a storage arg and then modify it (write to storage)
                // we handle this in the contract definition, so no action needed here for now
                // maybe later on we will be interetested in library calls

                // in library we want to get rid of `Library` calls then we will consider it processed
                let mut processed = true;
                let mut new_functions = Vec::new();

                for function in library.functions.clone() {
                    let mut new_calls = Vec::new();

                    // first we will expand all the `Library` calls
                    for call in function.calls.clone() {
                        if let Call::Library(library_struct_name, library_function) = call {
                            // @todo we are optimistic here
                            let library_name =
                                library_struct_name.split('_').next().unwrap_or_default();
                            if to_proccess_map.contains_key(library_name) {
                                processed = false;
                                break
                            }

                            if let Some(ParserOutput::Library(_, contract)) =
                                outputs.get(library_name)
                            {
                                // find the function we are looking for
                                let calls = contract
                                    .functions
                                    .iter()
                                    .filter(|function| {
                                        function.header.name.clone() == library_function
                                    })
                                    .flat_map(|function| function.calls.clone())
                                    .collect::<Vec<_>>();

                                new_calls.extend(calls);
                            }
                        } else {
                            new_calls.push(call);
                        }
                    }

                    if !processed {
                        break
                    }

                    let mut new_function = function.clone();
                    new_function.calls = new_calls;
                    new_functions.push(new_function);
                }

                if !processed {
                    continue;
                }

                let mut filtered_functions = Vec::default();
                for function in new_functions.clone() {
                    let mut filtered_function = function.clone();

                    // in case of library we are only interested in `ReadStorage` and `WriteStorage`
                    // so we will filter now
                    while filtered_function.calls.iter().any(|call| {
                        matches!(call, Call::Read(..)) || matches!(call, Call::Write(..))
                    }) {
                        let mut filtered_calls = Vec::new();

                        for call in filtered_calls.clone() {
                            match call {
                                Call::ReadStorage(..) | Call::WriteStorage(..) => {
                                    filtered_calls.push(call)
                                }
                                Call::Read(_, contract, function_name)
                                | Call::Write(_, contract, function_name) => {
                                    if contract == library.name {
                                        // if its the same contract we will look at already processed functions as
                                        // it may contain `Library` calls
                                        let calls = new_functions
                                            .clone()
                                            .iter()
                                            .filter(|function| {
                                                function.header.name.clone() == function_name
                                            })
                                            .flat_map(|function| function.calls.clone())
                                            .collect::<Vec<_>>();

                                        filtered_calls.extend(calls);
                                    } else if to_proccess_map.contains_key(&contract) {
                                        processed = false;
                                        break
                                    } else if let Some(ParserOutput::Library(_, contract)) =
                                        outputs.get(&contract)
                                    {
                                        // find the function we are looking for
                                        let calls = contract
                                            .functions
                                            .iter()
                                            .filter(|function| {
                                                function.header.name.clone() == function_name
                                            })
                                            .flat_map(|function| function.calls.clone())
                                            .collect::<Vec<_>>();

                                        filtered_calls.extend(calls);
                                    }
                                }
                                Call::Library(..) => unreachable!("Should be processed by now"),
                            }
                        }
                        if !processed {
                            break
                        }

                        filtered_function.calls = filtered_calls;
                    }
                    if !processed {
                        break
                    }

                    filtered_functions.push(filtered_function);
                }

                if !processed {
                    continue
                }

                let mut new_library = library.clone();
                new_library.functions = filtered_functions;

                to_proccess_vec.remove(index);
                to_proccess_map.remove(&name.clone());
                outputs.insert(
                    name.clone(),
                    ParserOutput::Library(name.clone(), new_library),
                );
                if index == to_proccess_vec.len() {
                    index = 0;
                }
            }
            _ => (),
        }
    }

    // now we pass processed vec to assembler

    let output = poseidon::generate_mermaid(&processed_vec, &slots_map, omit_read_storage);
    file_utils::write_mermaid(&output)?;

    Ok(())
}
