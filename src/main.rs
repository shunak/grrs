#![allow(unused)]

use clap::Parser;
use std::io::BufReader;
use std::fs::File;


#[derive(Parser)]
struct Cli {
    pattern: String,
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}


#[derive(Debug)]
struct CustomError(String);


fn main() -> Result<(), CustomError> {

    let path = "test.txt";
    let content = std::fs::read_to_string(path).map_err(|err| CustomError(format!("Error reading `{}`: {}", path, err)))?;
    println!("file content: {}", content);
    Ok(())

}
