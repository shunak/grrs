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


fn main() -> Result<(), Box<dyn std::error::Error>> {

    let content = std::fs::read_to_string("test.txt")?;
    println!("file content: {}", content);
    Ok(())

}
