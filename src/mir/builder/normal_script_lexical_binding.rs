//! Source-only lexical admission and the request-local lowering ledger.
//!
//! This module deliberately stops at BindingRef facts. Runtime ValueIds are
//! added later, after the existing Local owner has materialized a value.

use std::collections::{BTreeMap, BTreeSet};

use crate::ast::ASTNode;
use crate::mir::resolved_semantics::{
    BindingRefV1, ExprChildRoleV1, SourceExprSiteV1, SourceNodeSiteV1, SourcePathSegmentV1,
};
use crate::mir::ValueId;

use super::normal_script_runtime_work::{
    LocatedNormalScriptRuntimeAdmissionV1, NormalScriptRuntimeStatementAdmissionV1,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct ScriptLocalFactV1 {
    pub(super) source_statement_index: usize,
    pub(super) name: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct ScriptVariableFactV1 {
    pub(super) source_statement_index: usize,
    pub(super) binding_statement_index: usize,
    pub(super) initializer: bool,
    pub(super) path: Box<[SourcePathSegmentV1]>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct ScriptLexicalFactsV1 {
    pub(super) locals: Box<[ScriptLocalFactV1]>,
    pub(super) variables: Box<[ScriptVariableFactV1]>,
    pub(super) expression_source_indices: Box<[usize]>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct ScriptSemanticClosureFactsV1 {
    pub(super) lexical: ScriptLexicalFactsV1,
    pub(super) static_const_completion_source_indices: Box<[usize]>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum ScriptSemanticClosureAdmissionV1 {
    Complete(ScriptSemanticClosureFactsV1),
    Deferred(ScriptLexicalDeferredReasonV1),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum ScriptSemanticAdmissionInvariantErrorV1 {
    RuntimeCoverageMismatch,
    SourceAdmissionMismatch,
    DuplicateCompletionSite,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum ScriptLexicalDeferredReasonV1 {
    UnsafeRuntimeStatement,
    UndefinedVariable,
    DuplicateLocal,
    LocalShape,
}

pub(super) fn admit_runtime_script_lexical_v1(
    statements: &[ASTNode],
    admissions: &[LocatedNormalScriptRuntimeAdmissionV1],
) -> Result<ScriptSemanticClosureAdmissionV1, ScriptSemanticAdmissionInvariantErrorV1> {
    if statements.len() != admissions.len() {
        return Err(ScriptSemanticAdmissionInvariantErrorV1::RuntimeCoverageMismatch);
    }
    let mut locals = Vec::new();
    let mut visible = BTreeMap::<String, usize>::new();
    let mut variables = Vec::new();
    let mut expression_source_indices = Vec::new();
    let mut runtime_source_indices = BTreeSet::new();
    let mut static_const_completion_source_indices = BTreeSet::new();
    for (statement, admission) in statements.iter().zip(admissions) {
        if !runtime_source_indices.insert(admission.source_statement_index) {
            return Err(ScriptSemanticAdmissionInvariantErrorV1::DuplicateCompletionSite);
        }
        if matches!(
            admission.admission,
            NormalScriptRuntimeStatementAdmissionV1::DirectStaticConstRuntimeCompletion
        ) {
            if !matches!(statement, ASTNode::StaticConstTable { .. }) {
                return Err(ScriptSemanticAdmissionInvariantErrorV1::SourceAdmissionMismatch);
            }
            if !static_const_completion_source_indices.insert(admission.source_statement_index) {
                return Err(ScriptSemanticAdmissionInvariantErrorV1::DuplicateCompletionSite);
            }
            continue;
        }
        if !matches!(
            admission.admission,
            NormalScriptRuntimeStatementAdmissionV1::DirectPortAwareExpression
                | NormalScriptRuntimeStatementAdmissionV1::DirectPrint
        ) {
            return Ok(ScriptSemanticClosureAdmissionV1::Deferred(
                ScriptLexicalDeferredReasonV1::UnsafeRuntimeStatement,
            ));
        }
        let index = admission.source_statement_index;
        match statement {
            ASTNode::Print { expression, .. } => {
                let Some(segment) = ExprChildRoleV1::PrintValue.segment_for(statement) else {
                    return Ok(ScriptSemanticClosureAdmissionV1::Deferred(
                        ScriptLexicalDeferredReasonV1::UnsafeRuntimeStatement,
                    ));
                };
                let path = [segment];
                if let Err(reason) =
                    admit_expression_v1(expression, index, false, &path, &visible, &mut variables)
                {
                    return Ok(ScriptSemanticClosureAdmissionV1::Deferred(reason));
                }
                expression_source_indices.push(index);
            }
            ASTNode::Literal { .. }
            | ASTNode::Variable { .. }
            | ASTNode::UnaryOp { .. }
            | ASTNode::BinaryOp { .. }
            | ASTNode::CheckExpr { .. }
            | ASTNode::AwaitExpression { .. } => {
                if let Err(reason) =
                    admit_expression_v1(statement, index, false, &[], &visible, &mut variables)
                {
                    return Ok(ScriptSemanticClosureAdmissionV1::Deferred(reason));
                }
                expression_source_indices.push(index);
            }
            ASTNode::Local {
                variables: names,
                initial_values,
                declared_type_names,
                ..
            } => {
                if names.len() != 1
                    || initial_values.len() != 1
                    || !(declared_type_names.is_empty()
                        || (declared_type_names.len() == 1 && declared_type_names[0].is_none()))
                    || visible.contains_key(&names[0])
                {
                    return Ok(ScriptSemanticClosureAdmissionV1::Deferred(
                        ScriptLexicalDeferredReasonV1::LocalShape,
                    ));
                }
                let Some(initializer) = initial_values[0].as_deref() else {
                    return Ok(ScriptSemanticClosureAdmissionV1::Deferred(
                        ScriptLexicalDeferredReasonV1::LocalShape,
                    ));
                };
                if let Err(reason) =
                    admit_expression_v1(initializer, index, true, &[], &visible, &mut variables)
                {
                    return Ok(ScriptSemanticClosureAdmissionV1::Deferred(reason));
                }
                visible.insert(names[0].clone(), index);
                locals.push(ScriptLocalFactV1 {
                    source_statement_index: index,
                    name: names[0].clone(),
                });
            }
            _ => {
                return Ok(ScriptSemanticClosureAdmissionV1::Deferred(
                    ScriptLexicalDeferredReasonV1::UnsafeRuntimeStatement,
                ))
            }
        }
    }
    Ok(ScriptSemanticClosureAdmissionV1::Complete(
        ScriptSemanticClosureFactsV1 {
            lexical: ScriptLexicalFactsV1 {
                locals: locals.into_boxed_slice(),
                variables: variables.into_boxed_slice(),
                expression_source_indices: expression_source_indices.into_boxed_slice(),
            },
            static_const_completion_source_indices: static_const_completion_source_indices
                .into_iter()
                .collect(),
        },
    ))
}

fn admit_expression_v1(
    expression: &ASTNode,
    source_statement_index: usize,
    initializer: bool,
    path: &[SourcePathSegmentV1],
    visible: &BTreeMap<String, usize>,
    variables: &mut Vec<ScriptVariableFactV1>,
) -> Result<(), ScriptLexicalDeferredReasonV1> {
    match expression {
        ASTNode::Literal { .. } => Ok(()),
        ASTNode::Variable { name, .. } => {
            let Some(&binding_statement_index) = visible.get(name) else {
                return Err(ScriptLexicalDeferredReasonV1::UndefinedVariable);
            };
            variables.push(ScriptVariableFactV1 {
                source_statement_index,
                binding_statement_index,
                initializer,
                path: path.to_vec().into_boxed_slice(),
            });
            Ok(())
        }
        ASTNode::UnaryOp { operand, .. } => {
            let Some(segment) = ExprChildRoleV1::UnaryOperand.segment_for(expression) else {
                return Err(ScriptLexicalDeferredReasonV1::LocalShape);
            };
            if matches!(
                expression,
                ASTNode::UnaryOp {
                    operator: crate::ast::UnaryOperator::Weak,
                    ..
                }
            ) {
                return Err(ScriptLexicalDeferredReasonV1::UnsafeRuntimeStatement);
            }
            let mut child_path = path.to_vec();
            child_path.push(segment);
            admit_expression_v1(
                operand,
                source_statement_index,
                initializer,
                &child_path,
                visible,
                variables,
            )
        }
        ASTNode::BinaryOp { left, right, .. } => {
            let Some(left_segment) = ExprChildRoleV1::BinaryLeft.segment_for(expression) else {
                return Err(ScriptLexicalDeferredReasonV1::LocalShape);
            };
            let Some(right_segment) = ExprChildRoleV1::BinaryRight.segment_for(expression) else {
                return Err(ScriptLexicalDeferredReasonV1::LocalShape);
            };
            let mut left_path = path.to_vec();
            left_path.push(left_segment);
            admit_expression_v1(
                left,
                source_statement_index,
                initializer,
                &left_path,
                visible,
                variables,
            )?;
            let mut right_path = path.to_vec();
            right_path.push(right_segment);
            admit_expression_v1(
                right,
                source_statement_index,
                initializer,
                &right_path,
                visible,
                variables,
            )
        }
        ASTNode::AwaitExpression {
            expression: operand,
            ..
        } => {
            let Some(segment) = ExprChildRoleV1::AwaitOperand.segment_for(expression) else {
                return Err(ScriptLexicalDeferredReasonV1::LocalShape);
            };
            let mut child_path = path.to_vec();
            child_path.push(segment);
            admit_expression_v1(
                operand,
                source_statement_index,
                initializer,
                &child_path,
                visible,
                variables,
            )
        }
        ASTNode::CheckExpr { items, .. } => {
            for (item_index, item) in items.iter().enumerate() {
                let Some(segment) =
                    ExprChildRoleV1::CheckItem(item_index as u32).segment_for(expression)
                else {
                    return Err(ScriptLexicalDeferredReasonV1::LocalShape);
                };
                let mut item_path = path.to_vec();
                item_path.push(segment);
                admit_expression_v1(
                    &item.expression,
                    source_statement_index,
                    initializer,
                    &item_path,
                    visible,
                    variables,
                )?;
            }
            Ok(())
        }
        _ => Err(ScriptLexicalDeferredReasonV1::LocalShape),
    }
}

#[derive(Debug, Default)]
pub(super) struct ScriptSemanticLoweringState {
    variable_values: BTreeMap<BindingRefV1, ValueId>,
    variables: BTreeMap<SourceNodeSiteV1, BindingRefV1>,
    locals: BTreeMap<SourceNodeSiteV1, BindingRefV1>,
}

impl ScriptSemanticLoweringState {
    pub(super) fn from_facts(
        locals: impl IntoIterator<Item = (SourceNodeSiteV1, BindingRefV1)>,
        variables: impl IntoIterator<Item = (SourceExprSiteV1, BindingRefV1)>,
    ) -> Self {
        Self {
            variable_values: BTreeMap::new(),
            variables: variables
                .into_iter()
                .map(|(site, binding)| (site.node().clone(), binding))
                .collect(),
            locals: locals.into_iter().collect(),
        }
    }

    pub(super) fn variable_binding(&self, site: &SourceNodeSiteV1) -> Option<BindingRefV1> {
        self.variables.get(site).copied()
    }

    pub(super) fn local_binding(&self, site: &SourceNodeSiteV1) -> Option<BindingRefV1> {
        self.locals.get(site).copied()
    }

    pub(super) fn value(&self, binding: BindingRefV1) -> Option<ValueId> {
        self.variable_values.get(&binding).copied()
    }

    pub(super) fn record(&mut self, binding: BindingRefV1, value: ValueId) -> Result<(), String> {
        if self.variable_values.insert(binding, value).is_some() {
            return Err("[freeze:contract][script-lexical/duplicate-value]".to_owned());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{
        admit_runtime_script_lexical_v1, LocatedNormalScriptRuntimeAdmissionV1,
        NormalScriptRuntimeStatementAdmissionV1, ScriptLexicalDeferredReasonV1,
        ScriptSemanticClosureAdmissionV1,
    };
    use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span, UnaryOperator};

    fn admission() -> LocatedNormalScriptRuntimeAdmissionV1 {
        LocatedNormalScriptRuntimeAdmissionV1 {
            source_statement_index: 0,
            admission: NormalScriptRuntimeStatementAdmissionV1::DirectPortAwareExpression,
        }
    }

    fn static_const(name: &str) -> ASTNode {
        ASTNode::StaticConstTable {
            name: name.to_owned(),
            element_type: "u16".to_owned(),
            values: vec![1, 2, 3],
            span: Span::unknown(),
        }
    }

    #[test]
    fn ordinary_unary_is_admitted_recursively() {
        let statements = vec![ASTNode::UnaryOp {
            operator: UnaryOperator::Minus,
            operand: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(1),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }];
        let result = admit_runtime_script_lexical_v1(&statements, &[admission()]);
        let Ok(ScriptSemanticClosureAdmissionV1::Complete(facts)) = result else {
            panic!("ordinary unary must be Complete");
        };
        assert_eq!(facts.lexical.expression_source_indices.as_ref(), &[0]);
        assert!(facts.lexical.variables.is_empty());
    }

    #[test]
    fn print_admits_the_existing_expression_closure() {
        let statements = vec![ASTNode::Print {
            expression: Box::new(ASTNode::UnaryOp {
                operator: UnaryOperator::Minus,
                operand: Box::new(ASTNode::Literal {
                    value: LiteralValue::Integer(1),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }];
        let mut admission = admission();
        admission.admission = NormalScriptRuntimeStatementAdmissionV1::DirectPrint;
        let result = admit_runtime_script_lexical_v1(&statements, &[admission]);
        let Ok(ScriptSemanticClosureAdmissionV1::Complete(facts)) = result else {
            panic!("Print over the existing expression closure must be Complete");
        };
        assert_eq!(facts.lexical.expression_source_indices.as_ref(), &[0]);
        assert!(facts.lexical.variables.is_empty());
    }

    #[test]
    fn ordinary_binary_is_admitted_recursively() {
        let statements = vec![ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(1),
                span: Span::unknown(),
            }),
            right: Box::new(ASTNode::UnaryOp {
                operator: UnaryOperator::Minus,
                operand: Box::new(ASTNode::Literal {
                    value: LiteralValue::Integer(2),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }];
        let result = admit_runtime_script_lexical_v1(&statements, &[admission()]);
        assert!(matches!(
            result,
            Ok(ScriptSemanticClosureAdmissionV1::Complete(_))
        ));
    }

    #[test]
    fn logical_binary_is_admitted_recursively() {
        let statements = vec![ASTNode::BinaryOp {
            operator: BinaryOperator::And,
            left: Box::new(ASTNode::Literal {
                value: LiteralValue::Bool(true),
                span: Span::unknown(),
            }),
            right: Box::new(ASTNode::Literal {
                value: LiteralValue::Bool(false),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }];
        assert!(matches!(
            admit_runtime_script_lexical_v1(&statements, &[admission()]),
            Ok(ScriptSemanticClosureAdmissionV1::Complete(_))
        ));
    }

    #[test]
    fn await_admits_the_existing_expression_closure() {
        let statements = vec![ASTNode::AwaitExpression {
            expression: Box::new(ASTNode::UnaryOp {
                operator: UnaryOperator::Minus,
                operand: Box::new(ASTNode::Literal {
                    value: LiteralValue::Integer(1),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }];
        let result = admit_runtime_script_lexical_v1(&statements, &[admission()]);
        assert!(matches!(
            result,
            Ok(ScriptSemanticClosureAdmissionV1::Complete(_))
        ));
    }

    #[test]
    fn check_admits_items_through_the_existing_expression_closure() {
        let statements = vec![ASTNode::CheckExpr {
            name: None,
            items: vec![
                crate::ast::CheckItem {
                    label: None,
                    expression: ASTNode::Literal {
                        value: LiteralValue::Integer(1),
                        span: Span::unknown(),
                    },
                },
                crate::ast::CheckItem {
                    label: Some("second".to_owned()),
                    expression: ASTNode::UnaryOp {
                        operator: UnaryOperator::Minus,
                        operand: Box::new(ASTNode::Literal {
                            value: LiteralValue::Integer(2),
                            span: Span::unknown(),
                        }),
                        span: Span::unknown(),
                    },
                },
            ],
            span: Span::unknown(),
        }];
        let result = admit_runtime_script_lexical_v1(&statements, &[admission()]);
        assert!(matches!(
            result,
            Ok(ScriptSemanticClosureAdmissionV1::Complete(_))
        ));
    }

    #[test]
    fn weak_unary_remains_deferred() {
        let statements = vec![ASTNode::UnaryOp {
            operator: UnaryOperator::Weak,
            operand: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(1),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }];
        assert_eq!(
            admit_runtime_script_lexical_v1(&statements, &[admission()]),
            Ok(ScriptSemanticClosureAdmissionV1::Deferred(
                ScriptLexicalDeferredReasonV1::UnsafeRuntimeStatement
            ))
        );
    }

    #[test]
    fn static_const_exact_pair_is_a_zero_child_complete_boundary() {
        let statements = vec![static_const("TABLE")];
        let admissions = [LocatedNormalScriptRuntimeAdmissionV1 {
            source_statement_index: 2,
            admission: NormalScriptRuntimeStatementAdmissionV1::DirectStaticConstRuntimeCompletion,
        }];
        let result = admit_runtime_script_lexical_v1(&statements, &admissions);
        let Ok(ScriptSemanticClosureAdmissionV1::Complete(facts)) = result else {
            panic!("StaticConst exact pair must be Complete");
        };
        assert!(facts.lexical.locals.is_empty());
        assert_eq!(facts.static_const_completion_source_indices.as_ref(), &[2]);
    }

    #[test]
    fn static_const_admission_with_non_static_source_is_invariant_rejection() {
        let statements = vec![ASTNode::Literal {
            value: LiteralValue::Integer(1),
            span: Span::unknown(),
        }];
        let admissions = [LocatedNormalScriptRuntimeAdmissionV1 {
            source_statement_index: 0,
            admission: NormalScriptRuntimeStatementAdmissionV1::DirectStaticConstRuntimeCompletion,
        }];
        assert_eq!(
            admit_runtime_script_lexical_v1(&statements, &admissions),
            Err(super::ScriptSemanticAdmissionInvariantErrorV1::SourceAdmissionMismatch)
        );
    }

    #[test]
    fn static_const_with_unsupported_sibling_defers_the_whole_request() {
        let statements = vec![
            static_const("TABLE"),
            ASTNode::UnaryOp {
                operator: UnaryOperator::Weak,
                operand: Box::new(ASTNode::Literal {
                    value: LiteralValue::Integer(1),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            },
        ];
        let admissions = [
            LocatedNormalScriptRuntimeAdmissionV1 {
                source_statement_index: 0,
                admission:
                    NormalScriptRuntimeStatementAdmissionV1::DirectStaticConstRuntimeCompletion,
            },
            LocatedNormalScriptRuntimeAdmissionV1 {
                source_statement_index: 1,
                admission: NormalScriptRuntimeStatementAdmissionV1::DirectPortAwareExpression,
            },
        ];
        assert_eq!(
            admit_runtime_script_lexical_v1(&statements, &admissions),
            Ok(ScriptSemanticClosureAdmissionV1::Deferred(
                ScriptLexicalDeferredReasonV1::UnsafeRuntimeStatement
            ))
        );
    }
}
