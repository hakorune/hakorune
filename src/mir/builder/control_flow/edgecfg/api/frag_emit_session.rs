//! FragEmitSession - Frag emit の手順 SSOT（Phase 29bq+）
//!
//! # 設計原則
//! - sealing は emit_frag 経路でのみ enforce
//! - from 側 block を自動収集（frag.wires/branches から）
//! - 層中立（plan 専用でない）
//!
//! # Sealing セマンティクス
//! - 未登録 = Open 扱い（register_open 不要）
//! - seal = insert(Sealed)
//! - 既 Sealed に emit しようとした → Err（`[freeze:contract]`）

use std::collections::{BTreeMap, BTreeSet};
use crate::mir::BasicBlockId;
use crate::mir::MirFunction;
use super::frag::Frag;
use super::emit::emit_frag;

/// Block の状態（Sealed のみ記録、未登録 = Open）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockState {
    Sealed,
}

/// Frag emit のセッション（per-function sealing）
///
/// # 設計原則
/// - 未登録 = Open（register_open 不要）
/// - Sealed のみ記録
/// - emit_and_seal() で sealing を enforce
#[derive(Debug, Default)]
pub struct FragEmitSession {
    /// Sealed な block のみ記録（未登録 = Open）
    sealed: BTreeMap<BasicBlockId, BlockState>,
}

impl FragEmitSession {
    /// 空のセッションを生成
    pub fn new() -> Self {
        Self {
            sealed: BTreeMap::new(),
        }
    }

    /// 関数開始時にリセット
    ///
    /// # 呼び出し点（SSOT）
    /// - lifecycle.rs:110 - prepare_module()
    /// - calls/lowering.rs:84 - create_function_skeleton()
    /// - builder_test_api.rs:5 - enter_function_for_test()
    pub fn reset(&mut self) {
        self.sealed.clear();
    }

    /// from 側 block を自動収集（BTreeSet で dedup + stable order）
    fn collect_from_blocks(frag: &Frag) -> BTreeSet<BasicBlockId> {
        let mut from_blocks = BTreeSet::new();
        for wire in &frag.wires {
            from_blocks.insert(wire.from);
        }
        for branch in &frag.branches {
            from_blocks.insert(branch.from);
        }
        from_blocks
    }

    /// Sealed チェック（Result を返す、debug_assert ではない）
    ///
    /// # 戻り値
    /// - `Ok(())`: Open（emit 可能）
    /// - `Err(String)`: Sealed（`[freeze:contract]` エラー）
    fn assert_open(&self, bb: BasicBlockId, context: &str) -> Result<(), String> {
        if self.sealed.contains_key(&bb) {
            Err(format!(
                "[freeze:contract] Cannot emit to sealed block {:?}: {}",
                bb, context
            ))
        } else {
            Ok(())
        }
    }

    /// Block を Sealed に変更
    fn seal(&mut self, bb: BasicBlockId) {
        self.sealed.insert(bb, BlockState::Sealed);
    }

    /// Frag を emit して、from 側 block を seal
    ///
    /// # 手順 SSOT
    /// 1. from 側 block を自動収集（BTreeSet で dedup）
    /// 2. 全 from block が Open であることを assert（Err で fail-fast）
    /// 3. emit_frag() 呼び出し
    /// 4. 成功後に from 側 block を seal
    ///
    /// # 戻り値
    /// - `Ok(())`: emit 成功、from block を seal 済み
    /// - `Err(String)`: assert_open 違反 or emit_frag 失敗
    pub fn emit_and_seal(
        &mut self,
        func: &mut MirFunction,
        frag: &Frag,
    ) -> Result<(), String> {
        let from_blocks = Self::collect_from_blocks(frag);

        // Step 1: assert_open before emit（Err で fail-fast）
        for from_bb in &from_blocks {
            self.assert_open(*from_bb, "emit_and_seal")?;
        }

        // Step 2: emit_frag()
        emit_frag(func, frag)?;

        // Step 3: seal after success
        for from_bb in &from_blocks {
            self.seal(*from_bb);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_session_is_empty() {
        let session = FragEmitSession::new();
        assert!(session.sealed.is_empty());
    }

    #[test]
    fn test_reset_clears_sealed() {
        let mut session = FragEmitSession::new();
        let bb = BasicBlockId(42);
        session.seal(bb);
        assert!(!session.sealed.is_empty());

        session.reset();
        assert!(session.sealed.is_empty());
    }

    #[test]
    fn test_assert_open_passes_for_unregistered() {
        let session = FragEmitSession::new();
        let bb = BasicBlockId(42);

        // 未登録 = Open
        let result = session.assert_open(bb, "test");
        assert!(result.is_ok());
    }

    #[test]
    fn test_double_emit_returns_err() {
        // 同じ from を 2 回 emit_and_seal したら Err
        // （panic 依存ではない - Result で fail-fast）
        let mut session = FragEmitSession::new();
        let bb = BasicBlockId(42);

        // 最初の seal
        session.seal(bb);

        // 2 回目は Err
        let result = session.assert_open(bb, "test");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            err.contains("[freeze:contract]"),
            "Error should contain '[freeze:contract]', got: {}",
            err
        );
    }

    #[test]
    fn test_collect_from_blocks_dedup() {
        use super::super::edge_stub::EdgeStub;
        use super::super::exit_kind::ExitKind;
        use super::super::branch_stub::BranchStub;
        use crate::mir::basic_block::EdgeArgs;
        use crate::mir::join_ir::lowering::inline_boundary::JumpArgsLayout;
        use crate::mir::ValueId;
        use std::collections::BTreeMap;

        let bb0 = BasicBlockId(0);
        let bb1 = BasicBlockId(1);
        let bb2 = BasicBlockId(2);

        // 同じ from (bb0) が wires と branches に複数回出現
        let frag = Frag {
            entry: bb0,
            block_params: BTreeMap::new(),
            exits: BTreeMap::new(),
            wires: vec![
                EdgeStub::new(
                    bb0,
                    ExitKind::Normal,
                    Some(bb1),
                    EdgeArgs {
                        layout: JumpArgsLayout::CarriersOnly,
                        values: vec![],
                    },
                ),
            ],
            branches: vec![
                BranchStub::new(
                    bb0, // 同じ from
                    ValueId(100),
                    bb1,
                    EdgeArgs {
                        layout: JumpArgsLayout::CarriersOnly,
                        values: vec![],
                    },
                    bb2,
                    EdgeArgs {
                        layout: JumpArgsLayout::CarriersOnly,
                        values: vec![],
                    },
                ),
            ],
        };

        let from_blocks = FragEmitSession::collect_from_blocks(&frag);

        // BTreeSet で dedup されるので 1 つだけ
        assert_eq!(from_blocks.len(), 1);
        assert!(from_blocks.contains(&bb0));
    }
}
