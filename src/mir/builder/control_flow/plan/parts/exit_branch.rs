//! Exit-branch lowering helpers (Parts SSOT).
//!
//! Owns exit-branch prelude splitting and exit lowering.
//! SSOT: docs/development/current/main/design/recipe-tree-and-parts-ssot.md

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::normalizer::PlanNormalizer;
use crate::mir::builder::control_flow::plan::LoopPlanExpressionPortV1;
use crate::mir::builder::control_flow::plan::{CoreExitPlan, CorePlan, LoweredRecipe};
use crate::mir::builder::MirBuilder;
use crate::mir::resolved_semantics::ExprChildRoleV1;
use std::collections::BTreeMap;

use super::super::steps::effects_to_plans;
use super::exit as parts_exit;
use super::stmt as parts_stmt;
use super::var_map_scope::with_saved_variable_map;

pub(in crate::mir::builder) fn split_exit_branch<'a>(
    body: &'a [ASTNode],
    error_prefix: &str,
) -> Result<(Vec<&'a ASTNode>, &'a ASTNode, bool), String> {
    let Some(last) = body.last() else {
        return Err(format!(
            "{error_prefix}: if body must be single-exit (empty)"
        ));
    };
    if matches!(
        last,
        ASTNode::Return { .. } | ASTNode::Break { .. } | ASTNode::Continue { .. }
    ) {
        if body.len() == 1 {
            return Ok((Vec::new(), last, matches!(last, ASTNode::Return { .. })));
        }
        let mut prelude = Vec::new();
        for stmt in &body[..body.len() - 1] {
            prelude.push(stmt);
        }
        return Ok((prelude, last, matches!(last, ASTNode::Return { .. })));
    }
    Err(format!(
        "{error_prefix}: if body must be single-exit{}",
        match last {
            ASTNode::If {
                then_body,
                else_body,
                ..
            } => {
                let then_last = then_body.last().map(|n| n.node_type()).unwrap_or("empty");
                let else_last = else_body
                    .as_ref()
                    .and_then(|b| b.last())
                    .map(|n| n.node_type())
                    .unwrap_or("none");
                format!(" (last=If then_last={then_last} else_last={else_last})")
            }
            _ => format!(" (last={})", last.node_type()),
        }
    ))
}

/// Validate the first source-port ExitIf branch slice.
///
/// The source path uses the port's body/statement relation for the one branch
/// member; it never indexes a copied `RecipeBody` or reconstructs a source site
/// from an AST node. This preparation helper deliberately performs no lowering
/// or allocation.
pub(in crate::mir::builder) fn validate_return_exit_branch_input<'input, P>(
    port: &P,
    body: &P::BodyInput<'input>,
    error_prefix: &str,
) -> Result<(), String>
where
    P: LoopPlanExpressionPortV1 + 'input,
{
    let len = port.body_statements(body).len();
    if len != 1 {
        return Err(format!(
            "{error_prefix}: if body must contain exactly one return (len={len})"
        ));
    }
    let statement = port.body_stmt(body, 0).map_err(|error| error.render())?;
    if !matches!(
        port.stmt_syntax(&statement),
        ASTNode::Return { value: Some(_), .. }
    ) {
        return Err(format!(
            "{error_prefix}: if body must end in value return (actual={})",
            port.stmt_syntax(&statement).node_type()
        ));
    }
    port.child_expr_from_stmt(&statement, ExprChildRoleV1::ReturnValue)
        .map_err(|error| format!("{error_prefix}: return value: {}", error.render()))?;
    Ok(())
}

pub(in crate::mir::builder) fn lower_exit_branch_with_prelude(
    builder: &mut MirBuilder,
    current_bindings: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    prelude: &[&ASTNode],
    exit_stmt: &ASTNode,
    error_prefix: &str,
) -> Result<Vec<LoweredRecipe>, String> {
    lower_exit_branch_with_prelude_impl(
        builder,
        current_bindings,
        carrier_step_phis,
        None,
        prelude,
        exit_stmt,
        error_prefix,
    )
}

pub(in crate::mir::builder) fn lower_exit_branch_with_prelude_with_break_phi_args(
    builder: &mut MirBuilder,
    current_bindings: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: &BTreeMap<String, crate::mir::ValueId>,
    prelude: &[&ASTNode],
    exit_stmt: &ASTNode,
    error_prefix: &str,
) -> Result<Vec<LoweredRecipe>, String> {
    lower_exit_branch_with_prelude_impl(
        builder,
        current_bindings,
        carrier_step_phis,
        Some(break_phi_dsts),
        prelude,
        exit_stmt,
        error_prefix,
    )
}

fn lower_exit_branch_with_prelude_impl(
    builder: &mut MirBuilder,
    current_bindings: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: Option<&BTreeMap<String, crate::mir::ValueId>>,
    prelude: &[&ASTNode],
    exit_stmt: &ASTNode,
    error_prefix: &str,
) -> Result<Vec<LoweredRecipe>, String> {
    if !prelude.is_empty() {
        return with_saved_variable_map(builder, |builder| {
            let mut plans = Vec::new();
            let mut branch_bindings = current_bindings.clone();
            for stmt in prelude {
                plans.extend(parts_stmt::lower_return_prelude_stmt(
                    builder,
                    &mut branch_bindings,
                    carrier_step_phis,
                    break_phi_dsts,
                    stmt,
                    error_prefix,
                )?);
            }
            plans.extend(lower_exit_branch(
                builder,
                &branch_bindings,
                carrier_step_phis,
                break_phi_dsts,
                exit_stmt,
                error_prefix,
            )?);
            Ok(plans)
        });
    }
    let mut plans = Vec::new();
    plans.extend(lower_exit_branch(
        builder,
        current_bindings,
        carrier_step_phis,
        break_phi_dsts,
        exit_stmt,
        error_prefix,
    )?);
    Ok(plans)
}

fn lower_exit_branch(
    builder: &mut MirBuilder,
    current_bindings: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: Option<&BTreeMap<String, crate::mir::ValueId>>,
    stmt: &ASTNode,
    error_prefix: &str,
) -> Result<Vec<LoweredRecipe>, String> {
    let mut plans = Vec::new();
    let exit = match stmt {
        ASTNode::Break { .. } => match break_phi_dsts {
            Some(break_phi_dsts) => parts_exit::build_break_with_phi_args(
                break_phi_dsts,
                current_bindings,
                error_prefix,
            )?,
            None => CoreExitPlan::Break(1),
        },
        ASTNode::Continue { .. } => parts_exit::build_continue_with_phi_args(
            builder,
            carrier_step_phis,
            current_bindings,
            error_prefix,
        )?,
        ASTNode::Return { value, .. } => {
            let Some(value) = value.as_ref() else {
                return Err(format!("{error_prefix}: return without value"));
            };
            let (value_id, effects) =
                PlanNormalizer::lower_value_ast(value, builder, current_bindings)?;
            plans.extend(effects_to_plans(effects));
            CoreExitPlan::Return(Some(value_id))
        }
        _ => {
            return Err(format!(
                "{error_prefix}: if body must be break/continue/return"
            ));
        }
    };
    plans.push(CorePlan::Exit(exit));
    Ok(plans)
}

#[cfg(test)]
mod tests {
    use super::validate_return_exit_branch_input;
    use crate::ast::{ASTNode, LiteralValue, Span};
    use crate::mir::builder::control_flow::plan::RawLoopPlanExpressionPortV1;

    fn return_value(value: i64) -> ASTNode {
        ASTNode::Return {
            value: Some(Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(value),
                span: Span::unknown(),
            })),
            span: Span::unknown(),
        }
    }

    #[test]
    fn source_port_validates_single_value_return_branch() {
        let statements = vec![return_value(7)];
        let port = RawLoopPlanExpressionPortV1::new();
        let body = &statements[..];
        validate_return_exit_branch_input(&port, &body, "source-port-exit-branch")
            .expect("direct source exit branch");
    }

    #[test]
    fn source_port_rejects_branch_prelude_and_non_return_tail() {
        let statements = vec![
            ASTNode::Local {
                variables: vec!["value".to_owned()],
                initial_values: vec![Some(Box::new(ASTNode::Literal {
                    value: LiteralValue::Integer(1),
                    span: Span::unknown(),
                }))],
                declared_type_names: Vec::new(),
                span: Span::unknown(),
            },
            return_value(7),
        ];
        let port = RawLoopPlanExpressionPortV1::new();
        let body = &statements[..];
        let error = validate_return_exit_branch_input(&port, &body, "source-port-exit-branch")
            .expect_err("non-exit tail must reject");
        assert!(error.contains("exactly one return"), "{error}");
    }

    #[test]
    fn source_port_rejects_return_without_value() {
        let statements = vec![ASTNode::Return {
            value: None,
            span: Span::unknown(),
        }];
        let port = RawLoopPlanExpressionPortV1::new();
        let body = &statements[..];
        let error = validate_return_exit_branch_input(&port, &body, "source-port-exit-branch")
            .expect_err("value-less return must reject");
        assert!(error.contains("value return"), "{error}");
    }
}
