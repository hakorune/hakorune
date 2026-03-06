//! LoopSimpleWhile route lowering
//!
//! Target: Simple while loops with no break/continue
//! Example: `while(i < 10) { i = i + 1 }`
//!
//! # Transformation
//!
//! ```text
//! fn loop_step(i):
//!   exit_cond = !(i < 3)
//!   Jump(k_exit, [], cond=exit_cond)
//!   print(i)
//!   i_next = i + 1
//!   Call(loop_step, [i_next])
//! ```

use crate::mir::join_ir::lowering::loop_to_join::LoopToJoinLowerer;
use crate::mir::join_ir::JoinInst;
use crate::mir::loop_form::LoopForm;

/// Lowering for LoopSimpleWhile route
///
/// # Transformation (Pseudocode from design.md)
///
/// ```text
/// fn loop_step(i):
///   exit_cond = !(i < 3)
///   Jump(k_exit, [], cond=exit_cond)
///   print(i)
///   i_next = i + 1
///   Call(loop_step, [i_next])
/// ```
///
/// # Steps (from design.md § LoopSimpleWhile)
///
/// 1. **Extract Loop Variables (Carriers)**
///    - Analyze header PHI nodes
///    - Identify initial values and next values
///
/// 2. **Create loop_step Function Signature**
///    - Parameters: carrier variables + k_exit continuation
///
/// 3. **Create k_exit Continuation**
///    - Handles loop exit (returns final value)
///
/// 4. **Generate Exit Condition Check**
///    - Negate loop condition: `exit_cond = !(i < 3)`
///    - Add Jump to k_exit if exit_cond is true
///
/// 5. **Translate Loop Body**
///    - Copy body instructions to loop_step body
///    - Preserve side effects (print, etc.)
///
/// 6. **Generate Tail Recursion**
///    - Update carrier: `i_next = i + 1`
///    - Tail call: `Call(loop_step, [i_next], k_next: None)`
///
/// 7. **Wire Exit Continuation**
///    - Connect loop_step exit Jump to k_exit
///    - Set k_exit exit_cont to parent continuation
///
/// # Arguments
///
/// * `loop_form` - The loop structure to lower
/// * `lowerer` - The LoopToJoinLowerer builder (provides ValueId allocation, etc.)
///
/// # Returns
///
/// * `Some(JoinInst)` - Lowering succeeded, returns generated JoinIR instruction
/// * `None` - Lowering failed (route shape not matched or unsupported)
///
/// # Errors
///
/// Returns `None` if:
/// - Loop has breaks or continues
/// - Loop has multiple latches
/// - Loop condition is too complex
///
/// # Reference
///
/// See design.md § LoopSimpleWhile for full pseudocode.
///
/// # Example Usage
///
/// ```rust,ignore
/// use crate::mir::loop_pattern_detection::is_loop_simple_while_route;
///
/// if is_loop_simple_while_route(&loop_form) {
///     lower_simple_while_to_joinir(&loop_form, &mut lowerer)?;
/// }
/// ```
pub fn lower_simple_while_to_joinir(
    _loop_form: &LoopForm,
    _lowerer: &mut LoopToJoinLowerer,
) -> Option<JoinInst> {
    // TODO: Implement LoopSimpleWhile route lowering
    //
    // Step 1: Extract Loop Variables (Carriers)
    // ==========================================
    // From header PHI: %2 = phi [%1, bb1], [%6, bb4]
    // Extract: (var_name: "i", init_value: ValueId(1), next_value: ValueId(6))
    //
    // ```rust
    // let carriers = extract_carriers_from_header_phi(loop_form)?;
    // ```
    //
    // Step 2: Create loop_step Function Signature
    // ============================================
    // Signature: fn loop_step(i: ValueId, k_exit: JoinContId) -> ...
    //
    // ```rust
    // let loop_step_id = lowerer.allocate_join_func_id();
    // let k_exit_id = lowerer.allocate_join_func_id();
    //
    // let mut loop_step_params = vec![];
    // for carrier in &carriers {
    //     loop_step_params.push(carrier.param_valueid);
    // }
    // ```
    //
    // Step 3: Create k_exit Continuation
    // ===================================
    // fn k_exit() -> ValueId  // Returns final value (0 in this case)
    //
    // ```rust
    // let k_exit_func = JoinFunction {
    //     id: k_exit_id,
    //     name: "k_exit".to_string(),
    //     params: vec![],  // No exit values in LoopSimpleWhile route
    //     body: vec![
    //         JoinInst::Compute(MirLikeInst::Const {
    //             dst: lowerer.fresh_valueid(),
    //             value: ConstValue::Integer(0),
    //         }),
    //         JoinInst::Ret { value: Some(return_val) },
    //     ],
    //     exit_cont: None,
    // };
    // lowerer.register_join_function(k_exit_func);
    // ```
    //
    // Step 4: Generate Exit Condition Check
    // ======================================
    // exit_cond = !(i < 3)
    // Jump(k_exit, [], cond=exit_cond)
    //
    // ```rust
    // let loop_cond = extract_loop_condition_from_header(loop_form)?;
    // let exit_cond = lowerer.fresh_valueid();
    //
    // body.push(JoinInst::Compute(MirLikeInst::UnaryOp {
    //     dst: exit_cond,
    //     op: UnaryOp::Not,
    //     operand: loop_cond,
    // }));
    //
    // body.push(JoinInst::Jump {
    //     cont: k_exit_id.as_cont(),
    //     args: vec![],
    //     cond: Some(exit_cond),
    // });
    // ```
    //
    // Step 5: Translate Loop Body
    // ===========================
    // Copy body instructions to loop_step body
    //
    // ```rust
    // let body_insts = extract_body_instructions(loop_form)?;
    // for inst in body_insts {
    //     body.push(translate_mir_inst_to_joinir(inst, lowerer)?);
    // }
    // ```
    //
    // Step 6: Generate Tail Recursion
    // ================================
    // i_next = i + 1
    // Call(loop_step, [i_next], k_next: None)
    //
    // ```rust
    // let const_1 = lowerer.fresh_valueid();
    // let i_next = lowerer.fresh_valueid();
    //
    // body.push(JoinInst::Compute(MirLikeInst::Const {
    //     dst: const_1,
    //     value: ConstValue::Integer(1),
    // }));
    //
    // body.push(JoinInst::Compute(MirLikeInst::BinOp {
    //     dst: i_next,
    //     op: BinOp::Add,
    //     lhs: i,
    //     rhs: const_1,
    // }));
    //
    // body.push(JoinInst::Call {
    //     func: loop_step_id,
    //     args: vec![i_next],
    //     k_next: None,  // CRITICAL: Must be None (tail call)
    //     dst: Some(result_var),
    // });
    // ```
    //
    // Step 7: Wire Exit Continuation
    // ===============================
    // Connect loop_step to k_exit
    //
    // ```rust
    // let loop_step_func = JoinFunction {
    //     id: loop_step_id,
    //     name: "loop_step".to_string(),
    //     params: loop_step_params,
    //     body: body,
    //     exit_cont: Some(k_exit_id.as_cont()),
    // };
    // lowerer.register_join_function(loop_step_func);
    // ```
    //
    // Return success
    // ```rust
    // Some(JoinInst::Call { ... })
    // ```

    None
}
