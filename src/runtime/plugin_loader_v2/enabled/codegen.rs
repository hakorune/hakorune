//! External function implementations for `env.codegen.*`.

use crate::bid::{BidError, BidResult};
use crate::box_trait::{NyashBox, StringBox};

pub(super) fn handle_codegen(
    method_name: &str,
    args: &[Box<dyn NyashBox>],
) -> BidResult<Option<Box<dyn NyashBox>>> {
    match method_name {
        "compile_ll_text" => {
            let ll_text = args
                .first()
                .map(|b| b.to_string_box().value)
                .unwrap_or_default();
            let out = args.get(1).map(|b| b.to_string_box().value);
            match super::compat_codegen_receiver::compile_ll_text(&ll_text, out) {
                Ok(p) => {
                    Ok(Some(Box::new(StringBox::new(p)) as Box<dyn NyashBox>))
                }
                Err(_e) => Ok(None),
            }
        }
        "emit_object" => {
            let mir_json = args
                .get(0)
                .map(|b| b.to_string_box().value)
                .unwrap_or_default();
            match super::compat_codegen_receiver::emit_object(&mir_json, false) {
                Ok(p) => {
                    Ok(Some(Box::new(StringBox::new(p)) as Box<dyn NyashBox>))
                }
                Err(_e) => Ok(None),
            }
        }
        "link_object" => {
            let obj_path = args
                .first()
                .map(|b| b.to_string_box().value)
                .unwrap_or_default();
            let exe_out = args.get(1).map(|b| b.to_string_box().value);
            let extra = args.get(2).map(|b| b.to_string_box().value);
            match super::compat_codegen_receiver::link_object(&obj_path, exe_out, extra) {
                Ok(p) => {
                    Ok(Some(Box::new(StringBox::new(p)) as Box<dyn NyashBox>))
                }
                Err(_e) => Ok(None),
            }
        }
        _ => Err(BidError::PluginError),
    }
}
