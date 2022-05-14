#![allow(unused)]

use clap::Parser;
use std::io::BufReader;
use std::fs::File;
use anyhow::{Context, Result};


#[derive(Parser)]
struct Cli {
    pattern: String,
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}


#[derive(Debug)]
struct CustomError(String);


fn main() -> Result<()> {

    let path = "test.txt";
    let content = std::fs::read_to_string(path).with_context(|| format!("could not read file`{}`", path))?;
    println!("file content: {}", content);
    Ok(())

}
