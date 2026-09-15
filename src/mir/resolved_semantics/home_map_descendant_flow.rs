//! Descendant MapLiteral observation for contained map literals.
//!
//! Every sealed `MapLiteral` row under a walked statement's subtree needs
//! exactly one flow row for package-install loop1 — including maps nested
//! inside array elements, call arguments, and container subtrees that the
//! direct LocalBinding/ReturnBoundary arms never reach. Sites matching a
//! more specific verifier keep that destination (EntrySlot for map-entry
//! children, CallArgument for sealed-call argument children); the residual
//! family records `ContainedIn` destination evidence only.
use super::local_flow::PrefixLocalFlow;
use super::map_flow::{self, MapDestinationV1, MapHomeObservation};
use super::*;
use crate::mir::resolved_semantics::{
    BodyExpressionShapeV1, SourceNodeSiteV1, SourcePathSegmentV1,
};

/// Observe every not-yet-issued `MapLiteral` row under `statement`'s subtree
/// with the running program-point state. Rows issued by specific destination
/// arms or by an observed parent's EntrySlot recursion are skipped. Sweep
/// failures produce `Unavailable` rows without touching the caller's
/// `unavailable` — the enclosing statement already owns that record.
pub(super) fn observe_descendant_maps<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    statement: &SourceStmtSiteV1,
    locals: &mut PrefixLocalFlow<'_>,
    homes: &mut Vec<BindingRefV1>,
    maps: &mut Vec<MapHomeObservation>,
    map_compatible: &mut impl FnMut(&OwnedExprSiteV1, BindingRefV1) -> Result<bool, E>,
) -> Result<(), E> {
    let Some(shape) = input.body_shape() else {
        return Ok(());
    };
    let function = input.function();
    let prefix = statement.node().segments();
    let mut used = std::collections::BTreeSet::new();
    for row in shape.expressions() {
        let BodyExpressionShapeV1::MapLiteral { site, keys } = row else {
            continue;
        };
        let segments = site.node().segments();
        if segments.len() <= prefix.len() || !segments.starts_with(prefix) {
            continue;
        }
        if maps
            .iter()
            .any(|observation| observation.site().site() == site)
        {
            continue;
        }
        let Some((role, parent_segments)) = segments.split_last() else {
            continue;
        };
        let parent_site =
            SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(parent_segments.to_vec()));
        let parent = OwnedExprSiteV1::new(input.owner(), parent_site.clone());
        let destination = match role {
            SourcePathSegmentV1::EntryValue(ordinal)
                if matches!(
                    shape.expression_shape(&parent_site),
                    Some(BodyExpressionShapeV1::MapLiteral { .. })
                ) =>
            {
                MapDestinationV1::EntrySlot {
                    parent_map: parent,
                    ordinal: *ordinal,
                }
            }
            SourcePathSegmentV1::Argument(ordinal)
                if function.method_call(&parent_site).is_some()
                    || function.direct_call_observation(&parent_site).is_some() =>
            {
                MapDestinationV1::CallArgument {
                    call: parent,
                    ordinal: *ordinal,
                }
            }
            _ => MapDestinationV1::ContainedIn {
                parent,
                role: role.clone(),
            },
        };
        let owned = OwnedExprSiteV1::new(input.owner(), site.clone());
        let mut nested = Vec::new();
        match map_flow::observe_map(
            input,
            &owned,
            destination,
            keys,
            locals,
            homes,
            &mut used,
            &mut nested,
            map_compatible,
        )? {
            Ok((map, remaining)) => {
                *homes = remaining;
                maps.push(MapHomeObservation::Complete(map));
            }
            Err(_) => maps.push(MapHomeObservation::Unavailable { site: owned }),
        }
        maps.extend(nested);
    }
    Ok(())
}

/// Issue `Unavailable` rows for sealed `MapLiteral` sites the walk never
/// reached (post-terminal statements, nested bodies without coverage). One
/// row per literal keeps the loop1 invariant fail-closed.
pub(super) fn issue_unobserved_descendants(
    input: ResolvedFunctionLoweringInputV1<'_>,
    maps: &mut Vec<MapHomeObservation>,
) {
    let Some(shape) = input.body_shape() else {
        return;
    };
    for row in shape.expressions() {
        let BodyExpressionShapeV1::MapLiteral { site, .. } = row else {
            continue;
        };
        if maps
            .iter()
            .any(|observation| observation.site().site() == site)
        {
            continue;
        }
        maps.push(MapHomeObservation::Unavailable {
            site: OwnedExprSiteV1::new(input.owner(), site.clone()),
        });
    }
}
