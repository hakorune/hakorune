use super::*;
use crate::mir::builder::resolved_lowering::canonical_cfg::CanonicalCfgSessionV1;
use crate::mir::builder::MirBuilder;
use crate::mir::loop_recipe_contract::{
    issue_generic_g0_recipe_demand_v1, produce_generic_g0_recipe_v1, VerifiedLoopPhysicalBoundaryV1,
};
use crate::mir::loop_route_policy::generic_selection_for_test;
use crate::mir::{BasicBlockId, ValueId};

fn generic_boundary() -> VerifiedLoopPhysicalBoundaryV1 {
    produce_generic_g0_recipe_v1(
        issue_generic_g0_recipe_demand_v1(generic_selection_for_test()).expect("generic demand"),
    )
    .expect("generic product")
    .into_physical_boundary()
}

fn entry_for(
    boundary: &VerifiedLoopPhysicalBoundaryV1,
    preheader: BasicBlockId,
) -> ReadyLoopEntryV1 {
    let owner = boundary.core().owner();
    let bindings = boundary.core().binding_relations();
    let rows = boundary
        .recipe()
        .as_recipe()
        .inputs
        .iter()
        .enumerate()
        .map(|(index, key)| {
            ReadyLoopEntryRowV1::new(
                *key,
                bindings
                    .get(index)
                    .expect("one binding relation per input")
                    .source_binding(),
                ValueId::new(index as u32),
            )
        })
        .collect();
    ReadyLoopEntryV1::new_for_test(owner, preheader, rows)
}

#[test]
fn recursive_generic_recipe_allocates_child_and_root_after() {
    let boundary = generic_boundary();
    let owner = boundary.core().owner();
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("loop_topology".into());
    let preheader = builder.current_block_for_test().unwrap();
    let entry = entry_for(&boundary, preheader);
    let mut cfg = CanonicalCfgSessionV1::new();
    let mut services = LoopPhysicalServicesV1::new(&mut builder, &mut cfg);
    let receipt = physicalize_topology_v1(boundary, entry, &mut services).unwrap();
    assert_eq!(receipt.owner(), owner);
    assert_eq!(
        receipt.root_loop(),
        crate::mir::loop_recipe_contract::LoopNodeKeyV1::new(0)
    );
    assert_eq!(receipt.loop_count(), 2);
    let root = crate::mir::loop_recipe_contract::LoopNodeKeyV1::new(0);
    let child = crate::mir::loop_recipe_contract::LoopNodeKeyV1::new(1);
    assert!(receipt.after_for(root).is_some());
    assert!(receipt.after_for(child).is_some());
    assert_eq!(receipt.preheader_for(root), Some(preheader));
    assert_ne!(receipt.preheader_for(child), Some(preheader));
    assert_ne!(receipt.root_after(), preheader);
}

#[test]
fn entry_mismatch_is_rejected_before_block_allocation() {
    let boundary = generic_boundary();
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("loop_topology".into());
    let preheader = builder.current_block_for_test().unwrap();
    let mut entry = entry_for(&boundary, preheader);
    entry.rows = entry.rows[..entry.rows.len() - 1]
        .to_vec()
        .into_boxed_slice();
    let before = builder
        .function_state
        .current_function
        .as_ref()
        .unwrap()
        .block_ids()
        .len();
    let mut cfg = CanonicalCfgSessionV1::new();
    let mut services = LoopPhysicalServicesV1::new(&mut builder, &mut cfg);
    assert!(matches!(
        physicalize_topology_v1(boundary, entry, &mut services),
        Err(LoopPhysicalizerRejectV1::EntryKeyMismatch(_))
    ));
    let after = services
        .builder
        .function_state
        .current_function
        .as_ref()
        .unwrap()
        .block_ids()
        .len();
    assert_eq!(before, after);
}

#[test]
fn block_receipt_binds_every_nested_topology_role() {
    let boundary = generic_boundary();
    let owner = boundary.core().owner();
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("loop_topology".into());
    let preheader = builder.current_block_for_test().unwrap();
    let entry = entry_for(&boundary, preheader);
    let mut cfg = CanonicalCfgSessionV1::new();
    let mut services = LoopPhysicalServicesV1::new(&mut builder, &mut cfg);
    let receipt = physicalize_topology_v1(boundary, entry, &mut services).unwrap();
    let blocks = receipt.block_receipt();
    let root = crate::mir::loop_recipe_contract::LoopNodeKeyV1::new(0);
    let child = crate::mir::loop_recipe_contract::LoopNodeKeyV1::new(1);
    assert_eq!(blocks.owner(), owner);
    assert_eq!(blocks.preheader(), preheader);
    assert_eq!(blocks.rows().len(), 10);
    for loop_key in [root, child] {
        for role in [
            LoopPhysicalBlockRoleV1::Preheader,
            LoopPhysicalBlockRoleV1::Header,
            LoopPhysicalBlockRoleV1::Body,
            LoopPhysicalBlockRoleV1::Step,
            LoopPhysicalBlockRoleV1::After,
        ] {
            assert!(blocks.lookup(loop_key, role).is_some());
        }
    }
    assert!(blocks
        .lookup_logical(
            root,
            crate::mir::loop_recipe_contract::LoopBlockKeyV1::new(0)
        )
        .is_some());
    assert!(blocks
        .lookup_logical(
            root,
            crate::mir::loop_recipe_contract::LoopBlockKeyV1::new(1)
        )
        .is_some());
}

#[test]
fn block_receipt_rejects_duplicate_and_missing_roles() {
    let boundary = generic_boundary();
    let owner = boundary.core().owner();
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("loop_topology".into());
    let preheader = builder.current_block_for_test().unwrap();
    let entry = entry_for(&boundary, preheader);
    let mut cfg = CanonicalCfgSessionV1::new();
    let mut services = LoopPhysicalServicesV1::new(&mut builder, &mut cfg);
    let receipt = physicalize_topology_v1(boundary, entry, &mut services).unwrap();
    let blocks = receipt.block_receipt();
    let loop_keys = [
        crate::mir::loop_recipe_contract::LoopNodeKeyV1::new(0),
        crate::mir::loop_recipe_contract::LoopNodeKeyV1::new(1),
    ];
    let mut duplicate = blocks.rows().to_vec();
    duplicate.push(duplicate[0]);
    assert!(matches!(
        LoopPhysicalBlockReceiptV1::issue(owner, preheader, &loop_keys, duplicate),
        Err(LoopPhysicalBlockReceiptRejectV1::DuplicatePlacement { .. })
    ));
    let missing = blocks.rows()[..blocks.rows().len() - 1].to_vec();
    assert!(matches!(
        LoopPhysicalBlockReceiptV1::issue(owner, preheader, &loop_keys, missing),
        Err(LoopPhysicalBlockReceiptRejectV1::MissingRole { .. })
    ));
    let mut foreign = blocks.rows().to_vec();
    foreign.push(LoopPhysicalBlockRowV1::new(
        crate::mir::loop_recipe_contract::LoopNodeKeyV1::new(99),
        None,
        LoopPhysicalBlockRoleV1::Preheader,
        BasicBlockId::new(999),
    ));
    assert!(matches!(
        LoopPhysicalBlockReceiptV1::issue(owner, preheader, &loop_keys, foreign),
        Err(LoopPhysicalBlockReceiptRejectV1::ForeignLoop { .. })
    ));
}
