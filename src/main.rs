use parser::parse_program;
use preprocessor::{replace_macro, to_assembly};

mod parser;
mod preprocessor;

fn preprocess(input: &str) -> Option<String> {
    let program = parse_program(input).ok()?.1;
    let program = replace_macro(&program);
    Some(to_assembly(program))
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
