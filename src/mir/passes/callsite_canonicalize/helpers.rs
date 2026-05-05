use std::collections::{BTreeMap, BTreeSet};

use crate::mir::{ConstValue, MirFunction, MirModule, MirType, ValueId};

pub(super) fn canonicalize_legacy_global_name(
    name: &str,
    args_len: usize,
    function_names: &BTreeSet<String>,
) -> String {
    if has_explicit_arity(name) {
        return name.to_string();
    }
    if !name.contains('.') {
        return name.to_string();
    }
    let suffixed = format!("{name}/{args_len}");
    if function_names.contains(&suffixed) {
        return suffixed;
    }
    name.to_string()
}

fn has_explicit_arity(name: &str) -> bool {
    matches!(
        name.rsplit_once('/'),
        Some((_base, arity)) if arity.chars().all(|c| c.is_ascii_digit())
    )
}

pub(super) fn collect_const_string_literals(func: &MirFunction) -> BTreeMap<ValueId, String> {
    let mut out = BTreeMap::new();
    for block in func.blocks.values() {
        for inst in &block.instructions {
            if let crate::mir::MirInstruction::Const {
                dst,
                value: ConstValue::String(s),
            } = inst
            {
                out.insert(*dst, s.clone());
            }
        }
    }
    out
}

pub(super) fn collect_const_null_sentinels(func: &MirFunction) -> BTreeSet<ValueId> {
    let mut out = BTreeSet::new();
    for block in func.blocks.values() {
        for inst in &block.instructions {
            if let crate::mir::MirInstruction::Const {
                dst,
                value: ConstValue::Null | ConstValue::Void,
            } = inst
            {
                out.insert(*dst);
            }
        }
    }
    out
}

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

pub(super) fn parse_user_box_method_global_name(name: &str) -> Option<(&str, &str, usize)> {
    let (base, arity) = name.rsplit_once('/')?;
    let explicit_arity = arity.parse::<usize>().ok()?;
    let (box_name, method_name) = base.rsplit_once('.')?;
    Some((box_name, method_name, explicit_arity))
}
