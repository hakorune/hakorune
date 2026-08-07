use super::*;

use crate::mir::builder::emission::phi_lifecycle::PhiTxn;
use crate::mir::builder::resolved_lowering::canonical_ssa::ResolvedSsaIdentityStateV2;
use crate::mir::builder::MirBuilder;
use crate::mir::compiler::callable_single_loop_operation_effect::callable_operation_fixture_for_test;
use crate::mir::loop_recipe_contract::{
    generic_operation_demand_parts_for_test, LoopBlockKeyV1, LoopNodeKeyV1,
    LoopOperationPhysicalDemandRejectV1, VerifiedLoopOperationPhysicalDemandV1,
};
use crate::mir::resolved_semantics::BindingOriginV1;
use crate::mir::{BasicBlock, BasicBlockId, MirType, ValueId};

fn issue_program() -> (
    crate::mir::VerifiedResolvedSourceUnitV1,
    crate::mir::loop_recipe_contract::PreparedLoopOperationProgramV1,
) {
    let fixture = callable_operation_fixture_for_test();
    let unit = fixture.unit;
    let (effect, context, continuation) = fixture.product.into_operation_demand_parts();
    let demand = VerifiedLoopOperationPhysicalDemandV1::issue(context, effect, continuation)
        .expect("callable operation demand");
    let program = demand.prepare_all().expect("complete operation program");
    (unit, program)
}

fn block_receipt(
    builder: &mut MirBuilder,
    owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    preheader: BasicBlockId,
) -> LoopPhysicalBlockReceiptV1 {
    for id in 1..=4 {
        builder
            .function_state
            .current_function
            .as_mut()
            .expect("function")
            .add_block(BasicBlock::new(BasicBlockId::new(id)));
    }
    LoopPhysicalBlockReceiptV1::issue(
        owner,
        preheader,
        &[LoopNodeKeyV1::new(0)],
        vec![
            LoopPhysicalBlockRowV1::new(
                LoopNodeKeyV1::new(0),
                Some(LoopBlockKeyV1::new(0)),
                LoopPhysicalBlockRoleV1::Preheader,
                preheader,
            ),
            LoopPhysicalBlockRowV1::new(
                LoopNodeKeyV1::new(0),
                None,
                LoopPhysicalBlockRoleV1::Header,
                BasicBlockId::new(1),
            ),
            LoopPhysicalBlockRowV1::new(
                LoopNodeKeyV1::new(0),
                Some(LoopBlockKeyV1::new(1)),
                LoopPhysicalBlockRoleV1::Body,
                BasicBlockId::new(2),
            ),
            LoopPhysicalBlockRowV1::new(
                LoopNodeKeyV1::new(0),
                None,
                LoopPhysicalBlockRoleV1::Step,
                BasicBlockId::new(3),
            ),
            LoopPhysicalBlockRowV1::new(
                LoopNodeKeyV1::new(0),
                None,
                LoopPhysicalBlockRoleV1::After,
                BasicBlockId::new(4),
            ),
        ],
    )
    .expect("exact block receipt")
}

fn seed_identity<'source>(
    input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'source>,
    builder: &mut MirBuilder,
    block: BasicBlockId,
    binding: crate::mir::resolved_semantics::BindingRefV1,
    ty: MirType,
) -> ResolvedSsaIdentityStateV2<'source> {
    let mut identity = ResolvedSsaIdentityStateV2::new(input.function());
    let record = input.function().binding(binding).expect("binding record");
    let BindingOriginV1::Source(site) = record.origin() else {
        panic!("read fixture must use source binding")
    };
    let value = ValueId::new(20);
    identity
        .publish_declaration(site, record.kind(), record.diagnostic_name(), block, value)
        .expect("publish source declaration");
    builder.function_state.type_ctx.set_type(value, ty.clone());
    identity
}

#[test]
fn read_binding_projection_is_full_program_and_emits_canonical_value() {
    let (unit, program) = issue_program();
    let rows = program.read_binding_rows().expect("read projection");
    assert_eq!(rows.len(), 2);
    let row = rows.first().expect("first test row");
    let input = unit.root_function_input().expect("root input");
    let mut builder = MirBuilder::new();
    let mut session = builder.open_resolved_function_draft_seal_session_v1("read_leaf/0");
    let session_builder = session.builder_view_mut_for_test();
    session_builder.enter_function_for_test("read_leaf".into());
    let preheader = session_builder.current_block_for_test().expect("preheader");
    let mut identity = seed_identity(
        input,
        session_builder,
        preheader,
        row.source_binding(),
        MirType::Integer,
    );
    let blocks = block_receipt(session_builder, row.source_binding().owner(), preheader);
    let entry = ReadyLoopEntryV1::new_for_test(row.source_binding().owner(), preheader, Vec::new());
    let prepared = PreparedLoopReadBindingEmissionV1::from_row_for_test(
        row.source_binding().owner(),
        row,
        LoopPhysicalBlockRoleV1::Preheader,
        LoopReadEntryRequirementV1::CanonicalLive,
    );
    let mut phis = PhiTxn::begin("read_leaf");
    let mut services = CanonicalBindingReadServicesV1 {
        builder: session_builder,
        identity: &mut identity,
        phis: &mut phis,
    };
    let receipt = emit_prepared_read_binding_v1(&prepared, &entry, &blocks, &mut services)
        .expect("read emission");
    assert_eq!(receipt.owner(), row.source_binding().owner());
    assert_eq!(receipt.binding(), row.source_binding());
    assert_eq!(receipt.result(), row.result());
    assert_eq!(receipt.logical_block(), row.block());
    assert_eq!(receipt.physical_block(), preheader);
    assert_eq!(receipt.physical_value(), ValueId::new(20));
    drop(services);
    session.discard_unpublished();
    assert!(builder.function_state.current_function.is_none());
}

#[test]
fn read_binding_preheader_requirement_rejects_without_claim_or_mir() {
    let (unit, program) = issue_program();
    let row = program.read_binding_rows().unwrap().into_vec().remove(0);
    let input = unit.root_function_input().unwrap();
    let mut builder = MirBuilder::new();
    let mut session = builder.open_resolved_function_draft_seal_session_v1("read_missing_seed/0");
    let session_builder = session.builder_view_mut_for_test();
    session_builder.enter_function_for_test("read_missing_seed".into());
    let preheader = session_builder.current_block_for_test().unwrap();
    let mut identity = seed_identity(
        input,
        session_builder,
        preheader,
        row.source_binding(),
        MirType::Integer,
    );
    let blocks = block_receipt(session_builder, row.source_binding().owner(), preheader);
    let entry = ReadyLoopEntryV1::new_for_test(row.source_binding().owner(), preheader, Vec::new());
    let prepared = PreparedLoopReadBindingEmissionV1::from_row_for_test(
        row.source_binding().owner(),
        &row,
        LoopPhysicalBlockRoleV1::Preheader,
        LoopReadEntryRequirementV1::PreheaderSeed,
    );
    let mut phis = PhiTxn::begin("read_missing_seed");
    let mut services = CanonicalBindingReadServicesV1 {
        builder: session_builder,
        identity: &mut identity,
        phis: &mut phis,
    };
    assert!(matches!(
        emit_prepared_read_binding_v1(&prepared, &entry, &blocks, &mut services),
        Err(LoopReadBindingEmissionRejectV1::EntryBindingMissing(_))
    ));
    assert!(services
        .builder
        .function_state
        .current_function
        .as_ref()
        .unwrap()
        .get_block(BasicBlockId::new(1))
        .unwrap()
        .instructions
        .is_empty());
    drop(services);
    session.discard_unpublished();
    assert!(builder.function_state.current_function.is_none());
}

#[test]
fn read_binding_type_failure_discards_the_unpublished_session() {
    let (unit, program) = issue_program();
    let row = program.read_binding_rows().unwrap().into_vec().remove(0);
    let input = unit.root_function_input().unwrap();
    let mut builder = MirBuilder::new();
    let mut session = builder.open_resolved_function_draft_seal_session_v1("read_type/0");
    let session_builder = session.builder_view_mut_for_test();
    session_builder.enter_function_for_test("read_type".into());
    let preheader = session_builder.current_block_for_test().unwrap();
    let mut identity = seed_identity(
        input,
        session_builder,
        preheader,
        row.source_binding(),
        MirType::Bool,
    );
    let blocks = block_receipt(session_builder, row.source_binding().owner(), preheader);
    let entry = ReadyLoopEntryV1::new_for_test(row.source_binding().owner(), preheader, Vec::new());
    let prepared = PreparedLoopReadBindingEmissionV1::from_row_for_test(
        row.source_binding().owner(),
        &row,
        LoopPhysicalBlockRoleV1::Preheader,
        LoopReadEntryRequirementV1::CanonicalLive,
    );
    let mut phis = PhiTxn::begin("read_type");
    let mut services = CanonicalBindingReadServicesV1 {
        builder: session_builder,
        identity: &mut identity,
        phis: &mut phis,
    };
    assert!(matches!(
        emit_prepared_read_binding_v1(&prepared, &entry, &blocks, &mut services),
        Err(LoopReadBindingEmissionRejectV1::ResultTypeMismatch)
    ));
    drop(services);
    session.discard_unpublished();
    assert!(builder.function_state.current_function.is_none());
}

#[test]
fn generic_carrier_entry_is_not_admitted_as_read_leaf() {
    let (effect, context, continuation) = generic_operation_demand_parts_for_test();
    let demand = VerifiedLoopOperationPhysicalDemandV1::issue(context, effect, continuation)
        .expect("generic operation demand");
    let program = demand.prepare_all().expect("generic operation program");
    assert!(matches!(
        program.read_binding_rows(),
        Err(LoopOperationPhysicalDemandRejectV1::CarrierSeedUnavailable { item })
            if item.raw() == 3
    ));
}

#[test]
fn read_binding_does_not_offer_single_operation_demand_extraction() {
    let (_unit, program) = issue_program();
    assert_eq!(program.coverage().operation_count(), 7);
    assert!(program.read_binding_rows().unwrap().len() < program.coverage().operation_count());
}
