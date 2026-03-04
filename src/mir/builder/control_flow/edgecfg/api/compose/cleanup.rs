#[cfg(test)]
use std::collections::BTreeMap;

#[cfg(test)]
use crate::mir::basic_block::BasicBlockId;
#[cfg(test)]
use crate::mir::builder::control_flow::edgecfg::api::exit_kind::ExitKind;
#[cfg(test)]
use crate::mir::builder::control_flow::edgecfg::api::frag::Frag;

/// Phase 281 P3: cleanup() Normal + Return exit wiring implementation
///
/// Wires cleanup Normal/Return exits to specified targets or propagates them upward.
///
/// # Contract (P3 Implementation)
///
/// **Input**:
/// - `main`: Main control flow (loop structure Frag)
/// - `cleanup_frag`: Exit handler (Normal/Return exits only, no wires/branches)
/// - `normal_target`: Where to wire Normal exits
///   - `Some(bb)`: Wire Normal → bb (internal closure, target = Some)
///   - `None`: Propagate Normal → wires (upward propagation, target = None)
/// - `ret_target`: Where to wire Return exits
///   - `Some(bb)`: Wire Return → bb (internal closure, target = Some)
///   - `None`: Propagate Return → wires (upward propagation, target = None)
///
/// **Output**:
/// - Frag with main's structure + cleanup's exits wired/propagated
///
/// **Invariants**:
/// - 1 block = 1 terminator (no duplicate BranchStubs)
/// - cleanup_frag must have empty wires/branches (Fail-Fast if not)
/// - cleanup_frag.exits must contain only Normal/Return (Fail-Fast for other kinds)
/// - normal_target=Some: Normal exits → wires (internal)
/// - normal_target=None: Normal exits → wires (target=None, propagate upward)
/// - ret_target=Some: Return exits → wires (internal)
/// - ret_target=None: Return exits → wires (target=None, propagate upward)
///
/// # Implementation Status
///
/// P3: Normal + Return wiring logic implemented
/// Future: Break/Continue/Unwind support (P4+)
///
/// # Migration Notes (Phase 264 → Phase 281)
///
/// Old signature (Phase 264): `cleanup(body: Frag, cleanup_block: BasicBlockId) -> Frag`
/// Phase 281 P1: `cleanup(main: Frag, cleanup: Frag) -> Result<Frag, String>`
/// Phase 281 P2: `cleanup(main: Frag, cleanup_frag: Frag, ret_target: Option<BasicBlockId>) -> Result<Frag, String>`
/// Phase 281 P3: `cleanup(main: Frag, cleanup_frag: Frag, normal_target: Option<BasicBlockId>, ret_target: Option<BasicBlockId>) -> Result<Frag, String>`
///
/// Rationale: Pattern6/7 require flexible exit wiring for Normal/Return exits.
/// cleanup_frag must be "exit-only" to prevent terminator confusion.
#[cfg(test)]
pub(crate) fn cleanup(
    main: Frag,
    cleanup_frag: Frag,
    normal_target: Option<BasicBlockId>,
    ret_target: Option<BasicBlockId>,
) -> Result<Frag, String> {
    let strict = crate::config::env::joinir_dev::strict_enabled()
        || crate::config::env::joinir_dev_enabled();

    // Phase 281 P3: Normal + Return exit wiring implementation
    // - Supported: Normal, Return exits
    // - Unsupported: Break, Continue, Unwind (Fail-Fast)

    let mut exits = BTreeMap::new();
    let mut wires = Vec::new();
    let mut branches = Vec::new();
    let mut block_params = main.block_params;
    let Frag {
        block_params: cleanup_block_params,
        exits: cleanup_exits,
        wires: cleanup_wires,
        branches: cleanup_branches,
        ..
    } = cleanup_frag;

    if let Err(message) = super::merge_block_params(
        &mut block_params,
        cleanup_block_params,
        "compose::cleanup",
    ) {
        return Err(message);
    }

    // Validate cleanup_frag structure (only exits allowed, no wires/branches)
    if !cleanup_wires.is_empty() || !cleanup_branches.is_empty() {
        return Err(format!(
            "compose::cleanup() Phase 281 P3: cleanup_frag must have empty wires/branches (only exits allowed), found {} wires, {} branches",
            cleanup_wires.len(),
            cleanup_branches.len()
        ));
    }

    // Validate cleanup_frag exits (only Normal + Return allowed in P3)
    for (kind, _) in &cleanup_exits {
        match kind {
            ExitKind::Normal | ExitKind::Return => {} // OK
            _ => {
                if strict {
                    return Err(format!(
                        "[edgecfg/cleanup] unsupported exit kind {:?} in cleanup_frag",
                        kind
                    ));
                }
                return Err(format!(
                    "compose::cleanup() Phase 281 P3: unsupported exit kind {:?} in cleanup_frag (only Normal/Return allowed)",
                    kind
                ));
            }
        }
    }

    // Process cleanup Normal exits
    if let Some(normal_stubs) = cleanup_exits.get(&ExitKind::Normal) {
        for mut stub in normal_stubs.clone() {
            match normal_target {
                Some(target_bb) => {
                    // Wire: Normal → target_bb (internal closure)
                    stub.target = Some(target_bb);
                    wires.push(stub);
                }
                None => {
                    // Propagate: Normal → wires (target=None, upward propagation)
                    stub.target = None;
                    wires.push(stub);
                }
            }
        }
    }

    // Process cleanup Return exits
    if let Some(return_stubs) = cleanup_exits.get(&ExitKind::Return) {
        for mut stub in return_stubs.clone() {
            match ret_target {
                Some(target_bb) => {
                    // Wire: Return → target_bb (internal closure)
                    stub.target = Some(target_bb);
                    wires.push(stub);
                }
                None => {
                    // Propagate: Return → wires (target=None, will be emitted as Return terminator)
                    // Note: Return exits can have target=None in wires (Phase 267 special case)
                    stub.target = None;
                    wires.push(stub);
                }
            }
        }
    }

    // Preserve main's exits/wires/branches
    for (kind, stubs) in main.exits {
        exits.entry(kind).or_insert_with(Vec::new).extend(stubs);
    }
    wires.extend(main.wires);
    branches.extend(main.branches);

    Ok(Frag {
        entry: main.entry,  // Entry = main entry (header_bb)
        block_params,
        exits,
        wires,
        branches,
    })
}

#[cfg(test)]
mod tests {
    use super::cleanup;
    use crate::mir::basic_block::{BasicBlockId, EdgeArgs};
    use crate::mir::builder::control_flow::edgecfg::api::edge_stub::EdgeStub;
    use crate::mir::builder::control_flow::edgecfg::api::exit_kind::ExitKind;
    use crate::mir::builder::control_flow::edgecfg::api::frag::Frag;
    use crate::mir::join_ir::lowering::inline_boundary::JumpArgsLayout;
    use crate::mir::ValueId;
    use std::collections::BTreeMap;

    #[test]
    fn cleanup_preserves_edgeargs_for_return_exit() {
        let main = Frag::new(BasicBlockId::new(1));
        let cleanup_entry = BasicBlockId::new(2);
        let args = EdgeArgs {
            layout: JumpArgsLayout::ExprResultPlusCarriers,
            values: vec![ValueId(30)],
        };
        let mut exits = BTreeMap::new();
        exits.insert(
            ExitKind::Return,
            vec![EdgeStub::new(
                cleanup_entry,
                ExitKind::Return,
                None,
                args.clone(),
            )],
        );
        let cleanup_frag = Frag {
            entry: cleanup_entry,
            block_params: BTreeMap::new(),
            exits,
            wires: vec![],
            branches: vec![],
        };

        let composed =
            cleanup(main, cleanup_frag, None, None).expect("cleanup ok");
        assert_eq!(composed.wires.len(), 1);
        assert_eq!(composed.wires[0].args, args);
        assert!(composed.wires[0].target.is_none());
    }
}
