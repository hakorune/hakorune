/*!
 * StaticBoxRegistry - 静的Box管理の一元化箱
 *
 * 箱理論の実践:
 * - 箱にする: static_box_decls + static_boxes + MIR関数検出を1箱に集約
 * - 境界を作る: 静的Boxの存在証明・シングルトン管理を単一責務に
 * - Fail-Fast: 存在しないBoxへのアクセスは即座にエラー
 *
 * 設計原則:
 * - MIR関数名から自動検出: "BoxName.method/arity" パターンでusing importも対応
 * - 遅延シングルトン: 実際にアクセスされるまでインスタンス作成しない
 * - 明示的登録も可能: AST経由のBoxDeclarationも受け入れ
 */

use std::collections::{HashMap, HashSet};

use crate::config::env;
use crate::core::model::BoxDeclaration;
use crate::instance_v2::InstanceBox;
use crate::runtime::get_global_ring0;

/// 静的Box管理の一元化レジストリ
///
/// Phase 173-B で発生した問題を根本解決:
/// - using import された静的Boxが static_box_decls に登録されない問題
/// - MIR関数テーブルと static_box_decls の不整合問題
pub struct StaticBoxRegistry {
    /// 明示的に登録されたBoxDeclaration (AST経由)
    declarations: HashMap<String, BoxDeclaration>,

    /// MIR関数名から検出された静的Box名
    /// (using import でも自動検出可能)
    detected_boxes: HashSet<String>,

    /// 実行時シングルトン (遅延作成)
    instances: HashMap<String, InstanceBox>,
}

/// MIR関数名のパースユーティリティ
pub mod naming {
    /// "BoxName.method/arity" 形式の関数名をパース
    /// Returns: Some((box_name, method, arity)) or None
    pub fn parse_static_method_name(fn_name: &str) -> Option<(String, String, usize)> {
        // "BoxName.method/arity" or "BoxName.method"
        let dot_pos = fn_name.find('.')?;
        let box_name = &fn_name[..dot_pos];
        let rest = &fn_name[dot_pos + 1..];

        let (method, arity) = if let Some(slash_pos) = rest.find('/') {
            let method = &rest[..slash_pos];
            let arity_str = &rest[slash_pos + 1..];
            let arity = arity_str.parse::<usize>().ok()?;
            (method.to_string(), arity)
        } else {
            (rest.to_string(), 0)
        };

        Some((box_name.to_string(), method, arity))
    }

    /// 静的Boxメソッド名を生成
    pub fn static_method_name(box_name: &str, method: &str, arity: usize) -> String {
        format!("{}.{}/{}", box_name, method, arity)
    }

    /// 関数名からBox名を抽出 (メソッド・arity無視)
    pub fn extract_box_name(fn_name: &str) -> Option<String> {
        parse_static_method_name(fn_name).map(|(box_name, _, _)| box_name)
    }
}

/// 除外する組み込みBox名 (静的Boxではない)
const BUILTIN_RUNTIME_BOXES: &[&str] = &[
    "Main",
    "main",
    "StringBox",
    "IntegerBox",
    "BoolBox",
    "ArrayBox",
    "MapBox",
    "FloatBox",
    "VoidBox",
    "NullBox",
    "ConsoleBox",
    "MathBox",
    "FileBox",
    "NetBox",
    "CounterBox",
];

impl StaticBoxRegistry {
    /// 空のレジストリを作成
    pub fn new() -> Self {
        Self {
            declarations: HashMap::new(),
            detected_boxes: HashSet::new(),
            instances: HashMap::new(),
        }
    }

    /// MIR関数名一覧から静的Boxを自動検出
    ///
    /// using import された静的Boxも含めて検出可能
    pub fn detect_from_mir_functions<'a, I>(&mut self, fn_names: I)
    where
        I: Iterator<Item = &'a String>,
    {
        for fn_name in fn_names {
            if let Some(box_name) = naming::extract_box_name(fn_name) {
                // 組み込みランタイムBoxは除外
                if !BUILTIN_RUNTIME_BOXES.contains(&box_name.as_str()) {
                    self.detected_boxes.insert(box_name);
                }
            }
        }

        if env::env_bool("NYASH_STATIC_BOX_REGISTRY_TRACE") {
            get_global_ring0().log.debug(&format!(
                "[static-box-registry] detected {} static boxes from MIR functions",
                self.detected_boxes.len()
            ));
            for name in &self.detected_boxes {
                get_global_ring0()
                    .log
                    .debug(&format!("[static-box-registry] box={}", name));
            }
        }
    }

    /// BoxDeclarationを明示的に登録 (AST経由)
    pub fn register_declaration(&mut self, name: String, decl: BoxDeclaration) {
        self.declarations.insert(name, decl);
    }

    /// 静的Boxが存在するか確認
    ///
    /// 以下のいずれかで存在とみなす:
    /// 1. declarations に登録済み
    /// 2. detected_boxes に検出済み
    pub fn exists(&self, name: &str) -> bool {
        self.declarations.contains_key(name) || self.detected_boxes.contains(name)
    }

    /// シングルトンインスタンスを取得または作成
    ///
    /// 遅延作成: 初回アクセス時にのみ作成
    pub fn get_or_create_instance(&mut self, name: &str) -> Result<&mut InstanceBox, String> {
        if !self.exists(name) {
            return Err(format!(
                "static box '{}' not found in registry (neither declared nor detected)",
                name
            ));
        }

        // 既存インスタンスがあればそれを返す
        if self.instances.contains_key(name) {
            return Ok(self.instances.get_mut(name).unwrap());
        }

        // 新規作成
        let instance = if let Some(decl) = self.declarations.get(name) {
            // 明示的な宣言がある場合
            InstanceBox::from_declaration(
                name.to_string(),
                decl.fields.clone(),
                decl.methods.clone(),
            )
        } else {
            // MIR関数から検出された場合 (宣言なし)
            // 最小限のInstanceBoxを作成 (メソッドはMIR関数テーブルにある)
            InstanceBox::from_declaration(name.to_string(), vec![], HashMap::new())
        };

        if env::env_bool("NYASH_STATIC_BOX_REGISTRY_TRACE") {
            get_global_ring0().log.debug(&format!(
                "[static-box-registry] created singleton instance for '{}'",
                name
            ));
        }

        self.instances.insert(name.to_string(), instance);
        Ok(self.instances.get_mut(name).unwrap())
    }

    /// シングルトンインスタンスが既に作成済みか確認
    #[allow(dead_code)]
    pub fn has_instance(&self, name: &str) -> bool {
        self.instances.contains_key(name)
    }

    /// 登録済み/検出済みの静的Box名一覧
    pub fn all_box_names(&self) -> impl Iterator<Item = &String> {
        self.declarations.keys().chain(self.detected_boxes.iter())
    }

    /// declarations への直接アクセス (既存コードとの互換性)
    pub fn declarations(&self) -> &HashMap<String, BoxDeclaration> {
        &self.declarations
    }

    /// detected_boxes への直接アクセス
    #[allow(dead_code)]
    pub fn detected_boxes(&self) -> &HashSet<String> {
        &self.detected_boxes
    }
}

impl Default for StaticBoxRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_static_method_name() {
        assert_eq!(
            naming::parse_static_method_name("JsonParserBox.parse/1"),
            Some(("JsonParserBox".to_string(), "parse".to_string(), 1))
        );
        assert_eq!(
            naming::parse_static_method_name("MyBox.toString/0"),
            Some(("MyBox".to_string(), "toString".to_string(), 0))
        );
        assert_eq!(naming::parse_static_method_name("main"), None);
        assert_eq!(naming::parse_static_method_name("no_dot"), None);
    }

    #[test]
    fn test_static_method_name() {
        assert_eq!(
            naming::static_method_name("JsonParserBox", "parse", 1),
            "JsonParserBox.parse/1"
        );
    }

    #[test]
    fn test_detect_from_mir_functions() {
        let mut registry = StaticBoxRegistry::new();
        let fn_names = vec![
            "JsonParserBox.parse/1".to_string(),
            "JsonParserBox.toString/0".to_string(),
            "ProgramJSONBox.get/1".to_string(),
            "Main.main/0".to_string(),        // 除外される
            "StringBox.length/0".to_string(), // 除外される
        ];

        registry.detect_from_mir_functions(fn_names.iter());

        assert!(registry.exists("JsonParserBox"));
        assert!(registry.exists("ProgramJSONBox"));
        assert!(!registry.exists("Main")); // 除外
        assert!(!registry.exists("StringBox")); // 除外
    }

    #[test]
    fn test_get_or_create_instance() {
        let mut registry = StaticBoxRegistry::new();
        registry.detected_boxes.insert("TestBox".to_string());

        let result = registry.get_or_create_instance("TestBox");
        assert!(result.is_ok());

        // 2回目も同じインスタンスを返す
        assert!(registry.has_instance("TestBox"));
    }

    #[test]
    fn test_nonexistent_box_error() {
        let mut registry = StaticBoxRegistry::new();
        let result = registry.get_or_create_instance("NonExistent");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not found in registry"));
    }
}
