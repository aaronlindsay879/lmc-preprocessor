use nom::{
    bytes::complete::tag,
    character::complete::multispace0,
    combinator::{map, opt},
    multi::separated_list0,
    sequence::{delimited, pair, tuple},
    IResult,
};

use super::{identifier, instruction::Instruction, macro_call::MacroCall, Item};

/// Stores information about a single macro declaration.
#[derive(PartialEq, Debug, Clone)]
pub(crate) struct MacroDeclaration<'a> {
    pub(crate) identifier: &'a str,
    pub(crate) arguments: Vec<&'a str>,
    pub(crate) body: Vec<Item<'a>>,
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
                .map(|item| {
                    // find the index of the items operand in the macros argument list
                    let index = self.arguments.iter().position(|&elem| match item {
                        Item::Instruction(Instruction { operand, .. }) => Some(elem) == *operand,
                        Item::MacroCall(MacroCall { arguments, .. }) => arguments.contains(&elem),
                        _ => false,
                    });

                    // if the operand is in the argument list, create a new instruction with the
                    // operand from the same position in the replacements list
                    // if the operand is _not_ in the arguments list, simply return the instruction (cloned)
                    match index {
                        // Some(index) => Item::Instruction(Instruction::new(
                        //     inst.label,
                        //     inst.opcode.clone(),
                        //     Some(new_args[index]),
                        // )),
                        Some(index) => match item {
                            Item::Instruction(inst) => Item::Instruction(Instruction::new(
                                inst.label,
                                inst.opcode.clone(),
                                Some(new_args[index]),
                            )),
                            Item::MacroCall(call) => Item::MacroCall(MacroCall {
                                identifier: call.identifier,
                                arguments: call
                                    .arguments
                                    .iter()
                                    .map(|&x| if self.arguments.contains(&x) { "a" } else { x })
                                    .collect(),
                            }),
                            _ => item.clone(),
                        },
                        _ => item.clone(),
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
            body: body,
        },
    )(input)
}
