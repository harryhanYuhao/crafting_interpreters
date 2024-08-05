use std::error::Error;
use std::fmt;

use crate::interpreter::token::Token;
use crate::interpreter::AST_Node::AST_Node;
use clap::error::ErrorKind;
use colored::*;
use std::convert::From;
use std::fs::File;
use std::io::{BufRead, BufReader};
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

    pub fn from_token(token: &Token, description: &str, filename: &str) -> Self {
        let column = token.column;
        let row = token.line;
        let source = Source::from_filename(filename);
        ErrorLox {
            description: description.to_string(),
            error_type: ErrorType::UnKnown,
            row,
            column,
            source,
        }
    }

    // pub fn from_lox_variable

    pub fn from_arc_mutex_token(
        token: Arc<Mutex<Token>>,
        description: &str,
        filename: &str,
    ) -> Self {
        let tmp = token.lock().unwrap();
        ErrorLox::from_token(&tmp, description, filename)
    }

    pub fn from_ast_node(node: &AST_Node, description: &str, filename: &str) -> Self {
        let token = node.get_token();
        ErrorLox::from_arc_mutex_token(token, description, filename)
    }

    pub fn from_arc_mutex_ast_node(
        node: Arc<Mutex<AST_Node>>,
        description: &str,
        filename: &str,
    ) -> Self {
        let node = node.lock().unwrap();
        ErrorLox::from_ast_node(&node, description, filename)
    }

    pub fn panic(&self) {
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
