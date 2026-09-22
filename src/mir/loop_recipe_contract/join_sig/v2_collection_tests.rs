//! Logical closure tests; these do not authorize a source or physical profile.
use super::*;
use crate::mir::loop_recipe_contract::ids::*;
use crate::mir::loop_recipe_contract::schema_v2::*;
use crate::mir::loop_recipe_contract::typed_schema_v2::LoopRecipeVerifierV2;

fn recipe() -> LoopRecipeV2 {
    let root = LoopNodeKeyV1::new(0);
    let classes = [LoopValueClassV2::I64, LoopValueClassV2::Text];
    LoopRecipeV2 {
        root_loop: root,
        loops: vec![LoopNodeV2 {
            key: root,
            parent: None,
            condition: LoopConditionV2::Predicate {
                block: LoopBlockKeyV1::new(0),
                value: LoopValueKeyV1::new(2),
            },
            body: LoopBlockKeyV1::new(1),
        }],
        blocks: (0..2)
            .map(|index| LoopRecipeBlockV2 {
                key: LoopBlockKeyV1::new(index),
                owner_loop: root,
                items: vec![],
            })
            .collect(),
        items: vec![],
        bindings: classes
            .iter()
            .enumerate()
            .map(|(index, class)| LoopRecipeBindingV2 {
                key: LoopBindingKeyV1::new(index as u32),
                label: "same".into(),
                class: *class,
            })
            .collect(),
        values: classes
            .into_iter()
            .chain([LoopValueClassV2::Bool])
            .enumerate()
            .map(|(index, class)| LoopRecipeValueV2 {
                key: LoopValueKeyV1::new(index as u32),
                class,
            })
            .collect(),
        inputs: (0..3).map(LoopValueKeyV1::new).collect(),
        carriers: classes
            .iter()
            .enumerate()
            .map(|(index, class)| LoopRecipeCarrierV2 {
                key: LoopCarrierKeyV1::new(index as u32),
                owner_loop: root,
                binding: LoopBindingKeyV1::new(index as u32),
                class: *class,
                entry_value: LoopValueKeyV1::new(index as u32),
            })
            .collect(),
        exits: vec![],
    }
}

#[test]
fn collection_retains_each_exact_root_after_without_label_identity() {
    let verified = LoopRecipeVerifierV2::verify(recipe()).expect("typed recipe");
    let closure = issue_root_carrier_join_closure_v2(&verified).expect("all After rows");
    let rows = closure
        .after
        .iter()
        .map(|row| (row.loop_key(), row.binding(), row.class()))
        .collect::<Vec<_>>();
    assert_eq!(
        rows,
        vec![
            (
                LoopNodeKeyV1::new(0),
                LoopBindingKeyV1::new(0),
                LoopValueClassV2::I64
            ),
            (
                LoopNodeKeyV1::new(0),
                LoopBindingKeyV1::new(1),
                LoopValueClassV2::Text
            ),
        ]
    );
    assert!(matches!(
        issue_sole_root_carrier_join_closure_v2(&verified),
        Err(LoopJoinClosureRejectV2::RootCarrierCardinality { found: 2, .. })
    ));
}

#[test]
fn collection_rejects_root_without_after_instead_of_returning_partial_rows() {
    let mut input = recipe();
    input.loops[0].condition = LoopConditionV2::Always;
    input.loops[0].body = LoopBlockKeyV1::new(0);
    input.blocks.truncate(1);
    let verified = LoopRecipeVerifierV2::verify(input).expect("infinite typed recipe");
    assert!(matches!(
        issue_root_carrier_join_closure_v2(&verified),
        Err(LoopJoinClosureRejectV2::JoinSig(
            LoopJoinSigRejectReasonV1::AfterBindingUnavailable { .. }
        ))
    ));
}

#[test]
fn collection_rejects_empty_root_carrier_inventory() {
    let mut input = recipe();
    input.carriers.clear();
    let verified = LoopRecipeVerifierV2::verify(input).expect("typed recipe without carriers");
    assert!(matches!(
        issue_root_carrier_join_closure_v2(&verified),
        Err(LoopJoinClosureRejectV2::RootCarrierCardinality { found: 0, .. })
    ));
}
