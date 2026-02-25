// Phase 65.5: JoinIR 型ヒント適用ポリシー
//
// lifecycle.rs から型ヒント判定ロジックを箱化・モジュール化。
// 単一責務：どの関数に型ヒントを適用するかのポリシー判定のみ。

use crate::mir::{MirFunction, MirInstruction, MirType, ValueId};

/// Phase 65.5: 型ヒント適用ポリシー
///
/// JoinIR 型ヒント経路（Route B）を使用する関数を判定する。
/// 箱理論：段階的拡大のため、関数名フィルタで制御。
///
/// # 対象パターン
///
/// - **P1**: If Select パターン（Phase 63-6）
/// - **P2**: If Merge パターン（Phase 64-3）
/// - **P3-A**: StringBox メソッド使用関数（Phase 65-2-A）
/// - **P3-B**: NewBox コンストラクタ使用関数（Phase 65-2-B）
/// - **P3-C**: ジェネリック型推論（Phase 66+ 将来拡張）
///
/// # 箱化の利点
///
/// - ✅ 単一責務：ポリシー判定のみ
/// - ✅ テスト可能：各 Phase 独立テスト
/// - ✅ 拡張容易：Phase 66+ で P3-C 追加が簡単
/// - ✅ lifecycle.rs 簡素化：60行削減
pub struct TypeHintPolicy;

impl TypeHintPolicy {
    /// P1/P2/P3-A/P3-B 型ヒント対象判定
    ///
    /// # 引数
    /// - `func_name`: 関数名（例: "IfSelectTest.simple_return/0"）
    ///
    /// # 戻り値
    /// - `true`: 型ヒント経路（Route B）を使用
    /// - `false`: 従来ロジック（Route A）にフォールバック
    pub fn is_target(func_name: &str) -> bool {
        Self::is_p1_target(func_name)
            || Self::is_p2_target(func_name)
            || Self::is_p3a_target(func_name)
            || Self::is_p3b_target(func_name)
    }

    /// P1: If Select パターン判定
    ///
    /// # 対象
    /// - `IfSelectTest.*` - If Select パターンのテスト関数
    ///
    /// # Phase
    /// - Phase 63-6 で導入
    fn is_p1_target(func_name: &str) -> bool {
        func_name.starts_with("IfSelectTest.")
    }

    /// P2: If Merge パターン判定
    ///
    /// # 対象
    /// - `IfMergeTest.*` - If Merge パターンのテスト関数
    /// - `read_quoted*` - selfhost の read_quoted 系関数
    ///
    /// # Phase
    /// - Phase 64-3 で導入
    fn is_p2_target(func_name: &str) -> bool {
        func_name.starts_with("IfMergeTest.")
    }

    /// P3-A: StringBox メソッド使用関数判定
    ///
    /// # 対象
    /// - `read_quoted*` - StringBox メソッド（substring/length）使用
    ///
    /// # Phase
    /// - Phase 65-2-A で導入
    /// - P2 と重複（read_quoted 系関数）
    fn is_p3a_target(func_name: &str) -> bool {
        func_name.contains("read_quoted")
    }

    /// P3-B: NewBox コンストラクタ使用関数判定
    ///
    /// # 対象
    /// - `NewBoxTest.*` - NewBox コンストラクタテスト関数
    ///
    /// # Phase
    /// - Phase 65-2-B で導入
    fn is_p3b_target(func_name: &str) -> bool {
        func_name.starts_with("NewBoxTest.")
    }

    /// P3-C: ジェネリック型推論対象判定（Phase 66）
    ///
    /// # 対象
    /// - P1/P2/P3-A/P3-B 以外のすべての関数
    /// - ArrayBox.get, MapBox.get などを使用する可能性がある関数
    ///
    /// # Phase 66 設計
    /// - GenericTypeResolver と連携して P3-C 型推論を実行
    /// - is_target() が false の場合のフォールバック経路
    pub fn is_p3c_target(func_name: &str) -> bool {
        // P1/P2/P3-A/P3-B に該当しない場合は P3-C 候補
        !Self::is_p1_target(func_name)
            && !Self::is_p2_target(func_name)
            && !Self::is_p3a_target(func_name)
            && !Self::is_p3b_target(func_name)
            && !func_name.is_empty()
    }

    /// PHI 命令から型ヒントを抽出
    ///
    /// # 引数
    /// - `function`: MIR 関数
    /// - `ret_val`: 戻り値の ValueId
    ///
    /// # 戻り値
    /// - `Some(MirType)`: PHI の type_hint が存在する場合
    /// - `None`: type_hint が存在しない、または PHI が見つからない場合
    ///
    /// # Phase
    /// - Phase 63-6 で導入
    /// - Phase 65.5 で箱化・関数型スタイルに改善
    pub fn extract_phi_type_hint(function: &MirFunction, ret_val: ValueId) -> Option<MirType> {
        function
            .blocks
            .values()
            .flat_map(|bb| &bb.instructions)
            .find_map(|inst| {
                if let MirInstruction::Phi { dst, type_hint, .. } = inst {
                    if *dst == ret_val {
                        return type_hint.clone();
                    }
                }
                None
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Phase 65.5: P1 パターン判定テスト
    #[test]
    fn test_is_p1_target() {
        // P1: If Select パターン
        assert!(TypeHintPolicy::is_p1_target("IfSelectTest.simple_return/0"));
        assert!(TypeHintPolicy::is_p1_target("IfSelectTest.complex/2"));

        // P1 以外
        assert!(!TypeHintPolicy::is_p1_target("IfMergeTest.simple/0"));
        assert!(!TypeHintPolicy::is_p1_target("read_quoted_from/1"));
        assert!(!TypeHintPolicy::is_p1_target("NewBoxTest.array/0"));
    }

    /// Phase 65.5: P2 パターン判定テスト
    #[test]
    fn test_is_p2_target() {
        // P2: If Merge パターン
        assert!(TypeHintPolicy::is_p2_target("IfMergeTest.simple/0"));
        assert!(TypeHintPolicy::is_p2_target("IfMergeTest.multiple/0"));

        // P2 以外
        assert!(!TypeHintPolicy::is_p2_target("IfSelectTest.simple/0"));
        assert!(!TypeHintPolicy::is_p2_target("NewBoxTest.array/0"));
    }

    /// Phase 65.5: P3-A パターン判定テスト
    #[test]
    fn test_is_p3a_target() {
        // P3-A: read_quoted 系関数
        assert!(TypeHintPolicy::is_p3a_target("read_quoted_from/1"));
        assert!(TypeHintPolicy::is_p3a_target(
            "FuncScannerBox.read_quoted/1"
        ));

        // P3-A 以外
        assert!(!TypeHintPolicy::is_p3a_target("IfSelectTest.simple/0"));
        assert!(!TypeHintPolicy::is_p3a_target("NewBoxTest.array/0"));
    }

    /// Phase 65.5: P3-B パターン判定テスト
    #[test]
    fn test_is_p3b_target() {
        // P3-B: NewBox テスト関数
        assert!(TypeHintPolicy::is_p3b_target("NewBoxTest.array/0"));
        assert!(TypeHintPolicy::is_p3b_target("NewBoxTest.string/0"));

        // P3-B 以外
        assert!(!TypeHintPolicy::is_p3b_target("IfSelectTest.simple/0"));
        assert!(!TypeHintPolicy::is_p3b_target("read_quoted_from/1"));
    }

    /// Phase 65.5: 統合判定テスト
    #[test]
    fn test_is_target() {
        // P1
        assert!(TypeHintPolicy::is_target("IfSelectTest.simple/0"));
        // P2
        assert!(TypeHintPolicy::is_target("IfMergeTest.simple/0"));
        // P3-A
        assert!(TypeHintPolicy::is_target("read_quoted_from/1"));
        // P3-B
        assert!(TypeHintPolicy::is_target("NewBoxTest.array/0"));

        // P1/P2/P3-A/P3-B 以外
        assert!(!TypeHintPolicy::is_target("SomeBox.some_method/3"));
        assert!(!TypeHintPolicy::is_target("Main.main/0"));
    }

    /// Phase 65.5: P3-A と P2 の重複確認
    #[test]
    fn test_p2_p3a_overlap() {
        // read_quoted は P2 と P3-A の両方に該当
        assert!(TypeHintPolicy::is_p2_target("IfMergeTest.read_quoted/0"));
        assert!(TypeHintPolicy::is_p3a_target("read_quoted_from/1"));

        // どちらかに該当すれば is_target() は true
        assert!(TypeHintPolicy::is_target("read_quoted_from/1"));
    }

    /// Phase 66: P3-C パターン判定テスト
    #[test]
    fn test_is_p3c_target() {
        // P3-C: P1/P2/P3-A/P3-B 以外
        assert!(TypeHintPolicy::is_p3c_target("Main.main/0"));
        assert!(TypeHintPolicy::is_p3c_target("SomeBox.some_method/3"));
        assert!(TypeHintPolicy::is_p3c_target("ArrayProcessor.process/1"));

        // P1/P2/P3-A/P3-B は P3-C ではない
        assert!(!TypeHintPolicy::is_p3c_target("IfSelectTest.simple/0")); // P1
        assert!(!TypeHintPolicy::is_p3c_target("IfMergeTest.simple/0")); // P2
        assert!(!TypeHintPolicy::is_p3c_target("read_quoted_from/1")); // P3-A
        assert!(!TypeHintPolicy::is_p3c_target("NewBoxTest.array/0")); // P3-B

        // 空文字列は false
        assert!(!TypeHintPolicy::is_p3c_target(""));
    }

    /// Phase 66: is_target と is_p3c_target の排他性確認
    #[test]
    fn test_is_target_and_p3c_mutually_exclusive() {
        // is_target() が true なら is_p3c_target() は false
        let p1_func = "IfSelectTest.simple/0";
        assert!(TypeHintPolicy::is_target(p1_func));
        assert!(!TypeHintPolicy::is_p3c_target(p1_func));

        // is_target() が false なら is_p3c_target() は true
        let general_func = "Main.main/0";
        assert!(!TypeHintPolicy::is_target(general_func));
        assert!(TypeHintPolicy::is_p3c_target(general_func));
    }
}
