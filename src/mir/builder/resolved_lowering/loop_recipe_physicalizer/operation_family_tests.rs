use super::*;

use crate::mir::builder::resolved_lowering::canonical_cfg::CanonicalCfgSessionV1;
use crate::mir::builder::MirBuilder;
use crate::mir::loop_recipe_contract::{
    LoopBinaryI64OpV1, LoopBlockKeyV1, LoopCompareI64OpV1, LoopItemKeyV1, LoopNodeKeyV1,
    LoopOperationV1, LoopValueKeyV1,
};
use crate::mir::resolved_semantics::FunctionOwnerIssuerV1;
use crate::mir::{BasicBlock, BasicBlockId, MirType};

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
            None,
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
    let mut cfg = CanonicalCfgSessionV1::new();
    let mut services = LoopPhysicalServicesV1::new(&mut builder, &mut cfg);
    let mut state = LoopOperationValueStateV1::default();
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
