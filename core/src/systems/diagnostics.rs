use crate::{
    components::*,
    lang::{Lang, OtherPublisher, SimpleDiagnostic},
    utils::offset_to_position,
};
use bevy_ecs::prelude::*;
use lsp_types::{Diagnostic, TextDocumentItem};

pub fn publish_diagnostics<L: Lang>(
    query: Query<
        (
            &Errors<L::TokenError>,
            &Errors<L::ElementError>,
            &Wrapped<TextDocumentItem>,
            &RopeC,
        ),
        (
            Or<(
                Changed<Errors<L::TokenError>>,
                Changed<Errors<L::ElementError>>,
            )>,
        ),
    >,
    mut client: ResMut<OtherPublisher>,
) where
    L::TokenError: 'static + Clone,
    L::ElementError: 'static + Clone,
{
    for (token_errors, element_errors, params, rope) in &query {
        use std::iter::Iterator as _;
        let token_iter = token_errors
            .0
            .iter()
            .cloned()
            .map(|x| Into::<SimpleDiagnostic>::into(x));
        let turtle_iter = element_errors
            .0
            .iter()
            .cloned()
            .map(|x| Into::<SimpleDiagnostic>::into(x));

        let diagnostics: Vec<_> = Iterator::chain(token_iter, turtle_iter)
            .flat_map(|item| {
                let (span, message) = (item.range, item.msg);
                let start_position = offset_to_position(span.start, &rope.0)?;
                let end_position = offset_to_position(span.end, &rope.0)?;
                Some(Diagnostic {
                    range: lsp_types::Range::new(start_position, end_position),
                    message,
                    severity: item.severity,
                    ..Default::default()
                })
            })
            .collect();

        let _ = client.publish(&params.0, diagnostics, "syntax");
    }
}
