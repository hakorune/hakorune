use std::collections::BTreeMap;

use crate::mir::function::TypedObjectFieldStorage;

use super::state::{BoxOriginInference, FieldKey, FieldStorageInference, ParamKey};

pub(super) fn merge_storage_observation(
    inferred: &mut BTreeMap<FieldKey, FieldStorageInference>,
    key: FieldKey,
    storage: TypedObjectFieldStorage,
) -> bool {
    merge_storage_inference(inferred, key, storage)
}

pub(super) fn merge_param_storage_observation(
    inferred: &mut BTreeMap<ParamKey, FieldStorageInference>,
    key: ParamKey,
    storage: TypedObjectFieldStorage,
) -> bool {
    merge_storage_inference(inferred, key, storage)
}

pub(super) fn merge_storage_inference<K: Ord>(
    inferred: &mut BTreeMap<K, FieldStorageInference>,
    key: K,
    storage: TypedObjectFieldStorage,
) -> bool {
    use std::collections::btree_map::Entry;
    match inferred.entry(key) {
        Entry::Vacant(slot) => {
            slot.insert(FieldStorageInference::Known(storage));
            true
        }
        Entry::Occupied(mut slot) => {
            let next = match *slot.get() {
                FieldStorageInference::Known(existing) if existing == storage => {
                    FieldStorageInference::Known(existing)
                }
                FieldStorageInference::Known(_) | FieldStorageInference::Conflict => {
                    FieldStorageInference::Conflict
                }
            };
            let changed = *slot.get() != next;
            slot.insert(next);
            changed
        }
    }
}

pub(super) fn merge_box_origin_observation<K: Ord>(
    inferred: &mut BTreeMap<K, BoxOriginInference>,
    key: K,
    box_name: String,
) -> bool {
    use std::collections::btree_map::Entry;
    match inferred.entry(key) {
        Entry::Vacant(slot) => {
            slot.insert(BoxOriginInference::Known(box_name));
            true
        }
        Entry::Occupied(mut slot) => {
            let next = match slot.get() {
                BoxOriginInference::Known(existing) if existing == &box_name => {
                    BoxOriginInference::Known(existing.clone())
                }
                BoxOriginInference::Known(_) | BoxOriginInference::Conflict => {
                    BoxOriginInference::Conflict
                }
            };
            let changed = slot.get() != &next;
            slot.insert(next);
            changed
        }
    }
}
