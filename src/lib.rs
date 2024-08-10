#![allow(unused_imports)]
#[macro_use]
extern crate lazy_static;

pub mod interpreter;
pub mod runtime;

pub mod err_lox;

mod test;

use std::error::Error;
use std::fs::read_to_string;
use std::io::{self, prelude::*, stdout, BufReader, Write};
use std::sync::{Arc, Mutex};

use interpreter::parse_tree_unfinished::ParseTreeUnfinshed;
use interpreter::parser::{parse, ParseState};
use interpreter::scanner::scan_tokens;
use interpreter::token::TokenArcVec;

use runtime::run;

use crate::err_lox::ErrorLox;

// DEBUG:
use log::{debug, error, info, trace, warn};

// TODO: remove collect and return iterator
fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .expect(&format!("Failed to read{filename}"))
        .lines()
        .map(String::from)
        .collect()
}

/// run file process and execute the file line by line, as in run_prompt
pub fn run_file(path: &str) -> Result<(), ErrorLox> {
    let mut parse_tree: ParseTreeUnfinshed = ParseTreeUnfinshed::new();

    for (index, line) in read_lines(path).iter().enumerate() {
        println!("{:<2}{}", index + 1, line);
    }

    let res = parse(&mut parse_tree, path);
    match res {
        ParseState::Err(e) => {
            return Err(e);
        }
        ParseState::Unfinished => {
            println!("Unfinished:\n{:?}", parse_tree);
        }
        ParseState::Finished => {
            println!("{:?}", parse_tree);
        }
    }

    info!("START EXECUTION!");
    let tree = parse_tree.get_finished_node()?;
    let tree = tree.unwrap();

    let _ = run(tree);

    Ok(())
}

// TODO: add raw mode
pub fn run_prompt() -> Result<(), Box<dyn Error>> {
    let msg = r#"Welcome to Lox programming language"#;
    println!("{}", msg);
    let mut buffer = String::new();
    let mut line = 1;
    loop {
        print!("{line} >>> ");
        stdout().flush()?;
        io::stdin().read_line(&mut buffer)?;
        let tokens = scan_tokens(&buffer, &mut line, "stdin").unwrap();
        for i in tokens {
            println!("{:?}", i.lock().unwrap());
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
