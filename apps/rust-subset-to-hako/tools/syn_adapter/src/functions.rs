use serde_json::{json, Value};
use syn::FnArg;

use crate::stmts::block_stmts_to_json;
use crate::types::{pat_name, return_type, type_name};

pub(crate) fn function_to_json(func: &syn::Signature, block: &syn::Block) -> Value {
    let mut receiver = "none".to_string();
    let mut params = Vec::new();

    for input in &func.inputs {
        match input {
            FnArg::Receiver(recv) => {
                receiver = if recv.reference.is_some() {
                    if recv.mutability.is_some() {
                        "self_mut"
                    } else {
                        "self_ref"
                    }
                } else {
                    "self_value"
                }
                .to_string();
            }
            FnArg::Typed(arg) => {
                let name =
                    pat_name(arg.pat.as_ref()).unwrap_or_else(|| "unsupported_param".to_string());
                params.push(json!({"name": name, "type": type_name(arg.ty.as_ref())}));
            }
        }
    }

    let mut value = json!({
        "kind": "Function",
        "name": func.ident.to_string(),
        "params": params,
        "return_type": return_type(&func.output),
        "body": block_stmts_to_json(block, true),
    });
    if receiver != "none" {
        value["receiver"] = json!(receiver);
    }
    value
}
