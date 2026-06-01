/*!
 * Metadata-only summaries for selected direct-exact hot-core methods.
 *
 * This owner deliberately does not widen `Inline(required)`. Multi-block
 * hot-core methods are summarized so a later direct-exact call-plan producer
 * can explain and validate call boundaries before changing lowering.
 */

use std::collections::BTreeSet;

use crate::mir::core_method_op::LoweringPlanEmitKind;
use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
use crate::mir::{Callee, MirFunction, MirInstruction, MirType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HotCoreMethodSummary {
    pub method: String,
    pub block_count: usize,
    pub instruction_count: usize,
    pub return_kind: String,
    pub allocation_count: usize,
    pub provider_call_count: usize,
    pub public_observer_count: usize,
    pub result_capsule_materialization_count: usize,
    pub safepoint_count: usize,
    pub generic_method_route_count: usize,
    pub generic_method_fallback_count: usize,
    pub dynamic_route_count: usize,
    pub boxed_fallback_count: usize,
    pub nested_call_count: usize,
    pub nested_direct_exact_call_count: usize,
    pub direct_array_access_plan_count: usize,
    pub direct_array_proved_unchecked_count: usize,
    pub summary: &'static str,
    pub failure_reason: Option<&'static str>,
}

pub fn refresh_function_hotcore_method_summaries(function: &mut MirFunction) {
    function.metadata.hotcore_method_summaries.clear();

    if !is_selected_hotcore_method(&function.signature.name) {
        return;
    }

    let instruction_count = function
        .blocks
        .values()
        .map(|block| block.all_instructions().count())
        .sum();

    let mut allocation_count = 0;
    let mut provider_call_count = 0;
    let mut public_observer_count = 0;
    let mut result_capsule_materialization_count = 0;
    let mut safepoint_count = 0;
    let mut dynamic_route_count = 0;
    let mut boxed_fallback_count = 0;
    let mut nested_call_count = 0;

    for block in function.blocks.values() {
        for instruction in block.all_instructions() {
            match instruction {
                MirInstruction::NewBox { .. }
                | MirInstruction::NewClosure { .. }
                | MirInstruction::FutureNew { .. } => {
                    allocation_count += 1;
                }
                MirInstruction::Safepoint | MirInstruction::Await { .. } => {
                    safepoint_count += 1;
                }
                MirInstruction::Call { callee, .. } => {
                    nested_call_count += 1;
                    match callee {
                        Some(Callee::Extern(name)) => {
                            provider_call_count += usize::from(is_provider_like_name(name));
                        }
                        Some(Callee::Global(name)) => {
                            provider_call_count += usize::from(is_provider_like_name(name));
                            public_observer_count += usize::from(is_observer_like_name(name));
                            result_capsule_materialization_count +=
                                usize::from(is_result_capsule_like_name(name));
                        }
                        Some(Callee::Method {
                            box_name,
                            method,
                            certainty,
                            box_kind,
                            ..
                        }) => {
                            if *certainty != TypeCertainty::Known {
                                dynamic_route_count += 1;
                            }
                            if *box_kind != CalleeBoxKind::UserDefined {
                                boxed_fallback_count += 1;
                            }
                            public_observer_count +=
                                usize::from(is_observer_like_name(box_name))
                                    + usize::from(is_observer_like_name(method));
                            result_capsule_materialization_count +=
                                usize::from(is_result_capsule_like_name(box_name))
                                    + usize::from(is_result_capsule_like_name(method));
                        }
                        Some(Callee::Value(_)) | None => {
                            dynamic_route_count += 1;
                        }
                        Some(Callee::Constructor { box_type }) => {
                            allocation_count += 1;
                            result_capsule_materialization_count +=
                                usize::from(is_result_capsule_like_name(box_type));
                        }
                        Some(Callee::Closure { .. }) => {
                            allocation_count += 1;
                        }
                    }
                }
                _ => {}
            }
        }
    }

    let direct_array_sites: BTreeSet<_> = function
        .metadata
        .direct_array_access_plans
        .iter()
        .map(|plan| (plan.block(), plan.instruction_index()))
        .collect();
    let generic_method_route_count = function.metadata.generic_method_routes.len();
    let generic_method_fallback_count = function
        .metadata
        .generic_method_routes
        .iter()
        .filter(|route| !direct_array_sites.contains(&(route.block(), route.instruction_index())))
        .count();
    let nested_direct_exact_call_count = function
        .metadata
        .user_box_method_routes
        .iter()
        .filter(|route| route.lowering_emit_kind() == LoweringPlanEmitKind::DirectFunctionCall)
        .count();
    let direct_array_access_plan_count = function.metadata.direct_array_access_plans.len();
    let direct_array_proved_unchecked_count = function
        .metadata
        .direct_array_access_plans
        .iter()
        .filter(|plan| plan.bounds_policy().as_str() == "proved_unchecked")
        .count();

    let failure_reason = first_failure_reason(
        &function.signature.return_type,
        allocation_count,
        provider_call_count,
        public_observer_count,
        result_capsule_materialization_count,
        safepoint_count,
        generic_method_fallback_count,
        dynamic_route_count,
        boxed_fallback_count,
    );

    function
        .metadata
        .hotcore_method_summaries
        .push(HotCoreMethodSummary {
            method: function.signature.name.clone(),
            block_count: function.blocks.len(),
            instruction_count,
            return_kind: return_kind_label(&function.signature.return_type),
            allocation_count,
            provider_call_count,
            public_observer_count,
            result_capsule_materialization_count,
            safepoint_count,
            generic_method_route_count,
            generic_method_fallback_count,
            dynamic_route_count,
            boxed_fallback_count,
            nested_call_count,
            nested_direct_exact_call_count,
            direct_array_access_plan_count,
            direct_array_proved_unchecked_count,
            summary: if failure_reason.is_none() {
                "ok"
            } else {
                "failed"
            },
            failure_reason,
        });
}

fn is_selected_hotcore_method(name: &str) -> bool {
    matches!(
        name,
        "HakoAllocObjectLifecycleHotCore.objectLifecycleSmallAlloc/1"
            | "HakoAllocObjectLifecycleHotCore.objectLifecycleReleaseBlock/2"
    )
}

fn first_failure_reason(
    return_type: &MirType,
    allocation_count: usize,
    provider_call_count: usize,
    public_observer_count: usize,
    result_capsule_materialization_count: usize,
    safepoint_count: usize,
    generic_method_fallback_count: usize,
    dynamic_route_count: usize,
    boxed_fallback_count: usize,
) -> Option<&'static str> {
    if !matches!(return_type, MirType::Integer | MirType::Bool) {
        return Some("return_not_scalar_i64");
    }
    if allocation_count > 0 {
        return Some("allocation_present");
    }
    if provider_call_count > 0 {
        return Some("provider_call_present");
    }
    if public_observer_count > 0 {
        return Some("public_observer_present");
    }
    if result_capsule_materialization_count > 0 {
        return Some("result_capsule_materialization_present");
    }
    if safepoint_count > 0 {
        return Some("safepoint_present");
    }
    if generic_method_fallback_count > 0 {
        return Some("generic_method_fallback_present");
    }
    if dynamic_route_count > 0 {
        return Some("dynamic_route_present");
    }
    if boxed_fallback_count > 0 {
        return Some("boxed_fallback_present");
    }
    None
}

fn return_kind_label(return_type: &MirType) -> String {
    match return_type {
        MirType::Integer | MirType::Bool => "scalar_i64".to_string(),
        MirType::Void => "void".to_string(),
        MirType::String => "string_handle".to_string(),
        MirType::Box(name) => format!("object_handle:{name}"),
        MirType::Float => "f64".to_string(),
        MirType::Array(_) => "array_handle".to_string(),
        MirType::Future(_) => "future_handle".to_string(),
        MirType::WeakRef => "weak_ref".to_string(),
        MirType::Unknown => "unknown".to_string(),
    }
}

fn is_provider_like_name(name: &str) -> bool {
    name.contains("provider") || name.contains("Provider") || name.starts_with("nyash.provider")
}

fn is_observer_like_name(name: &str) -> bool {
    name.contains("Observer") || name.contains("observer")
}

fn is_result_capsule_like_name(name: &str) -> bool {
    name.contains("Result") || name.contains("Capsule") || name.contains("capsule")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::{BasicBlockId, EffectMask, FunctionSignature, MirInstruction, ValueId};

    fn make_hotcore_function(name: &str) -> MirFunction {
        MirFunction::new(
            FunctionSignature {
                name: name.to_string(),
                params: vec![MirType::Box("HakoAllocObjectLifecycleHotCore".to_string())],
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        )
    }

    #[test]
    fn selected_hotcore_summary_accepts_multiblock_scalar_body() {
        let mut function = make_hotcore_function(
            "HakoAllocObjectLifecycleHotCore.objectLifecycleSmallAlloc/1",
        );
        for id in 1..11 {
            function
                .blocks
                .insert(BasicBlockId::new(id), crate::mir::BasicBlock::new(BasicBlockId::new(id)));
        }
        function
            .get_block_mut(BasicBlockId::new(0))
            .unwrap()
            .set_terminator(MirInstruction::Return {
                value: Some(ValueId::new(1)),
            });

        refresh_function_hotcore_method_summaries(&mut function);

        let summary = &function.metadata.hotcore_method_summaries[0];
        assert_eq!(summary.method, function.signature.name);
        assert_eq!(summary.block_count, 11);
        assert_eq!(summary.return_kind, "scalar_i64");
        assert_eq!(summary.summary, "ok");
    }

    #[test]
    fn non_hotcore_function_has_no_summary() {
        let mut function = make_hotcore_function("OtherBox.objectLifecycleSmallAlloc/1");

        refresh_function_hotcore_method_summaries(&mut function);

        assert!(function.metadata.hotcore_method_summaries.is_empty());
    }
}
