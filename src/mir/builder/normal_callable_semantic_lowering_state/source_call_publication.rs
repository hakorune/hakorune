//! Existing source call handoff installation and consumption.

use super::*;

impl CallableSemanticLoweringState {
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

    pub(in crate::mir::builder) fn take_source_core_method_call(
        &mut self,
        site: &crate::mir::resolved_semantics::SourceExprSiteV1,
        method: &str,
        arity: u32,
    ) -> Result<Option<ExactSourceMethodCallV1>, String> {
        let Some(row) = self.source_core_method_calls.remove(site) else {
            if self.consumed_source_core_method_calls.contains(site) {
                return Err(freeze("duplicate-core-method-call-consumption"));
            }
            if let Some(handoff) = self.source_static_result_publications.get(site) {
                if handoff.site() != site
                    || handoff.target().namespace()
                        != crate::mir::builder::SameModuleCallableNamespaceV1::StaticBoxMethod
                    || handoff.target().name() != method
                    || handoff.target().arity() != arity
                {
                    return Err(freeze("static-publication-target"));
                }
                let result_type = match handoff.representation() {
                    crate::mir::callable_result_representation::VerifiedCallableResultRepresentationV1::ExactI64 => crate::mir::MirType::Integer,
                    crate::mir::callable_result_representation::VerifiedCallableResultRepresentationV1::ExactBool => crate::mir::MirType::Bool,
                    _ => return Err(freeze("static-publication-representation")),
                };
                if !self
                    .consumed_source_static_result_publications
                    .insert(site.clone())
                {
                    return Err(freeze("duplicate-static-publication-consumption"));
                }
                return Ok(Some(ExactSourceMethodCallV1::static_publication(
                    handoff.target().clone(),
                    result_type,
                    crate::mir::EffectMask::PURE.add(crate::mir::Effect::Io),
                )));
            }
            if self
                .consumed_source_static_result_publications
                .contains(site)
            {
                return Err(freeze("duplicate-static-publication-consumption"));
            }
            return Ok(None);
        };
        if !self.consumed_source_core_method_calls.insert(site.clone()) {
            return Err(freeze("duplicate-core-method-call-consumption"));
        }
        let contract = row.into_contract();
        if contract.call_site() != site
            || contract.result_site() != site
            || contract.arguments().len() != arity as usize
        {
            return Err(freeze("core-method-call-shape"));
        }
        if contract.named_array_requirement().is_some() {
            return Err(freeze("named-array-artifact-retention-required"));
        }
        let target = contract.target();
        let generated = target.row().row();
        if target.row().arity() != arity
            || (generated.canonical != method && !generated.aliases.contains(&method))
            || generated.effect != crate::mir::core_method_result_kind::CoreMethodEffectV1::PureRead
        {
            return Err(freeze("core-method-call-target"));
        }
        let receiver = match contract.receiver() {
            crate::mir::resolved_semantics::ResolvedMethodCallReceiverSourceV1::Lexical(
                crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(_),
            ) => self.read_variable(contract.receiver_site().node())?,
            _ => return Err(freeze("core-method-call-receiver")),
        };
        let result_type = match target.result() {
            crate::mir::resolved_semantics::CoreMethodHomeResultRelationV1::NoValue => {
                return Err(freeze("no-value-source-call-used-as-value"));
            }
            crate::mir::resolved_semantics::CoreMethodHomeResultRelationV1::I64ToCaller => {
                crate::mir::MirType::Integer
            }
            crate::mir::resolved_semantics::CoreMethodHomeResultRelationV1::TextToCaller => {
                crate::mir::MirType::String
            }
        };
        Ok(Some(ExactSourceMethodCallV1::new(
            receiver,
            result_type,
            crate::mir::EffectMask::PURE.add(crate::mir::Effect::Io),
        )))
    }

    pub(in crate::mir::builder) fn take_source_static_result_publication(
        &mut self,
        site: &crate::mir::resolved_semantics::SourceExprSiteV1,
    ) -> Result<Option<VerifiedStaticCallResultPublicationHandoffV1>, String> {
        if !self
            .consumed_source_static_result_publications
            .contains(site)
        {
            return Ok(None);
        }
        let handoff = self
            .source_static_result_publications
            .remove(site)
            .ok_or_else(|| freeze("missing-static-publication-handoff"))?;
        if handoff.site() != site {
            return Err(freeze("static-publication-site-mismatch"));
        }
        Ok(Some(handoff))
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
