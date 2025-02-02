use std::{ffi::OsString, io::{stdout, Write}, process::exit};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    script: Option<OsString>
}

fn main()
{
    let args = Args::parse();
    args.script.map_or_else(run_prompt, run_script);
}

fn run_prompt() -> !
{
    let input = std::io::stdin();
    let mut bytes = 1;
    let mut buffer: String = String::new();
    while bytes > 0 {
        print!("> ");
        stdout().flush().expect("Unable to flush stdout?");

        bytes = input.read_line(&mut buffer).expect("Error reading from stdin!");
        let trimmed = buffer.trim();
        println!("Bytes Read: {bytes}, String: '{trimmed}'");
        buffer.clear();
    }

    exit(0);
}

fn run_script(_script: OsString) -> !
{
    exit(0)
}
