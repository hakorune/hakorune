// Phase 65-2-A: JoinIR 型推論ユーティリティ
//
// MethodCall / NewBox 命令に対する型ヒント生成を提供する。
// P3-A（StringBox メソッド）と P3-B（Box コンストラクタ）のみ対応。
// P3-C（ジェネリック型推論）は Phase 66+ に延期。

use crate::mir::MirType;
use crate::runtime::core_box_ids::{CoreBoxId, CoreMethodId};
use crate::runtime::core_method_aliases::canonical_method_name;

/// Phase 65-2-A: MethodCall 戻り値型推論
///
/// 受け手 Box 名とメソッド名から戻り値型を推論する。
/// P3-A（StringBox, ArrayBox の基本メソッド）のみ対応。
/// P3-C（ジェネリック型）は Phase 66+ で対応。
///
/// # 引数
/// - `receiver_type`: 受け手の型（MirType::String, MirType::Box("ArrayBox") など）
/// - `method_name`: メソッド名（"substring", "length" など）
///
/// # 戻り値
/// - `Some(MirType)`: 推論成功時の戻り値型
/// - `None`: 推論失敗（未知のメソッド、P3-C 対象など）
pub fn infer_method_return_type(receiver_type: &MirType, method_name: &str) -> Option<MirType> {
    let canonical = canonical_method_name(method_name);
    if let Some(core_box_id) = core_box_id_for_receiver(receiver_type) {
        if let Some(method_id) = CoreMethodId::iter()
            .find(|method_id| method_id.box_id() == core_box_id && method_id.name() == canonical)
        {
            return mir_type_from_return_name(method_id.return_type_name());
        }
    }

    // Phase 65-2-A: non-core (legacy) method hints
    match receiver_type {
        MirType::String => match canonical {
            "charAt" => Some(MirType::String),
            _ => None,
        },
        MirType::Box(box_name) => match box_name.as_str() {
            "ArrayBox" => match canonical {
                "size" => Some(MirType::Integer),
                "push" => Some(MirType::Void),
                _ => None,
            },
            "MapBox" => crate::boxes::MapMethodId::from_name(method_name).and_then(|method_id| {
                match method_id {
                    crate::boxes::MapMethodId::Size | crate::boxes::MapMethodId::Len => {
                        Some(MirType::Integer)
                    }
                    crate::boxes::MapMethodId::Has => Some(MirType::Bool),
                    crate::boxes::MapMethodId::Get
                    | crate::boxes::MapMethodId::Set
                    | crate::boxes::MapMethodId::Delete
                    | crate::boxes::MapMethodId::Keys
                    | crate::boxes::MapMethodId::Clear => None,
                    crate::boxes::MapMethodId::Values => {
                        Some(MirType::Box("ArrayBox".to_string()))
                    }
                }
            }),
            _ => None,
        },
        _ => None,
    }
}

/// Phase 65-2-B: Box コンストラクタ型推論
///
/// Box 名から生成される Box の型を推論する。
/// P3-B（基本 Box）のみ対応。
///
/// # 引数
/// - `box_name`: Box 名（"ArrayBox", "MapBox", "StringBox" など）
///
/// # 戻り値
/// - `Some(MirType)`: 推論成功時の生成型
/// - `None`: 推論失敗（未知の Box）
pub fn infer_box_type(box_name: &str) -> Option<MirType> {
    match box_name {
        // ビルトイン型（primitive Box）
        "StringBox" => Some(MirType::String),
        "IntegerBox" => Some(MirType::Integer),
        "BoolBox" => Some(MirType::Bool),

        // コレクション型
        "ArrayBox" => Some(MirType::Box("ArrayBox".to_string())),
        "MapBox" => Some(MirType::Box("MapBox".to_string())),

        // プラグイン Box（代表例）
        "ConsoleBox" => Some(MirType::Box("ConsoleBox".to_string())),
        "FileBox" => Some(MirType::Box("FileBox".to_string())),

        // その他の Box は Unknown（Phase 66+ で拡張可能）
        _ => None,
    }
}

fn core_box_id_for_receiver(receiver_type: &MirType) -> Option<CoreBoxId> {
    match receiver_type {
        MirType::String => Some(CoreBoxId::String),
        MirType::Integer => Some(CoreBoxId::Integer),
        MirType::Bool => Some(CoreBoxId::Bool),
        MirType::Box(box_name) => CoreBoxId::from_name(box_name),
        _ => None,
    }
}

fn mir_type_from_return_name(return_name: &str) -> Option<MirType> {
    match return_name {
        "IntegerBox" => Some(MirType::Integer),
        "StringBox" => Some(MirType::String),
        "BoolBox" => Some(MirType::Bool),
        "Void" => Some(MirType::Void),
        "Unknown" => None,
        other if other.ends_with("Box") => Some(MirType::Box(other.to_string())),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_infer_method_return_type_string_methods() {
        // StringBox メソッド
        assert_eq!(
            infer_method_return_type(&MirType::String, "substring"),
            Some(MirType::String)
        );
        assert_eq!(
            infer_method_return_type(&MirType::String, "length"),
            Some(MirType::Integer)
        );
        assert_eq!(
            infer_method_return_type(&MirType::String, "charAt"),
            Some(MirType::String)
        );
        assert_eq!(
            infer_method_return_type(&MirType::String, "indexOf"),
            Some(MirType::Integer)
        );
    }

    #[test]
    fn test_infer_method_return_type_arraybox_methods() {
        let array_type = MirType::Box("ArrayBox".to_string());
        assert_eq!(
            infer_method_return_type(&array_type, "size"),
            Some(MirType::Integer)
        );
        assert_eq!(
            infer_method_return_type(&array_type, "push"),
            Some(MirType::Void)
        );
        // P3-C: get は Phase 66+
        assert_eq!(infer_method_return_type(&array_type, "get"), None);
    }

    #[test]
    fn test_infer_method_return_type_mapbox_methods() {
        let map_type = MirType::Box("MapBox".to_string());
        assert_eq!(
            infer_method_return_type(&map_type, "size"),
            Some(MirType::Integer)
        );
        assert_eq!(
            infer_method_return_type(&map_type, "length"),
            Some(MirType::Integer)
        );
        assert_eq!(
            infer_method_return_type(&map_type, "has"),
            Some(MirType::Bool)
        );
        // P3-C: get は Phase 66+
        assert_eq!(infer_method_return_type(&map_type, "get"), None);
    }

    #[test]
    fn test_infer_box_type_builtin_boxes() {
        assert_eq!(infer_box_type("StringBox"), Some(MirType::String));
        assert_eq!(infer_box_type("IntegerBox"), Some(MirType::Integer));
        assert_eq!(infer_box_type("BoolBox"), Some(MirType::Bool));
    }

    #[test]
    fn test_infer_box_type_collection_boxes() {
        assert_eq!(
            infer_box_type("ArrayBox"),
            Some(MirType::Box("ArrayBox".to_string()))
        );
        assert_eq!(
            infer_box_type("MapBox"),
            Some(MirType::Box("MapBox".to_string()))
        );
    }

    #[test]
    fn test_infer_box_type_plugin_boxes() {
        assert_eq!(
            infer_box_type("ConsoleBox"),
            Some(MirType::Box("ConsoleBox".to_string()))
        );
        assert_eq!(
            infer_box_type("FileBox"),
            Some(MirType::Box("FileBox".to_string()))
        );
    }

    #[test]
    fn test_infer_box_type_unknown() {
        assert_eq!(infer_box_type("UnknownBox"), None);
        assert_eq!(infer_box_type("CustomUserBox"), None);
    }
}
