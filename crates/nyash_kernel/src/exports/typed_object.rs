// Typed user object runtime helpers for EXE lowering.
//
// This is intentionally slot-based and opaque to the backend. MIR owns layout
// truth; the runtime owns allocation and field storage.

use std::sync::{Mutex, OnceLock};

const MAX_TYPED_OBJECT_FIELDS: i64 = 4096;

#[derive(Debug, Clone)]
struct TypedSlotObject {
    #[allow(dead_code)]
    type_id: i64,
    fields: Vec<i64>,
}

static TYPED_OBJECTS: OnceLock<Mutex<Vec<TypedSlotObject>>> = OnceLock::new();

fn typed_objects() -> &'static Mutex<Vec<TypedSlotObject>> {
    TYPED_OBJECTS.get_or_init(|| Mutex::new(Vec::new()))
}

fn handle_to_index(handle: i64) -> Option<usize> {
    if handle >= 0 {
        return None;
    }
    let idx = handle.checked_neg()?.checked_sub(1)?;
    usize::try_from(idx).ok()
}

#[export_name = "nyash.object.new_typed_hi"]
pub extern "C" fn nyash_object_new_typed_hi(type_id: i64, field_count: i64) -> i64 {
    if field_count < 0 || field_count > MAX_TYPED_OBJECT_FIELDS {
        return 0;
    }
    let mut objects = match typed_objects().lock() {
        Ok(objects) => objects,
        Err(_) => return 0,
    };
    objects.push(TypedSlotObject {
        type_id,
        fields: vec![0; field_count as usize],
    });
    -(objects.len() as i64)
}

#[export_name = "nyash.object.new_typed_h"]
pub extern "C" fn nyash_object_new_typed_h(type_id: i64) -> i64 {
    nyash_object_new_typed_hi(type_id, 0)
}

#[export_name = "nyash.object.field_get_hii"]
pub extern "C" fn nyash_object_field_get_hii(handle: i64, slot: i64) -> i64 {
    if slot < 0 {
        return 0;
    }
    let Some(idx) = handle_to_index(handle) else {
        return 0;
    };
    let slot = slot as usize;
    let objects = match typed_objects().lock() {
        Ok(objects) => objects,
        Err(_) => return 0,
    };
    objects
        .get(idx)
        .and_then(|object| object.fields.get(slot))
        .copied()
        .unwrap_or(0)
}

#[export_name = "nyash.object.field_set_hii"]
pub extern "C" fn nyash_object_field_set_hii(handle: i64, slot: i64, value: i64) {
    if slot < 0 {
        return;
    }
    let Some(idx) = handle_to_index(handle) else {
        return;
    };
    let slot = slot as usize;
    let mut objects = match typed_objects().lock() {
        Ok(objects) => objects,
        Err(_) => return,
    };
    if let Some(field) = objects
        .get_mut(idx)
        .and_then(|object| object.fields.get_mut(slot))
    {
        *field = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn typed_object_helpers_store_and_load_i64_slots() {
        let object = nyash_object_new_typed_hi(7, 2);
        assert!(object < 0);

        nyash_object_field_set_hii(object, 0, 10);
        nyash_object_field_set_hii(object, 1, 20);

        assert_eq!(nyash_object_field_get_hii(object, 0), 10);
        assert_eq!(nyash_object_field_get_hii(object, 1), 20);
        assert_eq!(nyash_object_field_get_hii(object, 2), 0);
    }
}
