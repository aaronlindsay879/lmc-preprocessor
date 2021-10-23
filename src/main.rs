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

    match options.path {
        Some(path) => {
            let data = std::fs::read_to_string(path);
            if let Ok(data) = data {
                println!("{}", preprocess(&data).unwrap());
            } else {
                println!("Failed to read file!");
            }
        }
        _ => todo!(),
    }
}
