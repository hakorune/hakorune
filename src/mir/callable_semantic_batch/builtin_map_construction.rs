//! The one place that decides an empty `new MapBox()` is the builtin Map.
//!
//! The body-shape layer keeps a `new` as a neutral row because it does not see
//! user declarations. The batch issuer is the first stage that holds both the
//! shape rows and the declaration coverage, so the decision is made here, once:
//! a `new MapBox()` with no arguments and no field initializers whose name has
//! no user declaration is the same empty Map as `%{}`. A user `box MapBox`
//! keeps its row and goes through the ordinary construction path.

use crate::ast::ASTNode;
use crate::mir::resolved_semantics::{
    project_source_node_v1, ProjectedSourceNodeV1, SourceExprSiteV1,
    VerifiedResolvedBodyShapeInventoryV1,
};
use crate::parser::ParserOrdinaryBoxSourceCoverageV1;

pub(crate) fn seal_builtin_empty_map_constructions_v1(
    shape: VerifiedResolvedBodyShapeInventoryV1,
    owner_root: &ASTNode,
    coverage: &ParserOrdinaryBoxSourceCoverageV1,
) -> VerifiedResolvedBodyShapeInventoryV1 {
    shape.with_empty_map_constructions_v1(|site| {
        is_builtin_empty_map_new_v1(owner_root, site, coverage)
    })
}

/// The source node at `site` is an argument-less `new` of the builtin `MapBox`.
pub(super) fn is_builtin_empty_map_new_v1(
    owner_root: &ASTNode,
    site: &SourceExprSiteV1,
    coverage: &ParserOrdinaryBoxSourceCoverageV1,
) -> bool {
    let Some(ProjectedSourceNodeV1::Node(ASTNode::New {
        class,
        arguments,
        field_initializers,
        ..
    })) = project_source_node_v1(owner_root, site.node())
    else {
        return false;
    };
    class == "MapBox"
        && arguments.is_empty()
        && field_initializers.is_empty()
        && coverage.resolves_to_builtin_box(class)
}
