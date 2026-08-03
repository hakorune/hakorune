//! Caller-zero P1-S0 products for the DirectAccum physicalizer seam.
//!
//! These tests stop at role planning, candidate-only reservation, and the
//! existing PHI transaction boundary.  They intentionally do not emit Loop
//! operations or connect a production route.

#![cfg(test)]

use super::*;
use crate::mir::builder::control_flow::plan::loop_phi_materializer::LoopPhiMaterializationHandleV1;
use crate::mir::builder::control_flow::plan::loop_phi_materializer_test_support::{bb, standard5_builder};
use crate::mir::loop_recipe_contract::{
    LoopJoinEdgeRoleV1, LoopItemKeyV1, LoopJoinSigElaboratorV1, LoopOperationV1,
    LoopRecipeArtifactV1, LoopRecipeItemV1, LoopRecipeVerifierV1, VerifiedLoopJoinSigV1,
};
use crate::mir::{BasicBlockId, MirType, ValueId};
use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum PhysicalBlockRoleV1 {
    Preheader,
    Header,
    Body,
    Step,
    After,
}

#[derive(Debug, PartialEq, Eq)]
struct PhysicalPathRoleV1 {
    edge: LoopJoinEdgeRoleV1,
    roles: Box<[PhysicalBlockRoleV1]>,
}

#[derive(Debug, PartialEq, Eq)]
struct PhysicalRolePlanV1 {
    loop_key: crate::mir::loop_recipe_contract::LoopNodeKeyV1,
    paths: Box<[PhysicalPathRoleV1]>,
    operation_keys: Box<[LoopItemKeyV1]>,
}

#[derive(Debug, PartialEq, Eq)]
struct PhysicalAllocationV1 {
    blocks: BTreeMap<PhysicalBlockRoleV1, BasicBlockId>,
    operation_results: BTreeMap<LoopItemKeyV1, ValueId>,
}

fn direct_verified_recipe() -> LoopRecipeArtifactV1 {
    serde_json::from_str(super::super::DIRECT_GOLDEN).expect("direct recipe golden")
}

fn direct_role_plan(
    sig: &VerifiedLoopJoinSigV1,
    artifact: &LoopRecipeArtifactV1,
) -> PhysicalRolePlanV1 {
    let recipe = artifact.recipe();
    let root = recipe
        .loops
        .iter()
        .find(|row| row.key == recipe.root_loop)
        .expect("direct root loop");
    let paths = sig
        .as_sig()
        .loops
        .iter()
        .find(|row| row.key == root.key)
        .expect("direct JoinSig row")
        .edges
        .iter()
        .map(|edge| {
            let roles = match edge.role {
                LoopJoinEdgeRoleV1::Enter => vec![PhysicalBlockRoleV1::Preheader, PhysicalBlockRoleV1::Header],
                LoopJoinEdgeRoleV1::PredicateTrue => vec![PhysicalBlockRoleV1::Header, PhysicalBlockRoleV1::Body],
                LoopJoinEdgeRoleV1::PredicateFalse => vec![PhysicalBlockRoleV1::Header, PhysicalBlockRoleV1::After],
                LoopJoinEdgeRoleV1::Backedge => vec![
                    PhysicalBlockRoleV1::Body,
                    PhysicalBlockRoleV1::Step,
                    PhysicalBlockRoleV1::Header,
                ],
                role => panic!("unexpected direct physical edge {role:?}"),
            };
            PhysicalPathRoleV1 {
                edge: edge.role,
                roles: roles.into_boxed_slice(),
            }
        })
        .collect::<Vec<_>>();
    let mut operation_keys = Vec::new();
    for block_key in [
        match root.condition {
            crate::mir::loop_recipe_contract::LoopConditionV1::Predicate { block, .. } => block,
            crate::mir::loop_recipe_contract::LoopConditionV1::Always => panic!("direct predicate"),
        },
        root.body,
    ] {
        let block = super::block(recipe, block_key);
        for item_key in &block.items {
            if matches!(super::item(recipe, *item_key), LoopRecipeItemV1::Operation { .. }) {
                operation_keys.push(*item_key);
            }
        }
    }
    PhysicalRolePlanV1 {
        loop_key: root.key,
        paths: paths.into_boxed_slice(),
        operation_keys: operation_keys.into_boxed_slice(),
    }
}

impl PhysicalAllocationV1 {
    fn reserve(
        builder: &mut crate::mir::builder::MirBuilder,
        plan: &PhysicalRolePlanV1,
        artifact: &LoopRecipeArtifactV1,
    ) -> Self {
        let blocks = [
            PhysicalBlockRoleV1::Preheader,
            PhysicalBlockRoleV1::Header,
            PhysicalBlockRoleV1::Body,
            PhysicalBlockRoleV1::Step,
            PhysicalBlockRoleV1::After,
        ]
        .into_iter()
        .map(|role| (role, builder.next_block_id()))
        .collect();
        let recipe = artifact.recipe();
        let operation_results = plan
            .operation_keys
            .iter()
            .filter_map(|item_key| match super::item(recipe, *item_key) {
                LoopRecipeItemV1::Operation { operation } => match operation {
                    LoopOperationV1::ReadBinding { .. } | LoopOperationV1::WriteBinding { .. } => None,
                    LoopOperationV1::ConstI64 { result, .. }
                    | LoopOperationV1::BinaryI64 { result, .. }
                    | LoopOperationV1::CompareI64 { result, .. } => {
                        let ty = if matches!(operation, LoopOperationV1::CompareI64 { .. }) {
                            MirType::Bool
                        } else {
                            MirType::Integer
                        };
                        Some((*item_key, builder.alloc_typed(ty)))
                    }
                },
                _ => None,
            })
            .collect();
        Self {
            blocks,
            operation_results,
        }
    }
}

#[test]
fn direct_role_plan_is_builder_free_and_standard5_explicit() {
    let artifact = direct_verified_recipe();
    let verified = LoopRecipeVerifierV1::verify(artifact.recipe().clone()).expect("verified recipe");
    let sig = LoopJoinSigElaboratorV1::elaborate(&verified).expect("verified JoinSig");
    let plan = direct_role_plan(&sig, &artifact);
    assert_eq!(plan.paths.len(), 4);
    assert!(plan.paths.iter().any(|path| {
        path.edge == LoopJoinEdgeRoleV1::Backedge
            && path.roles.as_ref()
                == [
                    PhysicalBlockRoleV1::Body,
                    PhysicalBlockRoleV1::Step,
                    PhysicalBlockRoleV1::Header,
                ]
    }));
    assert_eq!(plan.operation_keys.len(), 11);
}

#[test]
fn direct_candidate_reservation_is_alpha_stable_and_does_not_emit() {
    let artifact = direct_verified_recipe();
    let verified = LoopRecipeVerifierV1::verify(artifact.recipe().clone()).expect("verified recipe");
    let sig = LoopJoinSigElaboratorV1::elaborate(&verified).expect("verified JoinSig");
    let plan = direct_role_plan(&sig, &artifact);
    let mut left = crate::mir::builder::MirBuilder::new();
    left.enter_function_for_test("p1s0/left".to_owned());
    let mut right = crate::mir::builder::MirBuilder::new();
    right.enter_function_for_test("p1s0/right".to_owned());
    let left_alloc = PhysicalAllocationV1::reserve(&mut left, &plan, &artifact);
    let right_alloc = PhysicalAllocationV1::reserve(&mut right, &plan, &artifact);
    assert_eq!(left_alloc.blocks.keys().collect::<Vec<_>>(), right_alloc.blocks.keys().collect::<Vec<_>>());
    assert_eq!(left_alloc.operation_results.len(), 6);
    assert!(left
        .function_state
        .current_function
        .as_ref()
        .expect("candidate function")
        .blocks
        .values()
        .all(|block| block.instructions.is_empty()));
}

#[test]
fn direct_phi_handle_defines_before_read_and_aborts_cleanly() {
    let sig = super::super::direct_verified_sig();
    let mut builder = standard5_builder();
    let map = super::super::direct_materializer_input(&sig);
    let handle = LoopPhiMaterializationHandleV1::begin(&mut builder, &sig, map)
        .expect("begin PHI transaction");
    assert_eq!(handle.destination_values().len(), 2);
    let header = builder
        .function_state
        .current_function
        .as_ref()
        .expect("candidate function")
        .get_block(bb(1))
        .expect("header block");
    assert_eq!(
        header
            .instructions
            .iter()
            .filter(|instruction| matches!(instruction, crate::mir::MirInstruction::Phi { inputs, .. } if inputs.is_empty()))
            .count(),
        2
    );
    let error = handle.abort(&mut builder, "p1-s0 injected failure");
    assert!(error.to_string().contains("txn_abort"));
    assert!(builder
        .function_state
        .current_function
        .as_ref()
        .expect("candidate function")
        .get_block(bb(1))
        .expect("header block")
        .instructions
        .iter()
        .all(|instruction| !matches!(instruction, crate::mir::MirInstruction::Phi { .. })));
}
