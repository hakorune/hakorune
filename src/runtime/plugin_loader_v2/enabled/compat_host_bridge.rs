use super::host_bridge::{invoke_alloc, InvokeFn};

pub(super) fn invoke_alloc_compat(
    invoke_shim: InvokeFn,
    type_id: u32,
    method_id: u32,
    instance_id: u32,
    tlv_args: &[u8],
) -> (i32, usize, Vec<u8>) {
    invoke_alloc(invoke_shim, type_id, method_id, instance_id, tlv_args)
}
