use serde_json::{json, Value};
use syn::FnArg;

use crate::cli::fail;
use crate::names::{assert_unique_names, insert_name_metadata};
use crate::stmts::block_stmts_to_json;
use crate::types::{insert_pat_name_metadata, return_type, type_name};

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
                let mut param = json!({"type": type_name(arg.ty.as_ref())});
                if !insert_pat_name_metadata(&mut param, arg.pat.as_ref()) {
                    fail("unsupported function parameter pattern out of v0 scope");
                }
                params.push(param);
            }
        }
    }
    assert_unique_names(&params, "function params");

    let mut value = json!({
        "kind": "Function",
        "params": params,
        "return_type": return_type(&func.output),
        "body": block_stmts_to_json(block, true),
    });
    insert_name_metadata(&mut value, &func.ident.to_string());
    if receiver != "none" {
        value["receiver"] = json!(receiver);
    }
    value
}
