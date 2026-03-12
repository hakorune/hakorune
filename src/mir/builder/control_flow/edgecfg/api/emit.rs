/*!
 * wires → MIR terminator 変換（Phase 266: SSOT）
 *
 * # 目的
 * - EdgeStub の wires を MIR terminator に変換する唯一の入口
 * - Phase 260 の terminator 語彙ルールを厳守
 * - 1 block = 1 terminator 制約を強制
 *
 * # Phase 266 制約
 * - Jump/Return のみ実装（Branch は Phase 267）
 * - Return は target=None を許可（意味を持たない）
 * - from ごとにグループ化して1本だけ許可
 */

use super::edge_stub::EdgeStub;
use super::exit_kind::ExitKind;
use crate::mir::basic_block::BasicBlockId;
use crate::mir::builder::control_flow::joinir::trace;
use crate::mir::instruction::MirInstruction;
use std::collections::BTreeMap;

/// wires → MIR terminator 変換（Phase 266 P1: SSOT）
///
/// # 責務
/// - EdgeStub の target=Some(...) を MIR terminator に変換
/// - BasicBlock::set_*_with_edge_args() を使って terminator + successor を同期
/// - target=None の EdgeStub が混入したら Fail-Fast（Return を除く）
///
/// # 引数
/// - `function`: MIR function（BasicBlock アクセス用）
/// - `wires`: 配線済み EdgeStub のリスト（target=Some のみを期待、Return は target=None OK）
///
/// # 戻り値
/// - `Ok(())`: 全 wire を MIR terminator に変換成功
/// - `Err(String)`: target=None の EdgeStub を検出、または不正な kind、または複数 wire
pub fn emit_wires(
    function: &mut crate::mir::MirFunction,
    wires: &[EdgeStub],
) -> Result<(), String> {
    let trace_logger = trace::trace();
    let func_name = function.signature.name.clone();

    // Step 1: from ごとにグループ化（1 block = 1 terminator 制約）
    let mut by_block: BTreeMap<BasicBlockId, Vec<&EdgeStub>> = BTreeMap::new();
    for stub in wires {
        by_block.entry(stub.from).or_default().push(stub);
    }

    // Step 2: 各 block に対して1本だけ wire を許可
    for (block_id, stubs) in by_block {
        if stubs.len() > 1 {
            return Err(format!(
                "[emit_wires] Multiple wires from same block {:?} (count={}). \
                 1 block = 1 terminator constraint violated.",
                block_id,
                stubs.len()
            ));
        }

        let stub = stubs[0];

        // Fail-Fast: target=None 検出（Return 以外）
        let target = match stub.kind {
            ExitKind::Return => None, // Return は target 不要
            _ => {
                // Normal/Break/Continue/Unwind は target 必須
                Some(stub.target.ok_or_else(|| {
                    format!(
                        "[emit_wires] Unwired EdgeStub detected: from={:?}, kind={:?}. \
                         Wires (except Return) must have target=Some(...). This is a contract violation.",
                        stub.from, stub.kind
                    )
                })?)
            }
        };

        // Block 取得
        let block = function
            .get_block_mut(stub.from)
            .ok_or_else(|| format!("[emit_wires] Block {:?} not found", stub.from))?;

        // ExitKind 別に terminator 生成
        match stub.kind {
            ExitKind::Normal | ExitKind::Break(_) | ExitKind::Continue(_) | ExitKind::Unwind => {
                // Jump terminator（Phase 260 ルール: set_jump_with_edge_args を使用）
                block.set_jump_with_edge_args(target.unwrap(), Some(stub.args.clone()));
                trace_logger.debug(
                    "lowerer/term_set",
                    &format!(
                        "func={} bb={:?} term=Jump target={:?}",
                        func_name,
                        stub.from,
                        target.unwrap()
                    ),
                );
            }
            ExitKind::Return => {
                // Return terminator + metadata（Phase 260 例外ルール: set_terminator + set_return_env）
                block.set_terminator(MirInstruction::Return {
                    value: stub.args.values.first().copied(),
                });
                block.set_return_env(stub.args.clone());
                trace_logger.debug(
                    "lowerer/term_set",
                    &format!("func={} bb={:?} term=Return", func_name, stub.from),
                );
            }
            _ => {
                return Err(format!(
                    "[emit_wires] Unsupported ExitKind: {:?}",
                    stub.kind
                ));
            }
        }
    }

    Ok(())
}

fn emit_block_params_as_phis(
    function: &mut crate::mir::MirFunction,
    frag: &super::frag::Frag,
) -> Result<(), String> {
    use crate::ast::Span;
    use crate::mir::basic_block::{BasicBlockId, EdgeArgs};
    use crate::mir::builder::emission::phi_lifecycle;
    use std::collections::BTreeSet;

    if frag.block_params.is_empty() {
        return Ok(());
    }

    let strict =
        crate::config::env::joinir_strict_enabled() || crate::config::env::joinir_dev_enabled();

    let mut incoming: BTreeMap<BasicBlockId, Vec<(BasicBlockId, EdgeArgs)>> = BTreeMap::new();
    for stub in &frag.wires {
        if let Some(target) = stub.target {
            incoming
                .entry(target)
                .or_default()
                .push((stub.from, stub.args.clone()));
        }
    }
    for branch in &frag.branches {
        incoming
            .entry(branch.then_target)
            .or_default()
            .push((branch.from, branch.then_args.clone()));
        incoming
            .entry(branch.else_target)
            .or_default()
            .push((branch.from, branch.else_args.clone()));
    }

    for (target, params) in &frag.block_params {
        let edges = incoming.get(target).cloned().unwrap_or_default();
        if edges.is_empty() {
            if strict {
                return Err(format!(
                    "[emit_frag] BlockParams target {:?} has no incoming edges",
                    target
                ));
            }
            continue;
        }

        if strict {
            let mut seen = BTreeSet::new();
            for (pred, _) in &edges {
                if !seen.insert(*pred) {
                    return Err(format!(
                        "[emit_frag] Duplicate incoming edge {:?}->{:?}",
                        pred, target
                    ));
                }
            }
        }

        for (index, dst) in params.params.iter().enumerate() {
            let mut inputs = Vec::with_capacity(edges.len());
            for (pred, args) in &edges {
                match args.values.get(index) {
                    Some(value) => inputs.push((*pred, *value)),
                    None => {
                        if strict {
                            return Err(format!(
                                "[emit_frag] Missing edge arg for block_params {:?} index {}",
                                target, index
                            ));
                        }
                    }
                }
            }
            if inputs.is_empty() {
                if strict {
                    return Err(format!(
                        "[emit_frag] BlockParams target {:?} has no inputs for index {}",
                        target, index
                    ));
                }
                continue;
            }
            // SSOT: PHI insertion via phi_lifecycle (function-level)
            phi_lifecycle::define_phi_final_fn(function, *target, *dst, inputs, Span::unknown())?;
        }
    }

    Ok(())
}

/// Frag を MIR に emit（Phase 267 P0: SSOT）
///
/// # 責務
/// - verify_frag_invariants_strict() で事前検証（Fail-Fast）
/// - wires → Jump/Return terminator（emit_wires を呼ぶ）
/// - branches → Branch terminator（set_branch_with_edge_args を使う）
/// - 1 block = 1 terminator 制約を強制
///
/// # 引数
/// - `function`: MIR function
/// - `frag`: 配線済み Frag
///
/// # 戻り値
/// - `Ok(())`: 成功
/// - `Err(String)`: 同一 block に複数 terminator、または不正な配線
pub fn emit_frag(
    function: &mut crate::mir::MirFunction,
    frag: &super::frag::Frag,
) -> Result<(), String> {
    use super::branch_stub::BranchStub;
    let trace_logger = trace::trace();
    let func_name = function.signature.name.clone();

    // Step 0: verify_frag_invariants_strict() で事前検証（SSOT）
    super::verify::verify_frag_invariants_strict(frag)?;

    // Step 0.5: block_params → PHI 挿入（ValueJoin wiring SSOT）
    emit_block_params_as_phis(function, frag)?;

    // Step 1: branches を from ごとにグループ化（1本だけ許可）
    let mut branches_by_block: BTreeMap<BasicBlockId, Vec<&BranchStub>> = BTreeMap::new();
    for branch in &frag.branches {
        branches_by_block
            .entry(branch.from)
            .or_default()
            .push(branch);
    }

    for (block_id, branches) in &branches_by_block {
        if branches.len() > 1 {
            return Err(format!(
                "[emit_frag] Multiple branches from same block {:?} (count={}). \
                 1 block = 1 terminator constraint violated.",
                block_id,
                branches.len()
            ));
        }
    }

    // Step 2: wires と branches の from 重複チェック（1 block = 1 terminator）
    for wire in &frag.wires {
        if branches_by_block.contains_key(&wire.from) {
            return Err(format!(
                "[emit_frag] Block {:?} has both wire and branch. \
                 1 block = 1 terminator constraint violated.",
                wire.from
            ));
        }
    }

    // Step 3: wires を emit（既存の emit_wires を呼ぶ）
    emit_wires(function, &frag.wires)?;

    // Step 4: branches を emit
    for branch in &frag.branches {
        let block = function
            .get_block_mut(branch.from)
            .ok_or_else(|| format!("[emit_frag] Block {:?} not found", branch.from))?;

        // Phase 260 API を使用（terminator + successors 同期）
        block.set_branch_with_edge_args(
            branch.cond,
            branch.then_target,
            Some(branch.then_args.clone()),
            branch.else_target,
            Some(branch.else_args.clone()),
        );
        trace_logger.debug(
            "lowerer/term_set",
            &format!(
                "func={} bb={:?} term=Branch then={:?} else={:?}",
                func_name, branch.from, branch.then_target, branch.else_target
            ),
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::basic_block::EdgeArgs;
    use crate::mir::function::{FunctionSignature, MirFunction};
    use crate::mir::join_ir::lowering::inline_boundary::JumpArgsLayout;
    use crate::mir::types::MirType;
    use crate::mir::{BasicBlock, EffectMask, ValueId};

    /// テスト用の MirFunction を作成（最小構成）
    fn create_test_function() -> MirFunction {
        let signature = FunctionSignature {
            name: "test_func".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        };
        let entry_block = BasicBlockId(0);
        MirFunction::new(signature, entry_block)
    }

    #[test]
    fn test_emit_wires_jump_basic() {
        // Setup: MirFunction with 2 blocks
        let mut function = create_test_function();
        let bb0 = BasicBlockId(0); // entry
        let bb1 = BasicBlockId(1);
        function.add_block(BasicBlock::new(bb1));

        // Setup: wire (bb0 → bb1)
        let stub = EdgeStub::with_target(
            bb0,
            ExitKind::Normal,
            bb1,
            EdgeArgs {
                layout: JumpArgsLayout::CarriersOnly,
                values: vec![ValueId(100)],
            },
        );

        let wires = vec![stub];

        // Execute
        let result = emit_wires(&mut function, &wires);

        // Verify: success
        assert!(result.is_ok(), "emit_wires failed: {:?}", result.err());

        // Verify: bb0 has Jump terminator
        let block0 = function.get_block(bb0).unwrap();
        assert!(block0.is_terminated(), "bb0 should have a terminator");

        match &block0.terminator {
            Some(MirInstruction::Jump { target, edge_args }) => {
                assert_eq!(*target, bb1, "Jump target should be bb1");
                assert!(edge_args.is_some(), "Jump should have edge_args");
                let args = edge_args.as_ref().unwrap();
                assert_eq!(args.values, vec![ValueId(100)], "Edge args values mismatch");
            }
            other => panic!("Expected Jump terminator, got {:?}", other),
        }

        // Verify: successors updated
        assert!(
            block0.successors.contains(&bb1),
            "bb0 successors should contain bb1"
        );
    }

    #[test]
    fn test_emit_wires_return_basic() {
        // Setup: MirFunction with 1 block
        let mut function = create_test_function();
        let bb0 = BasicBlockId(0); // entry

        // Setup: Return wire（target=None OK、意味を持たない）
        let stub = EdgeStub::new(
            bb0,
            ExitKind::Return,
            None, // Return は target 不要（emit_wires で無視される）
            EdgeArgs {
                layout: JumpArgsLayout::CarriersOnly,
                values: vec![ValueId(200)],
            },
        );

        let wires = vec![stub];

        // Execute
        let result = emit_wires(&mut function, &wires);

        // Verify: success
        assert!(result.is_ok(), "emit_wires failed: {:?}", result.err());

        // Verify: bb0 has Return terminator
        let block0 = function.get_block(bb0).unwrap();
        match &block0.terminator {
            Some(MirInstruction::Return { value }) => {
                assert_eq!(*value, Some(ValueId(200)), "Return value mismatch");
            }
            other => panic!("Expected Return terminator, got {:?}", other),
        }

        // Verify: return_env set
        let return_env = block0.return_env().expect("return_env should be set");
        assert_eq!(
            return_env.values,
            vec![ValueId(200)],
            "return_env values mismatch"
        );
    }

    #[test]
    fn test_emit_wires_unwired_stub_fails() {
        // Setup: EdgeStub with target=None（Normal は target 必須）
        let mut function = create_test_function();
        let bb0 = BasicBlockId(0);

        let stub = EdgeStub::without_args(bb0, ExitKind::Normal);
        // stub.target = None（未配線）

        let wires = vec![stub];

        // Execute
        let result = emit_wires(&mut function, &wires);

        // Verify: failure
        assert!(result.is_err(), "Expected error for unwired Normal stub");
        let err_msg = result.unwrap_err();
        assert!(
            err_msg.contains("Unwired EdgeStub"),
            "Error message should mention 'Unwired EdgeStub', got: {}",
            err_msg
        );
    }

    #[test]
    fn test_emit_wires_multiple_from_same_block_fails() {
        // Setup: 同じ from に2本の wire（1 block = 1 terminator 違反）
        let mut function = create_test_function();
        let bb0 = BasicBlockId(0);
        let bb1 = BasicBlockId(1);
        let bb2 = BasicBlockId(2);
        function.add_block(BasicBlock::new(bb1));
        function.add_block(BasicBlock::new(bb2));

        let stub1 = EdgeStub::new(
            bb0,
            ExitKind::Normal,
            Some(bb1),
            EdgeArgs {
                layout: JumpArgsLayout::CarriersOnly,
                values: vec![],
            },
        );

        let stub2 = EdgeStub::new(
            bb0, // 同じ from
            ExitKind::Normal,
            Some(bb2),
            EdgeArgs {
                layout: JumpArgsLayout::CarriersOnly,
                values: vec![],
            },
        );

        let wires = vec![stub1, stub2];

        // Execute
        let result = emit_wires(&mut function, &wires);

        // Verify: failure
        assert!(
            result.is_err(),
            "Expected error for multiple wires from same block"
        );
        let err_msg = result.unwrap_err();
        assert!(
            err_msg.contains("Multiple wires from same block"),
            "Error message should mention 'Multiple wires from same block', got: {}",
            err_msg
        );
    }

    // ========================================================================
    // Phase 267 P0: emit_frag() テスト（3個）
    // ========================================================================

    #[test]
    fn test_emit_frag_branch_basic() {
        use super::super::branch_stub::BranchStub;
        use super::super::frag::Frag;
        use std::collections::BTreeMap;

        // Setup: MirFunction with 3 blocks (header, then, else)
        let mut function = create_test_function();
        let header = BasicBlockId(0);
        let then_bb = BasicBlockId(1);
        let else_bb = BasicBlockId(2);
        function.add_block(BasicBlock::new(then_bb));
        function.add_block(BasicBlock::new(else_bb));

        // Setup: BranchStub (header → then/else)
        let branch = BranchStub::new(
            header,
            ValueId(100),
            then_bb,
            EdgeArgs {
                layout: JumpArgsLayout::CarriersOnly,
                values: vec![ValueId(101)],
            },
            else_bb,
            EdgeArgs {
                layout: JumpArgsLayout::CarriersOnly,
                values: vec![ValueId(102)],
            },
        );

        let frag = Frag {
            entry: header,
            block_params: BTreeMap::new(),
            exits: BTreeMap::new(),
            wires: vec![],
            branches: vec![branch],
        };

        // Execute
        let result = emit_frag(&mut function, &frag);

        // Verify: success
        assert!(result.is_ok(), "emit_frag failed: {:?}", result.err());

        // Verify: header has Branch terminator
        let block = function.get_block(header).unwrap();
        match &block.terminator {
            Some(MirInstruction::Branch {
                condition,
                then_bb: t,
                else_bb: e,
                then_edge_args,
                else_edge_args,
            }) => {
                assert_eq!(*condition, ValueId(100));
                assert_eq!(*t, then_bb);
                assert_eq!(*e, else_bb);
                assert!(then_edge_args.is_some());
                assert!(else_edge_args.is_some());
                assert_eq!(then_edge_args.as_ref().unwrap().values, vec![ValueId(101)]);
                assert_eq!(else_edge_args.as_ref().unwrap().values, vec![ValueId(102)]);
            }
            other => panic!("Expected Branch, got {:?}", other),
        }

        // Verify: successors updated
        assert!(block.successors.contains(&then_bb));
        assert!(block.successors.contains(&else_bb));
    }

    #[test]
    fn test_emit_frag_block_params_inserts_phi() {
        use super::super::block_params::BlockParams;
        use super::super::frag::Frag;
        use std::collections::BTreeMap;

        let mut function = create_test_function();
        let pred1 = BasicBlockId(1);
        let pred2 = BasicBlockId(2);
        let target = BasicBlockId(3);
        function.add_block(BasicBlock::new(pred1));
        function.add_block(BasicBlock::new(pred2));
        function.add_block(BasicBlock::new(target));

        let args1 = EdgeArgs {
            layout: JumpArgsLayout::ExprResultPlusCarriers,
            values: vec![ValueId(10), ValueId(11)],
        };
        let args2 = EdgeArgs {
            layout: JumpArgsLayout::ExprResultPlusCarriers,
            values: vec![ValueId(12), ValueId(13)],
        };

        let wires = vec![
            EdgeStub::new(pred1, ExitKind::Normal, Some(target), args1),
            EdgeStub::new(pred2, ExitKind::Normal, Some(target), args2),
        ];

        let mut block_params = BTreeMap::new();
        block_params.insert(
            target,
            BlockParams {
                layout: JumpArgsLayout::ExprResultPlusCarriers,
                params: vec![ValueId(100), ValueId(101)],
            },
        );

        let frag = Frag {
            entry: pred1,
            block_params,
            exits: BTreeMap::new(),
            wires,
            branches: vec![],
        };

        emit_frag(&mut function, &frag).expect("emit_frag should succeed");

        let block = function.get_block(target).unwrap();
        match &block.instructions[0] {
            MirInstruction::Phi { dst, inputs, .. } => {
                assert_eq!(*dst, ValueId(100));
                assert_eq!(inputs, &vec![(pred1, ValueId(10)), (pred2, ValueId(12))]);
            }
            other => panic!("Expected Phi at head, got {:?}", other),
        }
        match &block.instructions[1] {
            MirInstruction::Phi { dst, inputs, .. } => {
                assert_eq!(*dst, ValueId(101));
                assert_eq!(inputs, &vec![(pred1, ValueId(11)), (pred2, ValueId(13))]);
            }
            other => panic!("Expected second Phi, got {:?}", other),
        }
    }

    #[test]
    fn test_emit_frag_branch_wire_conflict_fails() {
        use super::super::branch_stub::BranchStub;
        use super::super::frag::Frag;
        use std::collections::BTreeMap;

        // Setup: 同じ block に branch と wire（1 block = 1 terminator 違反）
        let mut function = create_test_function();
        let bb0 = BasicBlockId(0);
        let bb1 = BasicBlockId(1);
        let bb2 = BasicBlockId(2);
        function.add_block(BasicBlock::new(bb1));
        function.add_block(BasicBlock::new(bb2));

        let branch = BranchStub::new(
            bb0,
            ValueId(100),
            bb1,
            EdgeArgs {
                layout: JumpArgsLayout::CarriersOnly,
                values: vec![],
            },
            bb2,
            EdgeArgs {
                layout: JumpArgsLayout::CarriersOnly,
                values: vec![],
            },
        );

        let wire = EdgeStub::with_target(
            bb0, // 同じ from
            ExitKind::Normal,
            bb1,
            EdgeArgs {
                layout: JumpArgsLayout::CarriersOnly,
                values: vec![],
            },
        );

        let frag = Frag {
            entry: bb0,
            block_params: BTreeMap::new(),
            exits: BTreeMap::new(),
            wires: vec![wire],
            branches: vec![branch],
        };

        // Execute
        let result = emit_frag(&mut function, &frag);

        // Verify: failure
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("both wire and branch"));
    }

    #[test]
    fn test_compose_if_creates_branch() {
        use super::super::compose::if_;
        use super::super::frag::Frag;
        use std::collections::BTreeMap;

        // Setup: header, then, else, join blocks
        let header = BasicBlockId(0);
        let then_entry = BasicBlockId(1);
        let else_entry = BasicBlockId(2);
        let join_entry = BasicBlockId(3);

        let then_frag = Frag {
            entry: then_entry,
            block_params: BTreeMap::new(),
            exits: {
                let mut exits = BTreeMap::new();
                exits.insert(
                    ExitKind::Normal,
                    vec![EdgeStub::without_args(then_entry, ExitKind::Normal)],
                );
                exits
            },
            wires: vec![],
            branches: vec![],
        };

        let else_frag = Frag {
            entry: else_entry,
            block_params: BTreeMap::new(),
            exits: {
                let mut exits = BTreeMap::new();
                exits.insert(
                    ExitKind::Normal,
                    vec![EdgeStub::without_args(else_entry, ExitKind::Normal)],
                );
                exits
            },
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

        let cond = ValueId(100);

        // Execute
        let result = if_(
            header,
            cond,
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

        // Verify: 1本の BranchStub が生成された
        assert_eq!(result.branches.len(), 1);

        let branch = &result.branches[0];
        assert_eq!(branch.from, header);
        assert_eq!(branch.cond, cond);
        assert_eq!(branch.then_target, then_entry);
        assert_eq!(branch.else_target, else_entry);
    }
}
