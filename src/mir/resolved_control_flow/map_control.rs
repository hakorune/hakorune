//! Exact outward control membership for direct-local Map construction.
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::resolved_semantics::{
    BindingRefV1, BodyExpressionShapeV1, OwnedExprSiteV1, RegionId, ScopeId, SourceBindingSiteV1,
    SourcePathSegmentV1,
};

pub(crate) fn map_source_outward(
    input: ResolvedFunctionLoweringInputV1<'_>,
    site: &OwnedExprSiteV1,
    destination: BindingRefV1,
) -> Result<(ScopeId, RegionId), &'static str> {
    let function = input.function();
    if site.owner() != input.owner()
        || destination.owner() != input.owner()
        || !input
            .forest()
            .owner(input.owner())
            .is_some_and(|owner| std::ptr::eq(owner, function))
    {
        return Err("foreign-map-owner");
    }
    let shape = input.body_shape().ok_or("map-shape-missing")?;
    if shape.owner() != input.owner() || !shape.expressions().iter().any(|row|
        matches!(row, BodyExpressionShapeV1::MapLiteral { site: exact, .. } if exact == site.site()))
    { return Err("map-membership-missing"); }
    if !matches!(
        site.site().node().segments(),
        [
            SourcePathSegmentV1::Body(_),
            SourcePathSegmentV1::Initializer(_)
        ]
    ) {
        return Err("map-not-direct-local");
    }
    let mut initializers = function
        .expression_source()
        .initializers()
        .filter(|row| row.initializer_site() == Some(site.site()));
    let relation = initializers.next().ok_or("map-initializer-missing")?;
    if initializers.next().is_some()
        || relation.binding() != destination
        || !matches!(
            relation.declaration_site(),
            SourceBindingSiteV1::Local { .. }
        )
        || function.declaration_binding(relation.declaration_site()) != Some(destination)
    {
        return Err("map-destination-mismatch");
    }
    let scope = function
        .exact_scope_containing(site.site().node())
        .ok_or("map-scope-missing")?;
    let roots = function.lowering_roots();
    let target = roots.function_pair().region();
    if scope != roots.body_pair().scope()
        || target != function.function_region()
        || function
            .region(roots.body_pair().region())
            .and_then(|row| row.parent())
            != Some(target)
    {
        return Err("map-outward-target-mismatch");
    }
    Ok((scope, target))
}
