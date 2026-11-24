//! Existing source call handoff installation and consumption.

use super::*;

impl CallableSemanticLoweringState {

    /// The shared expression writer already reads operands and emits the Core.
    /// Discharge its SAME staged source without reading or emitting it again.
    pub(in crate::mir::builder) fn complete_expression_core_source_v1(
        &mut self, loan: &crate::mir::normal_callable_semantic_package::ExpressionCoreSourceLoanV1,
    ) -> Result<(), String> {
        let original = loan.source();
        let site = original.source().site();
        if self.consumed_source_core_method_calls.contains(site) {
            return Err(freeze("duplicate-core-method-call-consumption"));
        }
        let Some(row) = self.source_core_method_calls.get(site) else {
            // Ordinary Core expressions outside a Loop have no staged Loop row.
            return Ok(());
        };
        let contract = row.contract();
        let target = contract.target();
        if contract.owner() != self.owner || contract.named_array_requirement().is_some()
            || !std::sync::Arc::ptr_eq(contract.source(), original.source())
            || target.schema() != original.target().schema()
            || !std::ptr::eq(target.row().row(), original.target().row().row())
            || target.row().arity() != original.target().row().arity()
            || target.result() != original.target().result()
            || target.receiver() != original.target().receiver()
            || target.parameters() != original.target().parameters()
        { return Err(freeze("core-method-call-target")); }
        self.source_core_method_calls.remove(site);
        self.consumed_source_core_method_calls.insert(site.clone());
        Ok(())
    }

    pub(in crate::mir::builder) fn install_source_static_result_publication(
        &mut self,
        site: &crate::mir::resolved_semantics::SourceExprSiteV1,
        handoff: VerifiedStaticCallResultPublicationHandoffV1,
    ) -> Result<(), String> {
        if handoff.site() != site {
            return Err(freeze("static-publication-site-mismatch"));
        }
        if !matches!(handoff.representation(), crate::mir::callable_result_representation::VerifiedCallableResultRepresentationV1::ExactI64 | crate::mir::callable_result_representation::VerifiedCallableResultRepresentationV1::ExactBool)
        {
            return Err(freeze("static-publication-representation"));
        }
        if handoff.target().namespace()
            != crate::mir::builder::SameModuleCallableNamespaceV1::StaticBoxMethod
        {
            return Err(freeze("static-publication-target-namespace"));
        }
        if self.source_core_method_calls.contains_key(site)
            || self.source_static_result_publications.contains_key(site)
            || self
                .consumed_source_static_result_publications
                .contains(site)
        {
            return Err(freeze("duplicate-static-publication-site"));
        }
        self.source_static_result_publications
            .insert(site.clone(), handoff);
        Ok(())
    }



    /// Take one staged publication handoff for a provider `new` argument —
    /// the qualified-static call's exact `(caller, site)` row the port
    /// installed before delegating store emission. The handoff must name
    /// the sealed claim's target; consumption is marked so a second take
    /// or a residual staged row both fail closed.
    pub(in crate::mir::builder) fn take_provider_static_result_publication(
        &mut self,
        site: &crate::mir::resolved_semantics::SourceExprSiteV1,
        target: &crate::mir::builder::CanonicalSameModuleCallableKeyV1,
    ) -> Result<VerifiedStaticCallResultPublicationHandoffV1, String> {
        let handoff = self
            .source_static_result_publications
            .remove(site)
            .ok_or_else(|| freeze("provider-static-publication-missing"))?;
        if handoff.site() != site || handoff.target() != target {
            return Err(freeze("provider-static-publication-target"));
        }
        if !self
            .consumed_source_static_result_publications
            .insert(site.clone())
        {
            return Err(freeze("duplicate-static-publication-consumption"));
        }
        Ok(handoff)
    }

    pub(in crate::mir::builder) fn has_pending_source_static_result_publications(&self) -> bool {
        !self.source_static_result_publications.is_empty()
    }

    pub(in crate::mir::builder) fn explicit_extern_symbol(
        &self,
        site: &SourceNodeSiteV1,
    ) -> Option<&str> {
        self.explicit_extern_calls.get(site).map(Box::as_ref)
    }

    pub(in crate::mir::builder) fn take_brand_constructor(
        &mut self,
        site: &SourceNodeSiteV1,
    ) -> Result<
        Option<crate::mir::builder::brand_constructor_lowering_projection::ProjectedBrandConstructorV1>,
        crate::mir::builder::brand_constructor_lowering_projection::BrandConstructorProjectionErrorV1,
    >{
        let row = match self.brand_constructors.disposition(site)? {
            crate::mir::builder::brand_constructor_lowering_projection::BrandConstructorDispositionRefV1::NonBrand => return Ok(None),
            crate::mir::builder::brand_constructor_lowering_projection::BrandConstructorDispositionRefV1::Constructor(row) => row.clone(),
        };
        if !self.consumed_brand_constructors.insert(site.clone()) {
            return Err(crate::mir::builder::brand_constructor_lowering_projection::BrandConstructorProjectionErrorV1::DuplicateConstructorSite(site.clone()));
        }
        Ok(Some(row))
    }
}
