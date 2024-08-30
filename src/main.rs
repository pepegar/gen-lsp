use rusqlite::Connection;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

struct Indexer {
    db: Arc<Mutex<Connection>>,
    // Add fields for tree-sitter parser, etc.
}

impl Indexer {
    fn new(db: Arc<Mutex<Connection>>) -> Self {
        // Initialize indexer
        Indexer { db }
    }

    fn index_file(&self, path: &str) {}

    fn index_workspace(&self, root: &str) {
        // Walk directory
        // Call index_file for each relevant file
    }
}

struct LspState {
    db: Arc<Mutex<Connection>>,
    indexer: Arc<IndexerHandle>,
}

struct IndexerHandle {
    sender: mpsc::Sender<IndexerCommand>,
}
impl IndexerHandle {}

enum IndexerCommand {
    IndexFile(String),
    IndexWorkspace(String),
}
macro_rules! lsp_debug {
    ($self:expr, $($arg:tt)*) => {
        $self.send_debug_message(&format!($($arg)*)).await
    };
}

struct GenericLspServer {
    client: Client,
    state: Arc<LspState>,
}

impl GenericLspServer {
    async fn send_debug_message(&self, message: &str) {
        self.client.log_message(MessageType::LOG, message).await;
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for GenericLspServer {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        // Start indexing the workspace
        lsp_debug!(self, "Server initializing with config: {:?}", params);
        if let Some(root_uri) = params.root_uri {
            self.state
                .indexer
                .sender
                .send(IndexerCommand::IndexWorkspace(root_uri.to_string()))
                .await
                .ok();
        }
        // Return capabilities
        //
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                // Add more capabilities as you implement them
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "server initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        // Queue re-indexing of the changed file
        let uri = params.text_document.uri.to_string();
        self.state
            .indexer
            .sender
            .send(IndexerCommand::IndexFile(uri))
            .await
            .ok();
    }

    // Implement other LSP methods...
}

#[tokio::main]
async fn main() {
    let db = Arc::new(Mutex::new(Connection::open("lsp.db").unwrap()));

    let (tx, mut rx) = mpsc::channel(100);
    let indexer = Arc::new(IndexerHandle { sender: tx });
    let indexer_worker = Arc::new(IndexerHandle {
        sender: indexer.sender.clone(),
    });

    let state = Arc::new(LspState {
        db: db.clone(),
        indexer,
    });

    // Spawn indexer worker
    tokio::spawn(async move {
        let indexer = Indexer::new(db.clone());
        while let Some(cmd) = rx.recv().await {
            match cmd {
                IndexerCommand::IndexFile(path) => indexer.index_file(&path),
                IndexerCommand::IndexWorkspace(root) => indexer.index_workspace(&root),
            }
        }
    });

    // Create and run the LSP service
    let (service, socket) = LspService::new(|client| GenericLspServer {
        client,
        state: state.clone(),
    });
    Server::new(tokio::io::stdin(), tokio::io::stdout(), socket)
        .serve(service)
        .await;
}
