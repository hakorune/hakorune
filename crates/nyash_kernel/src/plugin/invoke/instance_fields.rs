use nyash_rust::instance_v2::InstanceBox;

#[inline]
fn instance_field_debug_enabled() -> bool {
    nyash_rust::config::env::debug_plugin() || nyash_rust::config::env::cli_verbose_enabled()
}

#[inline]
fn instance_field_debug_log(message: String) {
    if instance_field_debug_enabled() {
        eprintln!("{}", message);
    }
}

fn decode_handle_to_string(handle: i64) -> Result<String, String> {
    if handle <= 0 {
        return Err(format!("Invalid handle: {}", handle));
    }
    let obj = nyash_rust::runtime::host_handles::get(handle as u64)
        .ok_or_else(|| format!("Handle {} not found", handle))?;
    let sb = obj
        .as_any()
        .downcast_ref::<nyash_rust::box_trait::StringBox>()
        .ok_or_else(|| format!("Handle {} is not a StringBox", handle))?;
    Ok(sb.value.clone())
}

fn decode_handle_to_nyash_value(handle: i64) -> Result<nyash_rust::value::NyashValue, String> {
    use nyash_rust::box_trait::{BoolBox, IntegerBox, StringBox};
    use nyash_rust::value::NyashValue;

    if handle <= 0 {
        return Err(format!("Invalid handle: {}", handle));
    }

    let obj = nyash_rust::runtime::host_handles::get(handle as u64)
        .ok_or_else(|| format!("Handle {} not found", handle))?;
    if let Some(ib) = obj.as_any().downcast_ref::<IntegerBox>() {
        return Ok(NyashValue::Integer(ib.value));
    }
    if let Some(sb) = obj.as_any().downcast_ref::<StringBox>() {
        return Ok(NyashValue::String(sb.value.clone()));
    }
    if let Some(bb) = obj.as_any().downcast_ref::<BoolBox>() {
        return Ok(NyashValue::Bool(bb.value));
    }

    Err(format!(
        "Unsupported Box type for handle {}: supports Integer/String/Bool only",
        handle
    ))
}

pub(super) fn handle_instance_get_field(inst: &InstanceBox, field_handle: i64) -> i64 {
    use nyash_rust::box_trait::{BoolBox, IntegerBox, StringBox};
    use nyash_rust::value::NyashValue;
    use std::sync::Arc;

    let field_name = match decode_handle_to_string(field_handle) {
        Ok(s) => s,
        Err(e) => {
            instance_field_debug_log(format!(
                "[plugin/invoke/instance:get_field] decode failed handle={} err={}",
                field_handle, e
            ));
            return 0;
        }
    };
    let nv = match inst.get_field_ng(&field_name) {
        Some(v) => v,
        None => return 0,
    };
    match nv {
        NyashValue::Integer(i) => {
            let arc: Arc<dyn nyash_rust::box_trait::NyashBox> = Arc::new(IntegerBox::new(i));
            nyash_rust::runtime::host_handles::to_handle_arc(arc) as i64
        }
        NyashValue::String(s) => {
            let arc: Arc<dyn nyash_rust::box_trait::NyashBox> = Arc::new(StringBox::new(s));
            nyash_rust::runtime::host_handles::to_handle_arc(arc) as i64
        }
        NyashValue::Bool(b) => {
            let arc: Arc<dyn nyash_rust::box_trait::NyashBox> = Arc::new(BoolBox::new(b));
            nyash_rust::runtime::host_handles::to_handle_arc(arc) as i64
        }
        NyashValue::Null | NyashValue::Void => 0,
        _ => 0,
    }
}

pub(super) fn handle_instance_set_field(
    inst: &InstanceBox,
    field_handle: i64,
    value_handle: i64,
) -> i64 {
    use nyash_rust::value::NyashValue;

    let field_name = match decode_handle_to_string(field_handle) {
        Ok(s) => s,
        Err(e) => {
            instance_field_debug_log(format!(
                "[plugin/invoke/instance:set_field] decode failed handle={} err={}",
                field_handle, e
            ));
            return 0;
        }
    };
    let nv = if value_handle == 0 {
        NyashValue::Null
    } else {
        match decode_handle_to_nyash_value(value_handle) {
            Ok(v) => v,
            Err(_) => NyashValue::Integer(value_handle),
        }
    };
    match inst.set_field_ng(field_name, nv) {
        Ok(_) => 1,
        Err(e) => {
            instance_field_debug_log(format!(
                "[plugin/invoke/instance:set_field] set failed err={}",
                e
            ));
            0
        }
    }
}
