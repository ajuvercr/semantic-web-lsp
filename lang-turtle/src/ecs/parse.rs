use std::{
    collections::HashMap,
    ops::{DerefMut, Index},
};

use bevy_ecs::prelude::*;
use lsp_core::prelude::*;
use tracing::{info, instrument};

use crate::{
    lang::{context::Context, parser::parse_turtle, tokenizer::parse_tokens_str},
    TurtleLang,
};

#[instrument(skip(query, commands), name = "parse_source")]
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

struct TokenIdx<'a> {
    tokens: &'a Vec<Spanned<Token>>,
}
impl<'a> Index<usize> for TokenIdx<'a> {
    type Output = Token;

    fn index(&self, index: usize) -> &Self::Output {
        &self.tokens[index].value()
    }
}

#[instrument(skip(query, commands, old), name = "parse_turtle")]
pub fn parse_turtle_system(
    query: Query<
        (Entity, &Source, &Tokens, &Label, Option<&Open>),
        (Changed<Tokens>, With<TurtleLang>),
    >,
    mut commands: Commands,
    mut old: Local<HashMap<String, (Vec<Spanned<Token>>, Context)>>,
) {
    for (entity, source, tokens, label, open) in &query {
        let (ref mut old_tokens, ref mut context) = old.entry(label.to_string()).or_default();
        context.setup_current_to_prev(
            TokenIdx { tokens: &tokens },
            tokens.len(),
            TokenIdx {
                tokens: &old_tokens,
            },
            old_tokens.len(),
        );
        let ctx = context.ctx();

        if open.is_some() {
            for (i, nt) in tokens.iter().enumerate() {
                let span = nt.span();
                info!(
                    "Token {} ({}, {}, {})",
                    &source[span.start..span.end],
                    ctx.was_subject(i),
                    ctx.was_predicate(i),
                    ctx.was_object(i)
                );
            }
        }

        let (turtle, es) = parse_turtle(&label.0, tokens.0.clone(), source.0.len(), context.ctx());

        let es: Vec<_> = es
            .into_iter()
            .map(|(idx, e)| (idx, e.map(|PToken(t, _)| t)))
            .collect();

        info!(
            "{} triples ({} errors)",
            turtle.value().triples.len(),
            es.len()
        );

        *old_tokens = tokens.0.clone();
        context.clear();

        // TODO: Setup subject predicate and objects

        turtle.set_context(context);

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
