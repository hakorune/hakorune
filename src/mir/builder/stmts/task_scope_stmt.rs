//! Structured task-scope lowering.
//!
//! `co { ... }` / compatibility `task_scope { ... }` are lexical ownership
//! boundaries. Runtime task-group truth remains in `runtime::global_hooks`;
//! MIRBuilder only places enter/exit calls.

use super::super::{MirBuilder, ValueId};
use crate::ast::ASTNode;
use crate::mir::builder::recursive_child_lowering::{
    drive_legacy_body_v1, RecursiveChildLoweringPortV1,
};

const EARLY_EXIT_TAG: &str = "[freeze:contract][co/early-exit-unsupported]";

pub(in crate::mir::builder) fn build_task_scope_statement(
    builder: &mut MirBuilder,
    body: Vec<ASTNode>,
    source_keyword: String,
) -> Result<ValueId, String> {
    if let Some(reason) = first_unsupported_early_exit(&body) {
        return Err(format!(
            "{} spelling={} reason={} CONC-CO-MIR-001 v0 is normal-completion-only; scope-exit cleanup lowering is owned by CONC-CO-MIR-002",
            EARLY_EXIT_TAG, source_keyword, reason
        ));
    }

    builder.emit_extern_call("env.task_scope", "push", Vec::new(), None)?;
    let result = builder.cf_block(body)?;
    builder.emit_extern_call("env.task_scope", "pop", Vec::new(), None)?;
    Ok(result)
}

/// Port-aware task scope body driver.  Scope enter/exit stays here; every
/// child statement is delegated to the caller's recursive capability.
pub(in crate::mir::builder) fn build_task_scope_statement_with_port_v1<Port>(
    builder: &mut MirBuilder,
    child: &mut Port,
    body: Vec<ASTNode>,
    source_keyword: String,
) -> Result<ValueId, String>
where
    Port: RecursiveChildLoweringPortV1<BodyInput = Vec<ASTNode>, StatementInput = ASTNode>,
{
    if let Some(reason) = first_unsupported_early_exit(&body) {
        return Err(format!(
            "{} spelling={} reason={} CONC-CO-MIR-001 v0 is normal-completion-only; scope-exit cleanup lowering is owned by CONC-CO-MIR-002",
            EARLY_EXIT_TAG, source_keyword, reason
        ));
    }

    builder.emit_extern_call("env.task_scope", "push", Vec::new(), None)?;
    let result = drive_legacy_body_v1(builder, child, body);
    builder.emit_extern_call("env.task_scope", "pop", Vec::new(), None)?;
    result
}

fn first_unsupported_early_exit(statements: &[ASTNode]) -> Option<&'static str> {
    statements
        .iter()
        .find_map(|stmt| first_unsupported_early_exit_in_node(stmt, 0))
}

fn first_unsupported_early_exit_in_node(node: &ASTNode, loop_depth: usize) -> Option<&'static str> {
    match node {
        ASTNode::Return { .. } => Some("return"),
        ASTNode::Throw { .. } => Some("throw"),
        ASTNode::Break { .. } if loop_depth == 0 => Some("break"),
        ASTNode::Continue { .. } if loop_depth == 0 => Some("continue"),
        ASTNode::If {
            condition,
            then_body,
            else_body,
            ..
        } => first_unsupported_early_exit_in_node(condition, loop_depth)
            .or_else(|| first_unsupported_early_exit_in_block(then_body, loop_depth))
            .or_else(|| {
                else_body
                    .as_deref()
                    .and_then(|body| first_unsupported_early_exit_in_block(body, loop_depth))
            }),
        ASTNode::Loop {
            condition, body, ..
        } => first_unsupported_early_exit_in_node(condition, loop_depth)
            .or_else(|| first_unsupported_early_exit_in_block(body, loop_depth + 1)),
        ASTNode::TryCatch {
            try_body,
            catch_clauses,
            finally_body,
            ..
        } => first_unsupported_early_exit_in_block(try_body, loop_depth)
            .or_else(|| {
                catch_clauses.iter().find_map(|clause| {
                    first_unsupported_early_exit_in_block(&clause.body, loop_depth)
                })
            })
            .or_else(|| {
                finally_body
                    .as_deref()
                    .and_then(|body| first_unsupported_early_exit_in_block(body, loop_depth))
            }),
        ASTNode::TaskScope { body, .. }
        | ASTNode::ScopeBox { body, .. }
        | ASTNode::FastMemRegion { body, .. }
        | ASTNode::Program {
            statements: body, ..
        } => first_unsupported_early_exit_in_block(body, loop_depth),
        ASTNode::ContextScope { value, body, .. } => {
            first_unsupported_early_exit_in_node(value, loop_depth)
                .or_else(|| first_unsupported_early_exit_in_block(body, loop_depth))
        }
        ASTNode::Nowait { expression, .. }
        | ASTNode::AwaitExpression { expression, .. }
        | ASTNode::Print { expression, .. } => {
            first_unsupported_early_exit_in_node(expression, loop_depth)
        }
        ASTNode::Local { initial_values, .. } | ASTNode::Outbox { initial_values, .. } => {
            initial_values.iter().find_map(|value| {
                value
                    .as_deref()
                    .and_then(|node| first_unsupported_early_exit_in_node(node, loop_depth))
            })
        }
        ASTNode::Assignment { target, value, .. } => {
            first_unsupported_early_exit_in_node(target, loop_depth)
                .or_else(|| first_unsupported_early_exit_in_node(value, loop_depth))
        }
        ASTNode::BinaryOp { left, right, .. } => {
            first_unsupported_early_exit_in_node(left, loop_depth)
                .or_else(|| first_unsupported_early_exit_in_node(right, loop_depth))
        }
        ASTNode::UnaryOp { operand, .. }
        | ASTNode::QMarkPropagate {
            expression: operand,
            ..
        } => first_unsupported_early_exit_in_node(operand, loop_depth),
        ASTNode::MethodCall {
            object, arguments, ..
        } => first_unsupported_early_exit_in_node(object, loop_depth).or_else(|| {
            arguments
                .iter()
                .find_map(|arg| first_unsupported_early_exit_in_node(arg, loop_depth))
        }),
        ASTNode::FunctionCall { arguments, .. }
        | ASTNode::New { arguments, .. }
        | ASTNode::ArrayLiteral {
            elements: arguments,
            ..
        } => arguments
            .iter()
            .find_map(|arg| first_unsupported_early_exit_in_node(arg, loop_depth)),
        ASTNode::MapLiteral { entries, .. } => entries
            .iter()
            .find_map(|(_, value)| first_unsupported_early_exit_in_node(value, loop_depth)),
        ASTNode::Lambda { .. } => None,
        _ => None,
    }
}

fn first_unsupported_early_exit_in_block(
    statements: &[ASTNode],
    loop_depth: usize,
) -> Option<&'static str> {
    statements
        .iter()
        .find_map(|stmt| first_unsupported_early_exit_in_node(stmt, loop_depth))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{LiteralValue, Span};
    use crate::mir::builder::recursive_child_lowering::RecursiveChildLoweringPortV1;
    use crate::mir::MirInstruction;

    #[derive(Default)]
    struct RecordingTaskScopePortV1 {
        seen: Vec<i64>,
        fail_at: Option<i64>,
    }

    impl RecursiveChildLoweringPortV1 for RecordingTaskScopePortV1 {
        type BodyInput = Vec<ASTNode>;
        type StatementInput = ASTNode;
        type ExpressionInput = ASTNode;

        fn lower_body(
            &mut self,
            builder: &mut MirBuilder,
            input: Self::BodyInput,
        ) -> Result<ValueId, String> {
            let mut last = None;
            for statement in input {
                last = Some(self.lower_statement(builder, statement)?);
            }
            last.ok_or_else(|| "task-scope test body must be non-empty".to_owned())
        }

        fn lower_statement(
            &mut self,
            builder: &mut MirBuilder,
            input: Self::StatementInput,
        ) -> Result<ValueId, String> {
            match input {
                ASTNode::Literal {
                    value: LiteralValue::Integer(value),
                    ..
                } => {
                    self.seen.push(value);
                    if self.fail_at == Some(value) {
                        return Err(format!("task-scope child {value} failed"));
                    }
                    crate::mir::builder::emission::constant::emit_integer(builder, value)
                }
                ASTNode::TaskScope {
                    body,
                    source_keyword,
                    ..
                } => build_task_scope_statement_with_port_v1(builder, self, body, source_keyword),
                other => Err(format!(
                    "task-scope test port received {}",
                    other.node_type()
                )),
            }
        }

        fn lower_expression(
            &mut self,
            builder: &mut MirBuilder,
            input: Self::ExpressionInput,
        ) -> Result<ValueId, String> {
            self.lower_statement(builder, input)
        }
    }

    fn integer(value: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: Span::unknown(),
        }
    }

    fn task_scope(body: Vec<ASTNode>) -> ASTNode {
        ASTNode::TaskScope {
            body,
            source_keyword: "co".to_owned(),
            span: Span::unknown(),
        }
    }

    fn task_scope_calls(builder: &MirBuilder) -> Vec<String> {
        builder
            .function_state
            .current_function
            .as_ref()
            .expect("current test function")
            .blocks
            .values()
            .flat_map(|block| block.instructions.iter())
            .filter_map(|instruction| match instruction {
                MirInstruction::Call { .. } => {
                    let rendered = format!("{instruction:?}");
                    rendered.contains("env.task_scope").then_some(rendered)
                }
                _ => None,
            })
            .collect()
    }

    #[test]
    fn task_scope_drives_children_in_source_order_through_supplied_port() {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("task_scope_owner_order/0".to_owned());
        let mut port = RecordingTaskScopePortV1::default();

        build_task_scope_statement_with_port_v1(
            &mut builder,
            &mut port,
            vec![
                integer(1),
                task_scope(vec![integer(2), integer(3)]),
                integer(4),
            ],
            "co".to_owned(),
        )
        .unwrap();

        assert_eq!(port.seen, vec![1, 2, 3, 4]);
        let calls = task_scope_calls(&builder);
        assert_eq!(calls.len(), 4, "outer and nested scopes each push and pop");
        assert!(calls[0].contains("push"));
        assert!(calls[1].contains("push"));
        assert!(calls[2].contains("pop"));
        assert!(calls[3].contains("pop"));
    }

    #[test]
    fn task_scope_child_failure_still_emits_pop_once_and_stops_later_children() {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("task_scope_owner_failure/0".to_owned());
        let mut port = RecordingTaskScopePortV1 {
            fail_at: Some(2),
            ..Default::default()
        };

        let error = build_task_scope_statement_with_port_v1(
            &mut builder,
            &mut port,
            vec![integer(1), integer(2), integer(3)],
            "co".to_owned(),
        )
        .unwrap_err();

        assert_eq!(error, "task-scope child 2 failed");
        assert_eq!(port.seen, vec![1, 2]);
        let calls = task_scope_calls(&builder);
        assert_eq!(calls.len(), 2, "failed scope must still push and pop once");
        assert!(calls[0].contains("push"));
        assert!(calls[1].contains("pop"));
    }
}
