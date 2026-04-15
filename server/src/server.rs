use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

#[allow(warnings)]
struct RustfmtLSP {
    client: Client,
}

#[allow(warnings)]
fn completions_for_context() -> Vec<CompletionItem> {
    let keys = vec![(
        "array_width",
        "Maximum width of an array literal before falling back to vertical formatting.",
        "60",
    )];

    keys.into_iter()
        .map(|(key, doc, default)| CompletionItem {
            label: key.to_string(),
            kind: Some(CompletionItemKind::PROPERTY),
            detail: Some(format!("Default: {}", default)),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: doc.to_string(),
            })),
            insert_text: Some(format!("{} = {}", key, default)),
            insert_text_format: Some(InsertTextFormat::PLAIN_TEXT),
            ..Default::default()
        })
        .collect()
}

#[tower_lsp::async_trait]
impl LanguageServer for RustfmtLSP {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: Some(vec!["\n".into(), " ".into()]),
                    ..Default::default()
                }),
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "rustfmt-lsp initialized")
            .await;
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        // let uri = params.text_document_position.text_document.uri;
        // let pos = params.text_document_position.position;
        // let doc_text = self.client.get(&uri);
        // let current_line = doc_text.lines().nth(pos.line as usize).unwrap_or("");

        let items = completions_for_context();
        Ok(Some(CompletionResponse::Array(items)))
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }
}

#[allow(warnings)]
#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| RustfmtLSP { client });
    Server::new(stdin, stdout, socket).serve(service).await;
}
