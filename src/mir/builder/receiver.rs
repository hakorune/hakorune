use crate::mir::builder::MirBuilder;
use crate::mir::definitions::call_unified::Callee;

/// ReceiverMaterializationBox – centralizes Method receiver pinning + LocalSSA materialization.
///
/// Contract:
/// - If callee is a Method and has a receiver:
///   - Pin the receiver into a named slot (`__pin$*@recv`) so it participates in PHI/loop merges.
///   - Ensure the receiver has an in-block definition via LocalSSA (Copy in the current block).
/// - Args の LocalSSA は別レイヤ（ssa::local）で扱う。
pub fn finalize_method_receiver(builder: &mut MirBuilder, callee: &mut Callee) {
    if let Callee::Method {
        box_name,
        method,
        receiver: Some(r),
        certainty,
        box_kind,
    } = callee.clone()
    {
        // Pin to a named slot so start_new_block や LoopBuilder が slot 経由で追跡できる
        let r_pinned = builder.pin_to_slot(r, "@recv").unwrap_or(r);

        // Optional dev trace for receiver aliases
        if crate::config::env::builder_trace_recv() {
            let current_fn = builder
                .scope_ctx
                .current_function
                .as_ref()
                .map(|f| f.signature.name.clone())
                .unwrap_or_else(|| "<none>".to_string());
            let bb = builder.current_block;
            let names: Vec<String> = builder
                .variable_ctx
                .variable_map
                .iter()
                .filter(|(_, &vid)| vid == r)
                .map(|(k, _)| k.clone())
                .collect();
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!(
                "[builder/recv-trace] fn={} bb={:?} method={}.{} recv=%{} aliases={:?}",
                current_fn,
                bb,
                box_name.clone(),
                method,
                r.0,
                names
            ));
        }

        // LocalSSA: ensure an in-block definition in the current block
        let r_local = crate::mir::builder::ssa::local::recv(builder, r_pinned);
        *callee = Callee::Method {
            box_name,
            method,
            receiver: Some(r_local),
            certainty,
            box_kind,
        };
    }
}
