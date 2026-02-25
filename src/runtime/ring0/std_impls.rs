//! Phase 88: std ベースの Ring0 デフォルト実装

use super::errors::{IoError, TimeError};
use super::traits::*;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::SystemTime;

/// noop メモリ実装（Phase 88: 将来 hakmem に接続）
pub struct NoopMem;

impl MemApi for NoopMem {
    fn alloc(&self, _size: usize) -> *mut u8 {
        std::ptr::null_mut()
    }

    fn free(&self, _ptr: *mut u8) {}

    fn stats(&self) -> MemStats {
        MemStats::default()
    }
}

/// std::alloc ベースのメモリ実装 (Phase 102)
///
/// # 設計
/// - malloc/freeで実メモリ割り当て
/// - allocated/freed/currentの統計管理（sync型）
///
/// # 注意
/// - unsafe ポインタ操作は caller責任
/// - StdMem自体は Thread-safe (Mutex)
pub struct StdMem {
    stats: Arc<Mutex<MemStats>>,
}

impl StdMem {
    pub fn new() -> Self {
        Self {
            stats: Arc::new(Mutex::new(MemStats::default())),
        }
    }
}

impl MemApi for StdMem {
    fn alloc(&self, size: usize) -> *mut u8 {
        if size == 0 {
            return std::ptr::null_mut();
        }

        // stdlib allocatorを使用
        let layout = unsafe { std::alloc::Layout::from_size_align_unchecked(size, 1) };
        let ptr = unsafe { std::alloc::alloc(layout) };

        if !ptr.is_null() {
            let mut s = self.stats.lock().unwrap();
            s.allocated += size;
            s.current += size;
        }

        ptr
    }

    fn free(&self, ptr: *mut u8) {
        if ptr.is_null() {
            return;
        }

        // 注意: size情報がないため、完全な統計は不正確
        // Phase 102B (hakmem統合)でサイズペアリング導入予定
        let mut s = self.stats.lock().unwrap();
        s.freed += 1; // freed countのみ計上（サイズは未確定）
    }

    fn stats(&self) -> MemStats {
        self.stats.lock().unwrap().clone()
    }
}

/// std::io ベースの IO 実装
pub struct StdIo;

impl IoApi for StdIo {
    fn stdout_write(&self, data: &[u8]) -> Result<(), IoError> {
        use std::io::Write;
        std::io::stdout()
            .write_all(data)
            .map_err(|e| IoError::StdoutWriteFailed(format!("{}", e)))
    }

    fn stderr_write(&self, data: &[u8]) -> Result<(), IoError> {
        use std::io::Write;
        std::io::stderr()
            .write_all(data)
            .map_err(|e| IoError::StderrWriteFailed(format!("{}", e)))
    }

    fn stdin_read(&self, buf: &mut [u8]) -> Result<usize, IoError> {
        use std::io::Read;
        std::io::stdin()
            .read(buf)
            .map_err(|e| IoError::StdinReadFailed(format!("{}", e)))
    }
}

/// std::time ベースの時刻実装
pub struct StdTime;

impl TimeApi for StdTime {
    fn now(&self) -> Result<SystemTime, TimeError> {
        Ok(SystemTime::now())
    }

    fn monotonic_now(&self) -> Result<std::time::Instant, TimeError> {
        Ok(std::time::Instant::now())
    }

    fn elapsed(&self, start: std::time::Instant) -> std::time::Duration {
        start.elapsed()
    }
}

/// eprintln!/println! ベースのログ実装
pub struct StdLog;

impl StdLog {
    fn should_log(&self, level: LogLevel) -> bool {
        let min_level_str =
            crate::config::env::ring0_log_level().unwrap_or_else(|| "INFO".to_string());

        let min_level = match min_level_str.to_uppercase().as_str() {
            "DEBUG" => LogLevel::Debug,
            "INFO" => LogLevel::Info,
            "WARN" => LogLevel::Warn,
            "ERROR" => LogLevel::Error,
            _ => LogLevel::Info,
        };

        // level の優先度が min_level 以上なら true
        matches!(
            (level, min_level),
            (LogLevel::Error, _)
                | (
                    LogLevel::Warn,
                    LogLevel::Debug | LogLevel::Info | LogLevel::Warn
                )
                | (LogLevel::Info, LogLevel::Debug | LogLevel::Info)
                | (LogLevel::Debug, LogLevel::Debug)
        )
    }
}

impl LogApi for StdLog {
    fn log(&self, level: LogLevel, msg: &str) {
        if !self.should_log(level) {
            return;
        }

        match level {
            LogLevel::Debug => eprintln!("[DEBUG] {}", msg),
            LogLevel::Info => eprintln!("[INFO] {}", msg),
            LogLevel::Warn => eprintln!("[WARN] {}", msg),
            LogLevel::Error => eprintln!("[ERROR] {}", msg),
        }
    }
}

/// std::fs ベースのファイルシステム実装 (Phase 90-A)
pub struct StdFs;

impl FsApi for StdFs {
    fn read_to_string(&self, path: &Path) -> Result<String, IoError> {
        std::fs::read_to_string(path)
            .map_err(|e| IoError::ReadFailed(format!("read_to_string({}): {}", path.display(), e)))
    }

    fn read(&self, path: &Path) -> Result<Vec<u8>, IoError> {
        std::fs::read(path)
            .map_err(|e| IoError::ReadFailed(format!("read({}): {}", path.display(), e)))
    }

    fn write_all(&self, path: &Path, data: &[u8]) -> Result<(), IoError> {
        std::fs::write(path, data)
            .map_err(|e| IoError::WriteFailed(format!("write({}): {}", path.display(), e)))
    }

    fn append_all(&self, path: &Path, data: &[u8]) -> Result<(), IoError> {
        use std::fs::OpenOptions;
        use std::io::Write;

        let mut file = OpenOptions::new()
            .create(true) // 存在しなければ作成
            .append(true) // append モードで開く
            .open(path)
            .map_err(|e| IoError::WriteFailed(format!("append_all({}): {}", path.display(), e)))?;

        file.write_all(data)
            .map_err(|e| IoError::WriteFailed(format!("append write({}): {}", path.display(), e)))
    }

    fn exists(&self, path: &Path) -> bool {
        path.exists()
    }

    fn metadata(&self, path: &Path) -> Result<FsMetadata, IoError> {
        let meta = std::fs::metadata(path)
            .map_err(|e| IoError::MetadataFailed(format!("metadata({}): {}", path.display(), e)))?;
        Ok(FsMetadata {
            is_file: meta.is_file(),
            is_dir: meta.is_dir(),
            len: meta.len(),
        })
    }

    fn canonicalize(&self, path: &Path) -> Result<PathBuf, IoError> {
        std::fs::canonicalize(path).map_err(|e| {
            IoError::CanonicalizeFailed(format!("canonicalize({}): {}", path.display(), e))
        })
    }
}

/// std::thread ベースのスレッド実装 (Phase 90-D)
pub struct StdThread;

impl ThreadApi for StdThread {
    fn sleep(&self, duration: std::time::Duration) {
        std::thread::sleep(duration);
    }
}

/// Phase 112: No-FS profile 用 FsApi stub
///
/// FileSystem 操作がすべて「無効」として機能する。
/// Phase 109 の NoFsFileIo（FileIo trait）と異なり、
/// Ring0 レベルの FsApi trait を実装する。
const NOFS_ERROR_MSG: &str = "FileSystem operations disabled in no-fs profile";

pub struct NoFsApi;

impl FsApi for NoFsApi {
    fn read_to_string(&self, _path: &Path) -> Result<String, IoError> {
        Err(IoError::Other(NOFS_ERROR_MSG.to_string()))
    }

    fn read(&self, _path: &Path) -> Result<Vec<u8>, IoError> {
        Err(IoError::Other(NOFS_ERROR_MSG.to_string()))
    }

    fn write_all(&self, _path: &Path, _data: &[u8]) -> Result<(), IoError> {
        Err(IoError::Other(NOFS_ERROR_MSG.to_string()))
    }

    fn append_all(&self, _path: &Path, _data: &[u8]) -> Result<(), IoError> {
        Err(IoError::Other(NOFS_ERROR_MSG.to_string()))
    }

    fn exists(&self, _path: &Path) -> bool {
        false
    }

    fn metadata(&self, _path: &Path) -> Result<FsMetadata, IoError> {
        Err(IoError::Other(NOFS_ERROR_MSG.to_string()))
    }

    fn canonicalize(&self, _path: &Path) -> Result<PathBuf, IoError> {
        Err(IoError::Other(NOFS_ERROR_MSG.to_string()))
    }
}

// ===== テスト (Phase 102) =====

#[cfg(test)]
mod stdmem_tests {
    use super::*;

    fn unsafe_dealloc(ptr: *mut u8, size: usize) {
        unsafe { std::alloc::dealloc(ptr, std::alloc::Layout::from_size_align_unchecked(size, 1)) }
    }

    #[test]
    fn test_stdmem_alloc() {
        let mem = StdMem::new();
        let ptr = mem.alloc(1024);
        assert!(!ptr.is_null(), "alloc(1024) should return non-null pointer");

        let stats = mem.stats();
        assert_eq!(stats.allocated, 1024, "allocated should be 1024");
        assert_eq!(stats.current, 1024, "current should be 1024");

        // Clean up
        unsafe_dealloc(ptr, 1024);
    }

    #[test]
    fn test_stdmem_zero_alloc() {
        let mem = StdMem::new();
        let ptr = mem.alloc(0);
        assert!(ptr.is_null(), "alloc(0) should return null pointer");
    }

    #[test]
    fn test_stdmem_free() {
        let mem = StdMem::new();
        let ptr = mem.alloc(512);
        assert!(!ptr.is_null());

        mem.free(ptr);
        let stats = mem.stats();
        assert_eq!(stats.freed, 1, "freed count should be 1");

        // Clean up (actual deallocation)
        unsafe_dealloc(ptr, 512);
    }

    #[test]
    fn test_noopmem_compatibility() {
        let mem = NoopMem;
        let ptr = mem.alloc(1024);
        assert!(
            ptr.is_null(),
            "NoopMem should still return null for compatibility"
        );
    }
}
