//! Phase 94: BodyLocalDerivedEmitter (P5b "complete E2E" for body-local reassignment)
//!
//! Goal: represent a loop body-local that is conditionally overridden (e.g. `ch`)
//! as a pure derived JoinIR value, without PHI-carrying it across iterations.
//!
//! This is intentionally minimal and fail-fast:
//! - Supports a single derived variable recipe (typically `ch`)
//! - Supports the P5b escape shape where:
//!   - `ch_top` is computed by a top-level `local ch = ...`
//!   - `escape_cond` is computed from `ch_top` (e.g. `ch == "\\"`)
//!   - `ch` is conditionally overridden after a pre-increment of the loop counter
//!   - loop counter update is `else_delta` always + `pre_delta` when `escape_cond`
//!
//! The extraction/policy lives on the Pattern2 builder side; this module only
//! emits JoinIR given a validated recipe.

use crate::ast::ASTNode;
use crate::mir::join_ir::lowering::condition_env::ConditionEnv;
use crate::mir::join_ir::lowering::condition_lowerer::lower_condition_to_joinir;
use crate::mir::join_ir::lowering::loop_body_local_env::LoopBodyLocalEnv;
use crate::mir::join_ir::lowering::method_call_lowerer::MethodCallLowerer;
use crate::mir::join_ir::{BinOpKind, ConstValue, JoinInst, MirLikeInst};
use crate::mir::{MirType, ValueId};

/// SSOT recipe: "derived body-local" + loop-counter pre/post increment semantics.
#[derive(Debug, Clone)]
pub struct BodyLocalDerivedRecipe {
    /// Derived variable name to register into LoopBodyLocalEnv (e.g. "ch").
    pub name: String,
    /// Base init expression from `local name = <expr>` (diagnostics only; lowering is done elsewhere).
    #[allow(dead_code)]
    pub base_init_expr: ASTNode,
    /// Escape condition evaluated on the *base* value (e.g. `ch == "\\"`).
    pub escape_cond: ASTNode,
    /// Loop counter variable name (typically the loop var, e.g. "i").
    pub loop_counter_name: String,
    /// Pre-increment applied inside escape branch before override expr (e.g. `i = i + 1`).
    pub pre_delta: i64,
    /// Post-increment applied at the loop tail (e.g. the unconditional `i = i + 1`).
    pub post_delta: i64,
    /// Optional bounds check after the pre-increment (e.g. `i < n` in the escape branch).
    /// This is evaluated with `loop_counter_name` bound to `i_pre` (= i + pre_delta).
    pub bounds_check: Option<ASTNode>,
    /// Override expression (e.g. `s.substring(i, i + 1)` in the escape branch),
    /// evaluated with `loop_counter_name` bound to `i_pre`.
    pub override_expr: ASTNode,
}

#[derive(Debug, Clone, Copy)]
pub struct BodyLocalDerivedEmission {
    /// ValueId of the escape condition (truthy) evaluated on the base body-local value.
    #[allow(dead_code)]
    pub escape_cond_id: ValueId,
    /// ValueId of the loop counter next value (includes conditional pre-delta + post-delta).
    pub loop_counter_next: ValueId,
}

/// Generation box: emits JoinIR for the derived slot and the conditional loop-counter next.
pub struct BodyLocalDerivedEmitter;

impl BodyLocalDerivedEmitter {
    pub fn emit(
        recipe: &BodyLocalDerivedRecipe,
        alloc_value: &mut dyn FnMut() -> ValueId,
        env: &ConditionEnv,
        body_local_env: &mut LoopBodyLocalEnv,
        instructions: &mut Vec<JoinInst>,
    ) -> Result<BodyLocalDerivedEmission, String> {
        let base_value = body_local_env.get(&recipe.name).ok_or_else(|| {
            format!(
                "[phase94/body_local_derived/contract/missing_base_value] Missing base ValueId for body-local '{}' in LoopBodyLocalEnv",
                recipe.name
            )
        })?;

        if recipe.pre_delta < 0 || recipe.post_delta < 0 {
            return Err(format!(
                "[phase94/body_local_derived/contract/negative_delta] Invalid deltas: pre_delta={}, post_delta={} (must be >= 0)",
                recipe.pre_delta, recipe.post_delta
            ));
        }
        if recipe.post_delta == 0 {
            return Err(format!(
                "[phase94/body_local_derived/contract/post_delta_zero] post_delta=0 is not supported (would stall loop counter '{}')",
                recipe.loop_counter_name
            ));
        }

        // ------------------------------------------------------------
        // 1) escape_cond evaluated on base `name` (ch_top)
        // ------------------------------------------------------------
        let (escape_cond_id, escape_cond_insts) = lower_condition_to_joinir(
            &recipe.escape_cond,
            alloc_value,
            env,
            Some(body_local_env),
            None, // Phase 252: No static box context
        )?;
        instructions.extend(escape_cond_insts);

        // ------------------------------------------------------------
        // 2) i_pre = i + pre_delta (used for bounds + override expr)
        // ------------------------------------------------------------
        let counter_cur = env.get(&recipe.loop_counter_name).ok_or_else(|| {
            format!(
                "[phase94/body_local_derived/contract/missing_loop_counter] ConditionEnv missing loop counter '{}'",
                recipe.loop_counter_name
            )
        })?;

        let counter_pre = if recipe.pre_delta == 0 {
            counter_cur
        } else {
            let pre_const = alloc_value();
            instructions.push(JoinInst::Compute(MirLikeInst::Const {
                dst: pre_const,
                value: ConstValue::Integer(recipe.pre_delta),
            }));
            let pre_sum = alloc_value();
            instructions.push(JoinInst::Compute(MirLikeInst::BinOp {
                dst: pre_sum,
                op: BinOpKind::Add,
                lhs: counter_cur,
                rhs: pre_const,
            }));
            pre_sum
        };

        // ------------------------------------------------------------
        // 3) override_guard = escape_cond && bounds_check(i_pre) (if present)
        // ------------------------------------------------------------
        let override_guard = if let Some(bounds_ast) = &recipe.bounds_check {
            let mut env_pre = env.clone();
            env_pre.insert(recipe.loop_counter_name.clone(), counter_pre);
            let (bounds_ok, bounds_insts) =
                lower_condition_to_joinir(bounds_ast, alloc_value, &env_pre, Some(body_local_env), None)?; // Phase 252: No static box context
            instructions.extend(bounds_insts);

            let guard = alloc_value();
            instructions.push(JoinInst::Compute(MirLikeInst::BinOp {
                dst: guard,
                op: BinOpKind::And,
                lhs: escape_cond_id,
                rhs: bounds_ok,
            }));
            guard
        } else {
            escape_cond_id
        };

        // ------------------------------------------------------------
        // 4) override_val (evaluated with loop_counter_name := i_pre)
        // ------------------------------------------------------------
        let mut env_pre = env.clone();
        env_pre.insert(recipe.loop_counter_name.clone(), counter_pre);
        let override_val = Self::lower_override_expr(
            &recipe.override_expr,
            alloc_value,
            &env_pre,
            body_local_env,
            instructions,
        )?;

        // ------------------------------------------------------------
        // 5) derived = Select(override_guard, override_val, base_value)
        // ------------------------------------------------------------
        let derived = alloc_value();
        instructions.push(JoinInst::Select {
            dst: derived,
            cond: override_guard,
            then_val: override_val,
            else_val: base_value,
            type_hint: Some(MirType::String),
        });
        body_local_env.insert(recipe.name.clone(), derived);

        // ------------------------------------------------------------
        // 6) loop_counter_next = Select(escape_cond, i + (pre+post), i + post)
        // ------------------------------------------------------------
        let then_delta = recipe.pre_delta + recipe.post_delta;
        let else_delta = recipe.post_delta;
        if then_delta == else_delta {
            return Err(format!(
                "[phase94/body_local_derived/contract/equal_total_deltas] then_delta == else_delta == {} for loop counter '{}' (pre_delta={}, post_delta={})",
                then_delta, recipe.loop_counter_name, recipe.pre_delta, recipe.post_delta
            ));
        }

        let then_const = alloc_value();
        instructions.push(JoinInst::Compute(MirLikeInst::Const {
            dst: then_const,
            value: ConstValue::Integer(then_delta),
        }));
        let then_sum = alloc_value();
        instructions.push(JoinInst::Compute(MirLikeInst::BinOp {
            dst: then_sum,
            op: BinOpKind::Add,
            lhs: counter_cur,
            rhs: then_const,
        }));

        let else_const = alloc_value();
        instructions.push(JoinInst::Compute(MirLikeInst::Const {
            dst: else_const,
            value: ConstValue::Integer(else_delta),
        }));
        let else_sum = alloc_value();
        instructions.push(JoinInst::Compute(MirLikeInst::BinOp {
            dst: else_sum,
            op: BinOpKind::Add,
            lhs: counter_cur,
            rhs: else_const,
        }));

        let counter_next = alloc_value();
        instructions.push(JoinInst::Select {
            dst: counter_next,
            cond: escape_cond_id,
            then_val: then_sum,
            else_val: else_sum,
            type_hint: Some(MirType::Integer),
        });

        Ok(BodyLocalDerivedEmission {
            escape_cond_id,
            loop_counter_next: counter_next,
        })
    }

    fn lower_override_expr(
        expr: &ASTNode,
        alloc_value: &mut dyn FnMut() -> ValueId,
        env: &ConditionEnv,
        body_local_env: &LoopBodyLocalEnv,
        instructions: &mut Vec<JoinInst>,
    ) -> Result<ValueId, String> {
        match expr {
            ASTNode::MethodCall {
                object,
                method,
                arguments,
                ..
            } => {
                let recv_name = match object.as_ref() {
                    ASTNode::Variable { name, .. } => name,
                    _ => {
                        return Err(format!(
                            "[phase94/body_local_derived/contract/override_receiver] Override receiver must be a variable: {:?}",
                            object
                        ));
                    }
                };

                let recv_val = body_local_env
                    .get(recv_name)
                    .or_else(|| env.get(recv_name))
                    .ok_or_else(|| {
                        format!(
                            "[phase94/body_local_derived/contract/override_receiver_missing] Receiver '{}' not found in envs",
                            recv_name
                        )
                    })?;

                MethodCallLowerer::lower_for_init(
                    recv_val,
                    method,
                    arguments,
                    alloc_value,
                    env,
                    body_local_env,
                    instructions,
                )
            }
            _ => Err(format!(
                "[phase94/body_local_derived/contract/override_expr_kind] Override expr must be MethodCall (pure): {:?}",
                expr
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{BinaryOperator, LiteralValue, Span};
    use crate::mir::join_ir::lowering::condition_env::ConditionEnv;

    fn var(name: &str) -> ASTNode {
        ASTNode::Variable {
            name: name.to_string(),
            span: Span::unknown(),
        }
    }

    fn str_lit(s: &str) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::String(s.to_string()),
            span: Span::unknown(),
        }
    }

    fn int_lit(i: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(i),
            span: Span::unknown(),
        }
    }

    fn binop(op: BinaryOperator, left: ASTNode, right: ASTNode) -> ASTNode {
        ASTNode::BinaryOp {
            operator: op,
            left: Box::new(left),
            right: Box::new(right),
            span: Span::unknown(),
        }
    }

    fn method_call(obj: &str, method: &str, args: Vec<ASTNode>) -> ASTNode {
        ASTNode::MethodCall {
            object: Box::new(var(obj)),
            method: method.to_string(),
            arguments: args,
            span: Span::unknown(),
        }
    }

    #[test]
    fn derived_emits_select_and_updates_env() {
        // Pattern:
        //   base: ch_top already computed and in body_local_env
        //   escape_cond: ch == "\\"
        //   bounds: i < n (evaluated with i_pre)
        //   override_expr: s.substring(i, i + 1) (evaluated with i_pre)
        let recipe = BodyLocalDerivedRecipe {
            name: "ch".to_string(),
            base_init_expr: method_call(
                "s",
                "substring",
                vec![var("i"), binop(BinaryOperator::Add, var("i"), int_lit(1))],
            ),
            escape_cond: binop(BinaryOperator::Equal, var("ch"), str_lit("\\")),
            loop_counter_name: "i".to_string(),
            pre_delta: 1,
            post_delta: 1,
            bounds_check: Some(binop(BinaryOperator::Less, var("i"), var("n"))),
            override_expr: method_call(
                "s",
                "substring",
                vec![var("i"), binop(BinaryOperator::Add, var("i"), int_lit(1))],
            ),
        };

        let mut env = ConditionEnv::new();
        env.insert("i".to_string(), ValueId(10));
        env.insert("n".to_string(), ValueId(11));
        env.insert("s".to_string(), ValueId(12));

        let mut body_env = LoopBodyLocalEnv::new();
        body_env.insert("ch".to_string(), ValueId(20));

        let mut next = 100u32;
        let mut alloc_value = || {
            let id = ValueId(next);
            next += 1;
            id
        };

        let mut insts = Vec::new();
        let out = BodyLocalDerivedEmitter::emit(&recipe, &mut alloc_value, &env, &mut body_env, &mut insts)
            .expect("emit should succeed");

        // Env must now point `ch` to the derived value (not the base 20).
        let ch_now = body_env.get("ch").expect("ch should exist");
        assert_ne!(ch_now, ValueId(20));
        assert!(insts.iter().any(|i| matches!(i, JoinInst::Select { .. })));
        assert_ne!(out.loop_counter_next, ValueId(10));
    }

    #[test]
    fn fails_fast_on_equal_total_deltas() {
        let recipe = BodyLocalDerivedRecipe {
            name: "ch".to_string(),
            base_init_expr: str_lit("x"),
            escape_cond: binop(BinaryOperator::Equal, var("ch"), str_lit("\\")),
            loop_counter_name: "i".to_string(),
            pre_delta: 0,
            post_delta: 1,
            bounds_check: None,
            override_expr: method_call("s", "substring", vec![var("i"), binop(BinaryOperator::Add, var("i"), int_lit(1))]),
        };

        let mut env = ConditionEnv::new();
        env.insert("i".to_string(), ValueId(10));
        env.insert("s".to_string(), ValueId(12));

        let mut body_env = LoopBodyLocalEnv::new();
        body_env.insert("ch".to_string(), ValueId(20));

        let mut next = 200u32;
        let mut alloc_value = || {
            let id = ValueId(next);
            next += 1;
            id
        };

        let mut insts = Vec::new();
        let err = BodyLocalDerivedEmitter::emit(&recipe, &mut alloc_value, &env, &mut body_env, &mut insts)
            .unwrap_err();
        assert!(err.contains("equal_total_deltas"));
    }

    #[test]
    fn fails_fast_on_unsupported_override_expr_kind() {
        let recipe = BodyLocalDerivedRecipe {
            name: "ch".to_string(),
            base_init_expr: str_lit("x"),
            escape_cond: binop(BinaryOperator::Equal, var("ch"), str_lit("\\")),
            loop_counter_name: "i".to_string(),
            pre_delta: 1,
            post_delta: 1,
            bounds_check: None,
            // Literal is not allowed (must be MethodCall)
            override_expr: str_lit("!"),
        };

        let mut env = ConditionEnv::new();
        env.insert("i".to_string(), ValueId(1));
        env.insert("s".to_string(), ValueId(2));

        let mut body_env = LoopBodyLocalEnv::new();
        body_env.insert("ch".to_string(), ValueId(3));

        let mut next = 10u32;
        let mut alloc_value = || {
            let id = ValueId(next);
            next += 1;
            id
        };

        let mut insts = Vec::new();
        let err = BodyLocalDerivedEmitter::emit(&recipe, &mut alloc_value, &env, &mut body_env, &mut insts)
            .unwrap_err();
        assert!(
            err.contains("override_expr_kind"),
            "expected override_expr_kind contract violation, got: {err}"
        );
    }
}
