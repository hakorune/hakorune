//! Map source/formal predicate selecting the existing homes-aware walk.
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;

pub(super) fn has_map_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    has_map_formal: impl FnOnce() -> bool,
) -> bool {
    input.body_shape().is_some_and(|shape| {
        shape.expressions().iter().any(|row| {
            matches!(
                row,
                crate::mir::resolved_semantics::BodyExpressionShapeV1::MapLiteral { .. }
            )
        })
    }) || has_map_formal()
}
