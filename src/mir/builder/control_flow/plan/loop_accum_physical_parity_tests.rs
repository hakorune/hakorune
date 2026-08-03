//! Test-only physical MIR observation for the DirectAccum P4-S0 seam.
//!
//! This module observes the existing legacy PlanLowerer output.  It does not
//! lower recipe operations, insert PHIs, or wire a production caller.  The
//! future physicalizer will consume the same comparison-only snapshot shape.

#![cfg(test)]

use crate::mir::builder::control_flow::plan::{CoreEffectPlan, CorePlan, CoreLoopPlan};
use crate::mir::{BasicBlockId, MirFunction, MirInstruction, MirType, ValueId};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
struct MirPhysicalAlphaDigestV1 {
    cfg: Box<[String]>,
    instructions: Box<[String]>,
    phis: Box<[String]>,
    results: Box<[String]>,
}

#[derive(Default)]
struct ValueLabels {
    labels: BTreeMap<ValueId, String>,
    next_temp: usize,
}

impl ValueLabels {
    fn set(&mut self, value: ValueId, label: impl Into<String>) {
        self.labels.entry(value).or_insert_with(|| label.into());
    }

    fn get(&mut self, value: ValueId) -> String {
        if let Some(label) = self.labels.get(&value) {
            return label.clone();
        }
        let label = format!("tmp{}", self.next_temp);
        self.next_temp += 1;
        self.labels.insert(value, label.clone());
        label
    }
}

fn role_map(plan: &CoreLoopPlan) -> BTreeMap<BasicBlockId, &'static str> {
    let mut roles = [
        (plan.preheader_bb, "P"),
        (plan.header_bb, "H"),
        (plan.body_bb, "B"),
        (plan.step_bb, "S"),
        (plan.after_bb, "A"),
    ]
    .into_iter()
    .collect::<BTreeMap<_, _>>();
    if plan.found_bb != plan.after_bb {
        roles.insert(plan.found_bb, "F");
    }
    roles
}

fn role_for(roles: &BTreeMap<BasicBlockId, &'static str>, block: BasicBlockId) -> String {
    roles
        .get(&block)
        .copied()
        .unwrap_or("?")
        .to_owned()
}

fn normalize_phi_tag(tag: &str) -> String {
    tag.strip_prefix("loop_v0_")
        .unwrap_or(tag)
        .replace("_", ":")
}

fn seed_effect(effect: &CoreEffectPlan, labels: &mut ValueLabels) {
    match effect {
        CoreEffectPlan::Const { dst, value } => {
            labels.set(*dst, format!("const:{value:?}"));
        }
        CoreEffectPlan::Compare { dst, lhs, op, rhs } => {
            let left = labels.get(*lhs);
            let right = labels.get(*rhs);
            labels.set(*dst, format!("compare:{op:?}:{left}:{right}"));
        }
        CoreEffectPlan::BinOp { dst, lhs, op, rhs } => {
            let left = labels.get(*lhs);
            let right = labels.get(*rhs);
            labels.set(*dst, format!("bin:{op:?}:{left}:{right}"));
        }
        other => panic!("unexpected direct Accum effect: {other:?}"),
    }
}

fn seed_plan(plan: &CoreLoopPlan) -> ValueLabels {
    let mut labels = ValueLabels::default();
    for phi in &plan.phis {
        let phi_label = normalize_phi_tag(&phi.tag);
        labels.set(phi.dst, format!("phi:{phi_label}"));
        if let Some((_, value)) = phi.inputs.first() {
            if phi.tag.ends_with("carrier_i") {
                labels.set(*value, "binding:i");
            } else if phi.tag.ends_with("carrier_sum") {
                labels.set(*value, "binding:sum");
            }
        }
    }
    for (_, effects) in &plan.block_effects {
        for effect in effects {
            seed_effect(effect, &mut labels);
        }
    }
    for body in &plan.body {
        if let CorePlan::Effect(effect) = body {
            seed_effect(effect, &mut labels);
        }
    }
    labels
}

fn instruction_row(
    role: &str,
    instruction: &MirInstruction,
    roles: &BTreeMap<BasicBlockId, &'static str>,
    labels: &mut ValueLabels,
) -> String {
    match instruction {
        MirInstruction::Phi { dst, inputs, .. } => {
            let inputs = inputs
                .iter()
                .map(|(block, value)| format!("{}={}", role_for(roles, *block), labels.get(*value)))
                .collect::<Vec<_>>()
                .join(",");
            format!("{role}:phi:{}=[{inputs}]", labels.get(*dst))
        }
        MirInstruction::Const { dst, value } => {
            labels.set(*dst, format!("const:{value:?}"));
            format!("{role}:const:{}={value:?}", labels.get(*dst))
        }
        MirInstruction::Copy { dst, src } => {
            let source = labels.get(*src);
            labels.set(*dst, source.clone());
            format!("{role}:copy:{source}")
        }
        MirInstruction::BinOp { dst, op, lhs, rhs } => {
            let left = labels.get(*lhs);
            let right = labels.get(*rhs);
            let expression = format!("bin:{op:?}:{left}:{right}");
            labels.set(*dst, expression.clone());
            format!("{role}:{expression}")
        }
        MirInstruction::Compare { dst, op, lhs, rhs } => {
            let left = labels.get(*lhs);
            let right = labels.get(*rhs);
            let expression = format!("compare:{op:?}:{left}:{right}");
            labels.set(*dst, expression.clone());
            format!("{role}:{expression}")
        }
        MirInstruction::KeepAlive { values } => format!(
            "{role}:keepalive:{}",
            values
                .iter()
                .map(|value| labels.get(*value))
                .collect::<Vec<_>>()
                .join(",")
        ),
        other => panic!("unexpected direct Accum MIR instruction: {other:?}"),
    }
}

fn terminator_row(
    role: &str,
    terminator: Option<&MirInstruction>,
    roles: &BTreeMap<BasicBlockId, &'static str>,
    labels: &mut ValueLabels,
) -> String {
    match terminator {
        Some(MirInstruction::Branch {
            condition,
            then_bb,
            else_bb,
            ..
        }) => format!(
            "{role}:branch:{}:{}:{}",
            labels.get(*condition),
            role_for(roles, *then_bb),
            role_for(roles, *else_bb)
        ),
        Some(MirInstruction::Jump { target, .. }) => {
            format!("{role}:jump:{}", role_for(roles, *target))
        }
        Some(MirInstruction::Return { value }) => format!(
            "{role}:return:{}",
            value.map(|value| labels.get(value)).unwrap_or_else(|| "unit".to_owned())
        ),
        None => format!("{role}:open"),
        other => panic!("unexpected direct Accum terminator: {other:?}"),
    }
}

fn snapshot(
    function: &MirFunction,
    plan: &CoreLoopPlan,
    result: Option<ValueId>,
    value_types: &BTreeMap<ValueId, MirType>,
) -> MirPhysicalAlphaDigestV1 {
    let roles = role_map(plan);
    let mut labels = seed_plan(plan);
    if let Some(result) = result {
        labels.set(result, "unit");
    }
    let mut cfg = Vec::new();
    let mut instructions = Vec::new();
    let mut phis = Vec::new();
    for (block, role) in roles.iter() {
        let block_data = function.blocks.get(block).expect("legacy loop block");
        let predecessors = block_data
            .predecessors
            .iter()
            .map(|pred| role_for(&roles, *pred))
            .collect::<Vec<_>>()
            .join(",");
        let successors = block_data
            .successors
            .iter()
            .map(|succ| role_for(&roles, *succ))
            .collect::<Vec<_>>()
            .join(",");
        let terminator = terminator_row(role, block_data.terminator.as_ref(), &roles, &mut labels);
        cfg.push(format!("{role}:pred=[{predecessors}]:succ=[{successors}]:{terminator}"));
        for instruction in &block_data.instructions {
            let row = instruction_row(role, instruction, &roles, &mut labels);
            if matches!(instruction, MirInstruction::Phi { .. }) {
                phis.push(row.clone());
            }
            instructions.push(row);
        }
    }
    let mut results = plan
        .final_values
        .iter()
        .map(|(name, value)| {
            let ty = value_types
                .get(value)
                .map(|ty| format!("{ty:?}"))
                .unwrap_or_else(|| "Unknown".to_owned());
            format!("final:{name}:{}:{ty}", labels.get(*value))
        })
        .collect::<Vec<_>>();
    if let Some(result) = result {
        let ty = value_types
            .get(&result)
            .map(|ty| format!("{ty:?}"))
            .unwrap_or_else(|| "Unknown".to_owned());
        results.push(format!("result:{}:{ty}", labels.get(result)));
    }
    MirPhysicalAlphaDigestV1 {
        cfg: cfg.into_boxed_slice(),
        instructions: instructions.into_boxed_slice(),
        phis: phis.into_boxed_slice(),
        results: results.into_boxed_slice(),
    }
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
    let value_types = loop_plan
        .final_values
        .iter()
        .map(|(_, value)| *value)
        .chain(result)
        .filter_map(|value| builder.value_type(value).cloned().map(|ty| (value, ty)))
        .collect::<BTreeMap<_, _>>();
    let digest = snapshot(function, &loop_plan, result, &value_types);

    assert!(digest.cfg.iter().any(|row| row.contains("P:pred=[]:succ=[H]")));
    assert!(digest.cfg.iter().any(|row| row.contains("B:pred=[]:succ=[S]")));
    assert!(digest.cfg.iter().any(|row| row.contains("S:pred=[B]:succ=[H]")));
    assert!(digest
        .phis
        .iter()
        .any(|row| row.contains("phi:phi:carrier:i") && row.contains("P=binding:i")));
    assert!(digest
        .phis
        .iter()
        .any(|row| row.contains("phi:phi:carrier:sum") && row.contains("P=binding:sum")));
    assert!(digest.instructions.iter().any(|row| row.contains("compare:Lt")));
    assert!(digest.instructions.iter().any(|row| row.contains("bin:Add")));
    assert!(digest.results.iter().any(|row| row.contains("final:i:")));
    assert!(digest.results.iter().any(|row| row.contains("final:sum:")));
    assert!(digest.results.iter().any(|row| row == "result:unit:Void"));
    let rendered = format!("{digest:?}");
    assert!(!rendered.contains("BasicBlockId("));
    assert!(!rendered.contains("ValueId("));

    let sig = super::super::direct_verified_sig();
    let map = super::super::VerifiedLoopLogicalToPhysicalMapV1::try_new(
        &sig,
        super::super::direct_map_input(&sig),
    )
    .expect("direct structural witness");
    let structural = super::super::alpha_normalized_direct_digest(&sig, &map);
    assert!(structural.contains("edge=Backedge"));
    assert!(structural.contains("len3:terminal1"));
}
