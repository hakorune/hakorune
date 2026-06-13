use super::super::seq;
use crate::mir::builder::control_flow::edgecfg::api::edge_stub::EdgeStub;
use crate::mir::builder::control_flow::edgecfg::api::exit_kind::ExitKind;
use crate::mir::builder::control_flow::edgecfg::api::frag::Frag;
use crate::mir::BasicBlockId;
use std::collections::BTreeMap;

// Phase 265 P2: seq() のテスト

#[test]
fn test_seq_wiring_normal_to_wires() {
    // Setup: a with Normal exit, b with Return exit
    let a_entry = BasicBlockId(10);
    let a_exit = BasicBlockId(11);
    let b_entry = BasicBlockId(20);
    let b_exit = BasicBlockId(21);

    let mut a_exits = BTreeMap::new();
    a_exits.insert(
        ExitKind::Normal,
        vec![EdgeStub::without_args(a_exit, ExitKind::Normal)],
    );
    let a_frag = Frag {
        entry: a_entry,
        block_params: BTreeMap::new(),
        exits: a_exits,
        wires: vec![],
        branches: vec![],
    };

    let mut b_exits = BTreeMap::new();
    b_exits.insert(
        ExitKind::Return,
        vec![EdgeStub::without_args(b_exit, ExitKind::Return)],
    );
    let b_frag = Frag {
        entry: b_entry,
        block_params: BTreeMap::new(),
        exits: b_exits,
        wires: vec![],
        branches: vec![],
    };

    // Execute: compose::seq()
    let seq_frag = seq(a_frag, b_frag);

    // Verify: entry = a.entry
    assert_eq!(seq_frag.entry, a_entry);

    // a.Normal → b.entry is in wires
    assert_eq!(seq_frag.wires.len(), 1);
    assert_eq!(seq_frag.wires[0].from, a_exit);
    assert_eq!(seq_frag.wires[0].target, Some(b_entry));
    assert_eq!(seq_frag.wires[0].kind, ExitKind::Normal);

    // exits has no Normal (internal wiring)
    assert!(!seq_frag.exits.contains_key(&ExitKind::Normal));

    // b.Return is in exits (unwired)
    let return_stubs = seq_frag.exits.get(&ExitKind::Return).unwrap();
    assert_eq!(return_stubs[0].from, b_exit);
    assert_eq!(return_stubs[0].target, None);
}

#[test]
fn test_seq_preserves_non_normal_exits() {
    // Setup: a with Return + Normal, b with Unwind
    let a_entry = BasicBlockId(30);
    let b_entry = BasicBlockId(40);

    let mut a_exits = BTreeMap::new();
    a_exits.insert(
        ExitKind::Normal,
        vec![EdgeStub::without_args(BasicBlockId(31), ExitKind::Normal)],
    );
    a_exits.insert(
        ExitKind::Return,
        vec![EdgeStub::without_args(BasicBlockId(32), ExitKind::Return)],
    );
    let a_frag = Frag {
        entry: a_entry,
        block_params: BTreeMap::new(),
        exits: a_exits,
        wires: vec![],
        branches: vec![],
    };

    let mut b_exits = BTreeMap::new();
    b_exits.insert(
        ExitKind::Unwind,
        vec![EdgeStub::without_args(BasicBlockId(41), ExitKind::Unwind)],
    );
    let b_frag = Frag {
        entry: b_entry,
        block_params: BTreeMap::new(),
        exits: b_exits,
        wires: vec![],
        branches: vec![],
    };

    // Execute
    let seq_frag = seq(a_frag, b_frag);

    // Verify: a.Return + b.Unwind are in exits (unwired)
    assert!(seq_frag.exits.contains_key(&ExitKind::Return));
    assert!(seq_frag.exits.contains_key(&ExitKind::Unwind));
    assert_eq!(
        seq_frag.exits.get(&ExitKind::Return).unwrap()[0].target,
        None
    );
    assert_eq!(
        seq_frag.exits.get(&ExitKind::Unwind).unwrap()[0].target,
        None
    );

    // a.Normal is in wires
    assert_eq!(seq_frag.wires.len(), 1);
    assert_eq!(seq_frag.wires[0].kind, ExitKind::Normal);
    assert_eq!(seq_frag.wires[0].target, Some(b_entry));
}
