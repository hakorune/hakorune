//! Producer-backed Script semantic source for the lexical Complete closure.
//!
//! This product is intentionally narrow: the selected runtime window must be
//! closed by the admitted lexical expressions or an exact zero-child transfer
//! receipt before a Script owner is issued. It borrows the already-owned
//! Program while sealing one shared forest and projection; no raw source
//! carrier can manufacture the Complete loan.

use super::normal_script_lexical_binding::{
    ScriptLexicalFactsV1, ScriptSemanticClosureAdmissionV1, ScriptSemanticClosureFactsV1,
    ScriptSemanticLoweringState,
};
use crate::ast::ASTNode;
use crate::mir::compiler::source_projection::VerifiedSourceProjectionV1;
use crate::mir::resolved_semantics::{
    BindingRefV1, FunctionOwnerIdV1, ResolvedScriptSemanticDraftV1, SemanticOwnerForestDraftV1,
    SemanticOwnerRootProfileV1, SourceBindingSiteV1, SourceNodeSiteV1, SourcePathSegmentV1,
    SourcePathV1, SourceStmtSiteV1, VerifiedSemanticOwnerForestV1, VerifiedSemanticOwnerProductV1,
};

use super::normal_default_root_catalog_lifecycle::PreparedNormalDefaultProgramRootV1;

#[derive(Debug)]
pub(super) struct VerifiedScriptSemanticSourceV1<'source> {
    source: &'source PreparedNormalDefaultProgramRootV1,
    forest: VerifiedSemanticOwnerForestV1,
    projection: VerifiedSourceProjectionV1,
    static_const_completions: Box<[VerifiedScriptStaticConstCompletionV1]>,
    existing_diagnostic_boundaries: Box<[VerifiedScriptExistingDiagnosticBoundaryV1]>,
    runtime_source_indices: Box<[usize]>,
}

#[derive(Debug)]
pub(super) struct VerifiedScriptStaticConstCompletionV1 {
    site: SourceStmtSiteV1,
}

#[derive(Debug)]
pub(super) struct VerifiedScriptExistingDiagnosticBoundaryV1 {
    site: SourceStmtSiteV1,
}

impl<'source> VerifiedScriptSemanticSourceV1<'source> {
    pub(super) fn seal(
        source: &'source PreparedNormalDefaultProgramRootV1,
        owner: FunctionOwnerIdV1,
        admission: ScriptSemanticClosureAdmissionV1,
    ) -> Result<Self, String> {
        let ASTNode::Program { statements, .. } = source.source_ast() else {
            return Err("[mir/script-semantic/source-root] expected Program".to_owned());
        };
        let ScriptSemanticClosureAdmissionV1::Complete(ScriptSemanticClosureFactsV1 {
            lexical:
                ScriptLexicalFactsV1 {
                    locals,
                    variables,
                    expression_source_indices,
                },
            static_const_completion_source_indices,
            existing_diagnostic_source_indices,
        }) = admission
        else {
            return Err("[mir/script-semantic/admission] Deferred input cannot seal".to_owned());
        };
        let mut draft = ResolvedScriptSemanticDraftV1::new(owner);
        let mut local_bindings = std::collections::BTreeMap::new();
        for local in &locals {
            let site = program_statement_site(local.source_statement_index);
            if !matches!(
                statements.get(local.source_statement_index),
                Some(ASTNode::Local { .. })
            ) {
                return Err(format!(
                    "[mir/script-semantic/local-coverage] source_statement_index={}",
                    local.source_statement_index
                ));
            }
            let binding = draft.declare_local(site, local.name.clone());
            local_bindings.insert(local.source_statement_index, binding);
        }
        for variable in &variables {
            let Some(binding) = local_bindings
                .get(&variable.binding_statement_index)
                .copied()
            else {
                return Err(
                    "[mir/script-semantic/variable-binding] missing local binding".to_owned(),
                );
            };
            let mut path = SourcePathV1::from_node(
                &program_statement_site(variable.source_statement_index)
                    .node()
                    .clone(),
            );
            if variable.initializer {
                path = path.child(SourcePathSegmentV1::Initializer(0));
            }
            for segment in &variable.path {
                path = path.child(segment.clone());
            }
            let site = path.expr();
            draft.record_variable(site, binding);
        }
        let mut static_const_completions = Vec::new();
        for &source_statement_index in &static_const_completion_source_indices {
            if !matches!(
                statements.get(source_statement_index),
                Some(ASTNode::StaticConstTable { .. })
            ) {
                return Err(format!(
                    "[mir/script-semantic/static-const-coverage] source_statement_index={source_statement_index}"
                ));
            }
            static_const_completions.push(VerifiedScriptStaticConstCompletionV1 {
                site: program_statement_site(source_statement_index),
            });
        }
        let product = draft
            .seal()
            .map_err(|error| format!("[mir/script-semantic/seal] {error:?}"))?;
        let mut existing_diagnostic_boundaries = Vec::new();
        for &source_statement_index in &existing_diagnostic_source_indices {
            let Some(statement) = statements.get(source_statement_index) else {
                return Err(format!(
                    "[mir/script-semantic/diagnostic-coverage] source_statement_index={source_statement_index}"
                ));
            };
            if !super::normal_script_program_item_admission::is_direct_selected_unsupported_statement_v1(statement) {
                return Err(format!(
                    "[mir/script-semantic/diagnostic-coverage] source_statement_index={source_statement_index}"
                ));
            }
            existing_diagnostic_boundaries.push(VerifiedScriptExistingDiagnosticBoundaryV1 {
                site: program_statement_site(source_statement_index),
            });
        }
        let runtime_source_indices = locals
            .iter()
            .map(|local| local.source_statement_index)
            .chain(expression_source_indices.into_vec())
            .chain(static_const_completion_source_indices.iter().copied())
            .chain(existing_diagnostic_source_indices.iter().copied())
            .collect::<std::collections::BTreeSet<_>>()
            .into_iter()
            .collect::<Box<[_]>>();
        let mut draft = SemanticOwnerForestDraftV1::new();
        draft
            .insert_product(owner, VerifiedSemanticOwnerProductV1::Script(product))
            .map_err(|error| format!("[mir/script-semantic/forest] {error:?}"))?;
        let forest = draft
            .seal()
            .map_err(|error| format!("[mir/script-semantic/forest] {error:?}"))?;
        let projection = VerifiedSourceProjectionV1::seal_with_root_profile(
            source.source_ast(),
            &forest,
            SemanticOwnerRootProfileV1::Script,
        )
        .map_err(|error| format!("[mir/script-semantic/projection] {error}"))?;
        Ok(Self {
            source,
            forest,
            projection,
            static_const_completions: static_const_completions.into_boxed_slice(),
            existing_diagnostic_boundaries: existing_diagnostic_boundaries.into_boxed_slice(),
            runtime_source_indices,
        })
    }

    pub(super) fn source(&self) -> &PreparedNormalDefaultProgramRootV1 {
        &self.source
    }

    pub(super) fn forest(&self) -> &VerifiedSemanticOwnerForestV1 {
        &self.forest
    }

    pub(super) fn projection(&self) -> &VerifiedSourceProjectionV1 {
        &self.projection
    }

    pub(super) fn runtime_source_indices(&self) -> &[usize] {
        &self.runtime_source_indices
    }

    #[cfg(test)]
    pub(super) fn static_const_completion_sites(&self) -> impl Iterator<Item = &SourceStmtSiteV1> {
        self.static_const_completions
            .iter()
            .map(|receipt| &receipt.site)
    }

    #[cfg(test)]
    pub(super) fn existing_diagnostic_sites(&self) -> impl Iterator<Item = &SourceStmtSiteV1> {
        self.existing_diagnostic_boundaries
            .iter()
            .map(|receipt| &receipt.site)
    }

    pub(super) fn local_binding_at(&self, site: &SourceNodeSiteV1) -> Option<BindingRefV1> {
        let [root] = self.forest.roots() else {
            return None;
        };
        let owner = self.forest.semantic_owner(*root)?;
        owner.declaration_binding(&SourceBindingSiteV1::Local {
            statement: SourceStmtSiteV1::from_node(site.clone()),
            ordinal: 0,
        })
    }

    pub(super) fn variable_binding_at(&self, site: &SourceNodeSiteV1) -> Option<BindingRefV1> {
        let [root] = self.forest.roots() else {
            return None;
        };
        let owner = self.forest.semantic_owner(*root)?;
        owner.variable_refs().find_map(|(candidate, reference)| {
            if candidate.node() != site {
                return None;
            }
            match reference {
                crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(binding) => {
                    Some(*binding)
                }
                _ => None,
            }
        })
    }

    pub(super) fn lowering_state(&self) -> ScriptSemanticLoweringState {
        let [root] = self.forest.roots() else {
            return ScriptSemanticLoweringState::default();
        };
        let Some(owner) = self.forest.semantic_owner(*root) else {
            return ScriptSemanticLoweringState::default();
        };
        let locals = owner.declaration_sites().filter_map(|site| match site {
            SourceBindingSiteV1::Local { statement, .. } => Some((
                statement.node().clone(),
                owner
                    .declaration_binding(site)
                    .expect("local declaration binding"),
            )),
            _ => None,
        });
        let variables = owner
            .variable_refs()
            .filter_map(|(site, reference)| match reference {
                crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(binding) => {
                    Some((site.clone(), *binding))
                }
                _ => None,
            });
        ScriptSemanticLoweringState::from_facts(locals, variables)
    }
}

fn program_statement_site(index: usize) -> SourceStmtSiteV1 {
    SourceStmtSiteV1::from_node(
        SourcePathV1::program_body()
            .child(SourcePathSegmentV1::ProgramBody(index as u32))
            .node(),
    )
}

#[cfg(test)]
#[path = "normal_script_semantic_source_tests.rs"]
mod tests;
