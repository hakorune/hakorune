//! Phase 29aj P7: bool_predicate_scan facts (legacy type: Pattern8BoolPredicateScanFacts, SSOT)

use crate::ast::{ASTNode, BinaryOperator, LiteralValue, UnaryOperator};
use crate::mir::builder::control_flow::plan::facts::scan_shapes::{
    loop_var_from_profile, step_delta_from_profile, ConditionShape, LengthMethod,
    ScanConditionObservation, StepShape,
};
use crate::mir::builder::control_flow::plan::planner::Freeze;
use crate::mir::policies::CondProfile;

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct Pattern8BoolPredicateScanFacts {
    pub loop_var: String,
    pub haystack: String,
    pub predicate_receiver: String,
    pub predicate_method: String,
    pub condition: ASTNode,
    pub step_lit: i64,
    pub cond_profile: CondProfile,
}

pub(in crate::mir::builder) fn try_extract_pattern8_bool_predicate_scan_facts(
    condition: &ASTNode,
    body: &[ASTNode],
    observation: &ScanConditionObservation,
) -> Result<Option<Pattern8BoolPredicateScanFacts>, Freeze> {
    let condition_shape = &observation.condition_shape;
    let step_shape = &observation.step_shape;
    let haystack = match condition_shape {
        ConditionShape::VarLessLength {
            haystack_var,
            method: LengthMethod::Length,
            ..
        } => haystack_var.clone(),
        _ => return Ok(None),
    };

    let Some(loop_var) = loop_var_from_profile(&observation.cond_profile) else {
        return Ok(None);
    };

    let StepShape::AssignAddConst { var, .. } = step_shape else {
        return Ok(None);
    };
    if var != &loop_var {
        return Ok(None);
    }
    let Some(step_lit) = step_delta_from_profile(&observation.cond_profile) else {
        return Ok(None);
    };
    if step_lit != 1 {
        return Ok(None);
    }

    let (predicate_receiver, predicate_method) =
        match extract_predicate_check(body, &loop_var, &haystack) {
            Some(values) => values,
            None => return Ok(None),
        };

    Ok(Some(Pattern8BoolPredicateScanFacts {
        loop_var,
        haystack,
        predicate_receiver,
        predicate_method,
        condition: condition.clone(),
        step_lit,
        cond_profile: observation.cond_profile.clone(),
    }))
}

fn extract_predicate_check(
    body: &[ASTNode],
    loop_var: &str,
    haystack: &str,
) -> Option<(String, String)> {
    for stmt in body {
        let ASTNode::If {
            condition,
            then_body,
            else_body,
            ..
        } = stmt
        else {
            continue;
        };

        if else_body.is_some() || then_body.len() != 1 {
            continue;
        }

        let ASTNode::Return { value, .. } = &then_body[0] else {
            continue;
        };
        let Some(ret_val) = value.as_ref() else {
            continue;
        };
        if !matches!(
            ret_val.as_ref(),
            ASTNode::Literal {
                value: LiteralValue::Bool(false),
                ..
            }
        ) {
            continue;
        }

        let ASTNode::UnaryOp {
            operator: UnaryOperator::Not,
            operand,
            ..
        } = condition.as_ref()
        else {
            continue;
        };

        let ASTNode::MethodCall {
            object,
            method,
            arguments,
            ..
        } = operand.as_ref()
        else {
            continue;
        };

        if arguments.len() != 1 {
            continue;
        }

        if !validate_substring_call(&arguments[0], haystack, loop_var) {
            continue;
        }

        let receiver = match object.as_ref() {
            ASTNode::Variable { name, .. } => name.clone(),
            ASTNode::Me { .. } | ASTNode::This { .. } => "me".to_string(),
            _ => continue,
        };

        return Some((receiver, method.clone()));
    }

    None
}

fn validate_substring_call(arg: &ASTNode, haystack: &str, loop_var: &str) -> bool {
    let ASTNode::MethodCall {
        object,
        method,
        arguments,
        ..
    } = arg
    else {
        return false;
    };

    if method != "substring" {
        return false;
    }

    let ASTNode::Variable { name, .. } = object.as_ref() else {
        return false;
    };
    if name != haystack {
        return false;
    }

    if arguments.len() != 2 {
        return false;
    }

    if !matches!(
        &arguments[0],
        ASTNode::Variable { name, .. } if name == loop_var
    ) {
        return false;
    }

    let ASTNode::BinaryOp {
        operator: BinaryOperator::Add,
        left,
        right,
        ..
    } = &arguments[1]
    else {
        return false;
    };
    if !matches!(left.as_ref(), ASTNode::Variable { name, .. } if name == loop_var) {
        return false;
    }
    matches!(
        right.as_ref(),
        ASTNode::Literal {
            value: LiteralValue::Integer(1),
            ..
        }
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Span;
    use crate::mir::builder::control_flow::plan::facts::scan_shapes::scan_condition_observation;

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

    fn condition_length(loop_var: &str, haystack: &str) -> ASTNode {
        ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(v(loop_var)),
            right: Box::new(ASTNode::MethodCall {
                object: Box::new(v(haystack)),
                method: "length".to_string(),
                arguments: vec![],
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }
    }

    fn predicate_if(receiver: &str, method: &str, haystack: &str, loop_var: &str) -> ASTNode {
        ASTNode::If {
            condition: Box::new(ASTNode::UnaryOp {
                operator: UnaryOperator::Not,
                operand: Box::new(ASTNode::MethodCall {
                    object: Box::new(if receiver == "me" {
                        ASTNode::Me {
                            span: Span::unknown(),
                        }
                    } else if receiver == "this" {
                        ASTNode::This {
                            span: Span::unknown(),
                        }
                    } else {
                        v(receiver)
                    }),
                    method: method.to_string(),
                    arguments: vec![ASTNode::MethodCall {
                        object: Box::new(v(haystack)),
                        method: "substring".to_string(),
                        arguments: vec![
                            v(loop_var),
                            ASTNode::BinaryOp {
                                operator: BinaryOperator::Add,
                                left: Box::new(v(loop_var)),
                                right: Box::new(lit_int(1)),
                                span: Span::unknown(),
                            },
                        ],
                        span: Span::unknown(),
                    }],
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            }),
            then_body: vec![ASTNode::Return {
                value: Some(Box::new(ASTNode::Literal {
                    value: LiteralValue::Bool(false),
                    span: Span::unknown(),
                })),
                span: Span::unknown(),
            }],
            else_body: None,
            span: Span::unknown(),
        }
    }

    fn predicate_if_with_else() -> ASTNode {
        ASTNode::If {
            condition: Box::new(v("cond")),
            then_body: vec![ASTNode::Return {
                value: Some(Box::new(ASTNode::Literal {
                    value: LiteralValue::Bool(false),
                    span: Span::unknown(),
                })),
                span: Span::unknown(),
            }],
            else_body: Some(vec![ASTNode::Return {
                value: Some(Box::new(ASTNode::Literal {
                    value: LiteralValue::Bool(true),
                    span: Span::unknown(),
                })),
                span: Span::unknown(),
            }]),
            span: Span::unknown(),
        }
    }

    fn loop_increment(var: &str, step: i64) -> ASTNode {
        ASTNode::Assignment {
            target: Box::new(v(var)),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(v(var)),
                right: Box::new(lit_int(step)),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }
    }

    #[test]
    fn facts_extracts_pattern8_success() {
        let condition = condition_length("i", "s");
        let body = vec![
            predicate_if("this", "is_digit", "s", "i"),
            loop_increment("i", 1),
        ];

        let condition_shape = ConditionShape::VarLessLength {
            idx_var: "i".to_string(),
            haystack_var: "s".to_string(),
            method: LengthMethod::Length,
        };
        let step_shape = StepShape::AssignAddConst {
            var: "i".to_string(),
            k: 1,
        };

        let observation = scan_condition_observation(&condition_shape, &step_shape);
        let facts =
            try_extract_pattern8_bool_predicate_scan_facts(&condition, &body, &observation)
                .expect("Ok");
        let facts = facts.expect("Some");

        assert_eq!(facts.loop_var, "i");
        assert_eq!(facts.haystack, "s");
        assert_eq!(facts.predicate_receiver, "me");
        assert_eq!(facts.predicate_method, "is_digit");
        assert_eq!(facts.step_lit, 1);
    }

    #[test]
    fn facts_rejects_wrong_step() {
        let condition = condition_length("i", "s");
        let body = vec![
            predicate_if("me", "is_digit", "s", "i"),
            loop_increment("i", 2),
        ];

        let condition_shape = ConditionShape::VarLessLength {
            idx_var: "i".to_string(),
            haystack_var: "s".to_string(),
            method: LengthMethod::Length,
        };
        let step_shape = StepShape::AssignAddConst {
            var: "i".to_string(),
            k: 2,
        };

        let observation = scan_condition_observation(&condition_shape, &step_shape);
        let facts =
            try_extract_pattern8_bool_predicate_scan_facts(&condition, &body, &observation)
                .expect("Ok");
        assert!(facts.is_none());
    }

    #[test]
    fn facts_rejects_else_branch() {
        let condition = condition_length("i", "s");
        let body = vec![predicate_if_with_else(), loop_increment("i", 1)];

        let condition_shape = ConditionShape::VarLessLength {
            idx_var: "i".to_string(),
            haystack_var: "s".to_string(),
            method: LengthMethod::Length,
        };
        let step_shape = StepShape::AssignAddConst {
            var: "i".to_string(),
            k: 1,
        };

        let observation = scan_condition_observation(&condition_shape, &step_shape);
        let facts =
            try_extract_pattern8_bool_predicate_scan_facts(&condition, &body, &observation)
                .expect("Ok");
        assert!(facts.is_none());
    }
}
