use rusqlite::Connection;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;

pub struct Indexer {
    pub(crate) db: Arc<Mutex<Connection>>,
    // Add fields for tree-sitter parser, etc.
}

impl Indexer {
    pub(crate) fn new(db: Arc<Mutex<Connection>>) -> Self {
        // Initialize indexer
        Indexer { db }
    }

    pub(crate) fn index_file(&self, path: &str) {}

    pub(crate) fn index_workspace(&self, root: &str) {
        // Walk directory
        // Call index_file for each relevant file
    }
}

pub(crate) struct LspState {
    pub(crate) db: Arc<Mutex<Connection>>,
    pub(crate) indexer: Arc<IndexerHandle>,
}

pub(crate) struct IndexerHandle {
    pub(crate) sender: mpsc::Sender<IndexerCommand>,
}

pub(crate) enum IndexerCommand {
    IndexFile(String),
    IndexWorkspace(String),
}
