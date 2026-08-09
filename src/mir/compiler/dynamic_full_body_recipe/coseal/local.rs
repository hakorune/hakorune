//! Borrow-only projection of the already co-sealed iteration-local relation.
//!
//! This module issues no Home, lifetime, cleanup, Recipe binding, or physical
//! identity. It only makes the existing V10/ch/I7 relation available without
//! allowing callers to split or reconstruct its source authority.

use crate::mir::loop_recipe_contract::{LoopItemKeyV1, LoopValueKeyV1};
use crate::mir::resolved_semantics::{
    BindingRefV1, ResolvedScopeRegionPairV1, SourceBindingSiteV1, SourceExprSiteV1,
    SourceStmtSiteV1,
};

use super::super::super::dynamic_full_body_source::{
    DynamicFullBodyBindingRoleV1, DynamicFullBodySourceRoleV1, DynamicFullBodySourceSiteV1,
};
use super::super::claims::DynamicFullLoopClaimTargetV2;
use super::super::DynamicFullLoopRetainedSourceV1;
use super::calls::VerifiedDynamicFullLoopCallRelationsV2;
use super::coverage::VerifiedDynamicFullLoopClaimCoverageV2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct DynamicIterationLocalRelationV2 {
    value: LoopValueKeyV1,
    producer: LoopItemKeyV1,
    consumer: LoopItemKeyV1,
}

/// Neutral borrowed view over one iteration-local source/value relation.
///
/// The references keep the source inventory as the sole declaration/read
/// authority. Copied Recipe keys identify the logical producer and consumer;
/// they do not create a Recipe binding or carrier for the local value.
#[derive(Debug, Clone, Copy)]
pub(in crate::mir) struct DynamicIterationLocalValueRefV2<'a> {
    scope_region: ResolvedScopeRegionPairV1,
    declaration: &'a SourceBindingSiteV1,
    declaration_statement: &'a SourceStmtSiteV1,
    binding: BindingRefV1,
    read: &'a SourceExprSiteV1,
    value: LoopValueKeyV1,
    producer: LoopItemKeyV1,
    consumer: LoopItemKeyV1,
}

impl DynamicIterationLocalValueRefV2<'_> {
    pub(in crate::mir) const fn scope_region(&self) -> ResolvedScopeRegionPairV1 {
        self.scope_region
    }

    pub(in crate::mir) const fn declaration(&self) -> &SourceBindingSiteV1 {
        self.declaration
    }

    pub(in crate::mir) const fn declaration_statement(&self) -> &SourceStmtSiteV1 {
        self.declaration_statement
    }

    pub(in crate::mir) const fn binding(&self) -> BindingRefV1 {
        self.binding
    }

    pub(in crate::mir) const fn read(&self) -> &SourceExprSiteV1 {
        self.read
    }

    pub(in crate::mir) const fn value(&self) -> LoopValueKeyV1 {
        self.value
    }

    pub(in crate::mir) const fn producer(&self) -> LoopItemKeyV1 {
        self.producer
    }

    pub(in crate::mir) const fn consumer(&self) -> LoopItemKeyV1 {
        self.consumer
    }
}

impl DynamicIterationLocalRelationV2 {
    pub(super) fn borrow<'a>(
        &self,
        source: &'a DynamicFullLoopRetainedSourceV1,
    ) -> DynamicIterationLocalValueRefV2<'a> {
        let binding = source
            .bindings
            .iter()
            .find(|row| row.role() == DynamicFullBodyBindingRoleV1::IterationLocalCh)
            .expect("co-seal retained exact iteration-local binding role");
        let declaration_statement = statement(source, DynamicFullBodySourceRoleV1::ChLocal)
            .expect("co-seal retained exact iteration-local declaration role");
        let read = expression(source, DynamicFullBodySourceRoleV1::IndexOfArgumentCh)
            .expect("co-seal retained exact iteration-local read role");
        DynamicIterationLocalValueRefV2 {
            scope_region: source.scope_region,
            declaration: binding.declaration(),
            declaration_statement,
            binding: binding.binding(),
            read,
            value: self.value,
            producer: self.producer,
            consumer: self.consumer,
        }
    }
}

pub(super) fn verify_iteration_local_relation_v2(
    source: &DynamicFullLoopRetainedSourceV1,
    coverage: &VerifiedDynamicFullLoopClaimCoverageV2,
    calls: &VerifiedDynamicFullLoopCallRelationsV2,
) -> Option<DynamicIterationLocalRelationV2> {
    let DynamicFullLoopClaimTargetV2::IterationLocal { value } =
        coverage.binding_target(DynamicFullBodyBindingRoleV1::IterationLocalCh)?
    else {
        return None;
    };
    if coverage.source_target(DynamicFullBodySourceRoleV1::ChLocal)?
        != (DynamicFullLoopClaimTargetV2::IterationLocal { value })
        || coverage.source_target(DynamicFullBodySourceRoleV1::IndexOfArgumentCh)?
            != DynamicFullLoopClaimTargetV2::Value(value)
    {
        return None;
    }
    binding(source, DynamicFullBodyBindingRoleV1::IterationLocalCh)?;
    statement(source, DynamicFullBodySourceRoleV1::ChLocal)?;
    expression(source, DynamicFullBodySourceRoleV1::IndexOfArgumentCh)?;
    Some(DynamicIterationLocalRelationV2 {
        value,
        producer: calls.item_for(DynamicFullBodySourceRoleV1::SubstringCall)?,
        consumer: calls.item_for(DynamicFullBodySourceRoleV1::IndexOfCall)?,
    })
}

fn binding(
    source: &DynamicFullLoopRetainedSourceV1,
    role: DynamicFullBodyBindingRoleV1,
) -> Option<&super::super::super::dynamic_full_body_source::DynamicFullBodyBindingRowV1> {
    source.bindings.iter().find(|row| row.role() == role)
}

fn statement(
    source: &DynamicFullLoopRetainedSourceV1,
    role: DynamicFullBodySourceRoleV1,
) -> Option<&SourceStmtSiteV1> {
    source.rows.iter().find_map(|row| {
        (row.role() == role).then(|| match row.site() {
            DynamicFullBodySourceSiteV1::Statement(site) => Some(site),
            DynamicFullBodySourceSiteV1::Expression(_) => None,
        })?
    })
}

fn expression(
    source: &DynamicFullLoopRetainedSourceV1,
    role: DynamicFullBodySourceRoleV1,
) -> Option<&SourceExprSiteV1> {
    source.rows.iter().find_map(|row| {
        (row.role() == role).then(|| match row.site() {
            DynamicFullBodySourceSiteV1::Expression(site) => Some(site),
            DynamicFullBodySourceSiteV1::Statement(_) => None,
        })?
    })
}
