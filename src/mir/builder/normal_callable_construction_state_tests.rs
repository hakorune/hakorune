use super::*;
use crate::mir::resolved_semantics::SourcePathV1;
use crate::mir::{EffectMask, FunctionSignature, MirType};

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
    block.set_terminator(MirInstruction::Invoke {
        operation: InvokeOperation::FieldSet { field, base, value },
        fault_frame: frame,
        normal_landing: normal,
        fault_landing: landing,
    });
    function.add_block(block);
    let site = SourcePathV1::function_body().node();
    let mut state = ConstructionState::Selected {
        stores: BTreeMap::from([(
            site.clone(),
            (
                field,
                StoreProgress::Emitted {
                    block: origin,
                    normal,
                    base,
                    value,
                },
            ),
        )]),
        frame: Some((frame, landing)),
        completed: true,
    };
    state.validate_bindings(&function).unwrap();
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
    stores.get_mut(&site).unwrap().1 = StoreProgress::Taken;
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
