use super::super::loop_;
use crate::mir::builder::control_flow::edgecfg::api::edge_stub::EdgeStub;
use crate::mir::builder::control_flow::edgecfg::api::exit_kind::ExitKind;
use crate::mir::builder::control_flow::edgecfg::api::frag::Frag;
use crate::mir::control_form::LoopId;
use crate::mir::BasicBlockId;
use std::collections::BTreeMap;

#[test]
fn test_loop_preserves_exits() {
    // Setup: body with Normal and Return exits
    let loop_id = LoopId(0);
    let header = BasicBlockId(10);
    let after = BasicBlockId(11); // Phase 265 P1: after 追加
    let body_entry = BasicBlockId(20);

    let mut body_exits = BTreeMap::new();
    body_exits.insert(
        ExitKind::Normal,
        vec![EdgeStub::without_args(body_entry, ExitKind::Normal)],
    );
    body_exits.insert(
        ExitKind::Return,
        vec![EdgeStub::without_args(body_entry, ExitKind::Return)],
    );

    let body_frag = Frag {
        entry: body_entry,
        block_params: BTreeMap::new(),
        exits: body_exits,
        wires: vec![],
        branches: vec![],
    };

    // Execute: compose::loop_()
    let loop_frag = loop_(loop_id, header, after, body_frag);

    // Verify: entry is header, exits are preserved
    assert_eq!(loop_frag.entry, header);
    assert_eq!(loop_frag.exits.len(), 2);
    assert!(loop_frag.exits.contains_key(&ExitKind::Normal));
    assert!(loop_frag.exits.contains_key(&ExitKind::Return));
}

#[test]
fn test_loop_with_break_continue() {
    // Setup: body with Break and Continue
    let loop_id = LoopId(1);
    let header = BasicBlockId(30);
    let after = BasicBlockId(31); // Phase 265 P1: after 追加
    let body_entry = BasicBlockId(40);

    let mut body_exits = BTreeMap::new();
    body_exits.insert(
        ExitKind::Break(loop_id),
        vec![EdgeStub::without_args(body_entry, ExitKind::Break(loop_id))],
    );
    body_exits.insert(
        ExitKind::Continue(loop_id),
        vec![EdgeStub::without_args(
            body_entry,
            ExitKind::Continue(loop_id),
        )],
    );

    let body_frag = Frag {
        entry: body_entry,
        block_params: BTreeMap::new(),
        exits: body_exits,
        wires: vec![],
        branches: vec![],
    };

    // Execute: compose::loop_()
    let loop_frag = loop_(loop_id, header, after, body_frag);

    // Verify: Break/Continue are in wires (Phase 265 P2)
    assert_eq!(loop_frag.entry, header);

    // Phase 265 P2: wires に Break/Continue があることを確認
    assert_eq!(loop_frag.wires.len(), 2);

    // Break → after の wire
    let break_wire = loop_frag
        .wires
        .iter()
        .find(|w| w.kind == ExitKind::Break(loop_id))
        .unwrap();
    assert_eq!(break_wire.target, Some(after));
    assert_eq!(break_wire.from, body_entry);

    // Continue → header の wire
    let continue_wire = loop_frag
        .wires
        .iter()
        .find(|w| w.kind == ExitKind::Continue(loop_id))
        .unwrap();
    assert_eq!(continue_wire.target, Some(header));
    assert_eq!(continue_wire.from, body_entry);

    // exits には Break/Continue がない
    assert!(!loop_frag.exits.contains_key(&ExitKind::Break(loop_id)));
    assert!(!loop_frag.exits.contains_key(&ExitKind::Continue(loop_id)));
}

// Phase 265 P1: 配線の証明テスト

#[test]
fn test_loop_wiring_break_to_after() {
    let loop_id = LoopId(2);
    let header = BasicBlockId(50);
    let after = BasicBlockId(51);
    let body = BasicBlockId(52);

    // Setup: body with Break exit
    let mut body_exits = BTreeMap::new();
    body_exits.insert(
        ExitKind::Break(loop_id),
        vec![EdgeStub::without_args(body, ExitKind::Break(loop_id))],
    );
    let body_frag = Frag {
        entry: body,
        block_params: BTreeMap::new(),
        exits: body_exits,
        wires: vec![],
        branches: vec![],
    };

    // Execute: compose::loop_()
    let loop_frag = loop_(loop_id, header, after, body_frag);

    // Verify: Break wire has target = after (Phase 265 P2)
    assert_eq!(loop_frag.wires.len(), 1);
    let break_wire = &loop_frag.wires[0];
    assert_eq!(break_wire.kind, ExitKind::Break(loop_id));
    assert_eq!(break_wire.from, body);
    assert_eq!(break_wire.target, Some(after));

    // exits には Break がない
    assert!(!loop_frag.exits.contains_key(&ExitKind::Break(loop_id)));
}

#[test]
fn test_loop_wiring_continue_to_header() {
    let loop_id = LoopId(3);
    let header = BasicBlockId(60);
    let after = BasicBlockId(61);
    let body = BasicBlockId(62);

    // Setup: body with Continue exit
    let mut body_exits = BTreeMap::new();
    body_exits.insert(
        ExitKind::Continue(loop_id),
        vec![EdgeStub::without_args(body, ExitKind::Continue(loop_id))],
    );
    let body_frag = Frag {
        entry: body,
        block_params: BTreeMap::new(),
        exits: body_exits,
        wires: vec![],
        branches: vec![],
    };

    // Execute: compose::loop_()
    let loop_frag = loop_(loop_id, header, after, body_frag);

    // Verify: Continue wire has target = header (Phase 265 P2)
    assert_eq!(loop_frag.wires.len(), 1);
    let continue_wire = &loop_frag.wires[0];
    assert_eq!(continue_wire.kind, ExitKind::Continue(loop_id));
    assert_eq!(continue_wire.from, body);
    assert_eq!(continue_wire.target, Some(header));

    // exits には Continue がない
    assert!(!loop_frag.exits.contains_key(&ExitKind::Continue(loop_id)));
}

#[test]
fn test_loop_wiring_preserves_return() {
    let loop_id = LoopId(4);
    let header = BasicBlockId(70);
    let after = BasicBlockId(71);
    let body = BasicBlockId(72);

    // Setup: body with Return exit (should NOT be wired)
    let mut body_exits = BTreeMap::new();
    body_exits.insert(
        ExitKind::Return,
        vec![EdgeStub::without_args(body, ExitKind::Return)],
    );
    let body_frag = Frag {
        entry: body,
        block_params: BTreeMap::new(),
        exits: body_exits,
        wires: vec![],
        branches: vec![],
    };

    // Execute: compose::loop_()
    let loop_frag = loop_(loop_id, header, after, body_frag);

    // Verify: Return stub has target = None (unwired, propagates upward)
    let return_stubs = loop_frag.exits.get(&ExitKind::Return).unwrap();
    assert_eq!(return_stubs[0].target, None);
}
