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
    fixture_with_continuation(copy_block, false)
}

fn fixture_with_continuation(copy_block: BasicBlockId, discard: bool) -> Fixture {
    let text = "box Transport { birth() {} probe(p): i64 { return 0 } forward(q): i64 { local alias = q local recv = new Transport() local out = recv.probe(alias) return 0 } } static box Main { main() { local recv = new Transport() local out = recv.forward(true) return 0 } }";
    let text = if discard {
        text.replace("local out = recv.probe(alias)", "recv.probe(alias)")
    } else {
        text.into()
    };
    let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(&text).unwrap();
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

#[test]
fn finalized_call_visitor_lends_original_borrowed_discard_with_synthetic_physical_finish() {
    use super::super::super::super::{ChildPhysicalValidation, FinalizedRootSourceHandoffV1};
    use crate::mir::normal_callable_semantic_package::FinalizedLexicalCallContextV1 as Context;
    // Original unannotated borrowed source and one-shot disposition, synthetic
    // CFG finishing only. This does not claim tagged ABI or EXE execution.
    let (ledger, _, packet, physical, _) = fixture_with_continuation(BasicBlockId(0), true);
    let owner = packet.call_site().owner();
    assert!(ledger
        .lexical_i64_call_source(packet.call_site())
        .unwrap()
        .local_binding()
        .is_none());
    let bindings = ledger.lifecycle_bindings(owner).unwrap();
    let boundary = PhysicalBoundary::capture(&physical, &bindings).unwrap();
    let mut projection = boundary.project(&physical).unwrap();
    boundary
        .validate_complete(&physical, &mut projection, &bindings)
        .unwrap();
    ledger
        .validate_forwarded_copies(owner, &physical, &projection)
        .unwrap();
    // The final lender requires the root finalization prerequisite even when
    // this isolated synthetic component visits only an ordinary child.
    let mut root = crate::mir::MirFunction::new(
        crate::mir::FunctionSignature {
            name: "synthetic-root-prerequisite".into(),
            params: vec![],
            return_type: crate::mir::MirType::Integer,
            effects: crate::mir::EffectMask::PURE,
        },
        BasicBlockId(0),
    );
    root.blocks.get_mut(&BasicBlockId(0)).unwrap().terminator =
        Some(MirInstruction::Return { value: None });
    let root_boundary = PhysicalBoundary::capture(&root, &[]).unwrap();
    let mut root_projection = root_boundary.project(&root).unwrap();
    root_boundary
        .validate_complete(&root, &mut root_projection, &[])
        .unwrap();
    *ledger.root_validation.borrow_mut() =
        super::super::super::super::RootNewValidation::ArtifactFinalized {
            owner: ledger.root_owner().unwrap(),
            symbol: root.signature.name.clone(),
            projection: Rc::new(root_projection),
        };
    ledger.child_physical_validation.borrow_mut().insert(
        owner,
        ChildPhysicalValidation::FinishingChecked {
            symbol: physical.signature.name.clone(),
            projection,
        },
    );
    let group = RootLocalCallBindingGroupV1::new(
        packet.call_site().clone(),
        vec![packet.invoke.clone(), packet.projection.clone()],
        Some(Rc::clone(&packet)),
    )
    .unwrap();
    let source = FinalizedRootSourceHandoffV1 {
        ledger: Rc::clone(&ledger),
        app_main_identity: ledger.app_main_identity.as_ref().unwrap().clone(),
        terminals: ledger.terminal_relation.clone(),
        call_entries: Default::default(),
        local_calls: [(owner, vec![group])].into(),
    };
    let mut module = crate::mir::MirModule::new("borrowed-discard-loan".into());
    module
        .functions
        .insert(physical.signature.name.clone(), physical);
    let mut visited = 0;
    source.visit_finalized_lexical_call_nodes_v1(&module,
        |observed_owner, context, original, arguments, _, coordinate| {
            let Context::Discard { group_site } = context else { panic!("borrowed Discard"); };
            assert_eq!(group_site, packet.call_site());
            assert_eq!(observed_owner, owner);
            assert!(std::ptr::eq(original, packet.as_ref()));
            assert!(matches!(arguments, [crate::mir::resolved_semantics::home_new_prefix::LocalCallArgumentV1::BorrowedActual { ordinal: 0, .. }]));
            assert_eq!(coordinate.0, BasicBlockId(10));
            visited += 1;
            Ok(())
        }).unwrap();
    assert_eq!(visited, 1);
    let actual = module.functions.values().next().unwrap();
    let mut aliases = 0;
    source
        .with_borrowed_ordinary_alias_copies_v1(owner, actual, |_, _, _, _, value, copies| {
            assert_eq!(value, ValueId(73));
            assert_eq!(copies.len(), 1);
            aliases += 1;
            Ok(())
        })
        .unwrap();
    assert_eq!(aliases, 1);
    let mut foreign = actual.clone();
    foreign.signature.name.push_str("-foreign");
    assert!(source
        .with_borrowed_ordinary_alias_copies_v1(owner, &foreign, |_, _, _, _, _, _| panic!(
            "foreign function must not obtain alias proofs"
        ))
        .unwrap_err()
        .contains("finished-function"));
}
