use super::*;

impl MirInterpreter {
    /// Phase 124: Unified dispatch using TypeRegistry slot numbers
    /// This function replaces the old pattern-matching dispatch with a slot-based approach
    pub(super) fn dispatch_by_slot(
        &mut self,
        receiver: &VMValue,
        type_name: &str,
        slot: u16,
        args: &[ValueId],
    ) -> Result<VMValue, VMError> {
        match (type_name, slot) {
            ("String" | "StringBox", slot)
                if crate::boxes::basic::StringMethodId::from_slot_and_arity(slot, args.len())
                    .is_some() =>
            {
                let method_id =
                    crate::boxes::basic::StringMethodId::from_slot_and_arity(slot, args.len())
                        .expect("checked StringMethodId slot/arity");
                self.invoke_string_surface(receiver, method_id, args)
            }
            // String methods (slot 300+)
            ("String", 300) => {
                // length
                if let VMValue::String(s) = receiver {
                    Ok(VMValue::Integer(s.len() as i64))
                } else {
                    Err(self.err_invalid("String.length: invalid receiver"))
                }
            }
            ("String", 302) => {
                // concat
                if let VMValue::String(s) = receiver {
                    if let Some(arg_id) = args.get(0) {
                        let arg_val = self.reg_load(*arg_id)?;
                        let new_str = format!("{}{}", s, arg_val.to_string());
                        Ok(VMValue::String(new_str))
                    } else {
                        Err(self.err_invalid("String.concat: requires 1 argument"))
                    }
                } else {
                    Err(self.err_invalid("String.concat: invalid receiver"))
                }
            }
            ("String", 304) => {
                // replace
                if let VMValue::String(s) = receiver {
                    if args.len() == 2 {
                        let old = self.reg_load(args[0])?.to_string();
                        let new = self.reg_load(args[1])?.to_string();
                        Ok(VMValue::String(s.replace(&old, &new)))
                    } else {
                        Err(self.err_invalid("String.replace: requires 2 arguments"))
                    }
                } else {
                    Err(self.err_invalid("String.replace: invalid receiver"))
                }
            }
            ("String", 305) => {
                // trim
                if let VMValue::String(s) = receiver {
                    if args.is_empty() {
                        Ok(VMValue::String(s.trim().to_string()))
                    } else {
                        Err(self.err_invalid("String.trim: requires 0 arguments"))
                    }
                } else {
                    Err(self.err_invalid("String.trim: invalid receiver"))
                }
            }
            ("String", 303) => {
                // indexOf
                if let VMValue::String(s) = receiver {
                    let (needle, start) = parse_index_of_args(
                        self,
                        args,
                        ArgParsePolicy::LENIENT,
                        "String.indexOf: requires 1 argument",
                    )?;
                    let mode = string_ops::index_mode_from_env();
                    let idx = string_ops::index_of(s, &needle, start, mode);
                    Ok(VMValue::Integer(idx))
                } else {
                    Err(self.err_invalid("String.indexOf: invalid receiver"))
                }
            }
            ("String", 308) => {
                // lastIndexOf
                if let VMValue::String(s) = receiver {
                    let needle = parse_last_index_of_args(
                        self,
                        args,
                        ArgParsePolicy::LENIENT,
                        "String.lastIndexOf: requires 1 argument",
                    )?;
                    let mode = string_ops::index_mode_from_env();
                    let idx = string_ops::last_index_of(s, &needle, mode);
                    Ok(VMValue::Integer(idx))
                } else {
                    Err(self.err_invalid("String.lastIndexOf: invalid receiver"))
                }
            }
            ("String", 309) => {
                // contains
                if let VMValue::String(s) = receiver {
                    if args.len() == 1 {
                        let needle = self.reg_load(args[0])?.to_string();
                        Ok(VMValue::Bool(s.contains(&needle)))
                    } else {
                        Err(self.err_invalid("String.contains: requires 1 argument"))
                    }
                } else {
                    Err(self.err_invalid("String.contains: invalid receiver"))
                }
            }
            ("String", 301) => {
                // substring
                if let VMValue::String(s) = receiver {
                    let (start, end) = parse_substring_args(
                        self,
                        args,
                        ArgParsePolicy::LENIENT,
                        "String.substring: requires 1 or 2 arguments",
                    )?;
                    let mode = string_ops::index_mode_from_env();
                    let sub = string_ops::substring(s, start, end, mode);
                    Ok(VMValue::String(sub))
                } else {
                    Err(self.err_invalid("String.substring: invalid receiver"))
                }
            }

            // InstanceBox methods (slot 1..4)
            ("InstanceBox", 1) => {
                // getField(name)
                if let Some(out) = with_temp_receiver_dispatch(
                    self,
                    receiver,
                    TMP_RECV_INSTANCE_FIELD_OP,
                    TMP_OUT_INSTANCE_FIELD_OP,
                    |this, recv_tmp, out_tmp| {
                        super::boxes_object_fields::try_handle_object_fields(
                            this,
                            Some(out_tmp),
                            recv_tmp,
                            "getField",
                            args,
                        )
                    },
                )? {
                    return Ok(out);
                }
                Err(self.err_invalid("InstanceBox.getField: unsupported receiver"))
            }
            ("InstanceBox", 2) => {
                // setField(name, value)
                if let Some(out) = with_temp_receiver_dispatch(
                    self,
                    receiver,
                    TMP_RECV_INSTANCE_FIELD_OP,
                    TMP_OUT_INSTANCE_FIELD_OP,
                    |this, recv_tmp, out_tmp| {
                        super::boxes_object_fields::try_handle_object_fields(
                            this,
                            Some(out_tmp),
                            recv_tmp,
                            "setField",
                            args,
                        )
                    },
                )? {
                    return Ok(out);
                }
                Err(self.err_invalid("InstanceBox.setField: unsupported receiver"))
            }
            ("InstanceBox", 3) => {
                // has(name) -> bool
                let out = with_temp_receiver_dispatch(
                    self,
                    receiver,
                    TMP_RECV_INSTANCE_FIELD_OP,
                    TMP_OUT_INSTANCE_FIELD_OP,
                    |this, recv_tmp, out_tmp| {
                        super::boxes_object_fields::try_handle_object_fields(
                            this,
                            Some(out_tmp),
                            recv_tmp,
                            "getField",
                            args,
                        )
                    },
                )?;
                let exists = out.is_some_and(|v| !matches!(v, VMValue::Void));
                Ok(VMValue::Bool(exists))
            }
            ("InstanceBox", 4) => {
                // size()
                if let VMValue::BoxRef(bx) = receiver {
                    if let Some(inst) = bx
                        .as_any()
                        .downcast_ref::<crate::instance_v2::InstanceBox>()
                    {
                        let size = inst.field_count() as i64;
                        return Ok(VMValue::Integer(size));
                    }
                }
                Err(self.err_invalid("InstanceBox.size: invalid receiver"))
            }

            // ArrayBox methods (slot 100+)
            ("ArrayBox", slot) if crate::boxes::array::ArrayMethodId::from_slot(slot).is_some() => {
                let method_id = crate::boxes::array::ArrayMethodId::from_slot(slot)
                    .expect("checked ArrayMethodId slot");
                if let VMValue::BoxRef(bx) = receiver {
                    if let Some(arr) = bx.as_any().downcast_ref::<crate::boxes::array::ArrayBox>() {
                        return self.invoke_array_surface(arr, method_id, args);
                    }
                }
                Err(self.err_invalid(format!(
                    "ArrayBox.{}: invalid receiver",
                    method_id.canonical_name()
                )))
            }

            // MapBox methods (slot 200+)
            ("MapBox", slot) if crate::boxes::MapMethodId::from_slot(slot).is_some() => {
                let method_id =
                    crate::boxes::MapMethodId::from_slot(slot).expect("checked MapMethodId slot");
                if let VMValue::BoxRef(bx) = receiver {
                    if let Some(map) = bx.as_any().downcast_ref::<crate::boxes::MapBox>() {
                        return self.invoke_map_surface(map, method_id, args);
                    }
                }
                Err(self.err_invalid(format!(
                    "MapBox.{}: invalid receiver",
                    method_id.canonical_name()
                )))
            }

            // ConsoleBox methods (slot 400+)
            ("ConsoleBox", 400) => {
                // log/println
                if let VMValue::BoxRef(bx) = receiver {
                    if let Some(console) = bx
                        .as_any()
                        .downcast_ref::<crate::boxes::console_box::ConsoleBox>()
                    {
                        let message = if args.len() > 1 {
                            self.reg_load(args[1])?.to_string()
                        } else if args.len() > 0 {
                            self.reg_load(args[0])?.to_string()
                        } else {
                            return Err(self.err_invalid("ConsoleBox.log: requires 1 argument"));
                        };
                        console.log(&message);
                        return Ok(VMValue::Void);
                    }
                }
                Err(self.err_invalid("ConsoleBox.log: invalid receiver"))
            }
            ("ConsoleBox", 401) => {
                // warn
                if let VMValue::BoxRef(bx) = receiver {
                    if let Some(console) = bx
                        .as_any()
                        .downcast_ref::<crate::boxes::console_box::ConsoleBox>()
                    {
                        let message = if args.len() > 1 {
                            self.reg_load(args[1])?.to_string()
                        } else if args.len() > 0 {
                            self.reg_load(args[0])?.to_string()
                        } else {
                            return Err(self.err_invalid("ConsoleBox.warn: requires 1 argument"));
                        };
                        console.warn(&message);
                        return Ok(VMValue::Void);
                    }
                }
                Err(self.err_invalid("ConsoleBox.warn: invalid receiver"))
            }
            ("ConsoleBox", 402) => {
                // error
                if let VMValue::BoxRef(bx) = receiver {
                    if let Some(console) = bx
                        .as_any()
                        .downcast_ref::<crate::boxes::console_box::ConsoleBox>()
                    {
                        let message = if args.len() > 1 {
                            self.reg_load(args[1])?.to_string()
                        } else if args.len() > 0 {
                            self.reg_load(args[0])?.to_string()
                        } else {
                            return Err(self.err_invalid("ConsoleBox.error: requires 1 argument"));
                        };
                        console.error(&message);
                        return Ok(VMValue::Void);
                    }
                }
                Err(self.err_invalid("ConsoleBox.error: invalid receiver"))
            }
            ("ConsoleBox", 403) => {
                // clear
                if let VMValue::BoxRef(bx) = receiver {
                    if let Some(console) = bx
                        .as_any()
                        .downcast_ref::<crate::boxes::console_box::ConsoleBox>()
                    {
                        console.clear();
                        return Ok(VMValue::Void);
                    }
                }
                Err(self.err_invalid("ConsoleBox.clear: invalid receiver"))
            }

            // StringBox methods (slot 300+, overlaps with String primitive)
            ("StringBox", 308) => {
                // lastIndexOf
                if let VMValue::BoxRef(bx) = receiver {
                    let s_box = bx.to_string_box();
                    let s = s_box.value;
                    let needle = parse_last_index_of_args(
                        self,
                        args,
                        ArgParsePolicy::STRICT,
                        "StringBox.lastIndexOf: requires 1 argument",
                    )?;
                    let helper = crate::boxes::string_box::StringBox::new(s);
                    let result_box = helper.lastIndexOf(&needle);
                    return Ok(VMValue::from_nyash_box(result_box));
                }
                Err(self.err_invalid("StringBox.lastIndexOf: requires 1 argument"))
            }
            ("StringBox", 303) => {
                // indexOf/find
                if let VMValue::BoxRef(bx) = receiver {
                    let s_box = bx.to_string_box();
                    let s = s_box.value;
                    let helper = crate::boxes::string_box::StringBox::new(s);
                    let (needle, start) = parse_index_of_args(
                        self,
                        args,
                        ArgParsePolicy::STRICT,
                        "StringBox.indexOf: requires 1 or 2 arguments",
                    )?;
                    let result_box = match start {
                        Some(start) => helper.find_from(&needle, start),
                        None => helper.find(&needle),
                    };
                    return Ok(VMValue::from_nyash_box(result_box));
                }
                Err(self.err_invalid("StringBox.indexOf: requires 1 or 2 arguments"))
            }
            ("StringBox", 309) => {
                // contains
                if let VMValue::BoxRef(bx) = receiver {
                    if args.len() != 1 {
                        return Err(self.err_invalid("StringBox.contains: requires 1 argument"));
                    }
                    let needle = self.reg_load(args[0])?.to_string();
                    let s = bx.to_string_box().value;
                    return Ok(VMValue::Bool(s.contains(&needle)));
                }
                Err(self.err_invalid("StringBox.contains: invalid receiver"))
            }
            ("StringBox", 305) => {
                // trim
                if let VMValue::BoxRef(bx) = receiver {
                    if args.is_empty() {
                        let trimmed = bx.to_string_box().value.trim().to_string();
                        return Ok(VMValue::String(trimmed));
                    }
                    return Err(self.err_invalid("StringBox.trim: requires 0 arguments"));
                }
                Err(self.err_invalid("StringBox.trim: invalid receiver"))
            }

            // Plugin Box methods (slot >= 1000)
            (_, slot) if slot >= 1000 => {
                if let VMValue::BoxRef(bx) = receiver {
                    if let Some(_p) = bx
                        .as_any()
                        .downcast_ref::<crate::runtime::plugin_loader_v2::PluginBoxV2>()
                    {
                        let host = crate::runtime::plugin_loader_unified::get_global_plugin_host();
                        let _host = host.read().unwrap();
                        let _argv = self.load_args_as_boxes(args)?;
                        // Get method name from slot (reverse lookup would be needed in production)
                        // For now, fall back to old path
                        return Err(self.err_with_context(
                            "Plugin dispatch",
                            &format!("slot {} not yet implemented for plugin boxes", slot),
                        ));
                    }
                }
                Err(self.err_invalid(&format!("Plugin method slot {}: invalid receiver", slot)))
            }

            _ => Err(self.err_with_context(
                "dispatch_by_slot",
                &format!("Unknown type/slot combination: {} slot {}", type_name, slot),
            )),
        }
    }
}
