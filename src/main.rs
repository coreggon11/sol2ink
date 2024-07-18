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

#![feature(string_remove_matches)]
#![feature(if_let_guard)]

extern crate core;

pub mod cli;
pub mod file_utils;
pub mod parser;
pub mod structures;
pub mod toml_builder;

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
                let file_path = Path::new(&file).canonicalize().unwrap();
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
    initialize_parser!(parser);

    let mut impls = Vec::default();
    let mut traits = Vec::default();
    let mut libs = Vec::default();

    for file in path {
        let content = file_utils::read_file(file)?;
        let output = parser.parse_file(&content)?;

        for output in output {
            match output {
                ParserOutput::Contract(name, contract) => {
                    impls.push(name.clone());
                    traits.push(name.clone());

                    println!("File saved!");
                }
                ParserOutput::Interface(name, interface) => {
                    traits.push(name.clone());

                    println!("File saved!");
                }
                ParserOutput::Library(name, library) => {
                    libs.push(name.clone());

                    println!("File saved!");
                }
                _ => {}
            }
        }

        parser.clear();
    }

    Ok(())
}
