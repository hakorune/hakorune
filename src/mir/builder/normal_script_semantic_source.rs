//! Producer-backed Script semantic source for the lexical Complete closure.
//!
//! This product is intentionally narrow: the selected runtime window must be
//! closed by the admitted lexical expressions or an exact zero-child transfer
//! receipt before a Script owner is issued. It borrows the already-owned
//! Program while sealing one shared forest and projection; no raw source
//! carrier can manufacture the Complete loan.

use super::normal_script_boundary_receipt_pack::ScriptBoundaryReceiptPackV1;
use super::normal_script_direct_static_join_handoff::VerifiedScriptDirectStaticJoinHandoffV1;
use super::normal_script_direct_static_recipe::VerifiedScriptDirectStaticRecipeV1;
use super::normal_script_direct_static_result_bundle::VerifiedScriptDirectStaticResultBundleV1;
use super::normal_script_direct_static_result_publication_owner::VerifiedScriptDirectStaticResultPublicationOwnerV1;
use super::normal_script_operational_demand_receipt_pack::ScriptOperationalDemandReceiptPackV1;
#[cfg(test)]
use super::normal_script_operational_demand_receipt_pack::ScriptQMarkPropagationTargetV1;
use super::normal_script_semantic_lowering_input::VerifiedScriptSemanticLoweringInputV1;
use super::normal_script_semantic_lowering_projection::VerifiedScriptLoweringProjectionV1;
use super::normal_script_semantic_source_core::ScriptSemanticSourceCoreV1;
use super::normal_script_source_continuation::VerifiedScriptSourceContinuationV1;
use crate::mir::compiler::source_projection::VerifiedSourceProjectionV1;
use crate::mir::resolved_semantics::{
    BindingRefV1, SemanticOwnerForestDraftV1, SourceNodeSiteV1, VerifiedResolvedScriptV1,
    VerifiedScriptRootDemandWindowV1, VerifiedSemanticOwnerForestV1,
    VerifiedSemanticOwnerProductV1,
};
#[cfg(test)]
use crate::mir::resolved_semantics::{
    EnumVariantAdmissionV1, ScriptDiagnosticBoundaryV1, SourceExprSiteV1, SourceStmtSiteV1,
};

use super::normal_default_root_catalog_lifecycle::PreparedNormalDefaultProgramRootV1;

#[derive(Debug)]
pub(super) struct VerifiedScriptSemanticSourceV1<'source> {
    core: ScriptSemanticSourceCoreV1<'source>,
    boundaries: ScriptBoundaryReceiptPackV1,
    demands: ScriptOperationalDemandReceiptPackV1,
    lowering_projection: VerifiedScriptLoweringProjectionV1,
    continuation: VerifiedScriptSourceContinuationV1,
    direct_static_result_bundle: Option<VerifiedScriptDirectStaticResultBundleV1>,
    direct_static_result_publication_owner:
        Option<VerifiedScriptDirectStaticResultPublicationOwnerV1>,
    direct_static_recipe: Option<VerifiedScriptDirectStaticRecipeV1>,
    direct_static_join_handoff: Option<VerifiedScriptDirectStaticJoinHandoffV1>,
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
        Self::seal_ast_with_forest(source.source_ast(), forest, window)
    }

    pub(super) fn seal_ast_with_forest(
        source: &'source crate::ast::ASTNode,
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
        let continuation = VerifiedScriptSourceContinuationV1::issue(&forest, window)
            .map_err(|error| format!("[mir/script-semantic/continuation] {error:?}"))?;
        let core = ScriptSemanticSourceCoreV1::seal(
            source,
            forest,
            boundaries
                .runtime_source_indices()
                .to_vec()
                .into_boxed_slice(),
        )?;
        let lowering_projection =
            VerifiedScriptLoweringProjectionV1::seal(&core, &boundaries, &demands)?;
        Ok(Self {
            core,
            boundaries,
            demands,
            lowering_projection,
            continuation,
            direct_static_result_bundle: None,
            direct_static_result_publication_owner: None,
            direct_static_recipe: None,
            direct_static_join_handoff: None,
        })
    }

    pub(super) fn attach_direct_static_result_bundle(
        &mut self,
        bundle: VerifiedScriptDirectStaticResultBundleV1,
    ) -> Result<(), &'static str> {
        if self.direct_static_result_bundle.replace(bundle).is_some() {
            return Err("duplicate Script direct-static result bundle");
        }
        Ok(())
    }

    pub(super) fn direct_static_result_bundle(
        &self,
    ) -> Option<&VerifiedScriptDirectStaticResultBundleV1> {
        self.direct_static_result_bundle.as_ref()
    }

    pub(super) fn attach_direct_static_result_publication_owner(
        &mut self,
        owner: VerifiedScriptDirectStaticResultPublicationOwnerV1,
    ) -> Result<(), &'static str> {
        if self
            .direct_static_result_publication_owner
            .replace(owner)
            .is_some()
        {
            return Err("duplicate Script direct-static result publication owner");
        }
        Ok(())
    }

    pub(super) fn direct_static_result_publication_owner(
        &self,
    ) -> Option<&VerifiedScriptDirectStaticResultPublicationOwnerV1> {
        self.direct_static_result_publication_owner.as_ref()
    }

    pub(super) fn attach_direct_static_recipe(
        &mut self,
        recipe: VerifiedScriptDirectStaticRecipeV1,
    ) -> Result<(), &'static str> {
        if self.direct_static_recipe.replace(recipe).is_some() {
            return Err("duplicate Script direct-static Recipe");
        }
        Ok(())
    }

    pub(super) fn direct_static_recipe(&self) -> Option<&VerifiedScriptDirectStaticRecipeV1> {
        self.direct_static_recipe.as_ref()
    }

    pub(super) fn attach_direct_static_join_handoff(
        &mut self,
        handoff: VerifiedScriptDirectStaticJoinHandoffV1,
    ) -> Result<(), &'static str> {
        if self.direct_static_join_handoff.replace(handoff).is_some() {
            return Err("duplicate Script direct-static Join handoff");
        }
        Ok(())
    }

    pub(super) fn direct_static_join_handoff(
        &self,
    ) -> Option<&VerifiedScriptDirectStaticJoinHandoffV1> {
        self.direct_static_join_handoff.as_ref()
    }

    pub(super) fn continuation(&self) -> &VerifiedScriptSourceContinuationV1 {
        &self.continuation
    }

    pub(super) fn source(&self) -> &crate::ast::ASTNode {
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
        self.lowering_projection.local_binding_at(site)
    }

    pub(super) fn variable_binding_at(&self, site: &SourceNodeSiteV1) -> Option<BindingRefV1> {
        self.lowering_projection.variable_binding_at(site)
    }

    pub(super) fn into_lowering_input(self) -> VerifiedScriptSemanticLoweringInputV1 {
        VerifiedScriptSemanticLoweringInputV1::new(
            self.lowering_projection,
            self.continuation,
            self.direct_static_result_bundle,
            self.direct_static_result_publication_owner,
            self.direct_static_recipe,
            self.direct_static_join_handoff,
        )
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
