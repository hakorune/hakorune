//! Shared associated-input owner for the direct GenericLoopV1 body prefix.
//!
//! This module is deliberately disconnected from the located composer.  It
//! consumes only the expression port and delegates statement primitives and
//! join state to their existing owners.  Raw-only tail shapes stay in `v1`.

use std::collections::BTreeMap;

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::normalizer::loop_body_lowering_associated_input::{
    lower_assignment_inputs, lower_function_call_statement_input, lower_local_statement_input,
    lower_method_call_statement_input, lower_return_statement_input,
};
use crate::mir::builder::control_flow::plan::parts::conditional_update;
use crate::mir::builder::control_flow::plan::parts::var_map_scope::publish_defined_binding;
use crate::mir::builder::control_flow::plan::{LoopPlanExpressionPortV1, LoweredRecipe};
use crate::mir::builder::MirBuilder;
use crate::mir::resolved_semantics::{BodyChildRoleV1, ExprChildRoleV1};
use crate::mir::ValueId;

use super::lower_generic_loop_v1_direct_inputs;

pub(in crate::mir::builder) fn lower_direct_body_input<'input, P>(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, ValueId>,
    port: &P,
    body: P::BodyInput<'input>,
    carrier_step_phis: &BTreeMap<String, ValueId>,
    loop_var: &str,
    error_prefix: &str,
) -> Result<Vec<LoweredRecipe>, String>
where
    P: LoopPlanExpressionPortV1 + 'input,
{
    let mut reject = |_builder: &mut MirBuilder,
                      _bindings: &mut BTreeMap<String, ValueId>,
                      _port: &P,
                      _statement: P::StmtInput<'input>| {
        Err("unsupported associated direct statement".to_string())
    };
    lower_direct_body_input_with_policy(
        builder,
        current_bindings,
        port,
        body,
        carrier_step_phis,
        loop_var,
        error_prefix,
        &mut reject,
        &|_| false,
    )
}

pub(in crate::mir::builder) fn lower_direct_body_input_with_policy<'input, P, F, S>(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, ValueId>,
    port: &P,
    body: P::BodyInput<'input>,
    carrier_step_phis: &BTreeMap<String, ValueId>,
    loop_var: &str,
    error_prefix: &str,
    fallback: &mut F,
    skip_statement: &S,
) -> Result<Vec<LoweredRecipe>, String>
where
    P: LoopPlanExpressionPortV1 + 'input,
    F: FnMut(
        &mut MirBuilder,
        &mut BTreeMap<String, ValueId>,
        &P,
        P::StmtInput<'input>,
    ) -> Result<Vec<LoweredRecipe>, String>,
    S: Fn(&ASTNode) -> bool,
{
    let statements = port.body_statements(&body);
    lower_generic_loop_v1_direct_inputs(
        builder,
        current_bindings,
        port,
        statements.iter().enumerate().filter_map(|(index, _)| {
            let input = port
                .body_stmt(&body, index)
                .expect("body index came from body_statements");
            (!skip_statement(port.stmt_syntax(&input))).then_some(input)
        }),
        |builder, bindings, port, statement| {
            lower_direct_statement_input(
                builder,
                bindings,
                port,
                statement,
                carrier_step_phis,
                loop_var,
                error_prefix,
                fallback,
                skip_statement,
            )
        },
    )
}

fn lower_direct_statement_input<'input, P, F, S>(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, ValueId>,
    port: &P,
    statement: P::StmtInput<'input>,
    carrier_step_phis: &BTreeMap<String, ValueId>,
    loop_var: &str,
    error_prefix: &str,
    fallback: &mut F,
    skip_statement: &S,
) -> Result<Vec<LoweredRecipe>, String>
where
    P: LoopPlanExpressionPortV1 + 'input,
    F: FnMut(
        &mut MirBuilder,
        &mut BTreeMap<String, ValueId>,
        &P,
        P::StmtInput<'input>,
    ) -> Result<Vec<LoweredRecipe>, String>,
    S: Fn(&ASTNode) -> bool,
{
    match port.stmt_syntax(&statement) {
        ASTNode::Local { .. } => {
            let (inits, effects) = lower_local_statement_input(
                port,
                statement,
                builder,
                current_bindings,
                error_prefix,
            )?;
            for (name, value) in inits {
                publish_defined_binding(builder, current_bindings, name, value);
            }
            Ok(crate::mir::builder::control_flow::plan::steps::effects_to_plans(effects))
        }
        ASTNode::Assignment { .. } => {
            let target = port
                .child_expr_from_stmt(&statement, ExprChildRoleV1::AssignmentTarget)
                .map_err(|error| error.render())?;
            let value = port
                .child_expr_from_stmt(&statement, ExprChildRoleV1::AssignmentValue)
                .map_err(|error| error.render())?;
            let (binding, effects) = lower_assignment_inputs(
                port,
                target,
                value,
                builder,
                current_bindings,
                error_prefix,
            )?;
            if let Some((name, value)) = binding {
                publish_defined_binding(builder, current_bindings, name, value);
            }
            Ok(crate::mir::builder::control_flow::plan::steps::effects_to_plans(effects))
        }
        ASTNode::MethodCall { .. } => {
            let input = port
                .statement_expr(&statement)
                .map_err(|error| error.render())?;
            let effects = lower_method_call_statement_input(
                port,
                input,
                builder,
                current_bindings,
                error_prefix,
            )?;
            Ok(crate::mir::builder::control_flow::plan::steps::effects_to_plans(effects))
        }
        ASTNode::FunctionCall { .. } => {
            let input = port
                .statement_expr(&statement)
                .map_err(|error| error.render())?;
            let effects = lower_function_call_statement_input(
                port,
                input,
                builder,
                current_bindings,
                error_prefix,
            )?;
            Ok(crate::mir::builder::control_flow::plan::steps::effects_to_plans(effects))
        }
        ASTNode::Return { .. } => {
            lower_return_statement_input(port, statement, builder, current_bindings, error_prefix)
        }
        ASTNode::If { .. } => lower_direct_if_input(
            builder,
            current_bindings,
            port,
            statement,
            carrier_step_phis,
            loop_var,
            error_prefix,
            fallback,
            skip_statement,
        ),
        _ => fallback(builder, current_bindings, port, statement),
    }
}

fn lower_direct_if_input<'input, P, F, S>(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, ValueId>,
    port: &P,
    statement: P::StmtInput<'input>,
    carrier_step_phis: &BTreeMap<String, ValueId>,
    loop_var: &str,
    error_prefix: &str,
    fallback: &mut F,
    skip_statement: &S,
) -> Result<Vec<LoweredRecipe>, String>
where
    P: LoopPlanExpressionPortV1 + 'input,
    F: FnMut(
        &mut MirBuilder,
        &mut BTreeMap<String, ValueId>,
        &P,
        P::StmtInput<'input>,
    ) -> Result<Vec<LoweredRecipe>, String>,
    S: Fn(&ASTNode) -> bool,
{
    let condition = port
        .child_expr_from_stmt(&statement, ExprChildRoleV1::IfCondition)
        .map_err(|error| error.render())?;
    let then_body = port
        .child_body_from_stmt(&statement, BodyChildRoleV1::IfThen)
        .map_err(|error| error.render())?;
    let else_body = port
        .child_body_from_stmt(&statement, BodyChildRoleV1::IfElse)
        .ok();

    // Keep the existing Select-shaped conditional-update owner first.  The
    // helper is port-aware and therefore does not reconstruct source syntax.
    if !body_has_only_loop_update_shapes(port, &then_body, loop_var)
        || else_body
            .as_ref()
            .is_some_and(|body| !body_has_only_loop_update_shapes(port, body, loop_var))
    {
        return crate::mir::builder::control_flow::plan::parts::lower_direct_if_join_input(
            port,
            builder,
            current_bindings,
            condition,
            then_body,
            else_body,
            carrier_step_phis,
            loop_var,
            error_prefix,
            |builder, bindings, port, body| {
                lower_direct_body_input_with_policy(
                    builder,
                    bindings,
                    port,
                    body,
                    carrier_step_phis,
                    loop_var,
                    error_prefix,
                    fallback,
                    skip_statement,
                )
            },
        );
    }

    let mut updates = BTreeMap::new();
    let carrier_phis = BTreeMap::new();
    if let Some(plans) = conditional_update::try_lower_conditional_update_if_input(
        port,
        builder,
        current_bindings,
        &carrier_phis,
        carrier_step_phis,
        &mut updates,
        None,
        condition,
        &then_body,
        else_body.as_ref(),
        error_prefix,
    )? {
        return Ok(plans);
    }

    let condition = port
        .child_expr_from_stmt(&statement, ExprChildRoleV1::IfCondition)
        .map_err(|error| error.render())?;

    crate::mir::builder::control_flow::plan::parts::lower_direct_if_join_input(
        port,
        builder,
        current_bindings,
        condition,
        then_body,
        else_body,
        carrier_step_phis,
        loop_var,
        error_prefix,
        |builder, bindings, port, body| {
            lower_direct_body_input_with_policy(
                builder,
                bindings,
                port,
                body,
                carrier_step_phis,
                loop_var,
                error_prefix,
                fallback,
                skip_statement,
            )
        },
    )
}

fn body_has_only_loop_update_shapes<'input, P>(
    port: &P,
    body: &P::BodyInput<'input>,
    loop_var: &str,
) -> bool
where
    P: LoopPlanExpressionPortV1 + 'input,
{
    let statements = port.body_statements(body);
    let mut updated = std::collections::BTreeSet::new();
    let mut saw_exit = false;
    for (index, _) in statements.iter().enumerate() {
        let input = match port.body_stmt(body, index) {
            Ok(input) => input,
            Err(_) => return false,
        };
        let last = index + 1 == statements.len();
        match port.stmt_syntax(&input) {
            ASTNode::Assignment { target, value, .. } => {
                if saw_exit || !matches!(target.as_ref(), ASTNode::Variable { .. }) {
                    return false;
                }
                let ASTNode::Variable { name, .. } = target.as_ref() else {
                    return false;
                };
                if name == loop_var || !updated.insert(name.clone()) {
                    return false;
                }
                if !crate::mir::builder::control_flow::plan::facts::expr_generic_loop::is_pure_value_expr_for_generic_loop(value) {
                    return false;
                }
            }
            ASTNode::Break { .. } | ASTNode::Continue { .. } => {
                if !last || saw_exit {
                    return false;
                }
                saw_exit = true;
            }
            _ => return false,
        }
    }
    !updated.is_empty()
}
