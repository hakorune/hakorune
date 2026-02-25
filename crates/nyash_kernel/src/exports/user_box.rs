// User box registry exports.

use crate::user_box_registry::register_user_box_fields;

/// Register user-defined box declaration (LLVM harness support)
/// Phase 285LLVM-1.1: Enable user box instantiation with fields in LLVM harness mode
///
/// # Arguments
/// * `type_name` - Box type name (e.g., "SomeBox")
/// * `fields_json` - JSON array of field names (e.g., "[\"x\",\"y\"]")
///
/// # Returns
/// * `0` - Success
/// * `-1` - Error: null pointer
/// * `-2` - Error: invalid UTF-8
/// * `-3` - Error: invalid JSON
#[export_name = "nyrt_register_user_box_decl"]
pub extern "C" fn nyrt_register_user_box_decl(
    type_name: *const i8,
    fields_json: *const i8,
) -> i32 {
    use std::ffi::CStr;

    if type_name.is_null() || fields_json.is_null() {
        eprintln!("[nyrt_register_user_box_decl] Error: null pointer");
        return -1;
    }

    let ty = match unsafe { CStr::from_ptr(type_name) }.to_str() {
        Ok(s) => s.to_string(),
        Err(e) => {
            eprintln!(
                "[nyrt_register_user_box_decl] Error: invalid UTF-8 in type_name: {:?}",
                e
            );
            return -2;
        }
    };

    let fields_str = match unsafe { CStr::from_ptr(fields_json) }.to_str() {
        Ok(s) => s,
        Err(e) => {
            eprintln!(
                "[nyrt_register_user_box_decl] Error: invalid UTF-8 in fields_json: {:?}",
                e
            );
            return -2;
        }
    };

    // Parse JSON array of field names
    let fields: Vec<String> = match serde_json::from_str(fields_str) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("[nyrt_register_user_box_decl] Error: invalid JSON in fields: {:?}", e);
            return -3;
        }
    };

    // Store fields in global registry
    // The actual box creation will be handled in nyash_env_box_new_i64x
    register_user_box_fields(ty.clone(), fields.clone());
    eprintln!("[DEBUG] Registered user box '{}' with fields: {:?}", ty, fields);

    0 // Success
}
