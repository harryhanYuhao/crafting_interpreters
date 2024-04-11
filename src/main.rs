use std::env;

use lox_rust::{help, run_file, run_prompt};

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => {
            run_prompt().unwrap();
        }
        2 => {
            run_file(&args[1]).unwrap();
        }
        _ => {
            help();
        }
    }
}
