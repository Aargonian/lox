mod utils;
mod lexer;

use anyhow::Result;
use lexer::ToTokenIterator;
use std::{env::args, fs, process::exit, path::Path, io};

fn main() -> Result<()> {
    let args: Vec<String> = args().collect();
    match args.len() {
        n if n > 2 => {
            eprintln!("Usage: rlox [script]");
            exit(64);
        }
        2 => {
            let file_path = &args[1];
            run_file(Path::new(&file_path))?;
        }
        _ => run_prompt()?
    }

    Ok(())
}

fn run_file(file_path: &Path) -> Result<()> {
    println!("Running Lox File: {file_path:?}");
    let contents = fs::read_to_string(file_path)?;
    let result = run(&contents);

    if result.is_err() {
        exit(65);
    }

    Ok(())
}

fn run_prompt() -> Result<()> {
    println!("Running prompt!");
    let sin = io::stdin();
    let mut buffer = String::with_capacity(256);
    loop {
        print!("> ");
        let num_bytes = sin.read_line(&mut buffer)?;
        if num_bytes < 1 {
            break Ok(());
        }

        let _result = run(&buffer);
    }
}

#[allow(clippy::pedantic)]
fn run(code: &str) -> Result<()> {
    for token in code.chars().tokens() {
        println!("{:?}", token);
    }

    Ok(())
}
