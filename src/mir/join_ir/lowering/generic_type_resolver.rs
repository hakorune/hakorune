// Phase 66: P3-C ジェネリック型推論箱
//
// ArrayBox.get, MapBox.get などのジェネリック型を持つメソッドの
// 戻り値型を推論する。
//
// # 責務
//
// - P3-C 対象メソッドの判定
// - コンテナ使用文脈からの型推論（PHI 解析フォールバック）
// - 将来の型変数システム導入の土台
//
// # 設計原則（箱理論）
//
// - **単一責務**: P3-C ジェネリック型推論のみ
// - **質問箱**: is_generic_method() / resolve_generic_type() で判定
// - **if_phi.rs 依存削減**: infer_type_from_phi を内部に取り込み

use crate::mir::{MirFunction, MirInstruction, MirType, ValueId};
use std::collections::BTreeMap;

/// Phase 66: ジェネリック型推論箱
///
/// # 対象パターン（P3-C）
///
/// - `ArrayBox.get(index)` → 配列要素の型（T）
/// - `ArrayBox.pop()` → 配列要素の型（T）
/// - `MapBox.get(key)` → マップ値の型（V）
///
/// # Phase 65 との関係
///
/// Phase 65 で P1/P2/P3-A/P3-B は JoinIR 型ヒント経路に移行完了。
/// この箱は **P3-C 専用** として設計し、TypeHintPolicy と連携する。
///
/// # 将来拡張（Phase 67+）
///
/// - 型変数システム（`T`, `V` の明示的な型パラメータ）
/// - コンテナ生成時の型推論（`new ArrayBox<StringBox>()`）
pub struct GenericTypeResolver;

impl GenericTypeResolver {
    /// P3-C 対象メソッドかどうかを判定
    ///
    /// # 引数
    /// - `receiver_type`: 受け手の型
    /// - `method_name`: メソッド名
    ///
    /// # 戻り値
    /// - `true`: ジェネリック型推論が必要なメソッド
    /// - `false`: P3-A/P3-B で処理可能、または未対応
    pub fn is_generic_method(receiver_type: &MirType, method_name: &str) -> bool {
        match receiver_type {
            MirType::Array(_) => matches!(method_name, "get" | "pop" | "remove"),
            MirType::Box(box_name) => match box_name.as_str() {
                "ArrayBox" => matches!(method_name, "get" | "pop" | "remove" | "first" | "last"),
                "MapBox" => matches!(method_name, "get"),
                _ => false,
            },
            _ => false,
        }
    }

    /// P3-C 対象関数かどうかを判定（TypeHintPolicy 連携用）
    ///
    /// # 用途
    ///
    /// TypeHintPolicy.is_target() で P1/P2/P3-A/P3-B に該当しない場合、
    /// この関数で P3-C 候補かどうかを判定する。
    ///
    /// # 現在の実装
    ///
    /// ArrayBox/MapBox を使用する関数を P3-C 候補として判定。
    /// 関数名ベースのフィルタリングは行わない（汎用判定）。
    pub fn is_p3c_candidate(func_name: &str) -> bool {
        // Phase 66: 現在は全関数を P3-C 候補とみなす
        // 将来的に関数内の ArrayBox.get/MapBox.get 使用を解析して判定
        !func_name.is_empty() // 常に true（P1/P2/P3-A/B 以外は全て P3-C 候補）
    }

    /// PHI 解析によるジェネリック型推論
    ///
    /// # 責務
    ///
    /// P3-C メソッド（ArrayBox.get, MapBox.get など）の戻り値型を
    /// PHI 命令の incoming 値から推論する。
    ///
    /// # アルゴリズム
    ///
    /// 1. ret_val を定義する PHI 命令を探索
    /// 2. PHI の incoming 値の型を types マップから取得
    /// 3. 全ての incoming 値が同じ型なら、その型を返す
    ///
    /// # Phase 65 との違い
    ///
    /// - if_phi.rs::infer_type_from_phi() と同等のロジック
    /// - P3-C 専用として明示的に責務を限定
    /// - 将来的に型変数システムで置き換え予定
    pub fn resolve_from_phi(
        function: &MirFunction,
        ret_val: ValueId,
        types: &BTreeMap<ValueId, MirType>,
    ) -> Option<MirType> {
        for (_bid, bb) in function.blocks.iter() {
            for inst in bb.instructions.iter() {
                if let MirInstruction::Phi { dst, inputs, .. } = inst {
                    if *dst == ret_val {
                        let mut it = inputs.iter().filter_map(|(_, v)| types.get(v));
                        if let Some(first) = it.next() {
                            if it.all(|mt| mt == first) {
                                return Some(first.clone());
                            }
                        }
                    }
                }
            }
        }
        None
    }

    // NOTE: extract_type_hint() method was removed as dead code.
    // It was intended for Phase 67+ type variable system (ArrayBox<T>.get() type hints).
    // If needed in the future, reintroduce from git history.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_generic_method_arraybox() {
        let array_type = MirType::Box("ArrayBox".to_string());

        // P3-C 対象
        assert!(GenericTypeResolver::is_generic_method(&array_type, "get"));
        assert!(GenericTypeResolver::is_generic_method(&array_type, "pop"));
        assert!(GenericTypeResolver::is_generic_method(
            &array_type,
            "remove"
        ));
        assert!(GenericTypeResolver::is_generic_method(&array_type, "first"));
        assert!(GenericTypeResolver::is_generic_method(&array_type, "last"));
        assert!(GenericTypeResolver::is_generic_method(
            &MirType::Array(Box::new(MirType::Integer)),
            "remove"
        ));

        // P3-A/P3-B 対象（非 P3-C）
        assert!(!GenericTypeResolver::is_generic_method(&array_type, "size"));
        assert!(!GenericTypeResolver::is_generic_method(&array_type, "push"));
    }

    #[test]
    fn test_is_generic_method_mapbox() {
        let map_type = MirType::Box("MapBox".to_string());

        // P3-C 対象
        assert!(GenericTypeResolver::is_generic_method(&map_type, "get"));

        // P3-A/P3-B 対象（非 P3-C）
        assert!(!GenericTypeResolver::is_generic_method(&map_type, "size"));
        assert!(!GenericTypeResolver::is_generic_method(&map_type, "has"));
    }

    #[test]
    fn test_is_generic_method_stringbox() {
        // StringBox は P3-A で処理済み
        assert!(!GenericTypeResolver::is_generic_method(
            &MirType::String,
            "substring"
        ));
        assert!(!GenericTypeResolver::is_generic_method(
            &MirType::String,
            "length"
        ));
    }

    #[test]
    fn test_is_p3c_candidate() {
        // 全関数が P3-C 候補（P1/P2/P3-A/B 以外）
        assert!(GenericTypeResolver::is_p3c_candidate("Main.main/0"));
        assert!(GenericTypeResolver::is_p3c_candidate("FuncScanner.parse/1"));

        // 空文字列は false
        assert!(!GenericTypeResolver::is_p3c_candidate(""));
    }
}
