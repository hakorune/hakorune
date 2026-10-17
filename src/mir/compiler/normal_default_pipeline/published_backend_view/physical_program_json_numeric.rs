//! Serialize the numeric ABI owner's already-issued coordinate obligation.
//! No declaration, range proof or semantic permission is inferred here.
use super::*;
pub(super) fn range_check(
    input: Option<&PublishedLifecyclePhysicalAbiInputV1<'_>>, function: u32,
    context: Option<CallContext<'_>>,
) -> Value {
    context.and_then(|(.., (block, index))|
        input.and_then(|input| input.exact_numeric_check_at(function, block.0, index)))
        .map(|declared_type| json!({"kind": "dynamic_integer_range", "declared_type": declared_type}))
        .unwrap_or_else(|| json!({"kind": "none"}))
}
