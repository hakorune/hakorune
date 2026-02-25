// Phase 285LLVM-1.1: Global registry for user box field declarations.

use std::collections::HashMap;
use std::sync::RwLock;

static USER_BOX_FIELDS: RwLock<Option<HashMap<String, Vec<String>>>> = RwLock::new(None);

pub(crate) fn get_user_box_fields(box_name: &str) -> Option<Vec<String>> {
    if let Ok(guard) = USER_BOX_FIELDS.read() {
        if let Some(ref map) = *guard {
            return map.get(box_name).cloned();
        }
    }
    None
}

pub(crate) fn register_user_box_fields(box_name: String, fields: Vec<String>) {
    if let Ok(mut guard) = USER_BOX_FIELDS.write() {
        if guard.is_none() {
            *guard = Some(HashMap::new());
        }
        if let Some(ref mut map) = *guard {
            map.insert(box_name, fields);
        }
    }
}

// Phase 285LLVM-1.1: Factory function for user-defined boxes.
#[allow(dead_code)]
pub(crate) fn create_user_box_from_registry(
    box_name: &str,
    _args: &[Box<dyn nyash_rust::box_trait::NyashBox>],
) -> Result<Box<dyn nyash_rust::box_trait::NyashBox>, String> {
    use nyash_rust::{box_trait::NyashBox, instance_v2::InstanceBox};
    use std::collections::HashMap as StdHashMap;

    if let Some(fields) = get_user_box_fields(box_name) {
        let instance = InstanceBox::from_declaration(
            box_name.to_string(),
            fields,
            StdHashMap::new(),
        );
        Ok(Box::new(instance) as Box<dyn NyashBox>)
    } else {
        Err(format!(
            "User box '{}' not registered in field registry",
            box_name
        ))
    }
}
