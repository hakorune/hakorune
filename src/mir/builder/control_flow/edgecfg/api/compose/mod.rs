/*!
 * # Frag Composition API - Single Source of Truth (Phase 280)
 *
 * This module is the **Single Source of Truth** for Frag composition.
 *
 * ## Purpose (Phase 280)
 *
 * Pattern numbers (1-9+) are **symptom labels** for regression tests, NOT architectural concepts.
 * The architectural SSOT is **Frag composition rules** (`seq`/`if`/`loop`/`cleanup`).
 *
 * **Upstream (Extractor/Normalizer)**: Finish "shape recognition" and extract route-specific knowledge
 * **Downstream (Composition)**: Use Frag composition rules to build CFG converging to SSOT
 * **Terminator Generation**: `FragEmitSession`（手順SSOT）+ `emit_frag()`（低レベルSSOT）(Phase 29bq+)
 *
 * ## Entry Points (Composition Operations)
 *
 * - `seq(a, b)`: Sequential composition (Normal wiring)
 * - `if_(header, cond, t, e, join_frag)`: Conditional composition (Branch wiring)
 * - `loop_(loop_id, header, after, body)`: Loop composition (Break/Continue wiring)
 * - `cleanup(body, cleanup)`: Cleanup composition (TODO: Phase 280+)
 *
 * ## Composition Contract (Invariants)
 *
 * - **Input**: `Frag` (entry + exits + wires + branches)
 * - **Output**: `Frag` (new entry + merged exits + merged wires + merged branches)
 * - **Guarantee**: Composition preserves invariants (`verify_frag_invariants_strict`)
 * - **No Allocation**: Caller (Normalizer) allocates `BasicBlockId`/`ValueId`
 * - **Pure CFG Transform**: Composition rearranges `exits`/`wires`/`branches` only
 *
 * ## Ownership Model (3-tier)
 *
 * 1. **Normalizer** (Tier 1): Allocates blocks/values, route-specific knowledge
 * 2. **Composition** (Tier 2): Rearranges exits/wires/branches, route-agnostic
 * 3. **Lowerer** (Tier 3): Emits MIR terminators via `FragEmitSession::emit_and_seal()`
 *
 * ## Usage Example
 *
 * ```rust
 * // Tier 1: Normalizer allocates blocks
 * let header_bb = builder.next_block_id();
 * let body_bb = builder.next_block_id();
 * let after_bb = builder.next_block_id();
 *
 * // Build Frags for body
 * let body_frag = Frag { /* body CFG */ };
 *
 * // Tier 2: Composition wires exits (no allocation)
 * let loop_frag = compose::loop_(loop_id, header_bb, after_bb, body_frag);
 *
 * // Tier 3: Lowerer emits terminators
 * session.emit_and_seal(func, &loop_frag)?;
 * ```
 *
 * ## References
 *
 * - **SSOT Documentation**: `docs/development/current/main/design/edgecfg-fragments.md` (Active SSOT)
 * - **Pattern Absorption**: `docs/development/current/main/joinir-architecture-overview.md` (Section 0.2)
 * - **Phase 280 Roadmap**: `docs/development/current/main/phases/phase-280/README.md`
 *
 * ## History
 *
 * - Phase 264: Entry API creation (signatures only)
 * - Phase 265-268: Implementation (seq/if/loop wiring, emit_frag SSOT)
 * - Phase 280: SSOT positioning (composition as legacy numbered-label absorption destination)
 */

use std::collections::BTreeMap;

use crate::config::env;
use crate::mir::builder::control_flow::edgecfg::api::block_params::BlockParams;
use crate::mir::BasicBlockId;

mod cleanup;
mod if_;
mod loop_;
mod seq;

#[cfg(test)]
pub(crate) use cleanup::cleanup;
pub(crate) use if_::if_;
#[cfg(test)]
pub(crate) use loop_::loop_;
#[cfg(test)]
pub(crate) use seq::seq;

pub(super) fn merge_block_params(
    target: &mut BTreeMap<BasicBlockId, BlockParams>,
    incoming: BTreeMap<BasicBlockId, BlockParams>,
    context: &str,
) -> Result<(), String> {
    let strict = env::joinir_strict_enabled() || env::joinir_dev_enabled();
    for (block, params) in incoming {
        if target.contains_key(&block) {
            if strict {
                return Err(format!(
                    "[{}] duplicate block_params for {:?}",
                    context, block
                ));
            }
            continue;
        }
        target.insert(block, params);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{cleanup, if_, loop_, seq};
    use crate::mir::builder::control_flow::edgecfg::api::edge_stub::EdgeStub;
    use crate::mir::builder::control_flow::edgecfg::api::exit_kind::ExitKind;
    use crate::mir::builder::control_flow::edgecfg::api::frag::Frag;
    use crate::mir::control_form::LoopId;
    use crate::mir::join_ir::lowering::inline_boundary::JumpArgsLayout;
    use crate::mir::value_id::ValueId;
    use crate::mir::{BasicBlockId, EdgeArgs};
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
}
