use super::*;
use crate::box_trait::{BoolBox, IntegerBox, NyashBox, StringBox};
use std::sync::Arc;

fn collection_any_arg_to_box(arg: i64) -> Box<dyn NyashBox> {
    use crate::runtime::host_handles;

    if arg <= 0 {
        return Box::new(IntegerBox::new(arg));
    }

    host_handles::with_handle(arg as u64, |obj| {
        let Some(obj) = obj else {
            return Box::new(IntegerBox::new(arg)) as Box<dyn NyashBox>;
        };
        if let Some(sb) = obj.as_any().downcast_ref::<StringBox>() {
            return Box::new(StringBox::new(sb.value.clone())) as Box<dyn NyashBox>;
        }
        if let Some(ib) = obj.as_any().downcast_ref::<IntegerBox>() {
            return Box::new(IntegerBox::new(ib.value)) as Box<dyn NyashBox>;
        }
        if let Some(bb) = obj.as_any().downcast_ref::<BoolBox>() {
            return Box::new(BoolBox::new(bb.value)) as Box<dyn NyashBox>;
        }
        Box::new(IntegerBox::new(arg)) as Box<dyn NyashBox>
    })
}

impl MirInterpreter {
    pub(super) fn execute_extern_function(
        &mut self,
        extern_name: &str,
        args: &[ValueId],
    ) -> Result<VMValue, VMError> {
        // Normalize arity suffix (e.g., "env.get/1" -> "env.get")
        let base = super::super::utils::normalize_arity_suffix(extern_name);
        if let Some(res) = self.extern_provider_dispatch(base, args) {
            if std::env::var("HAKO_CABI_TRACE").ok().as_deref() == Some("1") {
                crate::runtime::get_global_ring0()
                    .log
                    .debug(&format!("[hb:dispatch:calls] provider {}", base));
            }
            return res;
        }
        match base {
            "nyash.box.from_i64" => {
                if args.len() < 1 {
                    return Err(self.err_arg_count("nyash.box.from_i64", 1, args.len()));
                }
                let value = self.reg_load(args[0])?.as_integer().unwrap_or(0);
                let arc: Arc<dyn NyashBox> = Arc::new(IntegerBox::new(value));
                let handle = crate::runtime::host_handles::to_handle_arc(arc) as i64;
                Ok(VMValue::Integer(handle))
            }
            "nyash.array.birth_h" => {
                let arc: Arc<dyn NyashBox> = Arc::new(crate::boxes::array::ArrayBox::new());
                let handle = crate::runtime::host_handles::to_handle_arc(arc) as i64;
                Ok(VMValue::Integer(handle))
            }
            "nyash.map.birth_h" => {
                let arc: Arc<dyn NyashBox> = Arc::new(crate::boxes::map_box::MapBox::new());
                let handle = crate::runtime::host_handles::to_handle_arc(arc) as i64;
                Ok(VMValue::Integer(handle))
            }
            "nyash.any.handle_live_h" => {
                if args.len() < 1 {
                    return Err(self.err_arg_count("nyash.any.handle_live_h", 1, args.len()));
                }
                let handle = self.reg_load(args[0])?.as_integer().unwrap_or(0);
                let live =
                    if handle > 0 && crate::runtime::host_handles::get(handle as u64).is_some() {
                        1
                    } else {
                        0
                    };
                Ok(VMValue::Integer(live))
            }
            "nyash.array.slot_len_h" => {
                if args.len() < 1 {
                    return Err(self.err_arg_count("nyash.array.slot_len_h", 1, args.len()));
                }
                let handle = self.reg_load(args[0])?.as_integer().unwrap_or(0);
                let len = if handle <= 0 {
                    0
                } else {
                    crate::runtime::host_handles::with_handle(handle as u64, |obj| {
                        obj.and_then(|obj| {
                            obj.as_any()
                                .downcast_ref::<crate::boxes::array::ArrayBox>()
                                .map(|arr| arr.len() as i64)
                        })
                        .unwrap_or(0)
                    })
                };
                Ok(VMValue::Integer(len))
            }
            "nyash.array.slot_load_hi" => {
                if args.len() < 2 {
                    return Err(self.err_arg_count("nyash.array.slot_load_hi", 2, args.len()));
                }
                let handle = self.reg_load(args[0])?.as_integer().unwrap_or(0);
                let idx = self.reg_load(args[1])?.as_integer().unwrap_or(0);
                let out = if handle <= 0 {
                    0
                } else {
                    crate::runtime::host_handles::with_handle(handle as u64, |obj| {
                        obj.and_then(|obj| {
                            obj.as_any()
                                .downcast_ref::<crate::boxes::array::ArrayBox>()
                                .map(|arr| {
                                    arr.get_index_i64(idx)
                                        .to_string_box()
                                        .value
                                        .parse::<i64>()
                                        .unwrap_or(0)
                                })
                        })
                        .unwrap_or(0)
                    })
                };
                Ok(VMValue::Integer(out))
            }
            "nyash.array.slot_store_hii" => {
                if args.len() < 3 {
                    return Err(self.err_arg_count("nyash.array.slot_store_hii", 3, args.len()));
                }
                let handle = self.reg_load(args[0])?.as_integer().unwrap_or(0);
                let idx = self.reg_load(args[1])?.as_integer().unwrap_or(0);
                let value = self.reg_load(args[2])?.as_integer().unwrap_or(0);
                let rc = if handle <= 0 {
                    0
                } else {
                    crate::runtime::host_handles::with_handle(handle as u64, |obj| {
                        obj.and_then(|obj| {
                            obj.as_any()
                                .downcast_ref::<crate::boxes::array::ArrayBox>()
                                .map(|arr| {
                                    if arr.try_set_index_i64_integer(idx, value) {
                                        1
                                    } else {
                                        0
                                    }
                                })
                        })
                        .unwrap_or(0)
                    })
                };
                Ok(VMValue::Integer(rc))
            }
            "nyash.array.slot_store_hih" => {
                if args.len() < 3 {
                    return Err(self.err_arg_count("nyash.array.slot_store_hih", 3, args.len()));
                }
                let handle = self.reg_load(args[0])?.as_integer().unwrap_or(0);
                let idx = self.reg_load(args[1])?.as_integer().unwrap_or(0);
                let value = self.reg_load(args[2])?.as_integer().unwrap_or(0);
                let rc = if handle <= 0 {
                    0
                } else {
                    crate::runtime::host_handles::with_handle(handle as u64, |obj| {
                        obj.and_then(|obj| {
                            obj.as_any()
                                .downcast_ref::<crate::boxes::array::ArrayBox>()
                                .map(|arr| {
                                    if arr.try_set_index_i64(idx, collection_any_arg_to_box(value))
                                    {
                                        1
                                    } else {
                                        0
                                    }
                                })
                        })
                        .unwrap_or(0)
                    })
                };
                Ok(VMValue::Integer(rc))
            }
            "nyash.array.slot_append_hh" => {
                if args.len() < 2 {
                    return Err(self.err_arg_count("nyash.array.slot_append_hh", 2, args.len()));
                }
                let handle = self.reg_load(args[0])?.as_integer().unwrap_or(0);
                let value = self.reg_load(args[1])?.as_integer().unwrap_or(0);
                let next_len = if handle <= 0 {
                    0
                } else {
                    crate::runtime::host_handles::with_handle(handle as u64, |obj| {
                        obj.and_then(|obj| {
                            obj.as_any()
                                .downcast_ref::<crate::boxes::array::ArrayBox>()
                                .map(|arr| {
                                    arr.slot_append_box_raw(collection_any_arg_to_box(value))
                                })
                        })
                        .unwrap_or(0)
                    })
                };
                Ok(VMValue::Integer(next_len))
            }
            // Minimal console externs
            "nyash.console.log" | "env.console.log" | "print" | "nyash.builtin.print" => {
                if let Some(arg_id) = args.get(0) {
                    let v = self.reg_load(*arg_id)?;
                    match &v {
                        VMValue::Void => println!("null"),
                        VMValue::BoxRef(bx) => {
                            if bx
                                .as_any()
                                .downcast_ref::<crate::box_trait::VoidBox>()
                                .is_some()
                            {
                                println!("null");
                            } else if let Some(sb) =
                                bx.as_any().downcast_ref::<crate::box_trait::StringBox>()
                            {
                                println!("{}", sb.value);
                            } else {
                                println!("{}", v.to_string());
                            }
                        }
                        VMValue::String(s) => println!("{}", s),
                        _ => println!("{}", v.to_string()),
                    }
                } else {
                    println!("");
                }
                Ok(VMValue::Void)
            }
            "nyash.string.concat_hh" => {
                if args.len() < 2 {
                    return Err(self.err_arg_count("nyash.string.concat_hh", 2, args.len()));
                }
                let a = self.reg_load(args[0])?.to_string();
                let b = self.reg_load(args[1])?.to_string();
                Ok(VMValue::String(format!("{}{}", a, b)))
            }
            "nyash.string.concat3_hhh" => {
                if args.len() < 3 {
                    return Err(self.err_arg_count("nyash.string.concat3_hhh", 3, args.len()));
                }
                let a = self.reg_load(args[0])?.to_string();
                let b = self.reg_load(args[1])?.to_string();
                let c = self.reg_load(args[2])?.to_string();
                Ok(VMValue::String(format!("{}{}{}", a, b, c)))
            }
            "nyash.string.substring_len_hii" => {
                if args.len() < 3 {
                    return Err(self.err_arg_count(
                        "nyash.string.substring_len_hii",
                        3,
                        args.len(),
                    ));
                }
                let source = self.reg_load(args[0])?.to_string();
                let start = self.reg_load(args[1])?.as_integer().unwrap_or(0);
                let end = self.reg_load(args[2])?.as_integer().unwrap_or(0);
                let mode = crate::boxes::string_ops::index_mode_from_env();
                let sub = crate::boxes::string_ops::substring(&source, start, Some(end), mode);
                Ok(VMValue::Integer(sub.len() as i64))
            }
            // Direct provider calls (bypass hostbridge.extern_invoke)
            // Above provider covers env.* family; keep legacy fallbacks below
            "exit" => {
                let code = if let Some(arg_id) = args.get(0) {
                    self.reg_load(*arg_id)?.as_integer().unwrap_or(0)
                } else {
                    0
                };
                std::process::exit(code as i32);
            }
            "panic" => {
                let msg = if let Some(arg_id) = args.get(0) {
                    self.reg_load(*arg_id)?.to_string()
                } else {
                    "VM panic".to_string()
                };
                panic!("{}", msg);
            }
            "hostbridge.extern_invoke" => Err(self.err_invalid(
                "hostbridge.extern_invoke should be routed via extern_provider_dispatch",
            )),
            _ => {
                Err(self.err_with_context("extern function", &format!("Unknown: {}", extern_name)))
            }
        }
    }
}
