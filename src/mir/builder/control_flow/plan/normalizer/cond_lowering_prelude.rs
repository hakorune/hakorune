//! Condition prelude statement processing.

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::normalizer::{loop_body_lowering, PlanNormalizer};
use crate::mir::builder::control_flow::plan::policies::cond_prelude_vocab::{classify_cond_prelude_stmt, CondPreludeStmtKind};
use crate::mir::builder::control_flow::plan::CoreEffectPlan;
use crate::mir::builder::MirBuilder;
use crate::mir::{Effect, EffectMask, ValueId};
use std::collections::BTreeMap;

pub(in crate::mir::builder::control_flow::plan::normalizer) fn lower_cond_prelude_stmts(
    builder: &mut MirBuilder,
    phi_bindings: &BTreeMap<String, ValueId>,
    prelude_stmts: &[ASTNode],
    error_prefix: &str,
) -> Result<(BTreeMap<String, ValueId>, Vec<CoreEffectPlan>), String> {
    if prelude_stmts.is_empty() {
        return Ok((phi_bindings.clone(), Vec::new()));
    }

    let mut bindings = phi_bindings.clone();
    let mut effects = Vec::new();
    for stmt in prelude_stmts {
        if stmt.contains_non_local_exit() {
            return Err("[freeze:contract][cond_prelude] exit stmt is forbidden in condition prelude".to_string());
        }

        let Some(kind) = classify_cond_prelude_stmt(stmt) else {
            return Err(format!("[freeze:contract][cond_prelude] {error_prefix}: unsupported stmt: {}", stmt.node_type()));
        };

        match kind {
            CondPreludeStmtKind::Assignment => {
                let ASTNode::Assignment { target, value, .. } = stmt else { unreachable!() };
                let (name, value_id, mut stmt_effects) = loop_body_lowering::lower_assignment_value(builder, &bindings, target, value, error_prefix)?;
                bindings.insert(name.clone(), value_id);
                builder.variable_ctx.variable_map.insert(name, value_id);
                effects.append(&mut stmt_effects);
            }
            CondPreludeStmtKind::Local => {
                let ASTNode::Local { variables, initial_values, .. } = stmt else { unreachable!() };
                let (inits, mut stmt_effects) = loop_body_lowering::lower_local_init_values(builder, &bindings, variables, initial_values, error_prefix)?;
                for (name, value_id) in inits {
                    bindings.insert(name.clone(), value_id);
                    builder.variable_ctx.variable_map.insert(name, value_id);
                }
                effects.append(&mut stmt_effects);
            }
            CondPreludeStmtKind::MethodCall => {
                let mut stmt_effects = loop_body_lowering::lower_method_call_stmt(builder, &bindings, stmt, error_prefix)?;
                effects.append(&mut stmt_effects);
            }
            CondPreludeStmtKind::FunctionCall => {
                let mut stmt_effects = loop_body_lowering::lower_function_call_stmt(builder, &bindings, stmt, error_prefix)?;
                effects.append(&mut stmt_effects);
            }
            CondPreludeStmtKind::Print => {
                let ASTNode::Print { expression, .. } = stmt else { unreachable!() };
                let (value_id, mut stmt_effects) = PlanNormalizer::lower_value_ast(expression, builder, &bindings)?;
                stmt_effects.push(CoreEffectPlan::ExternCall {
                    dst: None,
                    iface_name: "env.console".to_string(),
                    method_name: "log".to_string(),
                    args: vec![value_id],
                    effects: EffectMask::PURE.add(Effect::Io),
                });
                effects.append(&mut stmt_effects);
            }
        }
    }

    Ok((bindings, effects))
}
