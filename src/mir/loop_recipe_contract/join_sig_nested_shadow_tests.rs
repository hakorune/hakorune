use std::collections::BTreeMap;

use super::error::LoopRecipeRejectReasonV1;
use super::ids::{LoopBindingKeyV1, LoopBlockKeyV1, LoopNodeKeyV1, LoopValueKeyV1};
use super::join_sig::{visible_payloads, LoopJoinPayloadV1, LoopJoinSigElaboratorV1};
use super::schema::{
    LoopConditionV1, LoopNodeV1, LoopOperationV1, LoopRecipeCarrierV1, LoopRecipeItemV1,
    LoopRecipeV1, LoopValueClassV1,
};
use super::verify::LoopRecipeVerifierV1;

fn projection_recipe(depth: u32, carriers: &[(u32, u32, u32)]) -> LoopRecipeV1 {
    LoopRecipeV1 {
        root_loop: LoopNodeKeyV1::new(0),
        loops: (0..depth)
            .map(|raw| LoopNodeV1 {
                key: LoopNodeKeyV1::new(raw),
                parent: (raw > 0).then(|| LoopNodeKeyV1::new(raw - 1)),
                condition: LoopConditionV1::Always,
                body: LoopBlockKeyV1::new(raw),
            })
            .collect(),
        blocks: Vec::new(),
        items: Vec::new(),
        bindings: Vec::new(),
        values: Vec::new(),
        inputs: Vec::new(),
        carriers: carriers
            .iter()
            .enumerate()
            .map(
                |(index, (owner, binding, entry_value))| LoopRecipeCarrierV1 {
                    key: super::ids::LoopCarrierKeyV1::new(index as u32),
                    owner_loop: LoopNodeKeyV1::new(*owner),
                    binding: LoopBindingKeyV1::new(*binding),
                    class: LoopValueClassV1::I64,
                    entry_value: LoopValueKeyV1::new(*entry_value),
                },
            )
            .collect(),
        exits: Vec::new(),
    }
}

fn current_values(values: &[(u32, u32)]) -> BTreeMap<LoopBindingKeyV1, LoopValueKeyV1> {
    values
        .iter()
        .map(|(binding, value)| (LoopBindingKeyV1::new(*binding), LoopValueKeyV1::new(*value)))
        .collect()
}

fn binding_values(payloads: &[LoopJoinPayloadV1]) -> Vec<(u32, u32)> {
    payloads
        .iter()
        .map(|payload| (payload.binding.raw(), payload.value.raw()))
        .collect()
}

#[test]
fn visible_payloads_nearest_shadow_is_unique_and_binding_sorted() {
    let recipe = projection_recipe(
        3,
        &[(0, 2, 20), (0, 1, 10), (1, 2, 21), (1, 0, 30), (2, 1, 12)],
    );
    let payloads = visible_payloads(
        &recipe,
        LoopNodeKeyV1::new(2),
        &current_values(&[(0, 31), (1, 41), (2, 51)]),
    )
    .expect("projection succeeds");

    assert_eq!(binding_values(&payloads), vec![(0, 31), (1, 41), (2, 51)]);
}

#[test]
fn visible_payloads_isolates_sibling_carriers() {
    let mut recipe = projection_recipe(3, &[(0, 0, 10), (1, 1, 11), (2, 2, 12)]);
    recipe.loops[2].parent = Some(LoopNodeKeyV1::new(0));
    let payloads = visible_payloads(
        &recipe,
        LoopNodeKeyV1::new(1),
        &current_values(&[(0, 20), (1, 21), (2, 22)]),
    )
    .expect("projection succeeds");

    assert_eq!(binding_values(&payloads), vec![(0, 20), (1, 21)]);
}

#[test]
fn verified_nested_same_binding_publishes_one_child_row() {
    let mut recipe: super::schema::LoopRecipeArtifactV1 =
        serde_json::from_str(include_str!("fixtures/nested_predicate_v1.json"))
            .expect("nested fixture");
    for row in &mut recipe.recipe.items {
        if let LoopRecipeItemV1::Operation { operation } = &mut row.item {
            match operation {
                LoopOperationV1::ReadBinding { binding, .. }
                | LoopOperationV1::WriteBinding { binding, .. }
                    if *binding == LoopBindingKeyV1::new(2) =>
                {
                    *binding = LoopBindingKeyV1::new(1);
                }
                _ => {}
            }
        }
    }
    recipe.recipe.carriers[2].binding = LoopBindingKeyV1::new(1);

    let verified = LoopRecipeVerifierV1::verify(recipe.recipe).expect("same-binding recipe");
    let signature = LoopJoinSigElaboratorV1::elaborate(&verified).expect("join signature");
    let child = signature
        .as_sig()
        .loops
        .iter()
        .find(|row| row.key == LoopNodeKeyV1::new(1))
        .expect("child row");

    assert!(!child.edges.is_empty());
    for edge in &child.edges {
        let rows = edge
            .payload
            .iter()
            .filter(|payload| payload.binding == LoopBindingKeyV1::new(1))
            .count();
        assert_eq!(rows, 1, "edge {:?}", edge.role);
    }
}

#[test]
fn verifier_rejects_unknown_carrier_owner_before_joinsig() {
    let mut artifact: super::schema::LoopRecipeArtifactV1 =
        serde_json::from_str(include_str!("fixtures/nested_predicate_v1.json"))
            .expect("nested fixture");
    artifact.recipe.carriers[0].owner_loop = LoopNodeKeyV1::new(99);

    assert_eq!(
        LoopRecipeVerifierV1::verify(artifact.recipe).expect_err("unknown owner must be rejected"),
        LoopRecipeRejectReasonV1::DanglingLoop {
            key: LoopNodeKeyV1::new(99),
        }
    );
}

#[test]
fn verifier_rejects_duplicate_same_owner_carrier() {
    let mut artifact: super::schema::LoopRecipeArtifactV1 =
        serde_json::from_str(include_str!("fixtures/nested_predicate_v1.json"))
            .expect("nested fixture");
    artifact.recipe.carriers[1].binding = LoopBindingKeyV1::new(0);

    assert_eq!(
        LoopRecipeVerifierV1::verify(artifact.recipe)
            .expect_err("same-owner duplicate must be rejected"),
        LoopRecipeRejectReasonV1::DuplicateCarrierBinding {
            loop_key: LoopNodeKeyV1::new(0),
            binding: LoopBindingKeyV1::new(0),
        }
    );
}
