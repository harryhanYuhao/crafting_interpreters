use std::error::Error;
use std::fmt;

use crate::interpreter::token::Token;
use crate::interpreter::AST_Node::AST_Node;
use crate::runtime::lox_variable::LoxVariable;
use clap::error::ErrorKind;
use colored::*;
use std::convert::From;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::sync::{Arc, Mutex};

// DEBUG:
use log::{debug, error, info, trace, warn};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorType {
    ParseErr,
    ScanErr,
    UnterminatedDelimiter,
    UnKnown,
}

#[derive(Debug)]
pub enum Source {
    NoSource,
    FileName(String),
    Stdin,
}

impl Source {
    pub fn from_filename(filename: &str) -> Self {
        Source::FileName(filename.to_string())
    }
}

#[derive(Debug)]
pub struct ErrorLox {
    description: String,
    error_type: ErrorType,
    row: usize,
    column: usize,
    source: Source,
}

impl ErrorLox {
    pub fn from_filename(description: &str, row: usize, column: usize, filename: &str) -> Self {
        let source = Source::from_filename(filename);
        ErrorLox {
            description: description.to_string(),
            error_type: ErrorType::UnKnown,
            row,
            column,
            source,
        }
    }

    // TODO: remove filename, as token already has it
    pub fn from_token(token: &Token, description: &str) -> Self {
        let column = token.column;
        let row = token.line;
        let source = Source::from_filename(&token.source_file);
        ErrorLox {
            description: description.to_string(),
            error_type: ErrorType::UnKnown,
            row,
            column,
            source,
        }
    }

    pub fn from_lox_variable(variable: &LoxVariable, description: &str) -> Self {
        // TODO: UNFINISHED
        match variable.get_ref_node() {
            None => {
                return ErrorLox {
                    description: description.to_string(),
                    error_type: ErrorType::UnKnown,
                    row: 0,
                    column: 0,
                    source: Source::NoSource,
                }
            }
            Some(node) => {
                let token = AST_Node::get_token_from_arc(node);
                let ref_token = token.lock().unwrap();
                return ErrorLox {
                    description: description.to_string(),
                    error_type: ErrorType::UnKnown,
                    row: ref_token.line,
                    column: ref_token.column,
                    source: Source::from_filename(&ref_token.source_file),
                };
            }
        }
    }

    pub fn from_arc_mutex_token(token: Arc<Mutex<Token>>, description: &str) -> Self {
        let tmp = token.lock().unwrap();
        ErrorLox::from_token(&tmp, description)
    }

    pub fn from_ast_node(node: &AST_Node, description: &str) -> Self {
        let token = node.get_token();
        ErrorLox::from_arc_mutex_token(token, description)
    }

    pub fn from_arc_mutex_ast_node(node: Arc<Mutex<AST_Node>>, description: &str) -> Self {
        let node = node.lock().unwrap();
        ErrorLox::from_ast_node(&node, description)
    }

    pub fn panic(&self) {
        // TODO: WHAT IS A BETTER MAY TO HANDLE THIS?
        match &self.source {
            Source::Stdin => {
                panic!("ERROR: {}", self.description);
            }
            Source::FileName(f) => match Path::new(&f).try_exists() {
                Ok(exists) => {
                    if !exists {
                        panic!("ERROR: {}", self.description);
                    }
                }
                _ => {
                    panic!("ERROR: {}", self.description);
                }
            },
            Source::NoSource => {
                panic!("ERROR: {}", self.description);
            }
        }
        println!("{}", self);
        std::process::exit(1);
    }

    pub fn set_error_type(&mut self, error_type: ErrorType) {
        self.error_type = error_type;
    }

    pub fn get_error_type(&self) -> ErrorType {
        self.error_type.clone()
    }
}

impl fmt::Display for ErrorLox {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let source_name: String = match &self.source {
            Source::FileName(name) => format!("{}", name.underline()),
            Source::Stdin => "stdin".underline().to_string(),
            // TODO: needs better handling
            Source::NoSource => {
                panic!("{}", self.description);
            }
        };

        // error!("{:?}", self);
        let detailed_desr: String = match &self.source {
            Source::FileName(name) => {
                let reader =
                    BufReader::new(File::open(name).expect(&format!("Cannot open file {name}")));
                let mut content_at_nth = reader
                    .lines()
                    .nth(self.row - 1)
                    .expect(&format!(
                        "Internal Error: {} is not {} lines long!",
                        name, self.row
                    ))
                    .expect(&format!(
                        "Internal Error: Could not read the {}th line long from {}!",
                        self.row, name
                    ));
                let mut content_second_line = String::new();
                for _ in 1..self.column {
                    content_second_line.push_str(" ");
                }
                let red_tick = "^".red().to_string();
                content_second_line.push_str(&red_tick);
                content_at_nth.push_str("\n");
                content_at_nth.push_str(&content_second_line);
                content_at_nth
            }
            _ => String::new(),
        };

        write!(
            f,
            "{}: {} \n--> {}.{}:{}\n{}",
            "Error".bold().red(),
            self.description.bold(),
            source_name,
            self.row,
            self.column,
            detailed_desr
        )
    }
}

impl Error for ErrorLox {}
