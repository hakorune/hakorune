//! Phase 33-7: If/Else の IfMerge 命令への lowering
//!
//! 複数変数を merge する if/else を JoinInst::IfMerge に変換する。
//!
//! Phase 33-7 制約:
//! - return パターンのみ（continuation は Phase 33-8）
//! - k_next=None のみ
//!
//! ## 責務分離（Phase 33-9.1）
//!
//! **IfMergeLowerer の責務**:
//! - 複数変数を持つ if/else を IfMerge 命令に変換する
//!
//! **非責務**:
//! - Loop の PHI には触らない（Loop lowering の責務）

use crate::mir::join_ir::{JoinInst, MergePair};
use crate::mir::{BasicBlockId, MirFunction, MirInstruction, ValueId};
use crate::runtime::get_global_ring0;
use std::collections::HashSet;

// Phase 61-1: If-in-loop context support
use super::if_phi_context::IfPhiContext;

pub struct IfMergeLowerer {
    debug_level: u8,
    // Phase 61-1: If-in-loop context (None = Pure If)
    #[allow(dead_code)]
    context: Option<IfPhiContext>,
}

/// 検出された IfMerge パターン情報
#[derive(Debug, Clone)]
struct IfMergePattern {
    cond: ValueId,
    merge_pairs: Vec<MergePair>,
}

/// Branch 命令の情報
#[derive(Debug, Clone)]
struct IfBranch {
    cond: ValueId,
    then_block: BasicBlockId,
    else_block: BasicBlockId,
}

impl IfMergeLowerer {
    pub fn new(debug_level: u8) -> Self {
        Self {
            debug_level,
            context: None, // Phase 61-1: デフォルトは Pure If
        }
    }

    /// Phase 33-8: debug-level backward compat wrapper
    pub fn with_debug(debug: bool) -> Self {
        Self {
            debug_level: if debug { 1 } else { 0 },
            context: None, // Phase 61-1: デフォルトは Pure If
        }
    }

    /// Phase 61-1: If-in-loop 用コンストラクタ
    ///
    /// # Arguments
    ///
    /// * `debug_level` - デバッグログレベル (0-3)
    /// * `context` - If-in-loop コンテキスト（carrier_names 情報を含む）
    ///
    /// # Example
    ///
    /// ```ignore
    /// let context = IfPhiContext::for_loop_body(carrier_names);
    /// let lowerer = IfMergeLowerer::with_context(debug_level, context);
    /// ```
    pub fn with_context(debug_level: u8, context: IfPhiContext) -> Self {
        Self {
            debug_level,
            context: Some(context),
        }
    }

    /// if/else が IfMerge に lowering できるかチェック
    pub fn can_lower_to_if_merge(&self, func: &MirFunction, if_block_id: BasicBlockId) -> bool {
        self.find_if_merge_pattern(func, if_block_id).is_some()
    }

    /// if/else を IfMerge に変換
    pub fn lower_if_to_if_merge(
        &self,
        func: &MirFunction,
        if_block_id: BasicBlockId,
    ) -> Option<JoinInst> {
        let pattern = self.find_if_merge_pattern(func, if_block_id)?;

        // Phase 33-8: Level 1 - Basic lowering info
        if self.debug_level >= 1 {
            get_global_ring0().log.debug(&format!(
                "[IfMergeLowerer] ✅ lowering to IfMerge with {} merge pairs",
                pattern.merge_pairs.len()
            ));
        }

        // Phase 33-8: Level 3 - Full merge details
        if self.debug_level >= 3 {
            get_global_ring0()
                .log
                .debug(&format!("[IfMergeLowerer] cond: {:?}", pattern.cond));
            for (i, pair) in pattern.merge_pairs.iter().enumerate() {
                get_global_ring0().log.debug(&format!(
                    "[IfMergeLowerer]   pair[{}]: dst={:?}, then={:?}, else={:?}",
                    i, pair.dst, pair.then_val, pair.else_val
                ));
            }
        }

        // IfMerge 命令を生成
        Some(JoinInst::IfMerge {
            cond: pattern.cond,
            merges: pattern.merge_pairs,
            k_next: None, // Phase 33-7 制約
        })
    }

    /// MIR 関数から IfMerge パターンを探す
    fn find_if_merge_pattern(
        &self,
        func: &MirFunction,
        block_id: BasicBlockId,
    ) -> Option<IfMergePattern> {
        // 1. Block が Branch 命令で終わっているか確認
        let block = func.blocks.get(&block_id)?;
        let branch = match block.terminator.as_ref()? {
            MirInstruction::Branch {
                condition,
                then_bb,
                else_bb,
                ..
            } => IfBranch {
                cond: *condition,
                then_block: *then_bb,
                else_block: *else_bb,
            },
            _ => return None,
        };

        // 2. then/else ブロックを取得
        let then_block = func.blocks.get(&branch.then_block)?;
        let else_block = func.blocks.get(&branch.else_block)?;

        // 3. Phase 33-7 制約: return パターンのみ
        //    両方のブロックが Return で終わる必要がある
        let is_then_return = matches!(
            then_block.terminator.as_ref(),
            Some(MirInstruction::Return { .. })
        );
        let is_else_return = matches!(
            else_block.terminator.as_ref(),
            Some(MirInstruction::Return { .. })
        );

        if !is_then_return || !is_else_return {
            // Phase 33-8: Level 2 - shape matching details
            if self.debug_level >= 2 {
                get_global_ring0().log.debug(&format!(
                    "[IfMergeLowerer] ❌ not return shape (then={}, else={})",
                    is_then_return, is_else_return
                ));
            }
            return None;
        }

        // 4. then/else で書き込まれる変数を抽出
        let then_writes = self.extract_written_vars(&then_block.instructions);
        let else_writes = self.extract_written_vars(&else_block.instructions);

        // Phase 33-8: Level 3 - Full variable dump
        if self.debug_level >= 3 {
            get_global_ring0().log.debug(&format!(
                "[IfMergeLowerer] then writes: {:?}, else writes: {:?}",
                then_writes, else_writes
            ));
        }

        // 5. 両方で書き込まれる変数（共通集合）を抽出
        let common_writes: HashSet<_> = then_writes.intersection(&else_writes).copied().collect();

        if common_writes.is_empty() {
            // Phase 33-8: Level 2 - shape matching details
            if self.debug_level >= 2 {
                get_global_ring0()
                    .log
                    .debug("[IfMergeLowerer] ❌ no common writes found");
            }
            return None;
        }

        // 6. 各共通変数について MergePair を生成
        let mut merge_pairs = Vec::new();

        for &dst in &common_writes {
            // then ブロックで dst に書き込まれる値を探す
            let then_val = self.find_written_value(&then_block.instructions, dst)?;
            // else ブロックで dst に書き込まれる値を探す
            let else_val = self.find_written_value(&else_block.instructions, dst)?;

            // Phase 64-2: then_val / else_val から型ヒント推論
            let type_hint = infer_type_from_mir_pattern(func, then_val)
                .or_else(|| infer_type_from_mir_pattern(func, else_val));

            merge_pairs.push(MergePair {
                dst,
                then_val,
                else_val,
                type_hint, // Phase 64-2: Const 命令から型推論
            });
        }

        if merge_pairs.is_empty() {
            return None;
        }

        // 7. MergePair を dst でソートして決定的に
        merge_pairs.sort_by_key(|pair| pair.dst.0);

        Some(IfMergePattern {
            cond: branch.cond,
            merge_pairs,
        })
    }

    /// 命令列から書き込まれる変数集合を抽出
    fn extract_written_vars(&self, instructions: &[MirInstruction]) -> HashSet<ValueId> {
        let mut writes = HashSet::new();

        for inst in instructions {
            match inst {
                MirInstruction::Copy { dst, .. }
                | MirInstruction::Const { dst, .. }
                | MirInstruction::BinOp { dst, .. }
                | MirInstruction::Compare { dst, .. } => {
                    writes.insert(*dst);
                }
                MirInstruction::Call { dst: Some(dst), .. } => {
                    writes.insert(*dst);
                }
                _ => {}
            }
        }

        writes
    }

    /// 命令列から dst に書き込まれる値を探す（最後の書き込み）
    fn find_written_value(&self, instructions: &[MirInstruction], dst: ValueId) -> Option<ValueId> {
        // 逆順で探索して最後の書き込みを見つける
        for inst in instructions.iter().rev() {
            match inst {
                MirInstruction::Copy { dst: inst_dst, src } if *inst_dst == dst => {
                    return Some(*src);
                }
                MirInstruction::Const { dst: inst_dst, .. }
                | MirInstruction::BinOp { dst: inst_dst, .. }
                | MirInstruction::Compare { dst: inst_dst, .. }
                | MirInstruction::Call {
                    dst: Some(inst_dst),
                    ..
                } if *inst_dst == dst => {
                    // dst 自身が書き込まれる場合は dst を返す
                    return Some(dst);
                }
                _ => {}
            }
        }

        None
    }
}

// Phase 185: infer_type_from_mir_pattern() moved to common.rs
use super::common::infer_type_from_mir_pattern;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_if_merge_lowerer_creation() {
        let lowerer = IfMergeLowerer::new(0);
        assert_eq!(lowerer.debug_level, 0);

        let lowerer_compat = IfMergeLowerer::with_debug(true);
        assert_eq!(lowerer_compat.debug_level, 1);
    }
}
