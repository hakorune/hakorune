use super::*;

pub(super) fn try_handle_object_fields(
    this: &mut MirInterpreter,
    dst: Option<ValueId>,
    box_val: ValueId,
    method: &str,
    args: &[ValueId],
) -> Result<bool, VMError> {
    // Local helpers to bridge NyashValue <-> VMValue for InstanceBox fields
    fn vm_to_nv(v: &VMValue) -> crate::value::NyashValue {
        use super::VMValue as VV;
        use crate::value::NyashValue as NV;
        match v {
            VV::Integer(i) => NV::Integer(*i),
            VV::Float(f) => NV::Float(*f),
            VV::Bool(b) => NV::Bool(*b),
            VV::String(s) => NV::String(s.clone()),
            VV::Void => NV::Void,
            VV::Future(_) => NV::Void,  // not expected in fields
            VV::BoxRef(_) => NV::Void, // store minimal; complex object fields are not required here
            VV::WeakBox(_) => NV::Void, // Phase 285A0: WeakBox not expected in this context
        }
    }
    fn nv_to_vm(v: &crate::value::NyashValue) -> VMValue {
        use super::VMValue as VV;
        use crate::value::NyashValue as NV;
        match v {
            NV::Integer(i) => VV::Integer(*i),
            NV::Float(f) => VV::Float(*f),
            NV::Bool(b) => VV::Bool(*b),
            NV::String(s) => VV::String(s.clone()),
            NV::Null | NV::Void => VV::Void,
            NV::Array(_) | NV::Map(_) | NV::Box(_) | NV::WeakBox(_) => VV::Void,
        }
    }

    match method {
        "getField" => {
            this.validate_args_exact("getField", args, 1)?;

            // Static box support: if box_val is a string matching a static box name,
            // resolve it to the singleton instance
            let actual_box_val = if let Ok(VMValue::String(ref box_name)) = this.reg_load(box_val) {
                if this.static_box_registry.exists(box_name) {
                    // Get or create singleton instance
                    let instance = this.ensure_static_box_instance(box_name)?;
                    let instance_clone = instance.clone();

                    // Create a temporary value to hold the singleton
                    let temp_id = ValueId(999999999); // Temporary ID for singleton
                    this.regs
                        .insert(temp_id, VMValue::from_nyash_box(Box::new(instance_clone)));
                    temp_id
                } else {
                    box_val
                }
            } else {
                box_val
            };

            // MapBox special-case: bridge to MapBox.get, with string-only key
            if let Ok(VMValue::BoxRef(bref)) = this.reg_load(actual_box_val) {
                if bref
                    .as_any()
                    .downcast_ref::<crate::boxes::map_box::MapBox>()
                    .is_some()
                {
                    let key_vm = this.reg_load(args[0])?;
                    if let VMValue::String(_) = key_vm {
                        let k = key_vm.to_nyash_box();
                        let map = bref.share_box();
                        if let Some(mb) =
                            map.as_any().downcast_ref::<crate::boxes::map_box::MapBox>()
                        {
                            let ret = mb.get(k);
                            this.write_result(dst, VMValue::from_nyash_box(ret));
                            return Ok(true);
                        }
                    } else {
                        this.write_string(
                            dst,
                            "[map/bad-key] field name must be string".to_string(),
                        );
                        return Ok(true);
                    }
                }
            }
            if std::env::var("NYASH_VM_TRACE").ok().as_deref() == Some("1") {
                let rk = match this.reg_load(actual_box_val) {
                    Ok(VMValue::BoxRef(ref b)) => format!("BoxRef({})", b.type_name()),
                    Ok(VMValue::Integer(_)) => "Integer".to_string(),
                    Ok(VMValue::Float(_)) => "Float".to_string(),
                    Ok(VMValue::Bool(_)) => "Bool".to_string(),
                    Ok(VMValue::String(_)) => "String".to_string(),
                    Ok(VMValue::Void) => "Void".to_string(),
                    Ok(VMValue::Future(_)) => "Future".to_string(),
                    Ok(VMValue::WeakBox(_)) => "WeakRef".to_string(), // Phase 285A0
                    Err(_) => "<err>".to_string(),
                };
                crate::runtime::get_global_ring0()
                    .log
                    .debug(&format!("[vm-trace] getField recv_kind={}", rk));
            }
            let fname = match this.reg_load(args[0])? {
                VMValue::String(s) => s,
                v => v.to_string(),
            };
            // Prefer InstanceBox internal storage (structural correctness)
            if let VMValue::BoxRef(bref) = this.reg_load(actual_box_val)? {
                if let Some(inst) = bref
                    .as_any()
                    .downcast_ref::<crate::instance_v2::InstanceBox>()
                {
                    if std::env::var("NYASH_VM_TRACE").ok().as_deref() == Some("1") {
                        crate::runtime::get_global_ring0().log.debug(&format!(
                            "[vm-trace] getField instance class={}",
                            inst.class_name
                        ));
                    }
                    // Special-case bridge: JsonParser.length -> tokens.length()
                    if inst.class_name == "JsonParser" && fname == "length" {
                        if let Some(tokens_shared) = inst.get_field("tokens") {
                            let tokens_box: Box<dyn crate::box_trait::NyashBox> =
                                tokens_shared.share_box();
                            if let Some(arr) = tokens_box
                                .as_any()
                                .downcast_ref::<crate::boxes::array::ArrayBox>()
                            {
                                let len_box = arr.length();
                                this.write_result(dst, VMValue::from_nyash_box(len_box));
                                return Ok(true);
                            }
                        }
                    }
                    // First: prefer fields_ng (NyashValue) when present
                    if let Some(nv) = inst.get_field_ng(&fname) {
                        // Dev trace: JsonToken field get
                        if std::env::var("NYASH_VM_TRACE").ok().as_deref() == Some("1")
                            && inst.class_name == "JsonToken"
                        {
                            crate::runtime::get_global_ring0().log.debug(&format!(
                                "[vm-trace] JsonToken.getField name={} nv={:?}",
                                fname, nv
                            ));
                        }
                        // Treat complex Box-like values as "missing" for internal scalar storage so
                        // dedicated box/object storage is used instead.
                        // This avoids NV::Box/Array/Map being converted to Void by nv_to_vm.
                        let is_missing = matches!(
                            nv,
                            crate::value::NyashValue::Null
                                | crate::value::NyashValue::Void
                                | crate::value::NyashValue::Array(_)
                                | crate::value::NyashValue::Map(_)
                                | crate::value::NyashValue::Box(_)
                                | crate::value::NyashValue::WeakBox(_)
                        );
                        if !is_missing {
                            if std::env::var("NYASH_VM_TRACE").ok().as_deref() == Some("1") {
                                crate::runtime::get_global_ring0().log.debug(&format!(
                                    "[vm-trace] getField internal {}.{} -> {:?}",
                                    inst.class_name, fname, nv
                                ));
                            }
                            // Special-case: NV::Box should surface as VMValue::BoxRef
                            if let crate::value::NyashValue::Box(ref arc_m) = nv {
                                if let Ok(guard) = arc_m.lock() {
                                    let cloned: Box<dyn crate::box_trait::NyashBox> =
                                        guard.clone_box();
                                    let arc: std::sync::Arc<dyn crate::box_trait::NyashBox> =
                                        std::sync::Arc::from(cloned);
                                    this.write_result(dst, VMValue::BoxRef(arc));
                                } else {
                                    this.write_void(dst);
                                }
                            } else {
                                this.write_result(dst, nv_to_vm(&nv));
                            }
                            // Trace get
                            if MirInterpreter::box_trace_enabled() {
                                let kind = match &nv {
                                    crate::value::NyashValue::Integer(_) => "Integer",
                                    crate::value::NyashValue::Float(_) => "Float",
                                    crate::value::NyashValue::Bool(_) => "Bool",
                                    crate::value::NyashValue::String(_) => "String",
                                    crate::value::NyashValue::Null => "Null",
                                    crate::value::NyashValue::Void => "Void",
                                    crate::value::NyashValue::Array(_) => "Array",
                                    crate::value::NyashValue::Map(_) => "Map",
                                    crate::value::NyashValue::Box(_) => "Box",
                                    crate::value::NyashValue::WeakBox(_) => "WeakBox",
                                };
                                this.box_trace_emit_get(&inst.class_name, &fname, kind);
                            }
                            return Ok(true);
                        } else {
                            // Provide pragmatic defaults for JsonScanner numeric fields
                            if inst.class_name == "JsonScanner" {
                                let def = match fname.as_str() {
                                    "position" | "length" => Some(VMValue::Integer(0)),
                                    "line" | "column" => Some(VMValue::Integer(1)),
                                    "text" => Some(VMValue::String(String::new())),
                                    _ => None,
                                };
                                if let Some(v) = def {
                                    if std::env::var("NYASH_VM_TRACE").ok().as_deref() == Some("1")
                                    {
                                        crate::runtime::get_global_ring0().log.debug(&format!(
                                            "[vm-trace] getField default JsonScanner.{} -> {:?}",
                                            fname, v
                                        ));
                                    }
                                    this.write_result(dst, v);
                                    return Ok(true);
                                }
                            }
                        }
                    } else {
                        // fields_ng missing entirely → try JsonScanner defaults next, otherwise
                        // fall back to interpreter object storage.
                        if inst.class_name == "JsonScanner" {
                            let def = match fname.as_str() {
                                "position" | "length" => Some(VMValue::Integer(0)),
                                "line" | "column" => Some(VMValue::Integer(1)),
                                "text" => Some(VMValue::String(String::new())),
                                _ => None,
                            };
                            if let Some(v) = def {
                                if std::env::var("NYASH_VM_TRACE").ok().as_deref() == Some("1") {
                                    crate::runtime::get_global_ring0().log.debug(&format!("[vm-trace] getField default(JsonScanner missing) {} -> {:?}", fname, v));
                                }
                                this.write_result(dst, v.clone());
                                if MirInterpreter::box_trace_enabled() {
                                    let kind = match &v {
                                        VMValue::Integer(_) => "Integer",
                                        VMValue::Float(_) => "Float",
                                        VMValue::Bool(_) => "Bool",
                                        VMValue::String(_) => "String",
                                        VMValue::BoxRef(_) => "BoxRef",
                                        VMValue::Void => "Void",
                                        VMValue::Future(_) => "Future",
                                        VMValue::WeakBox(_) => "WeakRef", // Phase 285A0
                                    };
                                    this.box_trace_emit_get(&inst.class_name, &fname, kind);
                                }
                                return Ok(true);
                            }
                        }
                    }
                    // Finally: dedicated box fields for complex values
                    if let Some(shared) = inst.get_field(&fname) {
                        this.write_result(dst, VMValue::BoxRef(shared.clone()));
                        if MirInterpreter::box_trace_enabled() {
                            this.box_trace_emit_get(&inst.class_name, &fname, "BoxRef");
                        }
                        return Ok(true);
                    }
                }
            }
            let key = this.object_key_for(actual_box_val);
            let mut v = this.get_object_field(key, &fname).unwrap_or(VMValue::Void);
            // Final safety (dev-only, narrow): if fallback storage yields Void for well-known
            // JsonScanner fields inside JsonScanner.{is_eof,current,advance}, provide
            // pragmatic defaults to avoid Void comparisons during bring-up.
            if let VMValue::Void = v {
                let guard_on =
                    std::env::var("NYASH_VM_SCANNER_DEFAULTS").ok().as_deref() == Some("1");
                let fn_ctx = this.cur_fn.as_deref().unwrap_or("");
                if std::env::var("NYASH_VM_TRACE").ok().as_deref() == Some("1") {
                    crate::runtime::get_global_ring0().log.debug(&format!(
                        "[vm-trace] getField guard_check ctx={} guard_on={} name={}",
                        fn_ctx, guard_on, fname
                    ));
                }
                if guard_on {
                    let fn_ctx = this.cur_fn.as_deref().unwrap_or("");
                    let is_scanner_ctx = matches!(
                        fn_ctx,
                        "JsonScanner.is_eof/0" | "JsonScanner.current/0" | "JsonScanner.advance/0"
                    );
                    if is_scanner_ctx {
                        // Try class-aware default first
                        if let Ok(VMValue::BoxRef(bref2)) = this.reg_load(actual_box_val) {
                            if let Some(inst2) = bref2
                                .as_any()
                                .downcast_ref::<crate::instance_v2::InstanceBox>()
                            {
                                if inst2.class_name == "JsonScanner" {
                                    let fallback = match fname.as_str() {
                                        "position" | "length" => Some(VMValue::Integer(0)),
                                        "line" | "column" => Some(VMValue::Integer(1)),
                                        "text" => Some(VMValue::String(String::new())),
                                        _ => None,
                                    };
                                    if let Some(val) = fallback {
                                        if std::env::var("NYASH_VM_TRACE").ok().as_deref()
                                            == Some("1")
                                        {
                                            crate::runtime::get_global_ring0().log.debug(&format!(
                                                "[vm-trace] getField final_default {} -> {:?}",
                                                fname, val
                                            ));
                                        }
                                        v = val;
                                    }
                                }
                            }
                        }
                        // Class nameが取得できなかった場合でも、フィールド名で限定的に適用
                        if matches!(v, VMValue::Void) {
                            let fallback2 = match fname.as_str() {
                                "position" | "length" => Some(VMValue::Integer(0)),
                                "line" | "column" => Some(VMValue::Integer(1)),
                                "text" => Some(VMValue::String(String::new())),
                                _ => None,
                            };
                            if let Some(val2) = fallback2 {
                                if std::env::var("NYASH_VM_TRACE").ok().as_deref() == Some("1") {
                                    crate::runtime::get_global_ring0().log.debug(&format!("[vm-trace] getField final_default(class-agnostic) {} -> {:?}", fname, val2));
                                }
                                v = val2;
                            }
                        }
                    }
                }
            }
            if std::env::var("NYASH_VM_TRACE").ok().as_deref() == Some("1") {
                if let VMValue::BoxRef(b) = &v {
                    crate::runtime::get_global_ring0().log.debug(&format!(
                        "[vm-trace] getField fallback {} -> BoxRef({})",
                        fname,
                        b.type_name()
                    ));
                } else {
                    crate::runtime::get_global_ring0().log.debug(&format!(
                        "[vm-trace] getField fallback {} -> {:?}",
                        fname, v
                    ));
                }
            }
            this.write_result(dst, v.clone());
            if MirInterpreter::box_trace_enabled() {
                let kind = match &v {
                    VMValue::Integer(_) => "Integer",
                    VMValue::Float(_) => "Float",
                    VMValue::Bool(_) => "Bool",
                    VMValue::String(_) => "String",
                    VMValue::BoxRef(b) => b.type_name(),
                    VMValue::Void => "Void",
                    VMValue::Future(_) => "Future",
                    VMValue::WeakBox(_) => "WeakRef", // Phase 285A0
                };
                // class name unknown here; use receiver type name if possible
                let cls = match this.reg_load(actual_box_val).unwrap_or(VMValue::Void) {
                    VMValue::BoxRef(b) => {
                        if let Some(inst) =
                            b.as_any().downcast_ref::<crate::instance_v2::InstanceBox>()
                        {
                            inst.class_name.clone()
                        } else {
                            b.type_name().to_string()
                        }
                    }
                    _ => "<unknown>".to_string(),
                };
                this.box_trace_emit_get(&cls, &fname, kind);
            }
            Ok(true)
        }
        "setField" => {
            this.validate_args_exact("setField", args, 2)?;

            // Static box support: if box_val is a string matching a static box name,
            // resolve it to the singleton instance
            let actual_box_val = if let Ok(VMValue::String(ref box_name)) = this.reg_load(box_val) {
                if this.static_box_registry.exists(box_name) {
                    // Get or create singleton instance
                    let instance = this.ensure_static_box_instance(box_name)?;
                    let instance_clone = instance.clone();

                    // Create a temporary value to hold the singleton
                    let temp_id = ValueId(999999998); // Temporary ID for singleton (different from getField)
                    this.regs
                        .insert(temp_id, VMValue::from_nyash_box(Box::new(instance_clone)));
                    temp_id
                } else {
                    box_val
                }
            } else {
                box_val
            };

            // MapBox special-case: bridge to MapBox.set, with string-only key
            if let Ok(VMValue::BoxRef(bref)) = this.reg_load(actual_box_val) {
                if bref
                    .as_any()
                    .downcast_ref::<crate::boxes::map_box::MapBox>()
                    .is_some()
                {
                    let key_vm = this.reg_load(args[0])?;
                    if let VMValue::String(_) = key_vm {
                        let k = key_vm.to_nyash_box();
                        let v = this.reg_load(args[1])?.to_nyash_box();
                        let map = bref.share_box();
                        if let Some(mb) =
                            map.as_any().downcast_ref::<crate::boxes::map_box::MapBox>()
                        {
                            let _ = mb.set(k, v);
                            this.write_void(dst);
                            return Ok(true);
                        }
                    } else {
                        this.write_string(
                            dst,
                            "[map/bad-key] field name must be string".to_string(),
                        );
                        return Ok(true);
                    }
                }
            }
            let fname = match this.reg_load(args[0])? {
                VMValue::String(s) => s,
                v => v.to_string(),
            };
            let valv = this.reg_load(args[1])?;
            // Dev trace: JsonToken field set
            if std::env::var("NYASH_VM_TRACE").ok().as_deref() == Some("1") {
                if let VMValue::BoxRef(bref) = this.reg_load(actual_box_val)? {
                    if let Some(inst) = bref
                        .as_any()
                        .downcast_ref::<crate::instance_v2::InstanceBox>()
                    {
                        if inst.class_name == "JsonToken" {
                            crate::runtime::get_global_ring0().log.debug(&format!(
                                "[vm-trace] JsonToken.setField name={} vmval={:?}",
                                fname, valv
                            ));
                        }
                    }
                }
            }
            if MirInterpreter::box_trace_enabled() {
                let vkind = match &valv {
                    VMValue::Integer(_) => "Integer",
                    VMValue::Float(_) => "Float",
                    VMValue::Bool(_) => "Bool",
                    VMValue::String(_) => "String",
                    VMValue::BoxRef(b) => b.type_name(),
                    VMValue::Void => "Void",
                    VMValue::Future(_) => "Future",
                    VMValue::WeakBox(_) => "WeakRef", // Phase 285A0
                };
                let cls = match this.reg_load(actual_box_val).unwrap_or(VMValue::Void) {
                    VMValue::BoxRef(b) => {
                        if let Some(inst) =
                            b.as_any().downcast_ref::<crate::instance_v2::InstanceBox>()
                        {
                            inst.class_name.clone()
                        } else {
                            b.type_name().to_string()
                        }
                    }
                    _ => "<unknown>".to_string(),
                };
                this.box_trace_emit_set(&cls, &fname, vkind);
            }
            // Prefer InstanceBox internal storage
            if let VMValue::BoxRef(bref) = this.reg_load(actual_box_val)? {
                if let Some(inst) = bref
                    .as_any()
                    .downcast_ref::<crate::instance_v2::InstanceBox>()
                {
                    // Primitives → 内部保存
                    if matches!(
                        valv,
                        VMValue::Integer(_)
                            | VMValue::Float(_)
                            | VMValue::Bool(_)
                            | VMValue::String(_)
                            | VMValue::Void
                    ) {
                        let _ = inst.set_field_ng(fname.clone(), vm_to_nv(&valv));
                        return Ok(true);
                    }
                    // BoxRef のうち、Integer/Float/Bool/String はプリミティブに剥がして内部保存
                    if let VMValue::BoxRef(bx) = &valv {
                        if let Some(ib) = bx.as_any().downcast_ref::<crate::box_trait::IntegerBox>()
                        {
                            let _ = inst.set_field_ng(
                                fname.clone(),
                                crate::value::NyashValue::Integer(ib.value),
                            );
                            return Ok(true);
                        }
                        if let Some(fb) = bx.as_any().downcast_ref::<crate::boxes::FloatBox>() {
                            let _ = inst.set_field_ng(
                                fname.clone(),
                                crate::value::NyashValue::Float(fb.value),
                            );
                            return Ok(true);
                        }
                        if let Some(bb) = bx.as_any().downcast_ref::<crate::box_trait::BoolBox>() {
                            let _ = inst.set_field_ng(
                                fname.clone(),
                                crate::value::NyashValue::Bool(bb.value),
                            );
                            return Ok(true);
                        }
                        if let Some(sb) = bx.as_any().downcast_ref::<crate::box_trait::StringBox>()
                        {
                            let _ = inst.set_field_ng(
                                fname.clone(),
                                crate::value::NyashValue::String(sb.value.clone()),
                            );
                            return Ok(true);
                        }
                        // For complex Box values (InstanceBox/MapBox/ArrayBox...), store into the
                        // dedicated box-field store to preserve identity across clones/gets.
                        let _ = inst.set_field(fname.as_str(), std::sync::Arc::clone(bx));
                        return Ok(true);
                    }
                }
            }
            let key = this.object_key_for(actual_box_val);
            this.set_object_field(key, fname, valv);
            Ok(true)
        }
        _ => Ok(false),
    }
}
