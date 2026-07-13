//! Canonical resolved-function seal verifier.

use std::collections::{BTreeMap, BTreeSet};

use hakorune_mir_core::BindingId;

use super::ids::{BindingRefV1, FunctionOwnerIdV1, RegionId, ScopeId};
use super::normalized::{NormalizedBindingKeyV1, NormalizedRegionKeyV1, NormalizedScopeKeyV1};
use super::product::ResolvedFunctionDataV1;
use super::records::{
    BindingOriginV1, RegionKindV1, RegionOriginV1, ResolvedAssignmentTargetV1,
    ResolvedControlExitV1, ScopeKindV1, ScopeOriginV1,
};
use super::source_site::{SourceBindingSiteV1, SourceNodeSiteV1, SourceStmtSiteV1};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolvedFunctionVerificationErrorV1 {
    ForeignScopeId(ScopeId),
    ForeignRegionId(RegionId),
    MissingFunctionScope(ScopeId),
    MissingFunctionRegion(RegionId),
    InvalidFunctionScope,
    InvalidFunctionRegion,
    MissingScopeParent(ScopeId),
    ScopeParentCycle(ScopeId),
    MissingRegionParent(RegionId),
    RegionParentCycle(RegionId),
    MissingScopeOwnerRegion(ScopeId),
    MissingRegionLexicalScope(RegionId),
    ScopeRegionMismatch(ScopeId),
    MissingBindingOwnerScope(BindingId),
    ForeignBindingRef(BindingRefV1),
    DanglingBindingRef(BindingRefV1),
    DuplicateScopeBinding(BindingRefV1),
    BindingScopeMismatch(BindingRefV1),
    UnaccountedBinding(BindingId),
    DuplicateBindingOrigin,
    DuplicateScopeOrigin,
    DuplicateRegionOrigin,
    DeclarationBindingMismatch(SourceBindingSiteV1),
    MissingDeclarationIndex(BindingId),
    BindingKindOriginMismatch(BindingId),
    ScopeKindOriginMismatch(ScopeId),
    RegionKindOriginMismatch(RegionId),
    DanglingVariableUse,
    DanglingAssignmentBinding,
    DanglingControlTarget(SourceStmtSiteV1),
    MissingControlExitRegion(SourceStmtSiteV1),
    ExtraControlExitRegion(SourceStmtSiteV1),
    ControlExitRegionMismatch(SourceStmtSiteV1),
    WrongControlTargetKind(SourceStmtSiteV1),
    NonAncestorControlTarget(SourceStmtSiteV1),
    NonNearestLoopTarget(SourceStmtSiteV1),
    WrongReturnTarget(SourceStmtSiteV1),
}

pub(super) fn verify_resolved_function(
    data: &ResolvedFunctionDataV1,
) -> Result<(), ResolvedFunctionVerificationErrorV1> {
    verify_owner_and_roots(data)?;
    verify_scope_graph(data)?;
    verify_region_graph(data)?;
    verify_scope_region_bijection(data)?;
    verify_binding_inventory(data)?;
    verify_indexes(data)?;
    verify_kind_origin_contracts(data)?;
    verify_normalized_key_uniqueness(data)?;
    verify_control_targets(data)?;
    Ok(())
}

fn verify_kind_origin_contracts(
    data: &ResolvedFunctionDataV1,
) -> Result<(), ResolvedFunctionVerificationErrorV1> {
    for (binding, record) in &data.bindings {
        let valid = match (record.kind(), record.origin()) {
            (
                super::records::BindingKindV1::Receiver,
                BindingOriginV1::Source(SourceBindingSiteV1::Receiver),
            ) => true,
            (
                super::records::BindingKindV1::Parameter { index },
                BindingOriginV1::Source(SourceBindingSiteV1::Parameter { index: origin }),
            ) => index == *origin,
            (
                super::records::BindingKindV1::Local { ordinal },
                BindingOriginV1::Source(SourceBindingSiteV1::Local {
                    ordinal: origin, ..
                }),
            ) => ordinal == *origin,
            (
                super::records::BindingKindV1::Outbox { ordinal },
                BindingOriginV1::Source(SourceBindingSiteV1::Outbox {
                    ordinal: origin, ..
                }),
            ) => ordinal == *origin,
            (
                super::records::BindingKindV1::LoopBinder,
                BindingOriginV1::Source(SourceBindingSiteV1::LoopBinder { .. }),
            ) => true,
            (
                super::records::BindingKindV1::CatchBinder { ordinal },
                BindingOriginV1::Source(SourceBindingSiteV1::CatchBinder {
                    ordinal: origin, ..
                }),
            ) => ordinal == *origin,
            (
                super::records::BindingKindV1::PatternBinder { ordinal },
                BindingOriginV1::Source(SourceBindingSiteV1::PatternBinder {
                    ordinal: origin, ..
                }),
            ) => ordinal == *origin,
            (
                super::records::BindingKindV1::CompilerSynthetic,
                BindingOriginV1::Synthetic { .. },
            ) => true,
            _ => false,
        };
        if !valid {
            return Err(ResolvedFunctionVerificationErrorV1::BindingKindOriginMismatch(*binding));
        }
    }
    for (scope, record) in &data.scopes {
        let valid = matches!(
            (record.kind(), record.origin()),
            (ScopeKindV1::Function, ScopeOriginV1::Function(_))
        ) || (record.kind() != ScopeKindV1::Function
            && matches!(record.origin(), ScopeOriginV1::Source(_)));
        if !valid {
            return Err(ResolvedFunctionVerificationErrorV1::ScopeKindOriginMismatch(*scope));
        }
    }
    for (region, record) in &data.regions {
        let valid = matches!(
            (record.kind(), record.origin()),
            (RegionKindV1::Function, RegionOriginV1::Function(_))
        ) || (record.kind() != RegionKindV1::Function
            && matches!(record.origin(), RegionOriginV1::Source(_)));
        if !valid {
            return Err(ResolvedFunctionVerificationErrorV1::RegionKindOriginMismatch(*region));
        }
    }
    Ok(())
}

fn verify_owner_and_roots(
    data: &ResolvedFunctionDataV1,
) -> Result<(), ResolvedFunctionVerificationErrorV1> {
    for scope in data.scopes.keys().copied() {
        if scope.owner() != data.owner {
            return Err(ResolvedFunctionVerificationErrorV1::ForeignScopeId(scope));
        }
    }
    for region in data.regions.keys().copied() {
        if region.owner() != data.owner {
            return Err(ResolvedFunctionVerificationErrorV1::ForeignRegionId(region));
        }
    }
    let scope = data.scopes.get(&data.function_scope).ok_or(
        ResolvedFunctionVerificationErrorV1::MissingFunctionScope(data.function_scope),
    )?;
    if data.function_scope.owner() != data.owner
        || scope.kind() != ScopeKindV1::Function
        || scope.parent().is_some()
        || scope.origin() != &ScopeOriginV1::Function(data.function_origin)
    {
        return Err(ResolvedFunctionVerificationErrorV1::InvalidFunctionScope);
    }
    let region = data.regions.get(&data.function_region).ok_or(
        ResolvedFunctionVerificationErrorV1::MissingFunctionRegion(data.function_region),
    )?;
    if data.function_region.owner() != data.owner
        || region.kind() != RegionKindV1::Function
        || region.parent().is_some()
        || region.lexical_scope() != Some(data.function_scope)
        || region.origin() != &RegionOriginV1::Function(data.function_origin)
    {
        return Err(ResolvedFunctionVerificationErrorV1::InvalidFunctionRegion);
    }
    Ok(())
}

fn verify_scope_graph(
    data: &ResolvedFunctionDataV1,
) -> Result<(), ResolvedFunctionVerificationErrorV1> {
    for (scope, record) in &data.scopes {
        if let Some(parent) = record.parent() {
            if parent.owner() != data.owner || !data.scopes.contains_key(&parent) {
                return Err(ResolvedFunctionVerificationErrorV1::MissingScopeParent(
                    *scope,
                ));
            }
        } else if *scope != data.function_scope {
            return Err(ResolvedFunctionVerificationErrorV1::MissingScopeParent(
                *scope,
            ));
        }
        let mut seen = BTreeSet::new();
        let mut cursor = Some(*scope);
        while let Some(current) = cursor {
            if !seen.insert(current) {
                return Err(ResolvedFunctionVerificationErrorV1::ScopeParentCycle(
                    *scope,
                ));
            }
            cursor = data.scopes.get(&current).and_then(|entry| entry.parent());
        }
    }
    Ok(())
}

fn verify_region_graph(
    data: &ResolvedFunctionDataV1,
) -> Result<(), ResolvedFunctionVerificationErrorV1> {
    for (region, record) in &data.regions {
        if let Some(parent) = record.parent() {
            if parent.owner() != data.owner || !data.regions.contains_key(&parent) {
                return Err(ResolvedFunctionVerificationErrorV1::MissingRegionParent(
                    *region,
                ));
            }
        } else if *region != data.function_region {
            return Err(ResolvedFunctionVerificationErrorV1::MissingRegionParent(
                *region,
            ));
        }
        let mut seen = BTreeSet::new();
        let mut cursor = Some(*region);
        while let Some(current) = cursor {
            if !seen.insert(current) {
                return Err(ResolvedFunctionVerificationErrorV1::RegionParentCycle(
                    *region,
                ));
            }
            cursor = data.regions.get(&current).and_then(|entry| entry.parent());
        }
    }
    Ok(())
}

fn verify_scope_region_bijection(
    data: &ResolvedFunctionDataV1,
) -> Result<(), ResolvedFunctionVerificationErrorV1> {
    for (scope, record) in &data.scopes {
        let region = data
            .regions
            .get(&record.owner_region())
            .ok_or(ResolvedFunctionVerificationErrorV1::MissingScopeOwnerRegion(*scope))?;
        if region.lexical_scope() != Some(*scope) {
            return Err(ResolvedFunctionVerificationErrorV1::ScopeRegionMismatch(
                *scope,
            ));
        }
        if let Some(parent_scope) = record.parent() {
            let parent_region = data.scopes[&parent_scope].owner_region();
            if !is_region_ancestor(data, parent_region, record.owner_region()) {
                return Err(ResolvedFunctionVerificationErrorV1::ScopeRegionMismatch(
                    *scope,
                ));
            }
        }
    }
    for (region, record) in &data.regions {
        if let Some(scope) = record.lexical_scope() {
            let scope_record = data
                .scopes
                .get(&scope)
                .ok_or(ResolvedFunctionVerificationErrorV1::MissingRegionLexicalScope(*region))?;
            if scope_record.owner_region() != *region {
                return Err(ResolvedFunctionVerificationErrorV1::ScopeRegionMismatch(
                    scope,
                ));
            }
        }
    }
    Ok(())
}

fn verify_binding_inventory(
    data: &ResolvedFunctionDataV1,
) -> Result<(), ResolvedFunctionVerificationErrorV1> {
    let mut accounted = BTreeSet::new();
    for (binding, record) in &data.bindings {
        if !data.scopes.contains_key(&record.owner_scope()) {
            return Err(ResolvedFunctionVerificationErrorV1::MissingBindingOwnerScope(*binding));
        }
    }
    for (scope, record) in &data.scopes {
        for binding in record.declarations() {
            verify_binding_ref(data, *binding)?;
            if !accounted.insert(binding.binding()) {
                return Err(ResolvedFunctionVerificationErrorV1::DuplicateScopeBinding(
                    *binding,
                ));
            }
            if data.bindings[&binding.binding()].owner_scope() != *scope {
                return Err(ResolvedFunctionVerificationErrorV1::BindingScopeMismatch(
                    *binding,
                ));
            }
        }
    }
    if let Some(binding) = data.bindings.keys().find(|id| !accounted.contains(id)) {
        return Err(ResolvedFunctionVerificationErrorV1::UnaccountedBinding(
            *binding,
        ));
    }
    Ok(())
}

fn verify_indexes(
    data: &ResolvedFunctionDataV1,
) -> Result<(), ResolvedFunctionVerificationErrorV1> {
    let mut source_bindings = BTreeSet::new();
    for (site, binding) in &data.declarations {
        verify_binding_ref(data, *binding)?;
        let record = &data.bindings[&binding.binding()];
        if record.origin() != &BindingOriginV1::Source(site.clone())
            || !source_bindings.insert(binding.binding())
        {
            return Err(
                ResolvedFunctionVerificationErrorV1::DeclarationBindingMismatch(site.clone()),
            );
        }
    }
    for (binding, record) in &data.bindings {
        if matches!(record.origin(), BindingOriginV1::Source(_))
            && !source_bindings.contains(binding)
        {
            return Err(ResolvedFunctionVerificationErrorV1::MissingDeclarationIndex(*binding));
        }
    }
    for binding in data.variable_uses.values().copied() {
        if verify_binding_ref(data, binding).is_err() {
            return Err(ResolvedFunctionVerificationErrorV1::DanglingVariableUse);
        }
    }
    for target in data.assignment_targets.values() {
        if let ResolvedAssignmentTargetV1::BindingRebind(binding) = target {
            if verify_binding_ref(data, *binding).is_err() {
                return Err(ResolvedFunctionVerificationErrorV1::DanglingAssignmentBinding);
            }
        }
    }
    Ok(())
}

fn verify_normalized_key_uniqueness(
    data: &ResolvedFunctionDataV1,
) -> Result<(), ResolvedFunctionVerificationErrorV1> {
    let mut bindings = BTreeSet::new();
    for record in data.bindings.values() {
        if !bindings.insert(NormalizedBindingKeyV1(record.origin().clone())) {
            return Err(ResolvedFunctionVerificationErrorV1::DuplicateBindingOrigin);
        }
    }
    let mut scopes = BTreeSet::new();
    for record in data.scopes.values() {
        let key = NormalizedScopeKeyV1 {
            kind: record.kind(),
            origin: record.origin().clone(),
        };
        if !scopes.insert(key) {
            return Err(ResolvedFunctionVerificationErrorV1::DuplicateScopeOrigin);
        }
    }
    let mut regions = BTreeSet::new();
    for record in data.regions.values() {
        let key = NormalizedRegionKeyV1 {
            kind: record.kind(),
            origin: record.origin().clone(),
        };
        if !regions.insert(key) {
            return Err(ResolvedFunctionVerificationErrorV1::DuplicateRegionOrigin);
        }
    }
    Ok(())
}

fn verify_control_targets(
    data: &ResolvedFunctionDataV1,
) -> Result<(), ResolvedFunctionVerificationErrorV1> {
    for site in data.control_exit_regions.keys() {
        if !data.control_exits.contains_key(site) {
            return Err(ResolvedFunctionVerificationErrorV1::ExtraControlExitRegion(
                site.clone(),
            ));
        }
    }
    for (site, exit) in &data.control_exits {
        let owner_region = *data.control_exit_regions.get(site).ok_or_else(|| {
            ResolvedFunctionVerificationErrorV1::MissingControlExitRegion(site.clone())
        })?;
        if owner_region.owner() != data.owner || !data.regions.contains_key(&owner_region) {
            return Err(
                ResolvedFunctionVerificationErrorV1::MissingControlExitRegion(site.clone()),
            );
        }
        if !is_exact_source_container(data, owner_region, site.node()) {
            return Err(
                ResolvedFunctionVerificationErrorV1::ControlExitRegionMismatch(site.clone()),
            );
        }
        match exit {
            ResolvedControlExitV1::Return { target_function } => {
                if *target_function != data.function_region {
                    return Err(ResolvedFunctionVerificationErrorV1::WrongReturnTarget(
                        site.clone(),
                    ));
                }
            }
            ResolvedControlExitV1::Break { target_loop }
            | ResolvedControlExitV1::Continue { target_loop } => {
                let target = data.regions.get(target_loop).ok_or_else(|| {
                    ResolvedFunctionVerificationErrorV1::DanglingControlTarget(site.clone())
                })?;
                if target_loop.owner() != data.owner || target.kind() != RegionKindV1::Loop {
                    return Err(ResolvedFunctionVerificationErrorV1::WrongControlTargetKind(
                        site.clone(),
                    ));
                }
                if !is_region_ancestor(data, *target_loop, owner_region) {
                    return Err(
                        ResolvedFunctionVerificationErrorV1::NonAncestorControlTarget(site.clone()),
                    );
                }
                if nearest_loop_for_region(data, owner_region) != Some(*target_loop) {
                    return Err(ResolvedFunctionVerificationErrorV1::NonNearestLoopTarget(
                        site.clone(),
                    ));
                }
            }
        }
    }
    Ok(())
}

fn verify_binding_ref(
    data: &ResolvedFunctionDataV1,
    binding: BindingRefV1,
) -> Result<(), ResolvedFunctionVerificationErrorV1> {
    if binding.owner() != data.owner {
        return Err(ResolvedFunctionVerificationErrorV1::ForeignBindingRef(
            binding,
        ));
    }
    if !data.bindings.contains_key(&binding.binding()) {
        return Err(ResolvedFunctionVerificationErrorV1::DanglingBindingRef(
            binding,
        ));
    }
    Ok(())
}

fn nearest_loop_for_region(
    data: &ResolvedFunctionDataV1,
    mut region: RegionId,
) -> Option<RegionId> {
    loop {
        let record = data.regions.get(&region)?;
        if record.kind() == RegionKindV1::Loop {
            return Some(region);
        }
        region = record.parent()?;
    }
}

fn is_region_ancestor(
    data: &ResolvedFunctionDataV1,
    ancestor: RegionId,
    mut descendant: RegionId,
) -> bool {
    loop {
        if descendant == ancestor {
            return true;
        }
        let Some(parent) = data
            .regions
            .get(&descendant)
            .and_then(|record| record.parent())
        else {
            return false;
        };
        descendant = parent;
    }
}

fn is_exact_source_container(
    data: &ResolvedFunctionDataV1,
    owner: RegionId,
    site: &SourceNodeSiteV1,
) -> bool {
    let owner_record = &data.regions[&owner];
    if let RegionOriginV1::Source(origin) = owner_record.origin() {
        if !site.segments().starts_with(origin.segments()) {
            return false;
        }
    }
    !data.regions.iter().any(|(candidate, record)| {
        *candidate != owner
            && is_region_ancestor(data, owner, *candidate)
            && matches!(
                record.origin(),
                RegionOriginV1::Source(origin)
                    if site.segments().starts_with(origin.segments())
            )
    })
}
