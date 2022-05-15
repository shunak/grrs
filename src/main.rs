#![allow(unused)]

use clap::Parser;
use std::io::BufReader;
use std::fs::File;
use anyhow::{Context, Result};
use indicatif::{HumanDuration, MultiProgress, ProgressBar, ProgressStyle};


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


fn main() -> Result<()> {
// fn main() {

    let path = "test.txt";
    let content = std::fs::read_to_string(path).with_context(|| format!("could not read file`{}`", path))?;
    println!("file content: {}", content);
    Ok(())
// let pb = indicatif::ProgressBar::new(100);
//     for i in 0..100 {
//         // do_hard_work();
//         pb.println(format!("[+] finished #{}", i));
//         pb.inc(1);
//     }
//     pb.finish_with_message("done");

}
