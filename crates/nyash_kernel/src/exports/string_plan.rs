use super::string_view::{resolve_string_span_from_handle, TextPlan};

#[inline(always)]
pub(crate) fn concat_const_suffix_plan_from_handle<'a>(
    a_h: i64,
    suffix: &'a str,
) -> TextPlan<'a> {
    if let Some(plan) = TextPlan::from_handle(a_h) {
        return plan.concat_inline(suffix);
    }
    if let Some(span) = resolve_string_span_from_handle(a_h) {
        return TextPlan::from_span(span).concat_inline(suffix);
    }
    let lhs = super::string::to_owned_string_handle_arg(a_h);
    TextPlan::from_owned(lhs).concat_inline(suffix)
}

#[inline(always)]
pub(crate) fn insert_const_mid_plan_from_handle<'a>(
    source_h: i64,
    middle: &'a str,
    split: i64,
) -> TextPlan<'a> {
    if let Some(source_span) = resolve_string_span_from_handle(source_h) {
        let split = split.clamp(0, source_span.span_bytes_len() as i64) as usize;
        return TextPlan::from_span(source_span).insert_inline(middle, split);
    }

    let source = super::string::to_owned_string_handle_arg(source_h);
    let split = split.clamp(0, source.len() as i64) as usize;
    TextPlan::from_owned(source).insert_inline(middle, split)
}
