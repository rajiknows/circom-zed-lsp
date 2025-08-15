use tower_lsp::{LspService, Server};

use crate::backend::Backend;
mod ast;
mod backend;
mod cli;
mod constansts;
mod lsp_types_util;
mod parse;
mod wrapper;

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(Backend::new);
    Server::new(stdin, stdout, socket).serve(service).await;
}
