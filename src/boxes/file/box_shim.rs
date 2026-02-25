//! Thin FileBox shim that delegates to a selected provider.
//! Not wired into the registry yet (safe placeholder).

use super::provider::{FileCaps, FileIo, FileResult};
use std::sync::Arc;

#[allow(dead_code)]
pub struct FileBoxShim {
    provider: Arc<dyn FileIo>,
    caps: FileCaps,
}

#[allow(dead_code)]
impl FileBoxShim {
    pub fn new(provider: Arc<dyn FileIo>) -> Self {
        let caps = provider.caps();
        Self { provider, caps }
    }
    pub fn open(&self, path: &str) -> FileResult<()> {
        self.provider.open(path)
    }
    pub fn read(&self) -> FileResult<String> {
        self.provider.read()
    }
    pub fn close(&self) -> FileResult<()> {
        self.provider.close()
    }
    pub fn caps(&self) -> FileCaps {
        self.caps
    }
}
