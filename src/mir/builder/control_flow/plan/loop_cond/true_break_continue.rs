//! Phase 29bq P2: LoopTrueBreakContinueFacts (Facts SSOT)
//!
//! Recipe-first: Facts は recipe を作るだけ（意味論条件の羅列禁止）
//! Lower/pipeline は recipe を順に下ろすだけ（再判定禁止）

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::facts::extractors::common_helpers::is_true_literal;
use crate::mir::builder::control_flow::plan::facts::exit_only_block::{
    try_build_exit_allowed_block_recipe, ExitAllowedBlockRecipe,
};
use crate::mir::builder::control_flow::plan::loop_true_break_continue::recipe::LoopTrueBreakContinueRecipe;
use crate::mir::builder::control_flow::plan::planner::Freeze;
use crate::mir::builder::control_flow::recipes::loop_cond_shared::LoopCondRecipe;

#[derive(Debug, Clone)]
pub(in crate::mir::builder) enum LoopTrueBreakContinueLowering {
    ExitAllowed(ExitAllowedBlockRecipe),
    RecipeOnly,
}

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct LoopTrueBreakContinueFacts {
    pub recipe: LoopTrueBreakContinueRecipe,
    pub lowering: LoopTrueBreakContinueLowering,
}

pub(in crate::mir::builder) fn try_extract_loop_true_break_continue_facts(
    condition: &ASTNode,
    body: &[ASTNode],
) -> Result<Option<LoopTrueBreakContinueFacts>, Freeze> {
    if !is_true_literal(condition) {
        return Ok(None);
    }

    let counts = super::loop_cond_unified_helpers::count_control_flow_with_returns(body);
    // return は ExitIf で許可する（exit_if_map 制約: prelude+return+else=NG）
    if counts.break_count == 0 && counts.continue_count == 0 && counts.return_count == 0 {
        return Ok(None);
    }

    // Recipe-first: represent the body as an ExitAllowed RecipeBlock.
    //
    // This is critical for Stage-B parser loops such as `parse_block` where the body may contain
    // nested loops (e.g. whitespace skipping) and join-bearing `if` statements.
    let allow_extended = true;
    let body_exit_allowed = try_build_exit_allowed_block_recipe(body, allow_extended);
    if let Some(body_exit_allowed) = body_exit_allowed {
        return Ok(Some(LoopTrueBreakContinueFacts {
            recipe: LoopCondRecipe::new(body.to_vec(), Vec::new()),
            lowering: LoopTrueBreakContinueLowering::ExitAllowed(body_exit_allowed),
        }));
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::{try_extract_loop_true_break_continue_facts, LoopTrueBreakContinueLowering};
    use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};

    fn with_loop_true_break_continue_env<F: FnOnce()>(
        joinir_dev: Option<&str>,
        planner_required: Option<&str>,
        f: F,
    ) {
        crate::test_support::with_env_vars(
            &[
                ("NYASH_JOINIR_DEV", joinir_dev),
                ("HAKO_JOINIR_PLANNER_REQUIRED", planner_required),
            ],
            f,
        );
    }

    fn bool_lit(value: bool) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Bool(value),
            span: Span::unknown(),
        }
    }

    fn int(value: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: Span::unknown(),
        }
    }

    fn null_lit() -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Null,
            span: Span::unknown(),
        }
    }

    fn str_lit(value: &str) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::String(value.to_string()),
            span: Span::unknown(),
        }
    }

    fn var(name: &str) -> ASTNode {
        ASTNode::Variable {
            name: name.to_string(),
            span: Span::unknown(),
        }
    }

    fn me_call(method: &str, arguments: Vec<ASTNode>) -> ASTNode {
        ASTNode::MethodCall {
            object: Box::new(ASTNode::Me {
                span: Span::unknown(),
            }),
            method: method.to_string(),
            arguments,
            span: Span::unknown(),
        }
    }

    fn me_field(field: &str) -> ASTNode {
        ASTNode::FieldAccess {
            object: Box::new(ASTNode::Me {
                span: Span::unknown(),
            }),
            field: field.to_string(),
            span: Span::unknown(),
        }
    }

    fn receiver_call(receiver: &str, method: &str, arguments: Vec<ASTNode>) -> ASTNode {
        ASTNode::MethodCall {
            object: Box::new(var(receiver)),
            method: method.to_string(),
            arguments,
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

    fn ret(value: ASTNode) -> ASTNode {
        ASTNode::Return {
            value: Some(Box::new(value)),
            span: Span::unknown(),
        }
    }

    fn if_stmt(
        condition: ASTNode,
        then_body: Vec<ASTNode>,
        else_body: Option<Vec<ASTNode>>,
    ) -> ASTNode {
        ASTNode::If {
            condition: Box::new(condition),
            then_body,
            else_body,
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

    #[test]
    fn policy_exit_allowed_when_extended() {
        with_loop_true_break_continue_env(Some("1"), Some("1"), || {
            let condition = bool_lit(true);
            let body = vec![ASTNode::Return {
                value: Some(Box::new(int(1))),
                span: Span::unknown(),
            }];

            let facts = try_extract_loop_true_break_continue_facts(&condition, &body)
                .expect("no freeze")
                .expect("facts");

            assert!(matches!(
                facts.lowering,
                LoopTrueBreakContinueLowering::ExitAllowed(_)
            ));
        });
    }

    #[test]
    fn policy_exit_allowed_without_dev_env() {
        with_loop_true_break_continue_env(None, None, || {
            let condition = bool_lit(true);
            let body = vec![ASTNode::If {
                condition: Box::new(binop(BinaryOperator::Equal, bool_lit(true), bool_lit(true))),
                then_body: vec![ASTNode::Break {
                    span: Span::unknown(),
                }],
                else_body: None,
                span: Span::unknown(),
            }];

            let facts = try_extract_loop_true_break_continue_facts(&condition, &body)
                .expect("no freeze")
                .expect("facts");

            assert!(matches!(
                facts.lowering,
                LoopTrueBreakContinueLowering::ExitAllowed(_)
            ));
        });
    }

    #[test]
    fn policy_exit_allowed_accepts_continue_prelude_with_else_exit_tree() {
        with_loop_true_break_continue_env(None, None, || {
            let condition = bool_lit(true);
            let body = vec![
                local("value", me_call("parse_value", vec![])),
                if_stmt(
                    binop(BinaryOperator::Equal, var("value"), null_lit()),
                    vec![ret(null_lit())],
                    None,
                ),
                receiver_call("array_node", "array_push", vec![var("value")]),
                if_stmt(
                    me_call("match_token", vec![str_lit("COMMA")]),
                    vec![
                        receiver_call("JsonParserTrace", "log", vec![str_lit("array comma")]),
                        ASTNode::Continue {
                            span: Span::unknown(),
                        },
                    ],
                    Some(vec![if_stmt(
                        me_call("match_token", vec![str_lit("RBRACKET")]),
                        vec![
                            receiver_call("JsonParserTrace", "log", vec![str_lit("exit array")]),
                            ASTNode::Break {
                                span: Span::unknown(),
                            },
                        ],
                        Some(vec![
                            me_call("add_error_expected", vec![str_lit("',' or ']' in array")]),
                            ret(null_lit()),
                        ]),
                    )]),
                ),
            ];

            let facts = try_extract_loop_true_break_continue_facts(&condition, &body)
                .expect("no freeze")
                .expect("facts");

            assert!(matches!(
                facts.lowering,
                LoopTrueBreakContinueLowering::ExitAllowed(_)
            ));
        });
    }

    #[test]
    fn policy_exit_allowed_accepts_receiver_field_nested_exit() {
        with_loop_true_break_continue_env(None, None, || {
            let condition = bool_lit(true);
            let policy_is_present = binop(BinaryOperator::NotEqual, me_field("policy"), null_lit());
            let body = vec![
                if_stmt(
                    binop(
                        BinaryOperator::And,
                        policy_is_present,
                        me_call("policy_enabled", vec![]),
                    ),
                    vec![if_stmt(bool_lit(true), vec![ret(null_lit())], None)],
                    None,
                ),
                ASTNode::Break {
                    span: Span::unknown(),
                },
            ];

            let facts = try_extract_loop_true_break_continue_facts(&condition, &body)
                .expect("no freeze")
                .expect("facts");

            assert!(matches!(
                facts.lowering,
                LoopTrueBreakContinueLowering::ExitAllowed(_)
            ));
        });
    }
}
