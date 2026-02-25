//! PHI Type Inference - Multi-phase fallback chain for return type resolution
//!
//! Purpose: Infer return types for functions with Void/Unknown signatures
//!
//! Responsibilities:
//! - Multi-phase PHI type resolution (P3-A/B/C/D/P4)
//! - Return type inference from terminator Return values
//! - Debug classification for fallback cases
//!
//! Called by: `finalize_module()` in module_lifecycle.rs
//!
//! Critical Constraints:
//! 1. Must execute AFTER TypePropagationPipeline::run()
//! 2. Must execute AFTER type_hint_providers
//! 3. Resolver order固定: A → B → P3-D → P4 → P3-C
//! 4. Environment variables (NYASH_P3*_DEBUG) control output only, not logic

use super::MirBuilder;
use crate::mir::{MirFunction, MirType};

// Phase 65.5: 型ヒントポリシーを箱化モジュールから使用
use crate::mir::join_ir::lowering::type_hint_policy::TypeHintPolicy;
// Phase 67: P3-C ジェネリック型推論箱
use crate::mir::join_ir::lowering::generic_type_resolver::GenericTypeResolver;
// Phase 83: P3-D 既知メソッド戻り値型推論箱
use crate::mir::join_ir::lowering::method_return_hint::MethodReturnHintBox;
// Phase 84-2: Copy命令型伝播箱（ChatGPT Pro設計）
// Phase 84-3: PHI + Copy グラフ型推論箱（ChatGPT Pro設計）
use crate::mir::phi_core::phi_type_resolver::PhiTypeResolver;

/// Classify PHI fallback case for debug logging
///
/// Phase 82: dev guard helper - Case classification logic unified
///
/// Duplicated Case logic in infer_type_from_phi_with_hint() callsites
/// has been DRY'd.
///
/// Case classification:
/// - Case A: hint available (GenericTypeResolver not needed)
/// - Case B: P1/P2/P3-A/B hint failure (theoretically impossible)
/// - Case D: P3-C GenericTypeResolver failure (PHI scan fallback)
///
/// Note: controlled by dev flag, no #[cfg] needed (env var controlled)
#[allow(dead_code)]
pub(super) fn classify_phi_fallback_case(
    hint: Option<&MirType>,
    function_name: &str,
) -> &'static str {
    if hint.is_some() {
        "Case A (hint付き)"
    } else if TypeHintPolicy::is_target(function_name) {
        "Case B (P1/P2/P3-A/B hint失敗)"
    } else {
        "Case D (P3-C GenericTypeResolver失敗)"
    }
}

/// Infer return type from PHI with multi-phase fallback chain
///
/// Phase 82-5: lifecycle.rs bug fix - check terminator Return only
///   Problem: scanning instructions first incorrectly targets intermediate values (const void etc.)
///   Solution: check terminator Return only to correctly infer actual return value
///
/// # Multi-phase resolver order (SSOT):
/// - Phase A: TypeHintPolicy extract (P1/P2/P3-A/B targets)
/// - Phase B: Direct type lookup from value_types
/// - Phase D: MethodReturnHintBox (P3-D known method return types)
/// - Phase 4: PhiTypeResolver (P4 PHI+Copy graph DFS)
/// - Phase C: GenericTypeResolver (P3-C generic type inference)
///
/// # Arguments
/// - `builder`: MirBuilder with type_ctx for type lookup
/// - `function`: Function to infer return type
///
/// # Returns
/// - `Some(MirType)`: Inferred type
/// - `None`: Inference failed (caller should handle)
pub(super) fn infer_return_type_from_phi(
    builder: &mut MirBuilder,
    function: &mut MirFunction,
) -> Option<MirType> {
    if !matches!(
        function.signature.return_type,
        MirType::Void | MirType::Unknown
    ) {
        return None; // Already has concrete type
    }

    let mut inferred: Option<MirType> = None;
    for (_bid, bb) in function.blocks.iter() {
        // Phase 82-5: instructions scan removed, check terminator Return only
        if let Some(super::MirInstruction::Return { value: Some(v) }) = &bb.terminator {
            if let Some(mt) = builder.type_ctx.value_types.get(v).cloned() {
                inferred = Some(mt);
                break;
            }
            // Phase 65.5: TypeHintPolicy usage (boxed module)
            // Phase 67: P3-C path delegated to GenericTypeResolver
            let hint = if TypeHintPolicy::is_target(&function.signature.name) {
                TypeHintPolicy::extract_phi_type_hint(&function, *v)
            } else {
                None
            };
            // Phase 83: P3-D known method return type inference (try before P3-C)
            //
            // P3-D directly infers "known method return types".
            // Gets type from BoxCall method name via same mapping as TypeAnnotationBox.
            if hint.is_none() {
                if let Some(mt) = MethodReturnHintBox::resolve_for_return(
                    &function,
                    *v,
                    &builder.type_ctx.value_types,
                ) {
                    if crate::config::env::builder_p3d_debug() {
                        crate::runtime::get_global_ring0().log.debug(&format!(
                            "[lifecycle/p3d] {} type inferred via MethodReturnHintBox: {:?}",
                            function.signature.name, mt
                        ));
                    }
                    inferred = Some(mt);
                    break;
                }
            }
            // Phase 84-3: P4 PHI + Copy graph type inference (try before P3-C)
            //
            // DFS explores PHI + Copy small graph and returns only if converged to 1 type.
            // This resolves type inference after Loop edge copy / If merge.
            if hint.is_none() {
                let phi_resolver = PhiTypeResolver::new(&function, &builder.type_ctx.value_types);
                if let Some(mt) = phi_resolver.resolve(*v) {
                    if crate::config::env::builder_p4_debug() {
                        crate::runtime::get_global_ring0().log.debug(&format!(
                            "[lifecycle/p4] {} type inferred via PhiTypeResolver: {:?}",
                            function.signature.name, mt
                        ));
                    }
                    inferred = Some(mt);
                    break;
                }
            }
            // Phase 67: P3-C targets prefer GenericTypeResolver
            if hint.is_none() && TypeHintPolicy::is_p3c_target(&function.signature.name) {
                if let Some(mt) = GenericTypeResolver::resolve_from_phi(
                    &function,
                    *v,
                    &builder.type_ctx.value_types,
                ) {
                    if crate::config::env::builder_p3c_debug() {
                        crate::runtime::get_global_ring0().log.debug(&format!(
                            "[lifecycle/p3c] {} type inferred via GenericTypeResolver: {:?}",
                            function.signature.name, mt
                        ));
                    }
                    inferred = Some(mt);
                    break;
                }
            }
            // Phase 84-5: safe guard after if_phi.rs complete removal
            #[cfg(debug_assertions)]
            {
                panic!(
                    "[phase84-5] Type inference failed for {:?} in function {}\n\
                     This should not happen after Phase 84-4 completion.\n\
                     Please check: PhiTypeResolver, BoxCall type registration, CopyTypePropagator",
                    v, function.signature.name
                );
            }

            #[cfg(not(debug_assertions))]
            {
                crate::runtime::get_global_ring0().log.warn(&format!(
                    "[phase84-5/warning] Type inference failed for {:?} in {}, using Unknown fallback",
                    v, function.signature.name
                ));
                inferred = Some(MirType::Unknown);
            }
        }
    }
    inferred
}
