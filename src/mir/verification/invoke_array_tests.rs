//! Physical consumer tests; these do not issue Script source capability.
use crate::mir::instruction::InvokeOperation;
use crate::mir::{BasicBlock, BasicBlockId, MirInstruction, MirModule, MirVerifier, ValueId};

fn array_function() -> crate::mir::MirFunction {
    let mut function = super::tests::allocation_invoke_function();
    let MirInstruction::Invoke { operation, .. } = function
        .blocks
        .get_mut(&BasicBlockId::new(1))
        .unwrap()
        .terminator
        .as_mut()
        .unwrap()
    else {
        unreachable!()
    };
    *operation = InvokeOperation::IntrinsicArrayNew;
    function
}

#[test]
fn array_result_is_normal_only_and_needs_no_object_definition() {
    let function = array_function();
    let mut module = MirModule::new("array_control".into());
    module.add_function(function.clone());
    MirVerifier::new().verify_module(&module).unwrap();
    assert_eq!(
        crate::mir::array_element_write::project_module_to_legacy_calls(&module).unwrap_err(),
        crate::mir::array_element_write::TYPED_ARRAY_LEGACY_PROJECTION_FORBIDDEN_TAG
    );
    let release = MirInstruction::ArrayResidenceRelease {
        value: ValueId::new(2),
    };
    assert!(!crate::mir::contracts::backend_core_ops::is_supported_vm_instruction(&release));
    assert!(!crate::mir::contracts::backend_core_ops::is_supported_mir_json_instruction(&release));
    let mut invalid = function.clone();
    invalid
        .blocks
        .get_mut(&BasicBlockId::new(2))
        .unwrap()
        .instructions
        .clear();
    assert!(
        format!("{:?}", super::check_function(&invalid).unwrap_err())
            .contains("normal-result-count")
    );
    let mut invalid = function;
    invalid
        .blocks
        .get_mut(&BasicBlockId::new(3))
        .unwrap()
        .add_instruction(MirInstruction::ArrayResidenceRelease {
            value: ValueId::new(2),
        });
    assert!(super::check_function(&invalid).is_err());
}

#[test]
fn invoke_claim_write_refresh_and_unit_result_rejection() {
    use crate::mir::function::{
        ArrayStateTermKind, TypedArrayContractBoundary, TypedArrayContractSourceIdentity,
    };
    use crate::mir::{ArrayElementWriteKind, ArrayWriteProducerKind, ArrayWriteSiteId};
    let mut function = array_function();
    let claim = crate::mir::type_contracts::typed_array::register_instruction_source(
        &mut function,
        TypedArrayContractBoundary::LocalInit,
        TypedArrayContractSourceIdentity::LocalSlot(crate::mir::LocalSlotId::from(
            crate::mir::BindingId::new(0),
        )),
        ValueId::new(2),
        Some("Array<i8>"),
        "physical-array-test",
    )
    .unwrap()
    .unwrap();
    function
        .blocks
        .get_mut(&BasicBlockId::new(2))
        .unwrap()
        .set_terminator(MirInstruction::Invoke {
            operation: InvokeOperation::ArrayStateContractClaim {
                contract_id: claim.clone(),
                array: ValueId::new(2),
            },
            fault_frame: ValueId::new(0),
            normal_landing: BasicBlockId::new(4),
            fault_landing: BasicBlockId::new(3),
        });
    let mut write = BasicBlock::new(BasicBlockId::new(4));
    write.set_terminator(MirInstruction::Invoke {
        operation: InvokeOperation::ArrayElementWrite {
            site_id: ArrayWriteSiteId::new(7),
            kind: ArrayElementWriteKind::LiteralAppend,
            producer: ArrayWriteProducerKind::Literal,
            receiver: ValueId::new(2),
            index: None,
            value: ValueId::new(1),
        },
        fault_frame: ValueId::new(0),
        normal_landing: BasicBlockId::new(5),
        fault_landing: BasicBlockId::new(3),
    });
    let mut done = BasicBlock::new(BasicBlockId::new(5));
    done.add_instruction(MirInstruction::ArrayResidenceRelease {
        value: ValueId::new(2),
    });
    done.set_terminator(MirInstruction::Return { value: None });
    function.add_block(write);
    function.add_block(done);
    function.update_cfg();
    super::check_function(&function).unwrap();
    crate::mir::array_element_write::refresh_function_array_write_witnesses(&mut function).unwrap();
    crate::mir::type_contracts::typed_array::refresh_function(&mut function).unwrap();
    assert_eq!(function.metadata.array_element_write_witnesses.len(), 1);
    assert_eq!(
        function.metadata.array_element_write_witnesses[0].site_id,
        ArrayWriteSiteId::new(7)
    );
    assert!(matches!(function.metadata.array_state_terms[0].kind,
        ArrayStateTermKind::Fresh { allocation_site } if allocation_site == ValueId::new(2)));
    assert_eq!(function.metadata.typed_array_element_contracts.len(), 1);
    let mut drift = function.clone();
    drift
        .blocks
        .get_mut(&BasicBlockId::new(2))
        .unwrap()
        .set_terminator(MirInstruction::Jump {
            target: BasicBlockId::new(4),
            edge_args: None,
        });
    assert!(crate::mir::type_contracts::typed_array::refresh_function(&mut drift).is_err());
    for landing in [4, 5] {
        let mut invalid = function.clone();
        invalid
            .blocks
            .get_mut(&BasicBlockId::new(landing))
            .unwrap()
            .instructions
            .insert(
                0,
                MirInstruction::InvokeNormalResult {
                    dst: ValueId::new(9),
                    invoke_block: BasicBlockId::new(if landing == 4 { 2 } else { 4 }),
                },
            );
        assert!(
            format!("{:?}", super::check_function(&invalid).unwrap_err())
                .contains("normal-result-count")
        );
    }
}
