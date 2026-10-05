//! Final-MIR counterexamples for a nonempty retained source inventory.
use super::*;
use crate::mir::normal_callable_semantic_package::{OwnedFieldChildKindV1, OwnedFieldChildV1};
use crate::mir::{BasicBlock, EffectMask, FunctionSignature, MirType};

#[test]
fn prior_committed_cleanup_rejects_missing_reordered_and_duplicate_fields() {
    // Physical witness only: source issuance has independent package tests.
    let object = hakorune_mir_defs::CanonicalObjectIdV1::from_declaration_index(0).unwrap();
    let fields = [1, 0].map(|ordinal| OwnedFieldChildV1 {
        field: hakorune_mir_defs::CanonicalFieldRefV1::from_declaration_ordinal(object, ordinal)
            .unwrap(),
        kind: OwnedFieldChildKindV1::Array,
    });
    let base = ValueId::new(0);
    let frame = ValueId::new(1);
    let head = BasicBlockId::new(0);
    let tail = BasicBlockId::new(6);
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "retained_partial_cleanup".into(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::WRITE,
        },
        head,
    );
    for (index, child) in fields.iter().enumerate() {
        let id = BasicBlockId::new(index as u32 * 3);
        let normal = BasicBlockId::new(id.0 + 1);
        let fault = BasicBlockId::new(id.0 + 2);
        let next = BasicBlockId::new(id.0 + 3);
        let mut block = BasicBlock::new(id);
        block.set_terminator(MirInstruction::Invoke {
            operation: InvokeOperation::OwnedFieldResidenceRelease {
                field: child.field,
                base,
            },
            fault_frame: frame,
            normal_landing: normal,
            fault_landing: fault,
        });
        function.add_block(block);
        for landing in [normal, fault] {
            let mut block = BasicBlock::new(landing);
            block.set_terminator(MirInstruction::Jump {
                target: next,
                edge_args: None,
            });
            function.add_block(block);
        }
    }
    let check = |function: &MirFunction| {
        field_discharge_chain(function, head, &fields, base, Some((frame, tail)))
    };
    assert_eq!(check(&function), Some(tail));
    let mut missing = function.clone();
    missing.blocks.remove(&BasicBlockId::new(3));
    assert_eq!(check(&missing), None);
    for reordered in [false, true] {
        let mut changed = function.clone();
        for (index, replacement) in if reordered {
            [fields[1], fields[0]]
        } else {
            [fields[0], fields[0]]
        }
        .iter()
        .enumerate()
        {
            let Some(MirInstruction::Invoke { operation, .. }) = changed
                .blocks
                .get_mut(&BasicBlockId::new(index as u32 * 3))
                .unwrap()
                .terminator
                .as_mut()
            else {
                unreachable!()
            };
            *operation = InvokeOperation::OwnedFieldResidenceRelease {
                field: replacement.field,
                base,
            };
        }
        assert_eq!(check(&changed), None);
    }
    let mut bypass = function.clone();
    bypass
        .blocks
        .get_mut(&BasicBlockId::new(2))
        .unwrap()
        .set_terminator(MirInstruction::Jump {
            target: tail,
            edge_args: None,
        });
    assert_eq!(
        check(&bypass),
        None,
        "Fault must discharge the same remaining fields"
    );
}

#[test]
fn object_fault_discharge_rejects_foreign_child_base_frame_and_field() {
    use hakorune_mir_defs::{CanonicalFieldRefV1, CanonicalObjectIdV1};
    let parent = CanonicalObjectIdV1::from_declaration_index(1).unwrap();
    let child = CanonicalObjectIdV1::from_declaration_index(0).unwrap();
    let field = CanonicalFieldRefV1::from_declaration_ordinal(parent, 0).unwrap();
    let inventory = [OwnedFieldChildV1 {
        field,
        kind: OwnedFieldChildKindV1::Object(child),
    }];
    let base = ValueId(0);
    let frame = ValueId(1);
    let head = BasicBlockId(0);
    let tail = BasicBlockId(3);
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "object_fault_discharge".into(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::WRITE,
        },
        head,
    );
    let mut block = BasicBlock::new(head);
    block.set_terminator(MirInstruction::Invoke {
        operation: InvokeOperation::OwnedObjectFieldRelease { field, base, child },
        fault_frame: frame,
        normal_landing: BasicBlockId(1),
        fault_landing: BasicBlockId(2),
    });
    function.add_block(block);
    for id in [1, 2] {
        let mut landing = BasicBlock::new(BasicBlockId(id));
        landing.set_terminator(MirInstruction::Jump {
            target: tail,
            edge_args: None,
        });
        function.add_block(landing);
    }
    let check = |function: &MirFunction| {
        field_discharge_chain(function, head, &inventory, base, Some((frame, tail)))
    };
    assert_eq!(
        check(&function),
        Some(tail),
        "canonical child zero is valid"
    );
    for drift in ["child", "base", "frame", "field", "fault-continuation"] {
        let mut changed = function.clone();
        let MirInstruction::Invoke {
            operation,
            fault_frame,
            fault_landing,
            ..
        } = changed
            .blocks
            .get_mut(&head)
            .unwrap()
            .terminator
            .as_mut()
            .unwrap()
        else {
            unreachable!()
        };
        let InvokeOperation::OwnedObjectFieldRelease { field, base, child } = operation else {
            unreachable!()
        };
        match drift {
            "child" => *child = CanonicalObjectIdV1::from_declaration_index(2).unwrap(),
            "base" => *base = ValueId(9),
            "frame" => *fault_frame = ValueId(9),
            "field" => *field = CanonicalFieldRefV1::from_declaration_ordinal(parent, 1).unwrap(),
            "fault-continuation" => *fault_landing = tail,
            _ => unreachable!(),
        }
        assert_eq!(check(&changed), None, "{drift}");
    }
}
