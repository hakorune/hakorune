//! Test-only legacy DirectAccum physical snapshot adapter.
//!
//! The legacy composer/PlanLowerer remain the source authority here.  This
//! adapter only translates their result into the immutable alpha observer;
//! it does not add a writer or a production caller.

#![cfg(test)]

use super::physical_digest_test_support::{
    observe_mir, AlphaFinalBindingWitnessV1, AlphaFunctionResultWitnessV1,
    AlphaPhysicalMirDigestV1, MirRoleWitnessV1,
};
use super::semantic_digest_test_support::{
    semantic_digest, AlphaPhysicalMirDigestV2, DirectAccumLegacyAuxPolicyV1,
};
use crate::mir::builder::control_flow::plan::{CoreLoopPlan, CorePlan};
use crate::mir::{MirFunction, MirInstruction, MirType, ValueId};
use std::collections::BTreeMap;

#[derive(Default)]
struct ValueLabels {
    labels: BTreeMap<ValueId, String>,
}

impl ValueLabels {
    fn set(&mut self, value: ValueId, label: impl Into<String>) {
        self.labels.entry(value).or_insert_with(|| label.into());
    }

    fn get(&self, value: ValueId) -> Result<&str, String> {
        self.labels
            .get(&value)
            .map(String::as_str)
            .ok_or_else(|| format!("legacy adapter encountered uncredited value {value:?}"))
    }

    fn into_map(self) -> BTreeMap<ValueId, String> {
        self.labels
    }
}

fn normalize_phi_tag(tag: &str) -> String {
    tag.strip_prefix("loop_v0_")
        .unwrap_or(tag)
        .replace('_', ":")
}

fn role_witness(plan: &CoreLoopPlan) -> Result<MirRoleWitnessV1, String> {
    let mut rows = vec![
        ("P", plan.preheader_bb),
        ("H", plan.header_bb),
        ("B", plan.body_bb),
        ("S", plan.step_bb),
        ("A", plan.after_bb),
    ];
    if plan.found_bb != plan.after_bb {
        rows.push(("F", plan.found_bb));
    }
    MirRoleWitnessV1::new(rows)
}

fn derive_labels(function: &MirFunction, plan: &CoreLoopPlan) -> Result<ValueLabels, String> {
    let mut labels = ValueLabels::default();
    for phi in &plan.phis {
        if let Some((_, value)) = phi.inputs.first() {
            if phi.tag.ends_with("carrier_i") {
                labels.set(*value, "binding:i");
            } else if phi.tag.ends_with("carrier_sum") {
                labels.set(*value, "binding:sum");
            }
        }
    }
    for role_block in [
        plan.preheader_bb,
        plan.header_bb,
        plan.body_bb,
        plan.step_bb,
        plan.after_bb,
    ] {
        let block = function
            .blocks
            .get(&role_block)
            .ok_or_else(|| format!("legacy role block missing: {role_block:?}"))?;
        let phi_tags = plan
            .phis
            .iter()
            .filter(|phi| phi.block == role_block)
            .map(|phi| phi.tag.as_str())
            .collect::<Vec<_>>();
        let mut phi_index = 0;
        let mut entry_constants = 0;
        for instruction in &block.instructions {
            match instruction {
                MirInstruction::Phi { dst, .. } => {
                    let tag = phi_tags.get(phi_index).ok_or_else(|| {
                        format!("legacy PHI has no plan witness at {role_block:?}")
                    })?;
                    labels.set(*dst, format!("phi:{}", normalize_phi_tag(tag)));
                    phi_index += 1;
                }
                MirInstruction::Const { dst, value } => {
                    if role_block == plan.preheader_bb && entry_constants < 2 {
                        labels.set(
                            *dst,
                            if entry_constants == 0 {
                                "binding:i".to_owned()
                            } else {
                                "binding:sum".to_owned()
                            },
                        );
                        entry_constants += 1;
                    } else {
                        labels.set(*dst, format!("const:{value:?}"));
                    }
                }
                MirInstruction::Copy { dst, src } => {
                    labels.set(*dst, labels.get(*src)?.to_owned());
                }
                MirInstruction::BinOp { dst, op, lhs, rhs } => {
                    let left = labels.get(*lhs)?.to_owned();
                    let right = labels.get(*rhs)?.to_owned();
                    labels.set(*dst, format!("bin:{op:?}:{left}:{right}"));
                }
                MirInstruction::Compare { dst, op, lhs, rhs } => {
                    let left = labels.get(*lhs)?.to_owned();
                    let right = labels.get(*rhs)?.to_owned();
                    labels.set(*dst, format!("compare:{op:?}:{left}:{right}"));
                }
                MirInstruction::KeepAlive { values } => {
                    for value in values {
                        let _ = labels.get(*value)?;
                    }
                }
                _ => {}
            }
        }
        if phi_index != phi_tags.len() {
            return Err(format!(
                "legacy plan PHI witness was not emitted at {role_block:?}"
            ));
        }
    }
    Ok(labels)
}

fn final_bindings(
    function: &MirFunction,
    plan: &CoreLoopPlan,
    labels: &ValueLabels,
) -> Result<Vec<AlphaFinalBindingWitnessV1>, String> {
    let block = function
        .blocks
        .get(&plan.after_bb)
        .ok_or_else(|| "legacy after block missing".to_owned())?;
    let after_phis = block
        .instructions
        .iter()
        .filter_map(|instruction| match instruction {
            MirInstruction::Phi { dst, inputs, .. } => Some((*dst, inputs)),
            _ => None,
        })
        .collect::<Vec<_>>();
    if after_phis.len() != plan.final_values.len() {
        return Err(format!(
            "legacy final PHI count mismatch: {} != {}",
            after_phis.len(),
            plan.final_values.len()
        ));
    }
    plan.final_values
        .iter()
        .zip(after_phis)
        .map(|((name, _), (value, inputs))| {
            let provenance_value = inputs
                .first()
                .map(|(_, value)| *value)
                .ok_or_else(|| format!("legacy final PHI has no input: {name}"))?;
            Ok(AlphaFinalBindingWitnessV1 {
                name: name.clone(),
                value,
                provenance: labels.get(provenance_value)?.to_owned(),
            })
        })
        .collect()
}

pub(super) fn legacy_alpha_digest(
    function: &MirFunction,
    plan: &CoreLoopPlan,
    result: Option<ValueId>,
    value_types: &BTreeMap<ValueId, MirType>,
) -> Result<AlphaPhysicalMirDigestV1, String> {
    let labels = derive_labels(function, plan)?;
    let final_bindings = final_bindings(function, plan, &labels)?;
    let result = AlphaFunctionResultWitnessV1 {
        value: result,
        provenance: "unit".to_owned(),
        expected_type: MirType::Void,
    };
    observe_mir(
        function,
        &role_witness(plan)?,
        &labels.into_map(),
        &final_bindings,
        &result,
        value_types,
    )
}

pub(super) fn legacy_semantic_digest(
    function: &MirFunction,
    plan: &CoreLoopPlan,
    result: Option<ValueId>,
    value_types: &BTreeMap<ValueId, MirType>,
) -> Result<AlphaPhysicalMirDigestV2, String> {
    let raw = legacy_alpha_digest(function, plan, result, value_types)?;
    let mut digest = semantic_digest(
        &raw,
        &[
            "final:i:carrier:i:Integer",
            "final:sum:carrier:sum:Integer",
            "result:unit:Void",
        ],
    )?;
    let mut operations = digest.semantic.operations.into_vec();
    operations.extend([
        "P:const:binding:i=Integer(0)".to_owned(),
        "P:const:binding:sum=Integer(0)".to_owned(),
    ]);
    operations.sort();
    operations.dedup();
    digest.semantic.operations = operations.into_boxed_slice();
    DirectAccumLegacyAuxPolicyV1.validate(&digest.legacy_aux)?;
    Ok(digest)
}

pub(super) fn direct_legacy_semantic_digest() -> AlphaPhysicalMirDigestV2 {
    let mut builder = crate::mir::builder::MirBuilder::new();
    builder.enter_function_for_test("accum_physical_semantic/0".to_owned());
    let i = builder.alloc_typed(MirType::Integer);
    let sum = builder.alloc_typed(MirType::Integer);
    builder.bind_variable_for_test("i", i);
    builder.bind_variable_for_test("sum", sum);
    let _scope = crate::mir::builder::vars::lexical_scope::LexicalScopeGuard::new(&mut builder);
    let (condition, body) = super::legacy_oracle::direct_accum_source();
    let plan = super::legacy_oracle::prepare_accum_legacy_plan(
        &mut builder,
        &condition,
        &body,
        "accum_physical_semantic/0",
    )
    .expect("legacy Accum oracle should compose");
    let CorePlan::Loop(loop_plan) = plan.clone() else {
        panic!("legacy Accum oracle should produce a loop plan")
    };
    let result = crate::mir::builder::control_flow::lower::PlanLowerer::lower(
        &mut builder,
        plan,
        &crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext::new(
            &condition,
            &body,
            "accum_physical_semantic/0",
            false,
            false,
        ),
    )
    .expect("legacy Accum oracle should lower");
    let function = builder
        .function_state
        .current_function
        .as_ref()
        .expect("legacy function");
    let value_types = builder.function_state.type_ctx.value_types.clone();
    legacy_semantic_digest(function, &loop_plan, result, &value_types)
        .expect("legacy semantic digest")
}

#[test]
fn direct_legacy_physical_snapshot_observes_standard5_without_new_writer() {
    let mut builder = crate::mir::builder::MirBuilder::new();
    builder.enter_function_for_test("accum_physical_snapshot/0".to_owned());
    let i = builder.alloc_typed(MirType::Integer);
    let sum = builder.alloc_typed(MirType::Integer);
    builder.bind_variable_for_test("i", i);
    builder.bind_variable_for_test("sum", sum);
    let _scope = crate::mir::builder::vars::lexical_scope::LexicalScopeGuard::new(&mut builder);
    let (condition, body) = super::legacy_oracle::direct_accum_source();
    let plan = super::legacy_oracle::prepare_accum_legacy_plan(
        &mut builder,
        &condition,
        &body,
        "accum_physical_snapshot/0",
    )
    .expect("legacy Accum oracle should compose");
    let CorePlan::Loop(loop_plan) = plan.clone() else {
        panic!("legacy Accum oracle should produce a loop plan")
    };
    let result = crate::mir::builder::control_flow::lower::PlanLowerer::lower(
        &mut builder,
        plan,
        &crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext::new(
            &condition,
            &body,
            "accum_physical_snapshot/0",
            false,
            false,
        ),
    )
    .expect("legacy Accum oracle should lower");
    assert!(result.is_some());
    let function = builder
        .function_state
        .current_function
        .as_ref()
        .expect("current legacy function");
    let value_types = builder.function_state.type_ctx.value_types.clone();
    let digest = legacy_alpha_digest(function, &loop_plan, result, &value_types)
        .expect("legacy alpha observer");

    assert!(digest
        .cfg
        .iter()
        .any(|row| row.contains("P:pred=[]:succ=[H]")));
    assert!(digest
        .cfg
        .iter()
        .any(|row| row.contains("B:pred=[]:succ=[S]")));
    assert!(digest
        .cfg
        .iter()
        .any(|row| row.contains("S:pred=[B]:succ=[H]")));
    assert!(digest
        .phis
        .iter()
        .any(|row| row.contains("phi:phi:carrier:i") && row.contains("P=binding:i")));
    assert!(digest
        .phis
        .iter()
        .any(|row| row.contains("phi:phi:carrier:sum") && row.contains("P=binding:sum")));
    assert!(digest
        .instructions
        .iter()
        .any(|row| row.contains("compare:Lt")));
    assert!(digest
        .instructions
        .iter()
        .any(|row| row.contains("bin:Add")));
    assert!(digest.results.iter().any(|row| row.contains("final:i:")));
    assert!(digest.results.iter().any(|row| row.contains("final:sum:")));
    assert!(digest.results.iter().any(|row| row == "result:unit:Void"));
    let rendered = format!("{digest:?}");
    assert!(!rendered.contains("BasicBlockId("));
    assert!(!rendered.contains("ValueId("));
}
