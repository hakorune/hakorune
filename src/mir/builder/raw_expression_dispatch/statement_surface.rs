//! Statement-surface branch of the one Raw expression dispatcher.
//!
//! This module owns the existing statement-versus-expression partition only.
//! It does not create another expression matcher, source navigation path, or
//! call terminal.

use crate::ast::{ASTNode, AssignStmt, ReturnStmt};
use crate::mir::builder::exprs_enum_match::PreparedRawScopeBoxV1;
use crate::mir::builder::recursive_child_lowering::drive_legacy_body_v1;
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
        object: ASTNode,
        field: String,
        value: ASTNode,
    },
    Index {
        target: ASTNode,
        index: ASTNode,
        value: ASTNode,
    },
    Unsupported,
}

impl PreparedRawOrdinaryAssignmentV1 {
    fn prepare(statement: AssignStmt) -> Self {
        let AssignStmt { target, value, .. } = statement;
        let value = *value;
        let route = match *target {
            ASTNode::Variable { name, .. } => PreparedRawOrdinaryAssignmentRouteV1::Variable {
                input: RawLegacyVariableAssignmentInputV1::new(name, value),
            },
            ASTNode::FieldAccess { object, field, .. } => {
                PreparedRawOrdinaryAssignmentRouteV1::Field {
                    object: *object,
                    field,
                    value,
                }
            }
            ASTNode::Index { target, index, .. } => PreparedRawOrdinaryAssignmentRouteV1::Index {
                target: *target,
                index: *index,
                value,
            },
            _ => PreparedRawOrdinaryAssignmentRouteV1::Unsupported,
        };
        Self { route }
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
        PreparedRawOrdinaryAssignmentRouteV1::Field {
            object,
            field,
            value,
        } => builder.build_field_assignment_with_port_v1(port, object, field, value),
        PreparedRawOrdinaryAssignmentRouteV1::Index {
            target,
            index,
            value,
        } => builder.build_index_assignment_with_port_v1(port, target, index, value),
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
        ASTNode::Program { statements, .. } => Ok(StatementSurfaceDispatch::Lowered(
            drive_legacy_body_v1(builder, port, statements)?,
        )),
        ASTNode::ScopeBox { body, .. } => Ok(StatementSurfaceDispatch::Lowered(
            builder.lower_prepared_raw_scopebox_with_port_v1(
                port,
                PreparedRawScopeBoxV1::prepare(body),
            )?,
        )),
        ASTNode::TaskScope {
            body,
            source_keyword,
            ..
        } => Ok(StatementSurfaceDispatch::Lowered(
            crate::mir::builder::stmts::task_scope_stmt::build_task_scope_statement_with_port_v1(
                builder,
                port,
                body.clone(),
                source_keyword.clone(),
            )?,
        )),
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
        ASTNode::If {
            condition,
            then_body,
            else_body,
            ..
        } => {
            use crate::ast::Span;
            let then_node = ASTNode::Program {
                statements: then_body,
                span: Span::unknown(),
            };
            let else_node = else_body.map(|body| ASTNode::Program {
                statements: body,
                span: Span::unknown(),
            });
            Ok(StatementSurfaceDispatch::Lowered(builder.cf_if_with_port_v1(
                port,
                *condition,
                then_node,
                else_node,
            )?))
        }
        ASTNode::Loop {
            condition, body, ..
        } => {
            if crate::config::env::builder_loopform_debug() {
                crate::runtime::get_global_ring0()
                    .log
                    .debug("[exprs.rs] statement surface Loop route matched");
            }
            Ok(StatementSurfaceDispatch::Lowered(
                port.lower_loop(builder, *condition, body)?,
            ))
        }
        ASTNode::TryCatch {
            try_body,
            catch_clauses,
            finally_body,
            ..
        } => Ok(StatementSurfaceDispatch::Lowered(
            crate::mir::builder::control_flow::exception::cf_try_catch_with_port_v1(
                builder,
                port,
                try_body,
                catch_clauses,
                finally_body,
            )?,
        )),
        ASTNode::Throw { expression, .. } => Ok(StatementSurfaceDispatch::Lowered(
            crate::mir::builder::control_flow::exception::cf_throw_with_port_v1(
                builder,
                port,
                *expression,
            )?,
        )),
        node @ ASTNode::Assignment { .. } => {
            let statement = AssignStmt::try_from(node).expect("ASTNode::Assignment must convert");
            let prepared = PreparedRawOrdinaryAssignmentV1::prepare(statement);
            Ok(StatementSurfaceDispatch::Lowered(
                lower_prepared_raw_ordinary_assignment_with_port_v1(builder, port, prepared)?,
            ))
        }
        ASTNode::CompoundAssignment {
            target,
            operator,
            value,
            ..
        } => Ok(StatementSurfaceDispatch::Lowered(
            builder.build_compound_assignment_statement_with_port_v1(
                port,
                *target,
                operator,
                *value,
            )?,
        )),
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
