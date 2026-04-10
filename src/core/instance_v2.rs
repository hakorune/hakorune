/*!
 * Nyash Instance System v2 - Simplified Box Instance Implementation
 *
 * 🎯 Phase 9.78d: 簡素化InstanceBox統一実装
 * Everything is Box哲学に基づく統一オブジェクト指向システム
 *
 * 🔄 設計方針: trait objectによる完全統一
 * - すべてのBox型を同じように扱う
 * - Option<T>による柔軟性
 * - レガシー負債の完全削除
 */

use crate::ast::ASTNode;
use crate::box_trait::{BoolBox, BoxBase, BoxCore, NyashBox, SharedNyashBox, StringBox};
use crate::value::NyashValue;
use std::any::Any;
use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::sync::{Arc, Mutex};

/// 🎯 簡素化InstanceBox - すべてのBox型を統一管理
#[derive(Debug)]
pub struct InstanceBox {
    /// クラス名（StringBox, MyUserBox等統一）
    pub class_name: String,

    /// 統一フィールド管理（レガシーfields削除）
    pub fields_ng: Arc<Mutex<HashMap<String, NyashValue>>>,

    /// メソッド定義（ユーザー定義時のみ使用、ビルトインは空）
    pub methods: Arc<HashMap<String, ASTNode>>,

    /// 🏭 統一内容 - すべてのBox型を同じように扱う
    pub inner_content: Option<Box<dyn NyashBox>>,

    /// Box基底 + ライフサイクル管理
    base: BoxBase,
    finalized: Arc<Mutex<bool>>,

    /// Shared box-valued fields kept outside `fields_ng` so object identity stays stable.
    pub box_fields: Arc<Mutex<HashMap<String, SharedNyashBox>>>,
    init_field_order: Vec<String>,
    weak_fields_union: std::collections::HashSet<String>,
    in_finalization: Arc<Mutex<bool>>,
}

impl Clone for InstanceBox {
    fn clone(&self) -> Self {
        Self {
            class_name: self.class_name.clone(),
            fields_ng: Arc::clone(&self.fields_ng), // Shared reference
            methods: Arc::clone(&self.methods),
            inner_content: None,  // inner_content cannot be cloned (Box<dyn>)
            base: BoxBase::new(), // Fresh base for clone
            finalized: Arc::clone(&self.finalized),
            box_fields: Arc::clone(&self.box_fields),
            init_field_order: self.init_field_order.clone(),
            weak_fields_union: self.weak_fields_union.clone(),
            in_finalization: Arc::clone(&self.in_finalization),
        }
    }
}

impl InstanceBox {
    /// 🎯 統一コンストラクタ - すべてのBox型対応
    pub fn from_any_box(class_name: String, inner: Box<dyn NyashBox>) -> Self {
        Self {
            class_name,
            fields_ng: Arc::new(Mutex::new(HashMap::new())),
            methods: Arc::new(HashMap::new()), // ビルトインは空、ユーザー定義時は設定
            inner_content: Some(inner),        // 統一内包
            base: BoxBase::new(),
            finalized: Arc::new(Mutex::new(false)),
            box_fields: Arc::new(Mutex::new(HashMap::new())),
            init_field_order: Vec::new(),
            weak_fields_union: std::collections::HashSet::new(),
            in_finalization: Arc::new(Mutex::new(false)),
        }
    }

    /// ユーザー定義Box専用コンストラクタ
    pub fn from_declaration(
        class_name: String,
        fields: Vec<String>,
        methods: HashMap<String, ASTNode>,
    ) -> Self {
        // Invalidate caches for this class since methods layout may change between runs
        crate::runtime::cache_versions::bump_version(&format!("BoxRef:{}", class_name));
        let mut field_map = HashMap::new();
        // Value fields are initialized in `fields_ng`; box-valued fields populate
        // `box_fields` only when a real handle is assigned.
        for field in &fields {
            field_map.insert(field.clone(), NyashValue::Null);
        }

        Self {
            class_name,
            fields_ng: Arc::new(Mutex::new(field_map)),
            methods: Arc::new(methods),
            inner_content: None, // ユーザー定義は内包Boxなし
            base: BoxBase::new(),
            finalized: Arc::new(Mutex::new(false)),
            box_fields: Arc::new(Mutex::new(HashMap::new())),
            init_field_order: fields,
            weak_fields_union: std::collections::HashSet::new(),
            in_finalization: Arc::new(Mutex::new(false)),
        }
    }

    /// 🔄 レガシー互換性メソッド - 段階移行用
    pub fn new(class_name: String, fields: Vec<String>, methods: HashMap<String, ASTNode>) -> Self {
        Self::from_declaration(class_name, fields, methods)
    }

    /// 🔄 レガシー互換性 - 高度なfiniシステムを簡素化して対応
    pub fn new_with_box_info(
        class_name: String,
        fields: Vec<String>,
        methods: HashMap<String, ASTNode>,
        init_field_order: Vec<String>,
        weak_fields: Vec<String>,
    ) -> Self {
        let mut instance = Self::from_declaration(class_name, fields, methods);
        // レガシー互換：init順序とweak fieldsを設定
        instance.init_field_order = init_field_order;
        instance.weak_fields_union = weak_fields.into_iter().collect();
        instance
    }

    /// 🎯 統一フィールドアクセス（NyashValue版）
    pub fn get_field_ng(&self, field_name: &str) -> Option<NyashValue> {
        self.fields_ng.lock().unwrap().get(field_name).cloned()
    }

    /// 🎯 統一フィールド設定（NyashValue版）
    pub fn set_field_ng(&self, field_name: String, value: NyashValue) -> Result<(), String> {
        self.box_fields.lock().unwrap().remove(&field_name);
        self.fields_ng.lock().unwrap().insert(field_name, value);
        Ok(())
    }

    /// 動的フィールド追加（GlobalBox用）
    pub fn set_field_dynamic(&self, field_name: String, value: NyashValue) {
        self.box_fields.lock().unwrap().remove(&field_name);
        self.fields_ng.lock().unwrap().insert(field_name, value);
    }

    /// メソッド定義を取得
    pub fn get_method(&self, method_name: &str) -> Option<&ASTNode> {
        self.methods.get(method_name)
    }

    /// メソッドが存在するかチェック
    pub fn has_method(&self, method_name: &str) -> bool {
        self.methods.contains_key(method_name)
    }

    /// メソッド動的追加（GlobalBox用）
    pub fn add_method(&mut self, method_name: String, method_ast: ASTNode) -> Result<(), String> {
        let mut new_methods = (*self.methods).clone();
        new_methods.insert(method_name, method_ast);
        self.methods = Arc::new(new_methods);
        Ok(())
    }

    /// 🎯 統一初期化処理
    pub fn init(&mut self, _args: &[Box<dyn NyashBox>]) -> Result<(), String> {
        match &self.inner_content {
            Some(_) => Ok(()), // ビルトイン・プラグインは初期化済み
            None => {
                // ユーザー定義のinit実行（インタープリター側で実装）
                // TODO: インタープリター統合時に実装
                Ok(())
            }
        }
    }

    /// 🎯 統一解放処理
    pub fn fini(&self) -> Result<(), String> {
        let mut finalized = self.finalized.lock().unwrap();
        if *finalized {
            return Ok(()); // 既に解放済み
        }

        // フィールドクリア
        self.fields_ng.lock().unwrap().clear();

        *finalized = true;
        if crate::config::env::cli_verbose_enabled() {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!(
                "🎯 fini(): Instance {} (ID: {}) finalized",
                self.class_name, self.base.id
            ));
        }
        Ok(())
    }

    /// 解放済みかチェック
    pub fn is_finalized(&self) -> bool {
        *self.finalized.lock().unwrap()
    }

    pub fn field_names(&self) -> Vec<String> {
        let mut name_set = std::collections::BTreeSet::new();
        name_set.extend(self.fields_ng.lock().unwrap().keys().cloned());
        name_set.extend(self.box_fields.lock().unwrap().keys().cloned());
        let mut names: Vec<_> = name_set.into_iter().collect();
        names.sort();
        names
    }

    pub fn field_count(&self) -> usize {
        self.field_names().len()
    }

    /// レガシー互換：get_field（SharedNyashBoxを返す）
    pub fn get_field(&self, field_name: &str) -> Option<SharedNyashBox> {
        if let Some(value) = self.box_fields.lock().unwrap().get(field_name) {
            return Some(Arc::clone(value));
        }

        let nyash_value = self.get_field_ng(field_name)?;
        let boxed = nyash_value.to_box().ok()?;
        let guard = boxed.lock().ok()?;
        Some(Arc::from(guard.clone_box()))
    }

    /// レガシー互換：set_field（SharedNyashBoxを受け取る）
    pub fn set_field(&self, field_name: &str, value: SharedNyashBox) -> Result<(), String> {
        self.fields_ng.lock().unwrap().remove(field_name);
        self.box_fields
            .lock()
            .unwrap()
            .insert(field_name.to_string(), value);
        Ok(())
    }
}

/// 🎯 統一NyashBoxトレイト実装
impl NyashBox for InstanceBox {
    fn to_string_box(&self) -> StringBox {
        StringBox::new(format!("<{} instance #{}>", self.class_name, self.base.id))
    }

    fn equals(&self, other: &dyn NyashBox) -> BoolBox {
        if let Some(other_instance) = other.as_any().downcast_ref::<InstanceBox>() {
            BoolBox::new(self.base.id == other_instance.base.id)
        } else {
            BoolBox::new(false)
        }
    }

    fn type_name(&self) -> &'static str {
        // 内包Boxがあれば、その型名を返す（ビルトインBox用）
        if let Some(inner) = &self.inner_content {
            inner.type_name()
        } else {
            // ユーザー定義Boxの場合はclass_nameを使用したいが、
            // &'static strを要求されているので一時的に"InstanceBox"を返す
            // TODO: type_nameの戻り値型をStringに変更することを検討
            "InstanceBox"
        }
    }

    fn clone_box(&self) -> Box<dyn NyashBox> {
        Box::new(InstanceBox {
            class_name: self.class_name.clone(),
            fields_ng: Arc::clone(&self.fields_ng),
            methods: Arc::clone(&self.methods),
            inner_content: self.inner_content.as_ref().map(|inner| inner.clone_box()),
            base: self.base.clone(),
            finalized: Arc::clone(&self.finalized),
            box_fields: Arc::clone(&self.box_fields),
            init_field_order: self.init_field_order.clone(),
            weak_fields_union: self.weak_fields_union.clone(),
            in_finalization: Arc::clone(&self.in_finalization),
        })
    }

    fn share_box(&self) -> Box<dyn NyashBox> {
        // TODO: 正しいshare_boxセマンティクス実装
        self.clone_box()
    }
}

impl BoxCore for InstanceBox {
    fn box_id(&self) -> u64 {
        self.base.id
    }

    fn parent_type_id(&self) -> Option<std::any::TypeId> {
        self.base.parent_type_id
    }

    fn fmt_box(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{} instance #{}>", self.class_name, self.base.id)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl Display for InstanceBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt_box(f)
    }
}

// ===== Tests =====

#[cfg(test)]
mod tests {
    use super::*;
    use crate::box_trait::{IntegerBox, NyashBox};
    use std::sync::Arc;
    type SharedNyashBox = Arc<dyn NyashBox>;

    #[test]
    fn test_from_any_box_creation() {
        let string_box = Box::new(crate::box_trait::StringBox::new("hello"));
        let instance = InstanceBox::from_any_box("StringBox".to_string(), string_box);

        assert_eq!(instance.class_name, "StringBox");
        assert!(instance.inner_content.is_some());
        assert!(instance.methods.is_empty()); // ビルトインは空
    }

    // InstanceBox creation test（declared fields remain observable through the unified stores）.
    #[test]
    fn test_from_declaration_creation() {
        let fields = vec!["x".to_string(), "y".to_string()];
        let methods = HashMap::new();
        let instance = InstanceBox::from_declaration("Point".to_string(), fields, methods);

        assert_eq!(instance.class_name, "Point");
        assert!(instance.inner_content.is_none()); // ユーザー定義は内包なし
                                                   // フィールドが初期化されているかチェック
        assert!(instance.get_field("x").is_some());
        assert!(instance.get_field("y").is_some());
    }

    // Box-valued field test（identity-preserving path）.
    #[test]
    fn test_field_operations() {
        let instance = InstanceBox::from_declaration(
            "TestBox".to_string(),
            vec!["value".to_string()],
            HashMap::new(),
        );

        // フィールド設定
        let int_box: SharedNyashBox = Arc::new(IntegerBox::new(42));
        instance.set_field("value", int_box.clone()).unwrap();

        // フィールド取得
        let field_value = instance.get_field("value").unwrap();
        if let Some(int_box) = field_value.as_any().downcast_ref::<IntegerBox>() {
            assert_eq!(int_box.value, 42);
        } else {
            panic!("Expected IntegerBox");
        }
    }

    #[test]
    fn test_box_field_names_join_declared_and_box_only_fields() {
        let instance = InstanceBox::from_declaration(
            "TestBox".to_string(),
            vec!["declared".to_string()],
            HashMap::new(),
        );
        let refcell: SharedNyashBox = Arc::from(
            Box::new(crate::boxes::ref_cell_box::RefCellBox::new(Box::new(
                IntegerBox::new(9),
            ))) as Box<dyn NyashBox>,
        );
        instance.set_field("payload", refcell).expect("box field set");

        assert_eq!(
            instance.field_names(),
            vec!["declared".to_string(), "payload".to_string()]
        );
        assert_eq!(instance.field_count(), 2);
    }

    #[test]
    fn test_unified_approach() {
        // ビルトインBox
        let string_instance = InstanceBox::from_any_box(
            "StringBox".to_string(),
            Box::new(crate::box_trait::StringBox::new("test")),
        );

        // ユーザー定義Box
        let user_instance = InstanceBox::from_declaration(
            "MyBox".to_string(),
            vec!["field1".to_string()],
            HashMap::new(),
        );

        // どちらも同じ型として扱える！
        let instances: Vec<InstanceBox> = vec![string_instance, user_instance];

        for instance in instances {
            if crate::config::env::cli_verbose_enabled() {
                let ring0 = crate::runtime::get_global_ring0();
                ring0
                    .log
                    .debug(&format!("Instance: {}", instance.class_name));
            }
            // すべて Box<dyn NyashBox> として統一処理可能
            let _box_ref: &dyn NyashBox = &instance;
        }
    }
}
