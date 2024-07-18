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

use crate::toml_builder;
use convert_case::{
    Case::Snake,
    Casing,
};
use proc_macro2::TokenStream;
use rust_format::{
    Config,
    Formatter,
    PostProcess,
    RustFmt,
};
use std::{
    fs::{
        create_dir_all,
        metadata,
        File,
    },
    io::{
        prelude::*,
        BufReader,
    },
    path::Path,
};

static CONTRACTS_DIR: &str = "/generated/contracts";
static IMPLS_DIR: &str = "/generated/src/impls";
static TRAITS_DIR: &str = "/generated/src/traits";
static LIBS_DIR: &str = "/generated/src/libs";

/// Reads the file to be transpiled and returns its content as a String
///
/// `path` the path to the file
pub fn read_file(path: &String) -> std::io::Result<String> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}

/// Returns the paths to all Solidity files within a directory
///
/// `dir` the directory we want to search
pub fn get_solidity_files_from_directory(dir: &str) -> std::io::Result<Vec<String>> {
    let directory = Path::new(&dir).read_dir().unwrap();

    let mut paths = Vec::default();
    for file in directory {
        let directory = file.unwrap();
        let path = directory.path();
        let file = path.to_str().unwrap();

        if file.ends_with(".sol") {
            paths.push(file.to_string());
        } else if let Ok(metadata) = metadata(&path) {
            if metadata.is_dir() {
                let mut new_paths = get_solidity_files_from_directory(file)?;
                paths.append(&mut new_paths);
            }
        }
    }
    Ok(paths)
}

/// writes the output trait to a file
///
/// `tokens` the transpiled file in the form of TokenStream
/// `file_home` the home directory of the file we are parsing, or the directory we are parsing
/// `trait_name` the name of the trait we are writing
pub fn write_trait(
    tokens: TokenStream,
    file_home: &str,
    trait_name: &String,
) -> std::io::Result<()> {
    let mut file = File::create(format!("{file_home}{TRAITS_DIR}/{trait_name}.rs"))?;
    let config = Config::new_str().post_proc(PostProcess::ReplaceMarkersAndDocBlocks);
    file.write_all(
        RustFmt::from_config(config)
            .format_tokens(tokens)
            .unwrap()
            .as_bytes(),
    )?;

    Ok(())
}

/// generates the mod files for the result project
///
/// `file_home` the home directory of the file we are parsing, or the directory we are parsing
/// `impls` the mod file of the impls folder in the form of TokenStream
/// `traits` the mod file of the traits folder in the form of TokenStream
/// `libs` the mod file of the libs folder in the form of TokenStream
/// `lib` the main lib file in the form of TokenStream
pub fn write_mod_files(
    file_home: &str,
    impls: TokenStream,
    traits: TokenStream,
    libs: TokenStream,
    lib: TokenStream,
) -> std::io::Result<()> {
    let config = Config::new_str().post_proc(PostProcess::ReplaceMarkersAndDocBlocks);

    let mut impls_file = File::create(format!("{file_home}{IMPLS_DIR}/mod.rs"))?;
    impls_file.write_all(
        RustFmt::from_config(config.clone())
            .format_tokens(impls)
            .unwrap()
            .as_bytes(),
    )?;

    let mut traits_file = File::create(format!("{file_home}{TRAITS_DIR}/mod.rs"))?;
    traits_file.write_all(
        RustFmt::from_config(config.clone())
            .format_tokens(traits)
            .unwrap()
            .as_bytes(),
    )?;

    let mut libs_file = File::create(format!("{file_home}{LIBS_DIR}/mod.rs"))?;
    libs_file.write_all(
        RustFmt::from_config(config.clone())
            .format_tokens(libs)
            .unwrap()
            .as_bytes(),
    )?;

    let mut main_lib_file = File::create(format!("{file_home}/generated/src/lib.rs"))?;
    main_lib_file.write_all(
        RustFmt::from_config(config)
            .format_tokens(lib)
            .unwrap()
            .as_bytes(),
    )?;

    let mut main_cargo_toml = File::create(format!("{file_home}/generated/src/Cargo.toml"))?;
    // main_cargo_toml.write_all(toml_builder::generate_mermaid("generated", None).as_bytes())?;

    Ok(())
}

/// writes the output library to a file
///
/// `tokens` the transpiled file in the form of TokenStream
/// `file_home` the home directory of the file we are parsing, or the directory we are parsing
/// `lib_name` the name of the library we are writing
pub fn write_library(
    lines: TokenStream,
    file_home: &str,
    lib_name: &String,
) -> std::io::Result<()> {
    let mut file = File::create(format!("{file_home}{LIBS_DIR}/{lib_name}.rs"))?;
    let config = Config::new_str().post_proc(PostProcess::ReplaceMarkersAndDocBlocks);
    file.write_all(
        RustFmt::from_config(config)
            .format_tokens(lines)
            .unwrap()
            .as_bytes(),
    )?;

    Ok(())
}

/// generates the file structure of an ink! contract
///
/// `contract` the ink! contract file in the for of TokenStream
/// `implementation` the impl file of ink! contract in the for of TokenStream
/// `trait_definition` the trait definition file of ink! contract in the for of TokenStream
/// `contract_name_raw` the name of the original contract
/// `home_path` the home directory of the file we are parsing, or the directory we are parsing
pub fn write_contract_files(
    contract: TokenStream,
    implementation: TokenStream,
    trait_definition: TokenStream,
    contract_name_raw: &String,
    home_path: &str,
) -> std::io::Result<()> {
    let contract_name = contract_name_raw;
    let config = Config::new_str().post_proc(PostProcess::ReplaceMarkersAndDocBlocks);
    let rust_fmt = RustFmt::from_config(config);
    let contract_folder_path = format!("{home_path}{CONTRACTS_DIR}/{contract_name}/");

    create_dir_all(&contract_folder_path)?;

    // contract
    let mut contract_file = File::create(format!("{contract_folder_path}lib.rs"))?;
    contract_file.write_all(rust_fmt.format_tokens(contract).unwrap().as_bytes())?;

    let mut cargo_toml = File::create(format!("{contract_folder_path}/Cargo.toml"))?;

    // impl
    let mut impl_file = File::create(format!("{home_path}{IMPLS_DIR}/{contract_name}.rs"))?;
    impl_file.write_all(rust_fmt.format_tokens(implementation).unwrap().as_bytes())?;

    // trait
    let mut trait_file = File::create(format!("{home_path}{TRAITS_DIR}/{contract_name}.rs"))?;
    trait_file.write_all(rust_fmt.format_tokens(trait_definition).unwrap().as_bytes())?;

    Ok(())
}
