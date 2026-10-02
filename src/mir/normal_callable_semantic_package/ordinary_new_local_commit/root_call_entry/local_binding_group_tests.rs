//! Local prefixes keep one original packet through sibling views and final artifacts.
use super::super::RootLocalCallBindingGroupV1;
use super::*;
use std::rc::Rc;

fn packet() -> EmittedLexicalCallProjectionV1 {
    let (prepared, row, source) = crate::mir::builder::lexical_call_projection_test_fixture();
    let call = prepared
        .materialize(row.call_site().owner(), &row, &source)
        .unwrap();
    EmittedLexicalCallProjectionV1::new(
        row,
        prepared,
        (
            BasicBlockId(4),
            MirInstruction::Invoke {
                operation: InvokeOperation::Call {
                    call,
                    result: InvokeCallResultKind::I64,
                },
                fault_frame: ValueId(90),
                normal_landing: BasicBlockId(5),
                fault_landing: BasicBlockId(6),
            },
        ),
        (
            BasicBlockId(5),
            MirInstruction::InvokeNormalResult {
                invoke_block: BasicBlockId(4),
                dst: ValueId(82),
            },
        ),
    )
}

fn records(packet: &EmittedLexicalCallProjectionV1) -> Vec<Binding> {
    let mut result = vec![packet.invoke.clone(), packet.projection.clone()];
    for argument in &packet.prepared.arguments {
        match argument {
            LexicalCallArgumentProjectionV1::Integer(binding) => result.push(binding.clone()),
            LexicalCallArgumentProjectionV1::Scalar(_) => {}
            LexicalCallArgumentProjectionV1::CallResult(inner) => result.extend(records(inner)),
            _ => panic!("strict fixture contains no borrowed arguments"),
        }
    }
    result
}

#[test]
fn local_prefix_sibling_views_share_the_original_packet_without_rebinding_it() {
    let packet = Rc::new(packet());
    let original = records(&packet);
    let group = RootLocalCallBindingGroupV1::new(
        packet.call_site().clone(),
        original.clone(),
        Some(packet.clone()),
    )
    .unwrap();
    let sibling = group.clone();
    assert!(std::ptr::eq(
        group.lexical().unwrap(),
        sibling.lexical().unwrap()
    ));
    let mapped = group.with_bindings(
        original
            .iter()
            .map(|(block, instruction)| (BasicBlockId(block.0 + 100), instruction.clone()))
            .collect(),
    );
    assert!(std::ptr::eq(
        group.lexical().unwrap(),
        mapped.lexical().unwrap()
    ));
    assert_eq!(
        records(mapped.lexical().unwrap()),
        original,
        "shared proof is immutable"
    );
    assert_eq!(group.bindings(), original.as_slice());
    assert_ne!(mapped.bindings(), original.as_slice());
}

#[test]
fn local_prefix_rejects_foreign_packet_site_and_missing_or_duplicate_producer() {
    let packet = Rc::new(packet());
    let original = records(&packet);
    let foreign_owner =
        crate::mir::resolved_semantics::FunctionOwnerIssuerV1::new_for_compilation()
            .unwrap()
            .issue()
            .unwrap();
    let foreign = crate::mir::resolved_semantics::OwnedExprSiteV1::new(
        foreign_owner,
        packet.call_site().site().clone(),
    );
    assert!(
        RootLocalCallBindingGroupV1::new(foreign, original.clone(), Some(packet.clone())).is_err()
    );
    for index in 0..original.len() {
        let mut missing = original.clone();
        missing.remove(index);
        assert!(RootLocalCallBindingGroupV1::new(
            packet.call_site().clone(),
            missing,
            Some(packet.clone())
        )
        .is_err());
        let mut duplicate = original.clone();
        duplicate.push(original[index].clone());
        assert!(RootLocalCallBindingGroupV1::new(
            packet.call_site().clone(),
            duplicate,
            Some(packet.clone())
        )
        .is_err());
    }
}

#[test]
fn source_local_nested_call_packet_survives_plain_return_final_handoff() {
    let (_module, handoff) = crate::mir::builder::lexical_call_projection_artifact_fixture();
    let source = handoff.root_source().unwrap();
    let groups: Vec<_> = source.local_call_binding_groups().collect();
    assert_eq!(
        groups.len(),
        1,
        "one original group, not a nested site inventory"
    );
    let (owner, group) = groups[0];
    assert_eq!(owner, group.site().owner());
    let packet = group.lexical().expect("original lexical packet retained");
    assert_eq!(packet.call_site(), group.site());
    packet.validate_recorded(group.bindings()).unwrap();
    assert!(packet
        .prepared
        .arguments
        .iter()
        .any(|argument| matches!(argument, LexicalCallArgumentProjectionV1::CallResult(_))));
    assert_eq!(
        group
            .bindings()
            .iter()
            .filter(|(_, instruction)| matches!(
                instruction,
                MirInstruction::Invoke {
                    operation: InvokeOperation::Call { .. },
                    ..
                }
            ))
            .count(),
        2
    );
}

#[test]
fn source_local_nested_call_handoff_rejects_second_seal() {
    crate::mir::builder::assert_lexical_call_projection_reseal_rejected();
}
