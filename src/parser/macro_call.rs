use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, multispace0},
    combinator::map,
    multi::separated_list0,
    sequence::{delimited, pair},
    IResult,
};

use super::identifier;

/// Stores information about a single macro call
#[derive(PartialEq, Debug, Clone)]
pub(crate) struct MacroCall<'a> {
    identifier: &'a str,
    arguments: Vec<&'a str>,
}

impl<'a> MacroCall<'a> {
    /// Creates a new macro call from the given information
    pub(crate) fn new(identifier: &'a str, arguments: Vec<&'a str>) -> Self {
        Self {
            identifier,
            arguments,
        }
    }

    /// Gets the macro calls identifier
    pub(crate) fn get_identifier(&self) -> &'a str {
        self.identifier
    }

    pub(crate) fn get_arguments(&self) -> &Vec<&'a str> {
        &self.arguments
    }
}

/// Parses a single macro call, such as "IN_STO!(a)"
pub(crate) fn macro_call(input: &str) -> IResult<&str, MacroCall> {
    map(
        pair(
            identifier,
            delimited(
                tag("!("),
                separated_list0(pair(tag(","), multispace0), alpha1),
                tag(")"),
            ),
        ),
        |(identifier, arguments)| MacroCall::new(identifier, arguments),
    )(input)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parser::{instruction::*, macro_declaration::MacroDeclaration, Item};

    #[test]
    fn test_macro_substitute() {
        let macro_defn = MacroDeclaration::new(
            "IN_STO",
            vec!["a", "b"],
            vec![
                Item::Instruction(Instruction::new(None, Opcode::IN, None)),
                Item::Instruction(Instruction::new(None, Opcode::STO, Some("a"))),
                Item::Instruction(Instruction::new(None, Opcode::STO, Some("a"))),
                Item::Instruction(Instruction::new(None, Opcode::STO, Some("b"))),
            ],
        );

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
