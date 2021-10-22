use parser::parse_program;
use preprocessor::{assemble, replace_macro};

mod parser;
mod preprocessor;

fn preprocess(input: &str) -> Option<String> {
    let program = parse_program(input).ok()?.1;
    let program = replace_macro(program);
    Some(assemble(program))
}

fn main() {
    println!(
        "{}",
        preprocess(
            "
            IN_STO(location_a, location_b) = {
                    IN
                    STO location_a
                    STO location_b
            }

            IN_STO!(a, b)
            IN_STO!(c, d)
            "
        )
        .unwrap()
    );
}
