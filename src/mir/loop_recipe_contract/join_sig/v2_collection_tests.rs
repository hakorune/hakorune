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
    let view = closure
        .logical_transfer_view()
        .expect("complete collection view");
    assert_eq!(
        view.afters()
            .iter()
            .map(|row| (row.loop_key(), row.binding(), row.class()))
            .collect::<Vec<_>>(),
        rows
    );
    assert_eq!(
        view.boundaries()
            .iter()
            .filter(|row| row.role == super::super::model::LoopJoinEdgeRoleV1::PredicateFalse)
            .count(),
        1
    );
    assert!(view.branches().is_empty());
    assert!(view.summary_transfers().is_empty());
    assert!(matches!(
        issue_sole_root_carrier_join_closure_v2(&verified),
        Err(LoopJoinClosureRejectV2::RootCarrierCardinality { found: 2, .. })
    ));
}

#[test]
fn collection_transfer_retains_zero_trip_and_every_break_payload() {
    use super::super::model::{LoopJoinEdgeRoleV1 as Role, LoopJoinPortV1 as Port};
    use super::super::transfer_view_v2::LoopJoinBranchArmTransferRefV2 as Arm;
    let mut input = recipe();
    let root = input.root_loop;
    for class in [
        LoopValueClassV2::Text,
        LoopValueClassV2::I64,
        LoopValueClassV2::Bool,
        LoopValueClassV2::I64,
    ] {
        let key = LoopValueKeyV1::new(input.values.len() as u32);
        input.values.push(LoopRecipeValueV2 { key, class });
        input.inputs.push(key);
    }
    input.values.push(LoopRecipeValueV2 {
        key: LoopValueKeyV1::new(7),
        class: LoopValueClassV2::I64,
    });
    input.blocks[1].items = [0, 1, 2, 4, 7]
        .into_iter()
        .map(LoopItemKeyV1::new)
        .collect();
    for (block, items) in [(2, vec![3]), (3, vec![5, 6])] {
        input.blocks.push(LoopRecipeBlockV2 {
            key: LoopBlockKeyV1::new(block),
            owner_loop: root,
            items: items.into_iter().map(LoopItemKeyV1::new).collect(),
        });
    }
    let operation = |operation| LoopRecipeItemV2::Operation { operation };
    input.items = vec![
        operation(LoopOperationV2::WriteBinding {
            binding: LoopBindingKeyV1::new(0),
            value: LoopValueKeyV1::new(4),
        }),
        operation(LoopOperationV2::WriteBinding {
            binding: LoopBindingKeyV1::new(1),
            value: LoopValueKeyV1::new(3),
        }),
        LoopRecipeItemV2::If {
            condition: LoopValueKeyV1::new(5),
            then_block: LoopBlockKeyV1::new(2),
            else_block: None,
        },
        LoopRecipeItemV2::Exit {
            exit: LoopExitKeyV1::new(0),
        },
        LoopRecipeItemV2::If {
            condition: LoopValueKeyV1::new(2),
            then_block: LoopBlockKeyV1::new(3),
            else_block: None,
        },
        operation(LoopOperationV2::WriteBinding {
            binding: LoopBindingKeyV1::new(0),
            value: LoopValueKeyV1::new(6),
        }),
        LoopRecipeItemV2::Exit {
            exit: LoopExitKeyV1::new(1),
        },
        // Keep a real continuation for each implicit else. A terminal If without
        // an exact continuation remains outside the existing transfer contract.
        operation(LoopOperationV2::ReadBinding {
            binding: LoopBindingKeyV1::new(0),
            result: LoopValueKeyV1::new(7),
        }),
    ]
    .into_iter()
    .enumerate()
    .map(|(key, item)| LoopRecipeItemRowV2 {
        key: LoopItemKeyV1::new(key as u32),
        item,
    })
    .collect();
    input.exits = (0..2)
        .map(|key| LoopRecipeExitV2 {
            key: LoopExitKeyV1::new(key),
            owner_loop: root,
            kind: LoopExitKindV2::Break { target_loop: root },
        })
        .collect();
    let verified =
        LoopRecipeVerifierV2::verify(input).expect("mixed carrier source-independent recipe");
    let closure = issue_root_carrier_join_closure_v2(&verified).expect("complete mixed After");
    let view = closure
        .logical_transfer_view()
        .expect("shared branch validation");
    assert_eq!(view.afters().len(), 2);
    let zero_trip = view
        .boundaries()
        .iter()
        .find(|row| row.role == Role::PredicateFalse)
        .unwrap();
    assert_eq!((zero_trip.loop_key, zero_trip.to), (root, Port::After));
    assert_eq!(
        zero_trip
            .payload
            .iter()
            .map(|row| row.value)
            .collect::<Vec<_>>(),
        vec![LoopValueKeyV1::new(0), LoopValueKeyV1::new(1)]
    );
    assert_eq!(view.summary_transfers().len(), 2);
    assert_eq!(view.branches().len(), 2);
    for (item, exit_item, expected) in [(2, 3, 4), (4, 6, 6)] {
        let branch = view
            .branches()
            .iter()
            .find(|row| row.if_item == LoopItemKeyV1::new(item))
            .unwrap();
        let Arm::Exit(exit) = branch.then_arm else {
            panic!("original then Break")
        };
        assert_eq!(exit.exit_item, LoopItemKeyV1::new(exit_item));
        assert!(matches!(branch.else_arm, Arm::Fallthrough { .. }));
        assert_eq!(
            exit.payload.iter().map(|row| row.value).collect::<Vec<_>>(),
            vec![LoopValueKeyV1::new(expected), LoopValueKeyV1::new(3)]
        );
        assert_eq!(
            view.summary_transfers()
                .iter()
                .filter(|summary| summary.loop_key == root
                    && summary.role == Role::Break
                    && summary.to == Port::After
                    && summary.payload == exit.payload)
                .count(),
            1
        );
    }
    for mutation in 0..5 {
        let mut changed = issue_root_carrier_join_closure_v2(&verified).unwrap();
        let mut signature = changed.join_sig.as_sig().clone();
        let edges = &mut signature.loops[0].edges;
        let indices = edges
            .iter()
            .enumerate()
            .filter_map(|(index, row)| (row.role == Role::Break).then_some(index))
            .collect::<Vec<_>>();
        let index = indices[0];
        match mutation {
            0 => {
                edges.remove(index);
            }
            1 => {
                edges[index].payload.pop();
            }
            2 => {
                signature.branches.clear();
            }
            3 => {
                edges[index].role = Role::Continue;
            }
            4 => {
                edges[index].payload = edges[indices[1]].payload.clone();
            }
            _ => unreachable!(),
        }
        changed.join_sig = super::super::model::VerifiedLoopJoinSigV2::from_sig(signature);
        assert!(
            changed.logical_transfer_view().is_err(),
            "collection branch mutation {mutation}"
        );
    }
}

#[test]
fn collection_transfer_rejects_missing_duplicate_and_foreign_after_rows() {
    let verified = LoopRecipeVerifierV2::verify(recipe()).unwrap();
    for mutation in 0..3 {
        let mut closure = issue_root_carrier_join_closure_v2(&verified).unwrap();
        let mut rows = closure.after.into_vec();
        match mutation {
            0 => {
                rows.pop();
            }
            1 => {
                let mut duplicate = issue_root_carrier_join_closure_v2(&verified)
                    .unwrap()
                    .after
                    .into_vec();
                rows[1] = duplicate.remove(0);
            }
            2 => {
                let mut foreign = recipe();
                foreign.bindings[1].class = LoopValueClassV2::I64;
                foreign.values[1].class = LoopValueClassV2::I64;
                foreign.carriers[1].class = LoopValueClassV2::I64;
                let foreign = LoopRecipeVerifierV2::verify(foreign).unwrap();
                rows[1] = issue_root_carrier_join_closure_v2(&foreign)
                    .unwrap()
                    .after
                    .into_vec()
                    .remove(1);
            }
            _ => unreachable!(),
        }
        closure.after = rows.into_boxed_slice();
        assert!(matches!(closure.logical_transfer_view(), Err(
            super::super::transfer_view_v2::LoopJoinLogicalTransferRejectV2::AfterCollectionCoverage)), "After mutation {mutation}");
    }
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
