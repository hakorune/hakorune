//! Exact outward control membership for direct-local Map construction.
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::resolved_semantics::{
    BindingRefV1, BodyExpressionShapeV1, OwnedExprSiteV1, RegionId, ScopeId, SourceBindingSiteV1,
    SourcePathSegmentV1, SourceStmtSiteV1,
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

/// Exact outward membership for a nested `%{...}` literal bound to a parent
/// map's `EntryValue(ordinal)` slot: the site is itself a `MapLiteral` row
/// whose node path is `parent_map.node + EntryValue(ordinal)` and whose
/// sealed relation matches exactly.
pub(crate) fn map_entry_outward(
    input: ResolvedFunctionLoweringInputV1<'_>,
    site: &OwnedExprSiteV1,
    parent_map: &OwnedExprSiteV1,
    ordinal: u32,
) -> Result<(ScopeId, RegionId), &'static str> {
    let function = input.function();
    if site.owner() != input.owner()
        || parent_map.owner() != input.owner()
        || !input
            .forest()
            .owner(input.owner())
            .is_some_and(|owner| std::ptr::eq(owner, function))
    {
        return Err("foreign-map-owner");
    }
    let shape = input.body_shape().ok_or("map-shape-missing")?;
    let is_map_literal = |exact| {
        shape.expressions().iter().any(
            |row| matches!(row, BodyExpressionShapeV1::MapLiteral { site, .. } if site == exact),
        )
    };
    if shape.owner() != input.owner()
        || !is_map_literal(site.site())
        || !is_map_literal(parent_map.site())
    {
        return Err("map-membership-missing");
    }
    let mut expected = parent_map.site().node().segments().to_vec();
    expected.push(SourcePathSegmentV1::EntryValue(ordinal));
    if site.site().node().segments() != expected.as_slice() {
        return Err("map-entry-site-mismatch");
    }
    let mut relations = shape.relations().iter().filter(|row| {
        row.parent() == parent_map.site().node()
            && row.role() == &SourcePathSegmentV1::EntryValue(ordinal)
            && row.child() == site.site()
    });
    if relations.next().is_none() || relations.next().is_some() {
        return Err("map-entry-relation-mismatch");
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

/// Exact outward membership for a `%{...}` literal bound to a call's
/// `Argument(ordinal)` slot: the site is itself a `MapLiteral` row whose
/// node path is `call.node + Argument(ordinal)`, whose sealed relation
/// matches exactly, and whose parent call carries a sealed call row
/// (method-call or direct-call observation) linking the same ordinal to
/// this site. The row records destination evidence only; argument
/// transfer semantics stay the consumer's concern.
pub(crate) fn map_argument_outward(
    input: ResolvedFunctionLoweringInputV1<'_>,
    site: &OwnedExprSiteV1,
    call: &OwnedExprSiteV1,
    ordinal: u32,
) -> Result<(ScopeId, RegionId), &'static str> {
    let function = input.function();
    if site.owner() != input.owner()
        || call.owner() != input.owner()
        || !input
            .forest()
            .owner(input.owner())
            .is_some_and(|owner| std::ptr::eq(owner, function))
    {
        return Err("foreign-map-owner");
    }
    let shape = input.body_shape().ok_or("map-shape-missing")?;
    if shape.owner() != input.owner()
        || !shape.expressions().iter().any(|row| {
            matches!(row,
            BodyExpressionShapeV1::MapLiteral { site: exact, .. } if exact == site.site())
        })
    {
        return Err("map-membership-missing");
    }
    let mut expected = call.site().node().segments().to_vec();
    expected.push(SourcePathSegmentV1::Argument(ordinal));
    if site.site().node().segments() != expected.as_slice() {
        return Err("map-argument-site-mismatch");
    }
    let mut relations = shape.relations().iter().filter(|row| {
        row.parent() == call.site().node()
            && row.role() == &SourcePathSegmentV1::Argument(ordinal)
            && row.child() == site.site()
    });
    if relations.next().is_none() || relations.next().is_some() {
        return Err("map-argument-relation-mismatch");
    }
    let linked = function.method_call(call.site()).is_some_and(|row| {
        row.arguments()
            .iter()
            .any(|argument| argument.ordinal() == ordinal && argument.site() == site.site())
    }) || function
        .direct_call_observation(call.site())
        .is_some_and(|row| row.argument_sites().get(ordinal as usize) == Some(site.site()));
    if !linked {
        return Err("map-argument-call-missing");
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

/// Exact outward membership for a `%{...}` literal at an arbitrary sealed
/// child position `parent.node + role`: the site is itself a `MapLiteral`
/// row, the parent is a sealed expression row, the sealed relation row is
/// unique, and the scope/target triple matches. Positions with a more
/// specific destination (EntrySlot/CallArgument) verify through their own
/// verifier instead.
pub(crate) fn map_contained_outward(
    input: ResolvedFunctionLoweringInputV1<'_>,
    site: &OwnedExprSiteV1,
    parent: &OwnedExprSiteV1,
    role: &SourcePathSegmentV1,
) -> Result<(ScopeId, RegionId), &'static str> {
    let function = input.function();
    if site.owner() != input.owner()
        || parent.owner() != input.owner()
        || !input
            .forest()
            .owner(input.owner())
            .is_some_and(|owner| std::ptr::eq(owner, function))
    {
        return Err("foreign-map-owner");
    }
    let shape = input.body_shape().ok_or("map-shape-missing")?;
    if shape.owner() != input.owner()
        || !shape.expressions().iter().any(|row| {
            matches!(row,
            BodyExpressionShapeV1::MapLiteral { site: exact, .. } if exact == site.site())
        })
        || shape.expression_shape(parent.site()).is_none()
    {
        return Err("map-membership-missing");
    }
    let mut expected = parent.site().node().segments().to_vec();
    expected.push(role.clone());
    if site.site().node().segments() != expected.as_slice() {
        return Err("map-contained-site-mismatch");
    }
    let mut relations = shape.relations().iter().filter(|row| {
        row.parent() == parent.site().node() && row.role() == role && row.child() == site.site()
    });
    if relations.next().is_none() || relations.next().is_some() {
        return Err("map-contained-relation-mismatch");
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

/// Exact outward membership for a `%{...}` literal returned through the
/// function boundary: the site must be `[Body(i), Value]` where statement `i`
/// is an explicit `Return` whose sealed exit transfer targets this function.
pub(crate) fn map_return_outward(
    input: ResolvedFunctionLoweringInputV1<'_>,
    site: &OwnedExprSiteV1,
    return_site: &SourceStmtSiteV1,
) -> Result<(ScopeId, RegionId), &'static str> {
    use crate::mir::resolved_semantics::{
        ResolvedControlTransferV1, ResolvedExitOriginV1, ResolvedExitSiteV1,
    };
    let function = input.function();
    if site.owner() != input.owner()
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
    let &[SourcePathSegmentV1::Body(index), SourcePathSegmentV1::Value] =
        site.site().node().segments()
    else {
        return Err("map-not-return-value");
    };
    if return_site.node().segments() != [SourcePathSegmentV1::Body(index)] {
        return Err("map-return-site-mismatch");
    }
    let matched = function.resolved_exits().any(|(exit_site, exit)| {
        matches!(exit_site, ResolvedExitSiteV1::Statement(stmt) if stmt == return_site)
            && exit.origin() == ResolvedExitOriginV1::ExplicitReturn
            && matches!(
                exit.transfer(),
                ResolvedControlTransferV1::Return { target_function }
                    if target_function == function.function_region()
            )
    });
    if !matched {
        return Err("map-return-exit-missing");
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
