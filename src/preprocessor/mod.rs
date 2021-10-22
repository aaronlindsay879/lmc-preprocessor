use crate::parser::Item;

pub(crate) fn replace_macro(program: Vec<Item>) -> Vec<Item> {
    // initially need to find all macro definitions
    let macros = program
        .iter()
        .filter_map(|item| match item {
            Item::MacroDeclaration(macro_def) => Some(macro_def),
            _ => None,
        })
        .collect::<Vec<_>>();

    // then replace each macro call with the macro definition body
    program
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
        .collect()
}

pub(crate) fn assemble(program: Vec<Item>) -> String {
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
