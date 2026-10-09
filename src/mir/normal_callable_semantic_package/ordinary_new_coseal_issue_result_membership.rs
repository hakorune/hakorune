//! Return-position `new` membership for the ordinary source cohort.
//! The original construction must be the exact Value child of a Return;
//! a nested expression or builtin construction cannot borrow this lane.

use super::*;
use crate::ast::ASTNode;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::resolved_semantics::{SourceNodeSiteV1, SourcePathSegmentV1};

pub(super) fn issue_return_new_membership_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    candidates: &[OrdinaryNewCandidate],
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    instance_constructors: &VerifiedInstanceConstructorSemanticBatchV1,
) -> Result<(Vec<OrdinaryNewSiteResolutionV1>, BTreeSet<OwnedExprSiteV1>), OrdinaryNewCoSealIssueV1>
{
    let mut resolutions = Vec::new();
    let mut sites = BTreeSet::new();
    for construction in input.function().expression_source().constructions() {
        let segments = construction.site().node().segments();
        let Some((SourcePathSegmentV1::Value, parent)) = segments.split_last() else {
            continue;
        };
        let site = OwnedExprSiteV1::new(input.owner(), construction.site().clone());
        if candidates.iter().any(|row| row.site == site) || !sites.insert(site.clone()) {
            if candidates.iter().any(|row| row.site == site) {
                continue;
            }
            return Err(OrdinaryNewCoSealIssueV1::DuplicateSite { site });
        }
        let parent_site =
            SourceStmtSiteV1::from_node(SourceNodeSiteV1::from_segments(parent.to_vec()));
        let Ok(statement) = input.source().exact_stmt(&parent_site) else {
            sites.remove(&site);
            continue;
        };
        if !matches!(statement.node(), ASTNode::Return { value: Some(_), .. }) {
            sites.remove(&site);
            continue;
        }
        if let Some(resolution) = OrdinaryNewCandidate::resolve_site(
            batch,
            instance_constructors,
            site.clone(),
            construction.class().into(),
            construction.arguments().len(),
            !construction.field_initializers().is_empty(),
        )? {
            resolutions.push(resolution);
        } else {
            sites.remove(&site);
        }
    }
    Ok((resolutions, sites))
}
