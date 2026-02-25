//! Phase 84-2: CopyTypePropagator — Copy命令型伝播箱（ChatGPT Pro設計）
//!
//! # 責務
//!
//! Copy命令の型伝播のみを担当する専用箱。
//! 固定点ループで Copy チェーン全体に型を伝播させる。
//!
//! # 設計原則（箱理論）
//!
//! - **単一責務**: Copy命令の型伝播のみ
//! - **固定点ループ**: 変化がなくなるまで反復
//! - **副作用限定**: value_types のみ更新
//!
//! # アルゴリズム
//!
//! 1. 関数内の全 Copy 命令を走査
//! 2. src の型が既知 & dst の型が未知 → dst に型を伝播
//! 3. 変化がなくなるまで繰り返す（固定点）
//!
//! # 使用例
//!
//! ```ignore
//! // lifecycle.rs から呼び出し
//! CopyTypePropagator::propagate(&function, &mut value_types);
//! ```

use crate::mir::{MirFunction, MirInstruction, MirType, ValueId};
use std::collections::BTreeMap;

/// Phase 84-2: Copy命令型伝播箱
///
/// Copy チェーン: v1 → v2 → v3 で v1 の型が既知なら v2, v3 にも伝播。
/// Loop exit や If merge の edge copy で発生する型欠如を解消する。
pub struct CopyTypePropagator;

impl CopyTypePropagator {
    /// Copy命令の型を固定点ループで伝播
    ///
    /// # 引数
    ///
    /// - `function`: MIR 関数
    /// - `value_types`: 型マップ（更新される）
    ///
    /// # 戻り値
    ///
    /// 伝播された型の数
    pub fn propagate(
        function: &MirFunction,
        value_types: &mut BTreeMap<ValueId, MirType>,
    ) -> usize {
        let mut total_propagated = 0usize;

        // 固定点ループ: 変化がなくなるまで反復
        loop {
            let propagated = Self::propagate_single_pass(function, value_types);
            if propagated == 0 {
                break;
            }
            total_propagated += propagated;

            // 無限ループ防止（理論上は不要だが安全策）
            if total_propagated > 10000 {
                if std::env::var("NYASH_COPY_PROP_DEBUG").is_ok() {
                    crate::runtime::get_global_ring0()
                        .log
                        .warn("[copy_prop] warning: exceeded 10000 propagations, stopping");
                }
                break;
            }
        }

        if std::env::var("NYASH_COPY_PROP_DEBUG").is_ok() && total_propagated > 0 {
            crate::runtime::get_global_ring0().log.debug(&format!(
                "[copy_prop] {} total types propagated for function {}",
                total_propagated, function.signature.name
            ));
        }

        total_propagated
    }

    /// 1パスの型伝播（内部用）
    fn propagate_single_pass(
        function: &MirFunction,
        value_types: &mut BTreeMap<ValueId, MirType>,
    ) -> usize {
        let mut propagated = 0usize;

        for (_bid, bb) in function.blocks.iter() {
            for inst in bb.instructions.iter() {
                if let MirInstruction::Copy { dst, src } = inst {
                    // src の型が既知 & dst の型が未知 → 伝播
                    if !value_types.contains_key(dst) {
                        if let Some(src_type) = value_types.get(src).cloned() {
                            value_types.insert(*dst, src_type.clone());
                            propagated += 1;

                            if std::env::var("NYASH_COPY_PROP_TRACE").is_ok() {
                                crate::runtime::get_global_ring0().log.debug(&format!(
                                    "[copy_prop] {:?} <- {:?} : {:?}",
                                    dst, src, src_type
                                ));
                            }
                        }
                    }
                }
            }
        }

        propagated
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::builder::copy_emitter::{self, CopyEmitReason};
    use crate::mir::{BasicBlock, BasicBlockId, EffectMask, FunctionSignature, MirFunction};

    fn make_test_function() -> MirFunction {
        let sig = FunctionSignature {
            name: "test".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        };
        MirFunction::new(sig, BasicBlockId::new(0))
    }

    #[test]
    fn test_single_copy_propagation() {
        let mut f = make_test_function();
        let mut value_types = BTreeMap::new();

        // Setup: v1 has type Integer, Copy v2 <- v1
        value_types.insert(ValueId(1), MirType::Integer);

        let mut bb = BasicBlock::new(BasicBlockId::new(0));
        copy_emitter::emit_copy_into_detached_block(
            &mut bb,
            ValueId(2),
            ValueId(1),
            CopyEmitReason::TestCopyTypePropagatorSingle,
        )
        .unwrap();
        f.blocks.insert(BasicBlockId::new(0), bb);

        // Propagate
        let count = CopyTypePropagator::propagate(&f, &mut value_types);

        assert_eq!(count, 1);
        assert_eq!(value_types.get(&ValueId(2)), Some(&MirType::Integer));
    }

    #[test]
    fn test_chain_copy_propagation() {
        let mut f = make_test_function();
        let mut value_types = BTreeMap::new();

        // Setup: v1 -> v2 -> v3 chain
        value_types.insert(ValueId(1), MirType::String);

        let mut bb = BasicBlock::new(BasicBlockId::new(0));
        copy_emitter::emit_copy_into_detached_block(
            &mut bb,
            ValueId(2),
            ValueId(1),
            CopyEmitReason::TestCopyTypePropagatorChain1,
        )
        .unwrap();
        copy_emitter::emit_copy_into_detached_block(
            &mut bb,
            ValueId(3),
            ValueId(2),
            CopyEmitReason::TestCopyTypePropagatorChain2,
        )
        .unwrap();
        f.blocks.insert(BasicBlockId::new(0), bb);

        // Propagate (needs 2 iterations for chain)
        let count = CopyTypePropagator::propagate(&f, &mut value_types);

        assert_eq!(count, 2);
        assert_eq!(value_types.get(&ValueId(2)), Some(&MirType::String));
        assert_eq!(value_types.get(&ValueId(3)), Some(&MirType::String));
    }

    #[test]
    fn test_no_propagation_when_dst_has_type() {
        let mut f = make_test_function();
        let mut value_types = BTreeMap::new();

        // Setup: both v1 and v2 already have types
        value_types.insert(ValueId(1), MirType::Integer);
        value_types.insert(ValueId(2), MirType::Bool); // already typed

        let mut bb = BasicBlock::new(BasicBlockId::new(0));
        copy_emitter::emit_copy_into_detached_block(
            &mut bb,
            ValueId(2),
            ValueId(1),
            CopyEmitReason::TestCopyTypePropagatorDstHasType,
        )
        .unwrap();
        f.blocks.insert(BasicBlockId::new(0), bb);

        // Propagate - should not overwrite existing type
        let count = CopyTypePropagator::propagate(&f, &mut value_types);

        assert_eq!(count, 0);
        assert_eq!(value_types.get(&ValueId(2)), Some(&MirType::Bool));
    }

    #[test]
    fn test_no_propagation_when_src_unknown() {
        let mut f = make_test_function();
        let mut value_types = BTreeMap::new();

        // Setup: v1 has no type, Copy v2 <- v1
        let mut bb = BasicBlock::new(BasicBlockId::new(0));
        copy_emitter::emit_copy_into_detached_block(
            &mut bb,
            ValueId(2),
            ValueId(1),
            CopyEmitReason::TestCopyTypePropagatorSrcUnknown,
        )
        .unwrap();
        f.blocks.insert(BasicBlockId::new(0), bb);

        // Propagate - nothing to propagate
        let count = CopyTypePropagator::propagate(&f, &mut value_types);

        assert_eq!(count, 0);
        assert_eq!(value_types.get(&ValueId(2)), None);
    }
}
