use std::io::Read;

use clap::Parser;
use parser::parse_program;
use preprocessor::{replace_macro, to_assembly};

mod parser;
mod preprocessor;

fn preprocess(input: &str) -> Option<String> {
    let program = parse_program(input).ok()?.1;
    let program = replace_macro(&program);
    Some(to_assembly(program))
}

#[derive(Parser)]
#[clap(version = "0.1")]
struct Options {
    path: Option<String>,
}

fn main() {
    let options = Options::parse();

    let data = match options.path {
        Some(path) => std::fs::read_to_string(path).ok(),
        _ => {
            if atty::isnt(atty::Stream::Stdin) {
                handle_stdin()
            } else {
                None
            }
        }
    };

    match data {
        Some(data) => match preprocess(&data) {
            Some(program) => println!("{}", program.trim_end()),
            None => println!("Failed to parse program!"),
        },
        None => println!("Failed to get input!"),
    }
}

fn handle_stdin() -> Option<String> {
    let mut data = Vec::new();
    std::io::stdin().read_to_end(&mut data).ok()?;

    String::from_utf8(data).ok()
}
