use clap::Parser;
use parser::{parse_program, Item};
use preprocessor::replace_macro;
use std::{
    fs::File,
    io::{BufWriter, Read, Write},
};

mod parser;
mod preprocessor;

/// Main preprocessing function - currently just parses and replaces macro calls with declarations.
fn preprocess(input: &str) -> Option<Vec<Item>> {
    let program = parse_program(input).ok()?.1;
    Some(replace_macro(&program))
}

#[derive(Parser)]
#[clap(version = "0.1")]
struct Options {
    path: Option<String>,
    #[clap(short, long)]
    out_file: Option<String>,
}

fn main() {
    let options = Options::parse();

    let data = match options.path {
        Some(ref path) => std::fs::read_to_string(path).ok(),
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
            Some(program) => output(&options, &program).unwrap_or_else(|err| println!("{}", err)),
            None => println!("Failed to parse program!"),
        },
        None => println!("Failed to get input!"),
    }
}

/// Handles getting data from stdin, reads until end.
fn handle_stdin() -> Option<String> {
    let mut data = Vec::new();
    std::io::stdin().read_to_end(&mut data).ok()?;

    String::from_utf8(data).ok()
}

/// Outputs the program using the options provided
fn output(options: &Options, program: &[Item]) -> Result<(), &'static str> {
    match &options.out_file {
        Some(path) => {
            let file = File::create(path).map_err(|_| "Failed to create file!")?;
            let mut writer = BufWriter::new(file);

            for item in program {
                writeln!(writer, "{}", item).map_err(|_| "Failed to write to file!")?;
            }
        }
        None => {
            for item in program {
                println!("{}", item)
            }
        }
    }

    Ok(())
}
