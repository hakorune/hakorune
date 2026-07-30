//! Statement-surface branch of the one Raw expression dispatcher.
//!
//! This module owns the existing statement-versus-expression partition only.
//! It does not create another expression matcher, source navigation path, or
//! call terminal.

use crate::ast::{ASTNode, AssignStmt, ReturnStmt};
use crate::mir::builder::compound_assignment::{
    lower_prepared_raw_compound_assignment_with_port_v1, PreparedRawCompoundAssignmentV1,
};
use crate::mir::builder::control_flow::exception::{
    lower_prepared_raw_throw_with_port_v1, lower_prepared_raw_try_catch_with_port_v1,
    PreparedRawThrowV1, PreparedRawTryCatchV1,
};
use crate::mir::builder::exprs_enum_match::PreparedRawScopeBoxV1;
use crate::mir::builder::fields::{
    lower_prepared_raw_field_assignment_with_port_v1, PreparedRawFieldAssignmentV1,
};
use crate::mir::builder::indexing::{
    lower_prepared_raw_index_assignment_with_port_v1, PreparedRawIndexAssignmentV1,
};
use crate::mir::builder::raw_structured_child_scope::RawStructuredChildScopePortV1;
use crate::mir::builder::recursive_child_lowering::drive_legacy_body_v1;
use crate::mir::builder::stmts::task_scope_stmt::{
    lower_prepared_raw_task_scope_with_port_v1, PreparedRawTaskScopeV1,
};
use crate::mir::builder::stmts::{
    drive_local_statement_v1, drive_value_return_statement_v1, drive_variable_assignment_v1,
    RawLegacyLocalInputV1, RawLegacyValueReturnInputV1, RawLegacyVariableAssignmentInputV1,
};
use crate::mir::{MirBuilder, ValueId};

use super::RawExpressionDispatchPortV1;

pub(super) enum StatementSurfaceDispatch {
    Lowered(ValueId),
    RegularExpression(ASTNode),
}

struct PreparedRawOrdinaryAssignmentV1 {
    route: PreparedRawOrdinaryAssignmentRouteV1,
}

enum PreparedRawOrdinaryAssignmentRouteV1 {
    Variable {
        input: RawLegacyVariableAssignmentInputV1,
    },
    Field {
        prepared: PreparedRawFieldAssignmentV1,
    },
    Index {
        prepared: PreparedRawIndexAssignmentV1,
    },
    Unsupported,
}

impl PreparedRawOrdinaryAssignmentV1 {
    fn prepare(builder: &MirBuilder, statement: AssignStmt) -> Result<Self, String> {
        let AssignStmt { target, value, .. } = statement;
        let value = *value;
        let route = match *target {
            ASTNode::Variable { name, .. } => PreparedRawOrdinaryAssignmentRouteV1::Variable {
                input: RawLegacyVariableAssignmentInputV1::new(name, value),
            },
            ASTNode::FieldAccess { object, field, .. } => {
                PreparedRawOrdinaryAssignmentRouteV1::Field {
                    prepared: PreparedRawFieldAssignmentV1::prepare(
                        builder, *object, field, value,
                    )?,
                }
            }
            ASTNode::Index { target, index, .. } => PreparedRawOrdinaryAssignmentRouteV1::Index {
                prepared: PreparedRawIndexAssignmentV1::prepare(*target, *index, value),
            },
            _ => PreparedRawOrdinaryAssignmentRouteV1::Unsupported,
        };
        Ok(Self { route })
    }
}

fn lower_prepared_raw_ordinary_assignment_with_port_v1<Port>(
    builder: &mut MirBuilder,
    port: &mut Port,
    prepared: PreparedRawOrdinaryAssignmentV1,
) -> Result<ValueId, String>
where
    Port: RawExpressionDispatchPortV1,
{
    match prepared.route {
        PreparedRawOrdinaryAssignmentRouteV1::Variable { input } => {
            drive_variable_assignment_v1(builder, port, &input)
        }
        PreparedRawOrdinaryAssignmentRouteV1::Field { prepared } => {
            lower_prepared_raw_field_assignment_with_port_v1(builder, port, prepared)
        }
        PreparedRawOrdinaryAssignmentRouteV1::Index { prepared } => {
            lower_prepared_raw_index_assignment_with_port_v1(builder, port, prepared)
        }
        PreparedRawOrdinaryAssignmentRouteV1::Unsupported => {
            Err("Complex assignment targets not yet supported".to_string())
        }
    }
}

pub(super) fn try_build_with_port_v1<Port>(
    builder: &mut MirBuilder,
    port: &mut Port,
    ast: ASTNode,
) -> Result<StatementSurfaceDispatch, String>
where
    Port: RawExpressionDispatchPortV1,
{
    match ast {
        node @ ASTNode::Program { .. } => {
            let source = port.prepare_body_child_source_v1(
                &node,
                crate::mir::resolved_semantics::BodyChildRoleV1::ProgramBody,
            )?;
            let ASTNode::Program { statements, .. } = node else {
                unreachable!()
            };
            let mut scoped = RawStructuredChildScopePortV1::for_body(port, source);
            let result = drive_legacy_body_v1(builder, &mut scoped, statements)?;
            scoped.complete_exact_demands_v1()?;
            Ok(StatementSurfaceDispatch::Lowered(result))
        }
        node @ ASTNode::ScopeBox { .. } => {
            let source = port.prepare_body_child_source_v1(
                &node,
                crate::mir::resolved_semantics::BodyChildRoleV1::ScopeBody,
            )?;
            let ASTNode::ScopeBox { body, .. } = node else {
                unreachable!()
            };
            let mut scoped = crate::mir::builder::raw_structured_child_scope::
                RawStructuredChildScopePortV1::for_body(port, source);
            Ok(StatementSurfaceDispatch::Lowered(
                builder.lower_prepared_raw_scopebox_with_port_v1(
                    &mut scoped,
                    PreparedRawScopeBoxV1::prepare(body),
                )?,
            ))
        }
        node @ ASTNode::TaskScope { .. } => {
            let source = port.prepare_body_child_source_v1(
                &node,
                crate::mir::resolved_semantics::BodyChildRoleV1::TaskScopeBody,
            )?;
            let ASTNode::TaskScope {
                body,
                source_keyword,
                ..
            } = node
            else {
                unreachable!()
            };
            let prepared = PreparedRawTaskScopeV1::prepare(body, source_keyword)?;
            let mut scoped = crate::mir::builder::raw_structured_child_scope::
                RawStructuredChildScopePortV1::for_body(port, source);
            Ok(StatementSurfaceDispatch::Lowered(
                lower_prepared_raw_task_scope_with_port_v1(builder, &mut scoped, prepared)?,
            ))
        }
        ASTNode::ContextScope {
            source_keyword,
            name,
            ..
        } => Err(format!(
            "[freeze:contract][mir_builder/context_scope_lowering_missing] spelling={} name={} context propagation is owned by CONC-CONTEXT-002",
            source_keyword, name
        )),
        ASTNode::Print { expression, .. } => {
            let prepared =
                crate::mir::builder::stmts::print_stmt::PreparedRawPrintV1::prepare(*expression);
            Ok(StatementSurfaceDispatch::Lowered(
                crate::mir::builder::stmts::print_stmt::lower_prepared_raw_print_with_port_v1(
                builder,
                port,
                    prepared,
                )?,
            ))
        }
        node @ ASTNode::If { .. } => {
            use crate::ast::Span;
            use crate::mir::resolved_semantics::{BodyChildRoleV1, ExprChildRoleV1};
            let condition =
                port.prepare_expression_child_source_v1(&node, ExprChildRoleV1::IfCondition)?;
            let then_source =
                port.prepare_body_child_source_v1(&node, BodyChildRoleV1::IfThen)?;
            let else_source = match &node {
                ASTNode::If {
                    else_body: Some(_), ..
                } => Some(
                    port.prepare_body_child_source_v1(&node, BodyChildRoleV1::IfElse)?,
                ),
                _ => None,
            };
            let ASTNode::If {
                condition: condition_node,
                then_body,
                else_body,
                ..
            } = node
            else {
                unreachable!()
            };
            let then_node = ASTNode::Program {
                statements: then_body,
                span: Span::unknown(),
            };
            let else_node = else_body.map(|body| ASTNode::Program {
                statements: body,
                span: Span::unknown(),
            });
            let mut scoped =
                crate::mir::builder::raw_structured_child_scope::RawStructuredChildScopePortV1::new(
                    port,
                    vec![condition],
                    [Some(then_source), else_source]
                        .into_iter()
                        .flatten()
                        .collect(),
                );
            Ok(StatementSurfaceDispatch::Lowered(builder.cf_if_with_port_v1(
                &mut scoped,
                *condition_node,
                then_node,
                else_node,
            )?))
        }
        loop_node @ ASTNode::Loop { .. } => {
            if crate::config::env::builder_loopform_debug() {
                crate::runtime::get_global_ring0()
                    .log
                    .debug("[exprs.rs] statement surface Loop route matched");
            }
            Ok(StatementSurfaceDispatch::Lowered(
                port.lower_loop(builder, loop_node)?,
            ))
        }
        node @ ASTNode::TryCatch { .. } => {
            use crate::mir::resolved_semantics::BodyChildRoleV1;

            let try_source =
                port.prepare_body_child_source_v1(&node, BodyChildRoleV1::TryBody)?;
            let catch_source = match &node {
                ASTNode::TryCatch { catch_clauses, .. } if !catch_clauses.is_empty() => Some(
                    port.prepare_body_child_source_v1(
                        &node,
                        BodyChildRoleV1::FirstCatchBody,
                    )?,
                ),
                _ => None,
            };
            let cleanup_source = match &node {
                ASTNode::TryCatch {
                    finally_body: Some(_),
                    ..
                } => Some(
                    port.prepare_body_child_source_v1(
                        &node,
                        BodyChildRoleV1::CleanupBody,
                    )?,
                ),
                _ => None,
            };
            let ASTNode::TryCatch {
                try_body,
                catch_clauses,
                finally_body,
                ..
            } = node
            else {
                unreachable!("matched TryCatch")
            };
            let mut body_sources = vec![try_source];
            body_sources.extend(catch_source);
            body_sources.extend(cleanup_source);
            let mut scoped =
                RawStructuredChildScopePortV1::new(port, Vec::new(), body_sources);
            let prepared =
                PreparedRawTryCatchV1::prepare(try_body, catch_clauses, finally_body);
            let value =
                lower_prepared_raw_try_catch_with_port_v1(builder, &mut scoped, prepared)?;
            scoped.complete_exact_demands_v1()?;
            Ok(StatementSurfaceDispatch::Lowered(value))
        }
        ASTNode::Throw { expression, .. } => {
            let prepared = PreparedRawThrowV1::prepare(&builder.function_state, *expression)?;
            Ok(StatementSurfaceDispatch::Lowered(
                lower_prepared_raw_throw_with_port_v1(builder, port, prepared)?,
            ))
        }
        node @ ASTNode::Assignment { .. } => {
            let statement = AssignStmt::try_from(node).expect("ASTNode::Assignment must convert");
            let prepared = PreparedRawOrdinaryAssignmentV1::prepare(builder, statement)?;
            Ok(StatementSurfaceDispatch::Lowered(
                lower_prepared_raw_ordinary_assignment_with_port_v1(builder, port, prepared)?,
            ))
        }
        ASTNode::CompoundAssignment {
            target,
            operator,
            value,
            ..
        } => {
            let prepared =
                PreparedRawCompoundAssignmentV1::prepare(*target, operator, *value);
            Ok(StatementSurfaceDispatch::Lowered(
                lower_prepared_raw_compound_assignment_with_port_v1(builder, port, prepared)?,
            ))
        }
        node @ ASTNode::Return { .. } => {
            let statement = ReturnStmt::try_from(node).expect("ASTNode::Return must convert");
            Ok(StatementSurfaceDispatch::Lowered(
                build_return_with_port_v1(builder, port, statement)?,
            ))
        }
        ASTNode::Local {
            variables,
            initial_values,
            declared_type_names,
            ..
        } => Ok(StatementSurfaceDispatch::Lowered(drive_local_statement_v1(
            builder,
            port,
            &RawLegacyLocalInputV1::new(variables, initial_values, declared_type_names),
        )?)),
        ASTNode::Outbox { variables, .. } => Ok(StatementSurfaceDispatch::Lowered(
            crate::mir::builder::stmts::variable_stmt::build_outbox_statement(builder, variables.clone())?,
        )),
        ASTNode::Nowait {
            variable,
            expression,
            ..
        } => Ok(StatementSurfaceDispatch::Lowered(
            crate::mir::builder::stmts::async_stmt::build_nowait_statement_with_port_v1(
                builder,
                port,
                variable,
                *expression,
            )?,
        )),
        ASTNode::UsingStatement { .. } => Ok(StatementSurfaceDispatch::Lowered(
            crate::mir::builder::emission::constant::emit_void(builder)?,
        )),
        ast => Ok(StatementSurfaceDispatch::RegularExpression(ast)),
    }
}

fn build_return_with_port_v1<Port>(
    builder: &mut MirBuilder,
    port: &mut Port,
    statement: ReturnStmt,
) -> Result<ValueId, String>
where
    Port: RawExpressionDispatchPortV1,
{
    match statement.value {
        Some(value) => {
            let input = RawLegacyValueReturnInputV1::new(*value);
            drive_value_return_statement_v1(builder, port, &input)
        }
        None => crate::mir::builder::stmts::return_stmt::build_void_return_statement(builder),
    }
}
