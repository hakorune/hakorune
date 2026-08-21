//! Request-local BindingRef-to-ValueId materialization ledger.

use std::collections::{BTreeMap, BTreeSet};

use crate::mir::builder::stmts::variable_stmt::OutboxBindingValueV1;
use crate::mir::resolved_semantics::{BindingRefV1, SourceNodeSiteV1};
use crate::mir::ValueId;

use super::normal_script_direct_static_join_handoff::VerifiedScriptDirectStaticJoinRowV1;
use super::normal_script_direct_static_recipe::VerifiedScriptDirectStaticRecipeV1;
use super::normal_script_direct_static_result_publication_owner::VerifiedScriptDirectStaticResultPublicationOwnerV1;
use super::normal_script_semantic_lowering_input::{
    ScriptDirectStaticClaimInputV1, VerifiedScriptSemanticLoweringInputV1,
};
use super::normal_script_semantic_lowering_projection::VerifiedScriptLoweringProjectionV1;
use super::normal_script_source_continuation::VerifiedScriptSourceContinuationV1;

#[derive(Debug)]
pub(super) struct ScriptSemanticLoweringState {
    projection: VerifiedScriptLoweringProjectionV1,
    continuation: VerifiedScriptSourceContinuationV1,
    direct_static_products: ScriptDirectStaticLoweringProductsV1,
    direct_static_claim_ledger: direct_static_claim_ledger::ScriptDirectStaticClaimLedgerV1,
    variable_values: BTreeMap<BindingRefV1, ValueId>,
    materialized_outboxes: BTreeSet<SourceNodeSiteV1>,
}

#[derive(Debug)]
enum ScriptDirectStaticLoweringProductsV1 {
    CompleteNoDirect,
    Direct {
        publication_owner: VerifiedScriptDirectStaticResultPublicationOwnerV1,
        recipe: VerifiedScriptDirectStaticRecipeV1,
    },
}

impl ScriptSemanticLoweringState {
    pub(super) fn new(input: VerifiedScriptSemanticLoweringInputV1) -> Result<Self, String> {
        let (projection, continuation, direct_static_claim_input) = input.into_parts();
        let (
            direct_static_products,
            direct_static_claim_ledger,
        ) = match direct_static_claim_input {
            ScriptDirectStaticClaimInputV1::CompleteNoDirectStaticClaims(witness) => (
                ScriptDirectStaticLoweringProductsV1::CompleteNoDirect,
                direct_static_claim_ledger::ScriptDirectStaticClaimLedgerV1::complete_no_direct(
                    witness,
                ),
            ),
            ScriptDirectStaticClaimInputV1::DirectStaticClaims(products) => {
                let (bundle, publication_owner, recipe, handoff, proof) =
                    products.into_parts();
                let ledger =
                    direct_static_claim_ledger::ScriptDirectStaticClaimLedgerV1::issue_direct(
                        bundle, handoff, proof,
                    )
                    .map_err(|error| {
                        format!("[freeze:contract][script-direct-static/claim-ledger] {error:?}")
                    })?;
                (
                    ScriptDirectStaticLoweringProductsV1::Direct {
                        publication_owner,
                        recipe,
                    },
                    ledger,
                )
            }
        };
        Ok(Self {
            projection,
            continuation,
            direct_static_products,
            direct_static_claim_ledger,
            variable_values: BTreeMap::new(),
            materialized_outboxes: BTreeSet::new(),
        })
    }

    fn projection(&self) -> &VerifiedScriptLoweringProjectionV1 {
        &self.projection
    }

    pub(super) fn source_continuation(&self) -> &VerifiedScriptSourceContinuationV1 {
        &self.continuation
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

    pub(super) fn take_direct_static_claim(
        &mut self,
        site: &crate::mir::resolved_semantics::SourceExprSiteV1,
    ) -> Result<direct_static_claim_ledger::ScriptDirectStaticClaimTakeV1, String> {
        self.direct_static_claim_ledger
            .take(site)
            .map_err(|error| format!("[freeze:contract][script-direct-static/claim] {error:?}"))
    }

    pub(super) fn validate_direct_static_claim(
        &self,
        site: &crate::mir::resolved_semantics::SourceExprSiteV1,
        validate: impl FnOnce(&VerifiedScriptDirectStaticJoinRowV1) -> Result<(), String>,
    ) -> Result<(), String> {
        let row = self.direct_static_claim_ledger.peek(site).map_err(|error| {
            format!("[freeze:contract][script-direct-static/claim-peek] {error:?}")
        })?;
        let Some(row) = row else {
            return Err(
                "[freeze:contract][script-direct-static/claim-site-not-covered]".to_owned(),
            );
        };
        validate(row)
    }

    pub(super) fn complete_direct_static_claim(
        &mut self,
        claimed: direct_static_claim_ledger::ScriptDirectStaticClaimedRowV1,
    ) -> Result<(), String> {
        self.direct_static_claim_ledger.complete(claimed).map_err(|error| {
            format!("[freeze:contract][script-direct-static/claim-complete] {error:?}")
        })
    }

    pub(super) fn finish_direct_static_claims(&mut self) -> Result<(), String> {
        self.direct_static_claim_ledger.finish().map_err(|error| {
            format!("[freeze:contract][script-direct-static/claim-finish] {error:?}")
        })
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
#[path = "normal_script_direct_static_claim_ledger.rs"]
mod direct_static_claim_ledger;

pub(in crate::mir::builder) use direct_static_claim_ledger::{
    ScriptDirectStaticClaimLedgerIssueV1, ScriptDirectStaticClaimLedgerV1,
    ScriptDirectStaticClaimTakeV1, ScriptDirectStaticClaimedRowV1,
};
