// Phase 65-2-A: JoinIR 型推論ユーティリティ
//
// NewBox 命令に対する型ヒント生成を提供する。
// P3-B（Box コンストラクタ）のみ対応。
// P3-C（ジェネリック型推論）は Phase 66+ に延期。

use crate::mir::MirType;

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

#[cfg(test)]
mod tests {
    use super::*;

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
