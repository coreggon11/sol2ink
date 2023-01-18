use clap::{
    command,
    Parser,
};
use std::path::Path;

#[derive(Debug, Clone)]
pub enum CliInput {
    SolidityFile(String),
    Directory(String),
}

/// Sol2Ink - tool to convert Solidity smart contracts to Ink! smart contracts
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path to files or directories
    #[arg(value_parser = sol_file_parser)]
    pub(crate) files: Option<Vec<CliInput>>,
}

fn sol_file_parser(s: &str) -> Result<CliInput, String> {
    let result = s.to_string();

    if !Path::new(&result).exists() {
        return Err(format!("{} does not exist", result))
    }

    if result.ends_with(".sol") {
        Ok(CliInput::SolidityFile(result))
    } else if Path::new(&result).is_dir() {
        Ok(CliInput::Directory(result))
    } else {
        Err(format!("{} is not a solidity file or directory", result))
    }
}

pub fn cli() -> Args {
    let args = Args::parse();

    return args
}
