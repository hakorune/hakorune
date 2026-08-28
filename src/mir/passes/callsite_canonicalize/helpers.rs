use std::collections::{BTreeMap, BTreeSet};

use crate::mir::{MirModule, MirType, ValueId};

pub(super) fn collect_known_user_boxes(module: &MirModule) -> BTreeSet<String> {
    module
        .metadata
        .user_box_decls
        .keys()
        .chain(module.metadata.user_box_field_decls.keys())
        .cloned()
        .collect()
}

pub(super) fn known_user_box_name_from_value<'a>(
    value_types: &'a BTreeMap<ValueId, MirType>,
    known_user_boxes: &BTreeSet<String>,
    value: ValueId,
) -> Option<&'a str> {
    let MirType::Box(box_name) = value_types.get(&value)? else {
        return None;
    };
    if known_user_boxes.contains(box_name) {
        Some(box_name.as_str())
    } else {
        None
    }
}
