use super::invoke_core::InvokeFn;

pub(super) fn resolve_generic_fallback_route(
    fallback_invoke: InvokeFn,
) -> Option<(String, InvokeFn, Option<u32>)> {
    Some(("PluginBox".to_string(), fallback_invoke, None))
}

#[inline]
pub(super) fn encode_legacy_vm_args_range(
    dst: &mut Vec<u8>,
    start_pos: usize,
    end_pos_inclusive: usize,
) {
    if start_pos > end_pos_inclusive {
        return;
    }
    for pos in start_pos..=end_pos_inclusive {
        crate::encode::nyrt_encode_from_legacy_at(dst, pos);
    }
}
