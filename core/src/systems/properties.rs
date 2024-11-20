use std::borrow::Cow;

use crate::components::*;
use crate::ns::*;
use crate::triples::MyTerm;
use bevy_ecs::prelude::*;
use lsp_types::CompletionItemKind;
use lsp_types::TextEdit;
use sophia_api::ns::rdfs;
use sophia_api::prelude::{Any, Dataset};
use sophia_api::quad::Quad as _;
use sophia_api::term::Term;
use tracing::info;
use tracing::instrument;

pub struct DefinedClass {
    pub term: MyTerm<'static>,
    pub label: String,
    pub comment: String,
    pub reason: &'static str,
}

fn derive_class(
    subject: <MyTerm<'_> as Term>::BorrowTerm<'_>,
    triples: &Triples,
    source: &'static str,
) -> Option<DefinedClass> {
    info!("Derive class for {}", subject);
    let label = triples
        .object([subject], [rdfs::label])?
        .to_owned()
        .as_str()
        .to_string();
    let comment = triples
        .object([subject], [rdfs::comment])?
        .to_owned()
        .as_str()
        .to_string();
    Some(DefinedClass {
        label,
        comment,
        term: subject.to_owned(),
        reason: source,
    })
}

pub fn derive_classes(
    query: Query<(Entity, &Triples, &Label), (Changed<Triples>, Without<Dirty>)>,
    mut commands: Commands,
) {
    info!("Running derive_classes");
    for (e, triples, label) in &query {
        let classes: Vec<_> = triples
            .0
            .quads_matching(Any, [rdf::type_], [rdfs::Class], Any)
            .flatten()
            .flat_map(|x| derive_class(x.s(), &triples, "owl_class"))
            .collect();

        info!(
            "Found {} classes for {} ({} triples)",
            classes.len(),
            label.0,
            triples.0.len()
        );
        commands.entity(e).insert(Wrapped(classes));
    }
}

#[instrument(skip(query, other))]
pub fn complete_class(
    mut query: Query<(
        &TokenComponent,
        &TripleComponent,
        &Prefixes,
        &DocumentLinks,
        &mut CompletionRequest,
    )>,
    other: Query<(&Label, &Wrapped<Vec<DefinedClass>>)>,
) {
    for (token, triple, prefixes, links, mut request) in &mut query {
        if triple.triple.predicate.value == "http://www.w3.org/1999/02/22-rdf-syntax-ns#type"
            && triple.target == TripleTarget::Object
        {
            for (label, classes) in &other {
                // Check if this thing is actually linked
                if links.iter().find(|link| link.0 == label.0).is_none() {
                    continue;
                }

                for class in classes.0.iter() {
                    let to_beat = prefixes
                        .shorten(&class.term.value)
                        .map(|x| Cow::Owned(x))
                        .unwrap_or(class.term.value.clone());

                    if to_beat.starts_with(&token.text) {
                        request.push(
                            crate::lang::SimpleCompletion::new(
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

pub struct DefinedProperty {
    pub predicate: MyTerm<'static>,
    pub comment: String,
    pub label: String,
    pub range: Vec<String>,
    pub domain: Vec<String>,
    pub reason: &'static str,
}

fn derive_property(
    subject: <MyTerm<'_> as Term>::BorrowTerm<'_>,
    triples: &Triples,
    source: &'static str,
) -> Option<DefinedProperty> {
    info!("Derive class for {}", subject);
    let label = triples
        .object([subject], [rdfs::label])?
        .to_owned()
        .as_str()
        .to_string();
    let comment = triples
        .object([subject], [rdfs::comment])?
        .to_owned()
        .as_str()
        .to_string();
    let domain: Vec<_> = triples
        .objects([subject], [rdfs::domain])
        .map(|x| x.as_str().to_string())
        .collect();

    let range: Vec<_> = triples
        .objects([subject], [rdfs::range])
        .map(|x| x.as_str().to_string())
        .collect();

    Some(DefinedProperty {
        predicate: subject.to_owned(),
        range,
        domain,
        label,
        comment,
        reason: source,
    })
}

pub fn derive_properties(
    query: Query<(Entity, &Triples, &Label), (Changed<Triples>, Without<Dirty>)>,
    mut commands: Commands,
) {
    info!("Running derive_properties");
    for (e, triples, label) in &query {
        let classes: Vec<_> = triples
            .0
            .quads_matching(Any, [rdf::type_], [rdf::Property], Any)
            .flatten()
            .flat_map(|x| derive_property(x.s(), &triples, "owl_property"))
            .collect();

        info!(
            "Found {} properties for {} ({} triples)",
            classes.len(),
            label.0,
            triples.0.len()
        );
        commands.entity(e).insert(Wrapped(classes));
    }
}

#[instrument(skip(query))]
pub fn complete_properties(
    mut query: Query<(
        &TokenComponent,
        &TripleComponent,
        &Prefixes,
        &DocumentLinks,
        &mut CompletionRequest,
    )>,
    other: Query<(&Label, &Wrapped<Vec<DefinedProperty>>)>,
) {
    for (token, triple, prefixes, links, mut request) in &mut query {
        if triple.target == TripleTarget::Predicate {
            for (label, properties) in &other {
                // Check if this thing is actually linked
                if links.iter().find(|link| link.0 == label.0).is_none() {
                    continue;
                }

                for class in properties.0.iter() {
                    let to_beat = prefixes
                        .shorten(&class.predicate.value)
                        .map(|x| Cow::Owned(x))
                        .unwrap_or(class.predicate.value.clone());

                    if to_beat.starts_with(&token.text) {
                        request.push(
                            crate::lang::SimpleCompletion::new(
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
