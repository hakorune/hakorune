// Instance creation exports.

// Instance birth by name (packed u64x2 + len) -> handle
// export: nyash.instance.birth_name_u64x2(lo, hi, len) -> i64
#[export_name = "nyash.instance.birth_name_u64x2"]
pub extern "C" fn nyash_instance_birth_name_u64x2_export(lo: i64, hi: i64, len: i64) -> i64 {
    use nyash_rust::runtime::get_global_plugin_host;
    let mut bytes = Vec::with_capacity(len.max(0) as usize);
    let lo_u = lo as u64;
    let hi_u = hi as u64;
    let l = len.max(0) as usize;
    let take = core::cmp::min(16, l);
    for i in 0..take.min(8) {
        bytes.push(((lo_u >> (8 * i)) & 0xff) as u8);
    }
    for i in 0..take.saturating_sub(8) {
        bytes.push(((hi_u >> (8 * i)) & 0xff) as u8);
    }
    // If len > 16, remaining bytes are not represented in (lo,hi); assume names <=16 bytes for now.
    if bytes.len() != l {
        bytes.resize(l, 0);
    }
    let name = String::from_utf8_lossy(&bytes).to_string();
    if let Ok(host_g) = get_global_plugin_host().read() {
        if let Ok(b) = host_g.create_box(&name, &[]) {
            let arc: std::sync::Arc<dyn nyash_rust::box_trait::NyashBox> = std::sync::Arc::from(b);
            let h = nyash_rust::runtime::host_handles::to_handle_arc(arc) as u64;
            return h as i64;
        }
    }
    0
}
