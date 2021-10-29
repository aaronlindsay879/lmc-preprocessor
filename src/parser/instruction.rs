use std::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};

use nom::{
    branch::alt,
    bytes::complete::{tag_no_case, take_while},
    character::complete::{multispace1, space0, space1},
    combinator::{eof, map, map_opt, opt, peek},
    sequence::{preceded, terminated, tuple},
    AsChar, IResult,
};
use strum::{Display, EnumString, EnumVariantNames, VariantNames};

/// Stores information about a single instruction
#[derive(PartialEq, Debug, Clone)]
pub(crate) struct Instruction<'a> {
    label: Option<&'a str>,
    opcode: Opcode,
    operand: Option<&'a str>,
}

impl<'a> Instruction<'a> {
    /// Creates a new instruction from the given information
    pub(crate) fn new(label: Option<&'a str>, opcode: Opcode, operand: Option<&'a str>) -> Self {
        Self {
            label,
            opcode,
            operand,
        }
    }

    /// Creates a new instruction identical to the current one, but with a different operand
    pub(crate) fn clone_with_operand(&self, operand: &'a str) -> Self {
        Self {
            label: self.label,
            opcode: self.opcode.clone(),
            operand: Some(operand),
        }
    }

    /// Gets the instructions operand
    pub(crate) fn get_operand(&self) -> Option<&'a str> {
        self.operand
    }
}

impl<'a> Display for Instruction<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match (self.label, self.operand) {
            (Some(label), Some(operand)) => {
                write!(f, "{}\t{}\t{}", label, self.opcode.to_string(), operand)
            }
            (Some(label), None) => write!(f, "{}\t{}", label, self.opcode.to_string()),
            (None, Some(operand)) => write!(f, "\t{}\t{}", self.opcode.to_string(), operand),
            _ => write!(f, "\t{}", self.opcode.to_string()),
        }
    }
}

/// Various opcodes
#[allow(clippy::upper_case_acronyms)]
#[derive(EnumVariantNames, EnumString, Display, PartialEq, Debug, Clone)]
pub(crate) enum Opcode {
    ADD,
    SUB,
    STO,
    LDA,
    BRZ,
    BRP,
    BR,
    IN,
    OUT,
    HLT,
    DAT,
}

/// Matches a single instruction (optionally with a label), such as "label   ADD 10"
pub(crate) fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    /// Matches one of the given strings (ignoring case), returning the first match
    fn alternative<'a>(input: &'a str, alternatives: &'a [&'a str]) -> IResult<&'a str, &'a str> {
        for alternative in alternatives {
            match terminated(
                tag_no_case::<&str, &str, nom::error::Error<&str>>(alternative),
                peek(alt((multispace1, eof))),
            )(input)
            {
                Ok(ok) => return Ok(ok),
                _ => continue,
            }
        }

        IResult::Err(nom::Err::Error(nom::error::Error::new(
            "",
            nom::error::ErrorKind::Tag,
        )))
    }

    map_opt(
        alt((
            // match format "[label] [opcode] [operand]?"
            map(
                tuple((
                    take_while(AsChar::is_alphanum),
                    preceded(space1, |str| alternative(str, Opcode::VARIANTS)),
                    opt(preceded(
                        space0,
                        take_while(|c| AsChar::is_alphanum(c) || ['_', '$'].contains(&c)),
                    )),
                )),
                |(label, opcode, operand)| (Some(label), opcode, operand),
            ),
            // match format "[opcode] [operand]?"
            map(
                tuple((
                    preceded(space0, |str| alternative(str, Opcode::VARIANTS)),
                    opt(preceded(
                        space0,
                        take_while(|c| AsChar::is_alphanum(c) || ['_', '$'].contains(&c)),
                    )),
                )),
                |(opcode, operand)| (None, opcode, operand),
            ),
        )),
        |(label, opcode, operand)| {
            Opcode::from_str(opcode).ok().map(|opcode| {
                Instruction::new(
                    label,
                    opcode,
                    if let Some("") = operand {
                        None
                    } else {
                        operand
                    },
                )
            })
        },
    )(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_instruction_parser() {
        macro_rules! test_inst {
            ($($input:expr => $label:expr, $opcode:expr, $operand:expr),*) => {
                $(
                    assert_eq!(
                        parse_instruction($input),
                        Ok(("", Instruction::new(
                            $label,
                            $opcode,
                            $operand
                        )))
                    );
                )*
            };
        }

        test_inst!(
            "ADD 10" => None, Opcode::ADD, Some("10"),
            "LDA storage" => None, Opcode::LDA, Some("storage"),
            "aaaa ADD 10" => Some("aaaa"), Opcode::ADD, Some("10"),
            "OUT" => None, Opcode::OUT, None,
            "IN" => None, Opcode::IN, None,
            "aaa IN" => Some("aaa"), Opcode::IN, None,
            "abc DAT 10" => Some("abc"), Opcode::DAT, Some("10")
        );
    }
}
