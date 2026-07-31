//! Source-only lexical admission and the request-local lowering ledger.
//!
//! This module deliberately stops at BindingRef facts. Runtime ValueIds are
//! added later, after the existing Local owner has materialized a value.

use std::collections::BTreeMap;

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
pub(super) enum ScriptLexicalAdmissionV1 {
    Complete(ScriptLexicalFactsV1),
    Deferred(ScriptLexicalDeferredReasonV1),
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
) -> ScriptLexicalAdmissionV1 {
    if statements.len() != admissions.len() {
        return ScriptLexicalAdmissionV1::Deferred(
            ScriptLexicalDeferredReasonV1::UnsafeRuntimeStatement,
        );
    }
    let mut locals = Vec::new();
    let mut visible = BTreeMap::<String, usize>::new();
    let mut variables = Vec::new();
    let mut expression_source_indices = Vec::new();
    for (statement, admission) in statements.iter().zip(admissions) {
        if !matches!(
            admission.admission,
            NormalScriptRuntimeStatementAdmissionV1::DirectPortAwareExpression
                | NormalScriptRuntimeStatementAdmissionV1::DirectPrint
        ) {
            return ScriptLexicalAdmissionV1::Deferred(
                ScriptLexicalDeferredReasonV1::UnsafeRuntimeStatement,
            );
        }
        let index = admission.source_statement_index;
        match statement {
            ASTNode::Print { expression, .. } => {
                let Some(segment) = ExprChildRoleV1::PrintValue.segment_for(statement) else {
                    return ScriptLexicalAdmissionV1::Deferred(
                        ScriptLexicalDeferredReasonV1::UnsafeRuntimeStatement,
                    );
                };
                let path = [segment];
                if let Err(reason) =
                    admit_expression_v1(expression, index, false, &path, &visible, &mut variables)
                {
                    return ScriptLexicalAdmissionV1::Deferred(reason);
                }
                expression_source_indices.push(index);
            }
            ASTNode::Literal { .. } | ASTNode::Variable { .. } | ASTNode::UnaryOp { .. } => {
                if let Err(reason) =
                    admit_expression_v1(statement, index, false, &[], &visible, &mut variables)
                {
                    return ScriptLexicalAdmissionV1::Deferred(reason);
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
                    return ScriptLexicalAdmissionV1::Deferred(
                        ScriptLexicalDeferredReasonV1::LocalShape,
                    );
                }
                let Some(initializer) = initial_values[0].as_deref() else {
                    return ScriptLexicalAdmissionV1::Deferred(
                        ScriptLexicalDeferredReasonV1::LocalShape,
                    );
                };
                if let Err(reason) =
                    admit_expression_v1(initializer, index, true, &[], &visible, &mut variables)
                {
                    return ScriptLexicalAdmissionV1::Deferred(reason);
                }
                visible.insert(names[0].clone(), index);
                locals.push(ScriptLocalFactV1 {
                    source_statement_index: index,
                    name: names[0].clone(),
                });
            }
            _ => {
                return ScriptLexicalAdmissionV1::Deferred(
                    ScriptLexicalDeferredReasonV1::UnsafeRuntimeStatement,
                )
            }
        }
    }
    ScriptLexicalAdmissionV1::Complete(ScriptLexicalFactsV1 {
        locals: locals.into_boxed_slice(),
        variables: variables.into_boxed_slice(),
        expression_source_indices: expression_source_indices.into_boxed_slice(),
    })
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
        NormalScriptRuntimeStatementAdmissionV1, ScriptLexicalAdmissionV1,
        ScriptLexicalDeferredReasonV1,
    };
    use crate::ast::{ASTNode, LiteralValue, Span, UnaryOperator};

    fn admission() -> LocatedNormalScriptRuntimeAdmissionV1 {
        LocatedNormalScriptRuntimeAdmissionV1 {
            source_statement_index: 0,
            admission: NormalScriptRuntimeStatementAdmissionV1::DirectPortAwareExpression,
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
        let ScriptLexicalAdmissionV1::Complete(facts) = result else {
            panic!("ordinary unary must be Complete");
        };
        assert_eq!(facts.expression_source_indices.as_ref(), &[0]);
        assert!(facts.variables.is_empty());
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
        let ScriptLexicalAdmissionV1::Complete(facts) = result else {
            panic!("Print over the existing expression closure must be Complete");
        };
        assert_eq!(facts.expression_source_indices.as_ref(), &[0]);
        assert!(facts.variables.is_empty());
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
            ScriptLexicalAdmissionV1::Deferred(
                ScriptLexicalDeferredReasonV1::UnsafeRuntimeStatement
            )
        );
    }
}
