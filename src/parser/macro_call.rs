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
    pub(crate) identifier: &'a str,
    pub(crate) arguments: Vec<&'a str>,
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
        |(identifier, arguments)| MacroCall {
            identifier,
            arguments,
        },
    )(input)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parser::{
        instruction::*,
        macro_declaration::{macro_declaration, MacroDeclaration},
        Item,
    };

    #[test]
    fn test_macro_substitute() {
        let macro_defn = MacroDeclaration {
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
            MacroDeclaration {
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
