use super::operation_target::LoopOperationTargetRejectV1;
use super::*;

use crate::mir::builder::emission::phi_lifecycle::PhiTxn;
use crate::mir::builder::resolved_lowering::canonical_ssa::ResolvedSsaIdentityStateV2;
use crate::mir::builder::MirBuilder;
use crate::mir::compiler::callable_single_loop_operation_effect::callable_operation_fixture_for_test;
use crate::mir::loop_recipe_contract::{
    LoopBinaryI64OpV1, LoopBlockKeyV1, LoopCompareI64OpV1, LoopItemKeyV1, LoopNodeKeyV1,
    LoopOperationV1, LoopValueKeyV1,
};
use crate::mir::resolved_semantics::{BindingOriginV1, FunctionOwnerIssuerV1};
use crate::mir::{BasicBlock, BasicBlockId, MirType, ValueId};

fn receipt(
    builder: &mut MirBuilder,
    owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
) -> (ReadyLoopEntryV1, LoopPhysicalBlockReceiptV1) {
    let preheader = builder.current_block_for_test().expect("preheader");
    let function = builder
        .function_state
        .current_function
        .as_mut()
        .expect("function");
    for id in 1..=4 {
        function.add_block(BasicBlock::new(BasicBlockId::new(id)));
    }
    let loop_key = LoopNodeKeyV1::new(0);
    let block = LoopBlockKeyV1::new(0);
    let rows = vec![
        LoopPhysicalBlockRowV1::new(
            loop_key,
            None,
            LoopPhysicalBlockRoleV1::Preheader,
            preheader,
        ),
        LoopPhysicalBlockRowV1::new(
            loop_key,
            Some(block),
            LoopPhysicalBlockRoleV1::Header,
            BasicBlockId::new(1),
        ),
        LoopPhysicalBlockRowV1::new(
            loop_key,
            Some(LoopBlockKeyV1::new(1)),
            LoopPhysicalBlockRoleV1::Body,
            BasicBlockId::new(2),
        ),
        LoopPhysicalBlockRowV1::new(
            loop_key,
            None,
            LoopPhysicalBlockRoleV1::Step,
            BasicBlockId::new(3),
        ),
        LoopPhysicalBlockRowV1::new(
            loop_key,
            None,
            LoopPhysicalBlockRoleV1::After,
            BasicBlockId::new(4),
        ),
    ];
    let receipt = LoopPhysicalBlockReceiptV1::issue(owner, preheader, &[loop_key], rows)
        .expect("block receipt");
    (
        ReadyLoopEntryV1::new_for_test(owner, preheader, Vec::new()),
        receipt,
    )
}

#[test]
fn pure_operation_dispatcher_emits_const_binary_and_compare() {
    let owner = FunctionOwnerIssuerV1::new_for_compilation()
        .expect("issuer")
        .issue()
        .expect("owner");
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("operation_family/0".to_string());
    let (entry, blocks) = receipt(&mut builder, owner);
    let mut services = LoopOperationServicesV1::new(&mut builder);
    let mut state = LoopOperationValueLedgerV1::default();
    let make = |item, operation| {
        PreparedLoopOperationEmissionV1::from_operation_for_canary(
            owner,
            LoopItemKeyV1::new(item),
            operation,
            LoopNodeKeyV1::new(0),
            LoopBlockKeyV1::new(0),
            LoopPhysicalBlockRoleV1::Header,
        )
    };

    emit_prepared_pure_operation_v1(
        make(
            0,
            LoopOperationV1::ConstI64 {
                result: LoopValueKeyV1::new(0),
                value: 2,
            },
        ),
        &mut state,
        &entry,
        &blocks,
        &mut services,
    )
    .expect("left const");
    emit_prepared_pure_operation_v1(
        make(
            1,
            LoopOperationV1::ConstI64 {
                result: LoopValueKeyV1::new(1),
                value: 3,
            },
        ),
        &mut state,
        &entry,
        &blocks,
        &mut services,
    )
    .expect("right const");
    emit_prepared_pure_operation_v1(
        make(
            2,
            LoopOperationV1::BinaryI64 {
                op: LoopBinaryI64OpV1::Add,
                left: LoopValueKeyV1::new(0),
                right: LoopValueKeyV1::new(1),
                result: LoopValueKeyV1::new(2),
            },
        ),
        &mut state,
        &entry,
        &blocks,
        &mut services,
    )
    .expect("sum");
    let compare = emit_prepared_pure_operation_v1(
        make(
            3,
            LoopOperationV1::CompareI64 {
                op: LoopCompareI64OpV1::Equal,
                left: LoopValueKeyV1::new(2),
                right: LoopValueKeyV1::new(1),
                result: LoopValueKeyV1::new(3),
            },
        ),
        &mut state,
        &entry,
        &blocks,
        &mut services,
    )
    .expect("comparison");
    assert_eq!(
        services
            .builder
            .function_state
            .type_ctx
            .get_type(compare.physical_value()),
        Some(&MirType::Bool)
    );
    let function = services
        .builder
        .function_state
        .current_function
        .as_ref()
        .expect("function");
    let header = function.get_block(BasicBlockId::new(1)).expect("header");
    assert_eq!(
        header
            .instructions
            .iter()
            .filter(|instruction| {
                matches!(
                    instruction,
                    crate::mir::MirInstruction::Const { .. }
                        | crate::mir::MirInstruction::BinOp { .. }
                        | crate::mir::MirInstruction::Compare { .. }
                )
            })
            .count(),
        4
    );
    assert!(function
        .get_block(entry.preheader())
        .expect("preheader")
        .instructions
        .is_empty());
}

#[test]
fn common_dispatcher_publishes_read_before_binary_and_write() {
    let fixture = callable_operation_fixture_for_test();
    let unit = fixture.unit;
    let input = unit.root_function_input().expect("root input");
    let owner = input.function().owner();
    let (effect, context, continuation) = fixture.product.into_operation_demand_parts();
    let demand = crate::mir::loop_recipe_contract::VerifiedLoopOperationPhysicalDemandV1::issue(
        context,
        effect,
        continuation,
    )
    .expect("demand");
    let program = demand.prepare_all().expect("program");
    let read = program
        .read_binding_rows()
        .expect("read rows")
        .into_vec()
        .into_iter()
        .find(|row| row.item().raw() == 3)
        .expect("step read");
    let write = program
        .write_binding_rows()
        .expect("write rows")
        .into_vec()
        .remove(0);

    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("operation_dispatcher/read_write".to_string());
    let (entry, blocks) = receipt(&mut builder, owner);
    let mut identity = ResolvedSsaIdentityStateV2::new(input.function());
    let record = input
        .function()
        .binding(read.source_binding())
        .expect("binding record");
    let BindingOriginV1::Source(site) = record.origin() else {
        panic!("expected source binding");
    };
    identity
        .publish_declaration(
            site,
            record.kind(),
            record.diagnostic_name(),
            BasicBlockId::new(2),
            ValueId::new(20),
        )
        .expect("declaration");
    builder
        .function_state
        .type_ctx
        .set_type(ValueId::new(20), MirType::Integer);
    let mut phis = PhiTxn::begin("operation_dispatcher/read_write");
    let mut services = LoopOperationDispatchServicesV1::new(&mut builder, &mut identity, &mut phis);
    let mut state = LoopOperationValueLedgerV1::default();

    emit_prepared_operation_family_v1(
        PreparedLoopOperationDispatchV1::Read(
            PreparedLoopReadBindingEmissionV1::from_row_for_test(
                owner,
                &read,
                LoopPhysicalBlockRoleV1::Body,
                LoopReadEntryRequirementV1::CanonicalLive,
            ),
        ),
        &mut state,
        &entry,
        &blocks,
        &mut services,
    )
    .expect("step read");
    assert_eq!(state.get(read.result()), Some(ValueId::new(20)));

    emit_prepared_operation_family_v1(
        PreparedLoopOperationDispatchV1::Pure(
            PreparedLoopOperationEmissionV1::from_operation_for_canary(
                owner,
                LoopItemKeyV1::new(4),
                LoopOperationV1::ConstI64 {
                    result: LoopValueKeyV1::new(5),
                    value: 1,
                },
                LoopNodeKeyV1::new(0),
                LoopBlockKeyV1::new(1),
                LoopPhysicalBlockRoleV1::Body,
            ),
        ),
        &mut state,
        &entry,
        &blocks,
        &mut services,
    )
    .expect("step delta");
    emit_prepared_operation_family_v1(
        PreparedLoopOperationDispatchV1::Pure(
            PreparedLoopOperationEmissionV1::from_operation_for_canary(
                owner,
                LoopItemKeyV1::new(5),
                LoopOperationV1::BinaryI64 {
                    op: LoopBinaryI64OpV1::Add,
                    left: read.result(),
                    right: LoopValueKeyV1::new(5),
                    result: write.value(),
                },
                LoopNodeKeyV1::new(0),
                LoopBlockKeyV1::new(1),
                LoopPhysicalBlockRoleV1::Body,
            ),
        ),
        &mut state,
        &entry,
        &blocks,
        &mut services,
    )
    .expect("step add");
    let receipt = emit_prepared_operation_family_v1(
        PreparedLoopOperationDispatchV1::Write(
            PreparedLoopWriteBindingEmissionV1::from_row_for_test(
                owner,
                &write,
                LoopPhysicalBlockRoleV1::Body,
            ),
        ),
        &mut state,
        &entry,
        &blocks,
        &mut services,
    )
    .expect("step write");
    assert!(matches!(receipt, LoopOperationDispatchReceiptV1::Write(_)));
}

#[test]
fn full_dispatch_prepare_covers_callable_recipe_order_without_builder_effect() {
    let fixture = callable_operation_fixture_for_test();
    let unit = fixture.unit;
    let input = unit.root_function_input().expect("root input");
    let owner = input.function().owner();
    let (effect, context, continuation) = fixture.product.into_operation_demand_parts();
    let demand = crate::mir::loop_recipe_contract::VerifiedLoopOperationPhysicalDemandV1::issue(
        context,
        effect,
        continuation,
    )
    .expect("demand");
    let program = demand.prepare_all().expect("program");

    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("operation_dispatcher/full_prepare".to_string());
    let (entry, blocks) = receipt(&mut builder, owner);
    let before = builder
        .function_state
        .current_function
        .as_ref()
        .expect("function")
        .block_ids()
        .into_iter()
        .map(|id| {
            let block = builder
                .function_state
                .current_function
                .as_ref()
                .expect("function")
                .get_block(id)
                .expect("block");
            (id, block.instructions.len(), block.terminator.is_some())
        })
        .collect::<Vec<_>>();

    let plan = prepare_loop_operation_dispatch_v1(program, entry, blocks)
        .expect("full dispatch preflight");
    assert_eq!(plan.operation_count(), 7);
    assert_eq!(plan.rows().len(), 7);
    assert_eq!(
        plan.rows()
            .iter()
            .filter(|row| matches!(row, PreparedLoopOperationDispatchV1::Read(_)))
            .count(),
        2
    );
    assert_eq!(
        plan.rows()
            .iter()
            .filter(|row| matches!(row, PreparedLoopOperationDispatchV1::Pure(_)))
            .count(),
        4
    );
    assert_eq!(
        plan.rows()
            .iter()
            .filter(|row| matches!(row, PreparedLoopOperationDispatchV1::Write(_)))
            .count(),
        1
    );

    let after = builder
        .function_state
        .current_function
        .as_ref()
        .expect("function")
        .block_ids()
        .into_iter()
        .map(|id| {
            let block = builder
                .function_state
                .current_function
                .as_ref()
                .expect("function")
                .get_block(id)
                .expect("block");
            (id, block.instructions.len(), block.terminator.is_some())
        })
        .collect::<Vec<_>>();
    assert_eq!(before, after);
}

#[test]
fn full_dispatch_validates_all_targets_before_leaf_effect() {
    let fixture = callable_operation_fixture_for_test();
    let unit = fixture.unit;
    let input = unit.root_function_input().expect("root input");
    let owner = input.function().owner();
    let (effect, context, continuation) = fixture.product.into_operation_demand_parts();
    let demand = crate::mir::loop_recipe_contract::VerifiedLoopOperationPhysicalDemandV1::issue(
        context,
        effect,
        continuation,
    )
    .expect("demand");
    let program = demand.prepare_all().expect("program");

    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("operation_dispatcher/target_batch".to_string());
    let (entry, blocks) = receipt(&mut builder, owner);
    let plan = prepare_loop_operation_dispatch_v1(program, entry, blocks)
        .expect("full dispatch preflight");

    builder
        .function_state
        .current_function
        .as_mut()
        .expect("function")
        .get_block_mut(BasicBlockId::new(1))
        .expect("header block")
        .set_terminator(crate::mir::MirInstruction::Return { value: None });

    let error = plan
        .validate_targets(&builder)
        .expect_err("terminated target must reject before emission");
    assert_eq!(
        error,
        LoopOperationDispatchPhysicalFailureV1::Target(
            LoopOperationTargetRejectV1::TargetBlockTerminated(BasicBlockId::new(1)),
        )
    );
}

#[test]
fn operation_value_ledger_rejects_duplicate_without_overwrite() {
    let owner = FunctionOwnerIssuerV1::new_for_compilation()
        .expect("issuer")
        .issue()
        .expect("owner");
    let key = LoopValueKeyV1::new(9);
    let first = LoopOperationValueReceiptV1::new(
        owner,
        key,
        crate::mir::loop_recipe_contract::LoopValueClassV1::I64,
        LoopItemKeyV1::new(1),
        BasicBlockId::new(2),
        ValueId::new(20),
    );
    let second = LoopOperationValueReceiptV1::new(
        owner,
        key,
        crate::mir::loop_recipe_contract::LoopValueClassV1::I64,
        LoopItemKeyV1::new(2),
        BasicBlockId::new(3),
        ValueId::new(30),
    );
    let mut ledger = LoopOperationValueLedgerV1::default();
    ledger.publish(first).expect("first publish");
    assert!(ledger.publish(second).is_err());
    assert_eq!(ledger.receipt(key), Some(first));
}
