use crate::components::*;
use crate::ns::*;
use crate::triples::MyTerm;
use bevy_ecs::prelude::*;
use sophia_api::ns::rdfs;
use sophia_api::prelude::{Any, Dataset};
use sophia_api::quad::Quad as _;
use sophia_api::term::Term;
use tracing::info;

pub struct DefinedClass {
    pub term: MyTerm<'static>,
    pub label: String,
    pub comment: String,
    pub source: &'static str,
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
        source,
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
