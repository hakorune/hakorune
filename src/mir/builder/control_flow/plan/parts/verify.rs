//! RecipeVerifier (mechanical checks only).
//!
//! SSOT: docs/development/current/main/design/recipe-tree-and-parts-ssot.md
//!
//! Fail-fast policy:
//! - Contract checks are always enabled.
//! - Debug-only checks stay behind dev/strict guards.
//! - On violation returns `Err("[freeze:contract][recipe] ...")`.

use crate::ast::ASTNode;
use crate::config::env::joinir_dev;
use crate::mir::builder::control_flow::plan::planner::Freeze;
use crate::mir::builder::control_flow::plan::recipe_tree::verified::ObligationState;
use crate::mir::builder::control_flow::plan::recipe_tree::verified::VerifiedRecipeBlock;
use crate::mir::builder::control_flow::plan::recipe_tree::{
    BlockContractKind, ExitKind, IfContractKind, IfMode, RecipeBlock, RecipeBodies, RecipeItem,
};
use crate::mir::builder::control_flow::plan::recipes::RecipeBody;

// ============================================================================
// RecipeBlock verifier (M5m-3)
// ============================================================================

/// Verify RecipeBlock contract (always-on).
pub(in crate::mir::builder) fn verify_block_contract_if_enabled(
    arena: &RecipeBodies,
    block: &RecipeBlock,
    context: &str,
) -> Result<(), String> {
    verify_block_contract(arena, block, context)
}

/// Verify stmt-only RecipeBlock contract (always-on).
pub(in crate::mir::builder) fn verify_stmt_only_block_contract_if_enabled(
    arena: &RecipeBodies,
    block: &RecipeBlock,
    context: &str,
) -> Result<(), String> {
    verify_stmt_only_block_contract(arena, block, context)
}

pub(in crate::mir::builder) fn verify_no_exit_block_contract_if_enabled(
    arena: &RecipeBodies,
    block: &RecipeBlock,
    context: &str,
) -> Result<(), String> {
    verify_no_exit_block_contract(arena, block, context)
}

// ============================================================================
// Debug-only verifier entrypoints (Lower-side safety net)
// ============================================================================

#[cfg(debug_assertions)]
pub(in crate::mir::builder) fn debug_check_block_contract(
    arena: &RecipeBodies,
    block: &RecipeBlock,
    context: &str,
) -> Result<(), String> {
    verify_block_contract_if_enabled(arena, block, context)
}

#[cfg(debug_assertions)]
pub(in crate::mir::builder) fn debug_check_stmt_only_block_contract(
    arena: &RecipeBodies,
    block: &RecipeBlock,
    context: &str,
) -> Result<(), String> {
    verify_stmt_only_block_contract_if_enabled(arena, block, context)
}

#[cfg(debug_assertions)]
pub(in crate::mir::builder) fn debug_check_no_exit_block_contract(
    arena: &RecipeBodies,
    block: &RecipeBlock,
    context: &str,
) -> Result<(), String> {
    verify_no_exit_block_contract_if_enabled(arena, block, context)
}

fn verify_no_exit_block_contract(
    arena: &RecipeBodies,
    block: &RecipeBlock,
    context: &str,
) -> Result<(), String> {
    let Some(body) = arena.get(block.body_id) else {
        return Err(format!(
            "[freeze:contract][recipe] invalid_body_id: ctx={context}"
        ));
    };
    if block.items.len() > body.len() {
        return Err(format!(
            "[freeze:contract][recipe] block_items_len_exceeds_body_len: body_len={} items_len={} body_id={} ctx={context}",
            body.len(),
            block.items.len(),
            block.body_id.0
        ));
    }

    for item in &block.items {
        #[allow(unreachable_patterns)]
        match item {
            RecipeItem::Stmt(stmt_ref) => {
                if body.get_ref(*stmt_ref).is_none() {
                    return Err(format!(
                        "[freeze:contract][recipe] stmt_ref_out_of_range: idx={} ctx={context}",
                        stmt_ref.index()
                    ));
                }
            }
            RecipeItem::LoopV0 {
                loop_stmt,
                body_block,
                body_contract,
                ..
            } => {
                if body.get_ref(*loop_stmt).is_none() {
                    return Err(format!(
                        "[freeze:contract][recipe] stmt_ref_out_of_range: idx={} ctx={context}",
                        loop_stmt.index()
                    ));
                }
                if let Some(node) = body.get_ref(*loop_stmt) {
                    if !matches!(node, ASTNode::Loop { .. } | ASTNode::While { .. }) {
                        return Err(format!(
                            "[freeze:contract][recipe] loop_stmt_is_not_loop_node: idx={} ctx={context}",
                            loop_stmt.index()
                        ));
                    }
                }

                match body_contract {
                    BlockContractKind::StmtOnly => {
                        verify_stmt_only_block_contract(arena, body_block, context)?;
                    }
                    BlockContractKind::NoExit => {
                        verify_no_exit_block_contract(arena, body_block, context)?;
                    }
                    BlockContractKind::ExitAllowed => {
                        verify_block_contract(arena, body_block, context)?;
                    }
                    BlockContractKind::ExitOnly => {
                        verify_block_contract(arena, body_block, context)?;
                        if !is_exit_only_block(body_block) {
                            return Err(format!(
                                "[freeze:contract][recipe] loop_body_must_be_exit_only: ctx={context}"
                            ));
                        }
                    }
                }
            }
            RecipeItem::IfV2 {
                if_stmt,
                cond_view: _cond_view,
                contract,
                then_block,
                else_block,
            } => match contract {
                IfContractKind::Join => {
                    if body.get_ref(*if_stmt).is_none() {
                        return Err(format!(
                            "[freeze:contract][recipe] if_stmt_ref_out_of_range: idx={} ctx={context}",
                            if_stmt.index()
                        ));
                    }
                    if let Some(node) = body.get_ref(*if_stmt) {
                        if !matches!(node, ASTNode::If { .. }) {
                            return Err(format!(
                                "[freeze:contract][recipe] if_stmt_is_not_if_node: idx={} ctx={context}",
                                if_stmt.index()
                            ));
                        }
                    }
                    verify_no_exit_block_contract(arena, then_block, context)?;
                    if let Some(eb) = else_block.as_deref() {
                        verify_no_exit_block_contract(arena, eb, context)?;
                    }
                    verify_if_join_obligations_if_enabled(
                        arena,
                        then_block,
                        else_block.as_deref(),
                    )?;
                }
                _ => {
                    return Err(format!(
                        "[freeze:contract][recipe] no_exit_block_contains_unsupported_item: ctx={context}"
                    ));
                }
            },
            _ => {
                return Err(format!(
                    "[freeze:contract][recipe] no_exit_block_contains_unsupported_item: ctx={context}"
                ));
            }
        }
    }

    Ok(())
}

fn strict_planner_required() -> bool {
    let strict_or_dev = joinir_dev::strict_enabled() || crate::config::env::joinir_dev_enabled();
    strict_or_dev && joinir_dev::planner_required_enabled()
}

fn verify_if_join_obligations_if_enabled(
    _arena: &RecipeBodies,
    _then_block: &RecipeBlock,
    _else_block: Option<&RecipeBlock>,
) -> Result<(), String> {
    if !strict_planner_required() {
        return Ok(());
    }
    Ok(())
}

pub(in crate::mir::builder) fn verify_port_sig_obligations_if_enabled(
    verified: &VerifiedRecipeBlock<'_>,
    context: &str,
) -> Result<(), String> {
    use crate::mir::builder::control_flow::plan::recipe_tree::verified::PortType;

    // SSOT: docs/development/current/main/design/verified-recipe-port-sig-ssot.md
    // PortSig obligations are enforced for NoExit/ExitAllowed/ExitOnly
    // in strict/dev(+planner_required). StmtOnly is excluded.
    if !strict_planner_required() {
        return Ok(());
    }

    // Exclude StmtOnly (no PortSig obligations)
    match verified.kind() {
        BlockContractKind::StmtOnly => return Ok(()),
        BlockContractKind::NoExit
        | BlockContractKind::ExitAllowed
        | BlockContractKind::ExitOnly => {}
    }

    for (name, state) in verified.port_sig().fallthrough() {
        if *state != ObligationState::Defined {
            let freeze = Freeze::contract(format!(
                "port_sig_fallthrough_not_defined var={name} state={state:?} ctx={context}"
            ));
            return Err(freeze.to_string());
        }
    }

    for port in PortType::exit_ports() {
        if matches!(port, PortType::Return) {
            if verified.port_sig().return_seen() {
                if verified.port_sig().port(port).is_empty() {
                    // Empty return obligation means "no return value" and is allowed.
                    continue;
                }
                for (name, state) in verified.port_sig().port(port) {
                    if *state != ObligationState::Defined {
                        let freeze = Freeze::contract(format!(
                            "port_sig_exit_not_defined port={port:?} var={name} state={state:?} ctx={context}"
                        ));
                        return Err(freeze.to_string());
                    }
                }
            }
            continue;
        }
        for (name, state) in verified.port_sig().port(port) {
            if *state != ObligationState::Defined {
                let freeze = Freeze::contract(format!(
                    "port_sig_exit_not_defined port={port:?} var={name} state={state:?} ctx={context}"
                ));
                return Err(freeze.to_string());
            }
        }
    }

    Ok(())
}

fn verify_stmt_only_block_contract(
    arena: &RecipeBodies,
    block: &RecipeBlock,
    context: &str,
) -> Result<(), String> {
    let Some(body) = arena.get(block.body_id) else {
        return Err(format!(
            "[freeze:contract][recipe] invalid_body_id: ctx={context}"
        ));
    };
    if block.items.len() > body.len() {
        return Err(format!(
            "[freeze:contract][recipe] block_items_len_exceeds_body_len: body_len={} items_len={} body_id={} ctx={context}",
            body.len(),
            block.items.len(),
            block.body_id.0
        ));
    }

    for item in &block.items {
        let RecipeItem::Stmt(stmt_ref) = item else {
            return Err(format!(
                "[freeze:contract][recipe] stmt_only_block_contains_non_stmt_item: ctx={context}"
            ));
        };
        if body.get_ref(*stmt_ref).is_none() {
            return Err(format!(
                "[freeze:contract][recipe] stmt_ref_out_of_range: idx={} ctx={context}",
                stmt_ref.index()
            ));
        }
    }

    Ok(())
}

fn verify_block_contract(
    arena: &RecipeBodies,
    block: &RecipeBlock,
    context: &str,
) -> Result<(), String> {
    // Verifier is the only place that interprets `*_contract` enums.
    // Lowering must not re-derive / re-validate acceptance rules (avoid double-SSOT).
    // 1) body_id が有効
    let Some(body) = arena.get(block.body_id) else {
        return Err(format!(
            "[freeze:contract][recipe] invalid_body_id: ctx={context}"
        ));
    };
    if block.items.len() > body.len() {
        return Err(format!(
            "[freeze:contract][recipe] block_items_len_exceeds_body_len: body_len={} items_len={} body_id={} ctx={context}",
            body.len(),
            block.items.len(),
            block.body_id.0
        ));
    }

    // 2) 各 item が body 範囲内参照をしている（StmtRefが存在するか）
    // 3) Seq中の trailing-after-exit 禁止（Exit/exit-only If の後に item が続いたらNG）
    for (idx, item) in block.items.iter().enumerate() {
        verify_item_refs_in_range(body, item, context)?;

        if is_block_exit_only_item(item) && idx + 1 < block.items.len() {
            return Err(format!(
                "[freeze:contract][recipe] block_has_trailing_after_exit: ctx={context}"
            ));
        }

        match item {
            RecipeItem::IfV2 {
                contract,
                then_block,
                else_block,
                ..
            } => {
                let IfContractKind::ExitOnly { mode } = contract else {
                    continue;
                };

                match mode {
                    IfMode::ExitAll if else_block.is_none() => {
                        return Err(format!(
                            "[freeze:contract][recipe] if_exit_all_requires_else: ctx={context}"
                        ));
                    }
                    _ => {}
                }

                verify_block_contract(arena, then_block, context)?;
                if !is_exit_only_block(then_block) {
                    return Err(format!(
                        "[freeze:contract][recipe] if_then_must_be_exit_only: ctx={context}"
                    ));
                }

                if let Some(eb) = else_block.as_deref() {
                    match mode {
                        IfMode::ExitAll => {
                            verify_block_contract(arena, eb, context)?;
                            if !is_exit_only_block(eb) {
                                return Err(format!(
                                    "[freeze:contract][recipe] if_else_must_be_exit_only: ctx={context}"
                                ));
                            }
                        }
                        IfMode::ExitIf => {
                            // else branch is allowed, but must be fallthrough (no Exit items).
                            verify_no_exit_block_contract(arena, eb, context)?;
                        }
                        IfMode::ElseOnlyExit => {
                            // ElseOnlyExit is handled via IfContractKind::ExitAllowed,
                            // not ExitOnly, so this branch should not be reached.
                        }
                    }
                }
            }
            RecipeItem::LoopV0 {
                body_block,
                body_contract,
                ..
            } => match body_contract {
                // NOTE: `LoopV0` is structure-only; contract checks are verifier-only.
                BlockContractKind::StmtOnly => {
                    verify_stmt_only_block_contract(arena, body_block, context)?;
                }
                BlockContractKind::NoExit => {
                    verify_no_exit_block_contract(arena, body_block, context)?;
                }
                BlockContractKind::ExitAllowed => {
                    verify_block_contract(arena, body_block, context)?;
                }
                BlockContractKind::ExitOnly => {
                    verify_block_contract(arena, body_block, context)?;
                    if !is_exit_only_block(body_block) {
                        return Err(format!(
                            "[freeze:contract][recipe] loop_body_must_be_exit_only: ctx={context}"
                        ));
                    }
                }
            },
            _ => {}
        }
    }

    // 6) exit-only block なら末尾が exit-only item であること
    // （dispatch側でも見てるが、検証として固定。将来的に dispatch 側の再判定は strict/dev へ縮退する。）
    if is_exit_only_block(block) && !block.items.last().is_some_and(is_block_exit_only_item) {
        return Err(format!(
            "[freeze:contract][recipe] exit_only_block_must_end_with_exit: ctx={context}"
        ));
    }

    Ok(())
}

fn verify_item_refs_in_range(
    body: &RecipeBody,
    item: &RecipeItem,
    context: &str,
) -> Result<(), String> {
    #[allow(unreachable_patterns)]
    match item {
        RecipeItem::Stmt(r) => {
            if body.get_ref(*r).is_none() {
                return Err(format!(
                    "[freeze:contract][recipe] stmt_ref_out_of_range: idx={} ctx={context}",
                    r.index()
                ));
            }
            Ok(())
        }
        RecipeItem::Exit { kind, stmt } => {
            if body.get_ref(*stmt).is_none() {
                return Err(format!(
                    "[freeze:contract][recipe] stmt_ref_out_of_range: idx={} ctx={context}",
                    stmt.index()
                ));
            }
            // M25: depth!=1 is unsupported
            match kind {
                ExitKind::Break { depth } | ExitKind::Continue { depth } if *depth != 1 => {
                    return Err(format!(
                        "[freeze:contract][recipe][exit_depth] depth={} unsupported (only depth=1): ctx={context}",
                        depth
                    ));
                }
                _ => {}
            }
            Ok(())
        }
        RecipeItem::IfV2 { contract, .. } if matches!(contract, IfContractKind::Join) => Err(
            format!("[freeze:contract][recipe] exit_only_verifier_saw_if_join: ctx={context}"),
        ),
        RecipeItem::IfV2 {
            if_stmt,
            cond_view: _cond_view,
            contract,
            then_block,
            else_block,
        } => {
            // Accept both ExitOnly and ExitAllowed (Phase 29bq: else-only-exit pattern)
            match contract {
                IfContractKind::ExitOnly { .. } | IfContractKind::ExitAllowed { .. } => {}
                _ => {
                    return Err(format!(
                        "[freeze:contract][recipe] verifier_saw_unsupported_item: ctx={context}"
                    ));
                }
            }

            if body.get_ref(*if_stmt).is_none() {
                return Err(format!(
                    "[freeze:contract][recipe] if_stmt_ref_out_of_range: idx={} ctx={context}",
                    if_stmt.index()
                ));
            }
            if let Some(node) = body.get_ref(*if_stmt) {
                if !matches!(node, ASTNode::If { .. }) {
                    return Err(format!(
                        "[freeze:contract][recipe] if_stmt_is_not_if_node: idx={} ctx={context}",
                        if_stmt.index()
                    ));
                }
            }
            let _ = then_block;
            let _ = else_block;
            Ok(())
        }
        RecipeItem::LoopV0 {
            loop_stmt,
            cond_view: _cond_view,
            body_block: _body_block,
            body_contract: _body_contract,
            kind: _kind,
            features: _features,
        } => {
            if body.get_ref(*loop_stmt).is_none() {
                return Err(format!(
                    "[freeze:contract][recipe] stmt_ref_out_of_range: idx={} ctx={context}",
                    loop_stmt.index()
                ));
            }
            if let Some(node) = body.get_ref(*loop_stmt) {
                if !matches!(node, ASTNode::Loop { .. } | ASTNode::While { .. }) {
                    return Err(format!(
                        "[freeze:contract][recipe] loop_stmt_is_not_loop_node: idx={} ctx={context}",
                        loop_stmt.index()
                    ));
                }
            }
            Ok(())
        }
        _ => Err(format!(
            "[freeze:contract][recipe] verifier_saw_unsupported_item: ctx={context}"
        )),
    }
}

fn is_block_exit_only_item(item: &RecipeItem) -> bool {
    #[allow(unreachable_patterns)]
    match item {
        RecipeItem::Exit { .. } => true,
        RecipeItem::Stmt(_) => false,
        RecipeItem::IfV2 { contract, .. } if matches!(contract, IfContractKind::Join) => false,
        RecipeItem::IfV2 {
            contract,
            then_block,
            else_block,
            ..
        } => {
            let IfContractKind::ExitOnly { mode } = contract else {
                return false;
            };
            match mode {
                // ExitIf exits only on the `then` path; it must not be treated as a
                // block-exit item (trailing items are allowed).
                IfMode::ExitIf => false,
                IfMode::ExitAll => else_block
                    .as_deref()
                    .is_some_and(|eb| is_exit_only_block(then_block) && is_exit_only_block(eb)),
                // ElseOnlyExit: then falls through, so not a block-exit item
                IfMode::ElseOnlyExit => false,
            }
        }
        _ => false,
    }
}

fn is_exit_only_block(block: &RecipeBlock) -> bool {
    block.items.last().is_some_and(is_block_exit_only_item)
        && block
            .items
            .iter()
            .enumerate()
            .all(|(i, it)| !(is_block_exit_only_item(it) && i + 1 < block.items.len()))
}
