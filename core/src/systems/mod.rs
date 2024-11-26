use crate::{
    components::{
        CommandReceiver, CompletionRequest, DocumentLinks, DynLang, Label, PositionComponent,
        Prefixes, RopeC, TokenComponent, Tokens, TripleComponent, TripleTarget, Triples,
    },
    lang::SimpleCompletion,
    utils::{position_to_offset, range_to_range},
    CreateEvent, Parse,
};
use bevy_ecs::prelude::*;

mod diagnostics;
pub use diagnostics::publish_diagnostics;
mod semantics;
use lsp_types::CompletionItemKind;
pub use semantics::{
    basic_semantic_tokens, semantic_tokens_system, SemanticTokensSchedule, TokenTypesComponent,
};
mod properties;
pub use properties::{
    complete_class, complete_properties, derive_classes, derive_properties, DefinedClass,
    DefinedProperty,
};
mod lov;
pub use lov::fetch_lov_properties;

use tracing::{debug, instrument};

pub fn spawn_or_insert(
    url: lsp_types::Url,
    bundle: impl Bundle,
    language_id: Option<String>,
    extra: impl Bundle,
) -> impl (FnOnce(&mut World) -> Entity) + 'static + Send + Sync {
    move |world: &mut World| {
        let out = if let Some(entity) = world
            .query::<(Entity, &Label)>()
            .iter(&world)
            .find(|x| x.1 .0 == url)
            .map(|x| x.0)
        {
            world.entity_mut(entity).insert(bundle).insert(extra);
            entity
        } else {
            let entity = world.spawn(bundle).insert(extra).id();
            world.trigger_targets(CreateEvent { url, language_id }, entity);
            entity
        };

        world.flush_commands();
        world.run_schedule(Parse);
        out
    }
}

pub fn handle_tasks(mut commands: Commands, mut receiver: ResMut<CommandReceiver>) {
    while let Ok(Some(mut com)) = receiver.0.try_next() {
        commands.append(&mut com);
    }
}

#[instrument(skip(query, commands))]
pub fn get_current_token(
    mut query: Query<(Entity, &Tokens, &PositionComponent, &RopeC, &DynLang)>,
    mut commands: Commands,
) {
    for (entity, tokens, position, rope, helper) in &mut query {
        let Some(offset) = position_to_offset(position.0, &rope.0) else {
            debug!("Couldn't transform to an offset");
            continue;
        };

        let Some(token) = tokens
            .0
            .iter()
            .filter(|x| x.span().contains(&offset))
            .min_by_key(|x| x.span().end - x.span().start)
        else {
            let closest = tokens.0.iter().min_by_key(|x| {
                let start = if offset > x.span().start {
                    offset - x.span().start
                } else {
                    x.span().start - offset
                };

                let end = if offset > x.span().end {
                    offset - x.span().end
                } else {
                    x.span().end - offset
                };

                if start > end {
                    end
                } else {
                    start
                }
            });
            debug!(
                "Failed to find a token, offset {} closest {:?}",
                offset, closest
            );
            continue;
        };

        let (text, range) = helper.get_relevant_text(token, rope);
        let Some(range) = range_to_range(&range, &rope.0) else {
            debug!("Failed to transform span to range");
            continue;
        };

        debug!("Current token {:?} {}", token, text);
        commands.entity(entity).insert(TokenComponent {
            token: token.clone(),
            range,
            text,
        });
    }
}

#[instrument(skip(query, commands))]
pub fn get_current_triple(
    query: Query<(Entity, &PositionComponent, &Triples, &RopeC)>,
    mut commands: Commands,
) {
    for (e, position, triples, rope) in &query {
        commands.entity(e).remove::<TripleComponent>();

        let Some(offset) = position_to_offset(position.0, &rope.0) else {
            debug!("Couldn't transform to an offset");
            continue;
        };

        if let Some(t) = triples
            .0
            .iter()
            .filter(|triple| triple.span.contains(&offset))
            .min_by_key(|x| x.span.end - x.span.start)
        {
            let target = [
                (TripleTarget::Subject, &t.subject.span),
                (TripleTarget::Predicate, &t.predicate.span),
                (TripleTarget::Object, &t.object.span),
            ]
            .into_iter()
            .filter(|x| x.1.contains(&offset))
            .min_by_key(|x| x.1.end - x.1.start)
            .map(|x| x.0)
            .unwrap_or(TripleTarget::Subject);

            debug!("Current triple {} {:?}", t, target);
            commands.entity(e).insert(TripleComponent {
                triple: t.clone(),
                target,
            });
        } else {
            debug!("No current triple found");
            for t in &triples.0 {
                println!("triple {}", t);
            }
        }
    }
}

pub fn derive_prefix_links(
    mut query: Query<(Entity, &Prefixes, Option<&mut DocumentLinks>), Changed<Prefixes>>,
    mut commands: Commands,
) {
    const SOURCE: &'static str = "prefix import";
    for (e, turtle, mut links) in &mut query {
        let new_links: Vec<_> = turtle.iter().map(|u| (u.url.clone(), SOURCE)).collect();
        if let Some(links) = links.as_mut() {
            links.retain(|e| e.1 != SOURCE);
        }
        match (new_links.is_empty(), links) {
            (false, None) => {
                commands.entity(e).insert(DocumentLinks(new_links));
            }
            (false, Some(mut links)) => {
                links.extend(new_links);
            }
            _ => {}
        }
    }
}

#[instrument(skip(query))]
pub fn defined_prefix_completion(
    mut query: Query<(&TokenComponent, &Prefixes, &mut CompletionRequest)>,
) {
    for (word, prefixes, mut req) in &mut query {
        let st = &word.text;
        let pref = if let Some(idx) = st.find(':') {
            &st[..idx]
        } else {
            &st
        };

        debug!("matching {}", pref);

        let completions = prefixes
            .iter()
            .filter(|p| p.prefix.as_str().starts_with(pref))
            .flat_map(|x| {
                let new_text = format!("{}:", x.prefix.as_str());
                if new_text != word.text {
                    Some(
                        SimpleCompletion::new(
                            CompletionItemKind::MODULE,
                            format!("{}", x.prefix.as_str()),
                            lsp_types::TextEdit {
                                new_text,
                                range: word.range.clone(),
                            },
                        )
                        .documentation(x.url.as_str()),
                    )
                } else {
                    None
                }
            });

        req.0.extend(completions);
    }
}
