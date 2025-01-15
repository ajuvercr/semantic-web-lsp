use std::{
    borrow::{BorrowMut, Cow},
    collections::{HashMap, HashSet},
    ops::{Deref, DerefMut},
};

use crate::{components::*, features::diagnostic::DiagnosticPublisher, utils::range_to_range};
use bevy_ecs::prelude::*;
use lsp_types::{DiagnosticSeverity, TextDocumentItem};
use ropey::Rope;
use rudof_lib::{
    shacl_ast::{
        compiled::{schema::CompiledSchema, shape::CompiledShape},
        ShaclParser,
    },
    shacl_validation::{
        shacl_processor::{GraphValidation, ShaclProcessor},
        shape::{FocusNodesOps, Validate},
        store::graph::Graph,
    },
    srdf::{Object, SRDFGraph},
    RdfData, Rudof, RudofConfig, SRDFSparql,
};
use sophia_api::prelude::Quad as _;
use tracing::{error, info};

#[derive(Debug, Component)]
pub struct Shapes {
    rudof: Rudof,
}
unsafe impl Send for Shapes {}
unsafe impl Sync for Shapes {}

enum MyCow<'a, T> {
    Borrowed(&'a mut T),
    Owned(T),
}
impl<T> Deref for MyCow<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            MyCow::Borrowed(x) => &x,
            MyCow::Owned(x) => &x,
        }
    }
}

impl<T> DerefMut for MyCow<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            MyCow::Borrowed(x) => x,
            MyCow::Owned(x) => x,
        }
    }
}

fn get_reader<'a>(rope: &'a Rope) -> impl std::io::Read + 'a {
    use std::io::prelude::*;
    let reader: Box<dyn Read> = rope
        .chunks()
        .map(|x| std::io::Cursor::new(x.as_bytes()))
        .fold(Box::new(std::io::Cursor::new(&[])), |acc, chunk| {
            Box::new(acc.chain(chunk))
        });
    reader
}

type ShaclSchema = CompiledSchema<RdfData>;
pub fn derive_shapes(
    query: Query<(Entity, &RopeC, &Label), (Changed<Triples>, Without<Dirty>)>,
    mut commands: Commands,
) {
    for (e, rope, label) in &query {
        if let Some(compiled) = SRDFGraph::from_reader(
            get_reader(&rope),
            &rudof_lib::RDFFormat::Turtle,
            Some(label.0.as_str()),
            &rudof_lib::ReaderMode::Lax,
        )
        .ok()
        .and_then(|data| RdfData::from_graph(data).ok())
        .and_then(|data| ShaclParser::new(data.clone()).parse().ok())
        .and_then(|shacl| ShaclSchema::try_from(shacl).ok())
        {
            info!("Compiled shapes for {} {:?}", label.as_str(), compiled);
            commands.entity(e).insert(Wrapped(compiled));
        } else {
            info!("Failed to compile shapes for {}", label.as_str());
        }
    }
}

pub fn validate_shapes(
    query: Query<
        (
            Entity,
            &RopeC,
            &Label,
            &DocumentLinks,
            &Wrapped<TextDocumentItem>,
            &Triples,
        ),
        (Changed<Triples>, Without<Dirty>),
    >,
    other: Query<(&Label, &Wrapped<ShaclSchema>, &Prefixes)>,
    mut client: ResMut<DiagnosticPublisher>,
) {
    info!("Validate shapes");

    for (e, rope, label, links, item, triples) in &query {
        let mut diagnostics: Vec<lsp_types::Diagnostic> = Vec::new();

        if let Some(validator) = SRDFGraph::from_reader(
            get_reader(&rope),
            &rudof_lib::RDFFormat::Turtle,
            Some(label.0.as_str()),
            &rudof_lib::ReaderMode::Lax,
        )
        .ok()
        .and_then(|data| RdfData::from_graph(data).ok())
        .map(|data| {
            GraphValidation::from_graph(
                Graph::from_data(data),
                rudof_lib::ShaclValidationMode::Native,
            )
        }) {
            info!("Created graph validator for {}", label.as_str());
            for (other_label, schema, prefixes) in &other {
                if links
                    .iter()
                    .find(|link| link.0.as_str().starts_with(other_label.0.as_str()))
                    .is_none()
                    && label.0 != other_label.0
                {
                    continue;
                }

                info!("Schema {}", other_label.as_str());
                for (_, s) in schema.iter() {
                    if let Ok(res) = s.validate(validator.store(), validator.runner(), None) {
                        if !res.is_empty() {
                            info!("Validations for {:?}", s);
                        }

                        let get_path = |source: Option<&Object>| {
                            let source = source?;
                            let property =
                                s.property_shapes()
                                    .iter()
                                    .find(|x| match (x.id(), source) {
                                        (
                                            rudof_lib::oxrdf::Term::NamedNode(named_node),
                                            Object::Iri(iri_s),
                                        ) => named_node.as_str() == iri_s.as_str(),
                                        (
                                            rudof_lib::oxrdf::Term::BlankNode(blank_node),
                                            Object::BlankNode(st),
                                        ) => blank_node.as_str() == st.as_str(),
                                        _ => false,
                                    })?;
                            let path = property.path_str()?;
                            Some(prefixes.shorten(&path).unwrap_or(path))
                        };

                        let mut per_fn_per_path = HashMap::new();

                        for r in &res {
                            info!("Res {:?}", r);
                            let foc = r.focus_node().to_string();

                            let mut done = std::collections::HashSet::new();
                            for t in &triples.0 {
                                if t.s().as_str() == &foc && !done.contains(t.s()) {
                                    done.insert(t.s().to_owned());

                                    let entry: &mut HashMap<String, Vec<String>> =
                                        per_fn_per_path.entry(t.s().span.clone()).or_default();

                                    if let Some(range) = range_to_range(&t.s().span, &rope) {
                                        let path = get_path(r.source()).unwrap_or(String::new());
                                        let entry = entry.entry(path).or_default();

                                        let component = r.component().to_string();
                                        let component =
                                            prefixes.shorten(&component).unwrap_or(component);

                                        entry.push(component);
                                    }
                                }
                            }
                        }

                        for (range, per_path) in per_fn_per_path {
                            if let Some(range) = range_to_range(&range, &rope) {
                                for (path, components) in per_path {
                                    let mut comps = components[0].clone();
                                    for c in components.into_iter().skip(1) {
                                        comps += ", ";
                                        comps += &c;
                                    }

                                    diagnostics.push(lsp_types::Diagnostic {
                                        range: range.clone(),
                                        severity: Some(DiagnosticSeverity::ERROR),
                                        source: Some(String::from("SWLS")),
                                        message: format!("Path {} violates {}", path, comps),
                                        related_information: None,
                                        ..Default::default()
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }

        let _ = client.publish(&item.0, diagnostics, "shacl_validation");
    }
}
