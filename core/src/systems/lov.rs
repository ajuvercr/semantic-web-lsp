use std::{collections::HashMap, str::FromStr as _};

use bevy_ecs::{prelude::*, world::CommandQueue};
use hashbrown::HashSet;
use lsp_types::TextDocumentItem;
use serde::Deserialize;
use tracing::info;

use crate::{client::Client, components::*, prelude::ParseLabel, systems::spawn_or_insert};

#[derive(Deserialize, Debug)]
struct Version {
    #[serde(rename = "fileURL")]
    file_url: Option<String>,
    issued: chrono::DateTime<chrono::Utc>,
}

#[derive(Deserialize, Debug)]
struct Vocab {
    versions: Vec<Version>,
}

// Do we check whether or not the namespace url and the prefix url are the same?
async fn extract_file_url(prefix: &str, client: &impl Client) -> Option<String> {
    let url = format!(
        "https://lov.linkeddata.es/dataset/lov/api/v2/vocabulary/info?vocab={}",
        prefix
    );
    match client.fetch(&url, &std::collections::HashMap::new()).await {
        Ok(resp) if resp.status == 200 => match serde_json::from_str::<Vocab>(&resp.body) {
            Ok(x) => {
                let versions: Vec<_> = x.versions.iter().flat_map(|x| &x.file_url).collect();
                info!(
                    "Found lov response ({} versions) {:?}",
                    x.versions.len(),
                    versions
                );
                x.versions
                    .into_iter()
                    .flat_map(|x| x.file_url.map(|url| (url, x.issued)))
                    .max_by_key(|x| x.1)
                    .map(|x| x.0)
            }
            Err(e) => {
                info!("Deserialize failed ({}) {:?}", url, e);
                None
            }
        },
        Ok(resp) => {
            info!("Fetch ({}) failed status {}", url, resp.status);
            None
        }
        Err(e) => {
            info!("Fetch ({}) failed {:?}", url, e);
            None
        }
    }
}

/// First of al, fetch the lov dataset information at url https://lov.linkeddata.es/dataset/lov/api/v2/vocabulary/info?vocab=${prefix}
/// Next, extract that json object into an object and find the latest dataset

pub fn fetch_lov_properties<C: Client + Resource>(
    sender: Res<CommandSender>,
    query: Query<
        &Prefixes,
        (
            Or<((Changed<Prefixes>, With<Open>), Changed<Open>)>,
            // Without<Dirty>,
        ),
    >,
    mut prefixes: Local<HashSet<String>>,
    client: Res<C>,
) {
    println!("fetch lov properties");
    for prefs in &query {
        println!("Found some turtle!");
        for prefix in prefs.0.iter() {
            if !prefixes.contains(prefix.url.as_str()) {
                prefixes.insert(prefix.url.to_string());

                // let prefix = prefix.prefix.0.clone();
                if let Some(local) = lov::LOCAL_PREFIXES
                    .iter()
                    .find(|x| x.location == prefix.url.as_str())
                {
                    info!("Using local {}", local.name);
                    println!("Using local {}", local.name);
                    let mut command_queue = CommandQueue::default();

                    let url = lsp_types::Url::from_str(local.location).unwrap();
                    let item = TextDocumentItem {
                        version: 1,
                        uri: url.clone(),
                        language_id: String::from("turtle"),
                        text: String::new(),
                    };
                    let spawn = spawn_or_insert(
                        url.clone(),
                        (
                            Source(local.content.to_string()),
                            RopeC(ropey::Rope::from_str(local.content)),
                            Label(url), // this might crash
                            Wrapped(item),
                            Types(HashMap::new()),
                        ),
                        Some("turtle".into()),
                        (),
                    );

                    command_queue.push(move |world: &mut World| {
                        spawn(world);
                        world.run_schedule(ParseLabel);
                    });

                    let _ = sender.0.clone().start_send(command_queue);
                    continue;
                }

                let mut sender = sender.0.clone();
                let c = client.as_ref().clone();

                let prefix: Prefix = prefix.clone();
                let fut = async move {
                    let mut command_queue = CommandQueue::default();

                    if let Some(url) = extract_file_url(&prefix.prefix, &c).await {
                        match c.fetch(&url, &std::collections::HashMap::new()).await {
                            Ok(resp) if resp.status == 200 => {
                                let rope = ropey::Rope::from_str(&resp.body);
                                let item = TextDocumentItem {
                                    version: 1,
                                    uri: prefix.url.clone(),
                                    language_id: String::from("turtle"),
                                    text: String::new(),
                                };

                                info!(
                                    "Adding new text document for {}, body {}",
                                    url,
                                    resp.body.len()
                                );

                                let url = prefix.url.clone();
                                let spawn = spawn_or_insert(
                                    url.clone(),
                                    (
                                        Source(resp.body),
                                        RopeC(rope),
                                        Label(url), // this might crash
                                        Wrapped(item),
                                    ),
                                    Some("turtle".into()),
                                    (),
                                );

                                command_queue.push(move |world: &mut World| {
                                    spawn(world);
                                    world.run_schedule(ParseLabel);
                                });

                                let _ = sender.start_send(command_queue);
                            }
                            Ok(resp) => {
                                info!("Fetch ({}) failed status {}", url, resp.status);
                            }
                            Err(e) => {
                                info!("Fetch ({}) failed {:?}", url, e);
                            }
                        }
                    }
                };

                client.spawn(fut);
            }
        }
    }
}
