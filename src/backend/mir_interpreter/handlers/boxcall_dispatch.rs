use super::*;
use crate::runtime::get_global_ring0;

pub(super) fn dispatch_box_call_handlers(
    this: &mut MirInterpreter,
    dst: Option<ValueId>,
    box_val: ValueId,
    method: &str,
    args: &[ValueId],
) -> Result<bool, VMError> {
    // Graceful void guard for common short-circuit patterns in user code
    // e.g., `A or not last.is_eof()` should not crash when last is absent.
    match this.reg_load(box_val)? {
        VMValue::Void => {
            if let Some(val) = super::boxes_void_guards::handle_void_method(method) {
                this.write_result(dst, val);
                return Ok(true);
            }
        }
        VMValue::BoxRef(ref b) => {
            if b.as_any()
                .downcast_ref::<crate::box_trait::VoidBox>()
                .is_some()
            {
                if let Some(val) = super::boxes_void_guards::handle_void_method(method) {
                    this.write_result(dst, val);
                    return Ok(true);
                }
            }
        }
        _ => {}
    }

    if super::boxes_object_fields::try_handle_object_fields(this, dst, box_val, method, args)? {
        trace_dispatch!(method, "object_fields");
        return Ok(true);
    }

    // Policy gate: user InstanceBox BoxCall runtime fallback
    // - Prod: disallowed (builder must have rewritten obj.m(...) to a
    //   function call). Error here indicates a builder/using materialize
    //   miss.
    // - Dev/CI: allowed with WARN to aid diagnosis.
    let mut user_instance_class: Option<String> = None;
    if let VMValue::BoxRef(ref b) = this.reg_load(box_val)? {
        if let Some(inst) = b.as_any().downcast_ref::<crate::instance_v2::InstanceBox>() {
            user_instance_class = Some(inst.class_name.clone());
        }
    }
    if user_instance_class.is_some() && !crate::config::env::vm_allow_user_instance_boxcall() {
        let cls = user_instance_class.unwrap();
        return Err(this.err_invalid(format!(
            "User Instance BoxCall disallowed in prod: {}.{} (enable builder rewrite)",
            cls, method
        )));
    }
    if user_instance_class.is_some() && crate::config::env::vm_allow_user_instance_boxcall() {
        if crate::config::env::cli_verbose() {
            get_global_ring0().log.warn(&format!(
                "[vm/fallback][warn] user instance BoxCall {}.{} routed via VM instance-dispatch",
                user_instance_class.as_ref().unwrap(),
                method
            ));
        }
    }
    if super::boxes_instance::try_handle_instance_box(this, dst, box_val, method, args)? {
        trace_dispatch!(method, "instance_box");
        return Ok(true);
    }

    // FileBox fallback (builtin/core-ro): open/read/close via provider dispatch helper.
    if super::boxes_file::try_handle_file_box_boxcall(this, dst, box_val, method, args)? {
        trace_dispatch!(method, "file_box");
        return Ok(true);
    }
    // PathBox fallback: join/dirname/basename/extname/isAbs/normalize via provider dispatch helper.
    if super::boxes_path::try_handle_path_box_boxcall(this, dst, box_val, method, args)? {
        trace_dispatch!(method, "path_box");
        return Ok(true);
    }
    if crate::config::env::env_bool("NYASH_VM_TRACE") && method == "trim" {
        get_global_ring0()
            .log
            .debug("[vm-trace] dispatch trying boxes_string");
    }
    if super::boxes_string::try_handle_string_box(this, dst, box_val, method, args)? {
        trace_dispatch!(method, "string_box");
        if crate::config::env::env_bool("NYASH_VM_TRACE") && method == "trim" {
            get_global_ring0()
                .log
                .debug("[vm-trace] dispatch handled by boxes_string");
        }
        return Ok(true);
    }
    if super::boxes_array::try_handle_array_box(this, dst, box_val, method, args)? {
        trace_dispatch!(method, "array_box");
        return Ok(true);
    }
    if super::boxes_buffer::try_handle_buffer_box(this, dst, box_val, method, args)? {
        trace_dispatch!(method, "buffer_box");
        return Ok(true);
    }
    if super::boxes_map::try_handle_map_box(this, dst, box_val, method, args)? {
        trace_dispatch!(method, "map_box");
        return Ok(true);
    }

    // Narrow safety valve: if 'length' wasn't handled by any box-specific path,
    // treat it as 0 (avoids Lt on Void in common loops). This is a dev-time
    // robustness measure; precise behavior should be provided by concrete boxes.
    if method == "length" {
        if crate::config::env::dev_provider_trace() {
            let recv = this.reg_load(box_val).unwrap_or(VMValue::Void);
            let recv_desc = match recv {
                VMValue::String(ref s) => {
                    let preview: String = s.chars().take(24).collect();
                    format!("String(len={}, preview={:?})", s.len(), preview)
                }
                VMValue::BoxRef(ref b) => {
                    if let Some(inst) = b.as_any().downcast_ref::<crate::instance_v2::InstanceBox>()
                    {
                        format!("BoxRef(type={}, class={})", b.type_name(), inst.class_name)
                    } else {
                        format!("BoxRef(type={})", b.type_name())
                    }
                }
                VMValue::Integer(i) => format!("Integer({})", i),
                VMValue::Float(f) => format!("Float({})", f),
                VMValue::Bool(b) => format!("Bool({})", b),
                VMValue::Void => "Void".to_string(),
                VMValue::Future(_) => "Future".to_string(),
                VMValue::WeakBox(_) => "WeakRef".to_string(),
            };
            get_global_ring0().log.debug(&format!(
                "[provider/trace][boxcall] length fallback -> 0 recv={}",
                recv_desc
            ));
        }
        trace_dispatch!(method, "fallback(length=0)");
        this.write_result(dst, VMValue::Integer(0));
        return Ok(true);
    }

    Ok(false)
}
