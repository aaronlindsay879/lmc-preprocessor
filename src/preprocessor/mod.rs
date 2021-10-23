use crate::parser::{macros::macro_declaration::MacroDeclaration, Item};

/// Goes through the program, creating a new one with all macro invocations replaced with the given macro body.
/// If a macro does not have a declaration, it is simply ignored and replaced with nothing.
pub(crate) fn replace_macro<'a, 'b>(program: &'a [Item<'b>]) -> Vec<Item<'b>> {
    /// Replaces all macro calls with the definition once, may need to be ran multiple times.
    fn replace_once<'a, 'b>(
        program: &'a [Item<'b>],
        macros: &[&MacroDeclaration<'b>],
    ) -> Vec<Item<'b>> {
        program
            .iter()
            .flat_map(|item| match item.clone() {
                // simply move instructions over, no changes required
                Item::Instruction(inst) => vec![Item::Instruction(inst)],
                Item::MacroCall(call) => {
                    // find the corresponding macro definition
                    let macro_definition = macros
                        .iter()
                        .find(|macro_call| macro_call.get_identifier() == call.get_identifier());

                    // if a definition exists, substitute the arguments with the new ones
                    // if a definition does not exist, or substituting arguments fails, simply return an empty vector (outputting nothing)
                    macro_definition
                        .and_then(|x| x.substitute_arguments(call.get_arguments()))
                        .unwrap_or_else(Vec::new)
                }
                // everything else is discarded
                _ => Vec::new(),
            })
            .collect()
    }

    // initially need to find all macro definitions
    let macros: Vec<_> = program
        .iter()
        .filter_map(|item| match item {
            Item::MacroDeclaration(macro_def) => Some(macro_def),
            _ => None,
        })
        .collect();

    // then replace each macro call with the macro definition body
    let mut output: Vec<_> = replace_once(program, &macros);

    // if the output still contains any macro calls, need to repeat
    while output
        .iter()
        .any(|item| matches!(item, Item::MacroCall(..)))
    {
        output = replace_once(&output, &macros);
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
