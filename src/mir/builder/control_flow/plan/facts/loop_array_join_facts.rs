//! Phase 29ap P3: loop_array_join facts (stdlib StringUtils.join subset)

use crate::ast::{ASTNode, BinaryOperator, LiteralValue};
use crate::mir::builder::control_flow::facts::extractors::common_helpers::{
    extract_loop_increment_plan, has_break_statement, has_continue_statement,
    has_if_else_statement, has_return_statement,
};
use crate::mir::builder::control_flow::plan::facts::scan_shapes::{
    loop_var_from_profile, step_delta_from_profile, ConditionShape, ScanConditionObservation,
    StepShape,
};
use crate::mir::builder::control_flow::plan::planner::Freeze;
use crate::mir::policies::CondProfile;

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct LoopArrayJoinFacts {
    pub loop_var: String,
    pub condition: ASTNode,
    pub if_condition: ASTNode,
    pub loop_increment: ASTNode,
    pub array_var: String,
    pub result_var: String,
    pub separator_var: String,
    pub cond_profile: CondProfile,
}

pub(in crate::mir::builder) fn try_extract_loop_array_join_facts(
    condition: &ASTNode,
    body: &[ASTNode],
    observation: &ScanConditionObservation,
) -> Result<Option<LoopArrayJoinFacts>, Freeze> {
    let condition_shape = &observation.condition_shape;
    let step_shape = &observation.step_shape;
    let ConditionShape::VarLessLength { haystack_var, .. } = condition_shape else {
        return Ok(None);
    };

    let Some(loop_var) = loop_var_from_profile(&observation.cond_profile) else {
        return Ok(None);
    };

    let StepShape::AssignAddConst { var: step_var, .. } = step_shape else {
        return Ok(None);
    };

    if step_var != &loop_var {
        return Ok(None);
    }

    if step_delta_from_profile(&observation.cond_profile) != Some(1) {
        return Ok(None);
    }

    if has_break_statement(body) || has_continue_statement(body) || has_return_statement(body) {
        return Ok(None);
    }

    if has_if_else_statement(body) {
        return Ok(None);
    }

    if body.len() != 3 {
        return Ok(None);
    }

    let Some((if_condition, result_var, separator_var)) =
        extract_separator_guard(&body[0], &loop_var)
    else {
        return Ok(None);
    };

    let Some((append_result_var, array_var)) = extract_array_append(&body[1], &loop_var) else {
        return Ok(None);
    };

    if append_result_var != result_var {
        return Ok(None);
    }

    if &array_var != haystack_var {
        return Ok(None);
    }

    let loop_increment = match extract_loop_increment_plan(body, &loop_var) {
        Ok(Some(inc)) => inc,
        _ => return Ok(None),
    };

    Ok(Some(LoopArrayJoinFacts {
        loop_var,
        condition: condition.clone(),
        if_condition,
        loop_increment,
        array_var: array_var.clone(),
        result_var,
        separator_var,
        cond_profile: observation.cond_profile.clone(),
    }))
}

fn extract_separator_guard(stmt: &ASTNode, idx_var: &str) -> Option<(ASTNode, String, String)> {
    let ASTNode::If {
        condition,
        then_body,
        else_body,
        ..
    } = stmt
    else {
        return None;
    };

    if else_body.is_some() || then_body.len() != 1 {
        return None;
    }

    let ASTNode::BinaryOp {
        operator: BinaryOperator::Greater,
        left,
        right,
        ..
    } = condition.as_ref()
    else {
        return None;
    };

    if !matches!(left.as_ref(), ASTNode::Variable { name, .. } if name == idx_var) {
        return None;
    }

    if !matches!(
        right.as_ref(),
        ASTNode::Literal {
            value: LiteralValue::Integer(0),
            ..
        }
    ) {
        return None;
    }

    let (result_var, separator_var) = extract_append_with_separator(&then_body[0])?;
    Some((condition.as_ref().clone(), result_var, separator_var))
}

fn extract_append_with_separator(stmt: &ASTNode) -> Option<(String, String)> {
    let ASTNode::Assignment { target, value, .. } = stmt else {
        return None;
    };

    let ASTNode::Variable {
        name: result_var, ..
    } = target.as_ref()
    else {
        return None;
    };

    let ASTNode::BinaryOp {
        operator: BinaryOperator::Add,
        left,
        right,
        ..
    } = value.as_ref()
    else {
        return None;
    };

    if !matches!(left.as_ref(), ASTNode::Variable { name, .. } if name == result_var) {
        return None;
    }

    let ASTNode::Variable {
        name: separator_var,
        ..
    } = right.as_ref()
    else {
        return None;
    };

    Some((result_var.clone(), separator_var.clone()))
}

fn extract_array_append(stmt: &ASTNode, idx_var: &str) -> Option<(String, String)> {
    let ASTNode::Assignment { target, value, .. } = stmt else {
        return None;
    };

    let ASTNode::Variable {
        name: result_var, ..
    } = target.as_ref()
    else {
        return None;
    };

    let ASTNode::BinaryOp {
        operator: BinaryOperator::Add,
        left,
        right,
        ..
    } = value.as_ref()
    else {
        return None;
    };

    if !matches!(left.as_ref(), ASTNode::Variable { name, .. } if name == result_var) {
        return None;
    }

    let ASTNode::MethodCall {
        object,
        method,
        arguments,
        ..
    } = right.as_ref()
    else {
        return None;
    };

    if method != "get" || arguments.len() != 1 {
        return None;
    }

    let ASTNode::Variable {
        name: array_var, ..
    } = object.as_ref()
    else {
        return None;
    };

    if !matches!(&arguments[0], ASTNode::Variable { name, .. } if name == idx_var) {
        return None;
    }

    Some((result_var.clone(), array_var.clone()))
}

#[cfg(test)]
mod tests {
    use super::{try_extract_loop_array_join_facts, LoopArrayJoinFacts};
    use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
    use crate::mir::builder::control_flow::plan::facts::scan_shapes::{
        scan_condition_observation, ConditionShape, LengthMethod, StepShape,
    };

    fn v(name: &str) -> ASTNode {
        ASTNode::Variable {
            name: name.to_string(),
            span: Span::unknown(),
        }
    }

    fn lit_int(value: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: Span::unknown(),
        }
    }

    fn method_call(object: ASTNode, method: &str, args: Vec<ASTNode>) -> ASTNode {
        ASTNode::MethodCall {
            object: Box::new(object),
            method: method.to_string(),
            arguments: args,
            span: Span::unknown(),
        }
    }

    fn binop(operator: BinaryOperator, left: ASTNode, right: ASTNode) -> ASTNode {
        ASTNode::BinaryOp {
            operator,
            left: Box::new(left),
            right: Box::new(right),
            span: Span::unknown(),
        }
    }

    fn assign(target: &str, value: ASTNode) -> ASTNode {
        ASTNode::Assignment {
            target: Box::new(v(target)),
            value: Box::new(value),
            span: Span::unknown(),
        }
    }

    #[test]
    fn loop_array_join_facts_extracts_minimal_join_shape() {
        let condition = binop(
            BinaryOperator::Less,
            v("i"),
            method_call(v("arr"), "length", vec![]),
        );
        let body = vec![
            ASTNode::If {
                condition: Box::new(binop(BinaryOperator::Greater, v("i"), lit_int(0))),
                then_body: vec![assign(
                    "result",
                    binop(BinaryOperator::Add, v("result"), v("sep")),
                )],
                else_body: None,
                span: Span::unknown(),
            },
            assign(
                "result",
                binop(
                    BinaryOperator::Add,
                    v("result"),
                    method_call(v("arr"), "get", vec![v("i")]),
                ),
            ),
            assign("i", binop(BinaryOperator::Add, v("i"), lit_int(1))),
        ];
        let condition_shape = ConditionShape::VarLessLength {
            idx_var: "i".to_string(),
            haystack_var: "arr".to_string(),
            method: LengthMethod::Length,
        };
        let step_shape = StepShape::AssignAddConst {
            var: "i".to_string(),
            k: 1,
        };

        let observation = scan_condition_observation(&condition_shape, &step_shape);
        let facts = try_extract_loop_array_join_facts(&condition, &body, &observation)
            .expect("Ok")
            .expect("Some facts");

        assert_eq!(facts.loop_var, "i");
        assert_eq!(facts.array_var, "arr");
        assert_eq!(facts.result_var, "result");
        assert_eq!(facts.separator_var, "sep");
        let _: LoopArrayJoinFacts = facts;
    }

    #[test]
    fn loop_array_join_facts_rejects_else_branch() {
        let condition = binop(
            BinaryOperator::Less,
            v("i"),
            method_call(v("arr"), "length", vec![]),
        );
        let body = vec![
            ASTNode::If {
                condition: Box::new(binop(BinaryOperator::Greater, v("i"), lit_int(0))),
                then_body: vec![assign(
                    "result",
                    binop(BinaryOperator::Add, v("result"), v("sep")),
                )],
                else_body: Some(vec![assign("result", v("result"))]),
                span: Span::unknown(),
            },
            assign(
                "result",
                binop(
                    BinaryOperator::Add,
                    v("result"),
                    method_call(v("arr"), "get", vec![v("i")]),
                ),
            ),
            assign("i", binop(BinaryOperator::Add, v("i"), lit_int(1))),
        ];
        let condition_shape = ConditionShape::VarLessLength {
            idx_var: "i".to_string(),
            haystack_var: "arr".to_string(),
            method: LengthMethod::Length,
        };
        let step_shape = StepShape::AssignAddConst {
            var: "i".to_string(),
            k: 1,
        };

        let observation = scan_condition_observation(&condition_shape, &step_shape);
        let facts = try_extract_loop_array_join_facts(&condition, &body, &observation).expect("Ok");
        assert!(facts.is_none());
    }
}
