//! Request-local BindingRef-to-ValueId materialization ledger.

use std::collections::{BTreeMap, BTreeSet};

use crate::mir::builder::stmts::variable_stmt::OutboxBindingValueV1;
use crate::mir::resolved_semantics::{BindingRefV1, SourceNodeSiteV1};
use crate::mir::ValueId;

use super::normal_script_direct_static_recipe::VerifiedScriptDirectStaticRecipeV1;
use super::normal_script_direct_static_result_bundle::VerifiedScriptDirectStaticResultBundleV1;
use super::normal_script_direct_static_result_publication_owner::VerifiedScriptDirectStaticResultPublicationOwnerV1;
use super::normal_script_semantic_lowering_input::VerifiedScriptSemanticLoweringInputV1;
use super::normal_script_semantic_lowering_projection::VerifiedScriptLoweringProjectionV1;
use super::normal_script_source_continuation::VerifiedScriptSourceContinuationV1;

#[derive(Debug)]
pub(super) struct ScriptSemanticLoweringState {
    projection: VerifiedScriptLoweringProjectionV1,
    continuation: VerifiedScriptSourceContinuationV1,
    direct_static_result_bundle: Option<VerifiedScriptDirectStaticResultBundleV1>,
    direct_static_result_publication_owner:
        Option<VerifiedScriptDirectStaticResultPublicationOwnerV1>,
    direct_static_recipe: Option<VerifiedScriptDirectStaticRecipeV1>,
    variable_values: BTreeMap<BindingRefV1, ValueId>,
    materialized_outboxes: BTreeSet<SourceNodeSiteV1>,
}
impl ScriptSemanticLoweringState {
    pub(super) fn new(input: VerifiedScriptSemanticLoweringInputV1) -> Self {
        let (
            projection,
            continuation,
            direct_static_result_bundle,
            direct_static_result_publication_owner,
            direct_static_recipe,
        ) = input.into_parts();
        Self {
            projection,
            continuation,
            direct_static_result_bundle,
            direct_static_result_publication_owner,
            direct_static_recipe,
            variable_values: BTreeMap::new(),
            materialized_outboxes: BTreeSet::new(),
        }
    }

    fn projection(&self) -> &VerifiedScriptLoweringProjectionV1 {
        &self.projection
    }

    pub(super) fn source_continuation(&self) -> &VerifiedScriptSourceContinuationV1 {
        &self.continuation
    }

    pub(super) fn direct_static_result_bundle(
        &self,
    ) -> Option<&VerifiedScriptDirectStaticResultBundleV1> {
        self.direct_static_result_bundle.as_ref()
    }

    pub(super) fn direct_static_result_publication_owner(
        &self,
    ) -> Option<&VerifiedScriptDirectStaticResultPublicationOwnerV1> {
        self.direct_static_result_publication_owner.as_ref()
    }

    pub(super) fn direct_static_recipe(&self) -> Option<&VerifiedScriptDirectStaticRecipeV1> {
        self.direct_static_recipe.as_ref()
    }

    pub(super) fn lambda_captures(
        &self,
        site: &SourceNodeSiteV1,
    ) -> Option<Result<Vec<(String, ValueId)>, String>> {
        self.projection().lambda_captures_at(site).map(|captures| {
            captures
                .iter()
                .map(|(name, binding)| {
                    self.value(*binding)
                        .map(|value| (name.to_string(), value))
                        .ok_or_else(|| {
                            "[freeze:contract][script-lambda/capture-before-materialization]"
                                .to_owned()
                        })
                })
                .collect()
        })
    }

    pub(super) fn variable_binding(&self, site: &SourceNodeSiteV1) -> Option<BindingRefV1> {
        self.projection().variable_binding_at(site)
    }

    pub(super) fn assignment_binding(&self, site: &SourceNodeSiteV1) -> Option<BindingRefV1> {
        self.projection().assignment_binding_at(site)
    }

    pub(super) fn local_binding(&self, site: &SourceNodeSiteV1) -> Option<BindingRefV1> {
        self.projection().local_binding_at(site)
    }

    pub(super) fn nowait_binding(&self, site: &SourceNodeSiteV1) -> Option<BindingRefV1> {
        self.projection().nowait_binding_at(site)
    }

    pub(super) fn outbox_binding_count(&self, site: &SourceNodeSiteV1) -> Result<usize, String> {
        self.projection()
            .outbox_bindings_at(site)
            .map(|bindings| bindings.len())
            .ok_or_else(|| "[freeze:contract][script-lexical/outbox-binding]".to_owned())
    }

    pub(super) fn value(&self, binding: BindingRefV1) -> Option<ValueId> {
        self.variable_values.get(&binding).copied()
    }

    pub(super) fn record_literal_explicit_field_count(
        &self,
        site: &SourceNodeSiteV1,
    ) -> Option<u32> {
        self.projection()
            .record_literal_explicit_field_count_at(site)
    }

    pub(super) fn enum_variant_demand(
        &self,
        site: &SourceNodeSiteV1,
    ) -> Option<&crate::mir::resolved_semantics::EnumVariantAdmissionV1> {
        self.projection().enum_variant_demand_at(site)
    }

    pub(super) fn has_enum_match_scrutinee_receipt(&self, site: &SourceNodeSiteV1) -> bool {
        self.projection().has_enum_match_scrutinee_receipt_at(site)
    }

    pub(super) fn has_qmark_propagation_receipt(&self, site: &SourceNodeSiteV1) -> bool {
        self.projection().has_qmark_propagation_receipt_at(site)
    }

    pub(super) fn explicit_extern_symbol(&self, site: &SourceNodeSiteV1) -> Option<&str> {
        self.projection().explicit_extern_symbol_at(site)
    }

    pub(super) fn brand_constructor_disposition(
        &self,
        site: &SourceNodeSiteV1,
    ) -> Result<
        super::brand_constructor_lowering_projection::BrandConstructorDispositionRefV1<'_>,
        super::brand_constructor_lowering_projection::BrandConstructorProjectionErrorV1,
    > {
        self.projection().brand_constructor_disposition_at(site)
    }

    pub(super) fn record(&mut self, binding: BindingRefV1, value: ValueId) -> Result<(), String> {
        if self.variable_values.insert(binding, value).is_some() {
            return Err("[freeze:contract][script-lexical/duplicate-value]".to_owned());
        }
        Ok(())
    }

    pub(super) fn rebind(&mut self, binding: BindingRefV1, value: ValueId) -> Result<(), String> {
        let Some(slot) = self.variable_values.get_mut(&binding) else {
            return Err(
                "[freeze:contract][script-lexical/rebind-before-materialization]".to_owned(),
            );
        };
        *slot = value;
        Ok(())
    }

    pub(super) fn record_outbox_receipt(
        &mut self,
        site: &SourceNodeSiteV1,
        emitted: &[OutboxBindingValueV1],
    ) -> Result<(), String> {
        if self.materialized_outboxes.contains(site) {
            return Err("[freeze:contract][script-lexical/outbox-duplicate]".to_owned());
        }
        let materialized = {
            let bindings = self
                .projection()
                .outbox_bindings_at(site)
                .ok_or_else(|| "[freeze:contract][script-lexical/outbox-binding]".to_owned())?;
            if bindings.len() != emitted.len() {
                return Err("[freeze:contract][script-lexical/outbox-cardinality]".to_owned());
            }
            for (ordinal, (binding, row)) in bindings.iter().zip(emitted).enumerate() {
                if row.ordinal() != ordinal as u32 || self.variable_values.contains_key(binding) {
                    return Err("[freeze:contract][script-lexical/outbox-receipt]".to_owned());
                }
            }
            bindings
                .iter()
                .zip(emitted)
                .map(|(binding, row)| (*binding, row.value()))
                .collect::<Vec<_>>()
        };
        for (binding, value) in materialized {
            self.variable_values.insert(binding, value);
        }
        self.materialized_outboxes.insert(site.clone());
        Ok(())
    }
}

#[path = "normal_script_binding_materialization.rs"]
mod binding_materialization;
