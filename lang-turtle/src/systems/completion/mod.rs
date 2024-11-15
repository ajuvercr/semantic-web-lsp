mod prefix;
mod subject;

use std::borrow::Cow;

use lsp_core::systems::DefinedClass;
use lsp_core::systems::DefinedProperty;
use lsp_types::CompletionItemKind;
use lsp_types::TextEdit;
pub use prefix::turtle_lov_prefix_completion;
pub use prefix::turtle_prefix_completion;
pub use subject::subject_completion;
use tracing::instrument;

#[cfg(test)]
mod tests;

use bevy_ecs::prelude::*;
use lsp_core::components::*;

use crate::TurtleLang;

#[instrument(skip(query))]
pub fn complete_class(
    mut query: Query<(
        &TokenComponent,
        &TripleComponent,
        &Element<TurtleLang>,
        &DocumentLinks,
        &mut CompletionRequest,
    )>,
    other: Query<(&Label, &Wrapped<Vec<DefinedClass>>)>,
) {
    for (token, triple, turtle, links, mut request) in &mut query {
        if triple.triple.predicate.value == "http://www.w3.org/1999/02/22-rdf-syntax-ns#type"
            && triple.target == TripleTarget::Object
        {
            for (label, classes) in &other {
                // Check if this thing is actually linked
                if links.iter().find(|link| link.0 == label.0).is_none() {
                    continue;
                }

                for class in classes.0.iter() {
                    let to_beat = turtle
                        .0
                         .0
                        .shorten(&class.term.value)
                        .map(|x| Cow::Owned(x))
                        .unwrap_or(class.term.value.clone());

                    if to_beat.starts_with(&token.text) {
                        request.push(
                            lsp_core::lang::SimpleCompletion::new(
                                CompletionItemKind::CLASS,
                                format!("{}", to_beat),
                                TextEdit {
                                    range: token.range.clone(),
                                    new_text: to_beat.to_string(),
                                },
                            )
                            .documentation(&class.comment),
                        );
                    }
                }
            }
        }
    }
}

#[instrument(skip(query))]
pub fn complete_properties(
    mut query: Query<(
        &TokenComponent,
        &TripleComponent,
        &Element<TurtleLang>,
        &DocumentLinks,
        &mut CompletionRequest,
    )>,
    other: Query<(&Label, &Wrapped<Vec<DefinedProperty>>)>,
) {
    for (token, triple, turtle, links, mut request) in &mut query {
        if triple.target == TripleTarget::Predicate {
            for (label, properties) in &other {
                // Check if this thing is actually linked
                if links.iter().find(|link| link.0 == label.0).is_none() {
                    continue;
                }

                for class in properties.0.iter() {
                    let to_beat = turtle
                        .0
                         .0
                        .shorten(&class.predicate.value)
                        .map(|x| Cow::Owned(x))
                        .unwrap_or(class.predicate.value.clone());

                    if to_beat.starts_with(&token.text) {
                        request.push(
                            lsp_core::lang::SimpleCompletion::new(
                                CompletionItemKind::PROPERTY,
                                format!("{}", to_beat),
                                TextEdit {
                                    range: token.range.clone(),
                                    new_text: to_beat.to_string(),
                                },
                            )
                            .documentation(&class.comment),
                        );
                    }
                }
            }
        }
    }
}
