use std::collections::HashSet;

use lov::LocalPrefix;
use lsp_types::{CompletionItemKind, TextEdit};

use crate::{
    components::{Prefixes, TokenComponent},
    lang::SimpleCompletion,
    token::Token,
};

pub fn prefix_completion_helper(
    word: &TokenComponent,
    prefixes: &Prefixes,
    completions: &mut Vec<SimpleCompletion>,
    mut extra_edits: impl FnMut(&LocalPrefix) -> Option<Vec<TextEdit>>,
) {
    match word.token.value() {
        Token::Invalid(_) => {}
        _ => return,
    }

    let mut defined = HashSet::new();
    for p in prefixes.0.iter() {
        defined.insert(p.url.as_str());
    }

    completions.extend(
        lov::LOCAL_PREFIXES
            .iter()
            .filter(|x| x.name.starts_with(&word.text))
            .filter(|x| !defined.contains(&x.location))
            .flat_map(|lov| {
                let new_text = format!("{}:", lov.name);
                let sort_text = format!("2 {}", new_text);
                let filter_text = new_text.clone();
                if new_text != word.text {
                    let extra_edit = extra_edits(&lov)?;
                    let completion = SimpleCompletion::new(
                        CompletionItemKind::MODULE,
                        format!("{}", lov.name),
                        lsp_types::TextEdit {
                            new_text,
                            range: word.range.clone(),
                        },
                    )
                    .sort_text(sort_text)
                    .filter_text(filter_text);

                    let completion = extra_edit
                        .into_iter()
                        .fold(completion, |completion: SimpleCompletion, edit| {
                            completion.text_edit(edit)
                        });
                    Some(completion)
                } else {
                    None
                }
            }),
    );
}
