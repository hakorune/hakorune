//! Original source/packet transport tests; tagged execution remains closed.
use super::super::super::super::{
    physical_boundary::PhysicalBoundary, FinalizedRootSourceHandoffV1, RootNewValidation,
};
use super::super::super::{RootHomeExitEntry, RootHomeExitProgress};
use super::*;
use crate::mir::normal_callable_semantic_package::{
    OrdinaryNewClaimLedgerV1, RootCallDispositionV1,
};
use crate::mir::resolved_semantics::SourceStmtSiteV1;
use crate::mir::{BasicBlock, EffectMask, FunctionSignature, MirFunction, MirType};
use std::rc::Rc;

type Fixture = (
    Rc<OrdinaryNewClaimLedgerV1>,
    SourceStmtSiteV1,
    Rc<EmittedLexicalCallProjectionV1>,
    Vec<Binding>,
);

fn fixture() -> Fixture {
    let (mut prepared, row, source, ledger, mut arguments) =
        super::borrowed_projection_tests::fixture("true", "");
    let exit = ledger.root_completion_for_test().explicit_sites()[0].clone();
    // Place the same emitted literal at the original pre-contraction block.
    let LexicalCallArgumentProjectionV1::BorrowedLiteral { binding, .. } =
        &mut prepared.arguments[0]
    else {
        panic!("original Boolean producer");
    };
    binding.0 = BasicBlockId(10);
    arguments[0].0 = BasicBlockId(10);
    let call = prepared
        .materialize_with_ledger(row.call_site().owner(), &row, &source, &ledger)
        .unwrap();
    let packet = Rc::new(EmittedLexicalCallProjectionV1::new(
        row,
        prepared,
        (
            BasicBlockId(10),
            MirInstruction::Invoke {
                operation: InvokeOperation::Call {
                    call,
                    result: InvokeCallResultKind::I64,
                },
                fault_frame: ValueId(900),
                normal_landing: BasicBlockId(11),
                fault_landing: BasicBlockId(12),
            },
        ),
        (
            BasicBlockId(11),
            MirInstruction::InvokeNormalResult {
                invoke_block: BasicBlockId(10),
                dst: ValueId(901),
            },
        ),
    ));
    (ledger, exit, packet, arguments)
}

pub(super) fn function(
    packet: &EmittedLexicalCallProjectionV1,
    arguments: &[Binding],
) -> (MirFunction, Binding, Vec<Binding>) {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "terminal-projection".into(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId(0),
    );
    let frame = (
        BasicBlockId(0),
        MirInstruction::FaultFrameEnter {
            dst: ValueId(900),
            mode: crate::mir::instruction::FaultFrameMode::RootOwned,
        },
    );
    let jump = (
        BasicBlockId(0),
        MirInstruction::Jump {
            target: BasicBlockId(10),
            edge_args: None,
        },
    );
    let normal = (
        BasicBlockId(11),
        MirInstruction::Return {
            value: Some(ValueId(901)),
        },
    );
    let fault = (
        BasicBlockId(12),
        MirInstruction::ReturnFault {
            fault_frame: ValueId(900),
        },
    );
    for id in [0, 10, 11, 12] {
        function.add_block(BasicBlock::new(BasicBlockId(id)));
    }
    for (block, instruction) in arguments.iter().chain([
        &frame,
        &jump,
        &packet.invoke,
        &packet.projection,
        &normal,
        &fault,
    ]) {
        if matches!(
            instruction,
            MirInstruction::Invoke { .. }
                | MirInstruction::Jump { .. }
                | MirInstruction::Return { .. }
                | MirInstruction::ReturnFault { .. }
        ) {
            function
                .blocks
                .get_mut(block)
                .unwrap()
                .set_terminator(instruction.clone());
        } else {
            function
                .blocks
                .get_mut(block)
                .unwrap()
                .add_instruction(instruction.clone());
        }
    }
    (function, frame.clone(), vec![frame, jump, normal, fault])
}

fn record(
    ledger: &OrdinaryNewClaimLedgerV1,
    exit: &SourceStmtSiteV1,
    packet: Rc<EmittedLexicalCallProjectionV1>,
    arguments: Vec<Binding>,
    frame: Binding,
    cleanup: Vec<Binding>,
) {
    let owner = packet.call_site().owner();
    ledger.root_exits.borrow_mut().insert(
        (owner, exit.clone()),
        RootHomeExitProgress::Emitting(
            super::super::RootHomeCleanupOrderV1::ordinary(Vec::new()).unwrap(),
        ),
    );
    ledger
        .record_root_call_exit(
            owner,
            exit,
            RootCallDispositionV1::Lexical(Rc::clone(&packet)),
            arguments,
            packet.invoke.clone(),
            packet.projection.clone(),
            frame,
            vec![],
            cleanup,
        )
        .unwrap();
}

#[test]
fn lexical_return_record_keeps_original_packet_and_moves_affinely() {
    let (ledger, exit, packet, arguments) = fixture();
    let (_, frame, cleanup) = function(&packet, &arguments);
    let owner = packet.call_site().owner();
    ledger
        .validate_lexical_terminal_packet(owner, &exit, &packet)
        .unwrap();
    record(
        &ledger,
        &exit,
        Rc::clone(&packet),
        arguments,
        frame,
        cleanup,
    );
    let (entry, _) = ledger
        .take_finalized_root_call(owner, &exit)
        .unwrap()
        .unwrap();
    let RootHomeExitEntry::Call {
        row: RootCallDispositionV1::Lexical(retained),
        ..
    } = entry
    else {
        panic!("packet");
    };
    assert!(Rc::ptr_eq(&packet, &retained));
    assert_eq!(
        retained.original_row().unwrap().call_site(),
        packet.call_site()
    );
    assert!(ledger
        .take_finalized_root_call(owner, &exit)
        .unwrap_err()
        .contains("already-finalized"));
}

#[test]
fn lexical_return_refuses_foreign_exit_lender_and_changed_original_producer() {
    for mutation in 0..3 {
        let (ledger, exit, packet, _) = fixture();
        let owner = packet.call_site().owner();
        let mut packet = Rc::try_unwrap(packet).unwrap();
        match mutation {
            0 => packet.projection.0 = BasicBlockId(99),
            1 => {
                let MirInstruction::Invoke {
                    operation: InvokeOperation::Call { call, .. },
                    ..
                } = &mut packet.invoke.1
                else {
                    panic!("Invoke");
                };
                call.args[0] = ValueId(999);
            }
            _ => {
                let (foreign, _, _, _) = fixture();
                assert!(foreign
                    .validate_lexical_terminal_packet(owner, &exit, &packet)
                    .is_err());
                continue;
            }
        }
        assert!(ledger
            .validate_lexical_terminal_packet(owner, &exit, &packet)
            .is_err());
    }
    let (ledger, _, packet, _) = fixture();
    let wrong_exit = SourceStmtSiteV1::from_node(
        packet
            .original_row()
            .unwrap()
            .receiver_site()
            .node()
            .clone(),
    );
    assert!(ledger
        .validate_lexical_terminal_packet(packet.call_site().owner(), &wrong_exit, &packet)
        .is_err());
}

#[test]
fn lexical_return_finished_lender_preserves_original_coordinates_after_rebind() {
    let (ledger, exit, packet, arguments) = fixture();
    let owner = packet.call_site().owner();
    let (original, frame, cleanup) = function(&packet, &arguments);
    let bindings: Vec<_> = arguments
        .iter()
        .chain(cleanup.iter())
        .chain([&packet.invoke, &packet.projection])
        .cloned()
        .collect();
    let boundary = PhysicalBoundary::capture(&original, &bindings).unwrap();
    let mut finished = original.clone();
    let child = finished.blocks.remove(&BasicBlockId(10)).unwrap();
    let entry = finished.blocks.get_mut(&BasicBlockId(0)).unwrap();
    entry.instructions.extend(child.instructions);
    entry.terminator = child.terminator;
    let MirInstruction::InvokeNormalResult { invoke_block, .. } = &mut finished
        .blocks
        .get_mut(&BasicBlockId(11))
        .unwrap()
        .instructions[0]
    else {
        panic!("projection");
    };
    *invoke_block = BasicBlockId(0);
    let mut projection = boundary.project(&finished).unwrap();
    boundary
        .validate_complete(&finished, &mut projection, &bindings)
        .unwrap();
    record(
        &ledger,
        &exit,
        Rc::clone(&packet),
        arguments,
        frame,
        cleanup,
    );
    ledger
        .rebind_root_call_entry(owner, &exit, &projection)
        .unwrap();
    let (entry, cleanup) = ledger
        .take_finalized_root_call(owner, &exit)
        .unwrap()
        .unwrap();
    let RootHomeExitEntry::Call {
        invoke,
        projection: result,
        row: RootCallDispositionV1::Lexical(retained),
        ..
    } = &entry
    else {
        panic!("packet");
    };
    assert_eq!(invoke.0, BasicBlockId(0));
    assert!(matches!(
        result.1,
        MirInstruction::InvokeNormalResult {
            invoke_block: BasicBlockId(0),
            ..
        }
    ));
    assert!(Rc::ptr_eq(retained, &packet));
    assert_eq!(packet.invoke.0, BasicBlockId(10));
    *ledger.root_validation.borrow_mut() = RootNewValidation::ArtifactFinalized {
        owner,
        symbol: finished.signature.name.clone(),
        projection: Rc::new(projection),
    };
    let source = FinalizedRootSourceHandoffV1 {
        ledger: Rc::clone(&ledger),
        app_main_identity: ledger.app_main_identity.as_ref().unwrap().clone(),
        owner,
        terminals: ledger.terminal_relation.clone(),
        call_entries: [(exit.clone(), (entry, cleanup))].into(),
        local_calls: Default::default(),
    };
    let coordinate = source
        .finished_terminal_call_producer_v1(
            owner,
            &exit,
            packet.call_site(),
            &packet.invoke,
            &finished,
        )
        .unwrap();
    assert_eq!(coordinate.0, BasicBlockId(0));
    let false_original = (BasicBlockId(0), packet.invoke.1.clone());
    assert!(source
        .finished_terminal_call_producer_v1(
            owner,
            &exit,
            packet.call_site(),
            &false_original,
            &finished
        )
        .unwrap_err()
        .contains("original-producer"));
    let mut foreign = finished.clone();
    foreign.signature.name.push_str("foreign");
    assert!(source
        .finished_terminal_call_producer_v1(
            owner,
            &exit,
            packet.call_site(),
            &packet.invoke,
            &foreign
        )
        .is_err());
    let mut missing = finished.clone();
    missing.blocks.get_mut(&BasicBlockId(0)).unwrap().terminator =
        Some(MirInstruction::Return { value: None });
    assert!(source
        .finished_terminal_call_producer_v1(
            owner,
            &exit,
            packet.call_site(),
            &packet.invoke,
            &missing
        )
        .is_err());
}

#[test]
fn finalized_call_visitor_lends_original_return_and_demands_actual_function() {
    use crate::mir::normal_callable_semantic_package::FinalizedLexicalCallContextV1 as Context;
    let (ledger, exit, packet, arguments) = fixture();
    let owner = packet.call_site().owner();
    let (physical, frame, cleanup) = function(&packet, &arguments);
    let bindings: Vec<_> = arguments
        .iter()
        .chain(cleanup.iter())
        .chain([&packet.invoke, &packet.projection])
        .cloned()
        .collect();
    let boundary = PhysicalBoundary::capture(&physical, &bindings).unwrap();
    let mut projection = boundary.project(&physical).unwrap();
    boundary
        .validate_complete(&physical, &mut projection, &bindings)
        .unwrap();
    record(
        &ledger,
        &exit,
        Rc::clone(&packet),
        arguments,
        frame,
        cleanup,
    );
    let (entry, cleanup) = ledger
        .take_finalized_root_call(owner, &exit)
        .unwrap()
        .unwrap();
    *ledger.root_validation.borrow_mut() = RootNewValidation::ArtifactFinalized {
        owner,
        symbol: physical.signature.name.clone(),
        projection: Rc::new(projection),
    };
    let source = FinalizedRootSourceHandoffV1 {
        ledger: Rc::clone(&ledger),
        app_main_identity: ledger.app_main_identity.as_ref().unwrap().clone(),
        owner,
        terminals: ledger.terminal_relation.clone(),
        call_entries: [(exit.clone(), (entry, cleanup))].into(),
        local_calls: Default::default(),
    };
    let mut module = crate::mir::MirModule::new("return-loan".into());
    module
        .functions
        .insert(physical.signature.name.clone(), physical);
    let mut visited = 0;
    source
        .visit_finalized_lexical_call_nodes_v1(
            &module,
            |observed_owner, context, original, _, _, coordinate, copies| {
                let Context::Return {
                    exit: observed_exit,
                } = context
                else {
                    panic!("Return");
                };
                assert_eq!(observed_exit, &exit);
                assert_eq!(observed_owner, owner);
                assert!(std::ptr::eq(original, packet.as_ref()));
                assert_eq!(coordinate.0, BasicBlockId(10));
                assert!(
                    copies.is_empty(),
                    "this Return has no borrowed Copy dependencies"
                );
                visited += 1;
                Ok(())
            },
        )
        .unwrap();
    assert_eq!(visited, 1);
    module
        .functions
        .values_mut()
        .next()
        .unwrap()
        .signature
        .name
        .push_str("-foreign");
    assert!(source
        .visit_finalized_lexical_call_nodes_v1(&module, |_, _, _, _, _, _, _| Ok(()))
        .unwrap_err()
        .contains("/function"));
}

#[test]
fn borrowed_terminal_call_frame_role_stays_bound_to_original_root_owner() {
    let (ledger, exit, packet, arguments) = fixture();
    let owner = packet.call_site().owner();
    assert_eq!(ledger.root_owner(), Some(owner));
    let (mut function, frame, cleanup) = function(&packet, &arguments);
    // The fixture's full binding inventory includes entry setup; only the
    // Normal/Fault continuations belong to the exit's cleanup inventory.
    let cleanup: Vec<_> = cleanup
        .into_iter()
        .filter(|(block, _)| *block != function.entry_block)
        .collect();
    let mut entry = RootHomeExitEntry::Call {
        local_bindings: Vec::new(),
        row: RootCallDispositionV1::Lexical(Rc::clone(&packet)),
        arguments,
        invoke: packet.invoke.clone(),
        projection: packet.projection.clone(),
        frame,
    };
    ledger
        .validate_call_entry(owner, &exit, &function, None, &entry, &cleanup)
        .unwrap();
    let RootHomeExitEntry::Call { frame, .. } = &mut entry else {
        unreachable!()
    };
    let MirInstruction::FaultFrameEnter { mode, .. } = &mut frame.1 else {
        unreachable!()
    };
    *mode = crate::mir::instruction::FaultFrameMode::Borrowed;
    function.blocks.get_mut(&frame.0).unwrap().instructions[0] = frame.1.clone();
    let error = ledger
        .validate_call_entry(owner, &exit, &function, None, &entry, &cleanup)
        .unwrap_err();
    assert!(error.contains("call-frame-drift"), "{error}");
}
