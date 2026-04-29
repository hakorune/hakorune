//! PlanBuildSession - Plan lowering の手順 SSOT（Phase 29bq+）
//!
//! # 目的
//! - 「作成手順 SSOT」を提供する session-local コンテキスト
//! - clone_plans_with_fresh_loops と独立（跨いで共有しない）
//!
//! # 手順 SSOT のフロー
//! ```text
//! Frag 構築（compose::loop_ など）           // wires/branches 設定
//!     ↓
//! session.emit_and_seal(func, &frag)         // from 自動収集 → assert_open → emit → seal
//! ```
//!
//! # 設計（Phase 29bq+ 骨格拡大後）
//! - sealing は FragEmitSession へ delegation

use crate::mir::builder::control_flow::edgecfg::api::{Frag, FragEmitSession};
use crate::mir::MirFunction;

/// Plan lowering の 1 回の呼び出しに対応するセッション
///
/// # 設計原則
/// - clone_plans_with_fresh_loops と独立（跨いで共有しない）
/// - emit_and_seal() は FragEmitSession へ delegation
#[derive(Debug)]
pub struct PlanBuildSession {
    /// sealing は FragEmitSession へ delegation
    session: FragEmitSession,
}

impl PlanBuildSession {
    /// 新しいセッションを生成
    pub fn new() -> Self {
        Self {
            session: FragEmitSession::new(),
        }
    }

    // ========================================================================
    // Sealing enforce SSOT（FragEmitSession へ delegation）
    // ========================================================================

    /// Frag を emit して、from 側 block を seal
    ///
    /// # sealing enforce の SSOT
    /// FragEmitSession へ delegation。詳細は FragEmitSession::emit_and_seal() を参照。
    ///
    /// # 引数
    /// - `func`: MIR function
    /// - `frag`: 配線済み Frag
    ///
    /// # 戻り値
    /// - `Ok(())`: emit 成功、from block を seal 済み
    /// - `Err(String)`: assert_open 違反 or emit_frag 失敗
    pub fn emit_and_seal(&mut self, func: &mut MirFunction, frag: &Frag) -> Result<(), String> {
        self.session.emit_and_seal(func, frag)
    }
}

impl Default for PlanBuildSession {
    fn default() -> Self {
        Self::new()
    }
}
