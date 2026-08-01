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
