use super::*;
use crate::box_trait::NyashBox;

pub(super) fn try_handle_instance_box(
    this: &mut MirInterpreter,
    dst: Option<ValueId>,
    box_val: ValueId,
    method: &str,
    args: &[ValueId],
) -> Result<bool, VMError> {
    let recv_vm = this.reg_load(box_val)?;
    let recv_box_any: Box<dyn NyashBox> = match recv_vm.clone() {
        VMValue::BoxRef(b) => b.share_box(),
        other => other.to_nyash_box(),
    };
    if std::env::var("NYASH_VM_TRACE").ok().as_deref() == Some("1") && method == "toString" {
        crate::runtime::get_global_ring0().log.debug(&format!(
            "[vm-trace] instance-check recv_box_any.type={} args_len={}",
            recv_box_any.type_name(),
            args.len()
        ));
    }
    if let Some(inst) = recv_box_any
        .as_any()
        .downcast_ref::<crate::instance_v2::InstanceBox>()
    {
        // Development guard: ensure JsonScanner core fields have sensible defaults
        if inst.class_name == "JsonScanner" {
            // populate missing fields to avoid Void in comparisons inside is_eof/advance
            if inst.get_field_ng("position").is_none() {
                let _ =
                    inst.set_field_ng("position".to_string(), crate::value::NyashValue::Integer(0));
            }
            if inst.get_field_ng("length").is_none() {
                let _ =
                    inst.set_field_ng("length".to_string(), crate::value::NyashValue::Integer(0));
            }
            if inst.get_field_ng("line").is_none() {
                let _ = inst.set_field_ng("line".to_string(), crate::value::NyashValue::Integer(1));
            }
            if inst.get_field_ng("column").is_none() {
                let _ =
                    inst.set_field_ng("column".to_string(), crate::value::NyashValue::Integer(1));
            }
            if inst.get_field_ng("text").is_none() {
                let _ = inst.set_field_ng(
                    "text".to_string(),
                    crate::value::NyashValue::String(String::new()),
                );
            }
        }
        // JsonNodeInstance narrow bridges removed: rely on builder rewrite and instance dispatch
        // birth: do not short-circuit; allow dispatch to lowered function "Class.birth/arity"
        if std::env::var("NYASH_VM_TRACE").ok().as_deref() == Some("1") && method == "toString" {
            crate::runtime::get_global_ring0().log.debug(&format!(
                "[vm-trace] instance-check downcast=ok class={} stringify_present={{class:{}, alt:{}}}",
                inst.class_name,
                this.functions.contains_key(&format!("{}.stringify/0", inst.class_name)),
                this.functions.contains_key(&format!("{}Instance.stringify/0", inst.class_name))
            ));
        }
        // Resolve lowered method function: "Class.method/arity"
        let primary = format!(
            "{}.{}{}",
            inst.class_name,
            method,
            format!("/{}", args.len())
        );
        // Alternate naming: "ClassInstance.method/arity"
        let alt = format!(
            "{}Instance.{}{}",
            inst.class_name,
            method,
            format!("/{}", args.len())
        );
        // Static method variant that takes 'me' explicitly as first arg: "Class.method/(arity+1)"
        let static_variant = format!(
            "{}.{}{}",
            inst.class_name,
            method,
            format!("/{}", args.len() + 1)
        );
        // Special-case: toString() → stringify/0 if present
        // Prefer base class (strip trailing "Instance") stringify when available.
        let (stringify_base, stringify_inst) = if method == "toString" && args.is_empty() {
            let base = inst
                .class_name
                .strip_suffix("Instance")
                .map(|s| s.to_string());
            let base_name = base.unwrap_or_else(|| inst.class_name.clone());
            (
                Some(format!("{}.stringify/0", base_name)),
                Some(format!("{}.stringify/0", inst.class_name)),
            )
        } else {
            (None, None)
        };

        if std::env::var("NYASH_VM_TRACE").ok().as_deref() == Some("1") {
            crate::runtime::get_global_ring0().log.debug(&format!(
                "[vm-trace] instance-dispatch class={} method={} arity={} candidates=[{}, {}, {}]",
                inst.class_name,
                method,
                args.len(),
                primary,
                alt,
                static_variant
            ));
        }

        // Prefer stringify for toString() if present (semantic alias). Try instance first, then base.
        let func_opt = if let Some(ref sname) = stringify_inst {
            this.functions.get(sname).cloned()
        } else {
            None
        }
        .or_else(|| {
            stringify_base
                .as_ref()
                .and_then(|n| this.functions.get(n).cloned())
        })
        .or_else(|| this.functions.get(&primary).cloned())
        .or_else(|| this.functions.get(&alt).cloned())
        .or_else(|| this.functions.get(&static_variant).cloned());

        if let Some(func) = func_opt {
            if std::env::var("NYASH_VM_TRACE").ok().as_deref() == Some("1") {
                crate::runtime::get_global_ring0().log.debug(&format!(
                    "[vm-trace] instance-dispatch hit -> {}",
                    func.signature.name
                ));
            }
            // Phase 21.9 fix: Check if function expects 'me' as first param
            // Static box methods don't include 'me' in their signature, so we should
            // not prepend 'me' when calling them.
            let expected_params = func.params.len();
            let call_arity_with_me = 1 + args.len();
            let call_arity_without_me = args.len();

            let include_me = if expected_params == call_arity_with_me {
                // Function expects me + args (instance method)
                true
            } else if expected_params == call_arity_without_me {
                // Function only expects args (static method)
                false
            } else {
                // Mismatch - fallback to old behavior (include me)
                true
            };

            if std::env::var("NYASH_VM_TRACE").ok().as_deref() == Some("1") {
                crate::runtime::get_global_ring0().log.debug(&format!(
                    "[vm-trace] instance-dispatch param-check: expected={} with_me={} without_me={} include_me={}",
                    expected_params, call_arity_with_me, call_arity_without_me, include_me
                ));
            }

            let mut argv: Vec<VMValue> = Vec::with_capacity(if include_me {
                1 + args.len()
            } else {
                args.len()
            });
            // Dev assert: forbid birth(me==Void)
            if method == "birth" && crate::config::env::using_is_dev() {
                if matches!(recv_vm, VMValue::Void) {
                    return Err(this.err_invalid("Dev assert: birth(me==Void) is forbidden"));
                }
            }
            if include_me {
                argv.push(recv_vm.clone());
            }
            for a in args {
                argv.push(this.reg_load(*a)?);
            }
            let ret = this.exec_function_inner(&func, Some(&argv))?;
            this.write_result(dst, ret);
            return Ok(true);
        } else {
            // Conservative fallback: search unique function by name tail ".method/arity"
            let tail = format!(".{}{}", method, format!("/{}", args.len()));
            let cands: Vec<String> = this
                .functions
                .keys()
                .filter(|k| k.ends_with(&tail))
                .cloned()
                .collect();
            if !cands.is_empty() {
                // Always narrow by receiver class prefix (and optional "Instance" suffix)
                let recv_cls = inst.class_name.clone();
                let pref1 = format!("{}.", recv_cls);
                let pref2 = format!("{}Instance.", recv_cls);
                let filtered: Vec<String> = cands
                    .into_iter()
                    .filter(|k| k.starts_with(&pref1) || k.starts_with(&pref2))
                    .collect();
                if filtered.len() == 1 {
                    let fname = &filtered[0];
                    if std::env::var("NYASH_VM_TRACE").ok().as_deref() == Some("1") {
                        crate::runtime::get_global_ring0().log.debug(&format!(
                            "[vm-trace] instance-dispatch fallback (scoped) -> {}",
                            fname
                        ));
                    }
                    if let Some(func) = this.functions.get(fname).cloned() {
                        // Phase 21.9 fix: Same logic as primary path
                        let expected_params = func.params.len();
                        let call_arity_with_me = 1 + args.len();
                        let call_arity_without_me = args.len();

                        let include_me = if expected_params == call_arity_with_me {
                            true
                        } else if expected_params == call_arity_without_me {
                            false
                        } else {
                            true
                        };

                        let mut argv: Vec<VMValue> = Vec::with_capacity(if include_me {
                            1 + args.len()
                        } else {
                            args.len()
                        });
                        if method == "birth" && crate::config::env::using_is_dev() {
                            if matches!(recv_vm, VMValue::Void) {
                                return Err(
                                    this.err_invalid("Dev assert: birth(me==Void) is forbidden")
                                );
                            }
                        }
                        if include_me {
                            argv.push(recv_vm.clone());
                        }
                        for a in args {
                            argv.push(this.reg_load(*a)?);
                        }
                        let ret = this.exec_function_inner(&func, Some(&argv))?;
                        if let Some(d) = dst {
                            this.regs.insert(d, ret);
                        }
                        return Ok(true);
                    }
                } else if filtered.len() > 1 {
                    if std::env::var("NYASH_VM_TRACE").ok().as_deref() == Some("1") {
                        crate::runtime::get_global_ring0().log.debug(&format!("[vm-trace] instance-dispatch multiple candidates after narrowing: {:?}", filtered));
                    }
                    // Ambiguous: do not dispatch cross-class
                } else {
                    // No same-class candidate: do not dispatch cross-class
                    if std::env::var("NYASH_VM_TRACE").ok().as_deref() == Some("1") {
                        crate::runtime::get_global_ring0().log.debug(&format!(
                            "[vm-trace] instance-dispatch no same-class candidate for tail .{}{}",
                            method,
                            format!("/{}", args.len())
                        ));
                    }
                }
            }
        }
    }
    Ok(false)
}
