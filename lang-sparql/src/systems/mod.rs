use bevy_ecs::{prelude::*, system::Resource, world::World};
use lang_turtle::TriplesBuilder;
use lsp_core::{
    client::Client,
    components::*,
    systems::{
        derive_classes, derive_prefix_links, derive_properties, fetch_lov_properties,
        get_current_token, prefix::prefix_completion_helper,
    },
    Parse,
};
use sophia_iri::resolve::BaseIri;

use crate::{parsing::parse, tokenizer::tokenize, Sparql};

pub fn setup_parse<C: Client + Resource>(world: &mut World) {
    world.schedule_scope(Parse, |_, schedule| {
        schedule.add_systems((
            parse_source,
            parse_sparql_system.after(parse_source),
            derive_triples
                .after(parse_sparql_system)
                .before(fetch_lov_properties::<C>)
                .before(derive_classes)
                .before(derive_prefix_links)
                .before(derive_properties),
        ));
    });
}

pub fn setup_completion(world: &mut World) {
    world.schedule_scope(lsp_core::Completion, |_, schedule| {
        schedule.add_systems(sparql_lov_undefined_prefix_completion.after(get_current_token));
    });
}

#[instrument(skip(query, commands))]
fn parse_source(
    query: Query<(Entity, &Source), (Changed<Source>, With<Sparql>)>,
    mut commands: Commands,
) {
    for (entity, source) in &query {
        let (tok, es) = tokenize(source.0.as_str());
        info!("tokenized  {} tokens ({} errors)", tok.len(), es.len());
        commands.entity(entity).insert((Tokens(tok), Errors(es)));
    }
}

#[instrument(skip(query, commands))]
fn parse_sparql_system(
    query: Query<(Entity, &Source, &Tokens, &Label), (Changed<Tokens>, With<Sparql>)>,
    mut commands: Commands,
) {
    for (entity, source, tokens, label) in &query {
        let (jsonld, es) = parse(source.as_str(), label.0.clone(), tokens.0.clone());
        info!("{} triples ({} errors)", label.0, es.len());
        if es.is_empty() {
            let element = Element::<Sparql>(jsonld);
            commands
                .entity(entity)
                .insert((element, Errors(es)))
                .remove::<Dirty>();
        } else {
            let element = Element::<Sparql>(jsonld);
            commands.entity(entity).insert((Errors(es), element, Dirty));
        }
    }
}

#[instrument(skip(query, commands))]
fn derive_triples(
    query: Query<(Entity, &Element<Sparql>), Changed<Element<Sparql>>>,
    mut commands: Commands,
) {
    for (e, el) in &query {
        let query = el.0.value();

        let prefixes: Vec<_> = query
            .prefixes
            .iter()
            .flat_map(|prefix| {
                let url = prefix.value.expand(query)?;
                let url = lsp_types::Url::parse(&url).ok()?;
                Some(Prefix {
                    url,
                    prefix: prefix.prefix.value().clone(),
                })
            })
            .collect();

        commands.entity(e).insert(Prefixes(prefixes));

        if let Ok(base) = BaseIri::new(query.base.to_string()) {
            let mut builder = TriplesBuilder::new(query, base);
            let _ = query.ingest_triples(&mut builder);
            let triples: Vec<_> = builder.triples.into_iter().map(|x| x.to_owned()).collect();

            commands.entity(e).insert(Triples(triples));
        }
        // let prefix = triples::derive_prefixes(&el);
        // let triples = triples::derive_triples(&el, &prefix);
    }
}

pub fn sparql_lov_undefined_prefix_completion(
    mut query: Query<(
        &TokenComponent,
        &Element<Sparql>,
        &Prefixes,
        &mut CompletionRequest,
    )>,
) {
    for (word, turtle, prefixes, mut req) in &mut query {
        let mut start = Position::new(0, 0);

        if turtle.base_statement.is_some() {
            start = Position::new(1, 0);
        }

        use lsp_types::{Position, Range};
        prefix_completion_helper(word, prefixes, &mut req.0, |lov| {
            Some(vec![lsp_types::TextEdit {
                range: Range::new(start.clone(), start),
                new_text: format!("PREFIX {}: <{}>\n", lov.name, lov.location),
            }])
        });
    }
}
