//! Phase 186: Loop Body-Local Variable Initialization Lowerer
//!
//! This module lowers body-local variable initialization expressions to JoinIR.
//! It handles expressions like `local digit_pos = pos - start` by converting
//! them to JoinIR instructions and storing the result ValueId in LoopBodyLocalEnv.
//!
//! ## Design Philosophy
//!
//! **Single Responsibility**: This module ONLY handles body-local init lowering.
//! It does NOT:
//! - Store variables (that's LoopBodyLocalEnv)
//! - Resolve variable priority (that's ScopeManager / ConditionEnv)
//! - Emit update instructions (that's CarrierUpdateEmitter)
//!
//! ## Box-First Design
//!
//! Following 箱理論 (Box Theory) principles:
//! - **Single purpose**: Lower init expressions to JoinIR
//! - **Clear boundaries**: Only init expressions, not updates
//! - **Fail-Fast**: Unsupported expressions → explicit error
//! - **Deterministic**: Processes variables in declaration order
//!
//! ## Scope
//!
//! **Supported init expressions**:
//! - Binary operations: `+`, `-`, `*`, `/` (Phase 186)
//! - Constant literals: `42`, `"hello"` (Phase 186, 193)
//! - Variable references: `pos`, `start`, `i` (Phase 186)
//! - Method calls: `s.substring(...)`, `digits.indexOf(ch)` (Phase 193, 225)
//!   - Uses metadata-driven whitelist via `CoreMethodId::allowed_in_init()`
//!   - Delegates to `MethodCallLowerer` for consistent lowering
//!
//! **NOT supported** (Fail-Fast):
//! - Complex expressions: nested calls, function calls
//! - User-defined box methods (only CoreMethodId supported)

use crate::ast::ASTNode;
use crate::mir::join_ir::lowering::condition_env::ConditionEnv;
use crate::mir::join_ir::lowering::debug_output_box::DebugOutputBox;
use crate::mir::join_ir::lowering::loop_body_local_env::LoopBodyLocalEnv;
use crate::mir::join_ir::{BinOpKind, ConstValue, JoinInst, MirLikeInst};
use crate::mir::ValueId;

mod method_call_init;

/// Loop body-local variable initialization lowerer
///
/// Lowers initialization expressions for body-local variables declared
/// within loop bodies to JoinIR instructions.
///
/// # Example
///
/// ```nyash
/// loop(pos < 10) {
///     local digit_pos = pos - start  // ← This init expression
///     sum = sum + digit_pos
///     pos = pos + 1
/// }
/// ```
///
/// Lowering process:
/// 1. Find `local digit_pos = pos - start`
/// 2. Lower `pos - start` to JoinIR:
///    - `pos_vid = ConditionEnv.get("pos")`
///    - `start_vid = ConditionEnv.get("start")`
///    - `result_vid = BinOp(Sub, pos_vid, start_vid)`
/// 3. Store in LoopBodyLocalEnv: `digit_pos → result_vid`
pub struct LoopBodyLocalInitLowerer<'a> {
    /// Reference to ConditionEnv for variable resolution
    ///
    /// Init expressions can reference condition variables (e.g., `pos`, `start`)
    /// but cannot reference other body-local variables (forward reference not supported).
    cond_env: &'a ConditionEnv,

    /// Output buffer for JoinIR instructions
    instructions: &'a mut Vec<JoinInst>,

    /// ValueId allocator
    ///
    /// Box<dyn FnMut()> allows using closures that capture environment
    alloc_value: Box<dyn FnMut() -> ValueId + 'a>,

    /// Phase 256.6: Current static box name for me.method() resolution
    ///
    /// When a method call has `me` as receiver, this provides the box name
    /// for resolving user-defined methods (e.g., "StringUtils" for me.index_of()).
    current_static_box_name: Option<String>,
}

impl<'a> LoopBodyLocalInitLowerer<'a> {
    /// Create a new init lowerer
    ///
    /// # Arguments
    ///
    /// * `cond_env` - Condition environment (for resolving init variables)
    /// * `instructions` - Output buffer for JoinIR instructions
    /// * `alloc_value` - ValueId allocator closure
    /// * `current_static_box_name` - Phase 256.6: Box name for me.method() resolution
    pub fn new(
        cond_env: &'a ConditionEnv,
        instructions: &'a mut Vec<JoinInst>,
        alloc_value: Box<dyn FnMut() -> ValueId + 'a>,
        current_static_box_name: Option<String>,
    ) -> Self {
        Self {
            cond_env,
            instructions,
            alloc_value,
            current_static_box_name,
        }
    }

    /// Lower all body-local initializations in loop body
    ///
    /// Scans body AST for local declarations with initialization expressions,
    /// lowers them to JoinIR, and updates LoopBodyLocalEnv with computed ValueIds.
    ///
    /// # Arguments
    ///
    /// * `body_ast` - Loop body AST nodes
    /// * `env` - LoopBodyLocalEnv to update with ValueIds
    ///
    /// # Returns
    ///
    /// * `Ok(())` - All init expressions lowered successfully
    /// * `Err(msg)` - Unsupported expression found (Fail-Fast)
    ///
    /// # Example
    ///
    /// ```ignore
    /// let mut env = LoopBodyLocalEnv::new();
    /// let mut lowerer = LoopBodyLocalInitLowerer::new(...);
    ///
    /// // Lower: local digit_pos = pos - start
    /// lowerer.lower_inits_for_loop(body_ast, &mut env)?;
    ///
    /// // Now env contains: digit_pos → ValueId(X)
    /// assert!(env.get("digit_pos").is_some());
    /// ```
    pub fn lower_inits_for_loop(
        &mut self,
        body_ast: &[ASTNode],
        env: &mut LoopBodyLocalEnv,
    ) -> Result<(), String> {
        let debug = DebugOutputBox::new_dev("loop_body_local_init");
        for node in body_ast {
            if let ASTNode::Local {
                variables,
                initial_values,
                ..
            } = node
            {
                self.lower_single_init(variables, initial_values, env, &debug)?;
            }
        }
        Ok(())
    }

    /// Lower a single local assignment statement
    ///
    /// Handles both single and multiple variable declarations:
    /// - `local temp = i * 2` (single)
    /// - `local a = 1, b = 2` (multiple)
    ///
    /// # Arguments
    ///
    /// * `variables` - List of variable names being declared
    /// * `initial_values` - List of optional initialization expressions (parallel to variables)
    /// * `env` - LoopBodyLocalEnv to update
    fn lower_single_init(
        &mut self,
        variables: &[String],
        initial_values: &[Option<Box<ASTNode>>],
        env: &mut LoopBodyLocalEnv,
        debug: &DebugOutputBox,
    ) -> Result<(), String> {
        // Handle each variable-value pair
        for (var_name, maybe_init_expr) in variables.iter().zip(initial_values.iter()) {
            // Skip if already has JoinIR ValueId (avoid duplicate lowering)
            if env.get(var_name).is_some() {
                debug.log("skip", &format!("'{}' (already has ValueId)", var_name));
                continue;
            }

            // Skip if no initialization expression (e.g., `local temp` without `= ...`)
            let Some(init_expr) = maybe_init_expr else {
                debug.log("skip", &format!("'{}' (no init expression)", var_name));
                continue;
            };

            debug.log_if_enabled(|| format!("lower '{}' = {:?}", var_name, init_expr));

            // Lower init expression to JoinIR
            // Phase 226: Pass env for cascading LoopBodyLocal support
            let value_id = self.lower_init_expr(init_expr, env)?;

            debug.log("store", &format!("'{}' -> {:?}", var_name, value_id));

            // Store in env
            env.insert(var_name.clone(), value_id);
        }
        Ok(())
    }

    /// Lower an initialization expression to JoinIR
    ///
    /// Supported (Phase 186):
    /// - `Integer`: Constant literal (e.g., `42`)
    /// - `Variable`: Condition variable reference (e.g., `pos`)
    /// - `BinOp`: Binary operation (e.g., `pos - start`)
    ///
    /// Phase 226: Cascading LoopBodyLocal support
    /// - `MethodCall`: Method call (e.g., `s.substring(...)`, `digits.indexOf(ch)`)
    /// - Arguments can reference previously defined body-local variables
    ///
    /// Unsupported (Fail-Fast):
    /// - Other complex expressions
    ///
    /// # Arguments
    ///
    /// * `expr` - AST node representing initialization expression
    /// * `env` - LoopBodyLocal environment (for resolving cascading dependencies)
    ///
    /// # Returns
    ///
    /// * `Ok(ValueId)` - JoinIR ValueId of computed result
    /// * `Err(msg)` - Unsupported expression (Fail-Fast)
    fn lower_init_expr(
        &mut self,
        expr: &ASTNode,
        env: &LoopBodyLocalEnv,
    ) -> Result<ValueId, String> {
        let debug = DebugOutputBox::new_dev("loop_body_local_init");
        match expr {
            // Constant literal: 42, 0, 1, "string" (use Literal with value)
            ASTNode::Literal { value, .. } => {
                match value {
                    crate::ast::LiteralValue::Integer(i) => {
                        let vid = (self.alloc_value)();
                        self.instructions.push(JoinInst::Compute(MirLikeInst::Const {
                            dst: vid,
                            value: ConstValue::Integer(*i),
                        }));
                        debug.log("const", &format!("Int({}) -> {:?}", i, vid));
                        Ok(vid)
                    }
                    // Phase 193: String literal support (for method args like "0")
                    crate::ast::LiteralValue::String(s) => {
                        let vid = (self.alloc_value)();
                        self.instructions.push(JoinInst::Compute(MirLikeInst::Const {
                            dst: vid,
                            value: ConstValue::String(s.clone()),
                        }));
                        debug.log("const", &format!("String({:?}) -> {:?}", s, vid));
                        Ok(vid)
                    }
                    _ => Err(format!(
                        "Unsupported literal type in init: {:?} (Phase 193 - only Integer/String supported)",
                        value
                    )),
                }
            }

            // Variable reference: pos, start, i
            ASTNode::Variable { name, .. } => {
                let vid = self.cond_env.get(name).ok_or_else(|| {
                    format!(
                        "Init variable '{}' not found in ConditionEnv (must be condition variable)",
                        name
                    )
                })?;
                debug.log("var", &format!("Variable({}) → {:?}", name, vid));
                Ok(vid)
            }

            // Binary operation: pos - start, i * 2, etc.
            ASTNode::BinaryOp { operator, left, right, .. } => {
                debug.log("binop", &format!("BinaryOp({:?})", operator));

                // Recursively lower operands
                // Phase 226: Pass env for cascading support
                let lhs = self.lower_init_expr(left, env)?;
                let rhs = self.lower_init_expr(right, env)?;

                // Convert operator
                let op_kind = self.convert_binop(operator)?;

                // Emit BinOp instruction
                let result = (self.alloc_value)();
                self.instructions.push(JoinInst::Compute(MirLikeInst::BinOp {
                    dst: result,
                    op: op_kind,
                    lhs,
                    rhs,
                }));

                debug.log(
                    "binop",
                    &format!("BinOp({:?}, {:?}, {:?}) → {:?}", op_kind, lhs, rhs, result),
                );
                Ok(result)
            }

            // Phase 226: MethodCall support with cascading LoopBodyLocalEnv
            ASTNode::MethodCall { object, method, arguments, .. } => {
                method_call_init::emit(
                    object,
                    method,
                    arguments,
                    self.cond_env,
                    env,  // Phase 226: Pass LoopBodyLocalEnv for cascading support
                    self.instructions,
                    &mut self.alloc_value,
                    self.current_static_box_name.as_deref(),  // Phase 256.6
                )
            }
            _ => Err(format!(
                "Unsupported init expression: {:?} (Phase 186 limitation - only int/arithmetic supported)",
                expr
            )),
        }
    }

    /// Convert AST BinaryOperator to JoinIR BinOpKind
    ///
    /// Supported operators: `+`, `-`, `*`, `/`
    ///
    /// # Arguments
    ///
    /// * `op` - AST BinaryOperator enum
    ///
    /// # Returns
    ///
    /// * `Ok(BinOpKind)` - JoinIR operator
    /// * `Err(msg)` - Unsupported operator
    fn convert_binop(&self, op: &crate::ast::BinaryOperator) -> Result<BinOpKind, String> {
        use crate::ast::BinaryOperator;
        match op {
            BinaryOperator::Add => Ok(BinOpKind::Add),
            BinaryOperator::Subtract => Ok(BinOpKind::Sub),
            BinaryOperator::Multiply => Ok(BinOpKind::Mul),
            BinaryOperator::Divide => Ok(BinOpKind::Div),
            _ => Err(format!(
                "Unsupported binary operator in init: {:?} (Phase 186 - only Add/Subtract/Multiply/Divide supported)",
                op
            )),
        }
    }
}

#[cfg(test)]
mod tests;
