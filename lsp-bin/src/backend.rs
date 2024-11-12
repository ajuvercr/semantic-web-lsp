use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use bevy_ecs::schedule::ScheduleLabel;
use bevy_ecs::world::{CommandQueue, World};
use lang_turtle::testing::TurtleComponent;
use lang_turtle::TurtleLang;
use lsp_core::components::{
    CommandSender, CompletionRequest, FormatRequest, HighlightRequest, Label, RopeC, Source,
    Wrapped,
};
use lsp_core::lang::Lang;
use lsp_core::{Completion, Diagnostics, Format, Parse};
use lsp_types::*;

use futures::lock::Mutex;
use ropey::Rope;
use tracing::info;

use std::collections::HashMap;
use std::sync::Arc;
use tower_lsp::jsonrpc::Result;
use tower_lsp::LanguageServer;

#[derive(Debug)]
pub struct Backend {
    entities: Arc<Mutex<HashMap<String, Entity>>>,
    sender: CommandSender,
}

impl Backend {
    pub fn new(sender: CommandSender) -> Self {
        Self {
            entities: Default::default(),
            sender,
        }
    }

    async fn run<T: Send + Sync + 'static>(
        &self,
        f: impl FnOnce(&mut World) -> T + Send + Sync + 'static,
    ) -> Option<T> {
        let (tx, rx) = futures::channel::oneshot::channel();
        let mut commands = CommandQueue::default();
        commands.push(move |world: &mut World| {
            let o = f(world);
            if let Err(_) = tx.send(o) {
                tracing::error!("Failed to run schedule");
            };
        });

        if let Err(e) = self.sender.0.unbounded_send(commands) {
            tracing::error!("Failed to send commands {}", e);
            return None;
        }

        rx.await.ok()
    }

    async fn run_schedule<S: ScheduleLabel, T: Component>(
        &self,
        entity: Entity,
        schedule: S,
        param: T,
    ) -> Option<T> {
        let (tx, rx) = futures::channel::oneshot::channel();

        let mut commands = CommandQueue::default();
        commands.push(move |world: &mut World| {
            world.entity_mut(entity).insert(param);
            world.run_schedule(schedule);
            if let Err(_) = tx.send(world.entity_mut(entity).take::<T>()) {
                tracing::error!("Failed to run schedule");
            };
        });

        if let Err(e) = self.sender.0.unbounded_send(commands) {
            tracing::error!("Failed to send commands {}", e);
            return None;
        }

        rx.await.unwrap_or_default()
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    #[tracing::instrument(skip(self, _init))]
    async fn initialize(&self, _init: InitializeParams) -> Result<InitializeResult> {
        info!("Initialize");
        // let triggers = L::TRIGGERS.iter().copied().map(String::from).collect();
        Ok(InitializeResult {
            server_info: None,
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                code_action_provider: None,
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: None,
                    work_done_progress_options: Default::default(),
                    all_commit_characters: None,
                    completion_item: None,
                }),
                document_formatting_provider: Some(OneOf::Left(true)),
                semantic_tokens_provider: Some(
                    SemanticTokensServerCapabilities::SemanticTokensRegistrationOptions(
                        SemanticTokensRegistrationOptions {
                            text_document_registration_options: {
                                TextDocumentRegistrationOptions {
                                    document_selector: Some(vec![DocumentFilter {
                                        language: Some(String::from("turtle")),
                                        scheme: Some("file".to_string()),
                                        pattern: Some(String::from("*.ttl")),
                                    }]),
                                }
                            },
                            semantic_tokens_options: SemanticTokensOptions {
                                work_done_progress_options: WorkDoneProgressOptions::default(),
                                legend: SemanticTokensLegend {
                                    token_types: TurtleLang::LEGEND_TYPES.into(),
                                    token_modifiers: vec![],
                                },
                                range: Some(false),
                                full: Some(SemanticTokensFullOptions::Bool(true)),
                            },
                            static_registration_options: StaticRegistrationOptions::default(),
                        },
                    ),
                ),
                rename_provider: Some(OneOf::Right(RenameOptions {
                    prepare_provider: Some(true),
                    work_done_progress_options: Default::default(),
                })),
                ..ServerCapabilities::default()
            },
        })
    }

    #[tracing::instrument(skip(self, params), fields(uri = %params.text_document.uri.as_str()))]
    async fn semantic_tokens_full(
        &self,
        params: SemanticTokensParams,
    ) -> Result<Option<SemanticTokensResult>> {
        info!("semantic tokens full");
        let uri = params.text_document.uri.as_str();
        let entity = {
            let map = self.entities.lock().await;
            if let Some(entity) = map.get(uri) {
                entity.clone()
            } else {
                info!("Didn't find entity {}", uri);
                return Ok(None);
            }
        };

        if let Some(res) = self
            .run_schedule(
                entity,
                lsp_core::systems::semantic_tokens::Label,
                HighlightRequest(vec![]),
            )
            .await
        {
            info!("resulitng in {} tokens", res.0.len());
            Ok(Some(SemanticTokensResult::Tokens(
                lsp_types::SemanticTokens {
                    result_id: None,
                    data: res.0,
                },
            )))
        } else {
            info!("resulitng in no tokens");
            Ok(None)
        }
    }

    #[tracing::instrument(skip(self))]
    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    #[tracing::instrument(skip(self))]
    async fn formatting(&self, params: DocumentFormattingParams) -> Result<Option<Vec<TextEdit>>> {
        let uri = params.text_document.uri.as_str();
        let entity = {
            let map = self.entities.lock().await;
            if let Some(entity) = map.get(uri) {
                entity.clone()
            } else {
                info!("Didn't find entity {}", uri);
                return Ok(None);
            }
        };

        let request = self.run_schedule(entity, Format, FormatRequest(None)).await;
        Ok(request.and_then(|x| x.0))
    }

    #[tracing::instrument(skip(self, params), fields(uri = %params.text_document.uri.as_str()))]
    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let item = params.text_document;
        let url = item.uri.as_str().to_string();

        let entity = self
            .run(|world| {
                let id = world
                    .spawn((
                        TurtleComponent,
                        Source(item.text.clone()),
                        Label(item.uri.clone()),
                        RopeC(Rope::from_str(&item.text)),
                        Wrapped(item),
                    ))
                    .id();

                world.run_schedule(Parse);
                world.flush();
                info!("Running diagnostics");
                world.run_schedule(Diagnostics);
                id
            })
            .await;

        if let Some(entity) = entity {
            self.entities.lock().await.insert(url, entity);
        }
    }

    #[tracing::instrument(skip(self, params), fields(uri = %params.text_document.uri.as_str()))]
    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let entity = {
            let map = self.entities.lock().await;
            if let Some(entity) = map.get(params.text_document.uri.as_str()) {
                entity.clone()
            } else {
                info!("Didn't find entity {}", params.text_document.uri.as_str());
                return;
            }
        };

        let change = {
            if let Some(c) = params.content_changes.into_iter().next() {
                c
            } else {
                return;
            }
        };

        self.run(move |world| {
            let rope_c = RopeC(Rope::from_str(&change.text));
            world
                .entity_mut(entity)
                .insert((Source(change.text), rope_c));
            world.run_schedule(Parse);
            world.flush();
            info!("Running diagnostics");
            world.run_schedule(Diagnostics);
        })
        .await;
    }

    #[tracing::instrument(skip(self, params), fields(uri = %params.text_document_position.text_document.uri.as_str()))]
    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let entity = {
            let map = self.entities.lock().await;
            if let Some(entity) = map.get(params.text_document_position.text_document.uri.as_str())
            {
                entity.clone()
            } else {
                return Ok(None);
            }
        };

        let completions: Option<Vec<lsp_types::CompletionItem>> = self
            .run_schedule(entity, Completion, CompletionRequest(vec![]))
            .await
            .map(|x| x.0.into_iter().map(|x| x.into()).collect());

        Ok(completions.map(|c| CompletionResponse::Array(c)))
    }
}
