use crate::plugin::invoke_core;

#[no_mangle]
pub extern "C" fn nyash_plugin_invoke3_i64(
    type_id: i64,
    method_id: i64,
    argc: i64,
    a0: i64,
    a1: i64,
    a2: i64,
) -> i64 {
    let Some(recv) = invoke_core::resolve_receiver_for_a0(a0) else {
        return 0;
    };
    let Some(buf) = invoke_core::build_two_payload_tlv(argc, a1, a2) else {
        return 0;
    };
    invoke_core::invoke_receiver_to_i64(
        recv.invoke,
        type_id as u32,
        method_id as u32,
        recv.instance_id,
        &buf,
    )
    .unwrap_or(0)
}

#[no_mangle]
pub extern "C" fn nyash_plugin_invoke3_f64(
    type_id: i64,
    method_id: i64,
    argc: i64,
    a0: i64,
    a1: i64,
    a2: i64,
) -> f64 {
    let Some(recv) = invoke_core::resolve_receiver_for_a0(a0) else {
        return 0.0;
    };
    let Some(buf) = invoke_core::build_two_payload_tlv(argc, a1, a2) else {
        return 0.0;
    };
    invoke_core::invoke_receiver_to_f64(
        recv.invoke,
        type_id as u32,
        method_id as u32,
        recv.instance_id,
        &buf,
    )
    .unwrap_or(0.0)
}
