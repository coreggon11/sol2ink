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

#![feature(once_cell)]
#![feature(string_remove_matches)]
#![feature(exclusive_range_pattern)]
#![feature(if_let_guard)]

extern crate core;

pub mod assembler;
pub mod cli;
pub mod file_utils;
pub mod parser;
pub mod structures;
pub mod toml_builder;

use assembler::{
    assemble_lib,
    assemble_mod,
};
use file_utils::{
    create_structure,
    get_solidity_files_from_directory,
    write_mod_files,
};
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
use rbtree::RBTree;
use std::{
    collections::{
        HashMap,
        HashSet,
    },
    path::Path,
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
                let file_path = Path::new(&file);
                let file_home = file_path.parent().unwrap().to_str().unwrap();
                match run(file_home, &[file.clone()]) {
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

                match run(&dir, &paths) {
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
fn run(home: &str, path: &[String]) -> Result<(), ParserError> {
    let mut fields_map = HashMap::new();
    let mut modifier_map = HashMap::new();
    let mut imports = HashSet::new();
    let mut comments = RBTree::new();

    let mut parser = Parser::new(
        &mut fields_map,
        &mut modifier_map,
        &mut imports,
        &mut comments,
    );

    create_structure(home)?;
    let mut impls = Vec::default();
    let mut traits = Vec::default();
    let mut libs = Vec::default();

    for file in path {
        let content = file_utils::read_file(file)?;
        let output = parser.parse_file(&content)?;

        for output in output {
            match output {
                ParserOutput::Contract(name, contract) => {
                    let ink_contract = assembler::assemble_contract(&contract);
                    let implementation = assembler::assemble_impl(&contract);
                    let trait_definition = assembler::assemble_trait(&contract);

                    impls.push(name.clone());
                    traits.push(name.clone());

                    file_utils::write_contract_files(
                        ink_contract,
                        implementation,
                        trait_definition,
                        &contract.name,
                        home,
                    )?;
                    println!("File saved!");
                }
                ParserOutput::Interface(name, interface) => {
                    let ink_trait = assembler::assemble_interface(interface);

                    traits.push(name.clone());

                    file_utils::write_trait(ink_trait, home, &name)?;
                    println!("File saved!");
                }
                ParserOutput::Library(name, library) => {
                    let lib = assembler::assemble_library(library);

                    libs.push(name.clone());

                    file_utils::write_library(lib, home, &name)?;
                    println!("File saved!");
                }
                _ => {}
            }
        }

        parser.clear();
    }

    let impls_mod = assemble_mod(&impls);
    let traits_mod = assemble_mod(&traits);
    let libs_mod = assemble_mod(&libs);
    let lib = assemble_lib();

    write_mod_files(home, impls_mod, traits_mod, libs_mod, lib)?;

    Ok(())
}
