//! Immutable lowering projection co-sealed with one Script semantic source.
//!
//! This is the sole bridge from verified Script facts and typed receipts to a
//! request-local `BindingRefV1 -> ValueId` ledger. It owns no `ValueId`, does
//! not inspect the AST, and never re-runs source admission.

use std::collections::{BTreeMap, BTreeSet};

use super::normal_script_boundary_receipt_pack::ScriptBoundaryReceiptPackV1;
use super::normal_script_operational_demand_receipt_pack::ScriptOperationalDemandReceiptPackV1;
use super::normal_script_semantic_lowering_state::ScriptSemanticLoweringState;
use super::normal_script_semantic_source_core::ScriptSemanticSourceCoreV1;
use crate::mir::resolved_semantics::{
    BindingRefV1, EnumVariantAdmissionV1, ResolvedAssignmentTargetV1, ResolvedLexicalRefV1,
    SourceBindingSiteV1, SourceExprSiteV1, SourceNodeSiteV1,
};

/// Source-only lowering facts derived exactly once while the semantic product
/// is sealed. Physical `ValueId` materialization remains request-local.
#[derive(Debug)]
pub(super) struct VerifiedScriptLoweringProjectionV1 {
    locals: Box<[(SourceNodeSiteV1, BindingRefV1)]>,
    nowaits: Box<[(SourceNodeSiteV1, BindingRefV1)]>,
    outboxes: Box<[(SourceNodeSiteV1, Box<[BindingRefV1]>)]>,
    variables: Box<[(SourceExprSiteV1, BindingRefV1)]>,
    assignments: Box<[(SourceExprSiteV1, BindingRefV1)]>,
    lambda_captures: Box<[(SourceNodeSiteV1, Box<[(Box<str>, BindingRefV1)]>)]>,
    record_literal_demands: Box<[(SourceNodeSiteV1, u32)]>,
    enum_variant_demands: Box<[(SourceNodeSiteV1, EnumVariantAdmissionV1)]>,
    enum_match_scrutinee_receipts: Box<[SourceNodeSiteV1]>,
    qmark_propagation_receipts: Box<[SourceNodeSiteV1]>,
}

impl VerifiedScriptLoweringProjectionV1 {
    pub(super) fn seal(
        core: &ScriptSemanticSourceCoreV1<'_>,
        boundaries: &ScriptBoundaryReceiptPackV1,
        demands: &ScriptOperationalDemandReceiptPackV1,
    ) -> Result<Self, String> {
        let [root] = core.forest().roots() else {
            return Err(freeze("root-cardinality"));
        };
        let owner = core
            .forest()
            .semantic_owner(*root)
            .ok_or_else(|| freeze("root-owner"))?;
        let owner_id = *root;

        let mut locals = BTreeMap::new();
        let mut nowaits = BTreeMap::new();
        for site in owner.declaration_sites() {
            let binding = owner
                .declaration_binding(site)
                .ok_or_else(|| freeze("missing-declaration-binding"))?;
            if binding.owner() != owner_id {
                return Err(freeze("foreign-declaration-binding"));
            }
            let destination = match site {
                SourceBindingSiteV1::Local { statement, .. } => Some((&mut locals, statement)),
                SourceBindingSiteV1::Nowait { statement } => Some((&mut nowaits, statement)),
                _ => None,
            };
            if let Some((facts, statement)) = destination {
                if facts.insert(statement.node().clone(), binding).is_some() {
                    return Err(freeze("duplicate-declaration-site"));
                }
            }
        }

        let mut variables = BTreeMap::new();
        for (site, reference) in owner.variable_refs() {
            let ResolvedLexicalRefV1::Local(binding) = reference else {
                continue;
            };
            if binding.owner() != owner_id {
                return Err(freeze("foreign-variable-binding"));
            }
            if variables.insert(site.clone(), *binding).is_some() {
                return Err(freeze("duplicate-variable-site"));
            }
        }

        let mut assignments = BTreeMap::new();
        for (site, target) in owner.assignment_targets() {
            let ResolvedAssignmentTargetV1::BindingRebind(binding) = target else {
                continue;
            };
            if binding.owner() != owner_id {
                return Err(freeze("foreign-assignment-binding"));
            }
            if assignments.insert(site.clone(), *binding).is_some() {
                return Err(freeze("duplicate-assignment-site"));
            }
        }

        let mut lambda_captures = BTreeMap::new();
        for (child, _) in core.forest().semantic_owners() {
            let Some(parent) = core.forest().parent(child) else {
                continue;
            };
            let captures = core
                .forest()
                .ordered_capture_demands(child)
                .iter()
                .map(|demand| {
                    let binding = demand.source_binding();
                    let name = core
                        .forest()
                        .semantic_owner(binding.owner())
                        .and_then(|owner| owner.binding(binding))
                        .ok_or_else(|| freeze("capture-binding"))?
                        .diagnostic_name()
                        .into();
                    Ok((name, binding))
                })
                .collect::<Result<Vec<_>, String>>()?
                .into_boxed_slice();
            if lambda_captures
                .insert(parent.definition_site().site().node().clone(), captures)
                .is_some()
            {
                return Err(freeze("duplicate-lambda-site"));
            }
        }

        let mut outboxes = BTreeMap::new();
        for receipt in boundaries.outbox_materializations() {
            if outboxes
                .insert(receipt.site.node().clone(), receipt.bindings.clone())
                .is_some()
            {
                return Err(freeze("duplicate-outbox-site"));
            }
        }

        let mut record_literal_demands = BTreeMap::new();
        for receipt in demands.record_literal_demands() {
            if record_literal_demands
                .insert(receipt.site.node().clone(), receipt.explicit_field_count)
                .is_some()
            {
                return Err(freeze("duplicate-record-demand"));
            }
        }

        let mut enum_variant_demands = BTreeMap::new();
        for receipt in demands.enum_variant_demands() {
            if enum_variant_demands
                .insert(receipt.site.node().clone(), receipt.admission.clone())
                .is_some()
            {
                return Err(freeze("duplicate-enum-variant-demand"));
            }
        }

        let mut enum_match_scrutinee_receipts = BTreeSet::new();
        for receipt in demands.enum_match_demands() {
            if !enum_match_scrutinee_receipts.insert(receipt.site.node().clone()) {
                return Err(freeze("duplicate-enum-match-receipt"));
            }
        }

        let mut qmark_propagation_receipts = BTreeSet::new();
        for receipt in demands.qmark_propagations() {
            if !qmark_propagation_receipts.insert(receipt.site.node().clone()) {
                return Err(freeze("duplicate-qmark-receipt"));
            }
        }

        Ok(Self {
            locals: locals.into_iter().collect(),
            nowaits: nowaits.into_iter().collect(),
            outboxes: outboxes.into_iter().collect(),
            variables: variables.into_iter().collect(),
            assignments: assignments.into_iter().collect(),
            lambda_captures: lambda_captures.into_iter().collect(),
            record_literal_demands: record_literal_demands.into_iter().collect(),
            enum_variant_demands: enum_variant_demands.into_iter().collect(),
            enum_match_scrutinee_receipts: enum_match_scrutinee_receipts.into_iter().collect(),
            qmark_propagation_receipts: qmark_propagation_receipts.into_iter().collect(),
        })
    }

    pub(super) fn local_binding_at(&self, site: &SourceNodeSiteV1) -> Option<BindingRefV1> {
        self.locals
            .iter()
            .find_map(|(candidate, binding)| (candidate == site).then_some(*binding))
    }

    pub(super) fn variable_binding_at(&self, site: &SourceNodeSiteV1) -> Option<BindingRefV1> {
        self.variables
            .iter()
            .find_map(|(candidate, binding)| (candidate.node() == site).then_some(*binding))
    }

    /// Creates fresh physical state for one lowering execution without reading
    /// semantic forest data or reconstructing source paths.
    pub(super) fn lowering_state(&self) -> Result<ScriptSemanticLoweringState, String> {
        let mut state = ScriptSemanticLoweringState::from_facts(
            self.locals.iter().cloned(),
            self.nowaits.iter().cloned(),
            self.outboxes.iter().cloned(),
            self.variables.iter().cloned(),
            self.assignments.iter().cloned(),
        );
        state.install_lambda_captures(self.lambda_captures.iter().cloned())?;
        state.install_record_literal_demands(self.record_literal_demands.iter().cloned())?;
        state.install_enum_variant_demands(self.enum_variant_demands.iter().cloned())?;
        state.install_enum_match_scrutinee_receipts(
            self.enum_match_scrutinee_receipts.iter().cloned(),
        )?;
        state.install_qmark_propagation_receipts(
            self.qmark_propagation_receipts.iter().cloned(),
        )?;
        Ok(state)
    }
}

fn freeze(detail: &str) -> String {
    format!("[freeze:contract][script-lowering-projection/{detail}]")
}
