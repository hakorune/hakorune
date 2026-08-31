//! Backend-neutral locator for source-backed `me.method(...)` products.
//!
//! This module only co-seals already-issued resolver/package products.  A
//! locator row is a private cross-reference to relation/effect/result/signature
//! rows; it is not a target, receiver value, ABI, or lowering recipe.  The
//! selected-C admission gate is deliberately downstream and does not appear
//! in this product.

use std::collections::BTreeSet;

use crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1;
use crate::mir::callable_semantic_batch::VerifiedResolvedCallableSemanticBatchV1;
use crate::mir::resolved_semantics::{
    DeclaredInstanceCallEffectSourceDispositionV1, DeclaredInstanceCallSourceDispositionV1,
    OwnedExprSiteV1, SourceExprSiteV1,
};

use super::physical_signature::{
    PhysicalCallableLaneRoleV1, PhysicalCallableSignatureRowRefV1,
    VerifiedCallablePhysicalSignatureCohortV1,
};
use super::result_contract::VerifiedCallableResultContractCohortV1;
use super::selected_mapping::VerifiedSelectedCallableBatchMapV1;

/// One private cross-reference into the package's already-issued products.
/// No semantic value is copied into this row.
#[derive(Debug)]
pub(super) struct SealedDeclaredInstanceCallLocatorRowV1 {
    call_site: OwnedExprSiteV1,
    caller_batch_slot: u32,
    target_batch_slot: u32,
    relation_row_ordinal: u32,
    effect_row_ordinal: u32,
}

impl SealedDeclaredInstanceCallLocatorRowV1 {
    pub(super) fn call_site(&self) -> &OwnedExprSiteV1 {
        &self.call_site
    }

    pub(super) const fn caller_batch_slot(&self) -> u32 {
        self.caller_batch_slot
    }

    pub(super) const fn target_batch_slot(&self) -> u32 {
        self.target_batch_slot
    }

    pub(super) const fn relation_row_ordinal(&self) -> u32 {
        self.relation_row_ordinal
    }

    pub(super) const fn effect_row_ordinal(&self) -> u32 {
        self.effect_row_ordinal
    }
}

#[derive(Debug)]
pub(super) struct SealedDeclaredInstanceCallLocatorCatalogV1 {
    rows: Box<[SealedDeclaredInstanceCallLocatorRowV1]>,
}

/// Borrow-only view of the package locator.  The view does not own, clone, or
/// reinterpret any semantic product; it only keeps the install/Builder seam
/// callback-scoped until a downstream physical admission is designed.
#[derive(Debug)]
pub(in crate::mir) struct DeclaredInstanceCallLocatorViewV1<'a> {
    disposition: &'a DeclaredInstanceCallPackageLocatorDispositionV1,
}

impl<'a> DeclaredInstanceCallLocatorViewV1<'a> {
    pub(super) fn new(disposition: &'a DeclaredInstanceCallPackageLocatorDispositionV1) -> Self {
        Self { disposition }
    }

    pub(in crate::mir) fn is_no_root(&self) -> bool {
        matches!(
            self.disposition,
            DeclaredInstanceCallPackageLocatorDispositionV1::NoRootDeclaredInstanceCall
        )
    }

    pub(in crate::mir) fn row_count(&self) -> usize {
        match self.disposition {
            DeclaredInstanceCallPackageLocatorDispositionV1::NoRootDeclaredInstanceCall => 0,
            DeclaredInstanceCallPackageLocatorDispositionV1::Published(catalog) => catalog.len(),
        }
    }
}

impl SealedDeclaredInstanceCallLocatorCatalogV1 {
    pub(super) fn rows(&self) -> &[SealedDeclaredInstanceCallLocatorRowV1] {
        &self.rows
    }

    pub(super) const fn len(&self) -> usize {
        self.rows.len()
    }
}

#[derive(Debug)]
pub(super) enum DeclaredInstanceCallPackageLocatorDispositionV1 {
    NoRootDeclaredInstanceCall,
    Published(SealedDeclaredInstanceCallLocatorCatalogV1),
}

#[derive(Debug)]
pub(in crate::mir) enum DeclaredInstanceCallPackageLocatorIssueV1 {
    RelationEffectPresenceMismatch,
    RelationEffectCoverage {
        relation_rows: usize,
        effect_rows: usize,
    },
    CallerDeclarationMissing,
    CallerDeclarationDuplicate,
    CallerModeMismatch,
    DuplicateOwnedCallSite(OwnedExprSiteV1),
    EffectMissing(SourceExprSiteV1),
    EffectDuplicate(SourceExprSiteV1),
    EffectIdentityMismatch(SourceExprSiteV1),
    TargetSelectionMissing,
    TargetSelectionDuplicate,
    TargetResultMissing,
    TargetResultIdentityMismatch,
    TargetResultOwnerMismatch,
    TargetResultRoleMismatch,
    TargetSignatureMissing,
    TargetSignatureIdentityMismatch,
    TargetSignatureOwnerMismatch,
    TargetSignatureModeMismatch,
    TargetSignatureArityMismatch,
    ReceiverLaneMissing,
    ReceiverLaneShapeMismatch,
    LaneCountMismatch,
    LaneIndexMismatch,
    LaneLogicalOrdinalMismatch,
    LaneTextPairMismatch,
    DuplicateEffectRow(usize),
    EffectCoverageMismatch,
}

pub(super) fn issue_declared_instance_call_package_locator_v1(
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &VerifiedSelectedCallableBatchMapV1,
    result_contracts: &VerifiedCallableResultContractCohortV1,
    physical_signature: &VerifiedCallablePhysicalSignatureCohortV1,
) -> Result<
    DeclaredInstanceCallPackageLocatorDispositionV1,
    DeclaredInstanceCallPackageLocatorIssueV1,
> {
    let relation = match batch.declared_instance_call_source() {
        DeclaredInstanceCallSourceDispositionV1::NoRootDeclaredInstanceCall => {
            if !matches!(
                batch.declared_instance_call_effect_source(),
                DeclaredInstanceCallEffectSourceDispositionV1::NoRootDeclaredInstanceCall
            ) {
                return Err(
                    DeclaredInstanceCallPackageLocatorIssueV1::RelationEffectPresenceMismatch,
                );
            }
            return Ok(DeclaredInstanceCallPackageLocatorDispositionV1::NoRootDeclaredInstanceCall);
        }
        DeclaredInstanceCallSourceDispositionV1::Published(relation) => relation,
    };
    let effect = match batch.declared_instance_call_effect_source() {
        DeclaredInstanceCallEffectSourceDispositionV1::Published(effect) => effect,
        DeclaredInstanceCallEffectSourceDispositionV1::NoRootDeclaredInstanceCall => {
            return Err(DeclaredInstanceCallPackageLocatorIssueV1::RelationEffectPresenceMismatch)
        }
    };
    if relation.len() != effect.len() {
        return Err(
            DeclaredInstanceCallPackageLocatorIssueV1::RelationEffectCoverage {
                relation_rows: relation.len(),
                effect_rows: effect.len(),
            },
        );
    }

    let mut used_effect_rows = BTreeSet::new();
    let mut used_sites = BTreeSet::new();
    let mut rows = Vec::with_capacity(relation.len());
    for (relation_row_ordinal, relation_row) in relation.rows().iter().enumerate() {
        let caller_matches = batch
            .declarations()
            .filter(|declaration| {
                declaration.owner() == relation_row.caller_owner()
                    && declaration.same_declaration_identity(relation_row.caller_identity())
            })
            .collect::<Vec<_>>();
        let [caller] = caller_matches.as_slice() else {
            return Err(if caller_matches.is_empty() {
                DeclaredInstanceCallPackageLocatorIssueV1::CallerDeclarationMissing
            } else {
                DeclaredInstanceCallPackageLocatorIssueV1::CallerDeclarationDuplicate
            });
        };
        if caller.mode() != crate::mir::callable_semantic_batch::ResolvedCallableDeclarationModeV1::InstanceBoxMethod {
            return Err(DeclaredInstanceCallPackageLocatorIssueV1::CallerModeMismatch);
        }

        let owned_site = OwnedExprSiteV1::new(
            relation_row.caller_owner(),
            relation_row.call_site().clone(),
        );
        if !used_sites.insert(owned_site.clone()) {
            return Err(
                DeclaredInstanceCallPackageLocatorIssueV1::DuplicateOwnedCallSite(owned_site),
            );
        }

        let matching_effects = effect
            .rows()
            .iter()
            .enumerate()
            .filter(|(_, effect_row)| {
                effect_row.caller_owner() == relation_row.caller_owner()
                    && effect_row.call_site() == relation_row.call_site()
                    && effect_row
                        .target_identity()
                        .same_as(relation_row.target_identity())
                    && effect_row.target_method_identity() == relation_row.target_method_identity()
            })
            .collect::<Vec<_>>();
        let [(effect_row_ordinal, _effect_row)] = matching_effects.as_slice() else {
            return Err(if matching_effects.is_empty() {
                DeclaredInstanceCallPackageLocatorIssueV1::EffectMissing(
                    relation_row.call_site().clone(),
                )
            } else {
                DeclaredInstanceCallPackageLocatorIssueV1::EffectDuplicate(
                    relation_row.call_site().clone(),
                )
            });
        };
        if !used_effect_rows.insert(*effect_row_ordinal) {
            return Err(
                DeclaredInstanceCallPackageLocatorIssueV1::DuplicateEffectRow(*effect_row_ordinal),
            );
        }
        let target_matches = selected
            .keys()
            .filter_map(|key| selected.batch_slot(key))
            .filter(|slot| {
                selected
                    .identity_for_batch_slot(*slot)
                    .is_some_and(|identity| identity.same_as(relation_row.target_identity()))
            })
            .collect::<Vec<_>>();
        let target_batch_slot = match target_matches.as_slice() {
            [] => return Err(DeclaredInstanceCallPackageLocatorIssueV1::TargetSelectionMissing),
            [slot] => *slot,
            _ => return Err(DeclaredInstanceCallPackageLocatorIssueV1::TargetSelectionDuplicate),
        };
        let target_role = selected
            .role_for_batch_slot(target_batch_slot)
            .ok_or(DeclaredInstanceCallPackageLocatorIssueV1::TargetSelectionMissing)?;
        let result_row = result_contracts
            .row(target_batch_slot)
            .ok_or(DeclaredInstanceCallPackageLocatorIssueV1::TargetResultMissing)?;
        if !result_row
            .identity()
            .same_as(relation_row.target_identity())
        {
            return Err(DeclaredInstanceCallPackageLocatorIssueV1::TargetResultIdentityMismatch);
        }
        if result_row.owner() != relation_row.target_owner()
            || result_row.borrow().completion_owner() != relation_row.target_owner()
        {
            return Err(DeclaredInstanceCallPackageLocatorIssueV1::TargetResultOwnerMismatch);
        }
        if result_row.role() != target_role {
            return Err(DeclaredInstanceCallPackageLocatorIssueV1::TargetResultRoleMismatch);
        }

        let signature = physical_signature
            .row(target_batch_slot)
            .ok_or(DeclaredInstanceCallPackageLocatorIssueV1::TargetSignatureMissing)?;
        validate_signature(signature, relation_row, relation_row.target_identity())?;

        rows.push(SealedDeclaredInstanceCallLocatorRowV1 {
            call_site: owned_site,
            caller_batch_slot: caller.batch_slot(),
            target_batch_slot,
            relation_row_ordinal: u32::try_from(relation_row_ordinal)
                .map_err(|_| DeclaredInstanceCallPackageLocatorIssueV1::LaneCountMismatch)?,
            effect_row_ordinal: u32::try_from(*effect_row_ordinal)
                .map_err(|_| DeclaredInstanceCallPackageLocatorIssueV1::LaneCountMismatch)?,
        });
    }
    if used_effect_rows.len() != effect.len() {
        return Err(DeclaredInstanceCallPackageLocatorIssueV1::EffectCoverageMismatch);
    }
    Ok(DeclaredInstanceCallPackageLocatorDispositionV1::Published(
        SealedDeclaredInstanceCallLocatorCatalogV1 {
            rows: rows.into_boxed_slice(),
        },
    ))
}

fn validate_signature(
    signature: PhysicalCallableSignatureRowRefV1<'_>,
    relation_row: &crate::mir::resolved_semantics::VerifiedDeclaredInstanceCallRelationV1,
    target_identity: &crate::parser::CallableDeclarationIdentityV1,
) -> Result<(), DeclaredInstanceCallPackageLocatorIssueV1> {
    if !signature.identity().same_as(target_identity) {
        return Err(DeclaredInstanceCallPackageLocatorIssueV1::TargetSignatureIdentityMismatch);
    }
    if signature.owner() != relation_row.target_owner() {
        return Err(DeclaredInstanceCallPackageLocatorIssueV1::TargetSignatureOwnerMismatch);
    }
    if signature.mode() != CallableParameterDeclarationModeV1::InstanceBoxMethod {
        return Err(DeclaredInstanceCallPackageLocatorIssueV1::TargetSignatureModeMismatch);
    }
    if signature.source_logical_arity() != relation_row.source_arity() {
        return Err(DeclaredInstanceCallPackageLocatorIssueV1::TargetSignatureArityMismatch);
    }
    let lanes = signature.lanes();
    if signature.receiver_lane_count() != 1 {
        return Err(DeclaredInstanceCallPackageLocatorIssueV1::ReceiverLaneMissing);
    }
    if signature.physical_callable_lane_count() as usize != lanes.len()
        || signature.physical_formal_lane_count() + 1 != signature.physical_callable_lane_count()
    {
        return Err(DeclaredInstanceCallPackageLocatorIssueV1::LaneCountMismatch);
    }
    let Some(receiver) = lanes.first() else {
        return Err(DeclaredInstanceCallPackageLocatorIssueV1::ReceiverLaneMissing);
    };
    if receiver.index() != 0
        || receiver.role() != PhysicalCallableLaneRoleV1::InstanceReceiver
        || receiver.logical_ordinal().is_some()
        || receiver.binding().owner() != signature.owner()
    {
        return Err(DeclaredInstanceCallPackageLocatorIssueV1::ReceiverLaneShapeMismatch);
    }
    let mut logical_ordinals = BTreeSet::new();
    for (index, lane) in lanes.iter().enumerate() {
        if lane.index() != u32::try_from(index).unwrap_or(u32::MAX) {
            return Err(DeclaredInstanceCallPackageLocatorIssueV1::LaneIndexMismatch);
        }
        if index == 0 {
            continue;
        }
        let Some(ordinal) = lane.logical_ordinal() else {
            return Err(DeclaredInstanceCallPackageLocatorIssueV1::LaneLogicalOrdinalMismatch);
        };
        if ordinal >= signature.source_logical_arity() {
            return Err(DeclaredInstanceCallPackageLocatorIssueV1::LaneLogicalOrdinalMismatch);
        }
        logical_ordinals.insert(ordinal);
        match lane.role() {
            PhysicalCallableLaneRoleV1::OrdinaryScalar => {}
            PhysicalCallableLaneRoleV1::ExactTextSlot => {
                let Some(next) = lanes.get(index + 1) else {
                    return Err(DeclaredInstanceCallPackageLocatorIssueV1::LaneTextPairMismatch);
                };
                if next.role() != PhysicalCallableLaneRoleV1::ExactTextGeneration
                    || next.logical_ordinal() != Some(ordinal)
                    || next.index() != lane.index() + 1
                {
                    return Err(DeclaredInstanceCallPackageLocatorIssueV1::LaneTextPairMismatch);
                }
            }
            PhysicalCallableLaneRoleV1::ExactTextGeneration => {
                let Some(previous) = index.checked_sub(1).and_then(|i| lanes.get(i)) else {
                    return Err(DeclaredInstanceCallPackageLocatorIssueV1::LaneTextPairMismatch);
                };
                if previous.role() != PhysicalCallableLaneRoleV1::ExactTextSlot
                    || previous.logical_ordinal() != Some(ordinal)
                {
                    return Err(DeclaredInstanceCallPackageLocatorIssueV1::LaneTextPairMismatch);
                }
            }
            PhysicalCallableLaneRoleV1::InstanceReceiver => {
                return Err(DeclaredInstanceCallPackageLocatorIssueV1::ReceiverLaneShapeMismatch)
            }
        }
    }
    let expected_ordinals = (0..signature.source_logical_arity()).collect::<BTreeSet<_>>();
    if logical_ordinals != expected_ordinals {
        return Err(DeclaredInstanceCallPackageLocatorIssueV1::LaneLogicalOrdinalMismatch);
    }
    Ok(())
}
