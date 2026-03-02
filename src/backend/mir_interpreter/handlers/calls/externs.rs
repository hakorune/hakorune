use super::*;

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
