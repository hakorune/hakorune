//! Core read‑only File I/O provider (ring‑1).
//! Provides basic read-only file operations using std::fs::File.

use crate::boxes::file::provider::{normalize_newlines, FileCaps, FileError, FileIo, FileResult};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::RwLock;

pub struct CoreRoFileIo {
    handle: RwLock<Option<File>>,
    /// Store path for metadata operations (Phase 114)
    path: RwLock<Option<String>>,
}

impl CoreRoFileIo {
    pub fn new() -> Self {
        Self {
            handle: RwLock::new(None),
            path: RwLock::new(None),
        }
    }
}

impl FileIo for CoreRoFileIo {
    fn caps(&self) -> FileCaps {
        FileCaps::read_only()
    }

    fn open(&self, path: &str) -> FileResult<()> {
        let file = File::open(path)
            .map_err(|e| FileError::Io(format!("Failed to open {}: {}", path, e)))?;
        *self.handle.write().unwrap() = Some(file);
        *self.path.write().unwrap() = Some(path.to_string()); // Phase 114: Store path
        Ok(())
    }

    fn read(&self) -> FileResult<String> {
        let mut handle = self.handle.write().unwrap();
        if let Some(ref mut file) = *handle {
            let mut content = String::new();
            file.read_to_string(&mut content)
                .map_err(|e| FileError::Io(format!("Read failed: {}", e)))?;
            Ok(normalize_newlines(&content))
        } else {
            Err(FileError::Io("No file opened".to_string()))
        }
    }

    fn write(&self, _text: &str) -> FileResult<()> {
        // CoreRoFileIo is read-only, write is not supported
        Err(FileError::Unsupported(
            "CoreRoFileIo is read-only".to_string(),
        ))
    }

    fn close(&self) -> FileResult<()> {
        *self.handle.write().unwrap() = None;
        *self.path.write().unwrap() = None; // Phase 114: Clear path
        Ok(())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    // Phase 114: Metadata operations

    fn exists(&self) -> bool {
        let path_lock = self.path.read().unwrap();
        match path_lock.as_ref() {
            Some(path_str) => Path::new(path_str).exists(),
            None => false,
        }
    }

    fn stat(&self) -> FileResult<crate::boxes::file::provider::FileStat> {
        let path_lock = self.path.read().unwrap();
        match path_lock.as_ref() {
            Some(path_str) => {
                let path_obj = Path::new(path_str);
                let meta = std::fs::metadata(path_obj)
                    .map_err(|e| FileError::Io(format!("Metadata failed: {}", e)))?;
                Ok(crate::boxes::file::provider::FileStat {
                    is_file: meta.is_file(),
                    is_dir: meta.is_dir(),
                    size: meta.len(),
                })
            }
            None => Err(FileError::Io(
                "No file path set. Call open() first.".to_string(),
            )),
        }
    }

    fn canonicalize(&self) -> FileResult<String> {
        let path_lock = self.path.read().unwrap();
        match path_lock.as_ref() {
            Some(path_str) => {
                let path_obj = Path::new(path_str);
                let canonical = std::fs::canonicalize(path_obj)
                    .map_err(|e| FileError::Io(format!("Canonicalize failed: {}", e)))?;
                Ok(canonical.display().to_string())
            }
            None => Err(FileError::Io(
                "No file path set. Call open() first.".to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_core_ro_write_unsupported() {
        let fileio = CoreRoFileIo::new();

        // Write should fail with Unsupported
        let result = fileio.write("test");
        assert!(result.is_err());
        match result.unwrap_err() {
            FileError::Unsupported(_) => { /* expected */ }
            _ => panic!("Expected Unsupported error"),
        }
    }
}
