use super::*;
use crate::mir::normal_callable_semantic_package::ConstructionStoreRhsV1;
use crate::mir::resolved_semantics::{
    BindingRefV1, FunctionOwnerIssuerV1, SourcePathSegmentV1, SourcePathV1,
};
use crate::mir::{ConstValue, EffectMask, FunctionSignature, MirType};

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
        operation: InvokeOperation::FieldSet { field, base, value },
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
        stores: BTreeMap::from([(
            site.clone(),
            SelectedConstructionStore {
                field,
                receiver_site,
                receiver_binding,
                rhs: ConstructionStoreRhsV1::LiteralI64(7),
                progress: StoreProgress::Emitted {
                    block: origin,
                    normal,
                    base,
                    value,
                },
            },
        )]),
        frame: Some((frame, landing)),
        completed: true,
    };
    state.validate_bindings(&function).unwrap();
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
        operation: InvokeOperation::ReclaimUnpublished { object: field.object(), value: base },
        fault_frame: frame,
        normal_landing: normal,
        fault_landing: landing,
    });
    extra_invoke.add_block(block);
    assert!(state.validate_bindings(&extra_invoke).unwrap_err().contains("emission-count"));
    let mut missing_fault = function.clone();
    missing_fault.blocks.remove(&landing);
    assert!(state.validate_bindings(&missing_fault).unwrap_err().contains("fault-return-count"));
    let mut extra_fault = function.clone();
    let mut block = BasicBlock::new(BasicBlockId::new(9));
    block.set_terminator(MirInstruction::ReturnFault { fault_frame: frame });
    extra_fault.add_block(block);
    assert!(state.validate_bindings(&extra_fault).unwrap_err().contains("fault-return-drift"));
    let mut extra_result = function.clone();
    extra_result.blocks.get_mut(&origin).unwrap().add_instruction(
        MirInstruction::InvokeNormalResult { invoke_block: origin, dst: ValueId::new(99) });
    assert!(state.validate_bindings(&extra_result).unwrap_err().contains("unexpected-normal-result"));
    for case in 0..7 {
        let mut changed = function.clone();
        let terminator = &mut changed.blocks.get_mut(&origin).unwrap().terminator;
        if case == 6 {
            *terminator = Some(MirInstruction::Return { value: None });
        } else {
            let Some(MirInstruction::Invoke {
                operation: InvokeOperation::FieldSet { field, base, value },
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
                stores: BTreeMap::new(),
                frame: None,
                completed: true,
            },
            fault_frame: super::super::fault::CallableFaultFrame::borrowed(),
        }
    }
}

#[test]
fn artifact_requires_completed_source_construction() {
    let function = MirFunction::new(FunctionSignature {
        name: "empty_construction_validation".into(), params: vec![],
        return_type: MirType::Void, effects: EffectMask::PURE,
    }, BasicBlockId::new(0));
    RetainedConstructionValidation::empty_for_transport_test()
        .validate_artifact_after_compiler_finishing(&function).unwrap();
    for state in [ConstructionState::NotConstruction, ConstructionState::Transferred,
        ConstructionState::RetainedUnavailable(ConstructionUnavailableV1::BodyCoverageUnsupported),
        ConstructionState::Selected { stores: BTreeMap::new(), frame: None, completed: false }] {
        let mut retained = RetainedConstructionValidation::empty_for_transport_test();
        retained.construction = state;
        assert!(retained.validate_artifact_after_compiler_finishing(&function).is_err());
    }
}
