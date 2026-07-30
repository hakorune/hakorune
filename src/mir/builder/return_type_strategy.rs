//! Finalization return-type strategy.
//!
//! This is the one Builder-owned strategy for a function whose declared return
//! type is still `Void` or `Unknown` at finalization. It observes completed MIR
//! only; PHI materialization, module publication, and route selection stay with
//! their existing owners.

mod primary_hint;
mod uniform_phi;

use super::MirBuilder;
use crate::mir::{MirFunction, MirInstruction, MirType, TypeOpKind, ValueId};
use std::collections::BTreeMap;

/// Classify a resolver miss for the existing optional debug observation.
#[allow(dead_code)]
pub(super) fn classify_return_type_strategy_miss(
    hint: Option<&MirType>,
    function_name: &str,
) -> &'static str {
    if hint.is_some() {
        "Case A (hint付き)"
    } else if primary_hint::is_primary_target(function_name) {
        "Case B (P1/P2/P3-A/B hint失敗)"
    } else {
        "Case D (P3-C uniform-PHI fallback失敗)"
    }
}

/// Infer a missing return type from the completed function's terminal Return.
///
/// The first matching strategy wins, in this fixed order:
/// direct value type, primary name hint, known return definition, PHI/Copy
/// graph, then the uniform-PHI fallback.
pub(super) fn infer_return_type_from_phi(
    builder: &mut MirBuilder,
    function: &mut MirFunction,
) -> Option<MirType> {
    if !matches!(
        function.signature.return_type,
        MirType::Void | MirType::Unknown
    ) {
        return None;
    }

    let mut inferred = None;
    for (_bid, bb) in function.blocks.iter() {
        if let Some(MirInstruction::Return { value: Some(value) }) = &bb.terminator {
            if let Some(mt) = builder
                .function_state
                .type_ctx
                .value_types
                .get(value)
                .cloned()
            {
                inferred = Some(mt);
                break;
            }

            let hint = if primary_hint::is_primary_target(&function.signature.name) {
                primary_hint::extract_phi_type_hint(function, *value)
            } else {
                None
            };
            if hint.is_none() {
                if let Some(mt) = resolve_known_return_definition_type(
                    function,
                    *value,
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
            if hint.is_none() {
                let phi_resolver = crate::mir::phi_core::phi_type_resolver::PhiTypeResolver::new(
                    function,
                    &builder.function_state.type_ctx.value_types,
                );
                if let Some(mt) = phi_resolver.resolve(*value) {
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
            if hint.is_none()
                && primary_hint::is_uniform_phi_fallback_target(&function.signature.name)
            {
                if let Some(mt) = uniform_phi::resolve_from_phi(
                    function,
                    *value,
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

            #[cfg(debug_assertions)]
            panic!(
                "[phase84-5] Type inference failed for {:?} in function {}\n\
                 This should not happen after Phase 84-4 completion.\n\
                 Please check: PhiTypeResolver, BoxCall type registration, CopyTypePropagator",
                value, function.signature.name
            );

            #[cfg(not(debug_assertions))]
            {
                crate::runtime::get_global_ring0().log.warn(&format!(
                    "[phase84-5/warning] Type inference failed for {:?} in {}, using Unknown sentinel",
                    value, function.signature.name
                ));
                inferred = Some(MirType::Unknown);
            }
        }
    }
    inferred
}

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
                    if let Some(ty) = resolve_known_callee_return_type(callee) {
                        return Some(ty);
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
#[path = "return_type_strategy_tests.rs"]
mod tests;
