use super::*;
use crate::mir::instruction::InvokeOperation;
use crate::mir::normal_callable_semantic_package::{
    ConstructionStoreRhsV1, ConstructionUnavailableV1,
};
use crate::mir::resolved_semantics::{
    BindingRefV1, FunctionOwnerIssuerV1, SourcePathSegmentV1, SourcePathV1,
};
use crate::mir::{
    BasicBlock, BasicBlockId, ConstValue, EffectMask, FunctionSignature, MirFunction,
    MirInstruction, MirType, ValueId,
};
use hakorune_mir_defs::CanonicalFieldRefV1;
use std::collections::BTreeMap;

#[test]
fn completed_store_bindings_reject_finalizer_drift_and_residuals() {
    // Physical validator unit test only. Source issuance is exercised by
    // the fixed Pair production publication test, not by this local row.
    let field = CanonicalFieldRefV1::from_declaration_ordinal(
        hakorune_mir_defs::CanonicalObjectIdV1::from_declaration_index(0).unwrap(),
        0,
    )
    .unwrap();
    let origin = BasicBlockId::new(0);
    let normal = BasicBlockId::new(1);
    let landing = BasicBlockId::new(2);
    let base = ValueId::new(0);
    let value = ValueId::new(1);
    let frame = ValueId::new(2);
    let mut owners = FunctionOwnerIssuerV1::new_for_compilation().unwrap();
    let owner = owners.issue().unwrap();
    let receiver_site = SourcePathV1::function_body()
        .child(SourcePathSegmentV1::Target)
        .child(SourcePathSegmentV1::Receiver)
        .expr();
    let receiver_binding = BindingRefV1::new(owner, crate::mir::BindingId::new(0));
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "physical_store_binding".into(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::WRITE,
        },
        origin,
    );
    let mut block = BasicBlock::new(origin);
    block.add_instruction(MirInstruction::Const {
        dst: value,
        value: ConstValue::Integer(7),
    });
    block.set_terminator(MirInstruction::Invoke {
        operation: InvokeOperation::FieldSet { mode: crate::mir::instruction::FieldSetModeV1::ExactSlot, field, base, value },
        fault_frame: frame,
        normal_landing: normal,
        fault_landing: landing,
    });
    function.add_block(block);
    let mut fault_block = BasicBlock::new(landing);
    fault_block.set_terminator(MirInstruction::ReturnFault { fault_frame: frame });
    function.add_block(fault_block);
    let site = SourcePathV1::function_body().node();
    let mut state = ConstructionState::Selected {
        steps: vec![ConstructionSourceStepV1::Store(crate::mir::resolved_semantics::SourceStmtSiteV1::from_node(site.clone()))].into_boxed_slice(),
        next_step: 1, calls: Vec::new(),
        stores: BTreeMap::from([(
            site.clone(),
            SelectedConstructionStore {
                field,
                receiver_site,
                receiver_binding,
                rhs: ConstructionStoreRhsV1::LiteralI64(7),
                fault_discharge: Box::new([]),
                me_reads: Box::new([]),
                progress: StoreProgress::Emitted {
                    block: origin,
                    normal,
                    base,
                    value,
                    provider: None,
                    provider_child: None,
                    discharge: landing,
                },
            },
        )]),
        frame: Some((frame, landing)),
        completed: true,
    };
    state.validate_bindings(&function).unwrap();
    // The same sole validator rejects the retired Provided release tail.
    let child = hakorune_mir_defs::CanonicalObjectIdV1::from_declaration_index(1).unwrap();
    if let ConstructionState::Selected { stores, .. } = &mut state {
        stores.get_mut(&site).unwrap().rhs = ConstructionStoreRhsV1::Parameter {
            site: crate::mir::resolved_semantics::SourceExprSiteV1::from_node(site.clone()),
            binding: receiver_binding, provided: Some(child),
        };
    }
    let mut provided = function.clone();
    provided.blocks.get_mut(&origin).unwrap().set_terminator(MirInstruction::Invoke {
        operation: InvokeOperation::ObjectFieldSet { field, base, value, child },
        fault_frame: frame, normal_landing: normal, fault_landing: landing,
    });
    state.validate_bindings(&provided).unwrap();
    for operation in [InvokeOperation::HomeReleaseIfLive { object: child, value },
                      InvokeOperation::OwnedObjectFieldRelease { field, base, child }] {
        let mut drift = provided.clone();
        let tail = BasicBlockId::new(77);
        let mut block = BasicBlock::new(tail);
        block.set_terminator(MirInstruction::Invoke { operation, fault_frame: frame,
            normal_landing: landing, fault_landing: landing });
        drift.add_block(block);
        assert!(state.validate_bindings(&drift).is_err(), "extra Provided release must reject");
    }
    if let ConstructionState::Selected { stores, .. } = &mut state {
        stores.get_mut(&site).unwrap().rhs = ConstructionStoreRhsV1::LiteralI64(7);
    }
    let mut literal_drift = function.clone();
    literal_drift.blocks.get_mut(&origin).unwrap().instructions[0] = MirInstruction::Const {
        dst: value,
        value: ConstValue::Integer(8),
    };
    assert!(state
        .validate_bindings(&literal_drift)
        .unwrap_err()
        .contains("literal-value-drift"));
    let mut extra_invoke = function.clone();
    let mut block = BasicBlock::new(BasicBlockId::new(8));
    block.set_terminator(MirInstruction::Invoke {
        operation: InvokeOperation::ReclaimUnpublished {
            object: field.object(),
            value: base,
        },
        fault_frame: frame,
        normal_landing: normal,
        fault_landing: landing,
    });
    extra_invoke.add_block(block);
    assert!(state
        .validate_bindings(&extra_invoke)
        .unwrap_err()
        .contains("emission-count"));
    let mut missing_fault = function.clone();
    missing_fault.blocks.remove(&landing);
    assert!(state
        .validate_bindings(&missing_fault)
        .unwrap_err()
        .contains("fault-return-count"));
    let mut extra_fault = function.clone();
    let mut block = BasicBlock::new(BasicBlockId::new(9));
    block.set_terminator(MirInstruction::ReturnFault { fault_frame: frame });
    extra_fault.add_block(block);
    assert!(state
        .validate_bindings(&extra_fault)
        .unwrap_err()
        .contains("fault-return-drift"));
    let mut extra_result = function.clone();
    extra_result
        .blocks
        .get_mut(&origin)
        .unwrap()
        .add_instruction(MirInstruction::InvokeNormalResult {
            invoke_block: origin,
            dst: ValueId::new(99),
        });
    assert!(state
        .validate_bindings(&extra_result)
        .unwrap_err()
        .contains("unexpected-normal-result"));
    for case in 0..7 {
        let mut changed = function.clone();
        let terminator = &mut changed.blocks.get_mut(&origin).unwrap().terminator;
        if case == 6 {
            *terminator = Some(MirInstruction::Return { value: None });
        } else {
            let Some(MirInstruction::Invoke {
                operation: InvokeOperation::FieldSet { mode: crate::mir::instruction::FieldSetModeV1::ExactSlot, field, base, value },
                fault_frame,
                normal_landing,
                fault_landing,
            }) = terminator
            else {
                unreachable!()
            };
            match case {
                0 => {
                    *field =
                        CanonicalFieldRefV1::from_declaration_ordinal(field.object(), 1).unwrap()
                }
                1 => *base = ValueId::new(9),
                2 => *value = ValueId::new(9),
                3 => *fault_frame = ValueId::new(9),
                4 => *normal_landing = BasicBlockId::new(9),
                5 => *fault_landing = BasicBlockId::new(9),
                _ => unreachable!(),
            }
        }
        assert!(state.validate_bindings(&changed).is_err(), "drift {case}");
    }
    // Physical-only two-step witness: exact source order, including two
    // writes to the SAME field (default followed by explicit overwrite).
    let second_site = SourcePathV1::function_body().child(SourcePathSegmentV1::Body(1)).node();
    let second_normal = BasicBlockId::new(3);
    let mut pair = function.clone();
    let mut second_block = BasicBlock::new(normal);
    let second_value = ValueId::new(3);
    second_block.add_instruction(MirInstruction::Const { dst: second_value, value: ConstValue::Integer(9) });
    second_block.set_terminator(MirInstruction::Invoke {
        operation: InvokeOperation::FieldSet { mode: crate::mir::instruction::FieldSetModeV1::ExactSlot, field, base, value: second_value },
        fault_frame: frame, normal_landing: second_normal, fault_landing: landing,
    });
    pair.add_block(second_block);
    let original_step = ConstructionSourceStepV1::Store(crate::mir::resolved_semantics::SourceStmtSiteV1::from_node(site.clone()));
    let second_step = ConstructionSourceStepV1::Store(crate::mir::resolved_semantics::SourceStmtSiteV1::from_node(second_site.clone()));
    if let ConstructionState::Selected { steps, next_step, stores, .. } = &mut state {
        let original = stores.get(&site).unwrap();
        let second = SelectedConstructionStore {
            field, receiver_site: original.receiver_site.clone(), receiver_binding,
            rhs: ConstructionStoreRhsV1::LiteralI64(9), fault_discharge: Box::new([]), me_reads: Box::new([]),
            progress: StoreProgress::Emitted {
                block: normal, normal: second_normal, base, value: second_value, provider: None,
                provider_child: None, discharge: landing,
            },
        };
        stores.insert(second_site.clone(), second);
        *steps = vec![original_step.clone(), second_step.clone()].into_boxed_slice();
        *next_step = 2;
    }
    state.validate_bindings(&pair).unwrap();
    state.finish().unwrap();
    if let ConstructionState::Selected { steps, .. } = &mut state { steps.swap(0, 1); }
    assert!(state.validate_bindings(&pair).unwrap_err().contains("physical-step-order-drift"));
    if let ConstructionState::Selected { steps, next_step, completed, .. } = &mut state {
        steps.swap(0, 1); *next_step = 0; *completed = false;
    }
    assert!(state.require_next_store(&second_site).unwrap_err().contains("step-order-or-kind"));
    if let ConstructionState::Selected { next_step, completed, .. } = &mut state { *next_step = 1; *completed = true; }
    assert!(state.finish().unwrap_err().contains("step-residual"));
    if let ConstructionState::Selected { steps, next_step, .. } = &mut state { *steps = Box::new([]); *next_step = 0; }
    assert!(state.validate_bindings(&pair).unwrap_err().contains("step-store-coverage"));
    assert!(state.finish().unwrap_err().contains("step-store-coverage"));
    if let ConstructionState::Selected { steps, next_step, .. } = &mut state {
        *steps = vec![original_step.clone(), original_step.clone()].into_boxed_slice(); *next_step = 2;
    }
    assert!(state.validate_bindings(&pair).unwrap_err().contains("step-store-coverage"));
    if let ConstructionState::Selected { steps, next_step, stores, .. } = &mut state {
        stores.remove(&second_site); *steps = vec![original_step].into_boxed_slice(); *next_step = 1;
    }
    let ConstructionState::Selected {
        stores, completed, ..
    } = &mut state
    else {
        unreachable!()
    };
    *completed = false;
    stores.get_mut(&site).unwrap().progress = StoreProgress::Taken;
    assert!(state.finish().unwrap_err().contains("completion-missing"));
    assert!(state
        .validate_bindings(&function)
        .unwrap_err()
        .contains("store-residual"));
}

impl RetainedConstructionValidation {
    /// Physical transport witness only; never source acceptance evidence.
    pub(in crate::mir::builder) fn empty_for_transport_test() -> Self {
        let mut owners =
            crate::mir::resolved_semantics::FunctionOwnerIssuerV1::new_for_compilation().unwrap();
        Self {
            owner: owners.issue().unwrap(),
            construction: ConstructionState::Selected {
                steps: Box::new([]), next_step: 0, calls: Vec::new(),
                stores: BTreeMap::new(),
                frame: None,
                completed: true,
            },
            fault_frame: crate::mir::builder::function_fault_frame::FunctionFaultFrameV1::borrowed(
            ),
        }
    }
}

#[test]
fn artifact_requires_completed_source_construction() {
    let function = MirFunction::new(
        FunctionSignature {
            name: "empty_construction_validation".into(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    RetainedConstructionValidation::empty_for_transport_test()
        .validate_artifact_after_compiler_finishing(&function)
        .unwrap();
    for state in [
        ConstructionState::NotConstruction,
        ConstructionState::Transferred,
        ConstructionState::RetainedUnavailable(ConstructionUnavailableV1::BodyCoverageUnsupported),
        ConstructionState::Selected {
            steps: Box::new([]), next_step: 0, calls: Vec::new(),
            stores: BTreeMap::new(),
            frame: None,
            completed: false,
        },
    ] {
        let mut retained = RetainedConstructionValidation::empty_for_transport_test();
        retained.construction = state;
        assert!(retained
            .validate_artifact_after_compiler_finishing(&function)
            .is_err());
    }
}
