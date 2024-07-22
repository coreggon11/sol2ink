use std::{
    fs::{
        self,
        metadata,
        File,
    },
    io::{
        prelude::*,
        BufReader,
    },
    path::Path,
};

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
pub fn write_mermaid(mermaid_string: &String) -> std::io::Result<()> {
    fs::create_dir_all("./output")?;

    let mut mermaid = File::create("./output/output.txt")?;
    mermaid.write_all(mermaid_string.as_bytes())?;

    Ok(())
}
