// Plugin-First architecture encoding system
// Simplified encoding that works directly with plugins and handles

/// Mainline argument encoding for Plugin-First architecture.
pub(crate) fn nyrt_encode_arg(buf: &mut Vec<u8>, val: i64) {
    use nyash_rust::runtime::host_handles;
    // Handle direct values and plugin objects; no VM slot fallback.
    if val > 0 {
        if let Some(obj) = host_handles::get(val as u64) {
            if let Some(bufbox) = obj
                .as_any()
                .downcast_ref::<nyash_rust::boxes::buffer::BufferBox>()
            {
                nyash_rust::runtime::plugin_ffi_common::encode::bytes(buf, &bufbox.to_vec());
                return;
            }
            if let Some(p) = obj
                .as_any()
                .downcast_ref::<nyash_rust::runtime::plugin_loader_v2::PluginBoxV2>()
            {
                crate::plugin::encode_plugin_box_value(buf, p);
                return;
            }
        }
    }
    // Fallback: encode as i64 for non-plugin objects
    nyash_rust::runtime::plugin_ffi_common::encode::i64(buf, val);
}
