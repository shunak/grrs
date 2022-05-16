#![allow(unused)]

use clap::Parser;
use std::io::BufReader;
use std::fs::File;
use anyhow::{Context, Result};
use indicatif::{HumanDuration, MultiProgress, ProgressBar, ProgressStyle};
use grrs::find_matches;

#[derive(Parser)]
struct Cli {
    pattern: String,
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}


fn answer() -> i32 {
    return 42;
}

#[test]
fn check_answer_validity() {
    assert_eq!(answer(), 42);
}


#[derive(Debug)]
struct CustomError(String);

#[test]
fn find_a_match() {
    let mut result = Vec::new(); // Vec implements Write trait.
    find_matches("lorem ipsum\ndolor sit amet","lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}


fn main() -> Result<()> {
    let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path).with_context(|| format!("could not read file`{}`", args.path.display()))?;

    find_matches(&content, &args.pattern, &mut std::io::stdout()); // stdout() implements Write trait
    Ok(())

}
