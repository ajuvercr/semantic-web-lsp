use bevy_ecs::prelude::*;
use chumsky::Parser;
use lsp_core::components::*;
use tracing::info;
use tracing::instrument;

use crate::TurtleComponent;
use crate::TurtleLang;
use crate::{parse_turtle, tokenizer::parse_tokens};

#[instrument(skip(query, commands))]
pub fn parse_source(
    query: Query<(Entity, &Source), (Changed<Source>, With<TurtleComponent>)>,
    mut commands: Commands,
) {
    for (entity, source) in &query {
        let (tok, es) = parse_tokens().parse_recovery(source.0.as_str());
        if let Some(tokens) = tok {
            let t = Tokens(tokens);
            commands.entity(entity).insert(t);
        }
        commands.entity(entity).insert(Errors(es));
    }
}

#[instrument(skip(query, commands))]
pub fn parse_turtle_system(
    query: Query<(Entity, &Source, &Tokens, &Label), (Changed<Tokens>, With<TurtleComponent>)>,
    mut commands: Commands,
) {
    for (entity, source, tokens, label) in &query {
        let (turtle, es) = parse_turtle(&label.0, tokens.0.clone(), source.0.len());
        info!(
            "{} triples ({} errors)",
            turtle.value().triples.len(),
            es.len()
        );
        if es.is_empty() {
            let element = Element::<TurtleLang>(turtle);
            commands
                .entity(entity)
                .insert((element, Errors(es)))
                .remove::<Dirty>();
        } else {
            let element = Element::<TurtleLang>(turtle);
            commands.entity(entity).insert((Errors(es), element, Dirty));
        }
    }
}

pub fn derive_triples(
    query: Query<
        (Entity, &Element<TurtleLang>),
        (Changed<Element<TurtleLang>>, With<TurtleComponent>),
    >,
    mut commands: Commands,
) {
    for (entity, turtle) in &query {
        if let Ok(tripl) = turtle.0.get_simple_triples() {
            let triples: Vec<_> = tripl.iter().map(|x| x.to_owned()).collect();
            commands.entity(entity).insert(Triples(triples));
        }
    }
}
