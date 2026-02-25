//! Tests for StaticMethodId structure (Phase 21.7++ Phase 1)
//!
//! 責務: NamingBox SSOT の基盤となる StaticMethodId のパース・フォーマットを検証

use crate::mir::naming::StaticMethodId;

#[test]
fn test_parse_with_arity() {
    let id = StaticMethodId::parse("Main._nop/0").unwrap();
    assert_eq!(id.box_name, "Main");
    assert_eq!(id.method, "_nop");
    assert_eq!(id.arity, Some(0));
}

#[test]
fn test_parse_without_arity() {
    let id = StaticMethodId::parse("StringUtils.starts_with").unwrap();
    assert_eq!(id.box_name, "StringUtils");
    assert_eq!(id.method, "starts_with");
    assert_eq!(id.arity, None);
}

#[test]
fn test_normalize_box_name() {
    // main → Main に normalize されることを確認
    let id = StaticMethodId::parse("main._nop/0").unwrap();
    assert_eq!(id.box_name, "Main");
    assert_eq!(id.method, "_nop");
    assert_eq!(id.arity, Some(0));
}

#[test]
fn test_format_with_arity() {
    let id = StaticMethodId {
        box_name: "StringUtils".to_string(),
        method: "starts_with".to_string(),
        arity: Some(2),
    };
    assert_eq!(id.format(), "StringUtils.starts_with/2");
}

#[test]
fn test_format_without_arity() {
    let id = StaticMethodId {
        box_name: "StringUtils".to_string(),
        method: "starts_with".to_string(),
        arity: None,
    };
    assert_eq!(id.format(), "StringUtils.starts_with");
}

#[test]
fn test_with_arity() {
    let id = StaticMethodId::parse("StringUtils.starts_with").unwrap();
    let with_arity = id.with_arity(2);
    assert_eq!(with_arity.arity, Some(2));
    assert_eq!(with_arity.format(), "StringUtils.starts_with/2");
}

#[test]
fn test_round_trip() {
    let cases = vec!["Main._nop/0", "StringUtils.starts_with/2", "Console.log/1"];

    for case in cases {
        let id = StaticMethodId::parse(case).unwrap();
        let formatted = id.format();
        assert_eq!(formatted, case, "Round-trip failed for: {}", case);
    }
}

#[test]
fn test_parse_invalid_no_dot() {
    // ドットがない場合は None
    assert!(StaticMethodId::parse("print").is_none());
}

#[test]
fn test_parse_invalid_arity() {
    // arity が数値でない場合は None
    assert!(StaticMethodId::parse("Main.method/invalid").is_none());
}

#[test]
fn test_parse_multiple_dots() {
    // 最後のドットで分離されることを確認
    let id = StaticMethodId::parse("namespace.Box.method/2").unwrap();
    assert_eq!(id.box_name, "namespace.Box");
    assert_eq!(id.method, "method");
    assert_eq!(id.arity, Some(2));
}

#[test]
fn test_arity_zero() {
    // arity 0 を正しく処理
    let id = StaticMethodId::parse("Main._nop/0").unwrap();
    assert_eq!(id.arity, Some(0));
    assert_eq!(id.format(), "Main._nop/0");
}

#[test]
fn test_arity_large_number() {
    // 大きな arity も処理可能
    let id = StaticMethodId::parse("Utils.variadic/99").unwrap();
    assert_eq!(id.arity, Some(99));
    assert_eq!(id.format(), "Utils.variadic/99");
}

#[test]
fn test_with_arity_override() {
    // 既存の arity を上書き
    let id = StaticMethodId {
        box_name: "Box".to_string(),
        method: "method".to_string(),
        arity: Some(1),
    };
    let overridden = id.with_arity(3);
    assert_eq!(overridden.arity, Some(3));
    assert_eq!(overridden.format(), "Box.method/3");
}
