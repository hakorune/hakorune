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
//! - PlanBuildSession は JoinKey 管理のみ独自実装

use crate::mir::builder::control_flow::edgecfg::api::{Frag, FragEmitSession};
use crate::mir::BasicBlockId;
use crate::mir::MirFunction;

use super::join_key::{JoinKey, JoinRegistry};

/// Plan lowering の 1 回の呼び出しに対応するセッション
///
/// # 設計原則
/// - clone_plans_with_fresh_loops と独立（跨いで共有しない）
/// - session-local な JoinKey ID を発行
/// - emit_and_seal() は FragEmitSession へ delegation
#[derive(Debug)]
pub struct PlanBuildSession {
    /// sealing は FragEmitSession へ delegation
    session: FragEmitSession,
    join_registry: JoinRegistry,
    next_if_id: u32,
    next_loop_id: u32,
}

impl PlanBuildSession {
    /// 新しいセッションを生成
    pub fn new() -> Self {
        Self {
            session: FragEmitSession::new(),
            join_registry: JoinRegistry::new(),
            next_if_id: 0,
            next_loop_id: 0,
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

    // ========================================================================
    // JoinKey 管理（副産物）
    // ========================================================================

    /// if-else の JoinKey を発行（session-local）
    pub fn alloc_if_join(&mut self) -> JoinKey {
        let id = self.next_if_id;
        self.next_if_id += 1;
        JoinKey::IfJoin { id }
    }

    /// loop の JoinKey ペアを発行（session-local）
    ///
    /// # 戻り値
    /// - `(LoopHeader, LoopAfter)` のペア
    pub fn alloc_loop(&mut self) -> (JoinKey, JoinKey) {
        let id = self.next_loop_id;
        self.next_loop_id += 1;
        (JoinKey::LoopHeader { id }, JoinKey::LoopAfter { id })
    }

    /// JoinKey で登録済みの block を取得（再利用用）
    pub fn get_join(&self, key: JoinKey) -> Option<BasicBlockId> {
        self.join_registry.get(key)
    }

    /// JoinKey と block を関連付け（登録）
    pub fn register_join(&mut self, key: JoinKey, bb: BasicBlockId) {
        self.join_registry.register(key, bb);
    }
}

impl Default for PlanBuildSession {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alloc_if_join() {
        let mut session = PlanBuildSession::new();
        let key1 = session.alloc_if_join();
        let key2 = session.alloc_if_join();

        assert_eq!(key1, JoinKey::IfJoin { id: 0 });
        assert_eq!(key2, JoinKey::IfJoin { id: 1 });
    }

    #[test]
    fn test_alloc_loop() {
        let mut session = PlanBuildSession::new();
        let (header1, after1) = session.alloc_loop();
        let (header2, after2) = session.alloc_loop();

        assert_eq!(header1, JoinKey::LoopHeader { id: 0 });
        assert_eq!(after1, JoinKey::LoopAfter { id: 0 });
        assert_eq!(header2, JoinKey::LoopHeader { id: 1 });
        assert_eq!(after2, JoinKey::LoopAfter { id: 1 });
    }

    #[test]
    fn test_join_registry() {
        let mut session = PlanBuildSession::new();
        let key = session.alloc_if_join();
        let bb = BasicBlockId(42);

        assert_eq!(session.get_join(key), None);
        session.register_join(key, bb);
        assert_eq!(session.get_join(key), Some(bb));
    }
}
