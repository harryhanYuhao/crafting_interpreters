#[macro_use]
extern crate lazy_static;

pub mod parser;
pub mod scanner;
pub mod token;

use std::error::Error;
use std::fs::File;
use std::io::{self, prelude::*, stdout, BufReader, Write};

pub fn run_file(path: &str) -> Result<(), Box<dyn Error>> {
    // rust feature: file automatically closed once out of scope
    let f = File::open(path)?;
    let mut f = BufReader::new(f);
    let mut line = 1;
    let mut f_string: String = String::new();
    f.read_to_string(&mut f_string)?;
    let tokens = scanner::scan_tokens(&f_string, &mut line)?;

    for i in tokens.iter() {
        println!("{:?}", i);
    }

    Ok(())
}

// TODO: add raw mode
pub fn run_prompt() -> Result<(), Box<dyn Error>> {
    let msg = r#"Welcome to Lox programming language"#;
    println!("{}", msg);
    let mut buffer = String::new();
    let mut line: u32 = 1;
    loop {
        print!("{line} >>> ");
        stdout().flush()?;
        io::stdin().read_line(&mut buffer)?;
        let tokens = scanner::scan_tokens(&buffer, &mut line).unwrap();
        for i in tokens {
            println!("{:?}", i);
        }
        buffer.clear();
    }
    // Ok(())
}

pub fn help() {
    let msg = r#"usage:  lox [script] [-h]
        lox # start interactive shell
        lox script.lox # run script file"#;
    println!("{}", msg);
}
