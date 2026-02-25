//! Phase 279 P0: SSOT Type Propagation Pipeline
//!
//! # 責務
//!
//! 全ての型伝播処理を1つの入口（SSOT）に統一する。
//! lifecycle.rs と joinir_function_converter.rs の両方のルートがこのパイプラインを呼ぶ。
//!
//! # 設計原則（箱理論）
//!
//! - **単一責務**: 型伝播パイプライン全体の統括（SSOT）
//! - **固定順序**: Copy → BinOp → Copy → PHI（順序ドリフト防止）
//! - **Private step**: PHI 推論を private にして、直接呼び出しを防止（構造的保証）
//!
//! # アルゴリズム
//!
//! 1. Copy type propagation（初回）
//! 2. BinOp re-propagation（数値型昇格: Int+Float→Float）
//! 3. Copy type propagation（昇格後の型を伝播）
//! 4. PHI type inference（PHI ノードの型推論）
//!
//! # Fail-Fast ガード
//!
//! - PHI 推論は private step → lifecycle/joinir が直接 PhiTypeResolver を呼べない
//! - 入口一本化により、順序ドリフトが構造的に不可能

use crate::mir::phi_core::copy_type_propagator::CopyTypePropagator;
use crate::mir::phi_core::phi_type_resolver::PhiTypeResolver;
use crate::mir::{BinaryOp, MirFunction, MirInstruction, MirType, ValueId};
use crate::runtime::get_global_ring0;
use std::collections::BTreeMap;

/// Phase 279 P0: SSOT 型伝播パイプライン
///
/// lifecycle.rs と joinir_function_converter.rs の両方から呼ばれる唯一の入口。
pub struct TypePropagationPipeline;

impl TypePropagationPipeline {
    /// SSOT 入口: 完全な型伝播パイプラインを実行
    ///
    /// # 引数
    ///
    /// - `function`: MIR 関数
    /// - `value_types`: 型マップ（更新される）
    ///
    /// # 順序（固定）
    ///
    /// 1. Copy propagation（初回）
    /// 2. BinOp re-propagation（数値型昇格）
    /// 3. Copy propagation（昇格後の型を伝播）
    /// 4. PHI type inference（private step）
    ///
    /// # Fail-Fast ガード
    ///
    /// PHI 推論は private step なので、lifecycle/joinir が直接 PhiTypeResolver を呼ぶことは不可能。
    /// 入口一本化により、順序ドリフトが構造的に防止される。
    pub fn run(
        function: &mut MirFunction,
        value_types: &mut BTreeMap<ValueId, MirType>,
    ) -> Result<(), String> {
        // Step 1: Copy propagation (initial)
        Self::step1_copy_propagation(function, value_types)?;

        // Step 2: BinOp re-propagation (numeric promotion)
        Self::step2_binop_repropagation(function, value_types)?;

        // Step 3: Copy propagation (propagate promoted types)
        Self::step3_copy_propagation(function, value_types)?;

        // Step 4: PHI type inference (private - cannot be called directly)
        Self::step4_phi_type_inference(function, value_types)?;

        Ok(())
    }

    // ========================================================================
    // Private steps - 外部から直接呼び出し不可（構造的保証）
    // ========================================================================

    /// Step 1: Copy type propagation（初回）
    ///
    /// Loop exit や If merge の edge copy で発生する型欠如を解消する。
    fn step1_copy_propagation(
        function: &MirFunction,
        value_types: &mut BTreeMap<ValueId, MirType>,
    ) -> Result<(), String> {
        CopyTypePropagator::propagate(function, value_types);
        Ok(())
    }

    /// Step 2: BinOp re-propagation（数値型昇格）
    ///
    /// Phase 275 P0: BinOp 結果型の再推論（オペランド型に基づく）
    /// - String+String → StringBox
    /// - Int+Int → Integer
    /// - Int+Float → Float（数値型昇格）
    ///
    /// # アルゴリズム
    ///
    /// Pass 1: BinOp 命令の型更新を収集
    /// - Add: String+String→StringBox, Int+Int→Integer, Int+Float→Float
    /// - Other ops: Always Integer（型が欠けている場合）
    ///
    /// Pass 2: Copy チェーン経由で型を伝播（固定点ループ、最大10回）
    ///
    /// # 元の実装
    ///
    /// lifecycle.rs の repropagate_binop_types() (lines 687-797) から抽出。
    /// より包括的な実装を採用（joinir_function_converter.rs 版は破棄）。
    fn step2_binop_repropagation(
        function: &MirFunction,
        value_types: &mut BTreeMap<ValueId, MirType>,
    ) -> Result<(), String> {
        // Pass 1: Process BinOp instructions first
        let mut binop_updates: Vec<(ValueId, MirType)> = Vec::new();

        for (_bid, bb) in function.blocks.iter() {
            for inst in bb.instructions.iter() {
                if let MirInstruction::BinOp { dst, op, lhs, rhs } = inst {
                    // Only handle Add operations (string concat vs numeric addition)
                    if matches!(op, BinaryOp::Add) {
                        // Get current lhs/rhs types after initial Copy propagation
                        let lhs_type = value_types.get(lhs);
                        let rhs_type = value_types.get(rhs);

                        // Classify types (Phase 275 P0: Added Float)
                        let lhs_class = classify_operand_type(lhs_type);
                        let rhs_class = classify_operand_type(rhs_type);

                        use OperandTypeClass::*;
                        let new_type = match (lhs_class, rhs_class) {
                            (String, String) => Some(MirType::Box("StringBox".to_string())),
                            (Integer, Integer) | (Integer, Unknown) | (Unknown, Integer) => {
                                Some(MirType::Integer)
                            }
                            // Phase 275 P0 C2: Number promotion (Int+Float → Float)
                            (Integer, Float) | (Float, Integer) => Some(MirType::Float),
                            (Float, Float) => Some(MirType::Float),
                            _ => None, // Keep Unknown for mixed/unclear cases
                        };

                        if let Some(new_ty) = new_type {
                            // Check if type is missing or different
                            let current_type = value_types.get(dst);
                            if current_type.is_none() || current_type != Some(&new_ty) {
                                binop_updates.push((*dst, new_ty));
                            }
                        }
                    } else {
                        // Other arithmetic ops: always Integer
                        if !value_types.contains_key(dst) {
                            binop_updates.push((*dst, MirType::Integer));
                        }
                    }
                }
            }
        }

        // Apply binop updates first
        for (dst, ty) in binop_updates {
            if crate::config::env::binop_reprop_debug_enabled() {
                let ring0 = crate::runtime::get_global_ring0();
                ring0.log.debug(&format!(
                    "[binop-reprop] {} updated {:?} -> {:?}",
                    function.signature.name, dst, ty
                ));
            }
            value_types.insert(dst, ty);
        }

        // Pass 2: Propagate types through Copy chains
        // Iterate until no more updates (for chains like 5→6→20→23)
        for _iteration in 0..10 {
            // Max 10 iterations for safety
            let mut copy_updates: Vec<(ValueId, MirType)> = Vec::new();

            for (_bid, bb) in function.blocks.iter() {
                for inst in bb.instructions.iter() {
                    if let MirInstruction::Copy { dst, src } = inst {
                        if let Some(src_type) = value_types.get(src) {
                            let current_dst_type = value_types.get(dst);
                            // Propagate if dst has no type or has different type
                            if current_dst_type.is_none() || current_dst_type != Some(src_type) {
                                copy_updates.push((*dst, src_type.clone()));
                            }
                        }
                    }
                }
            }

            if copy_updates.is_empty() {
                break; // No more updates, done
            }

            // Apply copy updates
            for (dst, ty) in copy_updates {
                if crate::config::env::binop_reprop_debug_enabled() {
                    let ring0 = crate::runtime::get_global_ring0();
                    ring0.log.debug(&format!(
                        "[copy-reprop] {} updated {:?} -> {:?}",
                        function.signature.name, dst, ty
                    ));
                }
                value_types.insert(dst, ty);
            }
        }

        Ok(())
    }

    /// Step 3: Copy type propagation（昇格後の型を伝播）
    ///
    /// BinOp で昇格された Float 型を Copy チェーン経由で伝播する。
    /// 例: v11（Float）→ v12（Copy）
    fn step3_copy_propagation(
        function: &MirFunction,
        value_types: &mut BTreeMap<ValueId, MirType>,
    ) -> Result<(), String> {
        CopyTypePropagator::propagate(function, value_types);
        Ok(())
    }

    /// Step 4: PHI type inference（PHI ノードの型推論）
    ///
    /// **Private step**: lifecycle.rs / joinir_function_converter.rs が直接 PhiTypeResolver を呼ぶことは不可能。
    /// 入口一本化により、PHI 推論が BinOp re-propagation の前に実行されることが構造的に防止される。
    ///
    /// # アルゴリズム
    ///
    /// 1. 全ブロックから PHI dst を収集
    /// 2. PhiTypeResolver で各 PHI の型を推論
    /// 3. 型が変更された、または欠けている場合のみ value_types を更新
    ///
    /// # 元の実装
    ///
    /// lifecycle.rs の PHI type inference (lines 333-399) から抽出。
    fn step4_phi_type_inference(
        function: &MirFunction,
        value_types: &mut BTreeMap<ValueId, MirType>,
    ) -> Result<(), String> {
        // Collect ALL PHI dsts for re-inference (not just untyped)
        // This is necessary because propagate_phi_meta may have assigned incorrect types
        // due to circular dependencies (e.g., loop carrier PHIs)
        let mut all_phi_dsts: Vec<ValueId> = Vec::new();
        for (_bid, bb) in function.blocks.iter() {
            for inst in &bb.instructions {
                if let MirInstruction::Phi { dst, .. } = inst {
                    if crate::config::env::phi_global_debug_enabled() {
                        let existing_type = value_types.get(dst);
                        get_global_ring0().log.debug(&format!(
                            "[lifecycle/phi-scan] {} PHI {:?} existing type: {:?}",
                            function.signature.name, dst, existing_type
                        ));
                    }
                    all_phi_dsts.push(*dst);
                }
            }
        }

        if crate::config::env::phi_global_debug_enabled() {
            get_global_ring0().log.debug(&format!(
                "[lifecycle/phi-scan] {} found {} total PHIs to re-infer",
                function.signature.name,
                all_phi_dsts.len()
            ));
        }

        // Re-infer types for ALL PHI nodes using PhiTypeResolver
        // This fixes incorrect types assigned by propagate_phi_meta during circular dependencies
        if !all_phi_dsts.is_empty() {
            let phi_resolver = PhiTypeResolver::new(function, value_types);
            let mut inferred_types: Vec<(ValueId, MirType)> = Vec::new();
            for dst in all_phi_dsts {
                if let Some(mt) = phi_resolver.resolve(dst) {
                    // Check if type changed
                    let existing_type = value_types.get(&dst);
                    if existing_type.is_none() || existing_type != Some(&mt) {
                        inferred_types.push((dst, mt));
                    }
                }
            }

            // Now insert/update all inferred types
            for (dst, mt) in inferred_types {
                let old_type = value_types.get(&dst).cloned();
                value_types.insert(dst, mt.clone());
                if crate::config::env::phi_global_debug_enabled() {
                    if let Some(old) = old_type {
                        get_global_ring0().log.debug(&format!(
                            "[lifecycle/phi-global] {} PHI {:?} type corrected: {:?} -> {:?}",
                            function.signature.name, dst, old, mt
                        ));
                    } else {
                        get_global_ring0().log.debug(&format!(
                            "[lifecycle/phi-global] {} PHI {:?} type inferred: {:?}",
                            function.signature.name, dst, mt
                        ));
                    }
                }
            }
        }

        Ok(())
    }
}

// ============================================================================
// Helper types and functions
// ============================================================================

/// Phase 131-11-E: OperandTypeClass for BinOp type inference
/// Phase 275 P0: Added Float for number promotion
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OperandTypeClass {
    String,
    Integer,
    Float, // Phase 275 P0
    Unknown,
}

/// Classify operand type for BinOp re-propagation
fn classify_operand_type(ty: Option<&MirType>) -> OperandTypeClass {
    match ty {
        Some(MirType::String) => OperandTypeClass::String,
        Some(MirType::Box(bt)) if bt == "StringBox" => OperandTypeClass::String,
        Some(MirType::Integer) => OperandTypeClass::Integer,
        Some(MirType::Bool) => OperandTypeClass::Integer,
        Some(MirType::Float) => OperandTypeClass::Float, // Phase 275 P0
        _ => OperandTypeClass::Unknown,
    }
}
