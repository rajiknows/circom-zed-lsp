use std::{cell::RefCell, collections::HashMap};

use tokio::sync::Mutex;
use tower_lsp::{lsp_types::Url, Client, LspService, Server};
mod backend;
mod wrapper;

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(Backend::new);
    Server::new(stdin, stdout, socket).serve(service).await;
}
