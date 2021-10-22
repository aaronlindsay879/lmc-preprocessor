use crate::parser::{macro_declaration::MacroDeclaration, Item};

/// Goes through the program, creating a new one with all macro invocations replaced with the given macro body.
/// If a macro does not have a declaration, it is simply ignored and replaced with nothing.
pub(crate) fn replace_macro<'a, 'b>(
    program: &'a [Item<'b>],
    macros: Option<Vec<&MacroDeclaration<'b>>>,
) -> Vec<Item<'b>> {
    // initially need to find all macro definitions
    let macros = match macros {
        Some(a) => a,
        None => program
            .iter()
            .filter_map(|item| match item {
                Item::MacroDeclaration(macro_def) => Some(macro_def),
                _ => None,
            })
            .collect::<Vec<_>>(),
    };

    // then replace each macro call with the macro definition body
    let mut output: Vec<_> = program
        .iter()
        .flat_map(|item| match item.clone() {
            Item::MacroDeclaration(_) | Item::Comment(_) => Vec::new(),
            Item::Instruction(inst) => vec![Item::Instruction(inst)],
            Item::MacroCall(call) => {
                let macro_definition = macros
                    .iter()
                    .find(|macro_call| macro_call.identifier == call.identifier);

                macro_definition
                    .map(|x| x.substitute_arguments(&call.arguments))
                    .flatten()
                    .unwrap_or_else(|| Vec::new())
            }
        })
        .collect();

    // if the output still contains any macro calls, need to repeat
    if output
        .iter()
        .any(|item| matches!(item, Item::MacroCall(..)))
    {
        output = replace_macro(&output, Some(macros));
    }

    output
}

/// Converts a given program back to the string representation in assembly
pub(crate) fn to_assembly(program: Vec<Item>) -> String {
    program.iter().fold(String::new(), |acc, item| {
        format!(
            "{}{}\n",
            acc,
            match item {
                Item::Instruction(inst) => inst.to_string(),
                Item::Comment(comment) => comment.to_string(),
                _ => unreachable!(),
            }
        )
    })
}
