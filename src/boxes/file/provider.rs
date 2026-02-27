//! File I/O provider SSOT (trait + shared types)
//!
//! This module defines the unified File I/O abstraction used by both the
//! core read‑only implementation and the plugin implementation.
//!
//! # Phase 107: Ring0.FsApi 統合
//!
//! **FileIo = 現在開いているファイルハンドルに対する操作（stateful）**
//! - open() でファイルを開く
//! - read() で内容を読み込む
//! - close() でファイルを閉じる
//!
//! **FsApi = stateless な OS ファイル I/O 抽象（Ring0）**
//! - Path → 直接 read/write
//! - FileIo 実装は内部で FsApi を使用する
//!
//! **設計原則**:
//! - FileIo は「現在開いているファイル」の状態を管理
//! - FsApi は「パス → データ」の変換のみ担当
//! - 実装例: Ring0FsFileIo が FsApi を内部で使用

/// File capabilities (minimal flag set)
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct FileCaps {
    pub read: bool,
    pub write: bool,
}

impl FileCaps {
    pub const fn read_only() -> Self {
        Self {
            read: true,
            write: false,
        }
    }

    /// Phase 110.5: Capability check helper
    ///
    /// Validates if the given mode ("r", "w", or "a") is supported by this provider.
    ///
    /// # Arguments
    ///
    /// - mode: "r" (read), "w" (write), or "a" (append)
    ///
    /// # Returns
    ///
    /// - Ok(()) if mode is supported
    /// - Err(String) if mode is not supported
    ///
    /// # Design Benefits
    ///
    /// - **DRY**: Single implementation for all mode checks
    /// - **SSOT**: Capability checking logic in one place
    /// - **Consistent errors**: Same error messages everywhere
    pub fn check_mode(&self, mode: &str) -> Result<(), String> {
        match mode {
            "r" => {
                if !self.read {
                    return Err("Read not supported by FileBox provider".to_string());
                }
            }
            "w" | "a" => {
                // Phase 111: "a" added
                if !self.write {
                    return Err("Write not supported by FileBox provider".to_string());
                }
            }
            _ => {
                return Err(format!("Unsupported mode: {}", mode));
            }
        }
        Ok(())
    }
}

/// Phase 114: File statistics
#[derive(Debug, Clone, Copy)]
pub struct FileStat {
    pub is_file: bool,
    pub is_dir: bool,
    pub size: u64,
}

/// Unified error type (thin placeholder for now)
#[derive(thiserror::Error, Debug)]
pub enum FileError {
    #[error("io error: {0}")]
    Io(String),
    #[error("unsupported operation: {0}")]
    Unsupported(String),
}

pub type FileResult<T> = Result<T, FileError>;

/// Single source of truth for File I/O semantics
pub trait FileIo: Send + Sync {
    fn caps(&self) -> FileCaps;
    fn open(&self, path: &str) -> FileResult<()>;
    fn read(&self) -> FileResult<String>;
    fn read_bytes(&self) -> FileResult<Vec<u8>> {
        self.read().map(|text| text.into_bytes())
    }
    fn write(&self, text: &str) -> FileResult<()>; // Phase 108: write support
    fn write_bytes(&self, bytes: &[u8]) -> FileResult<()> {
        let text = std::str::from_utf8(bytes).map_err(|_| {
            FileError::Unsupported(
                "Binary write is not supported by this FileBox provider".to_string(),
            )
        })?;
        self.write(text)
    }
    fn close(&self) -> FileResult<()>;

    /// Phase 111: Downcast support for metadata access
    fn as_any(&self) -> &dyn std::any::Any;

    // Phase 114: Metadata operations
    /// Check if the file exists
    fn exists(&self) -> bool;

    /// Get file statistics (metadata)
    fn stat(&self) -> FileResult<FileStat>;

    /// Get canonicalized absolute path
    fn canonicalize(&self) -> FileResult<String>;
}

/// Normalize newlines to LF (optional helper)
#[allow(dead_code)]
pub fn normalize_newlines(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for b in s.as_bytes() {
        if *b == b'\r' {
            continue;
        }
        out.push(*b as char);
    }
    out
}
