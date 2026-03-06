//! Phase 29ap P2: loop_char_map facts (stdlib to_lower subset)

use crate::ast::{ASTNode, BinaryOperator, LiteralValue};
use crate::mir::builder::control_flow::plan::extractors::common_helpers::{
    extract_loop_increment_plan, has_break_statement, has_continue_statement, has_if_else_statement,
    has_return_statement,
};
use crate::mir::builder::control_flow::plan::facts::scan_shapes::{
    loop_var_from_profile, step_delta_from_profile, ConditionShape, ScanConditionObservation,
    StepShape,
};
use crate::mir::builder::control_flow::plan::planner::Freeze;
use crate::mir::policies::CondProfile;

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct LoopCharMapFacts {
    pub loop_var: String,
    pub condition: ASTNode,
    pub loop_increment: ASTNode,
    pub haystack_var: String,
    pub result_var: String,
    pub receiver_var: String,
    pub transform_method: String,
    pub cond_profile: CondProfile,
}

pub(in crate::mir::builder) fn try_extract_loop_char_map_facts(
    condition: &ASTNode,
    body: &[ASTNode],
    observation: &ScanConditionObservation,
) -> Result<Option<LoopCharMapFacts>, Freeze> {
    let condition_shape = &observation.condition_shape;
    let step_shape = &observation.step_shape;
    let ConditionShape::VarLessLength { haystack_var, .. } = condition_shape
    else {
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

    let Some(ch_var) = extract_local_substring(&body[0], &loop_var, haystack_var) else {
        return Ok(None);
    };

    let Some((result_var, receiver_var, transform_method)) =
        extract_result_update(&body[1], &ch_var)
    else {
        return Ok(None);
    };

    let loop_increment = match extract_loop_increment_plan(body, &loop_var) {
        Ok(Some(inc)) => inc,
        _ => return Ok(None),
    };

    Ok(Some(LoopCharMapFacts {
        loop_var,
        condition: condition.clone(),
        loop_increment,
        haystack_var: haystack_var.clone(),
        result_var,
        receiver_var,
        transform_method,
        cond_profile: observation.cond_profile.clone(),
    }))
}

fn extract_local_substring(
    stmt: &ASTNode,
    idx_var: &str,
    haystack_var: &str,
) -> Option<String> {
    let ASTNode::Local {
        variables,
        initial_values,
        ..
    } = stmt
    else {
        return None;
    };

    if variables.len() != 1 || initial_values.len() != 1 {
        return None;
    }

    let ch_var = variables[0].clone();
    let Some(init) = &initial_values[0] else {
        return None;
    };

    let ASTNode::MethodCall {
        object,
        method,
        arguments,
        ..
    } = init.as_ref()
    else {
        return None;
    };

    if method != "substring" || arguments.len() != 2 {
        return None;
    }

    match object.as_ref() {
        ASTNode::Variable { name, .. } if name == haystack_var => {}
        _ => return None,
    }

    if !matches!(&arguments[0], ASTNode::Variable { name, .. } if name == idx_var) {
        return None;
    }

    let ASTNode::BinaryOp {
        operator: BinaryOperator::Add,
        left,
        right,
        ..
    } = &arguments[1]
    else {
        return None;
    };

    if !matches!(left.as_ref(), ASTNode::Variable { name, .. } if name == idx_var) {
        return None;
    }

    if !matches!(
        right.as_ref(),
        ASTNode::Literal {
            value: LiteralValue::Integer(1),
            ..
        }
    ) {
        return None;
    }

    Some(ch_var)
}

fn extract_result_update(stmt: &ASTNode, ch_var: &str) -> Option<(String, String, String)> {
    let ASTNode::Assignment { target, value, .. } = stmt else {
        return None;
    };

    let ASTNode::Variable { name: result_var, .. } = target.as_ref() else {
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

    if arguments.len() != 1 {
        return None;
    }

    if !matches!(&arguments[0], ASTNode::Variable { name, .. } if name == ch_var) {
        return None;
    }

    let receiver_var = match object.as_ref() {
        ASTNode::This { .. } | ASTNode::Me { .. } => "me".to_string(),
        ASTNode::Variable { name, .. } => name.clone(),
        _ => return None,
    };

    Some((result_var.clone(), receiver_var, method.clone()))
}

#[cfg(test)]
mod tests {
    use super::{try_extract_loop_char_map_facts, LoopCharMapFacts};
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

    #[test]
    fn loop_char_map_facts_extracts_minimal_tolower_shape() {
        let condition = ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(v("i")),
            right: Box::new(method_call(v("s"), "length", vec![])),
            span: Span::unknown(),
        };
        let condition_shape = ConditionShape::VarLessLength {
            idx_var: "i".to_string(),
            haystack_var: "s".to_string(),
            method: LengthMethod::Length,
        };
        let step_shape = StepShape::AssignAddConst {
            var: "i".to_string(),
            k: 1,
        };

        let local_ch = ASTNode::Local {
            variables: vec!["ch".to_string()],
            initial_values: vec![Some(Box::new(method_call(
                v("s"),
                "substring",
                vec![
                    v("i"),
                    ASTNode::BinaryOp {
                        operator: BinaryOperator::Add,
                        left: Box::new(v("i")),
                        right: Box::new(lit_int(1)),
                        span: Span::unknown(),
                    },
                ],
            )))],
            span: Span::unknown(),
        };

        let result_update = ASTNode::Assignment {
            target: Box::new(v("result")),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(v("result")),
                right: Box::new(method_call(
                    ASTNode::This { span: Span::unknown() },
                    "char_to_lower",
                    vec![v("ch")],
                )),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };

        let step = ASTNode::Assignment {
            target: Box::new(v("i")),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(v("i")),
                right: Box::new(lit_int(1)),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };

        let body = vec![local_ch, result_update, step];
        let observation = scan_condition_observation(&condition_shape, &step_shape);
        let facts = try_extract_loop_char_map_facts(&condition, &body, &observation)
            .expect("Ok")
            .expect("Some");

        assert_eq!(facts.loop_var, "i");
        assert_eq!(facts.haystack_var, "s");
        assert_eq!(facts.result_var, "result");
        assert_eq!(facts.receiver_var, "me");
        assert_eq!(facts.transform_method, "char_to_lower");
        let _: LoopCharMapFacts = facts;
    }

    #[test]
    fn loop_char_map_facts_rejects_nonmatching_body() {
        let condition = ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(v("i")),
            right: Box::new(method_call(v("s"), "length", vec![])),
            span: Span::unknown(),
        };
        let condition_shape = ConditionShape::VarLessLength {
            idx_var: "i".to_string(),
            haystack_var: "s".to_string(),
            method: LengthMethod::Length,
        };
        let step_shape = StepShape::AssignAddConst {
            var: "i".to_string(),
            k: 1,
        };

        let body = vec![ASTNode::Assignment {
            target: Box::new(v("i")),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(v("i")),
                right: Box::new(lit_int(1)),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }];

        let observation = scan_condition_observation(&condition_shape, &step_shape);
        let facts = try_extract_loop_char_map_facts(&condition, &body, &observation)
            .expect("Ok");
        assert!(facts.is_none());
    }
}
