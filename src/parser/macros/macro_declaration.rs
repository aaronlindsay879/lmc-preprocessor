use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::multispace0,
    combinator::{map, opt},
    multi::separated_list0,
    sequence::{delimited, pair, preceded, tuple},
    IResult,
};

use super::{
    super::{identifier, Item},
    macro_call::MacroCall,
};

/// Stores information about a single macro declaration.
#[derive(PartialEq, Debug, Clone)]
pub(crate) struct MacroDeclaration<'a> {
    identifier: &'a str,
    arguments: Vec<&'a str>,
    body: Vec<Item<'a>>,
}

impl<'a> MacroDeclaration<'a> {
    /// Creates a new macro declaration from the given information
    pub(crate) fn new(identifier: &'a str, arguments: Vec<&'a str>, body: Vec<Item<'a>>) -> Self {
        Self {
            identifier,
            arguments,
            body,
        }
    }

    /// Gets the macro declaration's identifier
    pub(crate) fn get_identifier(&self) -> &str {
        self.identifier
    }

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
            match argument_map.get(instruction.get_operand().unwrap_or_default()) {
                Some(new_arg) => Item::Instruction(instruction.clone_with_operand(new_arg)),
                None => item.clone(),
            }
        }
        Item::MacroCall(macro_call) => {
            // slightly more tricky as can have multiple arguments, but basically repeat above for each argument
            let arguments = macro_call
                .get_arguments()
                .iter()
                .map(|argument| *argument_map.get(argument).unwrap_or(argument))
                .collect();

            // then can just reconstruct a macro call
            Item::MacroCall(MacroCall::new(macro_call.get_identifier(), arguments))
        }
        _ => item.clone(),
    }
}

/// Matches a macro declaration
pub(crate) fn macro_declaration(input: &str) -> IResult<&str, MacroDeclaration> {
    // a macro declaration looks like
    // macro IDENTIFIER(ARGUMENTS, ARGUMENTS, ...) => {
    //     PROGRAM
    // }
    map(
        tuple((
            // matches the identifier
            preceded(tag("macro "), identifier),
            // matches the argument list
            delimited(
                tag("("),
                separated_list0(pair(tag(","), opt(multispace0)), identifier),
                tag(")"),
            ),
            // matches the macro body
            delimited(
                tuple((multispace0, tag("="), multispace0, (tag("{")))),
                super::super::parse_program,
                pair(multispace0, tag("}")),
            ),
        )),
        |(identifier, arguments, body)| MacroDeclaration::new(identifier, arguments, body),
    )(input)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parser::instruction::{Instruction, Opcode};

    #[test]
    fn test_macro_parsing() {
        let macro_str = "macro IN_STO(location) = {
            IN
            STO location
        }";

        let macro_parsed = macro_declaration(macro_str);
        println!("{:?}", macro_parsed);
        assert!(macro_parsed.is_ok());
        let macro_parsed = macro_parsed.unwrap().1;

        assert_eq!(
            macro_parsed,
            MacroDeclaration::new(
                "IN_STO",
                vec!["location"],
                vec![
                    Item::Instruction(Instruction::new(None, Opcode::IN, None)),
                    Item::Instruction(Instruction::new(None, Opcode::STO, Some("location")))
                ]
            )
        );
    }
}
