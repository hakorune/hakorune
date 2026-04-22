//! TypeAnnotationBox — MIR 値への型注釈（仕様不変の最小）

use crate::mir::builder::MirBuilder;
use crate::mir::{MirType, ValueId};

/// 直接的に MirType を設定する（仕様不変）。
#[inline]
#[allow(dead_code)]
pub fn set_type(builder: &mut MirBuilder, dst: ValueId, ty: MirType) {
    builder.type_ctx.value_types.insert(dst, ty);
}

/// 関数名から既知の戻り型を注釈する（最小ハードコード）。
/// 例: "StringBox.str/0" → MirType::String
#[inline]
pub fn annotate_from_function(builder: &mut MirBuilder, dst: ValueId, func_name: &str) {
    if let Some(ty) = infer_return_type(func_name) {
        builder.type_ctx.value_types.insert(dst, ty);
    }
}

pub(crate) fn infer_return_type(func_name: &str) -> Option<MirType> {
    if let Some(id) = crate::mir::naming::StaticMethodId::parse(func_name) {
        if let Some(arity) = id.arity {
            if let Some(ty) = infer_method_return_type(Some(&id.box_name), &id.method, Some(arity))
            {
                return Some(ty);
            }
        }
    }

    // Very small whitelist; 仕様不変（既知の戻り型のみ）
    // Normalize forms like "JsonNode.str/0" or "StringBox.length/0" if needed
    if func_name.ends_with(".str/0") {
        return Some(MirType::String);
    }
    if func_name.ends_with(".length/0") {
        return Some(MirType::Integer);
    }
    if func_name.ends_with(".size/0") {
        return Some(MirType::Integer);
    }
    if func_name.ends_with(".len/0") {
        return Some(MirType::Integer);
    }
    if func_name.ends_with(".substring/2") {
        return Some(MirType::String);
    }
    if func_name.ends_with(".esc_json/0") {
        return Some(MirType::String);
    }
    if func_name.ends_with(".indexOf/1") {
        return Some(MirType::Integer);
    }
    if func_name.ends_with(".indexOf/2") {
        return Some(MirType::Integer);
    }
    if func_name.ends_with(".lastIndexOf/1") {
        return Some(MirType::Integer);
    }
    if func_name.ends_with(".is_digit_char/1") {
        return Some(MirType::Bool);
    }
    if func_name.ends_with(".is_hex_digit_char/1") {
        return Some(MirType::Bool);
    }
    if func_name.ends_with(".is_alpha_char/1") {
        return Some(MirType::Bool);
    }
    // Fallback: none (変更なし)
    None
}

pub(crate) fn infer_method_return_type(
    box_name: Option<&str>,
    method: &str,
    arity: Option<usize>,
) -> Option<MirType> {
    if box_name == Some("StringBox") {
        let method_id = match arity {
            Some(arity) => crate::boxes::basic::StringMethodId::from_name_and_arity(method, arity),
            None => crate::boxes::basic::StringMethodId::from_name(method),
        };
        if let Some(method_id) = method_id {
            return Some(infer_string_method_return_type(method_id));
        }
    }

    if box_name == Some("ArrayBox") {
        let method_id = match arity {
            Some(arity) => crate::boxes::array::ArrayMethodId::from_name_and_arity(method, arity),
            None => crate::boxes::array::ArrayMethodId::from_name(method),
        };
        if let Some(method_id) = method_id {
            return infer_array_method_return_type(method_id);
        }
    }

    if box_name == Some("MapBox") {
        let method_id = match arity {
            Some(arity) => crate::boxes::MapMethodId::from_name_and_arity(method, arity),
            None => crate::boxes::MapMethodId::from_name(method),
        };
        if let Some(method_id) = method_id {
            return infer_map_method_return_type(method_id);
        }
    }

    if arity.is_some() {
        return None;
    }

    // Method-only compatibility rows that predate catalog-backed CoreBox rows.
    if method == "length" || method == "size" || method == "len" {
        return Some(MirType::Integer);
    }
    if method == "str" {
        return Some(MirType::String);
    }
    if method == "substring" || method == "substr" {
        return Some(MirType::String);
    }
    if method == "esc_json" {
        return Some(MirType::String);
    }
    if method == "indexOf" || method == "find" || method == "lastIndexOf" {
        return Some(MirType::Integer);
    }
    if method == "is_digit_char" || method == "is_hex_digit_char" || method == "is_alpha_char" {
        return Some(MirType::Bool);
    }
    if method == "has" && box_name == Some("MapBox") {
        return Some(MirType::Bool);
    }
    if method == "push" {
        return Some(MirType::Void);
    }

    None
}

fn infer_string_method_return_type(method: crate::boxes::basic::StringMethodId) -> MirType {
    match method {
        crate::boxes::basic::StringMethodId::Length
        | crate::boxes::basic::StringMethodId::IndexOf
        | crate::boxes::basic::StringMethodId::IndexOfFrom
        | crate::boxes::basic::StringMethodId::LastIndexOf => MirType::Integer,
        crate::boxes::basic::StringMethodId::Contains => MirType::Bool,
        crate::boxes::basic::StringMethodId::Substring
        | crate::boxes::basic::StringMethodId::Concat
        | crate::boxes::basic::StringMethodId::Replace
        | crate::boxes::basic::StringMethodId::Trim => MirType::String,
    }
}

fn infer_array_method_return_type(method: crate::boxes::array::ArrayMethodId) -> Option<MirType> {
    match method {
        crate::boxes::array::ArrayMethodId::Length => Some(MirType::Integer),
        crate::boxes::array::ArrayMethodId::Set
        | crate::boxes::array::ArrayMethodId::Push
        | crate::boxes::array::ArrayMethodId::Insert => Some(MirType::Void),
        crate::boxes::array::ArrayMethodId::Slice => Some(MirType::Box("ArrayBox".to_string())),
        crate::boxes::array::ArrayMethodId::Get
        | crate::boxes::array::ArrayMethodId::Pop
        | crate::boxes::array::ArrayMethodId::Remove => None,
    }
}

fn infer_map_method_return_type(method: crate::boxes::MapMethodId) -> Option<MirType> {
    match method {
        crate::boxes::MapMethodId::Size | crate::boxes::MapMethodId::Len => Some(MirType::Integer),
        crate::boxes::MapMethodId::Has => Some(MirType::Bool),
        crate::boxes::MapMethodId::Get
        | crate::boxes::MapMethodId::Set
        | crate::boxes::MapMethodId::Delete
        | crate::boxes::MapMethodId::Keys
        | crate::boxes::MapMethodId::Values
        | crate::boxes::MapMethodId::Clear => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn corebox_surface_aliases_use_catalog_return_type() {
        assert_eq!(
            infer_return_type("StringBox.substr/2"),
            Some(MirType::String)
        );
        assert_eq!(
            infer_return_type("StringBox.find/1"),
            Some(MirType::Integer)
        );
        assert_eq!(
            infer_return_type("StringBox.contains/1"),
            Some(MirType::Bool)
        );
        assert_eq!(infer_return_type("StringBox.substring/1"), None);
        assert_eq!(
            infer_return_type("ArrayBox.length/0"),
            Some(MirType::Integer)
        );
        assert_eq!(infer_return_type("ArrayBox.size/0"), Some(MirType::Integer));
        assert_eq!(infer_return_type("ArrayBox.len/0"), Some(MirType::Integer));
        assert_eq!(infer_return_type("ArrayBox.push/1"), Some(MirType::Void));
        assert_eq!(
            infer_return_type("ArrayBox.slice/2"),
            Some(MirType::Box("ArrayBox".to_string()))
        );
        assert_eq!(infer_return_type("ArrayBox.get/1"), None);
        assert_eq!(infer_return_type("MapBox.size/0"), Some(MirType::Integer));
        assert_eq!(infer_return_type("MapBox.len/0"), Some(MirType::Integer));
        assert_eq!(infer_return_type("MapBox.has/1"), Some(MirType::Bool));
        assert_eq!(infer_return_type("MapBox.get/1"), None);
    }
}
