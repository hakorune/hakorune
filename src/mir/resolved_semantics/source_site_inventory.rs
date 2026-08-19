//! Resolver-owned exact source-site membership for one semantic owner.

use std::collections::BTreeSet;

use super::product::ResolvedFunctionDataV1;
use super::records::{RegionKindV1, RegionOriginV1, ResolvedAssignmentTargetV1};
use super::source_site::{
    FunctionOriginV1, ResolvedExitSiteV1, SourceBindingSiteV1, SourceExprSiteV1, SourceNodeSiteV1,
    SourcePathSegmentV1, SourceStmtSiteV1,
};
use super::{FunctionOwnerIdV1, SemanticOwnerSourceKindV1};

/// Construction-only receipt populated by the existing resolver traversal.
#[derive(Debug, Default)]
pub(crate) struct ResolvedSourceSiteInventoryDraftV1 {
    statements: BTreeSet<SourceStmtSiteV1>,
    expressions: BTreeSet<SourceExprSiteV1>,
}

impl ResolvedSourceSiteInventoryDraftV1 {
    pub(crate) fn record_statement(&mut self, site: SourceStmtSiteV1) {
        self.statements.insert(site);
    }

    pub(crate) fn record_expression(&mut self, site: SourceExprSiteV1) {
        self.expressions.insert(site);
    }

    #[cfg(test)]
    pub(crate) fn covering_existing_indexes(data: &ResolvedFunctionDataV1) -> Self {
        collect_index_validation_requirements(data)
    }
}

/// Exact statement/expression membership co-sealed with one resolved owner.
///
/// Paths remain the sole topology authority. This product intentionally has
/// no AST, source names, syntax roles, operators, literals, or parent map.
pub struct VerifiedResolvedSourceSiteInventoryV1 {
    owner: FunctionOwnerIdV1,
    function_origin: FunctionOriginV1,
    source_kind: SemanticOwnerSourceKindV1,
    statements: BTreeSet<SourceStmtSiteV1>,
    expressions: BTreeSet<SourceExprSiteV1>,
}

impl std::fmt::Debug for VerifiedResolvedSourceSiteInventoryV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("VerifiedResolvedSourceSiteInventoryV1")
            .field("owner", &self.owner)
            .field("function_origin", &self.function_origin)
            .field("source_kind", &self.source_kind)
            .finish_non_exhaustive()
    }
}

impl VerifiedResolvedSourceSiteInventoryV1 {
    pub const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub const fn function_origin(&self) -> FunctionOriginV1 {
        self.function_origin
    }

    pub const fn source_kind(&self) -> SemanticOwnerSourceKindV1 {
        self.source_kind
    }

    pub fn contains_statement(&self, site: &SourceStmtSiteV1) -> bool {
        self.statements.contains(site)
    }

    pub fn contains_expression(&self, site: &SourceExprSiteV1) -> bool {
        self.expressions.contains(site)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolvedSourceSiteInventoryVerificationErrorV1 {
    MissingIndexedStatement(SourceStmtSiteV1),
    MissingIndexedExpression(SourceExprSiteV1),
}

pub(crate) fn seal_source_site_inventory_v1(
    draft: ResolvedSourceSiteInventoryDraftV1,
    data: &ResolvedFunctionDataV1,
) -> Result<VerifiedResolvedSourceSiteInventoryV1, ResolvedSourceSiteInventoryVerificationErrorV1> {
    let verified = VerifiedResolvedSourceSiteInventoryV1 {
        owner: data.owner,
        function_origin: data.function_origin,
        source_kind: data.root_profile.source_kind(),
        statements: draft.statements,
        expressions: draft.expressions,
    };
    verify_existing_indexes(&verified, data)?;
    Ok(verified)
}

fn verify_existing_indexes(
    inventory: &VerifiedResolvedSourceSiteInventoryV1,
    data: &ResolvedFunctionDataV1,
) -> Result<(), ResolvedSourceSiteInventoryVerificationErrorV1> {
    let expected = collect_index_validation_requirements(data);
    for site in expected.statements {
        if !inventory.contains_statement(&site) {
            return Err(
                ResolvedSourceSiteInventoryVerificationErrorV1::MissingIndexedStatement(site),
            );
        }
    }
    for site in expected.expressions {
        if !inventory.contains_expression(&site) {
            return Err(
                ResolvedSourceSiteInventoryVerificationErrorV1::MissingIndexedExpression(site),
            );
        }
    }
    Ok(())
}

fn collect_index_validation_requirements(
    data: &ResolvedFunctionDataV1,
) -> ResolvedSourceSiteInventoryDraftV1 {
    let mut draft = ResolvedSourceSiteInventoryDraftV1::default();
    for site in data.declarations.keys() {
        match site {
            SourceBindingSiteV1::Local { statement, .. }
            | SourceBindingSiteV1::Outbox { statement, .. }
            | SourceBindingSiteV1::Nowait { statement }
            | SourceBindingSiteV1::LoopBinder {
                loop_site: statement,
            } => draft.record_statement(statement.clone()),
            _ => {}
        }
    }
    for site in data
        .variable_uses
        .keys()
        .chain(data.direct_call_targets.keys())
        .chain(data.explicit_extern_calls.keys())
    {
        draft.record_expression(site.clone());
    }
    for (site, target) in &data.assignment_targets {
        draft.record_expression(site.clone());
        if let ResolvedAssignmentTargetV1::FieldWrite { receiver }
        | ResolvedAssignmentTargetV1::IndexWrite { receiver } = target
        {
            draft.record_expression(receiver.clone());
        }
    }
    for site in data.resolved_exits.keys() {
        match site {
            ResolvedExitSiteV1::Statement(site) => draft.record_statement(site.clone()),
            ResolvedExitSiteV1::Expression(site) => draft.record_expression(site.clone()),
        }
    }
    for record in data.regions.values() {
        let RegionOriginV1::Source(site) = record.origin() else {
            continue;
        };
        match record.kind() {
            RegionKindV1::If | RegionKindV1::Loop => {
                draft.record_statement(SourceStmtSiteV1::from_node(site.clone()))
            }
            RegionKindV1::BlockExpr => {
                let Some((SourcePathSegmentV1::BlockExprPreludeRoot, parent)) =
                    site.segments().split_last()
                else {
                    continue;
                };
                draft.record_expression(SourceExprSiteV1::from_node(
                    SourceNodeSiteV1::from_segments(parent.to_vec()),
                ));
            }
            _ => {}
        }
    }
    draft
}
