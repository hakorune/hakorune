//! Verified source receipt for the direct EnumMatch scrutinee route.
//!
//! The shared shadow traversal authorizes the single scrutinee descent only.
//! Arm observation and every enum diagnostic remain with the raw EnumMatch
//! owner.

use crate::ast::ASTNode;
use crate::mir::resolved_semantics::{
    project_source_node_v1, SourceExprSiteV1, SourcePathSegmentV1, SourcePathV1,
    VerifiedResolvedScriptV1,
};

#[derive(Debug)]
pub(super) struct VerifiedScriptEnumMatchDemandV1 {
    pub(super) site: SourceExprSiteV1,
}

pub(super) fn seal_enum_match_demands_v1(
    source: &ASTNode,
    product: &VerifiedResolvedScriptV1,
) -> Result<Box<[VerifiedScriptEnumMatchDemandV1]>, String> {
    product
        .enum_match_demands()
        .map(|site| {
            let projected = project_source_node_v1(source, site.node()).ok_or_else(|| {
                "[mir/script-semantic/enum-match-projection] missing exact EnumMatchExpr".to_owned()
            })?;
            let crate::mir::resolved_semantics::ProjectedSourceNodeV1::Node(
                ASTNode::EnumMatchExpr { .. },
            ) = projected
            else {
                return Err("[mir/script-semantic/enum-match-site] expected EnumMatchExpr".to_owned());
            };
            let scrutinee_site = SourcePathV1::from_node(site.node())
                .child(SourcePathSegmentV1::EnumMatchScrutinee)
                .expr();
            if !matches!(
                project_source_node_v1(source, scrutinee_site.node()),
                Some(crate::mir::resolved_semantics::ProjectedSourceNodeV1::Node(_))
            ) {
                return Err(
                    "[mir/script-semantic/enum-match-scrutinee-site] expected node".to_owned(),
                );
            }
            Ok(VerifiedScriptEnumMatchDemandV1 { site: site.clone() })
        })
        .collect::<Result<Vec<_>, String>>()
        .map(Vec::into_boxed_slice)
}
