use core::fmt;
use std::{cell::RefCell, collections::HashMap, sync::Mutex};

use ropey::Rope;
use tower_lsp::{lsp_types::Url, Client};

use crate::wrapper::ProgramArchive;

struct Backend {
    client: Client,
    documents: Mutex<RefCell<HashMap<Url, DocumentData>>>,
}

#[derive(Debug)]
struct DocumentData {
    content: Rope,
    optionals: Option<OptionalDocumentData>,
}

#[derive(Debug)]
struct OptionalDocumentData {
    archive: ProgramArchive,
    main_file_id: usize,
}
