//! Ring0-based FileIo implementation (Phase 107)
//!
//! Provides FileBox I/O by delegating to Ring0.FsApi.
//! This is the default FileIo provider registered at startup.

use crate::boxes::file::provider::{FileCaps, FileError, FileIo, FileResult};
use crate::runtime::ring0::Ring0Context;
use std::path::Path;
use std::sync::{Arc, RwLock};

/// Ring0.FsApi-based FileIo implementation
///
/// # Design (Phase 107)
///
/// **Stateful wrapper around stateless FsApi**:
/// - open(path): Stores the path for subsequent read()
/// - read(): Calls ring0.fs.read_to_string() with the stored path
/// - close(): Clears the stored path
///
/// **Design decisions**:
/// - UTF-8 handling: Uses `read_to_string()` which handles UTF-8 internally
/// - One file at a time: Calling open() twice without close() returns Err
/// - Phase 111: Supports "r", "w", "a" modes
pub struct Ring0FsFileIo {
    ring0: Arc<Ring0Context>,
    /// Current opened file path (None if no file is open)
    path: RwLock<Option<String>>,
    /// File mode ("r", "w", "a")
    mode: RwLock<Option<String>>,
}

impl Ring0FsFileIo {
    /// Create new Ring0FsFileIo with given Ring0Context
    pub fn new(ring0: Arc<Ring0Context>) -> Self {
        Self {
            ring0,
            path: RwLock::new(None),
            mode: RwLock::new(None),
        }
    }

    /// Set file mode (internal helper for FileHandleBox)
    pub fn set_mode(&self, mode: String) {
        *self.mode.write().unwrap() = Some(mode);
    }

    /// Get metadata (internal helper for FileHandleBox)
    pub fn metadata(&self) -> FileResult<crate::runtime::ring0::FsMetadata> {
        let current_path = self.path.read().unwrap();
        match current_path.as_ref() {
            Some(path) => {
                let path_obj = Path::new(path);
                self.ring0
                    .fs
                    .metadata(path_obj)
                    .map_err(|e| FileError::Io(format!("Metadata failed: {}", e)))
            }
            None => Err(FileError::Io(
                "No file path set. Call open() first.".to_string(),
            )),
        }
    }
}

impl FileIo for Ring0FsFileIo {
    fn caps(&self) -> FileCaps {
        // Phase 108: Read/write support
        FileCaps {
            read: true,
            write: true,
        }
    }

    fn open(&self, path: &str) -> FileResult<()> {
        let mut current_path = self.path.write().unwrap();

        // Phase 107 Design Decision: One file at a time
        if current_path.is_some() {
            return Err(FileError::Io(
                "File already open. Call close() before opening another file.".to_string(),
            ));
        }

        // Check if file exists using Ring0.FsApi
        let path_obj = Path::new(path);
        if !self.ring0.fs.exists(path_obj) {
            return Err(FileError::Io(format!("File not found: {}", path)));
        }

        // Store path for subsequent read()
        *current_path = Some(path.to_string());
        Ok(())
    }

    fn read(&self) -> FileResult<String> {
        let current_path = self.path.read().unwrap();

        match current_path.as_ref() {
            Some(path) => {
                // Delegate to Ring0.FsApi (UTF-8 handling is done by FsApi)
                let path_obj = Path::new(path);
                self.ring0
                    .fs
                    .read_to_string(path_obj)
                    .map_err(|e| FileError::Io(format!("Read failed: {}", e)))
            }
            None => Err(FileError::Io(
                "No file is currently open. Call open() first.".to_string(),
            )),
        }
    }

    fn write(&self, text: &str) -> FileResult<()> {
        let current_path = self.path.read().unwrap();
        let current_mode = self.mode.read().unwrap();

        match (current_path.as_ref(), current_mode.as_ref()) {
            (Some(path), Some(mode)) => {
                let path_obj = Path::new(path);

                // Mode-based write behavior (Phase 111)
                match mode.as_str() {
                    "w" => {
                        // Truncate mode: overwrite existing file
                        self.ring0
                            .fs
                            .write_all(path_obj, text.as_bytes())
                            .map_err(|e| FileError::Io(format!("Write failed: {}", e)))
                    }
                    "a" => {
                        // Append mode: append to end of file
                        self.ring0
                            .fs
                            .append_all(path_obj, text.as_bytes())
                            .map_err(|e| FileError::Io(format!("Append failed: {}", e)))
                    }
                    "r" => {
                        // Read-only mode: cannot write
                        Err(FileError::Unsupported(
                            "Cannot write in read-only mode".to_string(),
                        ))
                    }
                    _ => Err(FileError::Unsupported(format!(
                        "Unsupported mode: {}",
                        mode
                    ))),
                }
            }
            (None, _) => Err(FileError::Io(
                "No file is currently open. Call open() first.".to_string(),
            )),
            (Some(_), None) => Err(FileError::Io(
                "File mode not set. Internal error.".to_string(),
            )),
        }
    }

    fn close(&self) -> FileResult<()> {
        let mut current_path = self.path.write().unwrap();
        let mut current_mode = self.mode.write().unwrap();
        *current_path = None;
        *current_mode = None;
        Ok(())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    // Phase 114: Metadata operations

    fn exists(&self) -> bool {
        let path_lock = self.path.read().unwrap();
        match path_lock.as_ref() {
            Some(path_str) => {
                let path_obj = Path::new(path_str);
                self.ring0.fs.exists(path_obj)
            }
            None => false,
        }
    }

    fn stat(&self) -> FileResult<crate::boxes::file::provider::FileStat> {
        let path_lock = self.path.read().unwrap();
        match path_lock.as_ref() {
            Some(path_str) => {
                let path_obj = Path::new(path_str);
                let meta = self
                    .ring0
                    .fs
                    .metadata(path_obj)
                    .map_err(|e| FileError::Io(format!("Metadata failed: {}", e)))?;
                Ok(crate::boxes::file::provider::FileStat {
                    is_file: meta.is_file,
                    is_dir: meta.is_dir,
                    size: meta.len,
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
                let canonical = self
                    .ring0
                    .fs
                    .canonicalize(path_obj)
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
    use crate::runtime::ring0::default_ring0;
    use std::fs;
    use std::io::Write;

    fn setup_test_file(path: &str, content: &str) {
        let mut file = fs::File::create(path).unwrap();
        file.write_all(content.as_bytes()).unwrap();
    }

    fn cleanup_test_file(path: &str) {
        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_ring0fs_fileio_basic_operations() {
        let test_path = "/tmp/phase107_test_basic.txt";
        let test_content = "Hello, Ring0.FsApi!";

        setup_test_file(test_path, test_content);

        let ring0 = Arc::new(default_ring0());
        let fileio = Ring0FsFileIo::new(ring0);

        // Test capabilities (Phase 108: write support added)
        let caps = fileio.caps();
        assert!(caps.read);
        assert!(caps.write);

        // Test open
        assert!(fileio.open(test_path).is_ok());

        // Test read
        let content = fileio.read().unwrap();
        assert_eq!(content, test_content);

        // Test close
        assert!(fileio.close().is_ok());

        cleanup_test_file(test_path);
    }

    #[test]
    fn test_ring0fs_fileio_double_open_error() {
        let test_path = "/tmp/phase107_test_double_open.txt";
        setup_test_file(test_path, "test");

        let ring0 = Arc::new(default_ring0());
        let fileio = Ring0FsFileIo::new(ring0);

        // First open succeeds
        assert!(fileio.open(test_path).is_ok());

        // Second open fails (one file at a time)
        let result = fileio.open(test_path);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("already open"));

        // Close and re-open should succeed
        assert!(fileio.close().is_ok());
        assert!(fileio.open(test_path).is_ok());

        cleanup_test_file(test_path);
    }

    #[test]
    fn test_ring0fs_fileio_read_without_open() {
        let ring0 = Arc::new(default_ring0());
        let fileio = Ring0FsFileIo::new(ring0);

        // Read without open should fail
        let result = fileio.read();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("No file is currently open"));
    }

    #[test]
    fn test_ring0fs_fileio_nonexistent_file() {
        let ring0 = Arc::new(default_ring0());
        let fileio = Ring0FsFileIo::new(ring0);

        // Open nonexistent file should fail
        let result = fileio.open("/tmp/nonexistent_phase107_file.txt");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }

    // ===== Phase 108: Write tests =====

    #[test]
    fn test_filebox_write_read_roundtrip() {
        let test_path = "/tmp/phase108_roundtrip.txt";
        let test_content = "Hello, Phase 108!";

        // Setup: Create file first (open() requires file to exist)
        setup_test_file(test_path, "initial");

        let ring0 = Arc::new(default_ring0());
        let fileio = Ring0FsFileIo::new(ring0);

        // Test capabilities
        let caps = fileio.caps();
        assert!(caps.read);
        assert!(caps.write);

        // Write content (truncate mode)
        fileio.set_mode("w".to_string()); // Phase 114: set mode before open
        assert!(fileio.open(test_path).is_ok());
        assert!(fileio.write(test_content).is_ok());
        assert!(fileio.close().is_ok());

        // Read back and verify
        fileio.set_mode("r".to_string()); // Phase 114: set mode before open
        assert!(fileio.open(test_path).is_ok());
        let content = fileio.read().unwrap();
        assert_eq!(content, test_content);
        assert!(fileio.close().is_ok());

        // Cleanup
        cleanup_test_file(test_path);
    }

    #[test]
    fn test_filebox_write_truncate_mode() {
        let test_path = "/tmp/phase108_truncate.txt";
        setup_test_file(test_path, "Original content");

        let ring0 = Arc::new(default_ring0());
        let fileio = Ring0FsFileIo::new(ring0);

        // Overwrite with new content (truncate mode)
        fileio.set_mode("w".to_string()); // Phase 114: set mode before open
        assert!(fileio.open(test_path).is_ok());
        assert!(fileio.write("New content").is_ok());
        assert!(fileio.close().is_ok());

        // Verify truncate behavior
        fileio.set_mode("r".to_string()); // Phase 114: set mode before open
        assert!(fileio.open(test_path).is_ok());
        let content = fileio.read().unwrap();
        assert_eq!(content, "New content");
        assert!(fileio.close().is_ok());

        cleanup_test_file(test_path);
    }

    #[test]
    fn test_filebox_write_without_open() {
        let ring0 = Arc::new(default_ring0());
        let fileio = Ring0FsFileIo::new(ring0);

        // Write without open should fail
        let result = fileio.write("test");
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("No file is currently open"));
    }

    // ===== Phase 114: Metadata operation tests =====

    #[test]
    fn test_ring0_fs_fileio_stat_default_profile() {
        let test_path = "/tmp/phase114_stat_test.txt";
        let test_content = "Hello, Phase 114!";

        setup_test_file(test_path, test_content);

        let ring0 = Arc::new(default_ring0());
        let fileio = Ring0FsFileIo::new(ring0);

        // Open file
        assert!(fileio.open(test_path).is_ok());

        // Test stat()
        let stat = fileio.stat().expect("stat should succeed");
        assert!(stat.is_file, "should be a file");
        assert!(!stat.is_dir, "should not be a directory");
        assert_eq!(stat.size, test_content.len() as u64, "size should match");

        fileio.close().unwrap();
        cleanup_test_file(test_path);
    }

    #[test]
    fn test_ring0_fs_fileio_exists_default_profile() {
        let test_path = "/tmp/phase114_exists_test.txt";
        setup_test_file(test_path, "test");

        let ring0 = Arc::new(default_ring0());
        let fileio = Ring0FsFileIo::new(ring0);

        // Open file
        assert!(fileio.open(test_path).is_ok());

        // Test exists() - should return true for existing file
        assert!(fileio.exists(), "exists() should return true");

        fileio.close().unwrap();
        cleanup_test_file(test_path);

        // After file is deleted, exists() with no path set should return false
        assert!(
            !fileio.exists(),
            "exists() should return false when no path set"
        );
    }

    #[test]
    fn test_ring0_fs_fileio_canonicalize_default_profile() {
        let test_path = "/tmp/phase114_canonicalize.txt";
        setup_test_file(test_path, "test");

        let ring0 = Arc::new(default_ring0());
        let fileio = Ring0FsFileIo::new(ring0);

        // Open file
        assert!(fileio.open(test_path).is_ok());

        // Test canonicalize()
        let canonical = fileio.canonicalize().expect("canonicalize should succeed");
        assert!(
            canonical.contains("phase114_canonicalize.txt"),
            "should contain filename"
        );
        assert!(canonical.starts_with("/"), "should be absolute path");

        fileio.close().unwrap();
        cleanup_test_file(test_path);
    }

    #[test]
    fn test_ring0_fs_fileio_stat_without_open() {
        let ring0 = Arc::new(default_ring0());
        let fileio = Ring0FsFileIo::new(ring0);

        // stat() without open() should fail
        let result = fileio.stat();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("No file path set"));
    }

    #[test]
    fn test_ring0_fs_fileio_canonicalize_without_open() {
        let ring0 = Arc::new(default_ring0());
        let fileio = Ring0FsFileIo::new(ring0);

        // canonicalize() without open() should fail
        let result = fileio.canonicalize();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("No file path set"));
    }
}
