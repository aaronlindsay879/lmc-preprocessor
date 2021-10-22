mod instruction;
mod macro_call;
pub(crate) mod macro_declaration;

use self::{
    macro_call::{macro_call, MacroCall},
    macro_declaration::{macro_declaration, MacroDeclaration},
};
use instruction::Instruction;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::{multispace0, not_line_ending},
    combinator::{map, opt},
    multi::many0,
    sequence::{delimited, preceded},
    AsChar, IResult,
};

#[derive(PartialEq, Debug, Clone)]
pub(crate) enum Item<'a> {
    Instruction(Instruction<'a>),
    MacroDeclaration(MacroDeclaration<'a>),
    MacroCall(MacroCall<'a>),
    Comment(&'a str),
}

/// Matches a comment, such as "# this is a comment"
fn comment(input: &str) -> IResult<&str, &str> {
    preceded(tag("#"), not_line_ending)(input)
}

/// Matches valid identifiers, such as "aaa_b"
fn identifier(input: &str) -> IResult<&str, &str> {
    take_while(|c: char| c.is_alpha() || c == '_')(input)
}

/// Parses an entire program, returning a vector of instructions and discarding comments
pub(crate) fn parse_program(input: &str) -> IResult<&str, Vec<Item>> {
    // a program consists of many (macro declarations, macro calls, instructions, comments) delimeted by spaces/newlines
    many0(delimited(
        multispace0,
        alt((
            // depending on the type of item matched, put in correct item enum
            map(macro_declaration, Item::MacroDeclaration),
            map(macro_call, Item::MacroCall),
            map(instruction::parse_instruction, Item::Instruction),
            map(comment, Item::Comment),
        )),
        // due to limitations with my program, comments on the end of the line will be discarded
        opt(comment),
    ))(input)
}

#[cfg(test)]
mod test {
    use super::{
        instruction::{Instruction, Opcode},
        parse_program, Item,
    };
    #[test]
    fn test_program_parser_simple_division() {
        let preparsed_program = "# Code to compute a divided by b
            IN
            STO	a
            IN
            STO	b
        # Keep subtracting a from b until you go negative
        # Keep a count of how many times you do it
        start	LDA	count
            ADD	one
            STO	count
            LDA	a
            SUB	b
            STO	a
            BRP	start
        done	LDA	count
        # Subtract one as we went one too far
            SUB	one
            OUT
            HLT
        a	DAT	000
        b	DAT	000
        count	DAT	000
        one	DAT	001";

        let parsed = parse_program(preparsed_program);
        assert!(parsed.is_ok());
        let parsed = parsed.unwrap().1;

        macro_rules! assert_program_eq {
            ($lhs:expr; $($label:expr, $opcode:expr, $operand:expr,)*) => {
                assert_eq!(
                    $lhs,
                    vec![$(
                        Item::Instruction(Instruction::new($label, $opcode, $operand)),
                    )*]
                );
            }
        }

        assert_program_eq!(
            parsed.iter().filter(|item| {
                matches!(item, Item::Instruction(..))
            }).cloned().collect::<Vec<_>>();
            None, Opcode::IN, None,
            None, Opcode::STO, Some("a"),
            None, Opcode::IN, None,
            None, Opcode::STO, Some("b"),
            Some("start"), Opcode::LDA, Some("count"),
            None, Opcode::ADD, Some("one"),
            None, Opcode::STO, Some("count"),
            None, Opcode::LDA, Some("a"),
            None, Opcode::SUB, Some("b"),
            None, Opcode::STO, Some("a"),
            None, Opcode::BRP, Some("start"),
            Some("done"), Opcode::LDA, Some("count"),
            None, Opcode::SUB, Some("one"),
            None, Opcode::OUT, None,
            None, Opcode::HLT, None,
            Some("a"), Opcode::DAT, Some("000"),
            Some("b"), Opcode::DAT, Some("000"),
            Some("count"), Opcode::DAT, Some("000"),
            Some("one"), Opcode::DAT, Some("001"),
        );
    }
}
