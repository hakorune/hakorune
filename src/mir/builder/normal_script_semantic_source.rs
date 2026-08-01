//! Producer-backed Script semantic source for the lexical Complete closure.
//!
//! This product is intentionally narrow: the selected runtime window must be
//! closed by the admitted lexical expressions or an exact zero-child transfer
//! receipt before a Script owner is issued. It borrows the already-owned
//! Program while sealing one shared forest and projection; no raw source
//! carrier can manufacture the Complete loan.

use super::normal_script_semantic_lowering_state::ScriptSemanticLoweringState;
use crate::ast::ASTNode;
use crate::mir::compiler::source_projection::VerifiedSourceProjectionV1;
use crate::mir::resolved_semantics::{
    BindingRefV1, ResolvedAssignmentTargetV1, ScriptDiagnosticBoundaryV1,
    ScriptRootRuntimeDispositionV1, ScriptRootSemanticDispositionV1, ScriptTransferredBoundaryV1,
    SemanticOwnerForestDraftV1, SemanticOwnerRootProfileV1, SourceBindingSiteV1, SourceExprSiteV1,
    SourceNodeSiteV1, SourcePathSegmentV1, SourcePathV1, SourceStmtSiteV1,
    VerifiedResolvedScriptV1, VerifiedScriptRootDemandWindowV1, VerifiedSemanticOwnerForestV1,
    VerifiedSemanticOwnerProductV1,
};

use super::normal_default_root_catalog_lifecycle::PreparedNormalDefaultProgramRootV1;

#[derive(Debug)]
pub(super) struct VerifiedScriptSemanticSourceV1<'source> {
    source: &'source PreparedNormalDefaultProgramRootV1,
    forest: VerifiedSemanticOwnerForestV1,
    projection: VerifiedSourceProjectionV1,
    outbox_materializations: Box<[VerifiedScriptOutboxMaterializationV1]>,
    static_const_completions: Box<[VerifiedScriptStaticConstCompletionV1]>,
    using_directives: Box<[VerifiedScriptUsingDirectiveV1]>,
    existing_diagnostic_boundaries: Box<[VerifiedScriptExistingDiagnosticBoundaryV1]>,
    record_literal_demands: Box<[VerifiedScriptRecordLiteralDemandV1]>,
    qmark_propagations: Box<[VerifiedScriptQMarkPropagationV1]>,
    match_controls: Box<[VerifiedScriptMatchControlDemandV1]>,
    runtime_source_indices: Box<[usize]>,
}

#[derive(Debug)]
pub(super) struct VerifiedScriptStaticConstCompletionV1 {
    site: SourceStmtSiteV1,
}

#[derive(Debug)]
pub(super) struct VerifiedScriptOutboxMaterializationV1 {
    site: SourceStmtSiteV1,
    bindings: Box<[BindingRefV1]>,
}

#[derive(Debug)]
pub(super) struct VerifiedScriptUsingDirectiveV1 {
    site: SourceStmtSiteV1,
}

#[derive(Debug)]
pub(super) struct VerifiedScriptExistingDiagnosticBoundaryV1 {
    site: SourceStmtSiteV1,
    boundary: ScriptDiagnosticBoundaryV1,
}

#[derive(Debug)]
pub(super) struct VerifiedScriptRecordLiteralDemandV1 {
    site: SourceExprSiteV1,
    explicit_field_count: u32,
}

/// A source-only authorization for the existing QMark control/result owner.
/// It proves the exact operand site and that propagation targets this Script
/// execution owner; CFG, Return, and result materialization remain elsewhere.
#[derive(Debug)]
pub(super) struct VerifiedScriptQMarkPropagationV1 {
    site: SourceExprSiteV1,
    operand_site: SourceExprSiteV1,
    target: ScriptQMarkPropagationTargetV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ScriptQMarkPropagationTargetV1 {
    CurrentScriptOwner,
}

/// Source-only receipt for a root Match expression. Blocks, branches, PHI,
/// result materialization, and type publication remain with the raw owner.
#[derive(Debug)]
pub(super) struct VerifiedScriptMatchControlDemandV1 {
    site: SourceExprSiteV1,
    arm_count: u32,
}

impl<'source> VerifiedScriptSemanticSourceV1<'source> {
    pub(super) fn seal(
        source: &'source PreparedNormalDefaultProgramRootV1,
        product: VerifiedResolvedScriptV1,
        window: &VerifiedScriptRootDemandWindowV1,
    ) -> Result<Self, String> {
        let owner = product.core().data().owner;
        let mut draft = SemanticOwnerForestDraftV1::new();
        draft
            .insert_product(owner, VerifiedSemanticOwnerProductV1::Script(product))
            .map_err(|error| format!("[mir/script-semantic/forest] {error:?}"))?;
        let forest = draft
            .seal()
            .map_err(|error| format!("[mir/script-semantic/forest] {error:?}"))?;
        Self::seal_with_forest(source, forest, window)
    }

    pub(super) fn seal_with_forest(
        source: &'source PreparedNormalDefaultProgramRootV1,
        forest: VerifiedSemanticOwnerForestV1,
        window: &VerifiedScriptRootDemandWindowV1,
    ) -> Result<Self, String> {
        let [root] = forest.roots() else {
            return Err("[mir/script-semantic/forest] expected one Script root".to_owned());
        };
        let product = forest
            .semantic_owner(*root)
            .and_then(VerifiedSemanticOwnerProductV1::as_script)
            .ok_or_else(|| {
                "[mir/script-semantic/forest] expected Script root product".to_owned()
            })?;
        let ASTNode::Program { statements, .. } = source.source_ast() else {
            return Err("[mir/script-semantic/source-root] expected Program".to_owned());
        };
        let mut static_const_completions = Vec::new();
        let mut outbox_materializations = Vec::new();
        let mut using_directives = Vec::new();
        let mut existing_diagnostic_boundaries = Vec::new();
        let record_literal_demands = product
            .record_literal_demands()
            .map(|(site, explicit_field_count)| {
                let projected = crate::mir::resolved_semantics::project_source_node_v1(
                    source.source_ast(),
                    site.node(),
                )
                .ok_or_else(|| {
                    "[mir/script-semantic/record-projection] missing exact RecordLiteral".to_owned()
                })?;
                let crate::mir::resolved_semantics::ProjectedSourceNodeV1::Node(
                    ASTNode::RecordLiteral { fields, .. },
                ) = projected
                else {
                    return Err(
                        "[mir/script-semantic/record-site] expected RecordLiteral".to_owned()
                    );
                };
                if fields.len() != explicit_field_count as usize {
                    return Err("[mir/script-semantic/record-cardinality] mismatch".to_owned());
                }
                Ok(VerifiedScriptRecordLiteralDemandV1 {
                    site: site.clone(),
                    explicit_field_count,
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        let qmark_propagations = product
            .qmark_propagation_sites()
            .map(|site| {
                let statement_site = SourceStmtSiteV1::from_node(site.node().clone());
                let source_statement_index = program_statement_index(&statement_site)?;
                let Some(entry) = window.entry_at(source_statement_index) else {
                    return Err("[mir/script-semantic/qmark-window] missing root demand".to_owned());
                };
                if entry.site().node() != site.node()
                    || !matches!(
                        entry.semantic(),
                        ScriptRootSemanticDispositionV1::Resolved(
                            crate::mir::resolved_semantics::ScriptRootResolvedDemandV1::QMarkPropagation(_)
                        )
                    )
                {
                    return Err("[mir/script-semantic/qmark-window] source mismatch".to_owned());
                }
                let projected = crate::mir::resolved_semantics::project_source_node_v1(
                    source.source_ast(),
                    site.node(),
                )
                .ok_or_else(|| "[mir/script-semantic/qmark-projection] missing QMark".to_owned())?;
                let crate::mir::resolved_semantics::ProjectedSourceNodeV1::Node(
                    ASTNode::QMarkPropagate { .. },
                ) = projected
                else {
                    return Err("[mir/script-semantic/qmark-site] expected QMarkPropagate".to_owned());
                };
                let operand_site = SourcePathV1::from_node(site.node())
                    .child(SourcePathSegmentV1::QMarkOperand)
                    .expr();
                let projected_operand = crate::mir::resolved_semantics::project_source_node_v1(
                    source.source_ast(),
                    operand_site.node(),
                )
                .ok_or_else(|| {
                    "[mir/script-semantic/qmark-operand-projection] missing exact operand".to_owned()
                })?;
                if !matches!(
                    projected_operand,
                    crate::mir::resolved_semantics::ProjectedSourceNodeV1::Node(_)
                ) {
                    return Err("[mir/script-semantic/qmark-operand-site] expected node".to_owned());
                }
                Ok(VerifiedScriptQMarkPropagationV1 {
                    site: site.clone(),
                    operand_site,
                    target: ScriptQMarkPropagationTargetV1::CurrentScriptOwner,
                })
            })
            .collect::<Result<Vec<_>, String>>()?;
        let match_controls = product
            .match_control_sites()
            .map(|site| {
                let statement_site = SourceStmtSiteV1::from_node(site.node().clone());
                let source_statement_index = program_statement_index(&statement_site)?;
                let Some(entry) = window.entry_at(source_statement_index) else {
                    return Err("[mir/script-semantic/match-window] missing root demand".to_owned());
                };
                if entry.site().node() != site.node()
                    || !matches!(
                        entry.semantic(),
                        ScriptRootSemanticDispositionV1::Resolved(
                            crate::mir::resolved_semantics::ScriptRootResolvedDemandV1::MatchControl(_)
                        )
                    )
                {
                    return Err("[mir/script-semantic/match-window] source mismatch".to_owned());
                }
                let projected = crate::mir::resolved_semantics::project_source_node_v1(
                    source.source_ast(),
                    site.node(),
                )
                .ok_or_else(|| "[mir/script-semantic/match-projection] missing MatchExpr".to_owned())?;
                let crate::mir::resolved_semantics::ProjectedSourceNodeV1::Node(
                    ASTNode::MatchExpr {
                        arms, ..
                    },
                ) = projected
                else {
                    return Err("[mir/script-semantic/match-site] expected MatchExpr".to_owned());
                };
                let arm_count = u32::try_from(arms.len())
                    .map_err(|_| "[mir/script-semantic/match-arm-count] overflow".to_owned())?;
                let mut roles = Vec::with_capacity(arms.len() + 2);
                roles.push(SourcePathSegmentV1::MatchScrutinee);
                roles.extend((0..arm_count).map(SourcePathSegmentV1::MatchArm));
                roles.push(SourcePathSegmentV1::MatchElse);
                for role in roles {
                    let child_site = SourcePathV1::from_node(site.node()).child(role).expr();
                    if !matches!(
                        crate::mir::resolved_semantics::project_source_node_v1(
                            source.source_ast(),
                            child_site.node(),
                        ),
                        Some(crate::mir::resolved_semantics::ProjectedSourceNodeV1::Node(_))
                    ) {
                        return Err("[mir/script-semantic/match-child-site] expected node".to_owned());
                    }
                }
                Ok(VerifiedScriptMatchControlDemandV1 {
                    site: site.clone(),
                    arm_count,
                })
            })
            .collect::<Result<Vec<_>, String>>()?;
        let mut runtime_source_indices = Vec::new();
        for entry in window.entries() {
            let source_statement_index = program_statement_index(&entry.site())?;
            let Some(statement) = statements.get(source_statement_index) else {
                return Err(format!(
                    "[mir/script-semantic/window-coverage] source_statement_index={source_statement_index}"
                ));
            };
            match entry.semantic() {
                ScriptRootSemanticDispositionV1::Resolved(
                    crate::mir::resolved_semantics::ScriptRootResolvedDemandV1::LexicalCore,
                )
                    if matches!(statement, ASTNode::Outbox { .. }) =>
                {
                    let ASTNode::Outbox { variables, .. } = statement else {
                        unreachable!("Outbox match stays exact");
                    };
                    let bindings = variables
                        .iter()
                        .enumerate()
                        .map(|(ordinal, _)| {
                            product
                                .declaration_binding(&SourceBindingSiteV1::Outbox {
                                    statement: entry.site().clone(),
                                    ordinal: ordinal as u32,
                                })
                                .ok_or_else(|| {
                                    "[mir/script-semantic/outbox-binding] missing exact binding"
                                        .to_owned()
                                })
                        })
                        .collect::<Result<Box<[_]>, _>>()?;
                    outbox_materializations.push(VerifiedScriptOutboxMaterializationV1 {
                        site: entry.site().clone(),
                        bindings,
                    });
                }
                ScriptRootSemanticDispositionV1::Transferred(
                    ScriptTransferredBoundaryV1::ProgramStaticMetadata,
                ) if matches!(statement, ASTNode::StaticConstTable { .. }) => {
                    static_const_completions.push(VerifiedScriptStaticConstCompletionV1 {
                        site: entry.site().clone(),
                    });
                }
                ScriptRootSemanticDispositionV1::Transferred(
                    ScriptTransferredBoundaryV1::ProgramEnumDeclaration,
                ) if matches!(statement, ASTNode::EnumDeclaration { .. }) => {}
                ScriptRootSemanticDispositionV1::Transferred(
                    ScriptTransferredBoundaryV1::ProgramRecordDeclaration,
                ) if matches!(
                    statement,
                    ASTNode::BoxDeclaration {
                        is_record: true,
                        is_static: false,
                        is_sync: false,
                        ..
                    }
                ) => {}
                ScriptRootSemanticDispositionV1::Transparent(
                    crate::mir::resolved_semantics::ScriptTransparentBoundaryV1::UsingDirective,
                ) if matches!(statement, ASTNode::UsingStatement { .. }) => {
                    using_directives.push(VerifiedScriptUsingDirectiveV1 {
                        site: entry.site().clone(),
                    });
                }
                ScriptRootSemanticDispositionV1::Diagnostic(
                    ScriptDiagnosticBoundaryV1::ExistingSelectedUnsupported,
                ) if super::normal_script_program_item_admission::is_direct_selected_unsupported_statement_v1(statement) => {
                    existing_diagnostic_boundaries.push(VerifiedScriptExistingDiagnosticBoundaryV1 {
                        site: entry.site().clone(),
                        boundary: ScriptDiagnosticBoundaryV1::ExistingSelectedUnsupported,
                    });
                }
                ScriptRootSemanticDispositionV1::Diagnostic(
                    ScriptDiagnosticBoundaryV1::ExistingReceiverAbsent,
                ) if matches!(statement, ASTNode::Me { .. }) => {
                    existing_diagnostic_boundaries.push(VerifiedScriptExistingDiagnosticBoundaryV1 {
                        site: entry.site().clone(),
                        boundary: ScriptDiagnosticBoundaryV1::ExistingReceiverAbsent,
                    });
                }
                ScriptRootSemanticDispositionV1::Diagnostic(
                    ScriptDiagnosticBoundaryV1::ExistingBareThisUnsupported,
                ) if matches!(statement, ASTNode::This { .. }) => {
                    existing_diagnostic_boundaries.push(VerifiedScriptExistingDiagnosticBoundaryV1 {
                        site: entry.site().clone(),
                        boundary: ScriptDiagnosticBoundaryV1::ExistingBareThisUnsupported,
                    });
                }
                ScriptRootSemanticDispositionV1::Diagnostic(
                    ScriptDiagnosticBoundaryV1::ExistingContextScopeUnsupported,
                ) if matches!(statement, ASTNode::ContextScope { .. }) => {
                    existing_diagnostic_boundaries.push(VerifiedScriptExistingDiagnosticBoundaryV1 {
                        site: entry.site().clone(),
                        boundary: ScriptDiagnosticBoundaryV1::ExistingContextScopeUnsupported,
                    });
                }
                ScriptRootSemanticDispositionV1::Resolved(_)
                | ScriptRootSemanticDispositionV1::Deferred(_)
                | ScriptRootSemanticDispositionV1::Transferred(
                    ScriptTransferredBoundaryV1::TopLevelCallable,
                ) => {}
                _ => return Err("[mir/script-semantic/window-boundary] source mismatch".to_owned()),
            }
            if entry.runtime() == ScriptRootRuntimeDispositionV1::RetainedExistingTerminal {
                runtime_source_indices.push(source_statement_index);
            }
        }
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
            outbox_materializations: outbox_materializations.into_boxed_slice(),
            static_const_completions: static_const_completions.into_boxed_slice(),
            using_directives: using_directives.into_boxed_slice(),
            existing_diagnostic_boundaries: existing_diagnostic_boundaries.into_boxed_slice(),
            record_literal_demands: record_literal_demands.into_boxed_slice(),
            qmark_propagations: qmark_propagations.into_boxed_slice(),
            match_controls: match_controls.into_boxed_slice(),
            runtime_source_indices: runtime_source_indices.into_boxed_slice(),
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
    pub(super) fn qmark_propagations(
        &self,
    ) -> impl Iterator<
        Item = (
            &SourceExprSiteV1,
            &SourceExprSiteV1,
            &ScriptQMarkPropagationTargetV1,
        ),
    > {
        self.qmark_propagations
            .iter()
            .map(|receipt| (&receipt.site, &receipt.operand_site, &receipt.target))
    }

    #[cfg(test)]
    pub(super) fn match_controls(&self) -> impl Iterator<Item = (&SourceExprSiteV1, u32)> {
        self.match_controls
            .iter()
            .map(|receipt| (&receipt.site, receipt.arm_count))
    }

    #[cfg(test)]
    pub(super) fn outbox_materializations(
        &self,
    ) -> impl Iterator<Item = (&SourceStmtSiteV1, &[BindingRefV1])> {
        self.outbox_materializations
            .iter()
            .map(|receipt| (&receipt.site, receipt.bindings.as_ref()))
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

    #[cfg(test)]
    pub(super) fn using_directive_sites(&self) -> impl Iterator<Item = &SourceStmtSiteV1> {
        self.using_directives.iter().map(|receipt| &receipt.site)
    }

    #[cfg(test)]
    pub(super) fn receiver_absent_sites(&self) -> impl Iterator<Item = &SourceStmtSiteV1> {
        self.existing_diagnostic_boundaries
            .iter()
            .filter(|receipt| {
                receipt.boundary == ScriptDiagnosticBoundaryV1::ExistingReceiverAbsent
            })
            .map(|receipt| &receipt.site)
    }

    #[cfg(test)]
    pub(super) fn bare_this_unsupported_sites(&self) -> impl Iterator<Item = &SourceStmtSiteV1> {
        self.existing_diagnostic_boundaries
            .iter()
            .filter(|receipt| {
                receipt.boundary == ScriptDiagnosticBoundaryV1::ExistingBareThisUnsupported
            })
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

    pub(super) fn lowering_state(&self) -> Result<ScriptSemanticLoweringState, String> {
        let [root] = self.forest.roots() else {
            return Err("[freeze:contract][script-record/root-cardinality]".to_owned());
        };
        let Some(owner) = self.forest.semantic_owner(*root) else {
            return Err("[freeze:contract][script-record/root-owner]".to_owned());
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
        let nowaits = owner.declaration_sites().filter_map(|site| match site {
            SourceBindingSiteV1::Nowait { statement } => Some((
                statement.node().clone(),
                owner
                    .declaration_binding(site)
                    .expect("nowait declaration binding"),
            )),
            _ => None,
        });
        let outboxes = self.outbox_materializations.iter().map(|receipt| {
            (
                receipt.site.node().clone(),
                receipt.bindings.iter().copied(),
            )
        });
        let variables = owner
            .variable_refs()
            .filter_map(|(site, reference)| match reference {
                crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(binding) => {
                    Some((site.clone(), *binding))
                }
                _ => None,
            });
        let assignments = owner
            .assignment_targets()
            .filter_map(|(site, target)| match target {
                ResolvedAssignmentTargetV1::BindingRebind(binding) => {
                    Some((site.clone(), *binding))
                }
                ResolvedAssignmentTargetV1::UpvarRebind(_)
                | ResolvedAssignmentTargetV1::FieldWrite { .. }
                | ResolvedAssignmentTargetV1::IndexWrite { .. } => None,
            });
        let lambda_captures = self
            .forest
            .semantic_owners()
            .filter_map(|(child, _)| {
                let parent = self.forest.parent(child)?;
                Some((parent.definition_site().site().node().clone(), child))
            })
            .map(|(site, child)| {
                let captures = self
                    .forest
                    .ordered_capture_demands(child)
                    .iter()
                    .map(|demand| {
                        let binding = demand.source_binding();
                        let name = self
                            .forest
                            .semantic_owner(binding.owner())
                            .and_then(|owner| owner.binding(binding))
                            .ok_or_else(|| {
                                "[freeze:contract][script-lambda/capture-binding]".to_owned()
                            })?
                            .diagnostic_name()
                            .into();
                        Ok((name, binding))
                    })
                    .collect::<Result<Vec<_>, String>>()?
                    .into_boxed_slice();
                Ok((site, captures))
            })
            .collect::<Result<Vec<_>, String>>()?;
        let mut state = ScriptSemanticLoweringState::from_facts(
            locals,
            nowaits,
            outboxes,
            variables,
            assignments,
        );
        state.install_lambda_captures(lambda_captures)?;
        state.install_record_literal_demands(
            self.record_literal_demands
                .iter()
                .map(|receipt| (receipt.site.node().clone(), receipt.explicit_field_count)),
        )?;
        state.install_qmark_propagation_receipts(
            self.qmark_propagations
                .iter()
                .map(|receipt| receipt.site.node().clone()),
        )?;
        Ok(state)
    }
}

fn program_statement_index(site: &SourceStmtSiteV1) -> Result<usize, String> {
    match site.node().segments() {
        [crate::mir::resolved_semantics::SourcePathSegmentV1::ProgramBodyRoot, crate::mir::resolved_semantics::SourcePathSegmentV1::ProgramBody(index)] => {
            Ok(*index as usize)
        }
        _ => Err("[mir/script-semantic/window-site] expected ProgramBody ordinal".to_owned()),
    }
}

#[cfg(test)]
#[path = "normal_script_array_literal_tests.rs"]
mod array_literal_tests;
#[cfg(test)]
#[path = "normal_script_binding_rebind_tests.rs"]
mod binding_rebind_tests;
#[cfg(test)]
#[path = "normal_script_block_expr_tests.rs"]
mod block_expr_tests;
#[cfg(test)]
#[path = "normal_script_semantic_source_call_retention_tests.rs"]
mod call_retention_tests;
#[cfg(test)]
#[path = "normal_script_enum_declaration_tests.rs"]
mod enum_declaration_tests;
#[cfg(test)]
#[path = "normal_script_map_literal_tests.rs"]
mod map_literal_tests;
#[cfg(test)]
#[path = "normal_script_match_tests.rs"]
mod match_tests;
#[cfg(test)]
#[path = "normal_script_qmark_tests.rs"]
mod qmark_tests;
#[cfg(test)]
#[path = "normal_script_record_literal_tests.rs"]
mod record_literal_tests;
#[cfg(test)]
#[path = "normal_script_root_return_tests.rs"]
mod return_tests;
#[cfg(test)]
#[path = "normal_script_semantic_source_tests.rs"]
mod tests;
#[cfg(test)]
#[path = "normal_script_weak_reference_tests.rs"]
mod weak_reference_tests;
