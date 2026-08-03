use super::*;
use crate::mir::builder::control_flow::plan::{CoreEffectPlan, CorePlan};
use crate::mir::loop_recipe_contract::{
    LoopBinaryI64OpV1, LoopBindingKeyV1, LoopBlockKeyV1, LoopCompareI64OpV1, LoopConditionV1,
    LoopItemKeyV1, LoopOperationV1, LoopRecipeArtifactV1, LoopRecipeItemV1, LoopRecipeVerifierV1,
    LoopValueKeyV1,
};
use crate::mir::{BinaryOp, CompareOp, ConstValue, ValueId};
use std::collections::BTreeMap;

#[path = "loop_accum_legacy_oracle_support.rs"]
mod legacy_oracle;

#[path = "loop_accum_physical_parity_tests.rs"]
mod physical_parity_tests;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ProjectedScalar {
    I64(i64),
    Bool(bool),
}

fn block<'a>(
    recipe: &'a crate::mir::loop_recipe_contract::LoopRecipeV1,
    key: LoopBlockKeyV1,
) -> &'a crate::mir::loop_recipe_contract::LoopRecipeBlockV1 {
    recipe
        .blocks
        .iter()
        .find(|candidate| candidate.key == key)
        .expect("recipe block")
}

fn item<'a>(
    recipe: &'a crate::mir::loop_recipe_contract::LoopRecipeV1,
    key: LoopItemKeyV1,
) -> &'a LoopRecipeItemV1 {
    &recipe
        .items
        .iter()
        .find(|candidate| candidate.key == key)
        .expect("recipe item")
        .item
}

fn i64_value(
    values: &std::collections::BTreeMap<LoopValueKeyV1, ProjectedScalar>,
    key: LoopValueKeyV1,
) -> i64 {
    match values.get(&key).copied().expect("defined value") {
        ProjectedScalar::I64(value) => value,
        ProjectedScalar::Bool(_) => panic!("expected i64 value {key:?}"),
    }
}

fn execute_block(
    recipe: &crate::mir::loop_recipe_contract::LoopRecipeV1,
    key: LoopBlockKeyV1,
    values: &mut std::collections::BTreeMap<LoopValueKeyV1, ProjectedScalar>,
    bindings: &mut std::collections::BTreeMap<LoopBindingKeyV1, i64>,
    reads: &mut Vec<(LoopBindingKeyV1, LoopValueKeyV1)>,
) {
    for item_key in &block(recipe, key).items {
        let LoopRecipeItemV1::Operation { operation } = item(recipe, *item_key) else {
            panic!("direct semantic fixture contains only operations")
        };
        match *operation {
            LoopOperationV1::ReadBinding { binding, result } => {
                let value = *bindings.get(&binding).expect("binding source");
                reads.push((binding, result));
                values.insert(result, ProjectedScalar::I64(value));
            }
            LoopOperationV1::ConstI64 { result, value } => {
                values.insert(result, ProjectedScalar::I64(value));
            }
            LoopOperationV1::BinaryI64 {
                op,
                left,
                right,
                result: result_key,
            } => {
                let left = i64_value(values, left);
                let right = i64_value(values, right);
                let result_value = match op {
                    LoopBinaryI64OpV1::Add => left + right,
                    LoopBinaryI64OpV1::Sub => left - right,
                };
                values.insert(result_key, ProjectedScalar::I64(result_value));
            }
            LoopOperationV1::CompareI64 {
                op,
                left,
                right,
                result: result_key,
            } => {
                let left = i64_value(values, left);
                let right = i64_value(values, right);
                let result_value = match op {
                    LoopCompareI64OpV1::Less => left < right,
                    LoopCompareI64OpV1::LessEqual => left <= right,
                    LoopCompareI64OpV1::Equal => left == right,
                };
                values.insert(result_key, ProjectedScalar::Bool(result_value));
            }
            LoopOperationV1::WriteBinding { binding, value } => {
                bindings.insert(binding, i64_value(values, value));
            }
        }
    }
}

fn portable_semantic_digest() -> String {
    let artifact: LoopRecipeArtifactV1 =
        serde_json::from_str(super::DIRECT_GOLDEN).expect("direct semantic golden");
    let verified =
        LoopRecipeVerifierV1::verify(artifact.recipe().clone()).expect("verified recipe");
    let recipe = verified.as_recipe();
    let root = recipe
        .loops
        .iter()
        .find(|loop_row| loop_row.key == recipe.root_loop)
        .expect("root loop");
    let condition_block = match root.condition {
        LoopConditionV1::Predicate { block, .. } => block,
        LoopConditionV1::Always => panic!("direct fixture must be predicate"),
    };
    let mut values = BTreeMap::<LoopValueKeyV1, String>::new();
    let mut bindings = BTreeMap::<LoopBindingKeyV1, String>::from([
        (LoopBindingKeyV1::new(0), "b0".to_owned()),
        (LoopBindingKeyV1::new(1), "b1".to_owned()),
    ]);
    for carrier in &recipe.carriers {
        values.insert(carrier.entry_value, format!("b{}", carrier.binding.raw()));
    }
    let mut condition_lines = Vec::new();
    project_block(
        recipe,
        condition_block,
        &mut values,
        &mut bindings,
        &mut condition_lines,
    );
    let mut body_lines = Vec::new();
    project_block(
        recipe,
        root.body,
        &mut values,
        &mut bindings,
        &mut body_lines,
    );
    format!(
        "condition={:?};body={:?};final:i={};sum={}",
        condition_lines,
        body_lines,
        bindings[&LoopBindingKeyV1::new(0)],
        bindings[&LoopBindingKeyV1::new(1)]
    )
}

fn project_block(
    recipe: &crate::mir::loop_recipe_contract::LoopRecipeV1,
    key: LoopBlockKeyV1,
    values: &mut BTreeMap<LoopValueKeyV1, String>,
    bindings: &mut BTreeMap<LoopBindingKeyV1, String>,
    lines: &mut Vec<String>,
) {
    for item_key in &block(recipe, key).items {
        let LoopRecipeItemV1::Operation { operation } = item(recipe, *item_key) else {
            panic!("direct semantic fixture contains only operations")
        };
        match *operation {
            LoopOperationV1::ReadBinding { binding, result } => {
                values.insert(result, bindings[&binding].clone());
            }
            LoopOperationV1::ConstI64 { result, value } => {
                let label = format!("c{value}");
                values.insert(result, label.clone());
                lines.push(format!("const:{value}"));
            }
            LoopOperationV1::BinaryI64 {
                op,
                left,
                right,
                result,
            } => {
                let left = values[&left].clone();
                let right = values[&right].clone();
                let op = match op {
                    LoopBinaryI64OpV1::Add => "add",
                    LoopBinaryI64OpV1::Sub => "sub",
                };
                let label = format!("{op}:{left}:{right}");
                lines.push(format!("bin:{label}"));
                values.insert(result, label);
            }
            LoopOperationV1::CompareI64 {
                op: LoopCompareI64OpV1::Less,
                left,
                right,
                result,
            } => {
                let left = values[&left].clone();
                let right = values[&right].clone();
                values.insert(result, "predicate".to_owned());
                lines.push(format!("cmp:less:{left}:{right}"));
            }
            LoopOperationV1::CompareI64 { .. } => panic!("unexpected direct compare"),
            LoopOperationV1::WriteBinding { binding, value } => {
                bindings.insert(binding, values[&value].clone());
            }
        }
    }
}

fn legacy_semantic_digest(plan: &CorePlan) -> String {
    let CorePlan::Loop(loop_plan) = plan else {
        panic!("legacy Accum plan must be a loop")
    };
    let carrier_i = loop_plan
        .phis
        .iter()
        .find(|phi| phi.tag == "loop_v0_carrier_i")
        .expect("legacy i carrier");
    let carrier_sum = loop_plan
        .phis
        .iter()
        .find(|phi| phi.tag == "loop_v0_carrier_sum")
        .expect("legacy sum carrier");
    let mut values = BTreeMap::<ValueId, String>::from([
        (carrier_i.dst, "b0".to_owned()),
        (carrier_sum.dst, "b1".to_owned()),
    ]);
    let mut condition_lines = Vec::new();
    for (block, effects) in &loop_plan.block_effects {
        if *block != loop_plan.header_bb {
            continue;
        }
        for effect in effects {
            legacy_effect(effect, &mut values, &mut condition_lines, "condition");
        }
    }
    let mut body_lines = Vec::new();
    for body_plan in &loop_plan.body {
        if let CorePlan::Effect(effect) = body_plan {
            legacy_effect(effect, &mut values, &mut body_lines, "body");
        }
    }
    let after_i = loop_plan
        .phis
        .iter()
        .find(|phi| phi.tag == "loop_v0_after_i")
        .expect("legacy i after PHI");
    let after_sum = loop_plan
        .phis
        .iter()
        .find(|phi| phi.tag == "loop_v0_after_sum")
        .expect("legacy sum after PHI");
    let i_final = loop_plan
        .final_values
        .iter()
        .find(|(name, _)| name == "i")
        .expect("legacy i final")
        .1;
    let sum_final = loop_plan
        .final_values
        .iter()
        .find(|(name, _)| name == "sum")
        .expect("legacy sum final")
        .1;
    values.insert(after_i.dst, "add:b0:c1".to_owned());
    values.insert(after_sum.dst, "add:b1:c1".to_owned());
    assert_eq!(values[&i_final], "add:b0:c1");
    assert_eq!(values[&sum_final], "add:b1:c1");
    format!(
        "condition={:?};body={:?};final:i={};sum={}",
        condition_lines, body_lines, values[&i_final], values[&sum_final]
    )
}

fn legacy_effect(
    effect: &CoreEffectPlan,
    values: &mut BTreeMap<ValueId, String>,
    lines: &mut Vec<String>,
    phase: &str,
) {
    match effect {
        CoreEffectPlan::Const {
            dst,
            value: ConstValue::Integer(value),
        } => {
            values.insert(*dst, format!("c{value}"));
            lines.push(format!("const:{value}"));
        }
        CoreEffectPlan::Compare {
            dst,
            lhs,
            op: CompareOp::Lt,
            rhs,
        } => {
            let left = values[lhs].clone();
            let right = values[rhs].clone();
            values.insert(*dst, "predicate".to_owned());
            assert_eq!(phase, "condition");
            lines.push(format!("cmp:less:{left}:{right}"));
        }
        CoreEffectPlan::BinOp {
            dst,
            lhs,
            op: BinaryOp::Add,
            rhs,
        } => {
            let left = values[lhs].clone();
            let right = values[rhs].clone();
            let label = format!("add:{left}:{right}");
            values.insert(*dst, label.clone());
            lines.push(format!("bin:{label}"));
        }
        other => panic!("unexpected legacy Accum effect in {phase}: {other:?}"),
    }
}

#[test]
fn direct_readbinding_projection_reaches_final_carriers() {
    let artifact: LoopRecipeArtifactV1 =
        serde_json::from_str(super::DIRECT_GOLDEN).expect("direct semantic golden");
    let verified =
        LoopRecipeVerifierV1::verify(artifact.recipe().clone()).expect("verified recipe");
    let recipe = verified.as_recipe();
    let root = recipe
        .loops
        .iter()
        .find(|loop_row| loop_row.key == recipe.root_loop)
        .expect("root loop");
    let condition = match root.condition {
        LoopConditionV1::Predicate { block, value } => (block, value),
        LoopConditionV1::Always => panic!("direct fixture must be predicate"),
    };
    let mut values = std::collections::BTreeMap::new();
    let mut bindings = std::collections::BTreeMap::from([
        (LoopBindingKeyV1::new(0), 0),
        (LoopBindingKeyV1::new(1), 0),
    ]);
    let mut reads = Vec::new();
    let mut iterations = 0;
    while {
        execute_block(recipe, condition.0, &mut values, &mut bindings, &mut reads);
        matches!(values.get(&condition.1), Some(ProjectedScalar::Bool(true)))
    } {
        execute_block(recipe, root.body, &mut values, &mut bindings, &mut reads);
        iterations += 1;
        assert!(iterations <= 3, "direct fixture did not converge");
    }
    assert_eq!(iterations, 3);
    assert_eq!(bindings[&LoopBindingKeyV1::new(0)], 3);
    assert_eq!(bindings[&LoopBindingKeyV1::new(1)], 3);
    assert_eq!(reads.len(), 10);
    assert_eq!(
        reads
            .iter()
            .filter(|(binding, result)| {
                *binding == LoopBindingKeyV1::new(0) && *result == LoopValueKeyV1::new(2)
            })
            .count(),
        4
    );
    assert_eq!(
        reads
            .iter()
            .filter(|(binding, result)| {
                *binding == LoopBindingKeyV1::new(1) && *result == LoopValueKeyV1::new(5)
            })
            .count(),
        3
    );
}

#[test]
fn direct_legacy_oracle_accepts_equivalent_source() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("accum_semantic_oracle/0".to_owned());
    let i = builder.alloc_typed(crate::mir::MirType::Integer);
    let sum = builder.alloc_typed(crate::mir::MirType::Integer);
    builder.bind_variable_for_test("i", i);
    builder.bind_variable_for_test("sum", sum);
    let _scope = crate::mir::builder::vars::lexical_scope::LexicalScopeGuard::new(&mut builder);
    let (condition, body) = legacy_oracle::direct_accum_source();
    let plan = legacy_oracle::prepare_accum_legacy_plan(
        &mut builder,
        &condition,
        &body,
        "accum_semantic_oracle/0",
    )
    .expect("legacy Accum oracle should compose");
    assert_eq!(portable_semantic_digest(), legacy_semantic_digest(&plan));
    let result = crate::mir::builder::control_flow::lower::PlanLowerer::lower(
        &mut builder,
        plan,
        &crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext::new(
            &condition,
            &body,
            "accum_semantic_oracle/0",
            false,
            false,
        ),
    )
    .expect("legacy Accum oracle should lower");
    assert!(
        result.is_some(),
        "legacy Accum must produce a terminal value"
    );
}
