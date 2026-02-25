//! Try/catch/finally exception handling implementation.
//!
//! This module implements the control flow for try/catch/finally blocks,
//! including proper handling of deferred returns and cleanup blocks.

use crate::ast::ASTNode;
use crate::mir::builder::{MirInstruction, ValueId};

/// Control-flow: try/catch/finally
///
/// Implements exception handling with:
/// - Try block execution
/// - Catch clause handling
/// - Finally cleanup block
/// - Deferred return state management
pub(in crate::mir::builder) fn cf_try_catch(
    builder: &mut super::super::super::MirBuilder,
    try_body: Vec<ASTNode>,
    catch_clauses: Vec<crate::ast::CatchClause>,
    finally_body: Option<Vec<ASTNode>>,
) -> Result<ValueId, String> {
    if crate::config::env::builder_disable_trycatch() {
        let try_ast = ASTNode::Program {
            statements: try_body,
            span: crate::ast::Span::unknown(),
        };
        let result = builder.build_expression(try_ast)?;
        return Ok(result);
    }

    let try_block = builder.next_block_id();
    let catch_block = builder.next_block_id();
    let finally_block = if finally_body.is_some() {
        Some(builder.next_block_id())
    } else {
        None
    };
    let exit_block = builder.next_block_id();

    // Snapshot deferred-return state
    let saved_defer_active = builder.return_defer_active;
    let saved_defer_slot = builder.return_defer_slot;
    let saved_defer_target = builder.return_defer_target;
    let saved_deferred_flag = builder.return_deferred_emitted;
    let saved_in_cleanup = builder.in_cleanup_block;
    let saved_allow_ret = builder.cleanup_allow_return;
    let saved_allow_throw = builder.cleanup_allow_throw;

    let ret_slot = builder.next_value_id();
    builder.return_defer_active = true;
    builder.return_defer_slot = Some(ret_slot);
    builder.return_deferred_emitted = false;
    builder.return_defer_target = Some(finally_block.unwrap_or(exit_block));

    if let Some(catch_clause) = catch_clauses.first() {
        if crate::config::env::builder_trycatch_debug() {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!(
                "[BUILDER] Emitting catch handler for {:?}",
                catch_clause.exception_type
            ));
        }
        let exception_value = builder.next_value_id();
        builder.emit_instruction(MirInstruction::Catch {
            exception_type: catch_clause.exception_type.clone(),
            exception_value,
            handler_bb: catch_block,
        })?;
    }

    // Enter try block
    crate::mir::builder::emission::branch::emit_jump(builder, try_block)?;
    builder.start_new_block(try_block)?;
    let try_ast = ASTNode::Program {
        statements: try_body,
        span: crate::ast::Span::unknown(),
    };
    let _try_result = builder.build_expression(try_ast)?;
    if !builder.is_current_block_terminated() {
        let next_target = finally_block.unwrap_or(exit_block);
        crate::mir::builder::emission::branch::emit_jump(builder, next_target)?;
    }

    // Catch block
    builder.start_new_block(catch_block)?;
    if crate::config::env::builder_trycatch_debug() {
        let ring0 = crate::runtime::get_global_ring0();
        ring0.log.debug(&format!("[BUILDER] Enter catch block {:?}", catch_block));
    }
    if let Some(catch_clause) = catch_clauses.first() {
        let catch_ast = ASTNode::Program {
            statements: catch_clause.body.clone(),
            span: crate::ast::Span::unknown(),
        };
        builder.build_expression(catch_ast)?;
    }
    if !builder.is_current_block_terminated() {
        let next_target = finally_block.unwrap_or(exit_block);
        crate::mir::builder::emission::branch::emit_jump(builder, next_target)?;
    }

    // Finally
    let mut cleanup_terminated = false;
    if let (Some(finally_block_id), Some(finally_statements)) = (finally_block, finally_body) {
        builder.start_new_block(finally_block_id)?;
        builder.in_cleanup_block = true;
        builder.cleanup_allow_return = crate::config::env::cleanup_allow_return();
        builder.cleanup_allow_throw = crate::config::env::cleanup_allow_throw();
        builder.return_defer_active = false; // do not defer inside cleanup

        let finally_ast = ASTNode::Program {
            statements: finally_statements,
            span: crate::ast::Span::unknown(),
        };
        builder.build_expression(finally_ast)?;
        cleanup_terminated = builder.is_current_block_terminated();
        if !cleanup_terminated {
            crate::mir::builder::emission::branch::emit_jump(builder, exit_block)?;
        }
        builder.in_cleanup_block = false;
    }

    // Exit block
    builder.start_new_block(exit_block)?;
    if builder.return_deferred_emitted && !cleanup_terminated {
        builder.emit_instruction(MirInstruction::Return {
            value: Some(ret_slot),
        })?;
    }
    let result = crate::mir::builder::emission::constant::emit_void(builder)?;

    // Restore context
    builder.return_defer_active = saved_defer_active;
    builder.return_defer_slot = saved_defer_slot;
    builder.return_defer_target = saved_defer_target;
    builder.return_deferred_emitted = saved_deferred_flag;
    builder.in_cleanup_block = saved_in_cleanup;
    builder.cleanup_allow_return = saved_allow_ret;
    builder.cleanup_allow_throw = saved_allow_throw;

    Ok(result)
}
