use crate::{
    components::{
        CommandReceiver, CompletionRequest, DocumentLinks, Label, PositionComponent, Prefixes, RopeC, TokenComponent, Tokens, TripleComponent, TripleTarget, Triples
    }, lang::SimpleCompletion, utils::{position_to_offset, range_to_range}
};
use bevy_ecs::prelude::*;

mod diagnostics;
use chumsky::Span;
pub use diagnostics::publish_diagnostics;
mod semantics;
use lsp_types::CompletionItemKind;
pub use semantics::{semantic_tokens_system, SemanticTokensSchedule};
mod properties;
pub use properties::{
    complete_class, complete_properties, derive_classes, derive_properties, DefinedClass,
    DefinedProperty,
};
use tracing::{debug, info};

pub fn spawn_or_insert(
    url: lsp_types::Url,
    bundle: impl Bundle,
) -> impl (FnOnce(&mut World) -> Entity) + 'static + Send + Sync {
    move |world: &mut World| {
        if let Some(entity) = world
            .query::<(Entity, &Label)>()
            .iter(&world)
            .find(|x| x.1 .0 == url)
            .map(|x| x.0)
        {
            world.entity_mut(entity).insert(bundle);
            entity
        } else {
            world.spawn(bundle).id()
        }
    }
}

// pub fn handle_document_link(
//     trigger: Trigger<DocumentLinkEvent>,
//     query: Query<(Entity, &Label)>,
//     mut commands: Commands,
//     mut state: Local<HashMap<lsp_types::Url, HashSet<lsp_types::Url>>>,
// ) {
//     let mut entity_to_url = |entity: &Result<lsp_types::Url, Entity>| -> (Entity, lsp_types::Url) {
//         let check = |e: Entity, label: &Label| match entity {
//             Ok(url) => url == &label.0,
//             Err(entity) => e == *entity,
//         };
//         for (e, l) in &query {
//             if check(e, l) {
//                 return (e, l.0.clone());
//             }
//         }
//         if let Ok(url) = entity {
//             let e = commands.spawn(Label(url.clone())).id();
//             return (e, url.clone());
//         }
//
//         panic!("This cannot happen, I promise");
//     };
//
//     let (s_e, s_u) = entity_to_url(&trigger.event().source);
//     let (t_e, t_u) = entity_to_url(&trigger.event().target);
//
//     let should_insert = if let Some(set) = state.get_mut(&t_u) {
//         if !set.contains(&s_u) {
//             set.insert(s_u);
//             true
//         } else {
//             false
//         }
//     } else {
//         state.insert(t_u, [s_u].into_iter().collect());
//         true
//     };
//
//     if should_insert {
//         let reason = trigger.event().reason;
//         commands.entity(s_e).add(move |mut entity: EntityWorldMut| {
//             if let Some(mut links) = entity.get_mut::<DocumentLinks>() {
//                 links.push((t_u, reason));
//             } else {
//                 entity.insert(DocumentLinks(vec![(t_u, reason)]));
//             }
//         });
//     }
// }

pub fn handle_tasks(mut commands: Commands, mut receiver: ResMut<CommandReceiver>) {
    while let Ok(Some(mut com)) = receiver.0.try_next() {
        commands.append(&mut com);
    }
}

pub fn get_current_token(
    mut query: Query<(Entity, &Tokens, &PositionComponent, &RopeC)>,
    mut commands: Commands,
) {
    println!("Get current token!");
    for (entity, tokens, position, rope) in &mut query {
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

        let Some(range) = range_to_range(token.span(), &rope.0) else {
            debug!("Failed to transform span to range");
            continue;
        };

        info!("Get current found {:?} {:?}", token, range);
        let text = rope.0.slice(token.span().clone()).to_string();

        commands.entity(entity).insert(TokenComponent {
            token: token.clone(),
            range,
            text,
        });
    }
}

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
            .next()
        {
            info!("Current triples {:?} {:?}", t, triples.0);

            let mut target = TripleTarget::Subject;
            if offset > t.subject.span.end() {
                target = TripleTarget::Predicate;
            }
            if offset > t.predicate.span.end() {
                target = TripleTarget::Object;
            }

            commands.entity(e).insert(TripleComponent {
                triple: t.clone(),
                target,
            });
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
