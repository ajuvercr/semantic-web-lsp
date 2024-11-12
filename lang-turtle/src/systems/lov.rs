use std::str::FromStr as _;

use bevy_ecs::{prelude::*, world::CommandQueue};
use hashbrown::HashSet;
use lsp_core::{client::Client, components::*, Parse};
use lsp_types::{TextDocumentItem, Url};
use tracing::info;

use crate::{TurtleComponent, TurtleLang};

pub fn fetch_lov_properties<C: Client + Resource>(
    sender: Res<CommandSender>,
    query: Query<&Element<TurtleLang>, Changed<Element<TurtleLang>>>,
    mut handled: Local<HashSet<String>>,
    client: Res<C>,
) {
    for turtle in &query {
        for prefix in &turtle.0.prefixes {
            if let Some(label) = prefix.value.expand(&turtle.0) {
                if !handled.contains(&label) {
                    handled.insert(label.clone());

                    let mut sender = sender.0.clone();
                    let c = client.as_ref().clone();

                    let fut = async move {
                        let mut command_queue = CommandQueue::default();
                        if let Ok(resp) = c.fetch(&label, &std::collections::HashMap::new()).await {
                            let content = resp.body;
                            let rope = ropey::Rope::from_str(&content);
                            let item = TextDocumentItem {
                                version: 1,
                                uri: Url::from_str(&label).unwrap(),
                                language_id: String::from("turtle"),
                                text: String::new(),
                            };

                            info!(
                                "Adding new text document for {}, body {}",
                                label,
                                content.len()
                            );

                            command_queue.push(move |world: &mut World| {
                                world.spawn((
                                    TurtleComponent,
                                    Source(content.to_string()),
                                    RopeC(rope),
                                    Label(lsp_types::Url::from_str(&label).unwrap()), // this might
                                    // crash
                                    Wrapped(item),
                                ));
                                world.run_schedule(Parse);
                            });
                        }

                        info!("Sending command queue!");
                        let _ = sender.start_send(command_queue);
                    };

                    client.spawn(fut);
                }
            }
        }
    }
}
