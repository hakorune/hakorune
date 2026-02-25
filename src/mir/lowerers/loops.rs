use crate::ast::ASTNode;
use crate::mir::builder::{BlockId, MirBuilder};
use crate::mir::lowerers::LoweringError;

/// Stage-3 loop lowering helpers (while / for-range).
///
/// Enabled when Stage-3 parsing is active (default ON via NYASH_FEATURES=stage3,
/// legacy NYASH_PARSER_STAGE3/HAKO_PARSER_STAGE3 still accepted). This module
/// provides minimal lowering support so Stage-3 style loops used by tools like
/// hako_check can execute on the existing MIR interpreter without modifying
/// default behavior when Stage-3 is disabled.
pub struct LoopLowerer;

impl LoopLowerer {
    /// Lower a Stage-3 style `while` loop:
    ///
    /// while <cond_expr> { <stmts> }
    ///
    /// Semantics:
    /// - Evaluate cond at loop_head.
    /// - If false, jump to exit.
    /// - If true, execute body, then jump back to loop_head.
    pub fn lower_while(
        builder: &mut MirBuilder,
        condition: &ASTNode,
        body: &[ASTNode],
    ) -> Result<(), LoweringError> {
        let func = builder.current_function_id()?;
        let loop_head: BlockId = builder.new_block(func);
        let body_blk: BlockId = builder.new_block(func);
        let exit_blk: BlockId = builder.new_block(func);

        // Jump from current block into loop_head
        builder.ensure_terminator_goto(loop_head)?;

        // loop_head: evaluate condition
        builder.set_insert_point(loop_head)?;
        let cond_val = builder.lower_expr_bool(condition)?;
        builder.build_cond_br(cond_val, body_blk, exit_blk)?;

        // body: lower statements, then jump back to loop_head if not already terminated
        builder.set_insert_point(body_blk)?;
        for stmt in body {
            builder.lower_statement(stmt)?;
        }
        builder.ensure_terminator_goto(loop_head)?;

        // Continue after loop
        builder.set_insert_point(exit_blk)?;
        Ok(())
    }

    /// Lower a minimal `for` range loop:
    ///
    /// for <ident> in <start_expr> .. <end_expr> { <stmts> }
    ///
    /// Semantics (half-open [start, end)):
    /// - init: i = start
    /// - loop_head: if i < end then body else exit
    /// - body: execute stmts
    /// - step: i = i + 1, jump back to loop_head
    pub fn lower_for_range(
        builder: &mut MirBuilder,
        var_name: &str,
        start_expr: &ASTNode,
        end_expr: &ASTNode,
        body: &[ASTNode],
    ) -> Result<(), LoweringError> {
        let func = builder.current_function_id()?;
        let init_blk: BlockId = builder.new_block(func);
        let loop_head: BlockId = builder.new_block(func);
        let body_blk: BlockId = builder.new_block(func);
        let step_blk: BlockId = builder.new_block(func);
        let exit_blk: BlockId = builder.new_block(func);

        // Jump into init from current position
        builder.ensure_terminator_goto(init_blk)?;

        // init: i = start_expr
        builder.set_insert_point(init_blk)?;
        let start_val = builder.lower_expr_i64(start_expr)?;
        let idx_slot = builder.declare_local_i64(var_name)?;
        builder.build_store_local(idx_slot, start_val)?;
        builder.build_goto(loop_head)?;

        // loop_head: cond = (i < end)
        builder.set_insert_point(loop_head)?;
        let cur_val = builder.build_load_local_i64(idx_slot)?;
        let end_val = builder.lower_expr_i64(end_expr)?;
        let cond_val = builder.build_icmp_lt(cur_val, end_val)?;
        builder.build_cond_br(cond_val, body_blk, exit_blk)?;

        // body: user statements
        builder.set_insert_point(body_blk)?;
        for stmt in body {
            builder.lower_statement(stmt)?;
        }
        builder.ensure_terminator_goto(step_blk)?;

        // step: i = i + 1; goto loop_head
        builder.set_insert_point(step_blk)?;
        let cur2 = builder.build_load_local_i64(idx_slot)?;
        let one = builder.const_i64(1);
        let next = builder.build_add_i64(cur2, one)?;
        builder.build_store_local(idx_slot, next)?;
        builder.build_goto(loop_head)?;

        // exit: continue
        builder.set_insert_point(exit_blk)?;
        Ok(())
    }
}
