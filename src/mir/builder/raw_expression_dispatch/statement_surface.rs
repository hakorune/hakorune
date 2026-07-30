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
use crate::mir::resolved_semantics::ExprChildRoleV1;
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
        value_source: crate::mir::builder::raw_structured_child_scope::PreparedRawChildSourceV1,
    },
    Field {
        prepared: PreparedRawFieldAssignmentV1,
        receiver_source: crate::mir::builder::raw_structured_child_scope::PreparedRawChildSourceV1,
        value_source: crate::mir::builder::raw_structured_child_scope::PreparedRawChildSourceV1,
    },
    Index {
        prepared: PreparedRawIndexAssignmentV1,
        target_source: crate::mir::builder::raw_structured_child_scope::PreparedRawChildSourceV1,
        index_source: crate::mir::builder::raw_structured_child_scope::PreparedRawChildSourceV1,
        value_source: crate::mir::builder::raw_structured_child_scope::PreparedRawChildSourceV1,
    },
    Unsupported,
}

impl PreparedRawOrdinaryAssignmentV1 {
    fn prepare(
        builder: &MirBuilder,
        statement: AssignStmt,
        sources: Option<PreparedRawOrdinaryAssignmentSourcesV1>,
    ) -> Result<Self, String> {
        let AssignStmt { target, value, .. } = statement;
        let value = *value;
        let route = match *target {
            ASTNode::Variable { name, .. } => PreparedRawOrdinaryAssignmentRouteV1::Variable {
                input: RawLegacyVariableAssignmentInputV1::new(name, value),
                value_source: match sources {
                    Some(PreparedRawOrdinaryAssignmentSourcesV1::Variable { value }) => value,
                    _ => {
                        return Err(
                            "[freeze:contract][raw-assignment/missing-variable-source]".to_owned()
                        )
                    }
                },
            },
            ASTNode::FieldAccess { object, field, .. } => {
                let (receiver_source, value_source) = match sources {
                    Some(PreparedRawOrdinaryAssignmentSourcesV1::Field { receiver, value }) => {
                        (receiver, value)
                    }
                    _ => {
                        return Err(
                            "[freeze:contract][raw-assignment/missing-field-source]".to_owned()
                        )
                    }
                };
                PreparedRawOrdinaryAssignmentRouteV1::Field {
                    prepared: PreparedRawFieldAssignmentV1::prepare(
                        builder, *object, field, value,
                    )?,
                    receiver_source,
                    value_source,
                }
            }
            ASTNode::Index { target, index, .. } => {
                let (target_source, index_source, value_source) = match sources {
                    Some(PreparedRawOrdinaryAssignmentSourcesV1::Index {
                        target,
                        index,
                        value,
                    }) => (target, index, value),
                    _ => {
                        return Err(
                            "[freeze:contract][raw-assignment/missing-index-source]".to_owned()
                        )
                    }
                };
                PreparedRawOrdinaryAssignmentRouteV1::Index {
                    prepared: PreparedRawIndexAssignmentV1::prepare(*target, *index, value),
                    target_source,
                    index_source,
                    value_source,
                }
            }
            _ => PreparedRawOrdinaryAssignmentRouteV1::Unsupported,
        };
        Ok(Self { route })
    }
}

enum PreparedRawOrdinaryAssignmentSourcesV1 {
    Variable {
        value: crate::mir::builder::raw_structured_child_scope::PreparedRawChildSourceV1,
    },
    Field {
        receiver: crate::mir::builder::raw_structured_child_scope::PreparedRawChildSourceV1,
        value: crate::mir::builder::raw_structured_child_scope::PreparedRawChildSourceV1,
    },
    Index {
        target: crate::mir::builder::raw_structured_child_scope::PreparedRawChildSourceV1,
        index: crate::mir::builder::raw_structured_child_scope::PreparedRawChildSourceV1,
        value: crate::mir::builder::raw_structured_child_scope::PreparedRawChildSourceV1,
    },
}

fn with_write_target_source_v1<Port, Output>(
    port: &mut Port,
    statement: &ASTNode,
    target_role: ExprChildRoleV1,
    project: impl FnOnce(&mut Port) -> Result<Output, String>,
) -> Result<Output, String>
where
    Port: RawExpressionDispatchPortV1,
{
    let target_source = port.prepare_expression_child_source_v1(statement, target_role)?;
    port.with_prepared_child_source_v1(target_source, project)
}

fn prepare_field_write_child_sources_v1<Port>(
    port: &mut Port,
    statement: &ASTNode,
    target: &ASTNode,
    target_role: ExprChildRoleV1,
    value_role: ExprChildRoleV1,
) -> Result<
    (
        crate::mir::builder::raw_structured_child_scope::PreparedRawChildSourceV1,
        crate::mir::builder::raw_structured_child_scope::PreparedRawChildSourceV1,
    ),
    String,
>
where
    Port: RawExpressionDispatchPortV1,
{
    let receiver = with_write_target_source_v1(port, statement, target_role, |port| {
        port.prepare_expression_child_source_v1(target, ExprChildRoleV1::Receiver)
    })?;
    let value = port.prepare_expression_child_source_v1(statement, value_role)?;
    Ok((receiver, value))
}

fn prepare_index_write_child_sources_v1<Port>(
    port: &mut Port,
    statement: &ASTNode,
    target: &ASTNode,
    target_role: ExprChildRoleV1,
    value_role: ExprChildRoleV1,
) -> Result<
    (
        crate::mir::builder::raw_structured_child_scope::PreparedRawChildSourceV1,
        crate::mir::builder::raw_structured_child_scope::PreparedRawChildSourceV1,
        crate::mir::builder::raw_structured_child_scope::PreparedRawChildSourceV1,
    ),
    String,
>
where
    Port: RawExpressionDispatchPortV1,
{
    let (index_target, index_subscript) =
        with_write_target_source_v1(port, statement, target_role, |port| {
            Ok::<_, String>((
                port.prepare_expression_child_source_v1(target, ExprChildRoleV1::IndexTarget)?,
                port.prepare_expression_child_source_v1(target, ExprChildRoleV1::IndexSubscript)?,
            ))
        })?;
    let value = port.prepare_expression_child_source_v1(statement, value_role)?;
    Ok((index_target, index_subscript, value))
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
        PreparedRawOrdinaryAssignmentRouteV1::Variable {
            input,
            value_source,
        } => {
            let mut scoped =
                RawStructuredChildScopePortV1::new(port, vec![value_source], Vec::new());
            let value = drive_variable_assignment_v1(builder, &mut scoped, &input)?;
            scoped.complete_exact_demands_v1()?;
            Ok(value)
        }
        PreparedRawOrdinaryAssignmentRouteV1::Field {
            prepared,
            receiver_source,
            value_source,
        } => {
            let mut scoped = RawStructuredChildScopePortV1::new(
                port,
                vec![receiver_source, value_source],
                Vec::new(),
            );
            let value =
                lower_prepared_raw_field_assignment_with_port_v1(builder, &mut scoped, prepared)?;
            scoped.complete_exact_demands_v1()?;
            Ok(value)
        }
        PreparedRawOrdinaryAssignmentRouteV1::Index {
            prepared,
            target_source,
            index_source,
            value_source,
        } => {
            let mut scoped = RawStructuredChildScopePortV1::new(
                port,
                vec![target_source, index_source, value_source],
                Vec::new(),
            );
            let value =
                lower_prepared_raw_index_assignment_with_port_v1(builder, &mut scoped, prepared)?;
            scoped.complete_exact_demands_v1()?;
            Ok(value)
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
        node @ ASTNode::Print { .. } => {
            Ok(StatementSurfaceDispatch::Lowered(
                crate::mir::builder::stmts::print_stmt::lower_raw_print_statement_with_port_v1(
                builder,
                port,
                    node,
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
            let sources = match &node {
                ASTNode::Assignment { target, .. }
                    if matches!(target.as_ref(), ASTNode::Variable { .. }) =>
                {
                    Some(PreparedRawOrdinaryAssignmentSourcesV1::Variable {
                        value: port.prepare_expression_child_source_v1(
                            &node,
                            ExprChildRoleV1::AssignmentValue,
                        )?,
                    })
                }
                ASTNode::Assignment { target, .. }
                    if matches!(target.as_ref(), ASTNode::FieldAccess { .. }) =>
                {
                    let (receiver, value) = prepare_field_write_child_sources_v1(
                        port,
                        &node,
                        target,
                        ExprChildRoleV1::AssignmentTarget,
                        ExprChildRoleV1::AssignmentValue,
                    )?;
                    Some(PreparedRawOrdinaryAssignmentSourcesV1::Field { receiver, value })
                }
                ASTNode::Assignment { target, .. }
                    if matches!(target.as_ref(), ASTNode::Index { .. }) =>
                {
                    let (index_target, index_subscript, value) =
                        prepare_index_write_child_sources_v1(
                            port,
                            &node,
                            target,
                            ExprChildRoleV1::AssignmentTarget,
                            ExprChildRoleV1::AssignmentValue,
                        )?;
                    Some(PreparedRawOrdinaryAssignmentSourcesV1::Index {
                        target: index_target,
                        index: index_subscript,
                        value,
                    })
                }
                _ => None,
            };
            let statement = AssignStmt::try_from(node).expect("ASTNode::Assignment must convert");
            let prepared = PreparedRawOrdinaryAssignmentV1::prepare(builder, statement, sources)?;
            Ok(StatementSurfaceDispatch::Lowered(
                lower_prepared_raw_ordinary_assignment_with_port_v1(builder, port, prepared)?,
            ))
        }
        node @ ASTNode::CompoundAssignment { .. } => {
            let child_sources = match &node {
                ASTNode::CompoundAssignment { target, .. }
                    if matches!(target.as_ref(), ASTNode::Variable { .. }) =>
                {
                    Some(vec![port.prepare_expression_child_source_v1(
                        &node,
                        ExprChildRoleV1::CompoundAssignmentValue,
                    )?])
                }
                ASTNode::CompoundAssignment { target, .. }
                    if matches!(target.as_ref(), ASTNode::FieldAccess { .. }) =>
                {
                    let (receiver, value) = prepare_field_write_child_sources_v1(
                        port,
                        &node,
                        target,
                        ExprChildRoleV1::CompoundAssignmentTarget,
                        ExprChildRoleV1::CompoundAssignmentValue,
                    )?;
                    Some(vec![receiver, value])
                }
                ASTNode::CompoundAssignment { target, .. }
                    if matches!(target.as_ref(), ASTNode::Index { .. }) =>
                {
                    let (index_target, index_subscript, value) =
                        prepare_index_write_child_sources_v1(
                            port,
                            &node,
                            target,
                            ExprChildRoleV1::CompoundAssignmentTarget,
                            ExprChildRoleV1::CompoundAssignmentValue,
                        )?;
                    Some(vec![index_target, index_subscript, value])
                }
                _ => None,
            };
            let ASTNode::CompoundAssignment {
                target,
                operator,
                value,
                ..
            } = node
            else {
                unreachable!("matched CompoundAssignment")
            };
            let prepared =
                PreparedRawCompoundAssignmentV1::prepare(*target, operator, *value);
            let value = match child_sources {
                Some(sources) => {
                    let mut scoped =
                        RawStructuredChildScopePortV1::new(port, sources, Vec::new());
                    let value = lower_prepared_raw_compound_assignment_with_port_v1(
                        builder,
                        &mut scoped,
                        prepared,
                    )?;
                    scoped.complete_exact_demands_v1()?;
                    value
                }
                None => lower_prepared_raw_compound_assignment_with_port_v1(
                    builder, port, prepared,
                )?,
            };
            Ok(StatementSurfaceDispatch::Lowered(value))
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
