use super::*;
use crate::config::env;
use crate::runtime::get_global_ring0;

pub(super) fn run_boxcall_prelude(
    this: &mut MirInterpreter,
    dst: Option<ValueId>,
    box_val: ValueId,
    method: &str,
    args: &[ValueId],
) -> Result<bool, VMError> {
    if super::string_fastpath::try_handle_fast_string_len(this, dst, box_val, method, args)? {
        return Ok(true);
    }

    // Dev-safe: stringify(Void) → "null" (最小安全弁)
    if method == "stringify" {
        if let VMValue::Void = this.reg_load(box_val)? {
            this.write_string(dst, "null".to_string());
            return Ok(true);
        }
        if let VMValue::BoxRef(b) = this.reg_load(box_val)? {
            if b.as_any()
                .downcast_ref::<crate::box_trait::VoidBox>()
                .is_some()
            {
                this.write_string(dst, "null".to_string());
                return Ok(true);
            }
        }
    }

    // Primitive stringify/toString helpers (compat with std/operators/stringify.hako probes).
    // Keep this narrow and total: it prevents "unknown method stringify on IntegerBox" when
    // code checks `value.stringify != null` and then calls it.
    if args.is_empty() && (method == "stringify" || method == "toString") {
        match this.reg_load(box_val)? {
            VMValue::Integer(i) => {
                this.write_string(dst, i.to_string());
                return Ok(true);
            }
            VMValue::Bool(b) => {
                this.write_string(dst, if b { "true" } else { "false" }.to_string());
                return Ok(true);
            }
            VMValue::Float(f) => {
                this.write_string(dst, f.to_string());
                return Ok(true);
            }
            _ => {}
        }
    }

    // Trace: method call (class inferred from receiver)
    if MirInterpreter::box_trace_enabled() {
        let cls = match this.reg_load(box_val).unwrap_or(VMValue::Void) {
            VMValue::BoxRef(b) => {
                if let Some(inst) = b.as_any().downcast_ref::<crate::instance_v2::InstanceBox>() {
                    inst.class_name.clone()
                } else {
                    b.type_name().to_string()
                }
            }
            VMValue::String(_) => "StringBox".to_string(),
            VMValue::Integer(_) => "IntegerBox".to_string(),
            VMValue::Float(_) => "FloatBox".to_string(),
            VMValue::Bool(_) => "BoolBox".to_string(),
            VMValue::Void => "<Void>".to_string(),
            VMValue::Future(_) => "<Future>".to_string(),
            VMValue::WeakBox(_) => "<WeakRef>".to_string(), // Phase 285A0
        };
        this.box_trace_emit_call(&cls, method, args.len());
    }

    if env::env_bool("NYASH_VM_TRACE") && method == "trim" {
        get_global_ring0()
            .log
            .debug("[vm-trace] handle_box_call: method=trim (pre-dispatch)");
    }
    // Debug: trace length dispatch receiver type before any handler resolution
    if method == "length" && env::env_bool("NYASH_VM_TRACE") {
        let recv = this.reg_load(box_val).unwrap_or(VMValue::Void);
        let type_name = match recv {
            VMValue::BoxRef(b) => b.type_name().to_string(),
            VMValue::Integer(_) => "Integer".to_string(),
            VMValue::Float(_) => "Float".to_string(),
            VMValue::Bool(_) => "Bool".to_string(),
            VMValue::String(_) => "String".to_string(),
            VMValue::Void => "Void".to_string(),
            VMValue::Future(_) => "Future".to_string(),
            VMValue::WeakBox(_) => "WeakRef".to_string(), // Phase 285A0
        };
        get_global_ring0().log.debug(&format!(
            "[vm-trace] length dispatch recv_type={}",
            type_name
        ));
    }

    Ok(false)
}
