//! Phase 33: If/Else の Select 命令への lowering
//!
//! 最小の if/else（副作用なし、単純な値選択）を JoinInst::Select に変換する。
//!
//! ## 責務分離（Phase 33-9.1）
//!
//! **IfSelectLowerer の責務**:
//! - 単純 if/else（副作用なし、単一変数）を Select 命令に変換する
//!
//! **非責務**:
//! - Loop の PHI には触らない（Loop lowering の責務）

use crate::mir::join_ir::JoinInst;
use crate::mir::{BasicBlockId, MirFunction, MirInstruction, ValueId};
use crate::runtime::get_global_ring0;
// Phase 63-2: Type hint inference from MIR

// Phase 61-1: If-in-loop context support
use super::if_phi_context::IfPhiContext;

pub struct IfSelectLowerer {
    debug_level: u8,
    // Phase 61-1: If-in-loop context (None = Pure If)
    #[allow(dead_code)]
    context: Option<IfPhiContext>,
}

/// If/Else パターンの分類
#[derive(Debug, Clone, Copy)]
enum IfPatternType {
    /// Simple pattern: if cond { return 1 } else { return 2 }
    Simple,
    /// Local pattern: if cond { x = a } else { x = b }; return x
    Local,
}

/// 検出された If/Else パターン情報
#[derive(Debug, Clone)]
struct IfPattern {
    pattern_type: IfPatternType,
    cond: ValueId,
    then_val: ValueId,
    else_val: ValueId,
    dst: Option<ValueId>,
}

/// Branch 命令の情報
#[derive(Debug, Clone)]
struct IfBranch {
    cond: ValueId,
    then_block: BasicBlockId,
    else_block: BasicBlockId,
}

impl IfSelectLowerer {
    pub fn new(debug_level: u8) -> Self {
        Self {
            debug_level,
            context: None, // Phase 61-1: デフォルトは Pure If
        }
    }

    /// Phase 33-8: debug-level backward compat wrapper
    #[allow(dead_code)]
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
    /// let lowerer = IfSelectLowerer::with_context(debug_level, context);
    /// ```
    pub fn with_context(debug_level: u8, context: IfPhiContext) -> Self {
        Self {
            debug_level,
            context: Some(context),
        }
    }

    /// if/else が Select に lowering できるかチェック
    pub fn can_lower_to_select(&self, func: &MirFunction, if_block_id: BasicBlockId) -> bool {
        self.find_if_pattern(func, if_block_id).is_some()
    }

    /// if/else を Select に変換
    pub fn lower_if_to_select(
        &self,
        func: &MirFunction,
        if_block_id: BasicBlockId,
    ) -> Option<JoinInst> {
        let pattern = self.find_if_pattern(func, if_block_id)?;

        // Phase 33-8: Level 1 - Basic lowering info
        if self.debug_level >= 1 {
            get_global_ring0().log.debug(&format!(
                "[IfSelectLowerer] ✅ lowering {:?} pattern to Select",
                pattern.pattern_type
            ));
        }

        // Phase 33-8: Level 3 - Full pattern details
        if self.debug_level >= 3 {
            let ring0 = get_global_ring0();
            ring0
                .log
                .debug(&format!("[IfSelectLowerer] cond: {:?}", pattern.cond));
            ring0
                .log
                .debug(&format!("[IfSelectLowerer] then_val: {:?}", pattern.then_val));
            ring0
                .log
                .debug(&format!("[IfSelectLowerer] else_val: {:?}", pattern.else_val));
            ring0
                .log
                .debug(&format!("[IfSelectLowerer] dst: {:?}", pattern.dst));
        }

        // Select 命令を生成
        let dst = pattern.dst.unwrap_or(pattern.then_val);

        // Phase 63-2: MIR の Const 命令から型ヒントを推論
        let type_hint = infer_type_from_mir_pattern(func, pattern.then_val)
            .or_else(|| infer_type_from_mir_pattern(func, pattern.else_val));

        Some(JoinInst::Select {
            dst,
            cond: pattern.cond,
            then_val: pattern.then_val,
            else_val: pattern.else_val,
            type_hint, // Phase 63-2: Const 命令から推論した型（Integer/Bool/String など）
        })
    }

    /// MIR 関数から if/else パターンを探す
    fn find_if_pattern(&self, func: &MirFunction, block_id: BasicBlockId) -> Option<IfPattern> {
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

        // 2. then/else ブロックの構造を確認
        let then_block = func.blocks.get(&branch.then_block)?;
        let else_block = func.blocks.get(&branch.else_block)?;

        // Phase 33-10: PHI早期チェック（パターンマッチング前）
        // JoinIRは「PHI生成器」であり「PHI変換器」ではない
        // then/elseがJumpで終わる場合、merge blockにPHI命令があるか早期確認
        if let Some(merge_block_id) =
            self.get_merge_block_if_jump_pattern(&branch, then_block, else_block)
        {
            let merge_block = func.blocks.get(&merge_block_id)?;
            if merge_block
                .instructions
                .iter()
                .any(|inst| matches!(inst, MirInstruction::Phi { .. }))
            {
                if self.debug_level >= 2 {
                    get_global_ring0().log.debug(
                        "[IfSelectLowerer] ⏭️ PHI already exists in merge block, skipping",
                    );
                }
                return None;
            }
        }

        // 3. simple パターンのチェック
        if let Some(pattern) = self.try_match_simple_pattern(&branch, then_block, else_block) {
            // Phase 33-8: Level 2 - Pattern matching details
            if self.debug_level >= 2 {
                get_global_ring0()
                    .log
                    .debug("[IfSelectLowerer] ✅ matched simple pattern");
            }
            return Some(pattern);
        }

        // 4. local パターンのチェック
        if let Some(pattern) = self.try_match_local_pattern(func, &branch, then_block, else_block) {
            // Phase 33-8: Level 2 - Pattern matching details
            if self.debug_level >= 2 {
                get_global_ring0()
                    .log
                    .debug("[IfSelectLowerer] ✅ matched local pattern");
            }
            return Some(pattern);
        }

        // Phase 33-8: Level 2 - Pattern matching details
        if self.debug_level >= 2 {
            get_global_ring0()
                .log
                .debug("[IfSelectLowerer] ❌ no pattern matched");
        }
        None
    }

    /// simple パターン: if cond { return 1 } else { return 2 }
    ///
    /// Phase 33-9.2: 実用MIR対応 - 副作用なし命令（Const/Copy）を許容
    /// - 旧: Return のみ（instructions empty）
    /// - 新: Const/Copy → Return を許容（実MIRパターン）
    fn try_match_simple_pattern(
        &self,
        branch: &IfBranch,
        then_block: &crate::mir::BasicBlock,
        else_block: &crate::mir::BasicBlock,
    ) -> Option<IfPattern> {
        // then ブロックが Return だけか確認
        let then_val = match then_block.terminator.as_ref()? {
            MirInstruction::Return { value: Some(v) } => *v,
            _ => return None,
        };

        // else ブロックが Return だけか確認
        let else_val = match else_block.terminator.as_ref()? {
            MirInstruction::Return { value: Some(v) } => *v,
            _ => return None,
        };

        // Phase 33-9.2: 副作用なし命令（Const/Copy）のみを許容
        // - ユニットテスト（empty）も通過（空配列 → all() = true）
        // - 実用MIR（const + ret）も通過
        if !self.is_side_effect_free(&then_block.instructions)
            || !self.is_side_effect_free(&else_block.instructions)
        {
            return None;
        }

        Some(IfPattern {
            pattern_type: IfPatternType::Simple,
            cond: branch.cond,
            then_val,
            else_val,
            dst: None,
        })
    }

    /// Phase 33-9.2: 副作用なし命令判定ヘルパー
    ///
    /// Const/Copy のみを許容（分岐・call・書き込み等は除外）
    fn is_side_effect_free(&self, instructions: &[MirInstruction]) -> bool {
        instructions.iter().all(|inst| {
            matches!(
                inst,
                MirInstruction::Const { .. } | MirInstruction::Copy { .. }
            )
        })
    }

    /// Phase 33-10: Jump pattern 検出ヘルパー
    ///
    /// then/else 両方が Jump で終わり、同じ merge block に飛んでいる場合、
    /// その merge block IDを返す
    fn get_merge_block_if_jump_pattern(
        &self,
        _branch: &IfBranch,
        then_block: &crate::mir::BasicBlock,
        else_block: &crate::mir::BasicBlock,
    ) -> Option<BasicBlockId> {
        // then が Jump で終わるか確認
        let then_target = match then_block.terminator.as_ref()? {
            MirInstruction::Jump { target, .. } => *target,
            _ => return None,
        };

        // else が Jump で終わるか確認
        let else_target = match else_block.terminator.as_ref()? {
            MirInstruction::Jump { target, .. } => *target,
            _ => return None,
        };

        // 両方が同じ merge block に飛んでいるか確認
        if then_target == else_target {
            Some(then_target)
        } else {
            None
        }
    }

    /// local パターン: if cond { x = a } else { x = b }; return x
    ///
    /// Phase 33-10: 実用MIR対応 - Const命令を許容
    /// - 旧: Copy命令のみ（ユニットテスト想定）
    /// - 新: Const/Copy命令を許容（実MIR対応、Simple patternと同じ修正）
    fn try_match_local_pattern(
        &self,
        func: &MirFunction,
        branch: &IfBranch,
        then_block: &crate::mir::BasicBlock,
        else_block: &crate::mir::BasicBlock,
    ) -> Option<IfPattern> {
        // Phase 33-10: 副作用なし命令のみを許容
        if !self.is_side_effect_free(&then_block.instructions) {
            return None;
        }

        // then ブロックの最後の値を取得
        // Phase 33-10: Const命令も許容（実MIR対応）
        let (dst_then, val_then) = if then_block.instructions.len() == 1 {
            match &then_block.instructions[0] {
                MirInstruction::Copy { dst, src } => (*dst, *src),
                MirInstruction::Const { dst, .. } => (*dst, *dst), // Constの場合、dst自身が値
                _ => return None,
            }
        } else {
            return None;
        };

        // then ブロックが Jump で終わるか確認
        let merge_block_id = match then_block.terminator.as_ref()? {
            MirInstruction::Jump { target, .. } => *target,
            _ => return None,
        };

        // Phase 33-10: else ブロックも副作用なし命令のみを許容
        if !self.is_side_effect_free(&else_block.instructions) {
            return None;
        }

        // else ブロックの最後の値を取得
        let (dst_else, val_else) = if else_block.instructions.len() == 1 {
            match &else_block.instructions[0] {
                MirInstruction::Copy { dst, src } => (*dst, *src),
                MirInstruction::Const { dst, .. } => (*dst, *dst), // Constの場合、dst自身が値
                _ => return None,
            }
        } else {
            return None;
        };

        // 代入先が同じ変数か確認
        if dst_then != dst_else {
            return None;
        }

        // else ブロックも同じ merge ブロックに Jump するか確認
        let else_merge = match else_block.terminator.as_ref()? {
            MirInstruction::Jump { target, .. } => *target,
            _ => return None,
        };

        if merge_block_id != else_merge {
            return None;
        }

        // merge ブロックが「return dst」だけか確認
        let merge_block = func.blocks.get(&merge_block_id)?;
        // Phase 33-10: PHIチェックは find_if_pattern() で早期実行済み

        match merge_block.terminator.as_ref()? {
            MirInstruction::Return { value: Some(v) } if *v == dst_then => {
                // OK
            }
            _ => return None,
        }

        if !merge_block.instructions.is_empty() {
            return None;
        }

        Some(IfPattern {
            pattern_type: IfPatternType::Local,
            cond: branch.cond,
            then_val: val_then,
            else_val: val_else,
            dst: Some(dst_then),
        })
    }
}

// Phase 185: infer_type_from_mir_pattern() moved to common.rs
use super::common::infer_type_from_mir_pattern;
