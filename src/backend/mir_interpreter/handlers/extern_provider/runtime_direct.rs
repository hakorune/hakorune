use super::super::*;
use crate::backend::mir_interpreter::utils::error_helpers::ErrorBuilder;

impl MirInterpreter {
    pub(super) fn dispatch_runtime_direct_extern(
        &mut self,
        extern_name: &str,
        args: &[ValueId],
    ) -> Result<VMValue, VMError> {
        match extern_name {
            "nyash.console.log" | "env.console.log" | "print" | "nyash.builtin.print" => {
                if let Some(a0) = args.get(0) {
                    let v = self.reg_load(*a0).ok();
                    if let Some(v) = v {
                        match &v {
                            VMValue::Void => println!("null"),
                            VMValue::String(s) => println!("{}", s),
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
                            _ => println!("{}", v.to_string()),
                        }
                    } else {
                        println!("");
                    }
                } else {
                    println!("");
                }
                Ok(VMValue::Void)
            }
            "env.console.warn" | "nyash.console.warn" => {
                if let Some(a0) = args.get(0) {
                    let v = self.reg_load(*a0).ok();
                    if let Some(v) = v {
                        match &v {
                            VMValue::Void => eprintln!("[warn] null"),
                            VMValue::String(s) => eprintln!("[warn] {}", s),
                            VMValue::BoxRef(bx) => {
                                if bx
                                    .as_any()
                                    .downcast_ref::<crate::box_trait::VoidBox>()
                                    .is_some()
                                {
                                    eprintln!("[warn] null");
                                } else if let Some(sb) =
                                    bx.as_any().downcast_ref::<crate::box_trait::StringBox>()
                                {
                                    eprintln!("[warn] {}", sb.value);
                                } else {
                                    eprintln!("[warn] {}", v.to_string());
                                }
                            }
                            _ => eprintln!("[warn] {}", v.to_string()),
                        }
                    } else {
                        eprintln!("[warn]");
                    }
                } else {
                    eprintln!("[warn]");
                }
                Ok(VMValue::Void)
            }
            "env.error"
            | "env.error/1"
            | "env.console.error"
            | "env.console.error/1"
            | "nyash.console.error" => {
                if let Some(a0) = args.get(0) {
                    let v = self.reg_load(*a0).ok();
                    if let Some(v) = v {
                        match &v {
                            VMValue::Void => eprintln!("[error] null"),
                            VMValue::String(s) => eprintln!("[error] {}", s),
                            VMValue::BoxRef(bx) => {
                                if bx
                                    .as_any()
                                    .downcast_ref::<crate::box_trait::VoidBox>()
                                    .is_some()
                                {
                                    eprintln!("[error] null");
                                } else if let Some(sb) =
                                    bx.as_any().downcast_ref::<crate::box_trait::StringBox>()
                                {
                                    eprintln!("[error] {}", sb.value);
                                } else {
                                    eprintln!("[error] {}", v.to_string());
                                }
                            }
                            _ => eprintln!("[error] {}", v.to_string()),
                        }
                    } else {
                        eprintln!("[error]");
                    }
                } else {
                    eprintln!("[error]");
                }
                Ok(VMValue::Void)
            }
            "env.get" => {
                if args.is_empty() {
                    return Err(ErrorBuilder::arg_count_mismatch("env.get", 1, args.len()));
                }
                let key = self.reg_load(args[0])?.to_string();
                let val = std::env::var(&key).ok();
                Ok(match val {
                    Some(s) => VMValue::String(s),
                    None => VMValue::Void,
                })
            }
            "env.file.read" => {
                if args.is_empty() {
                    return Err(ErrorBuilder::arg_count_mismatch(
                        "env.file.read",
                        1,
                        args.len(),
                    ));
                }
                let path = self.reg_load(args[0])?.to_string();
                Ok(match std::fs::read_to_string(&path) {
                    Ok(text) => VMValue::String(text),
                    Err(_) => VMValue::Void,
                })
            }
            "env.now_ms" => {
                if !args.is_empty() {
                    return Err(ErrorBuilder::arg_count_mismatch(
                        "env.now_ms",
                        0,
                        args.len(),
                    ));
                }
                let now_ms =
                    match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
                        Ok(d) => d.as_millis() as i64,
                        Err(_) => 0,
                    };
                Ok(VMValue::Integer(now_ms))
            }
            "env.set" => {
                if args.len() < 2 {
                    return Err(ErrorBuilder::arg_count_mismatch("env.set", 2, args.len()));
                }
                let key = self.reg_load(args[0])?.to_string();
                let val = match self.reg_load(args[1])? {
                    VMValue::Void => String::new(),
                    v => v.to_string(),
                };
                std::env::set_var(&key, &val);
                Ok(VMValue::Void)
            }
            _ => Err(self.err_invalid(format!(
                "runtime-direct extern routed to unsupported name: {}",
                extern_name
            ))),
        }
    }
}
