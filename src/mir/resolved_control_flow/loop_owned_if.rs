//! Exact partition check between one selected Loop and the outer If ledger.

use crate::ast::ASTNode;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::resolved_semantics::{SourceNodeSiteV1, SourcePathSegmentV1, SourceStmtSiteV1};

use super::if_control::VerifiedResolvedFunctionIfControlV1;

/// Issue the named zero-row witness only when the resolved function owns no
/// exact If region at all.
pub(super) fn verify_empty_loop_if_partition_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
) -> Result<VerifiedResolvedFunctionIfControlV1, String> {
    if input.function().if_region_bundle_count() != 0 {
        return Err("[freeze:contract][if_control/loop_profile_not_empty]".to_string());
    }
    Ok(VerifiedResolvedFunctionIfControlV1::empty_verified(
        input.owner(),
    ))
}

/// Close the outer If-control ledger only when every exact If belongs to the
/// selected Loop source. The Loop owner must consume those rows later; this
/// check never treats unrelated If control as empty.
pub(super) fn verify_owned_loop_if_partition_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    loop_site: &SourceNodeSiteV1,
) -> Result<VerifiedResolvedFunctionIfControlV1, String> {
    let loop_statement = SourceStmtSiteV1::from_node(loop_site.clone());
    let located = input
        .source()
        .exact_stmt(&loop_statement)
        .map_err(|error| format!("[freeze:contract][if_control/loop_source] {error}"))?;
    if !matches!(located.node(), ASTNode::Loop { .. }) {
        return Err("[freeze:contract][if_control/owned_source_not_loop]".to_string());
    }

    let prefix = loop_site.segments();
    let all_owned = input.function().if_region_sites().all(|site| {
        let segments = site.node().segments();
        segments.starts_with(prefix)
            && matches!(
                segments.get(prefix.len()),
                Some(SourcePathSegmentV1::LoopBody(_))
            )
    });
    if !all_owned {
        return Err("[freeze:contract][if_control/foreign_outer_if]".to_string());
    }

    Ok(VerifiedResolvedFunctionIfControlV1::empty_verified(
        input.owner(),
    ))
}
