//! Phase 29ab P4: BodyLocalDerivedSlotEmitter (seg derived slot)
//!
//! Goal: treat a single body-local variable (e.g., seg) as a derived value
//! computed before the break check, without promotion or PHI.
//!
//! This is intentionally minimal and fail-fast:
//! - Supports a single derived variable recipe
//! - Requires top-level if/else assignment shape (validated on builder side)
//! - Expression lowering is limited to pure expressions (Literal/Variable/MethodCall)

use crate::ast::ASTNode;
use crate::mir::join_ir::lowering::condition_env::ConditionEnv;
use crate::mir::join_ir::lowering::condition_lowerer::lower_condition_to_joinir;
use crate::mir::join_ir::lowering::error_tags;
use crate::mir::join_ir::lowering::loop_body_local_env::LoopBodyLocalEnv;
use crate::mir::join_ir::lowering::method_call_lowerer::MethodCallLowerer;
use crate::mir::join_ir::{ConstValue, JoinInst, MirLikeInst};
use crate::mir::{MirType, ValueId};

#[derive(Debug, Clone)]
pub struct BodyLocalDerivedSlotRecipe {
    pub name: String,
    /// Base init expression from `local name = <expr>` (diagnostics only; lowering is done elsewhere).
    #[allow(dead_code)]
    pub base_init_expr: ASTNode,
    pub assign_cond: ASTNode,
    pub then_expr: ASTNode,
    pub else_expr: Option<ASTNode>,
}

pub struct BodyLocalDerivedSlotEmitter;

impl BodyLocalDerivedSlotEmitter {
    pub fn emit(
        recipe: &BodyLocalDerivedSlotRecipe,
        alloc_value: &mut dyn FnMut() -> ValueId,
        env: &ConditionEnv,
        body_local_env: &mut LoopBodyLocalEnv,
        instructions: &mut Vec<JoinInst>,
        current_static_box_name: Option<&str>,
    ) -> Result<ValueId, String> {
        let base_value = body_local_env.get(&recipe.name).ok_or_else(|| {
            error_tags::freeze(&format!(
                "[phase29ab/body_local_derived_slot/contract/missing_base] Missing base ValueId for '{}'",
                recipe.name
            ))
        })?;

        let (cond_id, cond_insts) = lower_condition_to_joinir(
            &recipe.assign_cond,
            alloc_value,
            env,
            Some(body_local_env),
            current_static_box_name,
        )?;
        instructions.extend(cond_insts);

        let then_val = lower_value_expr(
            &recipe.then_expr,
            alloc_value,
            env,
            body_local_env,
            current_static_box_name,
            instructions,
        )?;

        let else_val = if let Some(expr) = &recipe.else_expr {
            lower_value_expr(
                expr,
                alloc_value,
                env,
                body_local_env,
                current_static_box_name,
                instructions,
            )?
        } else {
            base_value
        };

        let derived = alloc_value();
        instructions.push(JoinInst::Select {
            dst: derived,
            cond: cond_id,
            then_val,
            else_val,
            type_hint: Some(MirType::String),
        });

        body_local_env.insert(recipe.name.clone(), derived);
        Ok(derived)
    }
}

fn lower_value_expr(
    expr: &ASTNode,
    alloc_value: &mut dyn FnMut() -> ValueId,
    env: &ConditionEnv,
    body_local_env: &LoopBodyLocalEnv,
    _current_static_box_name: Option<&str>,
    instructions: &mut Vec<JoinInst>,
) -> Result<ValueId, String> {
    match expr {
        ASTNode::Literal { value, .. } => match value {
            crate::ast::LiteralValue::Integer(i) => {
                let vid = alloc_value();
                instructions.push(JoinInst::Compute(MirLikeInst::Const {
                    dst: vid,
                    value: ConstValue::Integer(*i),
                }));
                Ok(vid)
            }
            crate::ast::LiteralValue::String(s) => {
                let vid = alloc_value();
                instructions.push(JoinInst::Compute(MirLikeInst::Const {
                    dst: vid,
                    value: ConstValue::String(s.clone()),
                }));
                Ok(vid)
            }
            _ => Err(error_tags::freeze(&format!(
                "[phase29ab/body_local_derived_slot/contract/unsupported_literal] {:?}",
                value
            ))),
        },
        ASTNode::Variable { name, .. } => resolve_var_value(name, env, body_local_env).ok_or_else(|| {
            error_tags::freeze(&format!(
                "[phase29ab/body_local_derived_slot/contract/missing_var] '{}' not found in envs",
                name
            ))
        }),
        ASTNode::MethodCall {
            object,
            method,
            arguments,
            ..
        } => {
            let recv_val = match object.as_ref() {
                ASTNode::Variable { name, .. } => resolve_var_value(name, env, body_local_env)
                    .ok_or_else(|| {
                        error_tags::freeze(&format!(
                            "[phase29ab/body_local_derived_slot/contract/missing_receiver] '{}' not found in envs",
                            name
                        ))
                    })?,
                _ => {
                    return Err(error_tags::freeze(&format!(
                        "[phase29ab/body_local_derived_slot/contract/receiver_kind] Unsupported receiver: {:?}",
                        object
                    )))
                }
            };
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
        _ => Err(error_tags::freeze(&format!(
            "[phase29ab/body_local_derived_slot/contract/unsupported_expr] {:?}",
            expr
        ))),
    }
}

fn resolve_var_value(
    name: &str,
    env: &ConditionEnv,
    body_local_env: &LoopBodyLocalEnv,
) -> Option<ValueId> {
    body_local_env.get(name).or_else(|| env.get(name))
}
