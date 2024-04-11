#[macro_use]
extern crate lazy_static;

pub mod scanner;
pub mod token;

use std::error::Error;
use std::fs::File;
use std::io::{self, prelude::*, stdout, BufReader, Write};

pub fn run_file(path: &str) -> Result<(), Box<dyn Error>> {
    let f = File::open(path)?;
    let f = BufReader::new(f);

    for line in f.lines() {
        println!("{}", line.unwrap());
    }
    Ok(())
}

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
            println!("{}", i);
        }
        buffer.clear();
    }
    // Ok(())
}

pub fn help() {
    let msg = r#"
usage:  lox [script] [-h]
        lox # start interactive shell
        lox script.lox # run script file
    "#;
    println!("{}", msg);
}
