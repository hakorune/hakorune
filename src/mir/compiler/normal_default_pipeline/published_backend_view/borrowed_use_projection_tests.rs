//! Original source proofs checking synthetic published projections through the
//! production consumer. These are not ABI, source-to-publication or EXE proofs.
use super::*;

fn project(function: &MirFunction) -> PublishedLifecyclePhysicalFunctionV1<'_> {
    let row = &function.blocks[&function.entry_block];
    PublishedLifecyclePhysicalFunctionV1 {
        name: &function.signature.name,
        // The synthetic header is not evidence for instance receiver admission.
        role: PublishedLifecyclePhysicalFunctionRoleV1::OrdinaryI64 {
            key: hakorune_mir_defs::CanonicalSameModuleCallableKeyV1::instance_box_method(
                "Transport",
                "forward",
                1,
            ),
            receiver_object: None,
        },
        params: &function.params,
        param_types: &function.signature.params,
        param_carriers: None,
        value_types: &function.metadata.value_types,
        entry: function.entry_block,
        blocks: vec![PublishedLifecyclePhysicalBlockV1 {
            id: row.id,
            instructions: row
                .instructions
                .iter()
                .enumerate()
                .map(
                    |(index, instruction)| PublishedLifecyclePhysicalInstructionRefV1 {
                        index: index as u32,
                        instruction,
                        field_ref: None,
                    },
                )
                .collect(),
            terminator: PublishedLifecyclePhysicalInstructionRefV1 {
                index: row.instructions.len() as u32,
                instruction: row.terminator.as_ref().unwrap(),
                field_ref: None,
            },
            edges: Box::new([]),
        }]
        .into(),
    }
}

#[test]
fn borrowed_projection_checks_function_block_and_coordinate_identity() {
    let (function, check) =
        super::super::compiled_entry_contract::borrowed_projection_test_fixture();
    let row = project(&function);
    check(std::slice::from_ref(&row)).unwrap();
    assert!(check(&[]).unwrap_err().contains("published-missing"));
    assert!(check(&[row.clone(), row.clone()])
        .unwrap_err()
        .contains("published-duplicate"));
    let mut changed = row.clone();
    changed.name = "foreign";
    assert!(check(&[changed]).unwrap_err().contains("published-missing"));
    let mut changed = row.clone();
    changed.blocks = vec![row.blocks[0].clone(), row.blocks[0].clone()].into();
    assert!(check(&[changed])
        .unwrap_err()
        .contains("published-block-duplicate"));
    let mut changed = row.clone();
    changed.blocks[0].terminator.index = changed.blocks[0].instructions[0].index;
    assert!(check(&[changed])
        .unwrap_err()
        .contains("coordinate-duplicate"));
    let mut changed = row.clone();
    changed.blocks[0].instructions[0].index += 10;
    assert!(check(&[changed]).unwrap_err().contains("copy-definition"));
    let mut changed = row;
    changed.blocks[0].id = BasicBlockId(99);
    assert!(check(&[changed]).unwrap_err().contains("copy-definition"));
}

#[test]
fn borrowed_projection_rejects_missing_copy_call_and_parameter_drift() {
    let (function, check) =
        super::super::compiled_entry_contract::borrowed_projection_test_fixture();
    let row = project(&function);
    check(std::slice::from_ref(&row)).unwrap();
    let mut changed = row.clone();
    changed.blocks[0].instructions = Box::new([]);
    assert!(check(&[changed])
        .unwrap_err()
        .contains("definition-coverage"));
    let returned = MirInstruction::Return { value: None };
    let mut changed = row.clone();
    changed.blocks[0].terminator.instruction = &returned;
    assert!(check(&[changed]).unwrap_err().contains("argument-coverage"));
    let mut changed = row.clone();
    changed.blocks = Box::new([]);
    assert!(check(&[changed]).unwrap_err().contains("argument-coverage"));
    let duplicate = [function.params[0], function.params[0]];
    let mut changed = row.clone();
    changed.params = &duplicate;
    assert!(check(&[changed])
        .unwrap_err()
        .contains("definition-coverage"));
    let mut changed = row;
    changed.params = &[];
    assert!(check(&[changed])
        .unwrap_err()
        .contains("definition-coverage"));
}

#[test]
fn borrowed_projection_rejects_copied_edge_leaks_independently_of_terminator() {
    let (function, check) =
        super::super::compiled_entry_contract::borrowed_projection_test_fixture();
    let row = project(&function);
    check(std::slice::from_ref(&row)).unwrap();
    let MirInstruction::Copy { dst, .. } = row.blocks[0].instructions[0].instruction else {
        panic!("original alias Copy");
    };
    for value in [function.params[0], *dst] {
        let mut changed = row.clone();
        changed.blocks[0].edges = vec![PublishedLifecyclePhysicalEdgeV1 {
            target: BasicBlockId(1),
            args: Some(EdgeArgs {
                layout: crate::mir::edge_args::JumpArgsLayout::CarriersOnly,
                values: vec![value],
            }),
        }]
        .into();
        assert!(check(&[changed]).unwrap_err().contains("published-edge"));
    }
}

#[test]
fn borrowed_projection_rejects_receiver_fault_frame_and_argument_mutations() {
    let (function, check) =
        super::super::compiled_entry_contract::borrowed_projection_test_fixture();
    let row = project(&function);
    check(std::slice::from_ref(&row)).unwrap();
    for mutation in 0..4 {
        let mut altered = row.blocks[0].terminator.instruction.clone();
        let MirInstruction::Invoke {
            operation: InvokeOperation::Call { call, .. },
            fault_frame,
            ..
        } = &mut altered
        else {
            panic!("original Invoke");
        };
        match mutation {
            0 => {
                let Callee::SameModuleInstance { receiver, .. } = &mut call.callee else {
                    panic!("instance receiver");
                };
                *receiver = function.params[0];
            }
            1 => *fault_frame = function.params[0],
            2 => call.args[0] = ValueId(999),
            _ => call.args.clear(),
        }
        let mut changed = row.clone();
        changed.blocks[0].terminator.instruction = &altered;
        let error = check(&[changed]).unwrap_err();
        assert!(
            error.contains(match mutation {
                0 | 1 => "callee-or-fault-frame",
                2 => "borrowed-use/argument",
                _ => "argument-missing",
            }),
            "{error}"
        );
    }
}
