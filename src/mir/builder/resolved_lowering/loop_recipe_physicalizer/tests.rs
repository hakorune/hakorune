use super::*;
use crate::mir::builder::resolved_lowering::canonical_cfg::CanonicalCfgSessionV1;
use crate::mir::builder::MirBuilder;
use crate::mir::loop_recipe_contract::{
    issue_generic_g0_recipe_demand_v1, produce_generic_g0_recipe_v1, VerifiedLoopPhysicalBoundaryV1,
};
use crate::mir::loop_route_policy::generic_selection_for_test;
use crate::mir::resolved_semantics::FunctionOwnerIssuerV1;
use crate::mir::{BasicBlockId, ConstValue, MirInstruction, MirType, ValueId};

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

fn instruction_count(builder: &MirBuilder) -> usize {
    builder
        .function_state
        .current_function
        .as_ref()
        .expect("function")
        .blocks
        .values()
        .map(|block| block.instructions.len())
        .sum()
}

fn const_canary(
    owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
) -> PreparedLoopOperationEmissionV1 {
    PreparedLoopOperationEmissionV1::const_i64_for_canary(
        owner,
        crate::mir::loop_recipe_contract::LoopItemKeyV1::new(1),
        crate::mir::loop_recipe_contract::LoopNodeKeyV1::new(0),
        crate::mir::loop_recipe_contract::LoopBlockKeyV1::new(0),
        LoopPhysicalBlockRoleV1::Header,
        crate::mir::loop_recipe_contract::LoopValueKeyV1::new(3),
        42,
    )
}

fn emit_const_in_fresh_session() -> (ConstValue, usize) {
    let mut builder = MirBuilder::new();
    let mut session = builder.open_resolved_function_draft_seal_session_v1("const_canary/0");
    let session_builder = session.builder_view_mut_for_test();
    session_builder.enter_function_for_test("const_canary/0".into());
    let boundary = generic_boundary();
    let owner = boundary.core().owner();
    let preheader = session_builder.current_block_for_test().unwrap();
    let topology_entry = entry_for(&boundary, preheader);
    let emission_entry = entry_for(&boundary, preheader);
    let mut cfg = CanonicalCfgSessionV1::new();
    let mut services = LoopPhysicalServicesV1::new(session_builder, &mut cfg);
    let topology = physicalize_topology_v1(boundary, topology_entry, &mut services).unwrap();
    let target = topology
        .block_receipt()
        .lookup(
            crate::mir::loop_recipe_contract::LoopNodeKeyV1::new(0),
            LoopPhysicalBlockRoleV1::Header,
        )
        .unwrap();
    let emitted = emit_prepared_operation_v1(
        const_canary(owner),
        &emission_entry,
        topology.block_receipt(),
        &mut services,
    )
    .unwrap();
    let function = services
        .builder
        .function_state
        .current_function
        .as_ref()
        .unwrap();
    let constants = function
        .get_block(target)
        .unwrap()
        .instructions
        .iter()
        .filter_map(|instruction| match instruction {
            MirInstruction::Const { value, .. } => Some(value.clone()),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(emitted.physical_block(), target);
    assert_eq!(constants, vec![ConstValue::Integer(42)]);
    assert_eq!(
        services
            .builder
            .function_state
            .type_ctx
            .get_type(emitted.physical_value()),
        Some(&MirType::Integer)
    );
    let count = instruction_count(services.builder);
    drop(services);

    // Harness-only late failure: production emission has no injected branch.
    let injected: Result<(), &str> = Err("after-emission");
    assert!(injected.is_err());
    session.discard_unpublished();
    assert!(builder.function_state.current_function.is_none());
    (ConstValue::Integer(42), count)
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

#[test]
fn const_leaf_emits_once_into_exact_receipt_block() {
    let boundary = generic_boundary();
    let owner = boundary.core().owner();
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("const_leaf".into());
    let preheader = builder.current_block_for_test().unwrap();
    let topology_entry = entry_for(&boundary, preheader);
    let emission_entry = entry_for(&boundary, preheader);
    let mut cfg = CanonicalCfgSessionV1::new();
    let mut services = LoopPhysicalServicesV1::new(&mut builder, &mut cfg);
    let topology = physicalize_topology_v1(boundary, topology_entry, &mut services).unwrap();
    let current_before = services.builder.current_block_for_test().unwrap();
    let target = topology
        .block_receipt()
        .lookup(
            crate::mir::loop_recipe_contract::LoopNodeKeyV1::new(0),
            LoopPhysicalBlockRoleV1::Header,
        )
        .unwrap();
    let emitted = emit_prepared_operation_v1(
        const_canary(owner),
        &emission_entry,
        topology.block_receipt(),
        &mut services,
    )
    .unwrap();
    assert_eq!(emitted.owner(), owner);
    assert_eq!(emitted.item().raw(), 1);
    assert_eq!(emitted.result().raw(), 3);
    assert_eq!(emitted.physical_block(), target);
    assert_eq!(
        services.builder.current_block_for_test().unwrap(),
        current_before
    );
    let function = services
        .builder
        .function_state
        .current_function
        .as_ref()
        .unwrap();
    assert_eq!(function.get_block(target).unwrap().instructions.len(), 1);
    assert_eq!(instruction_count(services.builder), 1);
    assert_eq!(
        services
            .builder
            .function_state
            .type_ctx
            .get_type(emitted.physical_value()),
        Some(&MirType::Integer)
    );
}

#[test]
fn const_leaf_rejects_wrong_role_or_owner_before_emission() {
    let boundary = generic_boundary();
    let owner = boundary.core().owner();
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("const_leaf_reject".into());
    let preheader = builder.current_block_for_test().unwrap();
    let topology_entry = entry_for(&boundary, preheader);
    let emission_entry = entry_for(&boundary, preheader);
    let mut cfg = CanonicalCfgSessionV1::new();
    let mut services = LoopPhysicalServicesV1::new(&mut builder, &mut cfg);
    let topology = physicalize_topology_v1(boundary, topology_entry, &mut services).unwrap();
    let before = instruction_count(services.builder);
    let wrong_role = PreparedLoopOperationEmissionV1::const_i64_for_canary(
        owner,
        crate::mir::loop_recipe_contract::LoopItemKeyV1::new(1),
        crate::mir::loop_recipe_contract::LoopNodeKeyV1::new(0),
        crate::mir::loop_recipe_contract::LoopBlockKeyV1::new(0),
        LoopPhysicalBlockRoleV1::Body,
        crate::mir::loop_recipe_contract::LoopValueKeyV1::new(3),
        42,
    );
    assert!(matches!(
        emit_prepared_operation_v1(
            wrong_role,
            &emission_entry,
            topology.block_receipt(),
            &mut services,
        ),
        Err(LoopOperationEmissionRejectV1::PlacementMismatch { .. })
    ));
    assert_eq!(instruction_count(services.builder), before);

    let mut issuer = FunctionOwnerIssuerV1::new_for_compilation().unwrap();
    let foreign_owner = issuer.issue().unwrap();
    let foreign_entry = ReadyLoopEntryV1::new_for_test(
        foreign_owner,
        preheader,
        emission_entry.rows.iter().copied().collect(),
    );
    assert!(matches!(
        emit_prepared_operation_v1(
            const_canary(owner),
            &foreign_entry,
            topology.block_receipt(),
            &mut services,
        ),
        Err(LoopOperationEmissionRejectV1::EntryOwnerMismatch)
    ));
    assert_eq!(instruction_count(services.builder), before);
}

#[test]
fn const_leaf_repeats_after_late_failure_and_fresh_session_discard() {
    assert_eq!(emit_const_in_fresh_session(), (ConstValue::Integer(42), 1));
    assert_eq!(emit_const_in_fresh_session(), (ConstValue::Integer(42), 1));
}
