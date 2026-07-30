//! PHI Type Inference - Multi-phase resolver chain for return type resolution
//!
//! Purpose: Infer return types for functions with Void/Unknown signatures
//!
//! Responsibilities:
//! - Multi-phase PHI type resolution (P3-A/B/C/D/P4)
//! - Return type inference from terminator Return values
//! - Debug classification for resolver miss cases
//!
//! Called by: `finalize_module()` in module_lifecycle.rs
//!
//! Critical Constraints:
//! 1. Must execute AFTER TypePropagationPipeline::run()
//! 2. Must execute AFTER type_hint_providers
//! 3. Resolver order固定: A → B → P3-D → P4 → P3-C
//! 4. Environment variables (NYASH_P3*_DEBUG) control output only, not logic

use super::MirBuilder;
use crate::mir::{MirFunction, MirInstruction, MirType, TypeOpKind, ValueId};
use std::collections::BTreeMap;

// Phase 65.5: 型ヒントポリシーを箱化モジュールから使用
use crate::mir::join_ir::lowering::type_hint_policy::TypeHintPolicy;
// Phase 67: P3-C ジェネリック型推論箱
use crate::mir::join_ir::lowering::generic_type_resolver::GenericTypeResolver;
// Phase 84-2: Copy命令型伝播箱（ChatGPT Pro設計）
// Phase 84-3: PHI + Copy グラフ型推論箱（ChatGPT Pro設計）
use crate::mir::phi_core::phi_type_resolver::PhiTypeResolver;

/// Classify PHI resolver miss case for debug logging
///
/// Phase 82: dev guard helper - Case classification logic unified
///
/// Duplicated Case logic in infer_type_from_phi_with_hint() callsites
/// has been DRY'd.
///
/// Case classification:
/// - Case A: hint available (GenericTypeResolver not needed)
/// - Case B: P1/P2/P3-A/B hint failure (theoretically impossible)
/// - Case D: P3-C GenericTypeResolver failure (PHI scan resolver)
///
/// Note: controlled by dev flag, no #[cfg] needed (env var controlled)
#[allow(dead_code)]
pub(super) fn classify_phi_resolver_miss_case(
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

/// Infer return type from PHI with multi-phase resolver chain
///
/// Phase 82-5: lifecycle.rs bug fix - check terminator Return only
///   Problem: scanning instructions first incorrectly targets intermediate values (const void etc.)
///   Solution: check terminator Return only to correctly infer actual return value
///
/// # Multi-phase resolver order (SSOT):
/// - Phase A: TypeHintPolicy extract (P1/P2/P3-A/B targets)
/// - Phase B: Direct type lookup from value_types
/// - Phase D: known return-definition hint (P3-D)
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
            if let Some(mt) = builder.function_state.type_ctx.value_types.get(v).cloned() {
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
            // P3-D: known return-definition hint (try before P4/P3-C).
            if hint.is_none() {
                if let Some(mt) = resolve_known_return_definition_type(
                    &function,
                    *v,
                    &builder.function_state.type_ctx.value_types,
                ) {
                    if crate::config::env::builder_p3d_debug() {
                        crate::runtime::get_global_ring0().log.debug(&format!(
                            "[lifecycle/p3d] {} type inferred via known return definition: {:?}",
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
                let phi_resolver =
                    PhiTypeResolver::new(&function, &builder.function_state.type_ctx.value_types);
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
                    &builder.function_state.type_ctx.value_types,
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
                    "[phase84-5/warning] Type inference failed for {:?} in {}, using Unknown sentinel",
                    v, function.signature.name
                ));
                inferred = Some(MirType::Unknown);
            }
        }
    }
    inferred
}

/// P3-D: derive a return type from the definition of the returned value.
///
/// This is deliberately local to the finalization owner: it observes an
/// already-built MIR function and delegates the actual method-name policy to
/// the existing builder annotation helpers.
fn resolve_known_return_definition_type(
    function: &MirFunction,
    return_value: ValueId,
    value_types: &BTreeMap<ValueId, MirType>,
) -> Option<MirType> {
    if let Some(ty) = value_types.get(&return_value) {
        return Some(ty.clone());
    }

    for (_block_id, block) in function.blocks.iter() {
        for instruction in block.instructions.iter() {
            match instruction {
                MirInstruction::Call {
                    dst: Some(dst),
                    callee:
                        Some(crate::mir::Callee::Method {
                            receiver: Some(receiver),
                            method,
                            ..
                        }),
                    ..
                } if *dst == return_value => {
                    if let Some(ty) =
                        resolve_known_instance_method_return_type(value_types.get(receiver), method)
                    {
                        return Some(ty);
                    }
                }
                MirInstruction::Call {
                    dst: Some(dst),
                    callee: Some(callee),
                    ..
                } if *dst == return_value => {
                    let ty = resolve_known_callee_return_type(callee);
                    if ty.is_some() {
                        return ty;
                    }
                }
                MirInstruction::TypeOp { dst, op, ty, .. } if *dst == return_value => {
                    return Some(resolve_known_typeop_return_type(op, ty));
                }
                _ => {}
            }
        }
    }

    None
}

fn resolve_known_instance_method_return_type(
    receiver_type: Option<&MirType>,
    method: &str,
) -> Option<MirType> {
    let box_name = match receiver_type {
        Some(MirType::Box(name)) => Some(name.as_str()),
        _ => None,
    };
    crate::mir::builder::infer_known_method_return_type(box_name, method, None)
}

fn resolve_known_callee_return_type(callee: &crate::mir::Callee) -> Option<MirType> {
    match callee {
        crate::mir::Callee::Method {
            box_name, method, ..
        } => crate::mir::builder::infer_known_method_return_type(
            Some(box_name.as_str()),
            method,
            None,
        ),
        crate::mir::Callee::Global(name) => crate::mir::builder::infer_known_return_type(name),
        _ => None,
    }
}

fn resolve_known_typeop_return_type(op: &TypeOpKind, ty: &MirType) -> MirType {
    match op {
        TypeOpKind::Check => MirType::Bool,
        TypeOpKind::Cast => ty.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_instance_method_return_types_use_existing_annotation_policy() {
        let cases = [
            ("StringBox", "length", Some(MirType::Integer)),
            ("ArrayBox", "push", Some(MirType::Void)),
            ("IntegerBox", "str", Some(MirType::String)),
            ("MapBox", "has", Some(MirType::Bool)),
            ("UnknownBox", "unknown_method", None),
        ];

        for (box_name, method, expected) in cases {
            assert_eq!(
                resolve_known_instance_method_return_type(
                    Some(&MirType::Box(box_name.into())),
                    method,
                ),
                expected,
            );
        }
    }

    #[test]
    fn known_return_definition_typeop_policy_is_exact() {
        assert_eq!(
            resolve_known_typeop_return_type(&TypeOpKind::Check, &MirType::Integer),
            MirType::Bool,
        );
        assert_eq!(
            resolve_known_typeop_return_type(&TypeOpKind::Cast, &MirType::String),
            MirType::String,
        );
    }
}
