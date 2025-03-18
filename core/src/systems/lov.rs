use std::{collections::HashMap, str::FromStr as _};

use bevy_ecs::{prelude::*, world::CommandQueue};
use hashbrown::HashSet;
use lsp_types::{TextDocumentItem, Url};
use serde::{Deserialize, Serialize};
use sophia_api::{
    prelude::{Any, Dataset},
    quad::Quad,
    term::matcher::TermMatcher,
};
use tracing::{debug, error, info, instrument};

use crate::{prelude::*, util::ns::rdfs};

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
                debug!(
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
                error!("Deserialize failed ({}) {:?}", url, e);
                None
            }
        },
        Ok(resp) => {
            error!("Fetch ({}) failed status {}", url, resp.status);
            None
        }
        Err(e) => {
            error!("Fetch ({}) failed {:?}", url, e);
            None
        }
    }
}

pub fn finish_prefix_import(
    query: Query<(&FromPrefix, &RopeC), Added<FromPrefix>>,
    mut helper: ResMut<LovHelper>,
    cache: Res<Cache>,
) {
    for (p, rope) in &query {
        tracing::debug!("Finishing import for {:?}", p.0);
        if let Some(e) = helper.has_entry_mut(&p.0) {
            let _ = e.save(&cache, &rope.to_string());
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
    mut helper: ResMut<LovHelper>,
    cache: Res<Cache>,
) {
    println!("fetch lov properties");
    for prefs in &query {
        println!("Found some turtle!");
        for prefix in prefs.0.iter() {
            if let Some(e) = helper.has_entry_mut(&prefix) {
                let name = e.name();
                if !prefixes.contains(&name) {
                    prefixes.insert(name);
                    from_cache(&e, &cache, &sender);
                }
            } else {
                let e = helper.create_entry(&prefix);
                let name = e.name();
                if !prefixes.contains(&name) {
                    prefixes.insert(name);
                    if let Some(url) = e.url(&cache) {
                        // let prefix = prefix.prefix.0.clone();
                        if let Some(local) = lov::LOCAL_PREFIXES
                            .iter()
                            .find(|x| x.location == prefix.url.as_str())
                        {
                            local_lov(local, url, &sender);
                        } else {
                            let sender = sender.0.clone();
                            let c = client.as_ref().clone();
                            client.spawn(fetch_lov(prefix.clone(), url, c, sender));
                        }
                    }
                }
            }
        }
    }
}

fn from_cache(e: &LovEntry, cache: &Cache, sender: &CommandSender) -> Option<()> {
    debug!("Using cached {:?}", e);
    let mut command_queue = CommandQueue::default();
    let url = e.url(cache)?;
    let content = cache.get_file(&e.name())?;

    let item = TextDocumentItem {
        version: 1,
        uri: url.clone(),
        language_id: String::from("turtle"),
        text: String::new(),
    };

    let spawn = spawn_or_insert(
        url.clone(),
        (
            RopeC(ropey::Rope::from_str(&content)),
            Source(content),
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

    Some(())
}

async fn fetch_lov<C: Client + Resource>(
    prefix: Prefix,
    label: Url,
    c: C,
    mut sender: futures::channel::mpsc::UnboundedSender<CommandQueue>,
) {
    let mut command_queue = CommandQueue::default();

    if let Some(url) = extract_file_url(&prefix.prefix, &c).await {
        match c.fetch(&url, &std::collections::HashMap::new()).await {
            Ok(resp) if resp.status == 200 => {
                let rope = ropey::Rope::from_str(&resp.body);
                let item = TextDocumentItem {
                    version: 1,
                    uri: label.clone(),
                    language_id: String::from("turtle"),
                    text: String::new(),
                };

                let spawn = spawn_or_insert(
                    label.clone(),
                    (
                        Source(resp.body),
                        RopeC(rope),
                        Label(label), // this might crash
                        Wrapped(item),
                        FromPrefix(prefix),
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
                error!("Fetch ({}) failed status {}", url, resp.status);
            }
            Err(e) => {
                error!("Fetch ({}) failed {:?}", url, e);
            }
        }
    }
}

fn local_lov(local: &lov::LocalPrefix, label: Url, sender: &Res<CommandSender>) {
    info!("Using local {}", local.name);
    let mut command_queue = CommandQueue::default();

    let url = lsp_types::Url::from_str(local.location).unwrap();
    let item = TextDocumentItem {
        version: 1,
        uri: label.clone(),
        language_id: String::from("turtle"),
        text: String::new(),
    };
    let spawn = spawn_or_insert(
        url.clone(),
        (
            Source(local.content.to_string()),
            RopeC(ropey::Rope::from_str(local.content)),
            Label(label),
            Wrapped(item),
            Types(HashMap::new()),
            FromPrefix(Prefix {
                prefix: local.name.to_string(),
                url: Url::parse(&local.location).unwrap(),
            }),
        ),
        Some("turtle".into()),
        (),
    );

    command_queue.push(move |world: &mut World| {
        spawn(world);
        world.run_schedule(ParseLabel);
    });

    let _ = sender.0.clone().start_send(command_queue);
}

#[derive(Component)]
pub struct OntologyExtract;

#[instrument(skip(commands))]
pub fn init_onology_extractor(
    mut commands: Commands,
    mut helper: ResMut<LovHelper>,
    cache: Res<Cache>,
) {
    for local in lov::LOCAL_PREFIXES
        .iter()
        .filter(|x| ["rdf", "rdfs", "owl"].iter().any(|y| *y == x.name))
    {
        // HERE
        let entry = helper.create_entry(&Prefix {
            prefix: local.name.to_string(),
            url: lsp_types::Url::from_str(local.location).unwrap(),
        });
        let url = entry.url(&cache).unwrap();

        // let url = lsp_types::Url::from_str(local.location).unwrap();
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
            OntologyExtract,
        );

        info!("Init onology {}", local.name);
        commands.push(move |world: &mut World| {
            info!("Spawned");
            spawn(world);
        });
    }
}

#[instrument(skip(query, extractor))]
pub fn check_added_ontology_extract(
    query: Query<(&Triples, &Label), (Added<Triples>, With<OntologyExtract>)>,
    mut extractor: ResMut<OntologyExtractor>,
) {
    let mut changed = false;
    for (triples, label) in &query {
        info!("Added triples from {}", label.as_str());
        extractor.quads.extend(triples.0.iter().cloned());
        changed = true;
    }
    if changed {
        extractor.extract();
    }
}

#[derive(Debug, Resource)]
pub struct OntologyExtractor {
    quads: Vec<MyQuad<'static>>,
    properties: Vec<MyTerm<'static>>,
    classes: Vec<MyTerm<'static>>,
}

struct LocalMatcher<'a> {
    properties: &'a [MyTerm<'static>],
}

impl TermMatcher for LocalMatcher<'_> {
    type Term = MyTerm<'static>;

    fn matches<T2: sophia_api::prelude::Term + ?Sized>(&self, term: &T2) -> bool {
        for p in self.properties {
            if term.eq(p) {
                return false;
            }
        }

        true
    }
}

impl OntologyExtractor {
    pub fn new() -> Self {
        Self {
            quads: vec![],
            classes: vec![MyTerm::<'static>::named_node(
                "http://www.w3.org/2000/01/rdf-schema#Class",
                0..1,
            )],
            properties: vec![MyTerm::<'static>::named_node(
                "http://www.w3.org/1999/02/22-rdf-syntax-ns#Property",
                0..1,
            )],
        }
    }

    pub fn properties<'a>(&'a self) -> &'a [MyTerm<'static>] {
        &self.properties[..]
    }

    pub fn classes<'a>(&'a self) -> &'a [MyTerm<'static>] {
        &self.classes[..]
    }

    fn extract_step(quads: &Vec<MyQuad<'static>>, items: &mut Vec<MyTerm<'static>>) -> bool {
        let new_items: Vec<_> = quads
            .quads_matching(
                LocalMatcher { properties: &items },
                [rdfs::subClassOf],
                &items[..],
                Any,
            )
            .flatten()
            .map(|x| x.to_s().to_owned())
            .collect();

        let added = !new_items.is_empty();
        items.extend(new_items);
        added
    }

    fn extract(&mut self) {
        loop {
            if !OntologyExtractor::extract_step(&self.quads, &mut self.properties) {
                break;
            }
        }

        loop {
            if !OntologyExtractor::extract_step(&self.quads, &mut self.classes) {
                break;
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EntryState {
    Ready,
    Transit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LovEntry {
    prefix: String,
    url: String,
    num: usize,
    state: EntryState,
}

impl LovEntry {
    fn name(&self) -> String {
        format!("{}-{}.ttl", self.num, self.prefix)
    }
    pub fn url(&self, cache: &Cache) -> Option<Url> {
        let p = cache.path()?;
        let url = p.join(self.name());
        Url::from_file_path(url).ok()
    }

    pub fn save(&mut self, cache: &Cache, content: &str) -> Option<()> {
        self.state = EntryState::Ready;
        cache.write_file(&self.name(), content)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Resource)]
pub struct LovHelper {
    entries: Vec<LovEntry>,
}

impl LovHelper {
    fn try_from_cache(cache: &Cache) -> Option<Self> {
        let c = cache.get_file("index.json")?;
        info!("Found index file! {}", c);
        serde_json::from_str(&c).ok()
    }

    pub fn from_cache(cache: &Cache) -> Self {
        Self::try_from_cache(cache).unwrap_or_else(|| Self {
            entries: Vec::new(),
        })
    }

    pub fn save(mut self, cache: &Cache) -> Option<()> {
        self.entries = self
            .entries
            .into_iter()
            .filter(|x| x.state == EntryState::Ready)
            .collect();
        let st = serde_json::to_string(&self).ok()?;
        info!("Save index file! {}", st);
        cache.write_file("index.json", &st)
    }

    pub fn has_entry_mut(&mut self, prefix: &Prefix) -> Option<&mut LovEntry> {
        self.entries
            .iter_mut()
            .find(|e| e.prefix == prefix.prefix && e.url == prefix.url.as_str())
    }

    pub fn has_entry(&self, prefix: &Prefix) -> Option<&LovEntry> {
        self.entries
            .iter()
            .find(|e| e.prefix == prefix.prefix && e.url == prefix.url.as_str())
    }

    pub fn create_entry(&mut self, prefix: &Prefix) -> &LovEntry {
        if let Some(e) = self.entries.iter().enumerate().find_map(|(i, e)| {
            (e.prefix == prefix.prefix && e.url == prefix.url.as_str()).then_some(i)
        }) {
            return &self.entries[e];
        }
        let c = self
            .entries
            .iter()
            .filter(|x| x.prefix == prefix.prefix)
            .count();
        let entry = LovEntry {
            prefix: prefix.prefix.to_string(),
            url: prefix.url.to_string(),
            num: c,
            state: EntryState::Transit,
        };
        self.entries.push(entry);
        self.entries.last().unwrap()
    }

    pub fn save_prefix(&mut self, cache: &Cache, prefix: &Prefix, content: &str) -> Option<()> {
        let e = self
            .entries
            .iter_mut()
            .find(|e| (e.prefix == prefix.prefix && e.url == prefix.url.as_str()))?;
        e.save(cache, content)
    }
}

#[derive(Debug, Clone, Component)]
pub struct FromPrefix(pub Prefix);
