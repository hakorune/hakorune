use super::super::if_;
use crate::mir::builder::control_flow::edgecfg::api::edge_stub::EdgeStub;
use crate::mir::builder::control_flow::edgecfg::api::exit_kind::ExitKind;
use crate::mir::builder::control_flow::edgecfg::api::frag::Frag;
use crate::mir::join_ir::lowering::inline_boundary::JumpArgsLayout;
use crate::mir::value_id::ValueId;
use crate::mir::{BasicBlockId, EdgeArgs};
use std::collections::BTreeMap;

// Phase 265 P2: if_() のテスト

#[test]
fn test_if_wiring_then_else_normal_to_wires() {
    // Setup: then with Normal, else with Normal, join_frag with Return
    let header = BasicBlockId(50);
    let join_entry = BasicBlockId(51);
    let join_exit = BasicBlockId(52);
    let then_entry = BasicBlockId(60);
    let then_exit = BasicBlockId(61);
    let else_entry = BasicBlockId(70);
    let else_exit = BasicBlockId(71);

    let mut then_exits = BTreeMap::new();
    then_exits.insert(
        ExitKind::Normal,
        vec![EdgeStub::without_args(then_exit, ExitKind::Normal)],
    );
    let then_frag = Frag {
        entry: then_entry,
        block_params: BTreeMap::new(),
        exits: then_exits,
        wires: vec![],
        branches: vec![],
    };

    let mut else_exits = BTreeMap::new();
    else_exits.insert(
        ExitKind::Normal,
        vec![EdgeStub::without_args(else_exit, ExitKind::Normal)],
    );
    let else_frag = Frag {
        entry: else_entry,
        block_params: BTreeMap::new(),
        exits: else_exits,
        wires: vec![],
        branches: vec![],
    };

    let mut join_exits = BTreeMap::new();
    join_exits.insert(
        ExitKind::Return,
        vec![EdgeStub::without_args(join_exit, ExitKind::Return)],
    );
    let join_frag = Frag {
        entry: join_entry,
        block_params: BTreeMap::new(),
        exits: join_exits,
        wires: vec![],
        branches: vec![],
    };

    // Execute: compose::if_()
    let if_frag = if_(
        header,
        ValueId(0),
        then_frag,
        EdgeArgs {
            layout: JumpArgsLayout::CarriersOnly,
            values: vec![],
        },
        else_frag,
        EdgeArgs {
            layout: JumpArgsLayout::CarriersOnly,
            values: vec![],
        },
        join_frag,
    );

    // Verify: entry = header
    assert_eq!(if_frag.entry, header);

    // then/else Normal → join_entry are in wires
    assert_eq!(if_frag.wires.len(), 2);
    for wire in &if_frag.wires {
        assert_eq!(wire.kind, ExitKind::Normal);
        assert_eq!(wire.target, Some(join_entry));
    }

    // exits has no Normal (internal wiring)
    assert!(!if_frag.exits.contains_key(&ExitKind::Normal));

    // join_frag.Return is in exits
    assert!(if_frag.exits.contains_key(&ExitKind::Return));
    assert_eq!(
        if_frag.exits.get(&ExitKind::Return).unwrap()[0].from,
        join_exit
    );
}

#[test]
fn test_if_preserves_return_from_then_and_else() {
    // Setup: then with Normal + Return, else with Normal + Unwind
    let header = BasicBlockId(80);
    let join_entry = BasicBlockId(81);
    let then_entry = BasicBlockId(90);
    let else_entry = BasicBlockId(100);

    let mut then_exits = BTreeMap::new();
    then_exits.insert(
        ExitKind::Normal,
        vec![EdgeStub::without_args(BasicBlockId(91), ExitKind::Normal)],
    );
    then_exits.insert(
        ExitKind::Return,
        vec![EdgeStub::without_args(BasicBlockId(92), ExitKind::Return)],
    );
    let then_frag = Frag {
        entry: then_entry,
        block_params: BTreeMap::new(),
        exits: then_exits,
        wires: vec![],
        branches: vec![],
    };

    let mut else_exits = BTreeMap::new();
    else_exits.insert(
        ExitKind::Normal,
        vec![EdgeStub::without_args(BasicBlockId(101), ExitKind::Normal)],
    );
    else_exits.insert(
        ExitKind::Unwind,
        vec![EdgeStub::without_args(BasicBlockId(102), ExitKind::Unwind)],
    );
    let else_frag = Frag {
        entry: else_entry,
        block_params: BTreeMap::new(),
        exits: else_exits,
        wires: vec![],
        branches: vec![],
    };

    let join_frag = Frag {
        entry: join_entry,
        block_params: BTreeMap::new(),
        exits: BTreeMap::new(),
        wires: vec![],
        branches: vec![],
    };

    // Execute
    let if_frag = if_(
        header,
        ValueId(0),
        then_frag,
        EdgeArgs {
            layout: JumpArgsLayout::CarriersOnly,
            values: vec![],
        },
        else_frag,
        EdgeArgs {
            layout: JumpArgsLayout::CarriersOnly,
            values: vec![],
        },
        join_frag,
    );

    // Verify: Return and Unwind are in exits (unwired)
    assert!(if_frag.exits.contains_key(&ExitKind::Return));
    assert!(if_frag.exits.contains_key(&ExitKind::Unwind));
    assert_eq!(
        if_frag.exits.get(&ExitKind::Return).unwrap()[0].target,
        None
    );
    assert_eq!(
        if_frag.exits.get(&ExitKind::Unwind).unwrap()[0].target,
        None
    );

    // then/else Normal are in wires
    assert_eq!(if_frag.wires.len(), 2);
}
