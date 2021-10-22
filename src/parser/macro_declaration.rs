use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::multispace0,
    combinator::{map, opt},
    multi::separated_list0,
    sequence::{delimited, pair, tuple},
    IResult,
};

use super::{identifier, macro_call::MacroCall, Item};

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

        // create a map of macro declaration arg names -> replacement arg names
        let arg_map = {
            let mut map = HashMap::with_capacity(self.arguments.len());
            for (&a, &b) in self.arguments.iter().zip(new_args.iter()) {
                map.insert(a, b);
            }

            map
        };

        // then simply replace all arguments using helper function
        Some(
            self.body
                .iter()
                .map(|item| substitute_argument_item(item, &arg_map))
                .collect(),
        )
    }
}

/// Substitutes the arguments in a macro call for a single item.
fn substitute_argument_item<'a>(
    item: &Item<'a>,
    argument_map: &HashMap<&'a str, &'a str>,
) -> Item<'a> {
    match item {
        Item::Instruction(instruction) => {
            // easy case, just check if argument is in map, and replace if so
            match argument_map.get(instruction.operand.unwrap_or_default()) {
                Some(new_arg) => Item::Instruction(instruction.clone_with_operand(new_arg)),
                None => item.clone(),
            }
        }
        Item::MacroCall(macro_call) => {
            // slightly more tricky as can have multiple arguments, but basically repeat above for each argument
            let arguments = macro_call
                .arguments
                .iter()
                .map(|argument| *argument_map.get(argument).unwrap_or(argument))
                .collect();

            // then can just reconstruct a macro call
            Item::MacroCall(MacroCall {
                identifier: macro_call.identifier,
                arguments,
            })
        }
        _ => item.clone(),
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
