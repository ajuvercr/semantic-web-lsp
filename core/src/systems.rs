use crate::{
    components::{CommandReceiver, Element, HighlightRequest, RopeC, Tokens},
    lang::{Lang, Token},
};
use bevy_ecs::{prelude::*, schedule::ScheduleLabel};
use lsp_types::{SemanticToken, SemanticTokenType};

pub mod semantic_tokens {
    use bevy_ecs::prelude::World;
    use bevy_ecs::schedule::ScheduleLabel;
    use lsp_types::SemanticTokensResult;

    use crate::components::HighlightRequest;

    use super::LspSystem;

    #[derive(ScheduleLabel, Clone, Eq, PartialEq, Debug, Hash)]
    pub struct Label;

    pub struct SemanticTokenSystem;
    impl LspSystem for SemanticTokenSystem {
        type SchedulLabel = Label;
        type Out = SemanticTokensResult;

        fn invoke(world: &mut World, entity: bevy_ecs::entity::Entity) -> Option<Self::Out> {
            world.entity_mut(entity).insert(HighlightRequest(vec![]));
            world.run_schedule(Label);
            world
                .entity_mut(entity)
                .take::<HighlightRequest>()
                .map(|x| {
                    SemanticTokensResult::Tokens(lsp_types::SemanticTokens {
                        result_id: None,
                        data: x.0,
                    })
                })
        }
    }
}

pub trait LspSystem {
    type SchedulLabel: ScheduleLabel;
    type Out;

    fn invoke(world: &mut World, entity: Entity) -> Option<Self::Out>;
}

struct T {
    start: usize,
    length: usize,
    ty: usize,
}

pub fn semantic_tokens_system<L: Lang>(
    mut query: Query<(
        &RopeC,
        &Tokens<L>,
        Option<&Element<L>>,
        &mut HighlightRequest,
    )>,
) {
    for (rope, tokens, element, mut req) in &mut query {
        let rope = &rope.0;
        let mut ts: Vec<Option<SemanticTokenType>> = Vec::with_capacity(rope.len_chars());
        ts.resize(rope.len_chars(), None);
        tokens.0.iter().for_each(|token| {
            Token::span_tokens(token)
                .into_iter()
                .for_each(|(token, span)| span.for_each(|j| ts[j] = Some(token.clone())));
        });

        let _ = element;

        let mut last = None;
        let mut start = 0;
        let mut out_tokens = Vec::new();
        for (i, ty) in ts.into_iter().enumerate() {
            if last != ty {
                if let Some(t) = last {
                    out_tokens.push(T {
                        start,
                        length: i - start,
                        ty: L::LEGEND_TYPES.iter().position(|x| x == &t).unwrap_or(0),
                    });
                }

                last = ty;
                start = i;
            }
        }

        if let Some(t) = last {
            out_tokens.push(T {
                start,
                length: rope.len_chars() - start,
                ty: L::LEGEND_TYPES.iter().position(|x| x == &t).unwrap_or(0),
            });
        }

        let mut pre_line = 0;
        let mut pre_start = 0;
        req.0 = out_tokens
            .into_iter()
            .flat_map(|token| {
                let line = rope.try_byte_to_line(token.start as usize).ok()? as u32;
                let first = rope.try_line_to_char(line as usize).ok()? as u32;
                let start = rope.try_byte_to_char(token.start as usize).ok()? as u32 - first;
                let delta_line = line - pre_line;
                let delta_start = if delta_line == 0 {
                    start - pre_start
                } else {
                    start
                };
                let ret = Some(SemanticToken {
                    delta_line,
                    delta_start,
                    length: token.length as u32,
                    token_type: token.ty as u32,
                    token_modifiers_bitset: 0,
                });
                pre_line = line;
                pre_start = start;
                ret
            })
            .collect();
    }
}

// pub fn schedule_tokenizer<L: Lang, P, F, P2>(world: &mut World, parser: P, get_parse: F)
// where
//     P: Parser<char, Vec<Spanned<L::Token>>, Error = L::TokenError>,
//     L::Token: 'static,
//     L::Element: 'static,
//     L::TokenError: chumsky::Error<char, Span = std::ops::Range<usize>> + 'static,
//     F: Fn(&lsp_types::TextDocumentItem) -> P2,
//     P2: Parser<L::Token, Spanned<L::Element>, Error = L::ElementError>,
//     L::ElementError: chumsky::Error<L::Token, Span = std::ops::Range<usize>> + 'static,
// {
//     let tokenize_system = move |query: Query<(Entity, &Source), Changed<Source>>,
//                                 mut commands: Commands| {
//         for (entity, source) in &query {
//             let (tok, es) = parser.parse_recovery(source.0.as_str());
//             if let Some(tokens) = tok {
//                 let t = Tokens(tokens);
//                 commands.entity(entity).insert(t);
//             }
//             commands.entity(entity).insert(Errors(es));
//         }
//     };
//
//     let parse_system = move |query: Query<
//         (
//             Entity,
//             &Wrapped<lsp_types::TextDocumentItem>,
//             &Tokens<L::Token>,
//             &Label,
//         ),
//         (Changed<Tokens<L::Token>>),
//     >,
//                              mut commands: Commands| {
//         for (entity, source, tokens, label) in &query {
//             let len = source.0.text.len();
//             let rev_range = |range: std::ops::Range<usize>| (len - range.end)..(len - range.start);
//             let parser = get_parse(&source.0);
//
//             let stream = chumsky::Stream::from_iter(
//                 0..len,
//                 tokens
//                     .0
//                     .iter()
//                     .cloned()
//                     .rev()
//                     // .filter(|x| !x.is_comment())
//                     .map(|Spanned(x, s)| (x, rev_range(s))),
//             );
//             let (element, es) = parser.parse_recovery(stream);
//
//             if es.is_empty() {
//                 commands
//                     .entity(entity)
//                     .insert((Wrapped(element), Errors(es)));
//             } else {
//                 commands.entity(entity).insert(Errors(es));
//             }
//         }
//     };
// }

/// This system queries for entities that have our Task<Transform> component. It polls the
/// tasks to see if they're complete. If the task is complete it takes the result, adds a
/// new [`Mesh3d`] and [`MeshMaterial3d`] to the entity using the result from the task's work, and
/// removes the task component from the entity.
pub fn handle_tasks(mut commands: Commands, mut receiver: ResMut<CommandReceiver>) {
    while let Ok(Some(mut com)) = receiver.0.try_next() {
        commands.append(&mut com);
    }
}
