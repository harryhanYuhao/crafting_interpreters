use std::fmt;
// use std::error::Error;

use colored::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub enum ErrorType {
    ParseErr,
    ScanErr,
    UnKnown,
}

pub enum Source<'a> {
    FileName(&'a str),
    Stdin,
}

impl<'a> Source<'a> {
    pub fn from_filename(filename: &'a str) -> Self {
        Source::FileName(filename)
    }
}

pub struct ErrorLox<'a> {
    description: &'a str,
    error_type: ErrorType,
    row: usize,
    column: usize,
    source: Source<'a>,
}

impl<'a> ErrorLox<'a> {
    pub fn from_filename(
        description: &'a str,
        row: usize,
        column: usize,
        filename: &'a str,
    ) -> Self {
        let source = Source::from_filename(filename);
        ErrorLox {
            description,
            error_type: ErrorType::UnKnown,
            row,
            column,
            source,
        }
    }
}

impl<'a> fmt::Debug for ErrorLox<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let source_name: String = match self.source {
            Source::FileName(name) => format!("{}", name.underline()),
            Source::Stdin => "stdin".underline().to_string(),
        };

        let detailed_desr: String = match self.source {
            Source::FileName(name) => {
                let reader = BufReader::new(File::open(name).expect("Cannot open file"));
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
// impl <'a> Error for ErrorLox<'a>;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn debug() {
        println!("{:?}", ErrorLox::from_filename("aha", 1, 10, "test.lox"))
    }
}
