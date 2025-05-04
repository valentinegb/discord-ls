mod language;
mod tracing;

use std::{
    str::FromStr,
    time::{SystemTime, UNIX_EPOCH},
};

use ::tracing::{debug, error, info, subscriber};
use discord_rich_presence::{
    DiscordIpc, DiscordIpcClient,
    activity::{Activity, Assets, Timestamps},
};
use language::Language;
use tower_lsp::{
    LanguageServer, LspService, Server, jsonrpc,
    lsp_types::{
        DidChangeTextDocumentParams, InitializeParams, InitializeResult, ServerCapabilities,
        ServerInfo, TextDocumentSyncCapability, TextDocumentSyncKind,
    },
};
use tracing::LspSubscriber;

struct DiscordLanguageServer {
    language_client: tower_lsp::Client,
    discord: tokio::sync::Mutex<DiscordIpcClient>,
    last_edited_file: tokio::sync::Mutex<Option<String>>,
    started_timestamp: i64,
}

#[tower_lsp::async_trait]
impl LanguageServer for DiscordLanguageServer {
    async fn initialize(&self, _: InitializeParams) -> jsonrpc::Result<InitializeResult> {
        debug!("Received initialization request");
        to_jsonrpc_err(self.discord.lock().await.connect())?;
        info!("Connected to Discord successfully");
        to_jsonrpc_err(
            self.discord.lock().await.set_activity(
                Activity::new()
                    .assets(Assets::new().large_image("zed").large_text("Zed"))
                    .details("Idling")
                    .timestamps(Timestamps::new().start(self.started_timestamp)),
            ),
        )?;

        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::INCREMENTAL,
                )),
                ..Default::default()
            },
            server_info: Some(ServerInfo {
                name: env!("CARGO_PKG_NAME").to_string(),
                version: Some(env!("CARGO_PKG_VERSION").to_string()),
            }),
        })
    }

    async fn shutdown(&self) -> jsonrpc::Result<()> {
        debug!("Received shutdown request");
        to_jsonrpc_err(self.discord.lock().await.close())?;

        Ok(())
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        debug!("Got a textDocument/didChange notification");

        let mut last_edited_file = self.last_edited_file.lock().await;

        if last_edited_file
            .as_ref()
            .is_none_or(|last_edited_file| last_edited_file != params.text_document.uri.path())
        {
            debug!("Editing a different file than before, updating Discord activity");

            *last_edited_file = Some(params.text_document.uri.path().to_string());

            let workspace_folders = self
                .language_client
                .workspace_folders()
                .await
                .inspect_err(|err| error!("Failed to get workspace folders: {err}"))
                .ok()
                .flatten()
                .unwrap_or_default();
            let (relative_file_path, workspace) = workspace_folders
                .iter()
                .find_map(|workspace_folder| {
                    params
                        .text_document
                        .uri
                        .path()
                        .strip_prefix(&format!("{}/", workspace_folder.uri.path()))
                        .map(|relative_file_path| {
                            (
                                Some(relative_file_path),
                                workspace_folder
                                    .uri
                                    .path_segments()
                                    .and_then(|mut path_segments| path_segments.next_back()),
                            )
                        })
                })
                .unwrap_or_else(|| {
                    (
                        params
                            .text_document
                            .uri
                            .path_segments()
                            .and_then(|mut path_segments| path_segments.next_back()),
                        None,
                    )
                });
            let language = relative_file_path
                .and_then(|relative_file_path| {
                    Language::from_str(relative_file_path)
                        .inspect_err(|err| error!("Failed to determine language: {err}"))
                        .ok()
                })
                .map(|language| language.to_string())
                .map(|language| (language.to_lowercase(), language));
            let mut assets = Assets::new().large_image("zed").large_text("Zed");

            if let Some((small_image, small_text)) = &language {
                assets = assets.small_image(small_image).small_text(small_text);
            }

            let details = match relative_file_path {
                Some(relative_file_path) => &format!("Editing {relative_file_path}"),
                None => "Editing a file",
            };
            let state = match workspace {
                Some(workspace) => &format!("in {workspace}"),
                None => "in an unknown project",
            };

            if let Err(err) = self.discord.lock().await.set_activity(
                Activity::new()
                    .assets(assets)
                    .details(details)
                    .state(state)
                    .timestamps(Timestamps::new().start(self.started_timestamp)),
            ) {
                error!("Failed to update activity: {err}");
            }
        }
    }
}

fn to_jsonrpc_err<T, E>(result: Result<T, E>) -> Result<T, jsonrpc::Error>
where
    E: std::fmt::Display,
{
    result.map_err(|err| jsonrpc::Error {
        code: jsonrpc::ErrorCode::InternalError,
        message: (jsonrpc::ErrorCode::InternalError.description().to_string()
            + &format!(": {err}"))
            .into(),
        data: None,
    })
}

#[tokio::main]
async fn main() {
    let (service, socket) = LspService::new(|language_client| {
        subscriber::set_global_default(LspSubscriber::new(language_client.clone())).unwrap();

        DiscordLanguageServer {
            language_client,
            discord: tokio::sync::Mutex::new(
                DiscordIpcClient::new("1209555626435411988")
                    .expect("should not be possible to fail to create Discord IPC client"),
            ),
            last_edited_file: tokio::sync::Mutex::new(None),
            started_timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("system time is incorrect")
                .as_secs() as i64,
        }
    });

    Server::new(tokio::io::stdin(), tokio::io::stdout(), socket)
        .serve(service)
        .await;
}
