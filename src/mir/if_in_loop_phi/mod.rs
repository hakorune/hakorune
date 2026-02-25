//! Phase 61-3: If-in-loop PHI Emitter
//!
//! ループ内 if の PHI 生成を担当する「箱」。
//! JoinIR + PhiSpec から直接 PHI 命令を発行する。
//!
//! ## 箱理論における位置づけ
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────┐
//! │  if_lowering.rs（オーケストレーター）                       │
//! │    ↓                                                        │
//! │  ┌─────────────────────┐  ┌─────────────────────────────┐  │
//! │  │ JoinIR Lowering     │  │ IfInLoopPhiEmitter          │  │
//! │  │ (PhiSpec生成)       │→ │ (PHI命令発行)               │  │
//! │  └─────────────────────┘  └─────────────────────────────┘  │
//! └─────────────────────────────────────────────────────────────┘
//! ```
//!
//! ## 責務
//!
//! - Header PHI 生成（ループイテレーション継続用）
//! - VarLookup 方式による incoming 値解決
//! - PhiBuilderOps trait 経由での PHI 発行
//!
//! ## 設計原則
//!
//! - **Thin Box**: ロジックは最小限、組み立てに専念
//! - **CFG非依存**: incoming値は snapshot から直接取得
//! - **決定性保証**: BTreeSet/BTreeMap のみ使用

use crate::mir::control_form::IfShape;
use crate::mir::join_ir::lowering::if_phi_spec::PhiSpec;
use crate::mir::phi_core::phi_builder_box::PhiBuilderOps;
use crate::mir::ValueId;
use std::collections::{BTreeMap, BTreeSet};

/// If-in-loop PHI Emitter
///
/// JoinIR経路で生成された PhiSpec を元に、
/// 実際の PHI 命令を発行する。
pub struct IfInLoopPhiEmitter;

impl IfInLoopPhiEmitter {
    /// Header PHI 生成
    ///
    /// ループキャリア変数に対して、then/else 腕の値を統合する PHI を生成する。
    ///
    /// # Arguments
    ///
    /// * `phi_spec` - JoinIR から計算された PHI 仕様
    /// * `pre_if_var_map` - if 直前の変数マップ（VarLookup）
    /// * `then_snapshot` - then 腕終了時の変数スナップショット
    /// * `else_snapshot_opt` - else 腕終了時の変数スナップショット（ない場合は None）
    /// * `carrier_names` - ループキャリア変数名（片腕 PHI 対象）
    /// * `ops` - PHI 生成操作インターフェース
    /// * `if_shape` - If 構造情報
    ///
    /// # Returns
    ///
    /// 生成した PHI の数
    pub fn emit_header_phis<O: PhiBuilderOps>(
        phi_spec: &PhiSpec,
        pre_if_var_map: &BTreeMap<String, ValueId>,
        then_snapshot: &BTreeMap<String, ValueId>,
        else_snapshot_opt: Option<&BTreeMap<String, ValueId>>,
        carrier_names: &BTreeSet<String>,
        ops: &mut O,
        if_shape: &IfShape,
    ) -> Result<usize, String> {
        let mut phi_count = 0;
        let trace_on = crate::config::env::joinir_dev_enabled()
            && crate::config::env::joinir_dev::if_in_loop_trace_enabled();

        if trace_on {
            crate::runtime::get_global_ring0()
                .log
                .debug("[Phase 61-3] IfInLoopPhiEmitter::emit_header_phis start");
            crate::runtime::get_global_ring0().log.debug(&format!(
                "[Phase 61-3]   header_phis: {:?}",
                phi_spec.header_phis
            ));
            crate::runtime::get_global_ring0().log.debug(&format!(
                "[Phase 61-3]   carrier_names: {:?}",
                carrier_names
            ));
        }

        // Header PHI 生成: carrier_names に含まれる変数のみ
        for var_name in &phi_spec.header_phis {
            if !carrier_names.contains(var_name) {
                continue;
            }

            // VarLookup: pre_if値を取得
            let pre_val = match pre_if_var_map.get(var_name) {
                Some(&v) => v,
                None => {
                    if trace_on {
                        crate::runtime::get_global_ring0().log.debug(&format!(
                            "[Phase 61-3]   ⚠️ var={} not found in pre_if_var_map, skipping",
                            var_name
                        ));
                    }
                    continue;
                }
            };

            // Then値: snapshot から取得、なければ pre_val
            let then_val = then_snapshot.get(var_name).copied().unwrap_or(pre_val);

            // Else値: snapshot から取得、なければ pre_val（片腕 PHI パターン）
            let else_val = else_snapshot_opt
                .and_then(|m| m.get(var_name).copied())
                .unwrap_or(pre_val);

            // 値が同一なら PHI 不要
            if then_val == else_val {
                if trace_on {
                    crate::runtime::get_global_ring0().log.debug(&format!(
                        "[Phase 61-3]   var={}: then_val == else_val ({:?}), no PHI needed",
                        var_name, then_val
                    ));
                }
                ops.update_var(var_name.clone(), then_val);
                continue;
            }

            // PHI 生成
            let phi_dst = ops.new_value();
            let mut inputs = Vec::new();

            // Then 入力
            inputs.push((if_shape.then_block, then_val));

            // Else 入力
            if let Some(else_bb) = if_shape.else_block {
                inputs.push((else_bb, else_val));
            }

            if trace_on {
                crate::runtime::get_global_ring0().log.debug(&format!(
                    "[Phase 61-3]   var={}: emit PHI dst={:?} inputs={:?}",
                    var_name, phi_dst, inputs
                ));
            }

            ops.emit_phi(if_shape.merge_block, phi_dst, inputs)?;
            ops.update_var(var_name.clone(), phi_dst);
            phi_count += 1;
        }

        if trace_on {
            crate::runtime::get_global_ring0().log.debug(&format!(
                "[Phase 61-3] IfInLoopPhiEmitter::emit_header_phis done: {} PHIs",
                phi_count
            ));
        }

        Ok(phi_count)
    }

    /// Phase 61-4: ループ外 If PHI 生成（Toplevel If）
    ///
    /// ループ外の純粋な if に対して PHI を生成する。
    /// carrier_names が空のため、PhiSpec の全変数を対象とする。
    ///
    /// # Arguments
    ///
    /// * `phi_spec` - JoinIR から計算された PHI 仕様
    /// * `pre_if_var_map` - if 直前の変数マップ
    /// * `then_snapshot` - then 腕終了時の変数スナップショット
    /// * `else_snapshot_opt` - else 腕終了時の変数スナップショット
    /// * `ops` - PHI 生成操作インターフェース
    /// * `if_shape` - If 構造情報
    ///
    /// # Returns
    ///
    /// 生成した PHI の数
    pub fn emit_toplevel_phis<O: PhiBuilderOps>(
        phi_spec: &PhiSpec,
        pre_if_var_map: &BTreeMap<String, ValueId>,
        then_snapshot: &BTreeMap<String, ValueId>,
        else_snapshot_opt: Option<&BTreeMap<String, ValueId>>,
        ops: &mut O,
        if_shape: &IfShape,
    ) -> Result<usize, String> {
        let mut phi_count = 0;
        let trace_on = crate::config::env::joinir_dev_enabled()
            && crate::config::env::joinir_dev::if_toplevel_trace_enabled();

        if trace_on {
            crate::runtime::get_global_ring0()
                .log
                .debug("[Phase 61-4] IfInLoopPhiEmitter::emit_toplevel_phis start");
            crate::runtime::get_global_ring0().log.debug(&format!(
                "[Phase 61-4]   header_phis: {:?}",
                phi_spec.header_phis
            ));
        }

        // Toplevel PHI 生成: PhiSpec の全変数を対象
        for var_name in &phi_spec.header_phis {
            // pre_if値を取得
            let pre_val = match pre_if_var_map.get(var_name) {
                Some(&v) => v,
                None => {
                    if trace_on {
                        crate::runtime::get_global_ring0().log.debug(&format!(
                            "[Phase 61-4]   ⚠️ var={} not found in pre_if_var_map, skipping",
                            var_name
                        ));
                    }
                    continue;
                }
            };

            // Then値: snapshot から取得、なければ pre_val
            let then_val = then_snapshot.get(var_name).copied().unwrap_or(pre_val);

            // Else値: snapshot から取得、なければ pre_val
            let else_val = else_snapshot_opt
                .and_then(|m| m.get(var_name).copied())
                .unwrap_or(pre_val);

            // 値が同一なら PHI 不要
            if then_val == else_val {
                if trace_on {
                    crate::runtime::get_global_ring0().log.debug(&format!(
                        "[Phase 61-4]   var={}: then_val == else_val ({:?}), no PHI needed",
                        var_name, then_val
                    ));
                }
                ops.update_var(var_name.clone(), then_val);
                continue;
            }

            // PHI 生成
            let phi_dst = ops.new_value();
            let mut inputs = Vec::new();

            // Then 入力
            inputs.push((if_shape.then_block, then_val));

            // Else 入力
            if let Some(else_bb) = if_shape.else_block {
                inputs.push((else_bb, else_val));
            }

            if trace_on {
                crate::runtime::get_global_ring0().log.debug(&format!(
                    "[Phase 61-4]   var={}: emit PHI dst={:?} inputs={:?}",
                    var_name, phi_dst, inputs
                ));
            }

            ops.emit_phi(if_shape.merge_block, phi_dst, inputs)?;
            ops.update_var(var_name.clone(), phi_dst);
            phi_count += 1;
        }

        if trace_on {
            crate::runtime::get_global_ring0().log.debug(&format!(
                "[Phase 61-4] IfInLoopPhiEmitter::emit_toplevel_phis done: {} PHIs",
                phi_count
            ));
        }

        Ok(phi_count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::BasicBlockId;

    /// モック PhiBuilderOps 実装
    struct MockOps {
        next_id: u32,
        emitted_phis: Vec<(BasicBlockId, ValueId, Vec<(BasicBlockId, ValueId)>)>,
        var_updates: Vec<(String, ValueId)>,
    }

    impl MockOps {
        fn new() -> Self {
            Self {
                next_id: 1000,
                emitted_phis: Vec::new(),
                var_updates: Vec::new(),
            }
        }
    }

    impl PhiBuilderOps for MockOps {
        fn new_value(&mut self) -> ValueId {
            let id = self.next_id;
            self.next_id += 1;
            ValueId(id)
        }

        fn emit_phi(
            &mut self,
            block: BasicBlockId,
            dst: ValueId,
            inputs: Vec<(BasicBlockId, ValueId)>,
        ) -> Result<(), String> {
            self.emitted_phis.push((block, dst, inputs));
            Ok(())
        }

        fn update_var(&mut self, name: String, value: ValueId) {
            self.var_updates.push((name, value));
        }

        fn get_block_predecessors(&self, _block: BasicBlockId) -> Vec<BasicBlockId> {
            Vec::new()
        }

        fn emit_void(&mut self) -> Result<ValueId, String> {
            Ok(self.new_value())
        }

        fn set_current_block(&mut self, _block: BasicBlockId) -> Result<(), String> {
            Ok(())
        }

        fn block_exists(&self, _block: BasicBlockId) -> bool {
            true
        }
    }

    #[test]
    fn test_emit_header_phis_basic() {
        // Setup
        let mut phi_spec = PhiSpec::new();
        phi_spec.header_phis.insert("x".to_string());

        let mut pre_if = BTreeMap::new();
        pre_if.insert("x".to_string(), ValueId(1));

        let mut then_snap = BTreeMap::new();
        then_snap.insert("x".to_string(), ValueId(2)); // then で変更

        let else_snap = BTreeMap::new(); // else では変更なし

        let mut carriers = BTreeSet::new();
        carriers.insert("x".to_string());

        let if_shape = IfShape {
            cond_block: BasicBlockId(10),
            then_block: BasicBlockId(11),
            else_block: Some(BasicBlockId(12)),
            merge_block: BasicBlockId(13),
        };

        let mut ops = MockOps::new();

        // Execute
        let count = IfInLoopPhiEmitter::emit_header_phis(
            &phi_spec,
            &pre_if,
            &then_snap,
            Some(&else_snap),
            &carriers,
            &mut ops,
            &if_shape,
        )
        .unwrap();

        // Verify
        assert_eq!(count, 1);
        assert_eq!(ops.emitted_phis.len(), 1);

        let (block, _dst, inputs) = &ops.emitted_phis[0];
        assert_eq!(*block, BasicBlockId(13)); // merge_block
        assert_eq!(inputs.len(), 2);
        assert_eq!(inputs[0], (BasicBlockId(11), ValueId(2))); // then: 変更値
        assert_eq!(inputs[1], (BasicBlockId(12), ValueId(1))); // else: pre_val
    }

    #[test]
    fn test_emit_header_phis_same_value() {
        // 両腕で同じ値の場合、PHI不要
        let mut phi_spec = PhiSpec::new();
        phi_spec.header_phis.insert("x".to_string());

        let mut pre_if = BTreeMap::new();
        pre_if.insert("x".to_string(), ValueId(1));

        let mut then_snap = BTreeMap::new();
        then_snap.insert("x".to_string(), ValueId(1)); // 変更なし

        let else_snap = BTreeMap::new();

        let mut carriers = BTreeSet::new();
        carriers.insert("x".to_string());

        let if_shape = IfShape {
            cond_block: BasicBlockId(10),
            then_block: BasicBlockId(11),
            else_block: Some(BasicBlockId(12)),
            merge_block: BasicBlockId(13),
        };

        let mut ops = MockOps::new();

        let count = IfInLoopPhiEmitter::emit_header_phis(
            &phi_spec,
            &pre_if,
            &then_snap,
            Some(&else_snap),
            &carriers,
            &mut ops,
            &if_shape,
        )
        .unwrap();

        assert_eq!(count, 0); // PHI不要
        assert_eq!(ops.emitted_phis.len(), 0);
        assert_eq!(ops.var_updates.len(), 1);
        assert_eq!(ops.var_updates[0], ("x".to_string(), ValueId(1)));
    }
}
