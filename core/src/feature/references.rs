use bevy_ecs::{
    component::Component,
    schedule::{IntoSystemConfigs, Schedule, ScheduleLabel},
    world::World,
};
use lsp_types::Location;

pub use crate::{
    systems::{hover_class, hover_property, hover_types, infer_types},
    util::{token::get_current_token, triple::get_current_triple},
};

/// [`Component`] indicating that the current document is currently handling a References request.
#[derive(Component, Debug, Default)]
pub struct ReferencesRequest(pub Vec<Location>);

/// [`ScheduleLabel`] related to the Parse schedule
#[derive(ScheduleLabel, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Label;

pub fn setup_schedule(world: &mut World) {
    let mut references = Schedule::new(Label);
    references.add_systems((
        get_current_token,
        get_current_triple.after(get_current_token),
        system::add_references.after(get_current_triple),
    ));
    world.add_schedule(references);
}

mod system {
    use bevy_ecs::prelude::*;
    use lsp_types::Location;
    use references::ReferencesRequest;
    use ropey::Rope;
    use sophia_api::{
        quad::Quad,
        term::{Term, TermKind},
    };

    fn token_to_location(token: &Spanned<Token>, label: &Label, rope: &Rope) -> Option<Location> {
        let range = range_to_range(token.span(), rope)?;
        Some(Location {
            range,
            uri: label.0.clone(),
        })
    }

    use crate::prelude::*;
    pub fn add_references(
        mut query: Query<(
            &TokenComponent,
            &TripleComponent,
            &Tokens,
            &Label,
            &RopeC,
            &mut ReferencesRequest,
        )>,
        project: Query<(&Tokens, &RopeC, &Label), With<Open>>,
    ) {
        for (token, triple, tokens, label, rope, mut req) in &mut query {
            let target = match triple.target {
                TripleTarget::Subject => triple.triple.s().kind(),
                TripleTarget::Predicate => triple.triple.p().kind(),
                TripleTarget::Object => triple.triple.o().kind(),
                TripleTarget::Graph => triple
                    .triple
                    .g()
                    .map(|x| x.kind())
                    .unwrap_or(sophia_api::term::TermKind::Triple),
            };

            tracing::info!("Found {} with kind {:?}", token.text, target);

            if target == TermKind::Iri {
                // This is a named node, we should look project wide
                for (tokens, rope, label) in &project {
                    req.0.extend(
                        tokens
                            .iter()
                            .filter(|x| x.value() == token.token.value())
                            .flat_map(|t| token_to_location(t, label, &rope)),
                    );
                }
            } else if target == TermKind::BlankNode {
                // Blank node is constrained to current
                // document
                req.0.extend(
                    tokens
                        .iter()
                        .filter(|x| x.value() == token.token.value())
                        .flat_map(|t| token_to_location(t, label, &rope)),
                );
            }
        }
    }
}
