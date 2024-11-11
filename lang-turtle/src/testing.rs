use std::str::FromStr as _;

use bevy_ecs::{prelude::*, world::CommandQueue};
use chumsky::Parser;
use futures::channel::mpsc::{UnboundedReceiver, UnboundedSender};
use hashbrown::HashSet;
use lsp_core::{
    client::Client,
    components::*,
    lang::{Lang, OtherPublisher, SimpleCompletion, SimpleDiagnostic},
    model::Spanned,
    utils::{lsp_range_to_range, offset_to_position},
    Parse,
};
use lsp_types::{CompletionItemKind, Diagnostic, TextDocumentItem, Url};
use tracing::info;

use crate::TurtleLang;
use crate::{formatter::format_turtle, parse_turtle, shacl::MyQuad, tokenizer::parse_tokens};

#[derive(Component)]
pub struct TurtleComponent;

pub fn parse_source(
    query: Query<(Entity, &Source), (Changed<Source>, With<TurtleComponent>)>,
    mut commands: Commands,
) {
    for (entity, source) in &query {
        let (tok, es) = parse_tokens().parse_recovery(source.0.as_str());
        if let Some(tokens) = tok {
            let t = Tokens::<TurtleLang>(tokens);
            commands.entity(entity).insert(t);
        }
        commands.entity(entity).insert(Errors(es));
    }
}

pub fn parse_turtle_system(
    query: Query<
        (Entity, &Source, &Tokens<TurtleLang>, &Label),
        (Changed<Tokens<TurtleLang>>, With<TurtleComponent>),
    >,
    mut commands: Commands,
) {
    for (entity, source, tokens, label) in &query {
        let (turtle, es) = parse_turtle(
            &Url::parse(&label.0).unwrap(),
            tokens.0.clone(),
            source.0.len(),
        );
        if es.is_empty() {
            let element = Element::<TurtleLang>(turtle);
            info!("Setting specific errors {}", es.len());
            commands.entity(entity).insert((element, Errors(es)));
        } else {
            info!("Removing errors {}", es.len());
            commands.entity(entity).insert(Errors(es));
        }
    }
}

#[derive(Component)]
pub struct Triples(pub Vec<MyQuad<'static>>);

pub fn derive_triples(
    query: Query<
        (Entity, &Element<TurtleLang>),
        (Changed<Element<TurtleLang>>, With<TurtleComponent>),
    >,
    mut commands: Commands,
) {
    for (entity, turtle) in &query {
        if let Ok(tripl) = turtle.0.get_simple_triples() {
            let triples: Vec<_> = tripl.iter().map(|x| x.to_owned()).collect();
            commands.entity(entity).insert(Triples(triples));
        }
    }
}

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
    for (token, turtle, params, rope) in &query {
        use std::iter::Iterator as _;
        let token_iter = token
            .0
            .iter()
            .cloned()
            .map(|x| Into::<SimpleDiagnostic>::into(x));
        let turtle_iter = turtle
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

        let _ = client.publish(&params.0, diagnostics);
    }
}

pub fn notify_parsed(
    In(label): In<String>,
    query: Query<
        (Entity, &RopeC, &Element<TurtleLang>, &Label),
        (Changed<Element<TurtleLang>>, With<TurtleComponent>),
    >,
) -> Option<String> {
    for (_entity, source, turtle, e_label) in &query {
        if label.as_str() == e_label.0.as_str() {
            let formatted = format_turtle(
                &turtle.0,
                lsp_types::FormattingOptions {
                    tab_size: 2,
                    ..Default::default()
                },
                &vec![],
                &source.0,
            )
            .expect("formatting");
            return Some(formatted);
        }
    }

    None
}

pub fn turtle_prefix_completion(
    mut query: Query<(
        &CurrentWord,
        &Element<TurtleLang>,
        &RopeC,
        &mut CompletionRequest,
    )>,
) {
    for (word, turtle, rope, mut req) in &mut query {
        if let Some(r) = lsp_range_to_range(&word.0, &rope.0) {
            let st = rope.0.slice(r).to_string();
            let pref = if let Some(idx) = st.find(':') {
                &st[..idx]
            } else {
                &st
            };

            let completions = turtle
                .0
                .prefixes
                .iter()
                .filter(|p| p.prefix.as_str().starts_with(pref))
                .map(|x| {
                    let url = x.value.expand(&turtle.0);

                    let edits = vec![lsp_types::TextEdit {
                        new_text: format!("{}:", x.prefix.as_str()),
                        range: word.0,
                    }];
                    SimpleCompletion {
                        kind: CompletionItemKind::MODULE,
                        label: format!("{}", x.prefix.as_str()),
                        documentation: url,
                        sort_text: None,
                        filter_text: None,
                        edits,
                    }
                });

            req.0.extend(completions);
        }
    }
}

pub fn subject_completion(
    mut query: Query<(
        &CurrentWord,
        &Element<TurtleLang>,
        &RopeC,
        &mut CompletionRequest,
    )>,
    triples: Query<(&Triples, &Label)>,
) {
    for (word, turtle, rope, mut req) in &mut query {
        if let Some(r) = lsp_range_to_range(&word.0, &rope.0) {
            let st = rope.0.slice(r).to_string();

            for (triples, label) in &triples {
                for triple in &triples.0 {
                    let subj = triple.subject.as_str();
                    if subj.starts_with(&st) {
                        let new_text = turtle.0.shorten(subj).unwrap_or_else(|| String::from(subj));
                        let edits = vec![lsp_types::TextEdit {
                            new_text,
                            range: word.0,
                        }];
                        req.0.push(SimpleCompletion {
                            kind: CompletionItemKind::MODULE,
                            label: format!("{}", subj),
                            documentation: format!("Subject from {}", label.0).into(),
                            sort_text: None,
                            filter_text: None,
                            edits,
                        });
                    }
                }
            }
        }
    }
}

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

                            command_queue.push(move |world: &mut World| {
                                world.spawn((
                                    TurtleComponent,
                                    Source(content.to_string()),
                                    RopeC(rope),
                                    Label(label),
                                    Wrapped(item),
                                ));
                                world.run_schedule(Parse);
                            });
                        }

                        let _ = sender.start_send(command_queue);
                    };

                    client.spawn(fut);
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::{
        collections::HashMap,
        fmt::Display,
        str::FromStr,
        sync::{
            atomic::{AtomicU32, Ordering},
            Arc,
        },
        time::Duration,
    };

    use super::*;
    use bevy_ecs::system::RunSystemOnce as _;
    use bevy_tasks::futures_lite::FutureExt as _;
    use futures::{
        channel::mpsc::{unbounded, UnboundedReceiver},
        executor::block_on,
        lock::Mutex,
    };
    use lsp_core::{
        client::{ClientSync, Resp},
        lang::DiagnosticItem,
        systems::handle_tasks,
        Completion, Diagnostics, Parse,
    };
    use lsp_types::MessageType;
    use ropey::Rope;

    #[derive(Resource, Debug, Clone)]
    struct TestClient {
        logs: Arc<Mutex<Vec<(MessageType, String)>>>,
        diagnostics: Arc<Mutex<Vec<(Url, Vec<lsp_types::Diagnostic>)>>>,
        locations: HashMap<String, String>,
        tasks_running: Arc<std::sync::atomic::AtomicU32>,
        executor: Arc<async_executor::Executor<'static>>,
        // pool: Arc<LocalPool>,
    }

    impl TestClient {
        fn new() -> Self {
            // let pool = LocalPool::new();
            Self {
                logs: Default::default(),
                diagnostics: Default::default(),
                locations: Default::default(),
                tasks_running: Arc::new(AtomicU32::new(0)),
                executor: Arc::new(async_executor::Executor::new()),
            }
        }
    }
    impl Default for TestClient {
        fn default() -> Self {
            Self::new()
        }
    }

    impl TestClient {
        fn add_res(&mut self, loc: &str, cont: &str) {
            self.locations.insert(loc.to_string(), cont.to_string());
        }
    }

    impl TestClient {
        async fn await_futures<F: FnMut()>(&self, mut tick: F) {
            tick();
            while self.tasks_running.load(Ordering::Relaxed) != 0 {
                self.executor.tick().await;
                tick();
            }
        }
    }

    #[tower_lsp::async_trait]
    impl Client for TestClient {
        async fn log_message<M: Display + Sync + Send + 'static>(
            &self,
            ty: MessageType,
            msg: M,
        ) -> () {
            let mut lock = self.logs.lock().await;
            lock.push((ty, msg.to_string()));
        }

        async fn publish_diagnostics(
            &self,
            uri: Url,
            diags: Vec<Diagnostic>,
            _version: Option<i32>,
        ) -> () {
            let mut lock = self.diagnostics.lock().await;
            lock.push((uri, diags));
        }
    }

    impl ClientSync for TestClient {
        fn spawn<F: std::future::Future<Output = ()> + Send + 'static>(&self, fut: F) {
            self.tasks_running.fetch_add(1, Ordering::AcqRel);
            let tr = self.tasks_running.clone();
            self.executor
                .spawn(async move {
                    fut.await;
                    tr.fetch_sub(1, Ordering::AcqRel);
                })
                .detach();
        }

        fn fetch(
            &self,
            url: &str,
            _headers: &std::collections::HashMap<String, String>,
        ) -> std::pin::Pin<
            Box<dyn Send + std::future::Future<Output = Result<lsp_core::client::Resp, String>>>,
        > {
            let body = self.locations.get(url).cloned();
            async move {
                let mut headers = Vec::new();
                async_std::task::sleep(Duration::from_millis(200)).await;
                headers.push(("Content-Type".to_string(), "text/turtle".to_string()));
                let status = body.is_some().then_some(200).unwrap_or(404);
                Ok(Resp {
                    headers,
                    body: body.unwrap_or_default(),
                    status,
                })
            }
            .boxed()
        }
    }

    fn setup_world(client: TestClient) -> (World, UnboundedReceiver<DiagnosticItem>) {
        let mut world = World::new();

        let (tx, rx) = unbounded();
        world.insert_resource(CommandSender(tx));
        world.insert_resource(CommandReceiver(rx));
        world.insert_resource(client);

        world.add_schedule(Schedule::new(lsp_core::Tasks));
        world.schedule_scope(lsp_core::Tasks, |_, schedule| {
            schedule.add_systems(handle_tasks);
        });

        world.add_schedule(Schedule::new(Parse));
        world.schedule_scope(Parse, |_, schedule| {
            schedule.add_systems((
                parse_source,
                parse_turtle_system.after(parse_source),
                derive_triples.after(parse_turtle_system),
                fetch_lov_properties::<TestClient>.after(parse_turtle_system),
            ));
        });

        world.add_schedule(Schedule::new(lsp_core::Completion));
        world.schedule_scope(lsp_core::Completion, |_, schedule| {
            schedule.add_systems((turtle_prefix_completion, subject_completion));
        });

        world.add_schedule(Schedule::new(lsp_core::Diagnostics));
        world.schedule_scope(lsp_core::Diagnostics, |_, schedule| {
            schedule.add_systems(publish_diagnostics::<crate::TurtleLang>);
        });

        let (publisher, rx) = OtherPublisher::new();
        world.insert_resource(publisher);

        (world, rx)
    }

    fn create_file(world: &mut World, content: &str, url: &str) -> Entity {
        let rope = Rope::from_str(content);
        let item = TextDocumentItem {
            version: 1,
            uri: Url::from_str(url).unwrap(),
            language_id: String::from("turtle"),
            text: String::new(),
        };
        let entity = world
            .spawn((
                TurtleComponent,
                Source(content.to_string()),
                RopeC(rope),
                Label(url.to_string()),
                Wrapped(item),
            ))
            .id();

        world.run_schedule(Parse);

        entity
    }

    #[test]
    fn format_does_it() {
        let (mut world, _) = setup_world(TestClient::new());

        create_file(&mut world, "@prefix foaf: <>.", "http://example.com/ns#");

        let m_formatted =
            world.run_system_once_with("http://example.com/ns#".to_string(), notify_parsed);

        assert!(m_formatted.is_some());
        let formatted = m_formatted.unwrap();
        assert_eq!(formatted, "@prefix foaf: <>.\n\n");
    }

    // Note:
    // - I had to change how the paring worked and only update the turtle component when it was
    // succesful, this had a similar problem without working with a ecs
    // This should just be fixed in parsing allowing for errors and trying to extract 'most' of the
    // content
    #[test]
    fn completion_event_works() {
        let (mut world, _) = setup_world(TestClient::new());

        let t1 = "
@prefix foaf: <>.
            ";

        let t2 = "
@prefix foaf: <>.
foa
            ";

        let entity = create_file(&mut world, t1, "http://example.com/ns#");

        world
            .entity_mut(entity)
            .insert((Source(t2.to_string()), RopeC(Rope::from_str(t2))));
        world.run_schedule(Parse);

        // start call completion
        world.entity_mut(entity).insert((
            CompletionRequest(vec![]),
            CurrentWord(lsp_types::Range {
                start: lsp_types::Position {
                    line: 2,
                    character: 0,
                },
                end: lsp_types::Position {
                    line: 2,
                    character: 3,
                },
            }),
        ));
        world.run_schedule(Completion);
        let m_completions = world.entity_mut(entity).take::<CompletionRequest>();

        assert!(m_completions.is_some());
        let completions = m_completions.unwrap().0;
        assert_eq!(completions.len(), 1);
    }

    #[test]
    fn completion_event_works_multiple_files() {
        let (mut world, _) = setup_world(TestClient::new());

        let t1 = "
@prefix foaf: <http://xmlns.com/foaf/0.1/>.

            ";

        let t2 = "
@prefix foaf: <http://xmlns.com/foaf/0.1/>.

foaf:me foaf:friend <#me>.
            ";

        let entity = create_file(&mut world, t1, "http://example.com/first_file#");
        create_file(&mut world, t2, "http://example.com/second_file#");

        // start call completion
        world.entity_mut(entity).insert((
            CompletionRequest(vec![]),
            CurrentWord(lsp_types::Range {
                start: lsp_types::Position {
                    line: 3,
                    character: 0,
                },
                end: lsp_types::Position {
                    line: 3,
                    character: 0,
                },
            }),
        ));

        world.run_schedule(Completion);
        let m_completions = world.entity_mut(entity).take::<CompletionRequest>();

        assert!(m_completions.is_some());
        let completions = m_completions.unwrap().0;
        assert_eq!(completions.len(), 2);
    }

    #[test]
    fn diagnostics_work() {
        let (mut world, mut rx) = setup_world(TestClient::new());

        let t1 = "
@prefix foaf: <>.
            ";

        let t2 = "
@prefix foaf: <>.
foa:foaf
            ";

        let t3 = "
@prefix foaf: <>.
foa
            ";

        let entity = create_file(&mut world, t1, "http://example.com/ns#");
        world.run_schedule(Diagnostics);

        let mut get_diagnostics = move || {
            let mut out: Vec<DiagnosticItem> = Vec::new();
            while let Ok(Some(x)) = rx.try_next() {
                out.push(x);
            }
            out
        };
        let items = get_diagnostics();
        assert!(items.is_empty());

        world
            .entity_mut(entity)
            .insert((Source(t2.to_string()), RopeC(Rope::from_str(t2))));
        world.run_schedule(Parse);
        world.run_schedule(Diagnostics);

        let items = get_diagnostics();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].diagnostics.len(), 1);

        world
            .entity_mut(entity)
            .insert((Source(t3.to_string()), RopeC(Rope::from_str(t2))));
        world.run_schedule(Parse);
        world.run_schedule(Diagnostics);

        let items = get_diagnostics();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].diagnostics.len(), 2);
    }

    #[test]
    fn fetch_lov_properties_test() {
        let mut client = TestClient::new();
        client.add_res("http://xmlns.com/foaf/0.1/", " @prefix foaf: <>. ");
        let (mut world, _) = setup_world(client);

        let t1 = " @prefix foaf: <http://xmlns.com/foaf/0.1/>.";
        create_file(&mut world, t1, "http://example.com/ns#");

        assert_eq!(world.entities().len(), 1);
        let c = world.resource::<TestClient>().clone();
        block_on(c.await_futures(|| world.run_schedule(lsp_core::Tasks)));

        assert_eq!(world.entities().len(), 2);
    }
}
