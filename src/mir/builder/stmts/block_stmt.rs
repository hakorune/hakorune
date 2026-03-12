//! Block statement execution module
//!
//! ## Purpose
//! Sequential statement execution with JoinIR suffix router integration
//!
//! ## Responsibilities
//! - Block/statement execution coordination
//! - Phase 142 JoinIR suffix router integration (NormalizedShadowSuffixRouterBox)
//! - Termination checking
//! - Expression delegation
//!
//! ## Architecture
//! - Phase 142 suffix router is the JoinIR integration point
//! - Block execution coordinates statement → expression → block recursion
//! - Termination checking prevents duplicate terminators
//!
//! ## Integration Points
//! - Called by: control_flow::cf_block, expression building code
//! - Calls: build_statement, build_expression, suffix router
//! - Critical: Phase 142 JoinIR suffix router integration must be preserved

use crate::ast::ASTNode;
use crate::mir::builder::MirBuilder;
use crate::mir::builder::ValueId;
use crate::mir::utils::is_current_block_terminated;

/// Build a block by sequentially processing statements
///
/// This is a critical integration point for Phase 142 JoinIR suffix router.
/// The suffix router can consume multiple statements and return the count,
/// allowing the loop to skip ahead.
///
/// # Phase 142 Integration
/// - Uses NormalizedShadowSuffixRouterBox for JoinIR route-shape detection
/// - Suffix router can consume statements and return consumed count
/// - Loop continues processing subsequent statements after suffix match
///
/// # Termination Checking
/// - Checks if block was terminated after each statement
/// - Prevents duplicate terminators (Return, Branch, etc.)
///
/// # Returns
/// - Last statement value, or Void if no statements
pub(in crate::mir::builder) fn build_block(
    builder: &mut MirBuilder,
    statements: Vec<ASTNode>,
) -> Result<ValueId, String> {
    let trace = crate::mir::builder::control_flow::joinir::trace::trace();
    // Scope hint for bare block (Program)
    let scope_id = builder.current_block.map(|bb| bb.as_u32()).unwrap_or(0);
    builder.hint_scope_enter(scope_id);
    let _lex_scope = super::super::vars::lexical_scope::LexicalScopeGuard::new(builder);

    let mut last_value = None;
    let total = statements.len();
    trace.emit_if(
        "debug",
        "build_block",
        &format!("Processing {} statements", total),
        trace.is_enabled(),
    );

    // Phase 132 P0.5: Use while loop instead of for loop to support suffix skipping
    let mut idx = 0;
    while idx < statements.len() {
        // Phase 132 P0.5: Try suffix router (dev-only)
        if crate::config::env::joinir_dev_enabled() {
            let remaining = &statements[idx..];
            // Clone func_name to avoid borrow conflict
            let func_name = builder
                .scope_ctx
                .current_function
                .as_ref()
                .map(|f| f.signature.name.clone())
                .unwrap_or_else(|| "unknown".to_string());
            let debug = trace.is_enabled();

            use crate::mir::builder::control_flow::joinir::route_entry::policies::normalized_shadow_suffix_router_box::NormalizedShadowSuffixRouterBox;
            // Phase 141 P1.5: Pass prefix variables for external env inputs
            // Clone to avoid borrow checker conflict (self is borrowed mutably in try_lower_loop_suffix)
            let prefix_var_map = builder.variable_ctx.variable_map.clone();
            match NormalizedShadowSuffixRouterBox::try_lower_loop_suffix(
                builder,
                remaining,
                &func_name,
                debug,
                Some(&prefix_var_map),
            )? {
                Some(consumed) => {
                    trace.emit_if(
                        "debug",
                        "build_block/suffix_router",
                        &format!("Phase 142 P0: Suffix router consumed {} statement(s), continuing to process subsequent statements", consumed),
                        debug,
                    );
                    // Phase 142 P0: Normalization unit is now "statement (loop 1個)"
                    // Loop normalization returns consumed=1, and subsequent statements
                    // (return, assignments, etc.) are handled by normal MIR lowering
                    idx += consumed;
                    // No break - continue processing subsequent statements
                }
                None => {
                    // No match, proceed with normal statement build
                }
            }
        }

        trace.emit_if(
            "debug",
            "build_block",
            &format!(
                "Statement {}/{}  current_block={:?}  current_function={}",
                idx + 1,
                total,
                builder.current_block,
                builder
                    .scope_ctx
                    .current_function
                    .as_ref()
                    .map(|f| f.signature.name.as_str())
                    .unwrap_or("none")
            ),
            trace.is_enabled(),
        );
        last_value = Some(build_statement(builder, statements[idx].clone())?);
        idx += 1;

        // If the current block was terminated by this statement (e.g., return/throw),
        // do not emit any further instructions for this block.
        if is_current_block_terminated(builder)? {
            trace.emit_if(
                "debug",
                "build_block",
                &format!("Block terminated after statement {}", idx),
                trace.is_enabled(),
            );
            break;
        }
    }
    let out = match last_value {
        Some(v) => v,
        None => {
            // Use ConstantEmissionBox for Void
            crate::mir::builder::emission::constant::emit_void(builder)?
        }
    };
    // Scope leave only if block not already terminated
    if !builder.is_current_block_terminated() {
        builder.hint_scope_leave(scope_id);
    }
    trace.emit_if(
        "debug",
        "build_block",
        &format!("Completed, returning value {:?}", out),
        trace.is_enabled(),
    );
    Ok(out)
}

/// Build a single statement node
///
/// # Phase 212.5: If statement support
/// - Statement-level If (side effects only) is explicitly handled
/// - Expression-level If (value used) goes through build_expression
///
/// # Note
/// - While/ForRange will be delegated to Loop lowering in the future
/// - Currently delegates to build_expression like other specialized builders
pub(in crate::mir::builder) fn build_statement(
    builder: &mut MirBuilder,
    node: ASTNode,
) -> Result<ValueId, String> {
    // Align current_span to this statement node before lowering expressions under it.
    builder.metadata_ctx.set_current_span(node.span());
    match node {
        // Phase 212.5: Statement としての If 処理
        ASTNode::If {
            condition,
            then_body,
            else_body,
            ..
        } => {
            // Statement としての If - 既存 If lowering を呼ぶ
            builder.build_if_statement(*condition, then_body, else_body)?;
            // Statement なので値は使わない（Void を返す）
            Ok(crate::mir::builder::emission::constant::emit_void(builder)?)
        }
        // 将来ここに While / ForRange / Match / Using など statement 専用分岐を追加する。
        other => builder.build_expression(other),
    }
}
