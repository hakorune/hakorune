use super::*;
use crate::mir::loop_recipe_contract::{
    LoopBinaryI64OpV1, LoopBindingKeyV1, LoopBlockKeyV1, LoopCompareI64OpV1, LoopConditionV1,
    LoopItemKeyV1, LoopOperationV1, LoopRecipeArtifactV1, LoopRecipeItemV1, LoopRecipeVerifierV1,
    LoopValueKeyV1,
};

#[path = "loop_accum_legacy_oracle_support.rs"]
mod legacy_oracle;

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
    let result = legacy_oracle::lower_accum_legacy_oracle(
        &mut builder,
        &condition,
        &body,
        "accum_semantic_oracle/0",
    )
    .expect("legacy Accum oracle should lower");
    assert!(
        result.is_some(),
        "legacy Accum must produce a terminal value"
    );
}
