//! Phase 40-1実験用メタデータ
//!
//! JoinIR本体（JoinFunction/JoinModule）には埋め込まず、
//! 実験パスでのみ横から渡す設計。
//!
//! # Phase 40拡張計画
//! - Phase 40-1: if_modified_vars（このPhase）
//! - Phase 40-2: conservative_vars（後で追加）
//! - Phase 40-3: reset_vars（後で追加）

use super::super::JoinFuncId;
use std::collections::{BTreeMap, HashSet};

/// Phase 40-1実験用メタデータ
#[derive(Debug, Default, Clone)]
pub struct JoinFuncMeta {
    /// Phase 40-1: if-in-loop modified variables
    /// loop body内のif文で代入される変数名
    pub if_modified_vars: Option<HashSet<String>>,
    // Phase 40-2: conservative_vars（後で追加）
    // Phase 40-3: reset_vars（後で追加）
}

/// JoinFuncId → JoinFuncMeta のマッピング
pub type JoinFuncMetaMap = BTreeMap<JoinFuncId, JoinFuncMeta>;
