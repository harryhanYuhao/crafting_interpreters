#[macro_use]
extern crate lazy_static;

pub mod parser;
pub mod scanner;
pub mod token;

#[allow(non_snake_case)]
mod AST_Node;
pub mod err_lox;

mod test;

use std::error::Error;
use std::fs::read_to_string;
use std::io::{self, prelude::*, stdout, BufReader, Write};
use std::sync::{Arc, Mutex};

use parser::ParseTreeUnfinshed;

use crate::parser::ParseState;
use crate::token::TokenArcVec;
use crate::err_lox::ErrorLox;

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
    let mut line_number = 1;
    let mut parse_tree: ParseTreeUnfinshed = ParseTreeUnfinshed::new();

    for (index, lines) in read_lines(path).iter().enumerate() {
        // DEBUG: TOKEN
        println!("Debugging run_file, Line: {}", index + 1);
        println!("{lines}");

        let tokens: TokenArcVec = scanner::scan_tokens(lines, &mut line_number)?;
        
        // DEBUG: TOKEN
        for i in tokens.iter() {
            println!("Scanned Token: {:?}", i.lock().unwrap());
        }
        let res = parser::parse(&tokens, &mut parse_tree, path);
        match res {
            ParseState::Err(e) => {
                return Err(e);
            }
            _ => {}
        }
        println!("{:?}", parse_tree);
        line_number += 1;
    }

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
        let tokens = scanner::scan_tokens(&buffer, &mut line).unwrap();
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
