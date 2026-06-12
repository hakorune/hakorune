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
            }
            ExitKind::Return => {
                // Return terminator + metadata（Phase 260 例外ルール: set_terminator + set_return_env）
                block.set_terminator(MirInstruction::Return {
                    value: stub.args.values.first().copied(),
                });
                block.set_return_env(stub.args.clone());
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
    use crate::mir::ssot::cf_common::insert_phi_at_head_spanned;
    use std::collections::BTreeSet;

    if frag.block_params.is_empty() {
        return Ok(());
    }

    let strict = crate::config::env::joinir_strict_enabled()
        || crate::config::env::joinir_dev_enabled();

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
            insert_phi_at_head_spanned(function, *target, *dst, inputs, Span::unknown());
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

    // Step 0: verify_frag_invariants_strict() で事前検証（SSOT）
    super::verify::verify_frag_invariants_strict(frag)?;

    // Step 0.5: block_params → PHI 挿入（ValueJoin wiring SSOT）
    emit_block_params_as_phis(function, frag)?;

    // Step 1: branches を from ごとにグループ化（1本だけ許可）
    let mut branches_by_block: BTreeMap<BasicBlockId, Vec<&BranchStub>> = BTreeMap::new();
    for branch in &frag.branches {
        branches_by_block.entry(branch.from).or_default().push(branch);
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
    }

    Ok(())
}


#[cfg(test)]
#[path = "emit/tests.rs"]
mod tests;
