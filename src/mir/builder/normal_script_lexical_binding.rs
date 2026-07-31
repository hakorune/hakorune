//! Source-only lexical admission and the request-local lowering ledger.
//!
//! This module deliberately stops at BindingRef facts. Runtime ValueIds are
//! added later, after the existing Local owner has materialized a value.

use std::collections::BTreeMap;

use crate::ast::ASTNode;
use crate::mir::resolved_semantics::{BindingRefV1, SourceExprSiteV1, SourceNodeSiteV1};
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
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct ScriptLexicalFactsV1 {
    pub(super) locals: Box<[ScriptLocalFactV1]>,
    pub(super) variables: Box<[ScriptVariableFactV1]>,
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
    for (statement, admission) in statements.iter().zip(admissions) {
        if !matches!(
            admission.admission,
            NormalScriptRuntimeStatementAdmissionV1::DirectPortAwareExpression
        ) {
            return ScriptLexicalAdmissionV1::Deferred(
                ScriptLexicalDeferredReasonV1::UnsafeRuntimeStatement,
            );
        }
        let index = admission.source_statement_index;
        match statement {
            ASTNode::Literal { .. } => {}
            ASTNode::Variable { name, .. } => {
                let Some(&binding_statement_index) = visible.get(name) else {
                    return ScriptLexicalAdmissionV1::Deferred(
                        ScriptLexicalDeferredReasonV1::UndefinedVariable,
                    );
                };
                variables.push(ScriptVariableFactV1 {
                    source_statement_index: index,
                    binding_statement_index,
                    initializer: false,
                });
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
                match initializer {
                    ASTNode::Literal { .. } => {}
                    ASTNode::Variable { name, .. } => {
                        let Some(&binding_statement_index) = visible.get(name) else {
                            return ScriptLexicalAdmissionV1::Deferred(
                                ScriptLexicalDeferredReasonV1::UndefinedVariable,
                            );
                        };
                        variables.push(ScriptVariableFactV1 {
                            source_statement_index: index,
                            binding_statement_index,
                            initializer: true,
                        });
                    }
                    _ => {
                        return ScriptLexicalAdmissionV1::Deferred(
                            ScriptLexicalDeferredReasonV1::LocalShape,
                        )
                    }
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
    })
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
