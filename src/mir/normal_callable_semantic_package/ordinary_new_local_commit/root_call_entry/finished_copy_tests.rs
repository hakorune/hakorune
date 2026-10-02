//! Real child Forwarded source and synthetic physical finishing; ABI remains closed.
use super::super::super::super::physical_boundary::PhysicalBoundary;
use super::super::super::super::RootLocalCallBindingGroupV1;
use super::super::super::{RootHomeExitEntry, RootHomeExitProgress};
use super::terminal_tests::function;
use super::*;
use crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1;
use crate::mir::resolved_semantics::SourceStmtSiteV1;
use std::rc::Rc;

type Fixture = (
    Rc<OrdinaryNewClaimLedgerV1>,
    SourceStmtSiteV1,
    Rc<EmittedLexicalCallProjectionV1>,
    crate::mir::MirFunction,
    Vec<Binding>,
);

fn fixture() -> Fixture {
    fixture_at(BasicBlockId(0))
}

fn fixture_at(copy_block: BasicBlockId) -> Fixture {
    let text = "box Transport { birth() {} probe(p): i64 { return 0 } forward(q): i64 { local alias = q local recv = new Transport() local out = recv.probe(alias) return 0 } } static box Main { main() { local recv = new Transport() local out = recv.forward(true) return 0 } }";
    let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(text).unwrap();
    let (prepared, row, source, ledger, exit, copies) =
        crate::mir::builder::lexical_call_projection_forwarded_fixture(package, copy_block);
    let call = prepared
        .materialize_with_ledger(row.call_site().owner(), &row, &source, &ledger)
        .unwrap();
    assert_eq!(call.args, vec![ValueId(73)]);
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
    let (mut physical, _, cleanup) = function(&packet, &[]);
    for (block, copy) in &copies {
        physical
            .blocks
            .get_mut(block)
            .unwrap()
            .add_instruction(copy.clone());
    }
    let group = RootLocalCallBindingGroupV1::new(
        packet.call_site().clone(),
        vec![packet.invoke.clone(), packet.projection.clone()],
        Some(Rc::clone(&packet)),
    )
    .unwrap();
    ledger.root_exits.borrow_mut().insert(
        (packet.call_site().owner(), exit.clone()),
        RootHomeExitProgress::Emitted {
            origins: vec![],
            bindings: cleanup,
            entry: RootHomeExitEntry::Plain {
                local_bindings: vec![group],
            },
        },
    );
    (ledger, exit, packet, physical, copies)
}

#[test]
fn source_forwarded_child_copy_is_captured_once_and_validated_at_finish() {
    let (ledger, _, packet, physical, copies) = fixture();
    let owner = packet.call_site().owner();
    assert_ne!(ledger.root_owner().unwrap(), owner);
    let dependencies = packet.copy_dependencies(&ledger).unwrap();
    assert_eq!(
        dependencies,
        vec![(packet.call_site().clone(), copies[0].clone())]
    );
    let bindings = ledger.lifecycle_bindings(owner).unwrap();
    assert_eq!(
        bindings
            .iter()
            .filter(|binding| *binding == &copies[0])
            .count(),
        1
    );
    let boundary = PhysicalBoundary::capture(&physical, &bindings).unwrap();
    let mut projection = boundary.project(&physical).unwrap();
    boundary
        .validate_complete(&physical, &mut projection, &bindings)
        .unwrap();
    ledger
        .validate_forwarded_copies(owner, &physical, &projection)
        .unwrap();
}

#[test]
fn source_forwarded_child_finish_refuses_missing_changed_and_duplicate_copy() {
    for mutation in 0..3 {
        let (ledger, _, packet, mut physical, copies) = fixture();
        let owner = packet.call_site().owner();
        let bindings = ledger.lifecycle_bindings(owner).unwrap();
        let boundary = PhysicalBoundary::capture(&physical, &bindings).unwrap();
        let projection = boundary.project(&physical).unwrap();
        let block = physical.blocks.get_mut(&copies[0].0).unwrap();
        let index = block
            .instructions
            .iter()
            .position(|i| i == &copies[0].1)
            .unwrap();
        match mutation {
            0 => {
                block.instructions.remove(index);
            }
            1 => {
                block.instructions[index] = MirInstruction::Copy {
                    dst: ValueId(73),
                    src: ValueId(999),
                };
            }
            _ => block.instructions.push(copies[0].1.clone()),
        }
        assert!(ledger
            .validate_forwarded_copies(owner, &physical, &projection)
            .unwrap_err()
            .contains("forwarded-copy/finished-unique"));
    }
}

#[test]
fn source_forwarded_copy_survives_block_contraction_and_original_packet_rebind() {
    let (ledger, exit, packet, mut finished, copies) = fixture_at(BasicBlockId(10));
    let owner = packet.call_site().owner();
    let bindings = ledger.lifecycle_bindings(owner).unwrap();
    let boundary = PhysicalBoundary::capture(&finished, &bindings).unwrap();
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
        panic!("normal result");
    };
    *invoke_block = BasicBlockId(0);
    let mut projection = boundary.project(&finished).unwrap();
    boundary
        .validate_complete(&finished, &mut projection, &bindings)
        .unwrap();
    ledger
        .validate_forwarded_copies(owner, &finished, &projection)
        .unwrap();
    assert_eq!(
        projection
            .binding(copies[0].0, &copies[0].1)
            .unwrap()
            .unwrap()
            .0,
        BasicBlockId(0)
    );
    {
        let mut exits = ledger.root_exits.borrow_mut();
        let RootHomeExitProgress::Emitted {
            entry: RootHomeExitEntry::Plain { local_bindings },
            ..
        } = exits.get_mut(&(owner, exit.clone())).unwrap()
        else {
            panic!("Plain child exit");
        };
        for group in local_bindings {
            *group = group.with_bindings(projection.bindings(group.bindings()).unwrap());
        }
    }
    assert_eq!(
        packet.copy_dependencies(&ledger).unwrap()[0].1 .0,
        BasicBlockId(10)
    );
    ledger
        .validate_forwarded_copies(owner, &finished, &projection)
        .unwrap();
}
