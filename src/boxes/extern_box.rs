/*!
 * ExternBox - External API proxy for Phase 9.7 ExternCall
 */

use crate::box_trait::{NyashBox, StringBox, BoolBox, VoidBox, IntegerBox, BoxCore, BoxBase};
use crate::config::env;
use crate::runtime::get_global_ring0;
use std::any::Any;

/// External API proxy box for external calls
pub struct ExternBox {
    id: u64,
    api_name: String,
}

fn debug_log_enabled() -> bool {
    env::cli_verbose_enabled() || env::debug_plugin()
}

impl ExternBox {
    pub fn new_console() -> Box<dyn NyashBox> {
        Box::new(ExternBox {
            id: BoxBase::generate_box_id(),
            api_name: "console".to_string(),
        })
    }
    
    pub fn new_canvas() -> Box<dyn NyashBox> {
        Box::new(ExternBox {
            id: BoxBase::generate_box_id(),
            api_name: "canvas".to_string(),
        })
    }
}

impl BoxCore for ExternBox {
    fn box_id(&self) -> u64 {
        self.id
    }
    
    fn parent_type_id(&self) -> Option<std::any::TypeId> {
        None // ExternBox doesn't inherit from other built-in boxes
    }
    
    fn fmt_box(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ExternBox({})", self.api_name)
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl NyashBox for ExternBox {
    fn to_string_box(&self) -> StringBox {
        StringBox::new(format!("ExternBox({})", self.api_name))
    }
    
    fn equals(&self, other: &dyn NyashBox) -> BoolBox {
        if let Some(other_extern) = other.as_any().downcast_ref::<ExternBox>() {
            BoolBox::new(self.id == other_extern.id)
        } else {
            BoolBox::new(false)
        }
    }
    
    fn type_name(&self) -> &'static str {
        "ExternBox"
    }
    
    fn clone_box(&self) -> Box<dyn NyashBox> {
        Box::new(ExternBox { 
            id: self.id,
            api_name: self.api_name.clone(),
        })
    }
    
    fn share_box(&self) -> Box<dyn NyashBox> {
        // ExternBox is stateless, so share_box and clone_box behave the same
        self.clone_box()
    }

    fn call_method(&mut self, method: &str, args: Vec<Box<dyn NyashBox>>) -> Box<dyn NyashBox> {
        let debug_log = debug_log_enabled();
        if debug_log {
            get_global_ring0().log.debug(&format!(
                "[extern_box] {}.{} called with {} args",
                self.api_name,
                method,
                args.len()
            ));
        }
        
        match (self.api_name.as_str(), method) {
            ("console", "log") => {
                if debug_log {
                    let mut msg = String::from("Console:");
                    for (i, arg) in args.iter().enumerate() {
                        if i > 0 {
                            msg.push(' ');
                        }
                        msg.push_str(&arg.to_string_box().value);
                    }
                    get_global_ring0()
                        .log
                        .info(&format!("[extern_box] {}", msg));
                }
                print!("Console:");
                for arg in args.iter() {
                    print!(" {}", arg.to_string_box().value);
                }
                println!();
                Box::new(VoidBox::new())
            },
            ("canvas", "fillRect") => {
                if args.len() >= 6 {
                    if debug_log {
                        get_global_ring0().log.info(&format!(
                            "[extern_box] Canvas fillRect: canvas={}, x={}, y={}, w={}, h={}, color={}",
                            args[0].to_string_box().value,
                            args[1].to_string_box().value,
                            args[2].to_string_box().value,
                            args[3].to_string_box().value,
                            args[4].to_string_box().value,
                            args[5].to_string_box().value
                        ));
                    }
                } else {
                    if debug_log {
                        get_global_ring0().log.warn(&format!(
                            "[extern_box] Canvas fillRect called with {} args (expected 6)",
                            args.len()
                        ));
                    }
                }
                Box::new(VoidBox::new())
            },
            ("canvas", "fillText") => {
                if args.len() >= 6 {
                    if debug_log {
                        get_global_ring0().log.info(&format!(
                            "[extern_box] Canvas fillText: canvas={}, text={}, x={}, y={}, font={}, color={}",
                            args[0].to_string_box().value,
                            args[1].to_string_box().value,
                            args[2].to_string_box().value,
                            args[3].to_string_box().value,
                            args[4].to_string_box().value,
                            args[5].to_string_box().value
                        ));
                    }
                } else {
                    if debug_log {
                        get_global_ring0().log.warn(&format!(
                            "[extern_box] Canvas fillText called with {} args (expected 6)",
                            args.len()
                        ));
                    }
                }
                Box::new(VoidBox::new())
            },
            _ => {
                if debug_log {
                    get_global_ring0().log.warn(&format!(
                        "[extern_box] Unknown external method: {}.{}",
                        self.api_name, method
                    ));
                }
                Box::new(VoidBox::new())
            }
        }
    }

    fn get_field(&self, _field: &str) -> Option<Box<dyn NyashBox>> {
        None
    }

    fn set_field(&mut self, _field: &str, _value: Box<dyn NyashBox>) -> bool {
        false
    }

    fn list_methods(&self) -> Vec<String> {
        match self.api_name.as_str() {
            "console" => vec!["log".to_string()],
            "canvas" => vec!["fillRect".to_string(), "fillText".to_string()],
            _ => vec![],
        }
    }

    fn list_fields(&self) -> Vec<String> {
        vec![]
    }
}
