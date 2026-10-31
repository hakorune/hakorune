//! Destination-less `new` membership for the ordinary source cohort.
//! Return and Shared operands retain their exact, distinct source positions.

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
        let return_parent = match segments.split_last() {
            Some((SourcePathSegmentV1::Value, parent)) => Some(parent), _ => None,
        };
        let is_return = return_parent.is_some_and(|parent| {
            let statement = SourceStmtSiteV1::from_node(SourceNodeSiteV1::from_segments(parent.to_vec()));
            input.source().exact_stmt(&statement).is_ok_and(|row| matches!(row.node(), ASTNode::Return { value: Some(_), .. }))
        });
        let is_shared_payload = input.function().expression_source().initializers().any(|initializer| {
            initializer.initializer_site().and_then(|site| input.function().expression_source().unary(site))
                .is_some_and(|unary| unary.operator() == crate::mir::resolved_semantics::ResolvedUnaryOperatorV1::Shared
                    && unary.operand() == construction.site())
        });
        if !is_return && !is_shared_payload { continue; }
        let site = OwnedExprSiteV1::new(input.owner(), construction.site().clone());
        if candidates.iter().any(|row| row.site == site) || !sites.insert(site.clone()) {
            if candidates.iter().any(|row| row.site == site) {
                continue;
            }
            return Err(OrdinaryNewCoSealIssueV1::DuplicateSite { site });
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
