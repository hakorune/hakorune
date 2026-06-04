// Names-only mirror for user box fields.
// Typed authority lives upstream in compiler/MIR metadata; this registry stays
// as a thin reflection store for runtime consumers.

use std::collections::HashMap;
use std::sync::OnceLock;
use std::sync::RwLock;

static USER_BOX_FIELDS: OnceLock<RwLock<HashMap<String, Vec<String>>>> = OnceLock::new();

fn user_box_fields() -> &'static RwLock<HashMap<String, Vec<String>>> {
    USER_BOX_FIELDS.get_or_init(|| RwLock::new(HashMap::new()))
}

pub(crate) fn get_user_box_fields(box_name: &str) -> Option<Vec<String>> {
    let guard = user_box_fields().read().ok()?;
    guard.get(box_name).cloned()
}

pub(crate) fn register_user_box_fields(box_name: String, fields: Vec<String>) {
    if let Ok(mut guard) = user_box_fields().write() {
        guard.insert(box_name, fields);
    }
}
