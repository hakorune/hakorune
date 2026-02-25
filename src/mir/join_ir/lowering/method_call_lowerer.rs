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
use super::loop_body_local_env::LoopBodyLocalEnv;
use super::debug_output_box::DebugOutputBox;

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
                    self.debug
                        .log_if_enabled(|| format!("Arg '{}' found in LoopBodyLocalEnv → {:?}", name, vid));
                    Ok(vid)
                } else if let Some(vid) = self.cond_env.get(name) {
                    self.debug
                        .log_if_enabled(|| format!("Arg '{}' found in ConditionEnv → {:?}", name, vid));
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
mod tests {
    use super::*;
    use crate::ast::Span;
    use crate::mir::join_ir::lowering::loop_body_local_env::LoopBodyLocalEnv;
    use crate::mir::join_ir::JoinInst;

    #[test]
    fn test_resolve_string_length() {
        // Test: "length" → CoreMethodId::StringLength
        let method_id = CoreMethodId::iter().find(|m| m.name() == "length");
        assert!(method_id.is_some());
        assert!(method_id.unwrap().allowed_in_condition());
    }

    #[test]
    fn test_lower_string_length_for_condition() {
        // Test: s.length() in loop condition
        let recv_val = ValueId(10);
        let mut value_counter = 100u32;
        let mut alloc_value = || {
            let id = ValueId(value_counter);
            value_counter += 1;
            id
        };
        let mut instructions = Vec::new();

        // Phase 224-C: Use ConditionEnv
        let env = ConditionEnv::new();

        let result = MethodCallLowerer::lower_for_condition(
            recv_val,
            "length",
            &[],
            &mut alloc_value,
            &env,
            &mut instructions,
        );

        assert!(result.is_ok());
        let result_val = result.unwrap();
        assert_eq!(result_val, ValueId(100));
        assert_eq!(instructions.len(), 1);

        match &instructions[0] {
            JoinInst::Compute(MirLikeInst::BoxCall {
                dst,
                box_name,
                method,
                args,
            }) => {
                assert_eq!(*dst, Some(ValueId(100)));
                assert_eq!(box_name, "StringBox");
                assert_eq!(method, "length");
                assert_eq!(args.len(), 1); // Receiver is first arg
                assert_eq!(args[0], ValueId(10));
            }
            _ => panic!("Expected BoxCall instruction"),
        }
    }

    #[test]
    fn test_not_allowed_in_condition() {
        // Test: s.toUpper() not whitelisted for conditions
        let recv_val = ValueId(10);
        let mut value_counter = 100u32;
        let mut alloc_value = || {
            let id = ValueId(value_counter);
            value_counter += 1;
            id
        };
        let mut instructions = Vec::new();
        let env = ConditionEnv::new();

        let result = MethodCallLowerer::lower_for_condition(
            recv_val,
            "toUpper",
            &[],
            &mut alloc_value,
            &env,
            &mut instructions,
        );

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("not allowed in loop condition"));
    }

    #[test]
    fn test_unknown_method() {
        // Test: s.unknownMethod() not in CoreMethodId
        let recv_val = ValueId(10);
        let mut value_counter = 100u32;
        let mut alloc_value = || {
            let id = ValueId(value_counter);
            value_counter += 1;
            id
        };
        let mut instructions = Vec::new();
        let env = ConditionEnv::new();

        let result = MethodCallLowerer::lower_for_condition(
            recv_val,
            "unknownMethod",
            &[],
            &mut alloc_value,
            &env,
            &mut instructions,
        );

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("not recognized as CoreMethodId"));
    }

    #[test]
    fn test_lower_substring_for_init() {
        // Phase 224-C: substring is not allowed in condition but IS allowed in init
        let recv_val = ValueId(10);
        let i_val = ValueId(11);
        let j_val = ValueId(12);
        let mut value_counter = 100u32;
        let mut alloc_value = || {
            let id = ValueId(value_counter);
            value_counter += 1;
            id
        };
        let mut instructions = Vec::new();

        // Create ConditionEnv with i and j variables
        let mut env = ConditionEnv::new();
        env.insert("i".to_string(), i_val);
        env.insert("j".to_string(), j_val);

        // Create argument ASTs for substring(i, j)
        let arg1_ast = ASTNode::Variable {
            name: "i".to_string(),
            span: crate::ast::Span::unknown(),
        };
        let arg2_ast = ASTNode::Variable {
            name: "j".to_string(),
            span: crate::ast::Span::unknown(),
        };

        // substring is NOT in condition whitelist
        let cond_result = MethodCallLowerer::lower_for_condition(
            recv_val,
            "substring",
            &[arg1_ast.clone(), arg2_ast.clone()],
            &mut alloc_value,
            &env,
            &mut instructions,
        );
        assert!(cond_result.is_err());
        assert!(cond_result
            .unwrap_err()
            .contains("not allowed in loop condition"));

        // But IS allowed in init context
        // Phase 226: Create empty LoopBodyLocalEnv
        let body_local_env = LoopBodyLocalEnv::new();
        instructions.clear();
        let init_result = MethodCallLowerer::lower_for_init(
            recv_val,
            "substring",
            &[arg1_ast, arg2_ast],
            &mut alloc_value,
            &env,
            &body_local_env,
            &mut instructions,
        );
        assert!(init_result.is_ok());
        assert_eq!(instructions.len(), 1);
    }

    #[test]
    fn test_phase224c_arity_mismatch() {
        // Phase 224-C Test: Arity mismatch error
        let recv_val = ValueId(10);
        let mut value_counter = 100u32;
        let mut alloc_value = || {
            let id = ValueId(value_counter);
            value_counter += 1;
            id
        };
        let mut instructions = Vec::new();
        let env = ConditionEnv::new();

        // Create dummy argument
        let dummy_arg = ASTNode::Literal {
            value: crate::ast::LiteralValue::Integer(1),
            span: crate::ast::Span::unknown(),
        };

        let result = MethodCallLowerer::lower_for_condition(
            recv_val,
            "length",
            &[dummy_arg],
            &mut alloc_value,
            &env,
            &mut instructions,
        );

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Arity mismatch"));
    }

    #[test]
    fn test_lower_index_of_with_arg() {
        // Phase 226 Test: s.indexOf(ch) with 1 argument (cascading support)
        let recv_val = ValueId(10);
        let ch_val = ValueId(11);
        let mut value_counter = 100u32;
        let mut alloc_value = || {
            let id = ValueId(value_counter);
            value_counter += 1;
            id
        };
        let mut instructions = Vec::new();

        // Phase 226: Create empty LoopBodyLocalEnv
        let body_local_env = LoopBodyLocalEnv::new();

        // Create ConditionEnv with ch variable
        let mut env = ConditionEnv::new();
        env.insert("ch".to_string(), ch_val);

        // Create argument AST
        let arg_ast = ASTNode::Variable {
            name: "ch".to_string(),
            span: crate::ast::Span::unknown(),
        };

        let result = MethodCallLowerer::lower_for_init(
            recv_val,
            "indexOf",
            &[arg_ast],
            &mut alloc_value,
            &env,
            &body_local_env,
            &mut instructions,
        );

        assert!(result.is_ok());
        let result_val = result.unwrap();
        assert_eq!(result_val, ValueId(100));
        assert_eq!(instructions.len(), 1);

        match &instructions[0] {
            JoinInst::Compute(MirLikeInst::BoxCall {
                dst,
                box_name,
                method,
                args,
            }) => {
                assert_eq!(*dst, Some(ValueId(100)));
                assert_eq!(box_name, "StringBox");
                assert_eq!(method, "indexOf");
                assert_eq!(args.len(), 2); // Receiver + 1 arg
                assert_eq!(args[0], ValueId(10)); // Receiver
                assert_eq!(args[1], ValueId(11)); // ch argument
            }
            _ => panic!("Expected BoxCall instruction"),
        }
    }

    #[test]
    fn test_cascading_resolves_body_local_first() {
        // receiver: s, method: indexOf(ch) where ch is body-local
        let mut body_env = LoopBodyLocalEnv::new();
        body_env.insert("ch".to_string(), ValueId(2));

        let mut cond_env = ConditionEnv::new();
        cond_env.insert("s".to_string(), ValueId(1));

        let recv_val = ValueId(1);
        let mut next = 100u32;
        let mut alloc_value = || {
            let id = ValueId(next);
            next += 1;
            id
        };
        let mut instructions = Vec::new();

        let result = MethodCallLowerer::lower_for_init(
            recv_val,
            "indexOf",
            &[ASTNode::Variable {
                name: "ch".to_string(),
                span: Span::unknown(),
            }],
            &mut alloc_value,
            &cond_env,
            &body_env,
            &mut instructions,
        )
        .expect("lower_for_init should succeed");

        // Ensure BoxCall args include receiver + body-local resolved value (ValueId(2))
        let boxcall = instructions
            .iter()
            .find_map(|inst| match inst {
                JoinInst::Compute(MirLikeInst::BoxCall { args, .. }) => Some(args.clone()),
                _ => None,
            })
            .expect("BoxCall not emitted");
        assert_eq!(boxcall, vec![ValueId(1), ValueId(2)]);
        assert!(result.0 >= 100);
    }

    #[test]
    fn test_lower_substring_with_args() {
        // Phase 226 Test: s.substring(i, j) with 2 arguments (cascading support)
        let recv_val = ValueId(10);
        let i_val = ValueId(11);
        let j_val = ValueId(12);
        let mut value_counter = 100u32;
        let mut alloc_value = || {
            let id = ValueId(value_counter);
            value_counter += 1;
            id
        };
        let mut instructions = Vec::new();

        // Phase 226: Create empty LoopBodyLocalEnv
        let body_local_env = LoopBodyLocalEnv::new();

        // Create ConditionEnv with i and j variables
        let mut env = ConditionEnv::new();
        env.insert("i".to_string(), i_val);
        env.insert("j".to_string(), j_val);

        // Create argument ASTs
        let arg1_ast = ASTNode::Variable {
            name: "i".to_string(),
            span: crate::ast::Span::unknown(),
        };
        let arg2_ast = ASTNode::Variable {
            name: "j".to_string(),
            span: crate::ast::Span::unknown(),
        };

        let result = MethodCallLowerer::lower_for_init(
            recv_val,
            "substring",
            &[arg1_ast, arg2_ast],
            &mut alloc_value,
            &env,
            &body_local_env,
            &mut instructions,
        );

        assert!(result.is_ok());
        let result_val = result.unwrap();
        assert_eq!(result_val, ValueId(100));
        assert_eq!(instructions.len(), 1);

        match &instructions[0] {
            JoinInst::Compute(MirLikeInst::BoxCall {
                dst,
                box_name,
                method,
                args,
            }) => {
                assert_eq!(*dst, Some(ValueId(100)));
                assert_eq!(box_name, "StringBox");
                assert_eq!(method, "substring");
                assert_eq!(args.len(), 3); // Receiver + 2 args
                assert_eq!(args[0], ValueId(10)); // Receiver
                assert_eq!(args[1], ValueId(11)); // i argument
                assert_eq!(args[2], ValueId(12)); // j argument
            }
            _ => panic!("Expected BoxCall instruction"),
        }
    }
}
