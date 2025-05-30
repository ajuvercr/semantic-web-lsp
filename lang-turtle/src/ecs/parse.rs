use bevy_ecs::prelude::*;
use lsp_core::prelude::*;
use tracing::info;

use crate::{
    lang::{parser::parse_turtle, tokenizer::parse_tokens_str},
    TurtleLang,
};

// #[instrument(skip(query, commands), name = "parse_source")]
pub fn parse_source(
    query: Query<(Entity, &Source), (Changed<Source>, With<TurtleLang>)>,
    mut commands: Commands,
) {
    for (entity, source) in &query {
        let (tok, es) = parse_tokens_str(source.0.as_str());
        let t = Tokens(tok);
        commands.entity(entity).insert(t);
        commands.entity(entity).insert(Errors(es));
    }
}

// #[instrument(skip(query, commands), name = "parse_turtle")]
pub fn parse_turtle_system(
    query: Query<(Entity, &Source, &Tokens, &Label), (Changed<Tokens>, With<TurtleLang>)>,
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

// #[instrument(skip(query, commands), name = "derive_triples")]
pub fn derive_triples(
    query: Query<(Entity, &Element<TurtleLang>), (Changed<Element<TurtleLang>>, With<TurtleLang>)>,
    mut commands: Commands,
) {
    for (entity, turtle) in &query {
        if let Ok(tripl) = turtle.0.get_simple_triples() {
            let triples: Vec<_> = tripl.iter().map(|x| x.to_owned()).collect();
            commands.entity(entity).insert(Triples(triples));
        }
    }
}
