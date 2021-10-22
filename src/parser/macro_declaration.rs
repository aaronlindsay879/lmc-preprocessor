use nom::{
    bytes::complete::tag,
    character::complete::multispace0,
    combinator::{map, opt},
    multi::separated_list0,
    sequence::{delimited, pair, tuple},
    IResult,
};

use super::{identifier, instruction::Instruction, Item};

/// Stores information about a single macro declaration.
#[derive(PartialEq, Debug, Clone)]
pub(crate) struct MacroDeclaration<'a> {
    pub(crate) identifier: &'a str,
    pub(crate) arguments: Vec<&'a str>,
    pub(crate) body: Vec<Instruction<'a>>,
}

impl<'a> MacroDeclaration<'a> {
    /// Substitutes the given arguments into the macro, replacing all occurences with the same index.
    /// If the lengths of the new arguments and existing arguments do not match, None will be returned.
    pub(crate) fn substitute_arguments(&self, new_args: &[&'a str]) -> Option<Vec<Item<'a>>> {
        // will only work if same number of arguments
        if new_args.len() != self.arguments.len() {
            return None;
        }

        Some(
            self.body
                .iter()
                .map(|inst| {
                    // find the index of the instructions operand in the macros argument list
                    let index = self
                        .arguments
                        .iter()
                        .position(|&elem| Some(elem) == inst.operand);

                    // if the operand is in the argument list, create a new instruction with the
                    // operand from the same position in the replacements list
                    // if the operand is _not_ in the arguments list, simply return the instruction (cloned)
                    match index {
                        Some(index) => Item::Instruction(Instruction::new(
                            inst.label,
                            inst.opcode.clone(),
                            Some(new_args[index]),
                        )),
                        _ => Item::Instruction(inst.clone()),
                    }
                })
                .collect(),
        )
    }
}

/// Matches a macro declaration
pub(crate) fn macro_declaration(input: &str) -> IResult<&str, MacroDeclaration> {
    // a macro declaration looks like
    // IDENTIFIER(ARGUMENTS, ARGUMENTS, ...) => {
    //     PROGRAM
    // }
    map(
        tuple((
            // matches the identifier
            identifier,
            // matches the argument list
            delimited(
                tag("("),
                separated_list0(pair(tag(","), opt(multispace0)), identifier),
                tag(")"),
            ),
            // matches the function bodyd
            delimited(
                tuple((multispace0, tag("="), multispace0, (tag("{")))),
                super::parse_program,
                pair(multispace0, tag("}")),
            ),
        )),
        |(identifier, arguments, body)| MacroDeclaration {
            identifier,
            arguments,
            body: body
                .iter()
                .filter_map(|item| {
                    // discard all items that aren't instructions
                    if let Item::Instruction(inst) = item {
                        Some(inst.clone())
                    } else {
                        None
                    }
                })
                .collect(),
        },
    )(input)
}
