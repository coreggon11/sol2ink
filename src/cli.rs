use clap::{
    command,
    Parser,
};
use std::path::Path;

#[derive(Debug, Clone)]
pub enum CliInput {
    SolidityFile(String),
    Directory(String),
    SwitchFlag(SwitchFlag),
    SpecificContract(String),
}

#[derive(Debug, Clone)]
pub enum SwitchFlag {
    None,
    SpecifyContract,
    OmitContract,
    OmitReadStorage,
}

/// Sol2Ink - tool to convert Solidity smart contracts to Ink! smart contracts
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path to files or directories
    #[arg(value_parser = sol_file_parser)]
    pub(crate) inputs: Option<Vec<CliInput>>,
}

fn check_path(s: &str) -> Result<(), String> {
    if !Path::new(&s).exists() {
        return Err(format!("{s} does not exist"))
    }
    Ok(())
}

fn sol_file_parser(s: &str) -> Result<CliInput, String> {
    let result = s.to_string();

    if result.ends_with(".sol") {
        check_path(s)?;
        Ok(CliInput::SolidityFile(result))
    } else if Path::new(&result).is_dir() {
        check_path(s)?;
        Ok(CliInput::Directory(result))
    } else if result == *"contracts" {
        Ok(CliInput::SwitchFlag(SwitchFlag::SpecifyContract))
    } else if result == *"omit" {
        Ok(CliInput::SwitchFlag(SwitchFlag::OmitContract))
    } else if result == *"omit_read_storage" {
        Ok(CliInput::SwitchFlag(SwitchFlag::OmitReadStorage))
    } else {
        Ok(CliInput::SpecificContract(result))
    }
}

pub fn cli() -> Args {
    Args::parse()
}
