use clap::{
    command,
    Parser,
};
use std::path::Path;

#[derive(Debug, Clone)]
pub enum CliInput {
    SolidityFile(String),
    Directory(String),
    SpecificContract(String),
}

/// Sol2Ink - tool to convert Solidity smart contracts to Ink! smart contracts
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path to files or directories
    #[arg(value_parser = sol_file_parser)]
    pub(crate) files: Option<Vec<CliInput>>,
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
    } else {
        Ok(CliInput::SpecificContract(result))
    }
}

pub fn cli() -> Args {
    Args::parse()
}
