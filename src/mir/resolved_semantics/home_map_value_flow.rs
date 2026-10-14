//! Recursive Map/Array value classification from the original Home walk.
use super::*;

/// Classify each `Element(ordinal)` child of a sealed `[...]` literal:
/// nested array literals recurse, nested map literals issue child flow rows,
/// and leaves classify through `map_value_leaf`. Unsupported expressions and
/// live Home elements stay `MapCandidateNotCovered`; this slice never transfers
/// a Home from an array element.
pub(super) fn observe_array_elements<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    shape: &crate::mir::resolved_semantics::VerifiedResolvedBodyShapeInventoryV1,
    locals: &mut PrefixLocalFlow<'_>,
    array_site: &SourceExprSiteV1,
    element_count: u32,
    used: &mut std::collections::BTreeSet<BindingRefV1>,
    nested_out: &mut Vec<MapHomeObservation>,
    compatible: &mut impl FnMut(&OwnedExprSiteV1, BindingRefV1) -> Result<bool, E>,
) -> Result<Result<Vec<ArrayElementSource>, HomePrefixUnavailableV1>, E> {
    let mut elements = Vec::with_capacity(element_count as usize);
    for element_ordinal in 0..element_count {
        let mut relations = shape.relations().iter().filter(|row| {
            row.parent() == array_site.node()
                && row.role() == &SourcePathSegmentV1::Element(element_ordinal)
        });
        let Some(relation) = relations.next() else {
            return Ok(Err(HomePrefixUnavailableV1::SourceMismatch));
        };
        if relations.next().is_some() {
            return Ok(Err(HomePrefixUnavailableV1::SourceMismatch));
        }
        let element = relation.child();
        let kind = if let Some(nested_count) = array_literal_element_count(input, element) {
            ArrayElementKindV1::NestedArray(
                match observe_array_elements(
                    input,
                    shape,
                    locals,
                    element,
                    nested_count,
                    used,
                    nested_out,
                    compatible,
                )? {
                    Ok(elements) => elements,
                    Err(issue) => return Ok(Err(issue)),
                }
                .into_boxed_slice(),
            )
        } else if let Some(child_keys) = map_literal_keys(input, element) {
            // Array elements may own a nested Map. Keep the child as its own
            // source flow row and retain only its exact identity here; the
            // child itself is not allowed to consume an outer Home through
            // this first read slice, so it is observed with an empty Home
            // prefix and fails closed if it needs one.
            let child_owned = OwnedExprSiteV1::new(input.owner(), element.clone());
            match observe_map(
                input,
                &child_owned,
                MapDestinationV1::ContainedIn {
                    parent: OwnedExprSiteV1::new(input.owner(), array_site.clone()),
                    role: SourcePathSegmentV1::Element(element_ordinal),
                },
                child_keys,
                locals,
                &[],
                used,
                nested_out,
                compatible,
            )? {
                Ok((child_flow, _)) => {
                    nested_out.push(MapHomeObservation::Complete(child_flow));
                    ArrayElementKindV1::NestedMap(child_owned)
                }
                Err(issue) => {
                    nested_out.push(MapHomeObservation::Unavailable { site: child_owned });
                    return Ok(Err(issue));
                }
            }
        } else {
            let Some(value) = map_value_leaf(input, locals, element) else {
                return Ok(Err(HomePrefixUnavailableV1::MapCandidateNotCovered(
                    element.clone(),
                )));
            };
            ArrayElementKindV1::Leaf(value)
        };
        elements.push(ArrayElementSource {
            site: element.clone(),
            kind,
        });
    }
    Ok(Ok(elements))
}

/// Leaf entry/element source classification shared by map entry values and
/// array elements. `None` keeps the caller's fail-closed boundary.
pub(super) fn map_value_leaf(
    input: ResolvedFunctionLoweringInputV1<'_>,
    locals: &PrefixLocalFlow<'_>,
    site: &SourceExprSiteV1,
) -> Option<MapValueSource> {
    match locals.observe(site) {
        Some(OrdinaryObservation::Integer(value)) => Some(MapValueSource::Integer(value)),
        Some(OrdinaryObservation::Bool(value)) => Some(MapValueSource::Bool(value)),
        Some(OrdinaryObservation::TrivialLocal(binding, kind)) => {
            Some(MapValueSource::Local { binding, kind })
        }
        Some(OrdinaryObservation::Handle(root)) if locals.is_self_rooted_handle(root) => {
            Some(MapValueSource::BorrowedHandle(root))
        }
        Some(OrdinaryObservation::Handle(root))
            if locals.is_map_local(root) || locals.is_borrowed_map(root) =>
        {
            Some(MapValueSource::MapLocal(root))
        }
        _ => match input.function().expression_source().literal(site) {
            Some(ResolvedLiteralSourceV1::String(text)) => {
                Some(MapValueSource::String(text.clone()))
            }
            _ => None,
        },
    }
}
