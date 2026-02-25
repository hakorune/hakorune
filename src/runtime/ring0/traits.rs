//! Phase 88: Ring0 trait 定義
//!
//! OS API レイヤーの純粋な抽象化。
//! Box 名・Nyash 型を一切知らない。

use super::errors::{IoError, TimeError};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// メモリ API（Phase 88: noop、将来 hakmem 接続）
pub trait MemApi: Send + Sync {
    /// メモリ割り当て（Phase 88: 未実装）
    fn alloc(&self, size: usize) -> *mut u8;

    /// メモリ解放（Phase 88: 未実装）
    fn free(&self, ptr: *mut u8);

    /// メモリ統計（Phase 88: 未実装）
    fn stats(&self) -> MemStats;
}

/// メモリ統計情報
#[derive(Debug, Default, Clone)]
pub struct MemStats {
    pub allocated: usize,
    pub freed: usize,
    pub current: usize,
}

/// IO API
pub trait IoApi: Send + Sync {
    /// 標準出力への書き込み
    fn stdout_write(&self, data: &[u8]) -> Result<(), IoError>;

    /// 標準エラー出力への書き込み
    fn stderr_write(&self, data: &[u8]) -> Result<(), IoError>;

    /// 標準入力からの読み込み
    fn stdin_read(&self, buf: &mut [u8]) -> Result<usize, IoError>;
}

/// 時刻 API
pub trait TimeApi: Send + Sync {
    /// 現在時刻取得
    fn now(&self) -> Result<SystemTime, TimeError>;

    /// モノトニック時刻取得（高精度タイマー用）
    fn monotonic_now(&self) -> Result<std::time::Instant, TimeError>;

    /// 経過時間取得 (Phase 90-C)
    fn elapsed(&self, start: std::time::Instant) -> std::time::Duration;
}

/// ログレベル
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

/// ログ API
pub trait LogApi: Send + Sync {
    /// ログ出力
    fn log(&self, level: LogLevel, msg: &str);

    /// デバッグログ（便利メソッド）
    fn debug(&self, msg: &str) {
        self.log(LogLevel::Debug, msg);
    }

    /// 情報ログ（便利メソッド）
    fn info(&self, msg: &str) {
        self.log(LogLevel::Info, msg);
    }

    /// 警告ログ（便利メソッド）
    fn warn(&self, msg: &str) {
        self.log(LogLevel::Warn, msg);
    }

    /// エラーログ（便利メソッド）
    fn error(&self, msg: &str) {
        self.log(LogLevel::Error, msg);
    }
}

/// ファイルシステムメタデータ
#[derive(Debug, Clone)]
pub struct FsMetadata {
    pub is_file: bool,
    pub is_dir: bool,
    pub len: u64,
}

/// ファイルシステム API (Phase 90-A)
pub trait FsApi: Send + Sync {
    /// ファイルを文字列として読み込む
    fn read_to_string(&self, path: &Path) -> Result<String, IoError>;

    /// ファイルをバイト列として読み込む
    fn read(&self, path: &Path) -> Result<Vec<u8>, IoError>;

    /// ファイルに書き込む
    fn write_all(&self, path: &Path, data: &[u8]) -> Result<(), IoError>;

    /// ファイルに追記（append）
    ///
    /// ファイルが存在しない場合は新規作成、存在する場合は末尾に追記。
    /// Phase 111: write_all と対称的に提供。
    fn append_all(&self, path: &Path, data: &[u8]) -> Result<(), IoError>;

    /// パスが存在するか確認
    fn exists(&self, path: &Path) -> bool;

    /// ファイルメタデータを取得
    fn metadata(&self, path: &Path) -> Result<FsMetadata, IoError>;

    /// パスを正規化
    fn canonicalize(&self, path: &Path) -> Result<PathBuf, IoError>;
}

/// スレッド API (Phase 90-D)
pub trait ThreadApi: Send + Sync {
    /// 指定時間スリープ
    fn sleep(&self, duration: std::time::Duration);
    // spawn は Phase 91 以降で追加予定
}
