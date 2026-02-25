//! Pattern 8: Boolean Predicate Scan (is_integer/is_valid form) - plan-side implementation

use crate::ast::{ASTNode, BinaryOperator, LiteralValue, UnaryOperator};
use crate::mir::builder::MirBuilder;
use crate::mir::ValueId;

use crate::mir::builder::control_flow::joinir::trace;

/// Phase 259 P0: Extracted structure for boolean predicate scan
#[derive(Debug, Clone)]
pub(crate) struct BoolPredicateScanParts {
    /// Loop variable name (e.g., "i")
    pub(crate) loop_var: String,
    /// Haystack variable name (e.g., "s")
    pub(crate) haystack: String,
    /// Predicate method receiver (e.g., "this")
    pub(crate) predicate_receiver: String,
    /// Predicate method name (e.g., "is_digit")
    pub(crate) predicate_method: String,
    /// Step literal (P0: must be 1)
    pub(crate) step_lit: i64,
}

/// Phase 259 P0: Extract boolean predicate scan pattern parts
///
/// # P0 Restrictions (Fail-Fast)
///
/// - Loop condition: `i < s.length()` (forward only)
/// - If condition: `not this.method(s.substring(i, i + 1))` (UnaryOp::Not)
/// - Then branch: `return false` (Literal::Bool(false))
/// - Step: `i = i + 1` (step_lit == 1)
/// - Post-loop: `return true` (enforced by caller)
pub(crate) fn extract_bool_predicate_scan_parts(
    condition: &ASTNode,
    body: &[ASTNode],
) -> Result<Option<BoolPredicateScanParts>, String> {
    // Phase 269 P1: Debug output removed (was breaking quick smoke output)

    // 1. Check loop condition: i < s.length()
    let (loop_var, haystack) = match condition {
        ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left,
            right,
            ..
        } => {
            let loop_var = match left.as_ref() {
                ASTNode::Variable { name, .. } => name.clone(),
                _ => return Ok(None),
            };

            let haystack = match right.as_ref() {
                ASTNode::MethodCall {
                    object, method, ..
                } if method == "length" => match object.as_ref() {
                    ASTNode::Variable { name, .. } => name.clone(),
                    _ => return Ok(None),
                },
                _ => return Ok(None),
            };

            (loop_var, haystack)
        }
        _ => return Ok(None),
    };

    // 2. Find if statement with predicate check and return false
    let mut predicate_receiver_opt = None;
    let mut predicate_method_opt = None;

    for stmt in body.iter() {
        if let ASTNode::If {
            condition: if_cond,
            then_body,
            ..
        } = stmt
        {
            // Check if condition is: not this.method(...)
            if let ASTNode::UnaryOp {
                operator: UnaryOperator::Not,
                operand,
                ..
            } = if_cond.as_ref()
            {
                // Operand must be MethodCall
                if let ASTNode::MethodCall {
                    object,
                    method,
                    arguments,
                    ..
                } = operand.as_ref()
                {
                    // Extract receiver (e.g., "me")
                    // Phase 259 P0: Support both Variable and Me node
                    // Phase 269 P1.2: Skip This in static box context (handled by ReceiverNormalizeBox)
                    // IMPORTANT: Me is registered as "me" in variable_map (not "this")
                    let receiver = match object.as_ref() {
                        ASTNode::Variable { name, .. } => name.clone(),
                        ASTNode::Me { .. } => "me".to_string(), // Me is registered as "me" in MirBuilder
                        ASTNode::This { .. } => {
                            // Phase 269 P1.2: Skip pattern for This in static box context
                            // Let ReceiverNormalizeBox handle static call normalization instead
                            continue;
                        }
                        _ => continue,
                    };

                    // P0: Expect 1 argument: s.substring(i, i + 1)
                    if arguments.len() != 1 {
                        continue;
                    }

                    // Validate argument is substring call
                    if let ASTNode::MethodCall {
                        object: substr_obj,
                        method: substr_method,
                        arguments: substr_args,
                        ..
                    } = &arguments[0]
                    {
                        if substr_method != "substring" {
                            continue;
                        }

                        // Object must be haystack
                        if let ASTNode::Variable { name, .. } = substr_obj.as_ref() {
                            if name != &haystack {
                                continue;
                            }
                        } else {
                            continue;
                        }

                        // Args: (i, i + 1)
                        if substr_args.len() != 2 {
                            continue;
                        }

                        // Arg 0: loop_var
                        match &substr_args[0] {
                            ASTNode::Variable { name, .. } if name == &loop_var => {}
                            _ => continue,
                        }

                        // Arg 1: loop_var + 1
                        match &substr_args[1] {
                            ASTNode::BinaryOp {
                                operator: BinaryOperator::Add,
                                left,
                                right,
                                ..
                            } => {
                                // Left: loop_var
                                match left.as_ref() {
                                    ASTNode::Variable { name, .. } if name == &loop_var => {}
                                    _ => continue,
                                }

                                // Right: Literal(1)
                                match right.as_ref() {
                                    ASTNode::Literal {
                                        value: LiteralValue::Integer(1),
                                        ..
                                    } => {}
                                    _ => continue,
                                }
                            }
                            _ => continue,
                        }
                    } else {
                        continue;
                    }

                    // Check then_body contains: return false
                    if then_body.len() == 1 {
                        if let ASTNode::Return { value, .. } = &then_body[0] {
                            if let Some(ret_val) = value {
                                if let ASTNode::Literal {
                                    value: LiteralValue::Bool(false),
                                    ..
                                } = ret_val.as_ref()
                                {
                                    predicate_receiver_opt = Some(receiver);
                                    predicate_method_opt = Some(method.clone());
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let predicate_receiver = predicate_receiver_opt.ok_or_else(|| "No predicate pattern found")?;
    let predicate_method = predicate_method_opt.ok_or_else(|| "No predicate method found")?;

    // 3. Check for step: i = i + 1
    let mut step_lit_opt = None;
    for stmt in body {
        if let ASTNode::Assignment { target, value, .. } = stmt {
            if let ASTNode::Variable { name: target_name, .. } = target.as_ref() {
                if target_name == &loop_var {
                    if let ASTNode::BinaryOp {
                        operator: BinaryOperator::Add,
                        left,
                        right,
                        ..
                    } = value.as_ref()
                    {
                        if let ASTNode::Variable { name: left_name, .. } = left.as_ref() {
                            if left_name == &loop_var {
                                if let ASTNode::Literal {
                                    value: LiteralValue::Integer(lit),
                                    ..
                                } = right.as_ref()
                                {
                                    step_lit_opt = Some(*lit);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let step_lit = step_lit_opt.ok_or_else(|| "No step pattern found")?;

    // P0: Step must be 1
    if step_lit != 1 {
        return Ok(None);
    }

    Ok(Some(BoolPredicateScanParts {
        loop_var,
        haystack,
        predicate_receiver,
        predicate_method,
        step_lit,
    }))
}

/// Phase 269 P1: Lowering function for Pattern 8 (Frag-based)
/// Now uses EdgeCFG Frag API via emission entrypoint
pub(crate) fn lower_bool_predicate_scan(
    builder: &mut MirBuilder,
    condition: &ASTNode,
    body: &[ASTNode],
    func_name: &str,
    debug: bool,
) -> Result<Option<ValueId>, String> {
    use crate::mir::definitions::call_unified::TypeCertainty;
    use crate::mir::types::{BinaryOp, CompareOp, ConstValue, UnaryOp};
    use crate::mir::{Effect, EffectMask, MirInstruction, MirType};

    let trace = trace::trace();

    // Step 1: Extract pattern parts (SSOT)
    let parts = extract_bool_predicate_scan_parts(condition, body)?
        .ok_or_else(|| format!("[pattern8] Not a boolean predicate scan pattern in {}", func_name))?;

    if debug {
        trace.debug(
            "pattern8/lower",
            &format!(
                "Pattern8 Frag lowering: loop_var={}, haystack={}, predicate={}.{}",
                parts.loop_var, parts.haystack, parts.predicate_receiver, parts.predicate_method
            ),
        );
    }

    // Step 2: Get me_val using build_me_expression() (SSOT - no receiver name speculation)
    let me_val = crate::mir::builder::stmts::variable_stmt::build_me_expression(builder)?;

    // Step 3: Get i and s from variable_map
    let i_init_val = builder
        .variable_ctx
        .variable_map
        .get(&parts.loop_var)
        .copied()
        .ok_or_else(|| format!("[pattern8] Variable '{}' not found", parts.loop_var))?;

    let s_val = builder
        .variable_ctx
        .variable_map
        .get(&parts.haystack)
        .copied()
        .ok_or_else(|| format!("[pattern8] Variable '{}' not found", parts.haystack))?;

    // Step 4a: Capture preheader block (entry to loop) for PHI input
    let preheader_bb = builder
        .current_block
        .ok_or_else(|| "[pattern8] No current block for loop entry".to_string())?;

    // Step 4b: Allocate PHI destination for loop variable BEFORE generating blocks
    let i_current = builder.next_value_id();
    builder.type_ctx.set_type(i_current, MirType::Integer);

    // Step 4c: Allocate BasicBlockIds for 5 blocks
    let header_bb = builder.next_block_id();
    let body_bb = builder.next_block_id();
    let step_bb = builder.next_block_id();
    let after_bb = builder.next_block_id();
    let ret_false_bb = builder.next_block_id();

    // Add Jump from current block to header_bb (to terminate the previous block)
    if let Some(_current) = builder.current_block {
        builder.emit_instruction(MirInstruction::Jump {
            target: header_bb,
            edge_args: None,
        })?;
    }

    // Build header_bb: len = s.length(), cond_loop = (i < len)
    builder.start_new_block(header_bb)?;

    // Note: PHI node for i_current will be inserted AFTER all blocks are generated
    // (see Step 4 below, after step_bb generates i_next_val)

    let len_val = builder.next_value_id();
    builder.emit_instruction(crate::mir::ssot::method_call::runtime_method_call(
        Some(len_val),
        s_val,
        "RuntimeDataBox",
        "length",
        vec![],
        EffectMask::PURE.add(Effect::Io),
        TypeCertainty::Union,
    ))?;
    builder.type_ctx.set_type(len_val, MirType::Integer);

    let cond_loop = builder.next_value_id();
    builder.emit_instruction(MirInstruction::Compare {
        dst: cond_loop,
        lhs: i_current, // Use PHI result, not initial value
        op: CompareOp::Lt,
        rhs: len_val,
    })?;
    builder.type_ctx.set_type(cond_loop, MirType::Bool);

    // Create ret_false_val in header_bb (dominates both step_bb and ret_false_bb)
    let ret_false_val = builder.next_value_id();
    builder.emit_instruction(MirInstruction::Const {
        dst: ret_false_val,
        value: ConstValue::Bool(false),
    })?;
    builder.type_ctx.set_type(ret_false_val, MirType::Bool);

    // Build body_bb: ch = s.substring(i, i+1), ok = me.<predicate_method>(ch), cond_fail = not ok
    builder.start_new_block(body_bb)?;

    let one = builder.next_value_id();
    builder.emit_instruction(MirInstruction::Const {
        dst: one,
        value: ConstValue::Integer(1),
    })?;
    builder.type_ctx.set_type(one, MirType::Integer);

    let i_plus_one = builder.next_value_id();
    builder.emit_instruction(MirInstruction::BinOp {
        dst: i_plus_one,
        lhs: i_current, // Use PHI result, not initial value
        op: BinaryOp::Add,
        rhs: one,
    })?;
    builder.type_ctx.set_type(i_plus_one, MirType::Integer);

    let ch_val = builder.next_value_id();
    builder.emit_instruction(crate::mir::ssot::method_call::runtime_method_call(
        Some(ch_val),
        s_val,
        "RuntimeDataBox",
        "substring",
        vec![i_current, i_plus_one], // Use PHI result, not initial value
        EffectMask::PURE.add(Effect::Io),
        TypeCertainty::Union,
    ))?;
    builder.type_ctx.set_type(ch_val, MirType::String);

    let ok_val = builder.next_value_id();
    builder.emit_instruction(crate::mir::ssot::method_call::runtime_method_call(
        Some(ok_val),
        me_val,
        "RuntimeDataBox",
        parts.predicate_method.clone(),
        vec![ch_val],
        EffectMask::PURE.add(Effect::Io),
        TypeCertainty::Union,
    ))?;
    builder.type_ctx.set_type(ok_val, MirType::Bool);

    let cond_fail = builder.next_value_id();
    builder.emit_instruction(MirInstruction::UnaryOp {
        dst: cond_fail,
        op: UnaryOp::Not,
        operand: ok_val,
    })?;
    builder.type_ctx.set_type(cond_fail, MirType::Bool);

    // Build step_bb: i = i + 1
    builder.start_new_block(step_bb)?;

    let i_next_val = builder.next_value_id();
    builder.emit_instruction(MirInstruction::BinOp {
        dst: i_next_val,
        lhs: i_current, // Use PHI result, not initial value
        op: BinaryOp::Add,
        rhs: one,
    })?;
    builder.type_ctx.set_type(i_next_val, MirType::Integer);
    // Note: Do NOT update variable_map here - PHI will handle SSA renaming

    // Ensure ret_false_bb and after_bb exist (they don't have instructions, but must exist for emit_frag)
    builder.ensure_block_exists(ret_false_bb)?;
    builder.ensure_block_exists(after_bb)?;

    // Step 4: Insert PHI at head of header_bb - Phase 272 P0.2 Refactoring: use emission/phi.rs
    use crate::mir::builder::emission::phi::insert_loop_phi;

    insert_loop_phi(
        builder,
        header_bb,
        i_current,
        vec![
            (preheader_bb, i_init_val), // Entry edge: initial value
            (step_bb, i_next_val),      // Latch edge: updated value
        ],
        "pattern8/header_phi",
    )?;

    // Step 5: Call emission entrypoint
    use crate::mir::builder::emission::loop_predicate_scan::emit_bool_predicate_scan_edgecfg;

    if crate::config::env::is_joinir_debug() {
        let ring0 = crate::runtime::get_global_ring0();
        ring0.log.debug(&format!("[pattern8] using edgecfg (Frag版)"));
    }

    emit_bool_predicate_scan_edgecfg(
        builder,
        header_bb,
        body_bb,
        step_bb,
        after_bb,
        ret_false_bb,
        cond_loop,
        cond_fail,
        ret_false_val,
    )?;

    // Step 6: Update variable_map to use final loop variable value
    // (This is the value when loop exits normally via i >= len)
    builder
        .variable_ctx
        .variable_map
        .insert(parts.loop_var.clone(), i_current);

    // Step 7: Setup after_bb for subsequent AST lowering (return true)
    // CRITICAL: Use start_new_block() to create actual block, not just set current_block
    builder.start_new_block(after_bb)?;

    // Step 7: Return Void (loop as statement, not expression)
    use crate::mir::builder::emission::constant::emit_void;
    let void_val = emit_void(builder)?;

    if debug {
        trace.debug("pattern8/lower", "Pattern8 Frag lowering complete");
    }

    Ok(Some(void_val))
}

/// Phase 269 P1: Detection for Pattern 8 (BoolPredicateScan)
/// Now uses EdgeCFG Frag lowering via emission entrypoint
/// Phase 269 P1.2: Skip Pattern8 for static box this.method() - let ReceiverNormalizeBox handle
pub(crate) fn can_lower(
    builder: &MirBuilder,
    ctx: &crate::mir::builder::control_flow::joinir::patterns::router::LoopPatternContext,
) -> bool {
    // Phase 269 P1.2: Skip Pattern8 for static box contexts
    // this.method() in static boxes should be normalized to static calls by ReceiverNormalizeBox
    // Pattern8 requires a receiver object, which doesn't exist for static box methods
    if builder.comp_ctx.current_static_box.is_some() {
        if ctx.debug {
            trace::trace().debug(
                "pattern8/can_lower",
                "reject: static box context (ReceiverNormalizeBox handles this.method())",
            );
        }
        return false;
    }

    match extract_bool_predicate_scan_parts(ctx.condition, ctx.body) {
        Ok(Some(_)) => {
            if ctx.debug {
                trace::trace().debug(
                    "pattern8/can_lower",
                    "accept: boolean predicate scan pattern extractable",
                );
            }
            true
        }
        Ok(None) => {
            if ctx.debug {
                trace::trace().debug(
                    "pattern8/can_lower",
                    "reject: not a boolean predicate scan pattern",
                );
            }
            false
        }
        Err(e) => {
            if ctx.debug {
                trace::trace().debug(
                    "pattern8/can_lower",
                    &format!("reject: extraction error: {}", e),
                );
            }
            false
        }
    }
}

/// Phase 269 P1: Lowering function for Pattern 8 (Frag-based)
/// Now uses EdgeCFG Frag API via emission entrypoint
pub(crate) fn lower(
    builder: &mut MirBuilder,
    ctx: &crate::mir::builder::control_flow::joinir::patterns::router::LoopPatternContext,
) -> Result<Option<ValueId>, String> {
    lower_bool_predicate_scan(
        builder,
        ctx.condition,
        ctx.body,
        ctx.func_name,
        ctx.debug,
    )
}

/// Phase 269 P0: Pattern8 Frag 版 lowerer（test-only）
#[cfg(test)]
pub(crate) fn lower_pattern8_frag(
    _builder: &mut MirBuilder,
    _join_module: crate::mir::join_ir::JoinModule,
    _boundary: &crate::mir::join_ir::lowering::inline_boundary::JoinInlineBoundary,
    _debug: bool,
) -> Result<Option<ValueId>, String> {
    Err("[pattern8_frag] Phase 269 P0 stub - not yet implemented".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Phase 269 P0: Test Pattern8 Frag lowering (minimal test)
    ///
    /// Verifies that lower_pattern8_frag() can:
    /// - Accept JoinModule from lower_scan_bool_predicate_minimal()
    /// - Return appropriate error (stub implementation)
    #[test]
    fn test_pattern8_frag_lowering_stub() {
        use crate::mir::builder::MirBuilder;
        use crate::mir::join_ir::lowering::join_value_space::JoinValueSpace;
        use crate::mir::join_ir::lowering::scan_bool_predicate_minimal::lower_scan_bool_predicate_minimal;
        use crate::mir::join_ir::lowering::JoinInlineBoundaryBuilder;

        // 1. Create minimal JoinModule
        let mut join_value_space = JoinValueSpace::new();
        let join_module = lower_scan_bool_predicate_minimal(&mut join_value_space, "me", "is_digit");

        // 2. Create minimal boundary (stub)
        let boundary = JoinInlineBoundaryBuilder::new()
            .with_inputs(vec![], vec![])
            .build();

        // 3. Create MirBuilder
        let mut builder = MirBuilder::new();

        // 4. Call lower_pattern8_frag (should return stub error)
        let result = lower_pattern8_frag(&mut builder, join_module, &boundary, false);

        // 5. Verify stub error
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("stub"));
    }
}
