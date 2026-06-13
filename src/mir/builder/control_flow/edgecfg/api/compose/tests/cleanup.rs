use super::super::cleanup::cleanup;
use crate::mir::builder::control_flow::edgecfg::api::edge_stub::EdgeStub;
use crate::mir::builder::control_flow::edgecfg::api::exit_kind::ExitKind;
use crate::mir::builder::control_flow::edgecfg::api::frag::Frag;
use crate::mir::join_ir::lowering::inline_boundary::JumpArgsLayout;
use crate::mir::{BasicBlockId, EdgeArgs};
use std::collections::BTreeMap;

// Phase 281 P2: cleanup() test - Return propagation
#[test]
fn test_cleanup_return_propagation() {
    let main_entry = BasicBlockId(100);
    let cleanup_bb = BasicBlockId(200);

    // Main Frag: empty (no exits)
    let main_frag = Frag {
        entry: main_entry,
        block_params: BTreeMap::new(),
        exits: BTreeMap::new(),
        wires: vec![],
        branches: vec![],
    };

    // Cleanup Frag: Return exit
    let cleanup_frag = Frag {
        entry: cleanup_bb,
        block_params: BTreeMap::new(),
        exits: BTreeMap::from([(
            ExitKind::Return,
            vec![EdgeStub::new(
                cleanup_bb,
                ExitKind::Return,
                None, // Unresolved
                EdgeArgs {
                    layout: JumpArgsLayout::CarriersOnly,
                    values: vec![],
                },
            )],
        )]),
        wires: vec![],
        branches: vec![],
    };

    // Execute: normal_target=None, ret_target=None → propagate Return
    let result = cleanup(main_frag, cleanup_frag, None, None);

    // Verify: Return in wires (target=None, to be emitted as terminator)
    assert!(result.is_ok());
    let composed = result.unwrap();
    assert_eq!(composed.entry, main_entry);
    assert_eq!(composed.wires.len(), 1); // Return in wires

    let return_wire = &composed.wires[0];
    assert_eq!(return_wire.from, cleanup_bb);
    assert_eq!(return_wire.kind, ExitKind::Return);
    assert_eq!(return_wire.target, None); // Unresolved (upward propagation)
}

// Phase 281 P2: cleanup() test - Return wiring
#[test]
fn test_cleanup_return_wiring() {
    let main_entry = BasicBlockId(100);
    let cleanup_bb = BasicBlockId(200);
    let target_bb = BasicBlockId(300); // Wire destination

    // Main Frag: empty
    let main_frag = Frag {
        entry: main_entry,
        block_params: BTreeMap::new(),
        exits: BTreeMap::new(),
        wires: vec![],
        branches: vec![],
    };

    // Cleanup Frag: Return exit
    let cleanup_frag = Frag {
        entry: cleanup_bb,
        block_params: BTreeMap::new(),
        exits: BTreeMap::from([(
            ExitKind::Return,
            vec![EdgeStub::new(
                cleanup_bb,
                ExitKind::Return,
                None, // Unresolved
                EdgeArgs {
                    layout: JumpArgsLayout::CarriersOnly,
                    values: vec![],
                },
            )],
        )]),
        wires: vec![],
        branches: vec![],
    };

    // Execute: normal_target=None, ret_target=Some(target_bb) → wire Return
    let result = cleanup(main_frag, cleanup_frag, None, Some(target_bb));

    // Verify: Return in wires (not exits), wired to target_bb
    assert!(result.is_ok());
    let composed = result.unwrap();
    assert_eq!(composed.entry, main_entry);
    assert_eq!(composed.exits.len(), 0); // No exits (closed)
    assert_eq!(composed.wires.len(), 1); // Return wired

    let wired_stub = &composed.wires[0];
    assert_eq!(wired_stub.from, cleanup_bb);
    assert_eq!(wired_stub.kind, ExitKind::Return);
    assert_eq!(wired_stub.target, Some(target_bb)); // Wired!
}

// Phase 281 P3: cleanup() test - Normal propagation
#[test]
fn test_cleanup_normal_propagation() {
    let main_entry = BasicBlockId(100);
    let cleanup_bb = BasicBlockId(200);

    // Main Frag: empty
    let main_frag = Frag {
        entry: main_entry,
        block_params: BTreeMap::new(),
        exits: BTreeMap::new(),
        wires: vec![],
        branches: vec![],
    };

    // Cleanup Frag: Normal exit
    let cleanup_frag = Frag {
        entry: cleanup_bb,
        block_params: BTreeMap::new(),
        exits: BTreeMap::from([(
            ExitKind::Normal,
            vec![EdgeStub::new(
                cleanup_bb,
                ExitKind::Normal,
                None, // Unresolved
                EdgeArgs {
                    layout: JumpArgsLayout::CarriersOnly,
                    values: vec![],
                },
            )],
        )]),
        wires: vec![],
        branches: vec![],
    };

    // Execute: normal_target=None, ret_target=None → propagate Normal
    let result = cleanup(main_frag, cleanup_frag, None, None);

    // Verify: Normal in wires (target=None, upward propagation)
    assert!(result.is_ok());
    let composed = result.unwrap();
    assert_eq!(composed.entry, main_entry);
    assert_eq!(composed.wires.len(), 1); // Normal in wires

    let normal_wire = &composed.wires[0];
    assert_eq!(normal_wire.from, cleanup_bb);
    assert_eq!(normal_wire.kind, ExitKind::Normal);
    assert_eq!(normal_wire.target, None); // Unresolved (upward propagation)
}

// Phase 281 P3: cleanup() test - Normal wiring
#[test]
fn test_cleanup_normal_wiring() {
    let main_entry = BasicBlockId(100);
    let cleanup_bb = BasicBlockId(200);
    let target_bb = BasicBlockId(300); // Wire destination

    // Main Frag: empty
    let main_frag = Frag {
        entry: main_entry,
        block_params: BTreeMap::new(),
        exits: BTreeMap::new(),
        wires: vec![],
        branches: vec![],
    };

    // Cleanup Frag: Normal exit
    let cleanup_frag = Frag {
        entry: cleanup_bb,
        block_params: BTreeMap::new(),
        exits: BTreeMap::from([(
            ExitKind::Normal,
            vec![EdgeStub::new(
                cleanup_bb,
                ExitKind::Normal,
                None, // Unresolved
                EdgeArgs {
                    layout: JumpArgsLayout::CarriersOnly,
                    values: vec![],
                },
            )],
        )]),
        wires: vec![],
        branches: vec![],
    };

    // Execute: normal_target=Some(target_bb), ret_target=None → wire Normal
    let result = cleanup(main_frag, cleanup_frag, Some(target_bb), None);

    // Verify: Normal in wires (not exits), wired to target_bb
    assert!(result.is_ok());
    let composed = result.unwrap();
    assert_eq!(composed.entry, main_entry);
    assert_eq!(composed.exits.len(), 0); // No exits (closed)
    assert_eq!(composed.wires.len(), 1); // Normal wired

    let wired_stub = &composed.wires[0];
    assert_eq!(wired_stub.from, cleanup_bb);
    assert_eq!(wired_stub.kind, ExitKind::Normal);
    assert_eq!(wired_stub.target, Some(target_bb)); // Wired!
}
