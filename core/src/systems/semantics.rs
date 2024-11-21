use crate::{
    components::{Element, HighlightRequest, RopeC, Tokens},
    lang::{Lang, Token},
};
use bevy_ecs::prelude::*;
use bevy_ecs::schedule::ScheduleLabel;
use lsp_types::{SemanticToken, SemanticTokenType};
use tracing::info;

#[derive(ScheduleLabel, Clone, Eq, PartialEq, Debug, Hash)]
pub struct SemanticTokensSchedule;

struct T {
    start: usize,
    length: usize,
    ty: usize,
}

pub fn semantic_tokens_system<L: Lang>(
    mut query: Query<(&RopeC, &Tokens, Option<&Element<L>>, &mut HighlightRequest)>,
) {
    for (rope, tokens, element, mut req) in &mut query {
        info!("Found {} tokens", tokens.0.len());

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
