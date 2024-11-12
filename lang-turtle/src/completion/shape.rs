use hashbrown::HashMap;
use lsp_core::{lang::SimpleCompletion, triples::Triples};
use lsp_types::Range;
use tracing::info;

use crate::{
    green::{ClassProvider, Property, PropertyProvider},
    shacl::{parse_shapes, Shape},
    Turtle,
};

#[derive(Clone, Debug, Default)]
pub struct ShapeCompletionProviderState {}

#[derive(Clone, Default)]
pub struct ShapeCompletionProvider {
    shapes: HashMap<String, Vec<Shape>>,
}

impl PropertyProvider for ShapeCompletionProvider {
    fn provide<'a>(
        &mut self,
        triples: &'a Triples<'a>,
        class_provider: &mut impl crate::green::ClassProvider,
    ) -> Vec<crate::green::Property> {
        self.set_shapes(&triples, class_provider)
    }
}

impl ShapeCompletionProvider {
    fn set_shapes(
        &mut self,
        triples: &Triples<'_>,
        provider: &mut impl ClassProvider,
    ) -> Vec<Property> {
        let shapes = parse_shapes(&triples);

        let out = shapes
            .iter()
            .flat_map(|x| x.get_properties(provider))
            .collect();

        self.shapes.insert(triples.base_url.clone(), shapes);

        out
    }

    pub fn find_snippets(
        &self,
        class: &str,
        turtle: &Turtle,
        range: Range,
        completions: &mut Vec<SimpleCompletion>,
    ) {
        completions.extend(
            self.shapes
                .values()
                .flatten()
                .filter(|shape| shape.name() == class)
                .flat_map(|prop: &Shape| {
                    info!("Looking into shape {}", prop.name());
                    prop.into_completion(turtle, range)
                }),
        );
    }
}
