use crate::prelude::*;
use crate::{
    components::{
        CommandReceiver, CompletionRequest, DocumentLinks, DynLang, InlayRequest, Label,
        PositionComponent, Prefixes, PrepareRenameRequest, RenameEdits, RopeC, TokenComponent,
        Tokens, TripleComponent, TripleTarget, Triples, Wrapped,
    },
    utils::{offset_to_position, position_to_offset, range_to_range},
    CreateEvent, Parse,
};
use bevy_ecs::prelude::*;

mod shapes;
pub use shapes::*;
mod typed;
pub use typed::*;
// mod diagnostics;
pub mod prefix;
// pub use diagnostics::publish_diagnostics;
mod semantics;
use lsp_types::{CompletionItemKind, Diagnostic, DiagnosticSeverity, TextDocumentItem, TextEdit};
pub use semantics::{
    basic_semantic_tokens, semantic_tokens_system, SemanticTokensSchedule, TokenTypesComponent,
};
mod properties;
pub use properties::{
    complete_class, complete_properties, derive_classes, derive_properties, hover_class,
    hover_property, DefinedClass, DefinedProperty,
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

#[instrument(skip(query))]
pub fn keyword_complete(
    mut query: Query<(
        Option<&TokenComponent>,
        &PositionComponent,
        &DynLang,
        &mut CompletionRequest,
    )>,
) {
    tracing::info!("Keyword complete!");
    for (m_token, position, helper, mut req) in &mut query {
        let range = if let Some(ct) = m_token {
            ct.range
        } else {
            lsp_types::Range {
                start: position.0,
                end: position.0,
            }
        };

        for kwd in helper.keyword() {
            let completion = SimpleCompletion::new(
                CompletionItemKind::KEYWORD,
                kwd.to_string(),
                lsp_types::TextEdit {
                    range: range.clone(),
                    new_text: kwd.to_string(),
                },
            );
            req.push(completion);
        }
    }
}

#[instrument(skip(query, commands))]
pub fn get_current_token(
    mut query: Query<(Entity, &Tokens, &PositionComponent, &RopeC, &DynLang)>,
    mut commands: Commands,
) {
    for (entity, tokens, position, rope, helper) in &mut query {
        commands.entity(entity).remove::<TokenComponent>();
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

        for t in triples.iter() {
            debug!("Triple {}", t);
        }

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
        let new_links: Vec<_> = turtle.0.iter().map(|u| (u.url.clone(), SOURCE)).collect();
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
            .0
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

pub fn undefined_prefix(
    query: Query<
        (&Tokens, &Prefixes, &Wrapped<TextDocumentItem>, &RopeC),
        Or<(Changed<Prefixes>, Changed<Tokens>)>,
    >,
    mut client: ResMut<DiagnosticPublisher>,
) {
    for (tokens, prefixes, item, rope) in &query {
        let mut diagnostics: Vec<Diagnostic> = Vec::new();
        for t in &tokens.0 {
            match t.value() {
                crate::token::Token::PNameLN(x, _) => {
                    let pref = x.as_ref().map(|x| x.as_str()).unwrap_or("");
                    let found = prefixes.0.iter().find(|x| x.prefix == pref).is_some();
                    if !found {
                        if let Some(range) = range_to_range(t.span(), &rope) {
                            diagnostics.push(Diagnostic {
                                range,
                                severity: Some(DiagnosticSeverity::ERROR),
                                source: Some(String::from("SWLS")),
                                message: format!("Undefined prefix {}", pref),
                                related_information: None,
                                ..Default::default()
                            })
                        }
                    }
                }
                _ => {}
            }
        }
        let _ = client.publish(&item.0, diagnostics, "undefined_prefix");
    }
}

#[instrument(skip(query))]
pub fn inlay_triples(mut query: Query<(&Triples, &RopeC, &mut InlayRequest)>) {
    for (triples, rope, mut req) in &mut query {
        let mut out = Vec::new();
        for t in triples.iter() {
            let Some(position) = offset_to_position(t.span.end, &rope) else {
                continue;
            };
            out.push(lsp_types::InlayHint {
                position,
                label: lsp_types::InlayHintLabel::String(format!("{}", t)),
                kind: None,
                text_edits: None,
                tooltip: None,
                padding_left: None,
                padding_right: None,
                data: None,
            });
        }
        req.0 = Some(out);
    }
}

pub fn triples() {}
pub fn prefixes() {}

#[instrument(skip(query, commands,))]
pub fn prepare_rename(query: Query<(Entity, Option<&TokenComponent>)>, mut commands: Commands) {
    for (e, m_token) in &query {
        commands.entity(e).remove::<(PrepareRenameRequest,)>();
        if let Some(token) = m_token {
            let renameable = match token.token.value() {
                crate::token::Token::Variable(_) => true,
                crate::token::Token::IRIRef(_) => true,
                crate::token::Token::PNameLN(_, _) => true,
                crate::token::Token::BlankNodeLabel(_) => true,
                _ => false,
            };

            if renameable {
                commands.entity(e).insert(PrepareRenameRequest {
                    range: token.range.clone(),
                    placeholder: token.text.clone(),
                });
                continue;
            }
        }
        tracing::info!("Didn't find a good token");
    }
}

#[instrument(skip(query,))]
pub fn rename(mut query: Query<(&TokenComponent, &Tokens, &RopeC, &Label, &mut RenameEdits)>) {
    for (token, tokens, rope, label, mut edits) in &mut query {
        tracing::info!("Token {:?}", token);
        let new_text = edits.1.clone();
        for t in tokens.0.iter().filter(|x| x.value() == token.token.value()) {
            tracing::info!("Changing {:?}", t);
            if let Some(range) = range_to_range(t.span(), &rope.0) {
                edits.0.push((
                    label.0.clone(),
                    TextEdit {
                        range,
                        new_text: new_text.clone(),
                    },
                ))
            }
        }
        // commands.entity(e).insert(PrepareRenameRequest {
        //     range: token.range.clone(),
        //     placeholder: token.text.clone(),
        // });
    }
}
