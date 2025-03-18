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
    use references::ReferencesRequest;
    use sophia_api::term::TermKind;

    use crate::{prelude::*, util::token_to_location};

    pub fn add_references(
        mut query: Query<(
            &TokenComponent,
            &TripleComponent,
            &Tokens,
            &Label,
            &RopeC,
            &mut ReferencesRequest,
        )>,
        project: Query<(&Tokens, &RopeC, &Label)>,
    ) {
        for (token, triple, tokens, label, rope, mut req) in &mut query {
            let target = triple.kind();
            tracing::info!("Found {} with kind {:?}", token.text, target);
            if target == TermKind::Iri {
                // This is a named node, we should look project wide
                // TODO: This should not match on token, but on expanded token
                for (tokens, rope, label) in &project {
                    req.0.extend(
                        tokens
                            .iter()
                            .filter(|x| x.value() == token.token.value())
                            .flat_map(|t| token_to_location(t.span(), label, &rope)),
                    );
                }
            } else if target == TermKind::BlankNode {
                // Blank node is constrained to current
                // document
                req.0.extend(
                    tokens
                        .iter()
                        .filter(|x| x.value() == token.token.value())
                        .flat_map(|t| token_to_location(t.span(), label, &rope)),
                );
            }
        }
    }
}
