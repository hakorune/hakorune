//! Producer-backed Script semantic source for the lexical Complete closure.
//!
//! This product is intentionally narrow: the selected runtime window must be
//! closed by the admitted lexical expressions or an exact zero-child transfer
//! receipt before a Script owner is issued. It borrows the already-owned
//! Program while sealing one shared forest and projection; no raw source
//! carrier can manufacture the Complete loan.

use super::normal_script_boundary_receipt_pack::ScriptBoundaryReceiptPackV1;
use super::normal_script_operational_demand_receipt_pack::{
    ScriptOperationalDemandReceiptPackV1, ScriptQMarkPropagationTargetV1,
};
use super::normal_script_semantic_lowering_state::ScriptSemanticLoweringState;
use super::normal_script_semantic_source_core::ScriptSemanticSourceCoreV1;
use crate::mir::compiler::source_projection::VerifiedSourceProjectionV1;
use crate::mir::resolved_semantics::{
    BindingRefV1, EnumVariantAdmissionV1, ResolvedAssignmentTargetV1, ScriptDiagnosticBoundaryV1,
    SemanticOwnerForestDraftV1, SourceBindingSiteV1, SourceExprSiteV1, SourceNodeSiteV1,
    SourceStmtSiteV1, VerifiedResolvedScriptV1, VerifiedScriptRootDemandWindowV1,
    VerifiedSemanticOwnerForestV1, VerifiedSemanticOwnerProductV1,
};

use super::normal_default_root_catalog_lifecycle::PreparedNormalDefaultProgramRootV1;

#[derive(Debug)]
pub(super) struct VerifiedScriptSemanticSourceV1<'source> {
    core: ScriptSemanticSourceCoreV1<'source>,
    boundaries: ScriptBoundaryReceiptPackV1,
    demands: ScriptOperationalDemandReceiptPackV1,
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
        let boundaries = ScriptBoundaryReceiptPackV1::seal(source, product, window)?;
        let demands = ScriptOperationalDemandReceiptPackV1::seal(source, product, window)?;
        let core = ScriptSemanticSourceCoreV1::seal(
            source,
            forest,
            boundaries
                .runtime_source_indices()
                .to_vec()
                .into_boxed_slice(),
        )?;
        Ok(Self {
            core,
            boundaries,
            demands,
        })
    }

    pub(super) fn source(&self) -> &PreparedNormalDefaultProgramRootV1 {
        self.core.source()
    }

    pub(super) fn forest(&self) -> &VerifiedSemanticOwnerForestV1 {
        self.core.forest()
    }

    pub(super) fn projection(&self) -> &VerifiedSourceProjectionV1 {
        self.core.projection()
    }

    pub(super) fn runtime_source_indices(&self) -> &[usize] {
        self.core.runtime_source_indices()
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
        self.demands
            .qmark_propagations()
            .iter()
            .map(|receipt| (&receipt.site, &receipt.operand_site, &receipt.target))
    }

    #[cfg(test)]
    pub(super) fn match_controls(&self) -> impl Iterator<Item = (&SourceExprSiteV1, u32)> {
        self.demands
            .match_controls()
            .iter()
            .map(|receipt| (&receipt.site, receipt.arm_count))
    }

    #[cfg(test)]
    pub(super) fn enum_variant_demands(
        &self,
    ) -> impl Iterator<Item = (&SourceExprSiteV1, &EnumVariantAdmissionV1)> {
        self.demands
            .enum_variant_demands()
            .iter()
            .map(|receipt| (&receipt.site, &receipt.admission))
    }

    #[cfg(test)]
    pub(super) fn enum_match_demands(&self) -> impl Iterator<Item = &SourceExprSiteV1> {
        self.demands
            .enum_match_demands()
            .iter()
            .map(|receipt| &receipt.site)
    }

    #[cfg(test)]
    pub(super) fn outbox_materializations(
        &self,
    ) -> impl Iterator<Item = (&SourceStmtSiteV1, &[BindingRefV1])> {
        self.boundaries
            .outbox_materializations()
            .iter()
            .map(|receipt| (&receipt.site, receipt.bindings.as_ref()))
    }

    #[cfg(test)]
    pub(super) fn static_const_completion_sites(&self) -> impl Iterator<Item = &SourceStmtSiteV1> {
        self.boundaries
            .static_const_completions()
            .iter()
            .map(|receipt| &receipt.site)
    }

    #[cfg(test)]
    pub(super) fn existing_diagnostic_sites(&self) -> impl Iterator<Item = &SourceStmtSiteV1> {
        self.boundaries
            .existing_diagnostic_boundaries()
            .iter()
            .map(|receipt| &receipt.site)
    }

    #[cfg(test)]
    pub(super) fn using_directive_sites(&self) -> impl Iterator<Item = &SourceStmtSiteV1> {
        self.boundaries
            .using_directives()
            .iter()
            .map(|receipt| &receipt.site)
    }

    #[cfg(test)]
    pub(super) fn receiver_absent_sites(&self) -> impl Iterator<Item = &SourceStmtSiteV1> {
        self.boundaries
            .existing_diagnostic_boundaries()
            .iter()
            .filter(|receipt| {
                receipt.boundary == ScriptDiagnosticBoundaryV1::ExistingReceiverAbsent
            })
            .map(|receipt| &receipt.site)
    }

    #[cfg(test)]
    pub(super) fn bare_this_unsupported_sites(&self) -> impl Iterator<Item = &SourceStmtSiteV1> {
        self.boundaries
            .existing_diagnostic_boundaries()
            .iter()
            .filter(|receipt| {
                receipt.boundary == ScriptDiagnosticBoundaryV1::ExistingBareThisUnsupported
            })
            .map(|receipt| &receipt.site)
    }

    pub(super) fn local_binding_at(&self, site: &SourceNodeSiteV1) -> Option<BindingRefV1> {
        let [root] = self.core.forest().roots() else {
            return None;
        };
        let owner = self.core.forest().semantic_owner(*root)?;
        owner.declaration_binding(&SourceBindingSiteV1::Local {
            statement: SourceStmtSiteV1::from_node(site.clone()),
            ordinal: 0,
        })
    }

    pub(super) fn variable_binding_at(&self, site: &SourceNodeSiteV1) -> Option<BindingRefV1> {
        let [root] = self.core.forest().roots() else {
            return None;
        };
        let owner = self.core.forest().semantic_owner(*root)?;
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
        let [root] = self.core.forest().roots() else {
            return Err("[freeze:contract][script-record/root-cardinality]".to_owned());
        };
        let Some(owner) = self.core.forest().semantic_owner(*root) else {
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
        let outboxes = self
            .boundaries
            .outbox_materializations()
            .iter()
            .map(|receipt| {
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
            .core
            .forest()
            .semantic_owners()
            .filter_map(|(child, _)| {
                let parent = self.core.forest().parent(child)?;
                Some((parent.definition_site().site().node().clone(), child))
            })
            .map(|(site, child)| {
                let captures = self
                    .core
                    .forest()
                    .ordered_capture_demands(child)
                    .iter()
                    .map(|demand| {
                        let binding = demand.source_binding();
                        let name = self
                            .core
                            .forest()
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
            self.demands
                .record_literal_demands()
                .iter()
                .map(|receipt| (receipt.site.node().clone(), receipt.explicit_field_count)),
        )?;
        state.install_enum_variant_demands(
            self.demands
                .enum_variant_demands()
                .iter()
                .map(|receipt| (receipt.site.node().clone(), receipt.admission.clone())),
        )?;
        state.install_enum_match_scrutinee_receipts(
            self.demands
                .enum_match_demands()
                .iter()
                .map(|receipt| receipt.site.node().clone()),
        )?;
        state.install_qmark_propagation_receipts(
            self.demands
                .qmark_propagations()
                .iter()
                .map(|receipt| receipt.site.node().clone()),
        )?;
        Ok(state)
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
#[path = "normal_script_enum_match_tests.rs"]
mod enum_match_tests;
#[cfg(test)]
#[path = "normal_script_enum_variant_tests.rs"]
mod enum_variant_tests;
#[cfg(test)]
#[path = "normal_script_index_write_tests.rs"]
mod index_write_tests;
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
#[path = "normal_script_semantic_source_runtime_tests.rs"]
mod runtime_tests;
#[cfg(test)]
#[path = "normal_script_semantic_source_tests.rs"]
mod tests;
#[cfg(test)]
#[path = "normal_script_weak_reference_tests.rs"]
mod weak_reference_tests;
