//! The one place that decides an empty `new MapBox()` is the builtin Map.
//!
//! The body-shape layer keeps a `new` as a neutral row because it does not see
//! user declarations. The batch issuer is the first stage that holds both the
//! shape rows and the declaration coverage, so the decision is made here, once:
//! a `new MapBox()` with no arguments and no field initializers whose name has
//! no user declaration of any kind (ordinary, `static`, ...) is the same empty
//! Map as `%{}`. A user `box MapBox` keeps its row and goes through the
//! ordinary construction path; a `static box MapBox` keeps its row and is
//! rejected like any other `new` of a static box. The builder does not ask
//! again: it reads the `MapLiteral` row issued here.

use crate::ast::ASTNode;
use crate::mir::resolved_semantics::{
    project_source_node_v1, BodyExpressionShapeV1, ProjectedSourceNodeV1, SourceExprSiteV1,
    VerifiedResolvedBodyShapeInventoryV1,
};
use crate::parser::ParserOrdinaryBoxSourceCoverageV1;

/// Re-issue the rows of `shape` that are the builtin empty Map construction.
/// `owner_root` is the original source of the same owner, so each rewritten
/// site is projected to its source `new` first.
pub(crate) fn seal_builtin_empty_map_constructions_v1(
    shape: VerifiedResolvedBodyShapeInventoryV1,
    owner_root: &ASTNode,
    coverage: &ParserOrdinaryBoxSourceCoverageV1,
) -> Result<VerifiedResolvedBodyShapeInventoryV1, &'static str> {
    shape.with_empty_map_constructions_v1(|site| {
        matches!(
            project_source_node_v1(owner_root, site.node()),
            Some(ProjectedSourceNodeV1::Node(node))
                if is_builtin_empty_map_new_v1(node, coverage)
        )
    })
}

/// `node` is an argument-less `new` of the builtin `MapBox`: no arguments, no
/// field initializers, and `coverage` has no user declaration of that name.
/// The one place that names the builtin Map.
pub(crate) fn is_builtin_empty_map_new_v1(
    node: &ASTNode,
    coverage: &ParserOrdinaryBoxSourceCoverageV1,
) -> bool {
    let ASTNode::New {
        class,
        arguments,
        field_initializers,
        ..
    } = node
    else {
        return false;
    };
    class == "MapBox"
        && arguments.is_empty()
        && field_initializers.is_empty()
        && !coverage.declares_box(class)
        && crate::box_trait::is_builtin_box(class)
}

/// What the issuer decided for the `new` at `site`: `true` when it was issued
/// as the builtin empty Map (a `MapLiteral` row, which a `New` only gets from
/// `seal_builtin_empty_map_constructions_v1`), `false` for a user construction.
/// The builder asks this instead of deciding again. No shape, or no row at the
/// site, means the source answer is missing: a contract violation, never a
/// reason to pick another route.
pub(crate) fn issued_builtin_empty_map_new_v1(
    shape: Option<&VerifiedResolvedBodyShapeInventoryV1>,
    site: &SourceExprSiteV1,
) -> Result<bool, &'static str> {
    match shape.ok_or("map-shape-missing")?.expression_shape(site) {
        Some(BodyExpressionShapeV1::MapLiteral { .. }) => Ok(true),
        Some(_) => Ok(false),
        None => Err("map-shape-row-missing"),
    }
}
