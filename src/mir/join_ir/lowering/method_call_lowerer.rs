//! Phase 224-B: MethodCall Lowering Box
//!
//! This box provides metadata-driven lowering of MethodCall AST nodes to JoinIR.
//!
//! ## Design Philosophy
//!
//! **Box-First Design**: MethodCallLowerer is a single-responsibility box that
//! answers one question: "Can this MethodCall be lowered to JoinIR, and if so, how?"
//!
//! **Metadata-Driven**: Uses CoreMethodId metadata exclusively - NO method name hardcoding.
//! All decisions based on `is_pure()`, `allowed_in_condition()`, `allowed_in_init()`.
//!
//! **Fail-Fast**: If a method is not whitelisted, immediately returns Err.
//! No silent fallbacks or guessing.
//!
//! ## Supported Contexts
//!
//! - **Condition context**: Methods allowed in loop conditions (e.g., `s.length()`)
//! - **Init context**: Methods allowed in LoopBodyLocal initialization (e.g., `s.substring(0, 1)`)
//!
//! ## Example Usage
//!
//! ```ignore
//! // Loop condition: loop(i < s.length())
//! let recv_val = ValueId(0); // 's'
//! let result = MethodCallLowerer::lower_for_condition(
//!     recv_val,
//!     "length",
//!     &[],
//!     &mut alloc_value,
//!     &mut instructions,
//! )?;
//! // Result: BoxCall instruction emitted, returns result ValueId
//! ```

use crate::ast::ASTNode;
use crate::mir::join_ir::{JoinInst, MirLikeInst};
use crate::mir::ValueId;
use crate::runtime::core_box_ids::CoreMethodId;
use crate::runtime::core_method_aliases::canonical_method_name;

fn resolve_core_method_id(method_name: &str, arg_len: usize) -> Result<CoreMethodId, Vec<usize>> {
    CoreMethodId::resolve_by_name_and_arity(method_name, arg_len)
}

fn format_expected_arities(expected: &[usize]) -> String {
    let mut list = expected.to_vec();
    list.sort_unstable();
    list.dedup();
    if list.len() == 1 {
        list[0].to_string()
    } else {
        format!("{:?}", list)
    }
}

use super::condition_env::ConditionEnv;
use super::debug_output_box::DebugOutputBox;
use super::loop_body_local_env::LoopBodyLocalEnv;

/// Box: resolves method call arguments with cascading lookup (body-local → condition).
struct CascadingArgResolver<'a> {
    cond_env: &'a ConditionEnv,
    body_local_env: &'a LoopBodyLocalEnv,
    debug: DebugOutputBox,
}

impl<'a> CascadingArgResolver<'a> {
    fn new(cond_env: &'a ConditionEnv, body_local_env: &'a LoopBodyLocalEnv) -> Self {
        Self {
            cond_env,
            body_local_env,
            debug: DebugOutputBox::new_dev("method_call_lowerer"),
        }
    }

    fn resolve(
        &self,
        expr: &ASTNode,
        alloc_value: &mut dyn FnMut() -> ValueId,
        instructions: &mut Vec<JoinInst>,
    ) -> Result<ValueId, String> {
        match expr {
            // Variables - check body_local_env first, then cond_env
            ASTNode::Variable { name, .. } => {
                if let Some(vid) = self.body_local_env.get(name) {
                    self.debug.log_if_enabled(|| {
                        format!("Arg '{}' found in LoopBodyLocalEnv → {:?}", name, vid)
                    });
                    Ok(vid)
                } else if let Some(vid) = self.cond_env.get(name) {
                    self.debug.log_if_enabled(|| {
                        format!("Arg '{}' found in ConditionEnv → {:?}", name, vid)
                    });
                    Ok(vid)
                } else {
                    Err(format!(
                        "Variable '{}' not found in LoopBodyLocalEnv or ConditionEnv",
                        name
                    ))
                }
            }
            // Non-variables delegate to value expression lowering (body-local not needed)
            _ => super::condition_lowerer::lower_value_expression(
                expr,
                alloc_value,
                self.cond_env,
                None, // body-local not used for generic expressions
                None, // Phase 252: No static box context for argument lowering
                instructions,
            ),
        }
    }
}

/// Phase 224-B: MethodCall Lowerer Box
///
/// Provides metadata-driven lowering of MethodCall AST nodes to JoinIR instructions.
pub struct MethodCallLowerer;

impl MethodCallLowerer {
    /// Lower a MethodCall for use in loop condition expressions
    ///
    /// # Arguments
    ///
    /// * `recv_val` - Receiver ValueId (already lowered)
    /// * `method_name` - Method name from AST (e.g., "length")
    /// * `args` - Argument AST nodes (not yet supported in P0)
    /// * `alloc_value` - ValueId allocator function
    /// * `instructions` - Instruction buffer to append to
    ///
    /// # Returns
    ///
    /// * `Ok(ValueId)` - Result of method call
    /// * `Err(String)` - If method not found or not allowed in condition
    ///
    /// # Phase 224-C: Argument Support
    ///
    /// - Supports zero-argument methods (e.g., `s.length()`)
    /// - Supports methods with arguments (e.g., `s.substring(0, 5)`, `s.indexOf("x")`)
    /// - Only whitelisted methods (StringLength, ArrayLength, StringIndexOf, etc.)
    /// - Arity is checked against CoreMethodId metadata
    ///
    /// # Example
    ///
    /// ```ignore
    /// // Loop condition: loop(i < s.length())
    /// let recv_val = env.get("s").unwrap();
    /// let result = MethodCallLowerer::lower_for_condition(
    ///     recv_val,
    ///     "length",
    ///     &[],
    ///     &mut alloc_value,
    ///     &mut instructions,
    /// )?;
    /// ```
    pub fn lower_for_condition(
        recv_val: ValueId,
        method_name: &str,
        args: &[ASTNode],
        alloc_value: &mut dyn FnMut() -> ValueId,
        env: &ConditionEnv,
        instructions: &mut Vec<JoinInst>,
    ) -> Result<ValueId, String> {
        // Resolve method name + arity to CoreMethodId
        // Note: We don't know receiver type at this point, so we try all methods
        let canonical_name = canonical_method_name(method_name);
        let method_id = match resolve_core_method_id(canonical_name, args.len()) {
            Ok(id) => id,
            Err(expected) if expected.is_empty() => {
                return Err(format!(
                    "MethodCall not recognized as CoreMethodId: {}.{}()",
                    recv_val.0, method_name
                ));
            }
            Err(expected) => {
                return Err(format!(
                    "Arity mismatch: {}.{}() expects {} args, got {}",
                    recv_val.0,
                    method_name,
                    format_expected_arities(&expected),
                    args.len()
                ));
            }
        };

        // Check if allowed in condition context
        if !method_id.allowed_in_condition() {
            return Err(format!(
                "MethodCall not allowed in loop condition: {}.{}() (not whitelisted)",
                recv_val.0, method_name
            ));
        }

        // Phase 224-C: Check arity
        let expected_arity = method_id.arity();
        if args.len() != expected_arity {
            return Err(format!(
                "Arity mismatch: {}.{}() expects {} args, got {}",
                recv_val.0,
                method_name,
                expected_arity,
                args.len()
            ));
        }

        // Phase 224-C: Lower arguments using condition lowerer
        let mut lowered_args = Vec::new();
        for arg_ast in args {
            let arg_val = super::condition_lowerer::lower_value_expression(
                arg_ast,
                alloc_value,
                env,
                None, // Phase 92 P2-2: No body-local for method call args
                None, // Phase 252: No static box context for method call args
                instructions,
            )?;
            lowered_args.push(arg_val);
        }

        // Emit BoxCall instruction
        let dst = alloc_value();
        let box_name = method_id.box_id().name().to_string();

        // Build complete args: receiver + method args
        let mut full_args = vec![recv_val];
        full_args.extend(lowered_args);

        instructions.push(JoinInst::Compute(MirLikeInst::BoxCall {
            dst: Some(dst),
            box_name,
            method: canonical_name.to_string(),
            args: full_args,
        }));

        Ok(dst)
    }

    /// Lower a MethodCall for use in LoopBodyLocal initialization
    ///
    /// Similar to `lower_for_condition` but uses `allowed_in_init()` whitelist.
    /// More permissive - allows methods like `substring`, `indexOf`, etc.
    ///
    /// # Phase 224-C: Argument Support
    ///
    /// - Supports zero-argument methods
    /// - Supports methods with arguments (e.g., `substring(0, 5)`, `indexOf(ch)`)
    /// - Arity is checked against CoreMethodId metadata
    ///
    /// # Phase 226: Cascading LoopBodyLocal Support
    ///
    /// - Arguments can reference previously defined body-local variables
    /// - Checks `body_local_env` first, then `cond_env` for variable resolution
    /// - Example: `local digit_pos = digits.indexOf(ch)` where `ch` is body-local
    pub fn lower_for_init(
        recv_val: ValueId,
        method_name: &str,
        args: &[ASTNode],
        alloc_value: &mut dyn FnMut() -> ValueId,
        cond_env: &ConditionEnv,
        body_local_env: &LoopBodyLocalEnv,
        instructions: &mut Vec<JoinInst>,
    ) -> Result<ValueId, String> {
        // Resolve method name + arity to CoreMethodId
        let canonical_name = canonical_method_name(method_name);
        let method_id = match resolve_core_method_id(canonical_name, args.len()) {
            Ok(id) => id,
            Err(expected) if expected.is_empty() => {
                return Err(format!(
                    "MethodCall not recognized as CoreMethodId: {}.{}()",
                    recv_val.0, method_name
                ));
            }
            Err(expected) => {
                return Err(format!(
                    "Arity mismatch: {}.{}() expects {} args, got {}",
                    recv_val.0,
                    method_name,
                    format_expected_arities(&expected),
                    args.len()
                ));
            }
        };

        // Check if allowed in init context
        if !method_id.allowed_in_init() {
            return Err(format!(
                "MethodCall not allowed in LoopBodyLocal init: {}.{}() (not whitelisted)",
                recv_val.0, method_name
            ));
        }

        // Phase 224-C: Check arity
        let expected_arity = method_id.arity();
        if args.len() != expected_arity {
            return Err(format!(
                "Arity mismatch: {}.{}() expects {} args, got {}",
                recv_val.0,
                method_name,
                expected_arity,
                args.len()
            ));
        }

        // Phase 226: Lower arguments with cascading LoopBodyLocal support
        // Check body_local_env first, then cond_env
        let resolver = CascadingArgResolver::new(cond_env, body_local_env);
        let mut lowered_args = Vec::new();
        for arg_ast in args {
            let arg_val = resolver.resolve(arg_ast, alloc_value, instructions)?;
            lowered_args.push(arg_val);
        }

        // Emit BoxCall instruction
        let dst = alloc_value();
        let box_name = method_id.box_id().name().to_string();

        // Build complete args: receiver + method args
        let mut full_args = vec![recv_val];
        full_args.extend(lowered_args);

        instructions.push(JoinInst::Compute(MirLikeInst::BoxCall {
            dst: Some(dst),
            box_name,
            method: canonical_name.to_string(),
            args: full_args,
        }));

        Ok(dst)
    }
}

#[cfg(test)]
mod tests;
