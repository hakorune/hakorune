use super::CoreBoxId;

/// Phase 87: Core Method ID 定義
///
/// Box のメソッドを型安全に管理。
/// Phase 84-4-B のハードコード型情報を統合。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CoreMethodId {
    // ===== StringBox methods =====
    StringLength,
    StringUpper,
    StringLower,
    StringConcat,
    StringSubstring,
    StringIndexOf,
    StringIndexOfFrom,
    StringReplace,
    StringTrim,
    StringSplit,

    // ===== IntegerBox methods =====
    IntegerAbs,
    IntegerMin,
    IntegerMax,

    // ===== BoolBox methods =====
    BoolNot,
    BoolAnd,
    BoolOr,

    // ===== ArrayBox methods =====
    ArrayLength,
    ArrayPush,
    ArrayPop,
    ArrayGet,

    // ===== MapBox methods =====
    MapGet,
    MapSet,
    MapHas,
    MapKeys,

    // ===== ConsoleBox methods =====
    ConsolePrintln,
    ConsoleLog,
    ConsoleError,

    // ===== FileBox methods =====
    FileRead,
    FileWrite,
    FileOpen,

    // ===== ResultBox methods (QMark 対応) =====
    ResultIsOk,
    ResultGetValue,
}

use super::specs;

impl CoreMethodId {
    fn spec(&self) -> &'static specs::CoreMethodSpec {
        specs::iter_all_specs()
            .find(|spec| spec.id == *self)
            .expect("CoreMethodSpec missing for CoreMethodId")
    }

    /// メソッドが属する Box ID
    pub fn box_id(&self) -> CoreBoxId {
        self.spec().box_id
    }

    /// メソッド名（例: "length"）
    pub fn name(&self) -> &'static str {
        self.spec().name
    }

    /// 引数の数
    pub fn arity(&self) -> usize {
        self.spec().arity
    }

    /// Phase 84-4-B: 戻り値型（型推論用）
    pub fn return_type_name(&self) -> &'static str {
        self.spec().return_type_name
    }

    /// VTable slot for TypeRegistry (None when not exposed via vtable).
    pub fn vtable_slot(&self) -> Option<u16> {
        self.spec().vtable_slot
    }

    /// 全CoreMethodIdを反復
    pub fn iter() -> impl Iterator<Item = CoreMethodId> {
        specs::iter_all_specs().map(|spec| spec.id)
    }

    /// Box名とメソッド名から CoreMethodId を取得
    pub fn from_box_and_method(box_id: CoreBoxId, method: &str) -> Option<CoreMethodId> {
        let canonical = crate::runtime::core_method_aliases::canonical_method_name(method);
        specs::iter_all_specs()
            .find(|spec| spec.box_id == box_id && spec.name == canonical)
            .map(|spec| spec.id)
    }

    /// メソッド名とアリティから CoreMethodId を解決
    pub fn resolve_by_name_and_arity(
        method_name: &str,
        arg_len: usize,
    ) -> Result<CoreMethodId, Vec<usize>> {
        let canonical = crate::runtime::core_method_aliases::canonical_method_name(method_name);
        let mut expected = Vec::new();
        for spec in specs::iter_all_specs().filter(|spec| spec.name == canonical) {
            expected.push(spec.arity);
            if spec.arity == arg_len {
                return Ok(spec.id);
            }
        }
        expected.sort_unstable();
        expected.dedup();
        Err(expected)
    }

    /// Phase 224-B: Pure function (no side effects, deterministic)
    ///
    /// Pure functions can be safely:
    /// - Used in loop conditions
    /// - Called multiple times without changing behavior
    /// - Eliminated by dead code elimination if result unused
    ///
    /// Examples:
    /// - `StringLength`: Pure - always returns same length for same string
    /// - `ArrayPush`: Not pure - mutates the array (side effect)
    pub fn is_pure(&self) -> bool {
        self.spec().is_pure
    }

    /// Phase 224-B: Allowed in loop condition expressions
    ///
    /// Methods allowed in loop conditions must be:
    /// 1. Pure (no side effects)
    /// 2. Cheap to compute (no expensive I/O)
    /// 3. Deterministic (same input → same output)
    ///
    /// This is a whitelist approach - default to false for safety.
    pub fn allowed_in_condition(&self) -> bool {
        self.spec().allowed_in_condition
    }

    /// Phase 224-B: Allowed in loop body init expressions
    ///
    /// Methods allowed for LoopBodyLocal initialization.
    /// Similar to condition requirements but slightly more permissive.
    pub fn allowed_in_init(&self) -> bool {
        self.spec().allowed_in_init
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_core_method_id_box_id() {
        assert_eq!(CoreMethodId::StringLength.box_id(), CoreBoxId::String);
        assert_eq!(CoreMethodId::ArrayPush.box_id(), CoreBoxId::Array);
        assert_eq!(CoreMethodId::ConsolePrintln.box_id(), CoreBoxId::Console);
    }

    #[test]
    fn test_core_method_id_name() {
        assert_eq!(CoreMethodId::StringLength.name(), "length");
        assert_eq!(CoreMethodId::ArrayPush.name(), "push");
        assert_eq!(CoreMethodId::ConsolePrintln.name(), "println");
    }

    #[test]
    fn test_core_method_id_arity() {
        assert_eq!(CoreMethodId::StringLength.arity(), 0);
        assert_eq!(CoreMethodId::StringConcat.arity(), 1);
        assert_eq!(CoreMethodId::StringIndexOfFrom.arity(), 2);
        assert_eq!(CoreMethodId::MapSet.arity(), 2);
    }

    #[test]
    fn test_core_method_id_return_type() {
        assert_eq!(CoreMethodId::StringLength.return_type_name(), "IntegerBox");
        assert_eq!(CoreMethodId::StringUpper.return_type_name(), "StringBox");
        assert_eq!(CoreMethodId::BoolNot.return_type_name(), "BoolBox");
        assert_eq!(CoreMethodId::ArrayPush.return_type_name(), "Void");
    }

    #[test]
    fn test_core_method_id_from_box_and_method() {
        assert_eq!(
            CoreMethodId::from_box_and_method(CoreBoxId::String, "length"),
            Some(CoreMethodId::StringLength)
        );
        assert_eq!(
            CoreMethodId::from_box_and_method(CoreBoxId::Array, "push"),
            Some(CoreMethodId::ArrayPush)
        );
        assert_eq!(
            CoreMethodId::from_box_and_method(CoreBoxId::String, "unknown_method"),
            None
        );
    }

    #[test]
    fn test_core_method_id_iter() {
        let count = CoreMethodId::iter().count();
        assert!(count >= 27); // Phase 87: 27個以上のメソッド
    }

    #[test]
    fn test_core_method_spec_uniqueness() {
        let mut ids = HashSet::new();
        let mut signatures = HashSet::new();
        for spec in specs::iter_all_specs() {
            assert!(ids.insert(spec.id), "duplicate CoreMethodId in specs");
            let key = (spec.box_id, spec.name, spec.arity);
            assert!(
                signatures.insert(key),
                "duplicate CoreMethodSpec signature: {:?}",
                key
            );
        }
        assert_eq!(ids.len(), CoreMethodId::iter().count());
    }

    // ===== Phase 224-B tests =====

    #[test]
    fn test_core_method_id_is_pure() {
        // Pure string methods
        assert!(CoreMethodId::StringLength.is_pure());
        assert!(CoreMethodId::StringUpper.is_pure());
        assert!(CoreMethodId::StringSubstring.is_pure());

        // Pure array read methods
        assert!(CoreMethodId::ArrayLength.is_pure());
        assert!(CoreMethodId::ArrayGet.is_pure());

        // Impure - side effects
        assert!(!CoreMethodId::ArrayPush.is_pure());
        assert!(!CoreMethodId::ConsolePrintln.is_pure());
        assert!(!CoreMethodId::FileWrite.is_pure());
    }

    #[test]
    fn test_core_method_id_allowed_in_condition() {
        // Allowed - cheap and pure
        assert!(CoreMethodId::StringLength.allowed_in_condition());
        assert!(CoreMethodId::ArrayLength.allowed_in_condition());
        assert!(CoreMethodId::MapHas.allowed_in_condition());

        // Not allowed - not whitelisted (conservative)
        assert!(!CoreMethodId::StringUpper.allowed_in_condition());
        assert!(!CoreMethodId::StringSubstring.allowed_in_condition());

        // Not allowed - side effects
        assert!(!CoreMethodId::ArrayPush.allowed_in_condition());
        assert!(!CoreMethodId::ConsolePrintln.allowed_in_condition());
        assert!(!CoreMethodId::FileRead.allowed_in_condition());
    }

    #[test]
    fn test_core_method_id_allowed_in_init() {
        // Allowed - useful for LoopBodyLocal init
        assert!(CoreMethodId::StringLength.allowed_in_init());
        assert!(CoreMethodId::StringSubstring.allowed_in_init());
        assert!(CoreMethodId::StringUpper.allowed_in_init());
        assert!(CoreMethodId::ArrayGet.allowed_in_init());
        assert!(CoreMethodId::MapGet.allowed_in_init());

        // Not allowed - side effects
        assert!(!CoreMethodId::ArrayPush.allowed_in_init());
        assert!(!CoreMethodId::ConsolePrintln.allowed_in_init());
        assert!(!CoreMethodId::FileWrite.allowed_in_init());
    }
}