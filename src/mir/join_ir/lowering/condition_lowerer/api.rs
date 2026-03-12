use crate::ast::ASTNode;
use crate::mir::join_ir::JoinInst;
use crate::mir::ValueId;

use super::super::condition_env::ConditionEnv;
use super::super::loop_body_local_env::LoopBodyLocalEnv;
use super::condition_ops::lower_condition_recursive; // Phase 92 P2-2: Body-local support

/// Lower an AST condition to JoinIR instructions
///
/// # Arguments
///
/// * `cond_ast` - AST node representing the boolean condition
/// * `alloc_value` - ValueId allocator function
/// * `env` - ConditionEnv for variable resolution (JoinIR-local ValueIds)
/// * `body_local_env` - Phase 92 P2-2: Optional body-local variable environment
///
/// # Returns
///
/// * `Ok((ValueId, Vec<JoinInst>))` - Condition result ValueId and evaluation instructions
/// * `Err(String)` - Lowering error message
///
/// # Supported Patterns
///
/// - Comparisons: `i < n`, `x == y`, `a != b`, `x <= y`, `x >= y`, `x > y`
/// - Logical: `a && b`, `a || b`, `!cond`
/// - Variables and literals
///
/// # Phase 92 P2-2: Body-Local Variable Support
///
/// When lowering conditions that reference body-local variables (e.g., `ch == '\\'`
/// in escape patterns), the `body_local_env` parameter provides name → ValueId
/// mappings for variables defined in the loop body.
///
/// Variable resolution priority:
/// 1. ConditionEnv (loop parameters, captured variables)
/// 2. LoopBodyLocalEnv (body-local variables like `ch`)
///
/// # Phase 252: This-Method Support
///
/// When lowering conditions in static box methods (e.g., `StringUtils.trim_end/1`),
/// the `current_static_box_name` parameter enables `this.method(...)` calls to be
/// resolved to the appropriate static box method.
///
/// # Example
///
/// ```ignore
/// let mut env = ConditionEnv::new();
/// env.insert("i".to_string(), ValueId(0));
/// env.insert("end".to_string(), ValueId(1));
///
/// let mut body_env = LoopBodyLocalEnv::new();
/// body_env.insert("ch".to_string(), ValueId(5)); // Phase 92 P2-2
///
/// let mut value_counter = 2u32;
/// let mut alloc_value = || {
///     let id = ValueId(value_counter);
///     value_counter += 1;
///     id
/// };
///
/// // Lower condition: ch == '\\'
/// let (cond_value, cond_insts) = lower_condition_to_joinir(
///     condition_ast,
///     &mut alloc_value,
///     &env,
///     Some(&body_env), // Phase 92 P2-2: Body-local support
///     Some("StringUtils"), // Phase 252: Static box name for this.method
/// )?;
/// ```
pub fn lower_condition_to_joinir(
    cond_ast: &ASTNode,
    alloc_value: &mut dyn FnMut() -> ValueId,
    env: &ConditionEnv,
    body_local_env: Option<&LoopBodyLocalEnv>, // Phase 92 P2-2
    current_static_box_name: Option<&str>,     // Phase 252
) -> Result<(ValueId, Vec<JoinInst>), String> {
    let mut instructions = Vec::new();
    let result_value = lower_condition_recursive(
        cond_ast,
        alloc_value,
        env,
        body_local_env,
        current_static_box_name,
        &mut instructions,
    )?;
    Ok((result_value, instructions))
}

/// Convenience wrapper: lower a condition without body-local or static box support.
pub fn lower_condition_to_joinir_no_body_locals(
    cond_ast: &ASTNode,
    alloc_value: &mut dyn FnMut() -> ValueId,
    env: &ConditionEnv,
) -> Result<(ValueId, Vec<JoinInst>), String> {
    lower_condition_to_joinir(cond_ast, alloc_value, env, None, None)
}
