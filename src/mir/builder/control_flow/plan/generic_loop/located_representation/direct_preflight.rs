//! Builder-free preflight for the first located `DirectRecipeOnly` body.
//!
//! The O0 representation owns mode, ordinals, cleanup, and extraction. This
//! module only proves that its exact prefix can be consumed by the shared
//! associated-input statement owner. It never rebuilds a body, recipe, or
//! policy and it never touches Builder state.

use std::collections::BTreeSet;

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::expression_port::{
    LocatedLoopPlanBodyInputV1, LocatedLoopPlanExpressionPortV1, LocatedLoopPlanStmtInputV1,
    LoopPlanExpressionPortErrorV1, LoopPlanExpressionPortV1,
};
use crate::mir::builder::control_flow::plan::generic_loop::located_representation::{
    VerifiedLocatedDirectBodyLoweringViewV1, VerifiedLocatedGenericLoopLoweringModeV1,
    VerifiedLocatedGenericLoopLoweringViewV1,
};
use crate::mir::resolved_semantics::{BodyChildRoleV1, ExprChildRoleV1};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) enum LocatedGenericLoopDirectPreflightErrorV1 {
    WrongLoweringMode,
    UnsupportedStatement { path: String },
    LocalShape { path: String },
    AssignmentTarget { path: String },
    ReturnValue { path: String },
    Port(LoopPlanExpressionPortErrorV1),
}

impl From<LoopPlanExpressionPortErrorV1> for LocatedGenericLoopDirectPreflightErrorV1 {
    fn from(value: LoopPlanExpressionPortErrorV1) -> Self {
        Self::Port(value)
    }
}

pub(in crate::mir::builder) struct VerifiedLocatedGenericLoopDirectPreflightV1<'seal, 'view, 'plan>
{
    lowering: &'seal VerifiedLocatedGenericLoopLoweringViewV1<'view, 'plan>,
    carrier_targets: Box<[String]>,
    _seal: DirectPreflightSealV1,
}

pub(in crate::mir::builder) struct PreparedLocatedGenericLoopDirectExecutionV1<'seal, 'view, 'plan>
{
    lowering: &'seal VerifiedLocatedGenericLoopLoweringViewV1<'view, 'plan>,
    carrier_targets: Box<[String]>,
    _seal: DirectExecutionSealV1,
}

struct DirectPreflightSealV1;
struct DirectExecutionSealV1;

impl<'seal, 'view, 'plan> VerifiedLocatedGenericLoopDirectPreflightV1<'seal, 'view, 'plan> {
    pub(in crate::mir::builder) fn verify(
        lowering: &'seal VerifiedLocatedGenericLoopLoweringViewV1<'view, 'plan>,
    ) -> Result<Self, LocatedGenericLoopDirectPreflightErrorV1> {
        let VerifiedLocatedGenericLoopLoweringModeV1::DirectRecipeOnly { body } = lowering.mode()
        else {
            return Err(LocatedGenericLoopDirectPreflightErrorV1::WrongLoweringMode);
        };
        let targets = verify_direct_body(&body)?;
        Ok(Self {
            lowering,
            carrier_targets: targets.into_iter().collect::<Vec<_>>().into_boxed_slice(),
            _seal: DirectPreflightSealV1,
        })
    }

    pub(in crate::mir::builder) fn into_execution(
        self,
    ) -> PreparedLocatedGenericLoopDirectExecutionV1<'seal, 'view, 'plan> {
        PreparedLocatedGenericLoopDirectExecutionV1 {
            lowering: self.lowering,
            carrier_targets: self.carrier_targets,
            _seal: DirectExecutionSealV1,
        }
    }
}

impl<'seal, 'view, 'plan> PreparedLocatedGenericLoopDirectExecutionV1<'seal, 'view, 'plan> {
    pub(in crate::mir::builder) fn into_components(
        self,
    ) -> (
        &'seal VerifiedLocatedGenericLoopLoweringViewV1<'view, 'plan>,
        Box<[String]>,
    ) {
        (self.lowering, self.carrier_targets)
    }
}

fn verify_direct_body<'view, 'plan>(
    body: &VerifiedLocatedDirectBodyLoweringViewV1<'view, 'plan>,
) -> Result<BTreeSet<String>, LocatedGenericLoopDirectPreflightErrorV1> {
    let port = body.expression_port();
    let mut targets = BTreeSet::new();
    for index in 0..body.len() {
        let statement = body.statement(index).ok_or_else(|| {
            LocatedGenericLoopDirectPreflightErrorV1::UnsupportedStatement {
                path: format!("prefix[{index}]"),
            }
        })?;
        verify_statement(port, statement, &format!("prefix[{index}]"), &mut targets)?;
    }
    Ok(targets)
}

fn verify_body_input<'view, 'plan>(
    port: &LocatedLoopPlanExpressionPortV1<'plan>,
    body: LocatedLoopPlanBodyInputV1<'plan, 'view>,
    path: &str,
    targets: &mut BTreeSet<String>,
) -> Result<(), LocatedGenericLoopDirectPreflightErrorV1> {
    for index in 0..port.body_statements(&body).len() {
        let statement = port.body_stmt(&body, index)?;
        verify_statement(port, statement, &format!("{path}[{index}]"), targets)?;
    }
    Ok(())
}

fn verify_statement<'view, 'plan>(
    port: &LocatedLoopPlanExpressionPortV1<'plan>,
    statement: LocatedLoopPlanStmtInputV1<'plan, 'view>,
    path: &str,
    targets: &mut BTreeSet<String>,
) -> Result<(), LocatedGenericLoopDirectPreflightErrorV1> {
    match port.stmt_syntax(&statement) {
        ASTNode::Local {
            variables,
            initial_values,
            declared_type_names,
            ..
        } if variables.len() == 1
            && initial_values.len() == 1
            && declared_type_names.len() == 1
            && declared_type_names[0].is_none()
            && initial_values[0].is_some() =>
        {
            port.child_expr_from_stmt(&statement, ExprChildRoleV1::LocalInitializer(0))?;
            Ok(())
        }
        ASTNode::Assignment { .. } => {
            let target =
                port.child_expr_from_stmt(&statement, ExprChildRoleV1::AssignmentTarget)?;
            let value = port.child_expr_from_stmt(&statement, ExprChildRoleV1::AssignmentValue)?;
            let ASTNode::Variable { name, .. } = port.expr_syntax(&target) else {
                return Err(LocatedGenericLoopDirectPreflightErrorV1::AssignmentTarget {
                    path: path.to_string(),
                });
            };
            targets.insert(name.clone());
            let _ = port.expr_syntax(&value);
            Ok(())
        }
        ASTNode::Return { value: Some(_), .. } => {
            port.child_expr_from_stmt(&statement, ExprChildRoleV1::ReturnValue)?;
            Ok(())
        }
        ASTNode::If { else_body, .. } => {
            port.child_expr_from_stmt(&statement, ExprChildRoleV1::IfCondition)?;
            let then_body = port.child_body_from_stmt(&statement, BodyChildRoleV1::IfThen)?;
            verify_body_input(port, then_body, &format!("{path}.then"), targets)?;
            if else_body.is_some() {
                let else_body = port.child_body_from_stmt(&statement, BodyChildRoleV1::IfElse)?;
                verify_body_input(port, else_body, &format!("{path}.else"), targets)?;
            }
            Ok(())
        }
        ASTNode::Local { .. } => Err(LocatedGenericLoopDirectPreflightErrorV1::LocalShape {
            path: path.to_string(),
        }),
        ASTNode::Return { .. } => Err(LocatedGenericLoopDirectPreflightErrorV1::ReturnValue {
            path: path.to_string(),
        }),
        _ => Err(
            LocatedGenericLoopDirectPreflightErrorV1::UnsupportedStatement {
                path: path.to_string(),
            },
        ),
    }
}

#[cfg(test)]
#[path = "direct_preflight_tests.rs"]
mod tests;
