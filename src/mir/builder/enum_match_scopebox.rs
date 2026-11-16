//! ScopeBox route preparation kept independent from enum-match policy.

use crate::ast::ASTNode;
use crate::mir::builder::recursive_child_lowering::{
    drive_legacy_body_v1, drive_legacy_statement_v1, RawAstChildLoweringPortV1,
};
use crate::mir::ValueId;

pub(in crate::mir::builder) struct PreparedRawScopeBoxV1 {
    pub(in crate::mir::builder) route: PreparedRawScopeBoxRouteV1,
}

pub(in crate::mir::builder) enum PreparedRawScopeBoxRouteV1 {
    GuardLet {
        body: Vec<ASTNode>,
        temp_name: String,
    },
    Ordinary {
        body: Vec<ASTNode>,
    },
}

impl PreparedRawScopeBoxV1 {
    pub(in crate::mir::builder) fn prepare(body: Vec<ASTNode>) -> Self {
        let route = match guard_let_scopebox_subject(&body) {
            Some(temp_name) => PreparedRawScopeBoxRouteV1::GuardLet { body, temp_name },
            None => PreparedRawScopeBoxRouteV1::Ordinary { body },
        };
        Self { route }
    }
}

impl super::MirBuilder {
    pub(in crate::mir::builder) fn lower_prepared_raw_scopebox_with_port_v1<Port>(
        &mut self,
        port: &mut Port,
        prepared: PreparedRawScopeBoxV1,
    ) -> Result<ValueId, String>
    where
        Port: RawAstChildLoweringPortV1,
    {
        let (body, temp_name) = match prepared.route {
            PreparedRawScopeBoxRouteV1::Ordinary { body } => {
                return drive_legacy_body_v1(self, port, body)
            }
            PreparedRawScopeBoxRouteV1::GuardLet { body, temp_name } => (body, temp_name),
        };
        let mut last_value = None;
        for stmt in body {
            last_value = Some(drive_legacy_statement_v1(self, port, stmt)?);
        }
        self.function_state.variable_ctx.remove(&temp_name);
        self.function_state.binding_ctx.remove(&temp_name);
        Ok(last_value.unwrap_or_else(|| self.next_value_id()))
    }
}

fn guard_let_scopebox_subject(body: &[ASTNode]) -> Option<String> {
    let [subject_local, failure_if, binding_local] = body else {
        return None;
    };
    let temp_name = guard_let_subject_temp_name(subject_local)?;
    guard_let_failure_if_uses_temp(failure_if, &temp_name)
        .then(|| guard_let_binding_local_uses_temp(binding_local, &temp_name))
        .filter(|accepted| *accepted)
        .map(|_| temp_name)
}

fn guard_let_subject_temp_name(node: &ASTNode) -> Option<String> {
    let ASTNode::Local {
        variables,
        initial_values,
        ..
    } = node
    else {
        return None;
    };
    (variables.len() == 1 && initial_values.len() == 1 && initial_values[0].is_some())
        .then(|| variables[0].as_str())
        .filter(|name| name.starts_with("__ny_guard_let_subject_"))
        .map(str::to_owned)
}

fn guard_let_failure_if_uses_temp(node: &ASTNode, temp_name: &str) -> bool {
    matches!(node, ASTNode::If { condition, .. } if enum_match_scrutinee_is_temp(condition, temp_name))
}

fn guard_let_binding_local_uses_temp(node: &ASTNode, temp_name: &str) -> bool {
    let ASTNode::Local {
        variables,
        initial_values,
        ..
    } = node
    else {
        return false;
    };
    variables.len() == 1
        && initial_values.len() == 1
        && initial_values[0]
            .as_deref()
            .is_some_and(|value| enum_match_scrutinee_is_temp(value, temp_name))
}

fn enum_match_scrutinee_is_temp(node: &ASTNode, temp_name: &str) -> bool {
    matches!(node, ASTNode::EnumMatchExpr { scrutinee, .. } if matches!(scrutinee.as_ref(), ASTNode::Variable { name, .. } if name == temp_name))
}

#[cfg(test)]
mod raw_scopebox_route_tests {
    use super::*;
    use crate::ast::{LiteralValue, Span};
    use crate::mir::builder::recursive_child_lowering::RecursiveChildLoweringPortV1;
    use crate::mir::{BindingId, MirBuilder, ValueId};

    struct RecordingPort {
        body_calls: usize,
        statement_calls: usize,
        fail_statement: Option<usize>,
    }

    impl RecursiveChildLoweringPortV1 for RecordingPort {
        type BodyInput = Vec<ASTNode>;
        type StatementInput = ASTNode;
        type ExpressionInput = ASTNode;

        fn lower_body(
            &mut self,
            _builder: &mut MirBuilder,
            _input: Self::BodyInput,
        ) -> Result<ValueId, String> {
            self.body_calls += 1;
            Ok(ValueId::new(90))
        }

        fn lower_statement(
            &mut self,
            _builder: &mut MirBuilder,
            _input: Self::StatementInput,
        ) -> Result<ValueId, String> {
            self.statement_calls += 1;
            if self.fail_statement == Some(self.statement_calls) {
                return Err("scopebox child failure".to_string());
            }
            Ok(ValueId::new(self.statement_calls as u32))
        }

        fn lower_expression(
            &mut self,
            _builder: &mut MirBuilder,
            _input: Self::ExpressionInput,
        ) -> Result<ValueId, String> {
            unreachable!("scopebox route test lowers statements or a body")
        }
    }

    fn variable(name: &str) -> ASTNode {
        ASTNode::Variable {
            name: name.to_string(),
            span: Span::unknown(),
        }
    }

    fn enum_match(temp_name: &str) -> ASTNode {
        ASTNode::EnumMatchExpr {
            enum_name: "Result".to_string(),
            scrutinee: Box::new(variable(temp_name)),
            arms: vec![],
            else_expr: None,
            span: Span::unknown(),
        }
    }

    fn local(name: &str, value: ASTNode) -> ASTNode {
        ASTNode::Local {
            variables: vec![name.to_string()],
            initial_values: vec![Some(Box::new(value))],
            declared_type_names: vec![None],
            span: Span::unknown(),
        }
    }

    fn guard_let_body(temp_name: &str) -> Vec<ASTNode> {
        vec![
            local(
                temp_name,
                ASTNode::Literal {
                    value: LiteralValue::Integer(1),
                    span: Span::unknown(),
                },
            ),
            ASTNode::If {
                condition: Box::new(enum_match(temp_name)),
                then_body: vec![],
                else_body: None,
                span: Span::unknown(),
            },
            local("value", enum_match(temp_name)),
        ]
    }

    #[test]
    fn raw_scopebox_route_is_disjoint_before_lowering() {
        let temp_name = "__ny_guard_let_subject_0";
        let guard = PreparedRawScopeBoxV1::prepare(guard_let_body(temp_name));
        assert!(matches!(
            guard.route,
            PreparedRawScopeBoxRouteV1::GuardLet { .. }
        ));

        let ordinary = PreparedRawScopeBoxV1::prepare(vec![local(
            "value",
            ASTNode::Literal {
                value: LiteralValue::Integer(1),
                span: Span::unknown(),
            },
        )]);
        assert!(matches!(
            ordinary.route,
            PreparedRawScopeBoxRouteV1::Ordinary { .. }
        ));

        let mut builder = MirBuilder::new();
        let mut port = RecordingPort {
            body_calls: 0,
            statement_calls: 0,
            fail_statement: None,
        };
        let ordinary = PreparedRawScopeBoxV1::prepare(vec![local(
            "value",
            ASTNode::Literal {
                value: LiteralValue::Integer(2),
                span: Span::unknown(),
            },
        )]);
        let result = builder
            .lower_prepared_raw_scopebox_with_port_v1(&mut port, ordinary)
            .expect("ordinary ScopeBox must use the body terminal");
        assert_eq!(result, ValueId::new(90));
        assert_eq!(port.body_calls, 1);
        assert_eq!(port.statement_calls, 0);
    }

    #[test]
    fn guard_let_failure_never_retries_ordinary_or_runs_success_cleanup() {
        let temp_name = "__ny_guard_let_subject_0";
        let mut builder = MirBuilder::new();
        builder
            .function_state
            .variable_ctx
            .variable_map
            .insert(temp_name.to_string(), ValueId::new(7));
        builder
            .function_state
            .binding_ctx
            .insert(temp_name.to_string(), BindingId::new(7));
        let mut port = RecordingPort {
            body_calls: 0,
            statement_calls: 0,
            fail_statement: Some(2),
        };

        let error = builder
            .lower_prepared_raw_scopebox_with_port_v1(
                &mut port,
                PreparedRawScopeBoxV1::prepare(guard_let_body(temp_name)),
            )
            .expect_err("selected guard-let child must fail");

        assert_eq!(error, "scopebox child failure");
        assert_eq!(port.statement_calls, 2);
        assert_eq!(port.body_calls, 0, "ordinary body route must not retry");
        assert!(builder
            .function_state
            .variable_ctx
            .variable_map
            .contains_key(temp_name));
        assert!(builder.function_state.binding_ctx.contains(temp_name));
    }

    #[test]
    fn guard_let_success_lowers_once_then_removes_only_the_temp_binding() {
        let temp_name = "__ny_guard_let_subject_0";
        let mut builder = MirBuilder::new();
        builder
            .function_state
            .variable_ctx
            .variable_map
            .insert(temp_name.to_string(), ValueId::new(7));
        builder
            .function_state
            .binding_ctx
            .insert(temp_name.to_string(), BindingId::new(7));
        let mut port = RecordingPort {
            body_calls: 0,
            statement_calls: 0,
            fail_statement: None,
        };

        let result = builder
            .lower_prepared_raw_scopebox_with_port_v1(
                &mut port,
                PreparedRawScopeBoxV1::prepare(guard_let_body(temp_name)),
            )
            .expect("selected guard-let route must lower");

        assert_eq!(result, ValueId::new(3));
        assert_eq!(port.statement_calls, 3);
        assert_eq!(port.body_calls, 0);
        assert!(!builder
            .function_state
            .variable_ctx
            .variable_map
            .contains_key(temp_name));
        assert!(!builder.function_state.binding_ctx.contains(temp_name));
    }
}
