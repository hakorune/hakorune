//! FFI bridge for plugin method invocation and TLV encoding/decoding

use crate::bid::{BidError, BidResult};
use crate::box_trait::NyashBox;
use crate::runtime::get_global_ring0;
use crate::runtime::plugin_loader_v2::enabled::PluginLoaderV2;
use std::sync::Arc;

fn dbg_on() -> bool {
    std::env::var("PLUGIN_DEBUG").is_ok()
}

impl PluginLoaderV2 {
    /// Invoke a method on a plugin instance with TLV encoding/decoding
    pub fn invoke_instance_method(
        &self,
        box_type: &str,
        method_name: &str,
        instance_id: u32,
        args: &[Box<dyn NyashBox>],
    ) -> BidResult<Option<Box<dyn NyashBox>>> {
        // Mainline path: resolve route contract from selected library config/spec only.
        // This avoids route drift into legacy resolver fallback paths.
        let contract = super::route_resolver::resolve_method_contract(self, box_type, method_name)?;
        let type_id = contract.type_id;
        let method_id = contract.method_id;
        let lib_name = contract.lib_name;

        // Get plugin handle
        let plugins = self.plugins.read().map_err(|_| BidError::PluginError)?;
        let _plugin = plugins.get(&lib_name).ok_or(BidError::PluginError)?;

        super::compat_ffi_bridge::maybe_probe_c_wrap(box_type, method_name);
        super::compat_ffi_bridge::maybe_probe_c_core(
            box_type,
            method_name,
            args,
            type_id,
            instance_id,
        );

        // Encode TLV args via shared helper (numeric→string→toString)
        let tlv = crate::runtime::plugin_ffi_common::encode_args(args);

        super::compat_ffi_bridge::maybe_trace_call(box_type, method_name);

        super::compat_ffi_bridge::maybe_trace_tlv_shim(box_type, method_name, args.len());

        if dbg_on() {
            get_global_ring0().log.debug(&format!(
                "[PluginLoaderV2] call {}.{}: type_id={} method_id={} instance_id={}",
                box_type, method_name, type_id, method_id, instance_id
            ));
        }

        let route = super::route_resolver::resolve_invoke_route_contract(self, type_id);
        let (code, out_len, out) = super::host_bridge::invoke_alloc_with_route(
            route.invoke_box_fn,
            route.invoke_shim_fn,
            type_id,
            method_id,
            instance_id,
            &tlv,
        );
        if code != 0 {
            if dbg_on() {
                get_global_ring0().log.debug(&format!(
                    "[PluginLoaderV2] ERR: invoke failed {}.{} code={} type_id={} method_id={} instance_id={}",
                    box_type, method_name, code, type_id, method_id, instance_id
                ));
            }
            return Err(BidError::PluginError);
        }

        // Decode TLV (first entry) generically
        decode_tlv_result(self, box_type, &out[..out_len])
    }
}

/// Decode TLV result into a NyashBox
fn decode_tlv_result(
    loader: &PluginLoaderV2,
    box_type: &str,
    data: &[u8],
) -> BidResult<Option<Box<dyn NyashBox>>> {
    if let Some((tag, _sz, payload)) = crate::runtime::plugin_ffi_common::decode::tlv_first(data) {
        let bx: Box<dyn NyashBox> = match tag {
            1 => Box::new(crate::box_trait::BoolBox::new(
                crate::runtime::plugin_ffi_common::decode::bool(payload).unwrap_or(false),
            )),
            2 => Box::new(crate::box_trait::IntegerBox::new(
                crate::runtime::plugin_ffi_common::decode::i32(payload).unwrap_or(0) as i64,
            )),
            3 => {
                // i64 payload
                if payload.len() == 8 {
                    let mut b = [0u8; 8];
                    b.copy_from_slice(payload);
                    Box::new(crate::box_trait::IntegerBox::new(i64::from_le_bytes(b)))
                } else {
                    Box::new(crate::box_trait::IntegerBox::new(0))
                }
            }
            5 => {
                let x = crate::runtime::plugin_ffi_common::decode::f64(payload).unwrap_or(0.0);
                Box::new(crate::boxes::FloatBox::new(x))
            }
            6 | 7 => {
                let s = crate::runtime::plugin_ffi_common::decode::string(payload);
                Box::new(crate::box_trait::StringBox::new(s))
            }
            8 => {
                // Plugin handle (type_id, instance_id) → wrap into PluginBoxV2
                if let Some((ret_type, inst)) =
                    crate::runtime::plugin_ffi_common::decode::plugin_handle(payload)
                {
                    let (ret_box_type, fini_method_id) = if let Some(meta) =
                        loader.metadata_for_type_id(ret_type)
                    {
                            (meta.box_type, meta.fini_method_id)
                        } else {
                            (box_type.to_string(), None)
                        };
                    let route = super::route_resolver::resolve_invoke_route_contract(loader, ret_type);
                    let handle = Arc::new(super::types::PluginHandleInner {
                        type_id: ret_type,
                        invoke_fn: super::super::nyash_plugin_invoke_v2_shim,
                        invoke_box_fn: route.invoke_box_fn,
                        instance_id: inst,
                        fini_method_id,
                        finalized: std::sync::atomic::AtomicBool::new(false),
                    });
                    Box::new(super::types::PluginBoxV2 {
                        box_type: ret_box_type,
                        inner: handle,
                    })
                } else {
                    Box::new(crate::box_trait::VoidBox::new())
                }
            }
            9 => {
                // Host handle (u64) → try to map back to BoxRef, else void
                if let Some(u) = crate::runtime::plugin_ffi_common::decode::u64(payload) {
                    if let Some(arc) = crate::runtime::host_handles::get(u) {
                        return Ok(Some(arc.share_box()));
                    }
                }
                Box::new(crate::box_trait::VoidBox::new())
            }
            _ => Box::new(crate::box_trait::VoidBox::new()),
        };
        return Ok(Some(bx));
    }
    Ok(Some(Box::new(crate::box_trait::VoidBox::new())))
}
