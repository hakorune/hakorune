//! Verified recipe entrypoints (SSOT).
//!
//! Goal:
//! - Make "Verifier is the only acceptance gate" enforceable in release builds.
//! - Wrap a `RecipeBlock` with a verified contract kind, so lower/dispatch can rely on it.
//!
//! Notes:
//! - Verification is mechanical only (no AST rewrite).
//! - Contract checks are always-on; debug-only checks live elsewhere.

use crate::mir::builder::control_flow::plan::features::carriers;
use crate::mir::builder::control_flow::plan::planner::Freeze;
use crate::mir::builder::control_flow::plan::recipe_tree::{
    BlockContractKind, ExitKind, IfContractKind, IfMode, RecipeBlock, RecipeBodies, RecipeItem,
};
use crate::mir::ValueId;
use crate::{ast::ASTNode, config::env::joinir_dev};
use std::collections::BTreeMap;
use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum ObligationState {
    Defined,
    MaybeUndefined,
    OutOfScope,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::mir::builder) enum PortType {
    Fallthrough,
    Break,
    Continue,
    Return,
}

impl PortType {
    pub(in crate::mir::builder) fn exit_ports() -> [PortType; 3] {
        [PortType::Break, PortType::Continue, PortType::Return]
    }
}

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct PortSig {
    ports: BTreeMap<PortType, BTreeMap<String, ObligationState>>,
    return_seen: bool,
}

impl PortSig {
    fn fallthrough(&self) -> &BTreeMap<String, ObligationState> {
        self.ports
            .get(&PortType::Fallthrough)
            .unwrap_or(&EMPTY_PORT)
    }

    fn port(&self, port: PortType) -> &BTreeMap<String, ObligationState> {
        self.ports.get(&port).unwrap_or(&EMPTY_PORT)
    }

    fn return_seen(&self) -> bool {
        self.return_seen
    }

    fn port_mut(&mut self, port: PortType) -> &mut BTreeMap<String, ObligationState> {
        self.ports.entry(port).or_default()
    }
}

impl Default for PortSig {
    fn default() -> Self {
        let mut ports = BTreeMap::new();
        ports.insert(PortType::Fallthrough, BTreeMap::new());
        ports.insert(PortType::Break, BTreeMap::new());
        ports.insert(PortType::Continue, BTreeMap::new());
        ports.insert(PortType::Return, BTreeMap::new());
        Self {
            ports,
            return_seen: false,
        }
    }
}

static EMPTY_PORT: BTreeMap<String, ObligationState> = BTreeMap::new();

/// A `RecipeBlock` that passed contract verification for a specific `BlockContractKind`.
#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct VerifiedRecipeBlock<'a> {
    arena: &'a RecipeBodies,
    block: &'a RecipeBlock,
    kind: BlockContractKind,
    port_sig: PortSig,
}

impl<'a> VerifiedRecipeBlock<'a> {
    pub fn arena(&self) -> &'a RecipeBodies {
        self.arena
    }

    pub fn block(&self) -> &'a RecipeBlock {
        self.block
    }

    pub fn kind(&self) -> BlockContractKind {
        self.kind
    }

    #[cfg(test)]
    pub(in crate::mir::builder) fn return_port_contains(&self, name: &str) -> bool {
        self.port_sig.port(PortType::Return).contains_key(name)
    }

    #[cfg(test)]
    pub(in crate::mir::builder) fn break_port_contains(&self, name: &str) -> bool {
        self.port_sig.port(PortType::Break).contains_key(name)
    }

    #[cfg(test)]
    pub(in crate::mir::builder) fn continue_port_contains(&self, name: &str) -> bool {
        self.port_sig.port(PortType::Continue).contains_key(name)
    }
}

/// Verify a block contract (check-only, no wrapper).
pub(in crate::mir::builder) fn check_block_contract(
    arena: &RecipeBodies,
    block: &RecipeBlock,
    kind: BlockContractKind,
    context: &str,
) -> Result<(), String> {
    verify_block_contract_impl(arena, block, kind, context)
}

/// Verify a block contract and return a verified wrapper.
///
/// This is intended to be the *single* release-mode acceptance gate:
/// `verify -> VerifiedRecipeBlock -> lower`.
pub(in crate::mir::builder::control_flow::plan) fn verify_block_contract<'a>(
    arena: &'a RecipeBodies,
    block: &'a RecipeBlock,
    kind: BlockContractKind,
    context: &str,
) -> Result<VerifiedRecipeBlock<'a>, String> {
    verify_block_contract_with_pre(arena, block, kind, context, None)
}

pub(in crate::mir::builder::control_flow::plan) fn verify_block_contract_with_pre<'a>(
    arena: &'a RecipeBodies,
    block: &'a RecipeBlock,
    kind: BlockContractKind,
    context: &str,
    pre_bindings: Option<&BTreeMap<String, ValueId>>,
) -> Result<VerifiedRecipeBlock<'a>, String> {
    verify_block_contract_impl(arena, block, kind, context)?;
    let port_sig = build_port_sig_with_pre(arena, block, kind, pre_bindings)?;
    Ok(VerifiedRecipeBlock {
        arena,
        block,
        kind,
        port_sig,
    })
}

pub(in crate::mir::builder) fn verify_port_sig_obligations_if_enabled(
    verified: &VerifiedRecipeBlock<'_>,
    context: &str,
) -> Result<(), String> {
    // SSOT: docs/development/current/main/design/verified-recipe-port-sig-ssot.md
    // PortSig obligations are enforced for NoExit/ExitAllowed/ExitOnly
    // in strict/dev(+planner_required). StmtOnly is excluded.
    if !strict_planner_required() {
        return Ok(());
    }

    match verified.kind() {
        BlockContractKind::StmtOnly => return Ok(()),
        BlockContractKind::NoExit
        | BlockContractKind::ExitAllowed
        | BlockContractKind::ExitOnly => {}
    }

    for (name, state) in verified.port_sig.fallthrough() {
        if *state != ObligationState::Defined {
            let freeze = Freeze::contract(format!(
                "port_sig_fallthrough_not_defined var={name} state={state:?} ctx={context}"
            ));
            return Err(freeze.to_string());
        }
    }

    for port in PortType::exit_ports() {
        if matches!(port, PortType::Return) {
            if verified.port_sig.return_seen() {
                if verified.port_sig.port(port).is_empty() {
                    // Empty return obligation means "no return value" and is allowed.
                    continue;
                }
                for (name, state) in verified.port_sig.port(port) {
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
        for (name, state) in verified.port_sig.port(port) {
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

fn verify_block_contract_impl(
    arena: &RecipeBodies,
    block: &RecipeBlock,
    kind: BlockContractKind,
    context: &str,
) -> Result<(), String> {
    use crate::mir::builder::control_flow::plan::parts::verify as parts_verify;

    match kind {
        BlockContractKind::StmtOnly => {
            parts_verify::verify_stmt_only_block_contract_if_enabled(arena, block, context)?;
        }
        BlockContractKind::NoExit => {
            parts_verify::verify_no_exit_block_contract_if_enabled(arena, block, context)?;
        }
        BlockContractKind::ExitAllowed => {
            parts_verify::verify_block_contract_if_enabled(arena, block, context)?;
        }
        BlockContractKind::ExitOnly => {
            parts_verify::verify_block_contract_if_enabled(arena, block, context)?;
            if !is_exit_only_block(block) {
                return Err(format!(
                    "[freeze:contract][recipe] expected_exit_only_block: ctx={context}"
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

fn build_port_sig_with_pre(
    arena: &RecipeBodies,
    block: &RecipeBlock,
    kind: BlockContractKind,
    pre_bindings: Option<&BTreeMap<String, ValueId>>,
) -> Result<PortSig, String> {
    let Some(body) = arena.get(block.body_id) else {
        return Ok(PortSig::default());
    };

    let mut port_sig = PortSig::default();
    port_sig.return_seen = block_contains_return(arena, block);
    let Some(pre_bindings) = pre_bindings else {
        return Ok(port_sig);
    };
    let base_pre_vars: BTreeSet<String> = pre_bindings.keys().cloned().collect();
    let mut pre_vars = base_pre_vars.clone();
    for var in &base_pre_vars {
        port_sig
            .port_mut(PortType::Fallthrough)
            .insert(var.clone(), ObligationState::Defined);
    }

    for item in &block.items {
        match item {
            RecipeItem::Stmt(stmt_ref) => {
                if let Some(ASTNode::Local { variables, .. }) = body.get_ref(*stmt_ref) {
                    for name in variables {
                        pre_vars.insert(name.clone());
                    }
                }
            }
            RecipeItem::LoopV0 { loop_stmt, .. } => {
                let Some(loop_node) = body.get_ref(*loop_stmt) else {
                    continue;
                };
                let loop_body = match loop_node {
                    ASTNode::Loop { body, .. } | ASTNode::While { body, .. } => body.as_slice(),
                    _ => continue,
                };
                let carrier_vars = carriers::collect_from_body(loop_body).vars;
                // Loop-body locals are block-scoped and excluded from PortSig obligations.
                if strict_planner_required() {
                    for var in &carrier_vars {
                        if !pre_vars.contains(var) {
                            let freeze =
                                Freeze::contract(format!("loop_carrier_missing_in_pre var={var}"));
                            return Err(freeze.to_string());
                        }
                    }
                }
                for var in carrier_vars {
                    port_sig
                        .port_mut(PortType::Break)
                        .insert(var.clone(), ObligationState::Defined);
                    port_sig
                        .port_mut(PortType::Continue)
                        .insert(var, ObligationState::Defined);
                }
            }
            RecipeItem::IfV2 {
                contract,
                then_block,
                else_block,
                ..
            } => {
                if !matches!(contract, IfContractKind::Join) {
                    continue;
                }
                let then_sig = build_port_sig_with_pre(
                    arena,
                    then_block,
                    BlockContractKind::NoExit,
                    Some(pre_bindings),
                )?;
                for (name, state) in then_sig.fallthrough() {
                    if *state == ObligationState::OutOfScope {
                        port_sig
                            .port_mut(PortType::Fallthrough)
                            .insert(name.clone(), ObligationState::OutOfScope);
                    }
                }
                if let Some(else_block) = else_block.as_deref() {
                    let else_sig = build_port_sig_with_pre(
                        arena,
                        else_block,
                        BlockContractKind::NoExit,
                        Some(pre_bindings),
                    )?;
                    for (name, state) in else_sig.fallthrough() {
                        if *state == ObligationState::OutOfScope {
                            port_sig
                                .port_mut(PortType::Fallthrough)
                                .insert(name.clone(), ObligationState::OutOfScope);
                        }
                    }
                }
                if else_block.is_some() {
                    continue;
                }
                // then-only: branch-local locals are scoped and excluded from join obligations.
            }
            _ => {}
        }
    }

    if matches!(
        kind,
        BlockContractKind::ExitAllowed | BlockContractKind::ExitOnly
    ) {
        for var in &base_pre_vars {
            port_sig
                .port_mut(PortType::Break)
                .insert(var.clone(), ObligationState::Defined);
            port_sig
                .port_mut(PortType::Continue)
                .insert(var.clone(), ObligationState::Defined);
            port_sig
                .port_mut(PortType::Return)
                .insert(var.clone(), ObligationState::Defined);
        }
    }

    Ok(port_sig)
}

fn block_contains_return(arena: &RecipeBodies, block: &RecipeBlock) -> bool {
    for item in &block.items {
        match item {
            RecipeItem::Exit {
                kind: ExitKind::Return,
                ..
            } => return true,
            RecipeItem::IfV2 {
                then_block,
                else_block,
                ..
            } => {
                if block_contains_return(arena, then_block) {
                    return true;
                }
                if else_block
                    .as_deref()
                    .is_some_and(|eb| block_contains_return(arena, eb))
                {
                    return true;
                }
            }
            RecipeItem::LoopV0 { body_block, .. } => {
                if block_contains_return(arena, body_block) {
                    return true;
                }
            }
            _ => {}
        }
    }
    false
}

fn collect_local_vars_from_block_recursive(
    arena: &RecipeBodies,
    block: &RecipeBlock,
) -> BTreeSet<String> {
    let mut locals = BTreeSet::new();
    let Some(body) = arena.get(block.body_id) else {
        return locals;
    };
    for item in &block.items {
        match item {
            RecipeItem::Stmt(stmt_ref) => {
                if let Some(ASTNode::Local { variables, .. }) = body.get_ref(*stmt_ref) {
                    for name in variables {
                        locals.insert(name.clone());
                    }
                }
            }
            RecipeItem::IfV2 {
                then_block,
                else_block,
                ..
            } => {
                locals.extend(collect_local_vars_from_block_recursive(arena, then_block));
                if let Some(else_block) = else_block.as_deref() {
                    locals.extend(collect_local_vars_from_block_recursive(arena, else_block));
                }
            }
            RecipeItem::LoopV0 { body_block, .. } => {
                locals.extend(collect_local_vars_from_block_recursive(arena, body_block));
            }
            _ => {}
        }
    }
    locals
}

fn is_block_exit_only_item(item: &RecipeItem) -> bool {
    match item {
        RecipeItem::Exit { .. } => true,
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
