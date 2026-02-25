//! Phase 109: NoFs profile FileIo stub
//!
//! Provides a stub FileIo implementation that returns errors for all operations.
//! Used when FileBox is disabled in no-fs runtime profile.

use crate::boxes::file::provider::{FileCaps, FileError, FileIo, FileResult};

/// Phase 109: No-filesystem FileIo stub
///
/// Returns Unsupported errors for all operations.
/// Used in NoFs runtime profile where FileBox is disabled.
///
/// # Design
///
/// - caps(): Returns read=false, write=false
/// - All operations: Return FileError::Unsupported with clear message
///
/// # Logger/ConsoleService availability (Phase 109 Modification 2)
///
/// ✅ Still available in NoFs profile:
/// - Ring0.log (OS abstraction layer - panic/exit final output)
/// - ConsoleBox (language-level console - stdout/stderr)
/// - Core required boxes (String/Integer/Bool/Array/Map/Console)
///
/// ❌ Disabled in NoFs profile:
/// - FileBox (filesystem-dependent)
/// - Optional boxes (Regex/Time/JSON - future: profile-controlled)
pub struct NoFsFileIo;

impl FileIo for NoFsFileIo {
    fn caps(&self) -> FileCaps {
        FileCaps {
            read: false,
            write: false,
        }
    }

    fn open(&self, _path: &str) -> FileResult<()> {
        Err(FileError::Unsupported(
            "FileBox disabled in NoFs profile".to_string(),
        ))
    }

    fn read(&self) -> FileResult<String> {
        Err(FileError::Unsupported(
            "FileBox disabled in NoFs profile".to_string(),
        ))
    }

    fn write(&self, _text: &str) -> FileResult<()> {
        Err(FileError::Unsupported(
            "FileBox disabled in NoFs profile".to_string(),
        ))
    }

    fn close(&self) -> FileResult<()> {
        Err(FileError::Unsupported(
            "FileBox disabled in NoFs profile".to_string(),
        ))
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    // Phase 114: Metadata operations (stub)

    fn exists(&self) -> bool {
        // NoFs profile: all files are considered non-existent
        false
    }

    fn stat(&self) -> FileResult<crate::boxes::file::provider::FileStat> {
        Err(FileError::Unsupported(
            "FileSystem operations disabled in no-fs profile".to_string(),
        ))
    }

    fn canonicalize(&self) -> FileResult<String> {
        Err(FileError::Unsupported(
            "FileSystem operations disabled in no-fs profile".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nofs_fileio_caps() {
        let fileio = NoFsFileIo;
        let caps = fileio.caps();
        assert!(!caps.read, "NoFsFileIo should report read=false");
        assert!(!caps.write, "NoFsFileIo should report write=false");
    }

    #[test]
    fn test_nofs_fileio_open_unsupported() {
        let fileio = NoFsFileIo;
        let result = fileio.open("/tmp/test.txt");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("unsupported"));
    }

    #[test]
    fn test_nofs_fileio_read_unsupported() {
        let fileio = NoFsFileIo;
        let result = fileio.read();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("unsupported"));
    }

    #[test]
    fn test_nofs_fileio_write_unsupported() {
        let fileio = NoFsFileIo;
        let result = fileio.write("test");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("unsupported"));
    }

    #[test]
    fn test_nofs_fileio_close_unsupported() {
        let fileio = NoFsFileIo;
        let result = fileio.close();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("unsupported"));
    }

    // ===== Phase 114: Metadata operation tests =====

    #[test]
    fn test_nofs_fileio_exists() {
        let fileio = NoFsFileIo;
        // NoFsFileIo.exists() should always return false
        assert!(
            !fileio.exists(),
            "NoFsFileIo.exists() should always return false"
        );
    }

    #[test]
    fn test_nofs_fileio_stat_error() {
        let fileio = NoFsFileIo;
        let result = fileio.stat();
        assert!(result.is_err(), "stat() should return error");
        assert!(
            result.unwrap_err().to_string().contains("unsupported"),
            "should contain 'unsupported'"
        );
    }

    #[test]
    fn test_nofs_fileio_canonicalize_error() {
        let fileio = NoFsFileIo;
        let result = fileio.canonicalize();
        assert!(result.is_err(), "canonicalize() should return error");
        assert!(
            result.unwrap_err().to_string().contains("unsupported"),
            "should contain 'unsupported'"
        );
    }
}
