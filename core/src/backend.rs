use crate::components::{
    CommandSender, CompletionRequest, FormatRequest, HighlightRequest, HoverRequest, InlayRequest,
    Label, Open, PositionComponent, RopeC, Source, Types, Wrapped,
};
use crate::systems::spawn_or_insert;
use crate::{Completion, Diagnostics, Format, Hover, Inlay, OnSave, Parse};
use bevy_ecs::bundle::Bundle;
use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use bevy_ecs::schedule::ScheduleLabel;
use bevy_ecs::world::{CommandQueue, World};
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
    #[allow(unused)]
    client: tower_lsp::Client,
    semantic_tokens: Vec<SemanticTokenType>,
}

impl Backend {
    pub fn new(
        sender: CommandSender,
        client: tower_lsp::Client,
        tokens: Vec<SemanticTokenType>,
    ) -> Self {
        Self {
            entities: Default::default(),
            sender,
            client,
            semantic_tokens: tokens,
        }
    }

    #[must_use]
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

    #[must_use]
    async fn run_schedule<T: Component>(
        &self,
        entity: Entity,
        schedule: impl ScheduleLabel,
        param: impl Bundle,
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
                inlay_hint_provider: Some(OneOf::Left(true)),
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                code_action_provider: None,
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: Some(vec![String::from(":")]),
                    work_done_progress_options: Default::default(),
                    all_commit_characters: None,
                    completion_item: None,
                }),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                definition_provider: Some(OneOf::Left(true)),
                document_formatting_provider: Some(OneOf::Left(true)),
                semantic_tokens_provider: Some(
                    SemanticTokensServerCapabilities::SemanticTokensRegistrationOptions(
                        SemanticTokensRegistrationOptions {
                            text_document_registration_options: {
                                TextDocumentRegistrationOptions {
                                    document_selector: Some(vec![
                                        DocumentFilter {
                                            language: Some(String::from("turtle")),
                                            scheme: None,
                                            pattern: Some(String::from("*.ttl")),
                                        },
                                        DocumentFilter {
                                            language: Some(String::from("jsonld")),
                                            scheme: None,
                                            pattern: Some(String::from("*.jsonld")),
                                        },
                                        DocumentFilter {
                                            language: Some(String::from("sparql")),
                                            scheme: None,
                                            pattern: Some(String::from("*.sq")),
                                        },
                                        DocumentFilter {
                                            language: Some(String::from("sparql")),
                                            scheme: None,
                                            pattern: Some(String::from("*.rq")),
                                        },
                                    ]),
                                }
                            },
                            semantic_tokens_options: SemanticTokensOptions {
                                work_done_progress_options: WorkDoneProgressOptions::default(),
                                legend: SemanticTokensLegend {
                                    token_types: self.semantic_tokens.clone(),
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
            let e = {
                let map = self.entities.lock().await;
                if let Some(entity) = map.get(uri) {
                    Some(entity.clone())
                } else {
                    info!("Didn't find entity {} retrying", uri);
                    None
                }
            };

            if let Some(e) = e {
                e
            } else {
                let map = self.entities.lock().await;
                if let Some(entity) = map.get(uri) {
                    entity.clone()
                } else {
                    info!("Didn't find entty {} stopping", uri);
                    return Ok(None);
                }
            }
        };

        if let Some(res) = self
            .run_schedule::<HighlightRequest>(
                entity,
                crate::systems::SemanticTokensSchedule,
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
        info!("Shutting down!");
        Ok(())
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<lsp_types::Hover>> {
        let request: HoverRequest = HoverRequest::default();

        let entity = {
            let map = self.entities.lock().await;
            if let Some(entity) = map.get(
                params
                    .text_document_position_params
                    .text_document
                    .uri
                    .as_str(),
            ) {
                entity.clone()
            } else {
                return Ok(None);
            }
        };

        let mut pos = params.text_document_position_params.position;
        pos.character = if pos.character > 0 {
            pos.character - 1
        } else {
            pos.character
        };

        if let Some(hover) = self
            .run_schedule::<HoverRequest>(entity, Hover, (request, PositionComponent(pos)))
            .await
        {
            if hover.0.len() > 0 {
                return Ok(Some(lsp_types::Hover {
                    contents: lsp_types::HoverContents::Array(
                        hover.0.into_iter().map(MarkedString::String).collect(),
                    ),
                    range: hover.1,
                }));
            }
        }

        Ok(None)
    }

    async fn inlay_hint(&self, params: InlayHintParams) -> Result<Option<Vec<InlayHint>>> {
        info!("Inlay hints called");
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

        let request = self
            .run_schedule::<InlayRequest>(entity, Inlay, InlayRequest(None))
            .await;

        Ok(request.and_then(|x| x.0))
    }

    #[tracing::instrument(skip(self))]
    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        let _ = params;
        info!("Goto definition");
        let url = params
            .text_document_position_params
            .text_document
            .uri
            .join("pipeline.ttl")
            .unwrap();
        // let url = Url::parse("untitled://cdn.jsdelivr.net/gh/treecg/specification@master/tree.ttl")
        //     .unwrap();
        // let mut map = HashMap::new();
        // map.insert(
        //     url.clone(),
        //     vec![TextEdit {
        //         range: lsp_types::Range::new(
        //             lsp_types::Position::new(0, 0),
        //             lsp_types::Position::new(0, 0),
        //         ),
        //         new_text: String::from("@prefix foaf: <>."),
        //     }],
        // );
        // let res = self.client.apply_edit(WorkspaceEdit::new(map)).await;
        // info!("result {:?}", res);
        Ok(Some(GotoDefinitionResponse::Array(vec![Location {
            uri: url,
            range: Range::new(Position::new(0, 0), Position::new(0, 5)),
            // target_selection_range: Range::new(Position::new(0, 0), Position::new(0, 5)),
            // origin_selection_range: None,
        }])))
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

        let request = self
            .run_schedule::<FormatRequest>(entity, Format, FormatRequest(None))
            .await;
        Ok(request.and_then(|x| x.0))
    }

    #[tracing::instrument(skip(self, params), fields(uri = %params.text_document.uri.as_str()))]
    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let item = params.text_document;
        let url = item.uri.as_str().to_string();

        let lang_id = Some(item.language_id.clone());
        let spawn = spawn_or_insert(
            item.uri.clone(),
            (
                Source(item.text.clone()),
                Label(item.uri.clone()),
                RopeC(Rope::from_str(&item.text)),
                Wrapped(item),
                Open,
                Types(HashMap::new()),
            ),
            lang_id,
            (),
        );

        let entity = self
            .run(|world| {
                let id = spawn(world);
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

    #[tracing::instrument(skip(self, params), fields(uri = %params.text_document.uri.as_str()))]
    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        info!("Did save");
        self.run(move |world| {
            world.run_schedule(OnSave);

            info!("Ran OnSave Schedule");
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

        // Problem: whne the cursor is at the end of en ident, that ident is not in range of the
        // cursor
        let mut pos = params.text_document_position.position;
        pos.character = if pos.character > 0 {
            pos.character - 1
        } else {
            pos.character
        };
        let completions: Option<Vec<lsp_types::CompletionItem>> = self
            .run_schedule::<CompletionRequest>(
                entity,
                Completion,
                (CompletionRequest(vec![]), PositionComponent(pos)),
            )
            .await
            .map(|x| x.0.into_iter().map(|x| x.into()).collect());

        Ok(completions.map(|c| CompletionResponse::Array(c)))
    }
}
