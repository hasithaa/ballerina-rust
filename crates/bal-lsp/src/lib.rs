//! Ballerina Language Server Protocol Implementation

use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};

pub struct BalLanguageServer {
    client: Client,
}

#[tower_lsp::async_trait]
impl LanguageServer for BalLanguageServer {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult::default())
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "Ballerina language server initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }
}

mod capabilities {
    // TODO: Add capabilities module implementation
}

mod handlers {
    // TODO: Add handlers module implementation
}

mod utils {
    // TODO: Add utils module implementation
}
