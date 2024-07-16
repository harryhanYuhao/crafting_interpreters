use std::env;

use lox_rust::{help, run_file, run_prompt};

// TODO: USE CLAP TO PARSE COMMAND LINE ARGUMENTS
// https://docs.rs/clap/latest/clap/_derive/_tutorial/chapter_0/index.html

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => {
            run_prompt().unwrap();
        }
        2 => {
            match run_file(&args[1]) {
                Err(e)=> {
                    e.panic();
                },
                _ => {}
            }
        }
        _ => {
            help();
        }
    }
}
