//! Phase 88: Ring0 エラー型定義

/// IO 操作エラー (Phase 90-A: fs 系エラー追加)
#[derive(Debug, Clone)]
pub enum IoError {
    /// ファイル読み込み失敗
    ReadFailed(String),
    /// ファイル書き込み失敗
    WriteFailed(String),
    /// メタデータ取得失敗
    MetadataFailed(String),
    /// 正規化失敗
    CanonicalizeFailed(String),
    /// stdin 読み込み失敗
    StdinReadFailed(String),
    /// stdout 書き込み失敗
    StdoutWriteFailed(String),
    /// stderr 書き込み失敗
    StderrWriteFailed(String),
    /// その他のエラー（Phase 88 互換用）
    Other(String),
}

impl std::fmt::Display for IoError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            IoError::ReadFailed(msg) => write!(f, "IoError (ReadFailed): {}", msg),
            IoError::WriteFailed(msg) => write!(f, "IoError (WriteFailed): {}", msg),
            IoError::MetadataFailed(msg) => write!(f, "IoError (MetadataFailed): {}", msg),
            IoError::CanonicalizeFailed(msg) => write!(f, "IoError (CanonicalizeFailed): {}", msg),
            IoError::StdinReadFailed(msg) => write!(f, "IoError (StdinReadFailed): {}", msg),
            IoError::StdoutWriteFailed(msg) => write!(f, "IoError (StdoutWriteFailed): {}", msg),
            IoError::StderrWriteFailed(msg) => write!(f, "IoError (StderrWriteFailed): {}", msg),
            IoError::Other(msg) => write!(f, "IoError: {}", msg),
        }
    }
}

impl std::error::Error for IoError {}

/// 時刻取得エラー
#[derive(Debug, Clone)]
pub struct TimeError(pub String);

impl std::fmt::Display for TimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "TimeError: {}", self.0)
    }
}

impl std::error::Error for TimeError {}
