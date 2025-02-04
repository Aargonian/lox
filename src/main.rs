#![allow(dead_code)]

mod lexer;

use crate::lexer::IntoScanner;
use clap::Parser;
use std::{
    ffi::OsString,
    io::{stdout, Write},
    process::exit,
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    script: Option<OsString>,
}

fn main() {
    let args = Args::parse();
    args.script.map_or_else(run_prompt, run_script);
}

fn run_prompt() -> ! {
    let input = std::io::stdin();
    loop {
        let mut buffer: String = String::with_capacity(256);
        print!("> ");
        stdout().flush().expect("Unable to flush stdout!");

        input.read_line(&mut buffer).expect("Error reading stdin!");
        run(&buffer);
    }
}

fn run_script(script: OsString) -> ! {
    let script_str = std::fs::read_to_string(script).expect("Unable to read in script file!");

    run(&script_str);
    exit(0)
}

#[allow(unused_variables)]
fn run(source: &str) {
    for token in source.scan() {
        println!("Token {token:?}");
    }
}
