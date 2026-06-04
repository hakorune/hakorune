use nyash_rust::runtime::plugin_loader_v2::PluginBoxV2;

#[derive(Clone, Copy)]
pub(crate) enum MissingHandleEncoding {
    RawI64,
    Zero,
}

pub(crate) fn encode_plugin_box_value(buf: &mut Vec<u8>, p: &PluginBoxV2) {
    use nyash_rust::box_trait::{IntegerBox, StringBox};

    if p.box_type == "StringBox" {
        let host = nyash_rust::runtime::get_global_plugin_host();
        if let Ok(hg) = host.read() {
            if let Ok(Some(value)) =
                hg.invoke_instance_method("StringBox", "toUtf8", p.instance_id(), &[])
            {
                if let Some(s) = value.as_any().downcast_ref::<StringBox>() {
                    nyash_rust::runtime::plugin_ffi_common::encode::string(buf, &s.value);
                    return;
                }
            }
        };
    } else if p.box_type == "IntegerBox" {
        let host = nyash_rust::runtime::get_global_plugin_host();
        if let Ok(hg) = host.read() {
            if let Ok(Some(value)) =
                hg.invoke_instance_method("IntegerBox", "get", p.instance_id(), &[])
            {
                if let Some(i) = value.as_any().downcast_ref::<IntegerBox>() {
                    nyash_rust::runtime::plugin_ffi_common::encode::i64(buf, i.value);
                    return;
                }
            }
        };
    }
    nyash_rust::runtime::plugin_ffi_common::encode::plugin_handle(
        buf,
        p.inner.type_id,
        p.instance_id(),
    );
}

pub(crate) fn encode_tagged_value(
    buf: &mut Vec<u8>,
    value: i64,
    tag: i64,
    missing_encoding: MissingHandleEncoding,
) {
    match tag {
        3 => nyash_rust::runtime::plugin_ffi_common::encode::i64(buf, value),
        5 => {
            let bits = value as u64;
            nyash_rust::runtime::plugin_ffi_common::encode::f64(buf, f64::from_bits(bits));
        }
        8 => {
            use nyash_rust::runtime::host_handles;

            let encode_missing =
                |buf: &mut Vec<u8>, value: i64, missing_encoding: MissingHandleEncoding| {
                    match missing_encoding {
                        MissingHandleEncoding::RawI64 => {
                            nyash_rust::runtime::plugin_ffi_common::encode::i64(buf, value)
                        }
                        MissingHandleEncoding::Zero => {
                            nyash_rust::runtime::plugin_ffi_common::encode::i64(buf, 0)
                        }
                    }
                };

            if value <= 0 {
                encode_missing(buf, value, missing_encoding);
                return;
            }

            let Some(obj) = host_handles::get(value as u64) else {
                encode_missing(buf, value, missing_encoding);
                return;
            };

            if let Some(p) = obj
                .as_any()
                .downcast_ref::<nyash_rust::runtime::plugin_loader_v2::PluginBoxV2>()
            {
                encode_plugin_box_value(buf, p);
                return;
            }

            let s = obj.to_string_box().value;
            nyash_rust::runtime::plugin_ffi_common::encode::string(buf, &s);
        }
        _ => nyash_rust::runtime::plugin_ffi_common::encode::i64(buf, value),
    }
}
