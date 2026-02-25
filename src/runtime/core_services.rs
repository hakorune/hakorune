//! Phase 91: CoreServices 定義
//!
//! Ring1-Core: core_required Box の Service trait 群。
//! Phase 87 CoreBoxId の core_required (6個) 全てをカバー。

use crate::box_trait::NyashBox;
use crate::runtime::CoreBoxId;
use std::sync::Arc;

use crate::providers::ring1::array::Ring1ArrayService;
use crate::providers::ring1::map::Ring1MapService;

/// StringBox Service trait
///
/// Phase 95: len のみ実装
pub trait StringService: Send + Sync {
    fn len(&self, s: &str) -> i64;
}

/// IntegerBox Service trait
///
/// Phase 97: 純粋関数型（加算・減算・比較など）
pub trait IntegerService: Send + Sync {
    /// 2つの整数を加算
    fn add(&self, a: i64, b: i64) -> i64;

    /// 2つの整数を減算
    fn sub(&self, a: i64, b: i64) -> i64;

    /// 2つの整数を乗算
    fn mul(&self, a: i64, b: i64) -> i64;

    /// 2つの整数を除算（ゼロ除算はNone）
    fn div(&self, a: i64, b: i64) -> Option<i64>;
}

/// BoolBox Service trait
///
/// Phase 97: 純粋関数型（論理演算）
pub trait BoolService: Send + Sync {
    /// 論理NOT
    fn not(&self, value: bool) -> bool;

    /// 論理AND
    fn and(&self, a: bool, b: bool) -> bool;

    /// 論理OR
    fn or(&self, a: bool, b: bool) -> bool;

    /// 論理XOR
    fn xor(&self, a: bool, b: bool) -> bool;
}

/// ArrayBox Service trait
///
/// Phase 96: len/get/set/push 実装
pub trait ArrayService: Send + Sync {
    /// 配列の要素数を取得
    fn len(&self, arr: &dyn NyashBox) -> i64;

    /// 指定インデックスの要素を取得
    fn get(&self, arr: &dyn NyashBox, index: i64) -> Option<Box<dyn NyashBox>>;

    /// 指定インデックスに要素を設定
    fn set(&self, arr: &dyn NyashBox, index: i64, value: Box<dyn NyashBox>) -> Result<(), String>;

    /// 配列の末尾に要素を追加
    fn push(&self, arr: &dyn NyashBox, value: Box<dyn NyashBox>) -> Result<(), String>;
}

/// MapBox Service trait
///
/// Phase 96: size/has/get/set 実装
pub trait MapService: Send + Sync {
    /// マップのサイズを取得
    fn size(&self, map: &dyn NyashBox) -> i64;

    /// キーが存在するか確認
    fn has(&self, map: &dyn NyashBox, key: &str) -> bool;

    /// 値を取得
    fn get(&self, map: &dyn NyashBox, key: &str) -> Option<Box<dyn NyashBox>>;

    /// 値を設定
    fn set(&self, map: &dyn NyashBox, key: &str, value: Box<dyn NyashBox>) -> Result<(), String>;
}

/// ConsoleBox Service trait
///
/// Phase 95: println と print のみ実装
pub trait ConsoleService: Send + Sync {
    fn println(&self, msg: &str);
    fn print(&self, msg: &str);
}

/// CoreServices: core_required Box の集合
///
/// Phase 85 設計原則:
/// - core_required は必ず全て揃っていなければならない
/// - 起動時に全フィールドが初期化されていることを保証
///
/// Phase 87 CoreBoxId との対応:
/// - String → string
/// - Integer → integer
/// - Bool → bool
/// - Array → array
/// - Map → map
/// - Console → console
///
/// Phase 103: Optional化対応
/// - 各サービスは Option<Arc<dyn XyzService>> に変更
/// - ConsoleBox は必須（Graceful Degradation原則）
pub struct CoreServices {
    pub string: Option<Arc<dyn StringService>>,
    pub integer: Option<Arc<dyn IntegerService>>,
    pub bool: Option<Arc<dyn BoolService>>,
    pub array: Option<Arc<dyn ArrayService>>,
    pub map: Option<Arc<dyn MapService>>,
    pub console: Option<Arc<dyn ConsoleService>>,
}

impl std::fmt::Debug for CoreServices {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CoreServices")
            .field("string", &"StringService")
            .field("integer", &"IntegerService")
            .field("bool", &"BoolService")
            .field("array", &"ArrayService")
            .field("map", &"MapService")
            .field("console", &"ConsoleService")
            .finish()
    }
}

impl CoreServices {
    /// Phase 87 CoreBoxId の core_required (6個) を返す
    ///
    /// Phase 87 CoreBoxId::is_core_required() と完全一致する。
    pub fn required_ids() -> &'static [CoreBoxId] {
        &[
            CoreBoxId::String,
            CoreBoxId::Integer,
            CoreBoxId::Bool,
            CoreBoxId::Array,
            CoreBoxId::Map,
            CoreBoxId::Console,
        ]
    }

    /// 全フィールドが初期化されているか検証
    /// Phase 92 以降で各 Service の初期化を検証
    pub fn ensure_initialized(&self) {
        // Phase 91 では trait が空なので何もしない
        // Phase 92 以降で各 Service の初期化を検証
    }
}

// ============================================================================
// Phase 94: Adapter Pattern - Box → Service 変換
// ============================================================================

/// StringBox → StringService Adapter
///
/// Phase 95.5: 純粋関数設計
/// - Box を保持せず、純粋関数として実装
/// - len(s) → s.chars().count() (UTF-8 文字数)
/// - Phase 96 以降で substring(), concat() など追加予定
pub struct StringBoxAdapter;

impl StringBoxAdapter {
    pub fn new() -> Self {
        Self
    }
}

impl StringService for StringBoxAdapter {
    fn len(&self, s: &str) -> i64 {
        // Phase 95.5: 文字列長を返す（UTF-8 バイト数ではなく文字数）
        s.chars().count() as i64
    }
}

/// IntegerBox → IntegerService Adapter
///
/// Phase 97: 純粋関数型（Box状態不要）
pub struct IntegerBoxAdapter;

impl IntegerBoxAdapter {
    pub fn new() -> Self {
        Self
    }
}

impl IntegerService for IntegerBoxAdapter {
    fn add(&self, a: i64, b: i64) -> i64 {
        a.saturating_add(b) // オーバーフロー対策
    }

    fn sub(&self, a: i64, b: i64) -> i64 {
        a.saturating_sub(b) // アンダーフロー対策
    }

    fn mul(&self, a: i64, b: i64) -> i64 {
        a.saturating_mul(b) // オーバーフロー対策
    }

    fn div(&self, a: i64, b: i64) -> Option<i64> {
        if b == 0 {
            None // ゼロ除算
        } else {
            Some(a / b)
        }
    }
}

/// BoolBox → BoolService Adapter
///
/// Phase 97: 純粋関数型（Box状態不要）
pub struct BoolBoxAdapter;

impl BoolBoxAdapter {
    pub fn new() -> Self {
        Self
    }
}

impl BoolService for BoolBoxAdapter {
    fn not(&self, value: bool) -> bool {
        !value
    }

    fn and(&self, a: bool, b: bool) -> bool {
        a && b
    }

    fn or(&self, a: bool, b: bool) -> bool {
        a || b
    }

    fn xor(&self, a: bool, b: bool) -> bool {
        a ^ b
    }
}

/// ArrayBox → ArrayService Adapter
///
/// Ring1ArrayService への委譲で Array 実装SSOTを一元化する。
pub struct ArrayBoxAdapter;

impl ArrayBoxAdapter {
    pub fn new() -> Self {
        Self
    }
}

impl ArrayService for ArrayBoxAdapter {
    fn len(&self, arr: &dyn NyashBox) -> i64 {
        Ring1ArrayService::new().len(arr)
    }

    fn get(&self, arr: &dyn NyashBox, index: i64) -> Option<Box<dyn NyashBox>> {
        Ring1ArrayService::new().get(arr, index)
    }

    fn set(&self, arr: &dyn NyashBox, index: i64, value: Box<dyn NyashBox>) -> Result<(), String> {
        Ring1ArrayService::new().set(arr, index, value)
    }

    fn push(&self, arr: &dyn NyashBox, value: Box<dyn NyashBox>) -> Result<(), String> {
        Ring1ArrayService::new().push(arr, value)
    }
}

/// MapBox → MapService Adapter
///
/// Ring1MapService への委譲で Map 実装SSOTを一元化する。
pub struct MapBoxAdapter;

impl MapBoxAdapter {
    pub fn new() -> Self {
        Self
    }
}

impl MapService for MapBoxAdapter {
    fn size(&self, map: &dyn NyashBox) -> i64 {
        Ring1MapService::new().size(map)
    }

    fn has(&self, map: &dyn NyashBox, key: &str) -> bool {
        Ring1MapService::new().has(map, key)
    }

    fn get(&self, map: &dyn NyashBox, key: &str) -> Option<Box<dyn NyashBox>> {
        Ring1MapService::new().get(map, key)
    }

    fn set(&self, map: &dyn NyashBox, key: &str, value: Box<dyn NyashBox>) -> Result<(), String> {
        Ring1MapService::new().set(map, key, value)
    }
}

/// ConsoleBox → ConsoleService Adapter
///
/// Phase 95.5: Ring0 直結設計
/// - Box を保持せず、Ring0Context に直結
/// - println() → Ring0Context.log.info()
/// - print() → Ring0Context.io.stdout_write()
pub struct ConsoleBoxAdapter;

impl ConsoleBoxAdapter {
    pub fn new() -> Self {
        Self
    }
}

impl ConsoleService for ConsoleBoxAdapter {
    fn println(&self, msg: &str) {
        // Phase 95.5: Ring0Context 経由でログ出力（自動改行）
        use crate::runtime::ring0::get_global_ring0;
        let ring0 = get_global_ring0();
        ring0.log.info(msg);
    }

    fn print(&self, msg: &str) {
        // Phase 95.5: Ring0Context 経由で stdout 出力（改行なし）
        use crate::runtime::ring0::get_global_ring0;
        let ring0 = get_global_ring0();
        ring0.io.stdout_write(msg.as_bytes()).ok();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_required_ids_consistency() {
        let required = CoreServices::required_ids();
        assert_eq!(required.len(), 6);

        // Phase 87 CoreBoxId::is_core_required() と一致
        for id in required {
            assert!(id.is_core_required());
        }

        // 全ての core_required が含まれているか確認
        assert!(required.contains(&CoreBoxId::String));
        assert!(required.contains(&CoreBoxId::Integer));
        assert!(required.contains(&CoreBoxId::Bool));
        assert!(required.contains(&CoreBoxId::Array));
        assert!(required.contains(&CoreBoxId::Map));
        assert!(required.contains(&CoreBoxId::Console));
    }

    // Phase 95.5: Ring0 初期化ヘルパー（テスト用）
    #[cfg(test)]
    fn ensure_ring0_initialized() {
        use crate::runtime::ring0::{default_ring0, GLOBAL_RING0};
        use std::sync::Arc;

        // 既に初期化済みなら何もしない
        if GLOBAL_RING0.get().is_none() {
            GLOBAL_RING0.set(Arc::new(default_ring0())).ok();
        }
    }

    #[test]
    fn test_console_service_println() {
        // Phase 95.5: Ring0 初期化（安全な初期化）
        ensure_ring0_initialized();

        let adapter = ConsoleBoxAdapter::new();

        // Phase 95.5: println を呼び出し（Ring0 経由、panic しないことを確認）
        adapter.println("Test message from Ring0");
        // 実際の出力検証は Phase 96 以降
    }

    #[test]
    fn test_console_service_print() {
        // Phase 95.5: Ring0 初期化（安全な初期化）
        ensure_ring0_initialized();

        let adapter = ConsoleBoxAdapter::new();

        // Phase 95.5: print を呼び出し（Ring0 経由、panic しないことを確認）
        adapter.print("No newline");
        // 実際の出力検証は Phase 96 以降
    }

    #[test]
    fn test_string_service_len() {
        // Phase 95.5: 純粋関数なので Ring0 不要
        let adapter = StringBoxAdapter::new();

        // Phase 95: len を呼び出し
        let length = adapter.len("Hello");
        assert_eq!(length, 5);

        // UTF-8 対応確認
        let length_utf8 = adapter.len("こんにちは");
        assert_eq!(length_utf8, 5); // 5文字（バイト数は15）
    }

    #[test]
    fn test_console_service_ring0_integration() {
        // Phase 95.5: Ring0 初期化（安全な初期化）
        ensure_ring0_initialized();

        // ConsoleService 経由で出力（Ring0 使用）
        let adapter = ConsoleBoxAdapter::new();
        adapter.println("Test message from Ring0");
        adapter.print("No newline");

        // panic しないことを確認
    }

    #[test]
    fn test_array_service_basic_operations() {
        use crate::box_trait::IntegerBox;
        use crate::boxes::array::ArrayBox;

        let arr = ArrayBox::new();
        let adapter = ArrayBoxAdapter::new();

        // push
        let value = Box::new(IntegerBox::new(42));
        adapter.push(&arr, value).unwrap();

        // len
        assert_eq!(adapter.len(&arr), 1);

        // get
        let result = adapter.get(&arr, 0).unwrap();
        let int_box = result.as_any().downcast_ref::<IntegerBox>().unwrap();
        assert_eq!(int_box.value, 42);
    }

    #[test]
    fn test_array_service_set() {
        use crate::box_trait::IntegerBox;
        use crate::boxes::array::ArrayBox;

        let arr = ArrayBox::new();
        let adapter = ArrayBoxAdapter::new();

        // push initial value
        adapter.push(&arr, Box::new(IntegerBox::new(10))).unwrap();

        // set
        adapter.set(&arr, 0, Box::new(IntegerBox::new(20))).unwrap();

        // verify
        let result = adapter.get(&arr, 0).unwrap();
        let int_box = result.as_any().downcast_ref::<IntegerBox>().unwrap();
        assert_eq!(int_box.value, 20);
    }

    #[test]
    fn test_map_service_basic_operations() {
        use crate::box_trait::StringBox;
        use crate::boxes::map_box::MapBox;

        let map = MapBox::new();
        let adapter = MapBoxAdapter::new();

        // set
        let value = Box::new(StringBox::new("Hello"));
        adapter.set(&map, "key1", value).unwrap();

        // has
        assert!(adapter.has(&map, "key1"));
        assert!(!adapter.has(&map, "key2"));

        // get
        let result = adapter.get(&map, "key1").unwrap();
        let str_box = result.as_any().downcast_ref::<StringBox>().unwrap();
        assert_eq!(str_box.value, "Hello");

        // size
        assert_eq!(adapter.size(&map), 1);
    }

    #[test]
    fn test_map_service_multiple_keys() {
        use crate::box_trait::{IntegerBox, StringBox};
        use crate::boxes::map_box::MapBox;

        let map = MapBox::new();
        let adapter = MapBoxAdapter::new();

        // set multiple keys
        adapter
            .set(&map, "name", Box::new(StringBox::new("Alice")))
            .unwrap();
        adapter
            .set(&map, "age", Box::new(IntegerBox::new(25)))
            .unwrap();

        // verify size
        assert_eq!(adapter.size(&map), 2);

        // verify values
        let name = adapter.get(&map, "name").unwrap();
        let name_str = name.as_any().downcast_ref::<StringBox>().unwrap();
        assert_eq!(name_str.value, "Alice");

        let age = adapter.get(&map, "age").unwrap();
        let age_int = age.as_any().downcast_ref::<IntegerBox>().unwrap();
        assert_eq!(age_int.value, 25);
    }

    #[test]
    fn test_integer_service_operations() {
        let adapter = IntegerBoxAdapter::new();

        // add
        assert_eq!(adapter.add(10, 20), 30);
        assert_eq!(adapter.add(i64::MAX, 1), i64::MAX); // saturating

        // sub
        assert_eq!(adapter.sub(20, 10), 10);
        assert_eq!(adapter.sub(i64::MIN, 1), i64::MIN); // saturating

        // mul
        assert_eq!(adapter.mul(5, 6), 30);
        assert_eq!(adapter.mul(i64::MAX, 2), i64::MAX); // saturating

        // div
        assert_eq!(adapter.div(20, 5), Some(4));
        assert_eq!(adapter.div(10, 3), Some(3)); // 整数除算
        assert_eq!(adapter.div(10, 0), None); // ゼロ除算
    }

    #[test]
    fn test_bool_service_operations() {
        let adapter = BoolBoxAdapter::new();

        // not
        assert_eq!(adapter.not(true), false);
        assert_eq!(adapter.not(false), true);

        // and
        assert_eq!(adapter.and(true, true), true);
        assert_eq!(adapter.and(true, false), false);
        assert_eq!(adapter.and(false, false), false);

        // or
        assert_eq!(adapter.or(true, false), true);
        assert_eq!(adapter.or(false, false), false);

        // xor
        assert_eq!(adapter.xor(true, false), true);
        assert_eq!(adapter.xor(true, true), false);
        assert_eq!(adapter.xor(false, false), false);
    }
}
