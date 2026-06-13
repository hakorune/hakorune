use super::ast_support;
use super::scope_resolution;
use super::{ExprContext, ExprLoweringError};
use crate::ast::ASTNode;
use crate::mir::builder::MirBuilder;
use crate::mir::join_ir::JoinInst;
use crate::mir::ValueId;
use crate::runtime::get_global_ring0;

use super::super::condition_lowerer::lower_condition_to_joinir_no_body_locals;
use super::super::scope_manager::ScopeManager;

/// Phase 231: Expression lowerer (pilot implementation)
///
/// This struct provides a unified interface for lowering AST expressions to
/// JoinIR instructions, using ScopeManager for variable resolution.
///
/// ## Current Scope (Phase 231)
///
/// - **Context**: Condition only (loop/break conditions)
/// - **Supported**: Literals, variables, comparisons (<, >, ==, !=, <=, >=), logical ops (and, or, not)
/// - **Not Supported**: Method calls, NewBox, complex expressions
///
/// ## Usage Pattern
///
/// ```ignore
/// let scope = LoopBreakScopeManager { ... };
/// let mut expr_lowerer = ExprLowerer::new(&scope, ExprContext::Condition, builder);
///
/// match expr_lowerer.lower(&break_condition_ast) {
///     Ok(value_id) => {
///         // Use value_id in JoinIR
///     }
///     Err(ExprLoweringError::UnsupportedNode(_)) => {
///         // Let the caller choose another explicit lowering route.
///     }
///     Err(e) => {
///         // Handle other errors (variable not found, etc.)
///     }
/// }
/// ```
pub struct ExprLowerer<'env, S: ScopeManager> {
    /// Scope manager for variable resolution
    scope: &'env S,

    /// Expression context (Condition vs General)
    context: ExprContext,

    /// Debug flag (inherited from caller)
    debug: bool,

    /// Last lowered instruction sequence (for testing/inspection)
    ///
    /// Phase 235: Tests can inspect this to assert that appropriate Compare / BinOp / Not
    /// instructions are emitted for supported patterns. Productionコードからは未使用。
    last_instructions: Vec<JoinInst>,
}

impl<'env, S: ScopeManager> ExprLowerer<'env, S> {
    /// Create a new expression lowerer
    ///
    /// # Arguments
    ///
    /// * `scope` - ScopeManager for variable resolution
    /// * `context` - Expression context (currently Condition only)
    /// * `_builder` - Retained for caller compatibility; value allocation is not owned here
    pub fn new(scope: &'env S, context: ExprContext, _builder: &mut MirBuilder) -> Self {
        Self {
            scope,
            context,
            debug: false,
            last_instructions: Vec::new(),
        }
    }

    /// Enable debug output
    pub fn with_debug(mut self, debug: bool) -> Self {
        self.debug = debug;
        self
    }

    /// Take the last lowered instruction sequence (mainly for tests)
    ///
    /// Phase 235: This allows unit tests to validate that Compare / BinOp / Not
    /// instructions are present without影響を与えずに ExprLowerer の外から観察できる。
    pub fn take_last_instructions(&mut self) -> Vec<JoinInst> {
        std::mem::take(&mut self.last_instructions)
    }

    /// Lower an expression to JoinIR ValueId
    ///
    /// Phase 231: This is the main entry point. Currently delegates to
    /// lower_condition for Condition context.
    ///
    /// # Returns
    ///
    /// * `Ok(ValueId)` - Expression result ValueId
    /// * `Err(ExprLoweringError)` - Lowering failed (caller can fall back to legacy)
    pub fn lower(&mut self, ast: &ASTNode) -> Result<ValueId, ExprLoweringError> {
        match self.context {
            ExprContext::Condition => self.lower_condition(ast),
        }
    }

    /// Lower a condition expression to JoinIR ValueId
    ///
    /// Phase 231: Thin wrapper around condition_lowerer. The main innovation
    /// is using ScopeManager for variable resolution instead of direct ConditionEnv.
    ///
    /// # Returns
    ///
    /// * `Ok(ValueId)` - Condition result ValueId (boolean)
    /// * `Err(ExprLoweringError)` - Lowering failed
    fn lower_condition(&mut self, ast: &ASTNode) -> Result<ValueId, ExprLoweringError> {
        // 1. Check if AST is supported in condition context
        if !ast_support::is_supported_condition(ast) {
            return Err(ExprLoweringError::UnsupportedNode(format!(
                "Unsupported condition node: {:?}",
                ast
            )));
        }

        let condition_env = scope_resolution::build_condition_env_from_scope(self.scope, ast)?;

        // 3. Delegate to existing condition_lowerer
        // Phase 231: We use the existing, well-tested lowering logic.
        let mut value_counter = 1000u32; // Phase 231: Start high to avoid collisions
        let mut alloc_value = || {
            let id = ValueId(value_counter);
            value_counter += 1;
            id
        };

        let (result_value, instructions) =
            lower_condition_to_joinir_no_body_locals(ast, &mut alloc_value, &condition_env) // Phase 92 P2-2
                .map_err(|e| ExprLoweringError::LoweringError(e))?;

        // Phase 235: 保存しておき、テストから観察できるようにする
        self.last_instructions = instructions;

        if self.debug {
            get_global_ring0().log.debug(&format!(
                "[expr_lowerer/phase231] Lowered condition → ValueId({:?})",
                result_value
            ));
        }

        Ok(result_value)
    }

    /// Public helper used by `loop_break` / `if_phi_join` callers to gate ExprLowerer usage.
    pub fn is_supported_condition(ast: &ASTNode) -> bool {
        ast_support::is_supported_condition(ast)
    }
}
