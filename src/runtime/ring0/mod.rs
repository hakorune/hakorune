//! Phase 88: Ring0Context - OS API 抽象化レイヤー
//!
//! Ring0 は Box を知らない、Nyash を知らない純粋な OS API 層。

mod errors;
mod std_impls;
mod traits;

pub use errors::{IoError, TimeError};
pub use std_impls::{NoFsApi, NoopMem, StdFs, StdIo, StdLog, StdMem, StdThread, StdTime};
pub use traits::{
    FsApi, FsMetadata, IoApi, LogApi, LogLevel, MemApi, MemStats, ThreadApi, TimeApi,
};

use crate::runtime::runtime_profile::RuntimeProfile;
use std::sync::{Arc, OnceLock};

/// Phase 88: Ring0 コンテキスト
///
/// OS API レイヤーを trait で抽象化し、1つの構造体に束ねる。
pub struct Ring0Context {
    pub mem: Arc<dyn MemApi>,
    pub io: Arc<dyn IoApi>,
    pub time: Arc<dyn TimeApi>,
    pub log: Arc<dyn LogApi>,
    pub fs: Arc<dyn FsApi>,         // Phase 90-A
    pub thread: Arc<dyn ThreadApi>, // Phase 90-D
}

impl Ring0Context {
    /// 新規 Ring0Context を作成
    pub fn new(
        mem: Arc<dyn MemApi>,
        io: Arc<dyn IoApi>,
        time: Arc<dyn TimeApi>,
        log: Arc<dyn LogApi>,
        fs: Arc<dyn FsApi>,
        thread: Arc<dyn ThreadApi>,
    ) -> Self {
        Self {
            mem,
            io,
            time,
            log,
            fs,
            thread,
        }
    }
}

impl std::fmt::Debug for Ring0Context {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Ring0Context")
            .field("mem", &"<dyn MemApi>")
            .field("io", &"<dyn IoApi>")
            .field("time", &"<dyn TimeApi>")
            .field("log", &"<dyn LogApi>")
            .field("fs", &"<dyn FsApi>")
            .field("thread", &"<dyn ThreadApi>")
            .finish()
    }
}

/// Phase 112: Ring0 service registry
///
/// profile ごとに適切な FsApi 実装（等）を選択して Ring0Context を構築する factory。
pub struct Ring0Registry;

impl Ring0Registry {
    /// Ring0Context を profile に応じて構築
    pub fn build(profile: RuntimeProfile) -> Ring0Context {
        match profile {
            RuntimeProfile::Default => Self::build_with_fs(Arc::new(StdFs)),
            RuntimeProfile::NoFs => Self::build_with_fs(Arc::new(NoFsApi)),
        }
    }

    fn build_with_fs(fs: Arc<dyn FsApi>) -> Ring0Context {
        Ring0Context {
            mem: Arc::new(StdMem::new()),
            io: Arc::new(StdIo),
            time: Arc::new(StdTime),
            log: Arc::new(StdLog),
            fs,
            thread: Arc::new(StdThread),
        }
    }
}

/// Phase 88: デフォルト Ring0Context を作成
///
/// Phase 112 以降は、initialize_runtime() を通じて
/// Ring0Registry::build(profile) 経由で初期化されることが推奨。
///
/// この関数は直接呼び出しに対する互換性レイヤーとして保持。
pub fn default_ring0() -> Ring0Context {
    Ring0Registry::build(RuntimeProfile::Default)
}

// ===== グローバル Ring0Context =====

pub static GLOBAL_RING0: OnceLock<Arc<Ring0Context>> = OnceLock::new();

/// グローバル Ring0Context を初期化
pub fn init_global_ring0(ctx: Ring0Context) {
    GLOBAL_RING0
        .set(Arc::new(ctx))
        .expect("[Phase 88] Ring0Context already initialized");
}

/// グローバル Ring0Context を初期化済みで返す
///
/// まだ未初期化の場合は RuntimeProfile に基づいて構築し、OnceLock に登録する。
pub fn ensure_global_ring0_initialized() -> Arc<Ring0Context> {
    GLOBAL_RING0
        .get_or_init(|| {
            let profile = RuntimeProfile::from_env();
            Arc::new(Ring0Registry::build(profile))
        })
        .clone()
}

/// グローバル Ring0Context を取得
pub fn get_global_ring0() -> Arc<Ring0Context> {
    GLOBAL_RING0
        .get()
        .expect("[Phase 88] Ring0Context not initialized")
        .clone()
}

// ===== テスト =====

#[cfg(test)]
mod tests {
    use super::*;

    fn unsafe_dealloc(ptr: *mut u8, size: usize) {
        unsafe { std::alloc::dealloc(ptr, std::alloc::Layout::from_size_align_unchecked(size, 1)) }
    }

    #[test]
    fn test_ring0_context_creation() {
        let ring0 = default_ring0();
        ring0.log.info("test message");
    }

    #[test]
    fn test_io_api() {
        let ring0 = default_ring0();
        let result = ring0.io.stdout_write(b"test\n");
        assert!(result.is_ok());
    }

    #[test]
    fn test_time_api() {
        let ring0 = default_ring0();
        let now = ring0.time.now();
        assert!(now.is_ok());

        let instant = ring0.time.monotonic_now();
        assert!(instant.is_ok());
    }

    #[test]
    fn test_log_levels() {
        let ring0 = default_ring0();
        ring0.log.debug("debug message");
        ring0.log.info("info message");
        ring0.log.warn("warn message");
        ring0.log.error("error message");
    }

    #[test]
    fn test_default_ring0_uses_stdmem() {
        let ring0 = default_ring0();
        let ptr = ring0.mem.alloc(512);
        assert!(!ptr.is_null(), "default_ring0 should use StdMem");
        ring0.mem.free(ptr);

        // Clean up
        unsafe_dealloc(ptr, 512);
    }

    // Phase 112: Ring0Registry tests
    #[test]
    fn test_ring0_registry_default_profile() {
        let ctx = Ring0Registry::build(RuntimeProfile::Default);

        // Verify basic operations work
        ctx.log.info("Test message from Default profile");
        assert!(ctx.time.now().is_ok());
    }

    #[test]
    fn test_ring0_registry_nofs_profile() {
        use std::path::Path;

        let ctx = Ring0Registry::build(RuntimeProfile::NoFs);

        // Verify NoFsApi returns errors
        let result = ctx.fs.read_to_string(Path::new("/tmp/test.txt"));
        assert!(result.is_err());

        // Verify exists returns false
        assert!(!ctx.fs.exists(Path::new("/tmp/test.txt")));

        // Other services should still work
        ctx.log.info("Test message from NoFs profile");
        assert!(ctx.time.now().is_ok());
    }

    #[test]
    fn test_default_ring0_uses_registry() {
        let ctx = default_ring0();

        // Should behave same as Default profile
        ctx.log.info("Test from default_ring0()");
        assert!(ctx.time.now().is_ok());
    }

    #[test]
    fn test_nofs_api_read_to_string() {
        let api = NoFsApi;
        let result = api.read_to_string(std::path::Path::new("/tmp/test.txt"));
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("disabled"));
    }

    #[test]
    fn test_nofs_api_read() {
        let api = NoFsApi;
        assert!(api.read(std::path::Path::new("/tmp/test.txt")).is_err());
    }

    #[test]
    fn test_nofs_api_write_all() {
        let api = NoFsApi;
        assert!(api
            .write_all(std::path::Path::new("/tmp/test.txt"), b"data")
            .is_err());
    }

    #[test]
    fn test_nofs_api_append_all() {
        let api = NoFsApi;
        assert!(api
            .append_all(std::path::Path::new("/tmp/test.txt"), b"data")
            .is_err());
    }

    #[test]
    fn test_nofs_api_exists() {
        let api = NoFsApi;
        assert!(!api.exists(std::path::Path::new("/tmp/test.txt")));
    }

    #[test]
    fn test_nofs_api_metadata() {
        let api = NoFsApi;
        assert!(api.metadata(std::path::Path::new("/tmp/test.txt")).is_err());
    }

    #[test]
    fn test_nofs_api_canonicalize() {
        let api = NoFsApi;
        assert!(api
            .canonicalize(std::path::Path::new("/tmp/test.txt"))
            .is_err());
    }
}
