use crate::parser::instruction::Instruction;
use nom::{
    bytes::complete::{tag, take_while},
    character::complete::{alpha0, alpha1, multispace0},
    combinator::{map, opt},
    multi::separated_list0,
    sequence::{delimited, pair, tuple},
    AsChar, IResult,
};

use super::{identifier_chars, Item};

#[derive(PartialEq, Debug, Clone)]
pub(crate) struct Macro<'a> {
    pub(crate) identifier: &'a str,
    pub(crate) arguments: Vec<&'a str>,
    pub(crate) body: Vec<Instruction<'a>>,
}

impl<'a> Macro<'a> {
    /// Substitutes the given arguments into the macro, replacing all occurences with the same index.
    /// If the lengths of the new arguments and existing arguments do not match, None will be returned.
    pub(crate) fn substitute_arguments(&self, new_args: &[&'a str]) -> Option<Vec<Item<'a>>> {
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
pub(crate) fn macro_declaration(input: &str) -> IResult<&str, Macro> {
    map(
        tuple((
            identifier_chars,
            delimited(
                tag("("),
                separated_list0(pair(tag(","), opt(multispace0)), identifier_chars),
                tag(")"),
            ),
            delimited(
                tuple((multispace0, tag("="), multispace0, (tag("{")))),
                super::parse_program,
                pair(multispace0, tag("}")),
            ),
        )),
        |(identifier, arguments, body)| Macro {
            identifier,
            arguments,
            body: body
                .iter()
                .filter_map(|item| {
                    if let Item::Instruction(inst) = item {
                        Some(inst)
                    } else {
                        None
                    }
                })
                .cloned()
                .collect(),
        },
    )(input)
}

#[derive(PartialEq, Debug, Clone)]
pub(crate) struct MacroCall<'a> {
    pub(crate) identifier: &'a str,
    pub(crate) arguments: Vec<&'a str>,
}

pub(crate) fn macro_call(input: &str) -> IResult<&str, MacroCall> {
    map(
        pair(
            take_while(|c: char| c.is_alpha() || c == '_'),
            delimited(
                tag("!("),
                separated_list0(pair(tag(","), multispace0), alpha1),
                tag(")"),
            ),
        ),
        |(identifier, arguments)| MacroCall {
            identifier,
            arguments,
        },
    )(input)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parser::{instruction::*, macros};

    #[test]
    fn test_macro_substitute() {
        let macro_defn = Macro {
            identifier: "IN_STO",
            arguments: vec!["a", "b"],
            body: vec![
                Instruction::new(None, Opcode::IN, None),
                Instruction::new(None, Opcode::STO, Some("a")),
                Instruction::new(None, Opcode::STO, Some("a")),
                Instruction::new(None, Opcode::STO, Some("b")),
            ],
        };

        assert_eq!(macro_defn.substitute_arguments(&[]), None);
        assert_eq!(macro_defn.substitute_arguments(&["count"]), None);
        assert_eq!(
            macro_defn.substitute_arguments(&["count", "count"]),
            Some(vec![
                Item::Instruction(Instruction::new(None, Opcode::IN, None)),
                Item::Instruction(Instruction::new(None, Opcode::STO, Some("count"))),
                Item::Instruction(Instruction::new(None, Opcode::STO, Some("count"))),
                Item::Instruction(Instruction::new(None, Opcode::STO, Some("count"))),
            ])
        );
        assert_eq!(
            macro_defn.substitute_arguments(&["count", "count_two"]),
            Some(vec![
                Item::Instruction(Instruction::new(None, Opcode::IN, None)),
                Item::Instruction(Instruction::new(None, Opcode::STO, Some("count"))),
                Item::Instruction(Instruction::new(None, Opcode::STO, Some("count"))),
                Item::Instruction(Instruction::new(None, Opcode::STO, Some("count_two"))),
            ])
        );
    }

    #[test]
    fn test_macro_parsing() {
        let macro_str = "IN_STO(location) = {
            IN
            STO location
        }";

        let macro_parsed = macro_declaration(macro_str);
        println!("{:?}", macro_parsed);
        assert!(macro_parsed.is_ok());
        let macro_parsed = macro_parsed.unwrap().1;

        assert_eq!(
            macro_parsed,
            Macro {
                identifier: "IN_STO",
                arguments: vec!["location"],
                body: vec![
                    Instruction::new(None, Opcode::IN, None),
                    Instruction::new(None, Opcode::STO, Some("location"))
                ]
            }
        );
    }

    #[test]
    fn test_macro_call_parsing() {
        let macro_call_str = "IN_STO!(a, b)";
        let macro_call_parsed = macro_call(macro_call_str);

        assert!(macro_call_parsed.is_ok());
        let macro_call_parsed = macro_call_parsed.unwrap().1;

        assert_eq!(
            macro_call_parsed,
            MacroCall {
                identifier: "IN_STO",
                arguments: vec!["a", "b"]
            }
        );
    }
}
