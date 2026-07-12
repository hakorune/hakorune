use super::*;

#[test]
fn strict_json_tree_uses_root_zero_and_preserves_order() {
    let arena =
        StrictJsonArenaV0::parse(r#"{"b":["猫",true],"a":{"nested":null}}"#).expect("strict JSON");
    let root = arena.root();
    assert_eq!(root.raw(), 0);
    assert_eq!(arena.kind(root), Some(StrictJsonKindV0::Object));
    assert_eq!(arena.object_len(root), Some(2));
    assert_eq!(arena.object_key_at(root, 0), Some("b"));
    assert_eq!(arena.object_key_at(root, 1), Some("a"));

    let array = arena.object_value_at(root, 0).expect("array child");
    assert_eq!(arena.array_len(array), Some(2));
    let text = arena.array_at(array, 0).expect("text child");
    assert_eq!(arena.string_value(text), Some("猫"));
    assert_eq!(
        arena.kind(arena.array_at(array, 1).unwrap()),
        Some(StrictJsonKindV0::Bool)
    );
}

#[test]
fn strict_json_tree_keeps_generic_scalar_kinds_and_checked_ids() {
    let arena = StrictJsonArenaV0::parse(r#"[null,false,-1,9223372036854775808,1.5,"x"]"#)
        .expect("strict JSON");
    let root = arena.root();
    assert_eq!(arena.node_count(), 7);
    assert_eq!(
        arena.kind(arena.array_at(root, 0).unwrap()),
        Some(StrictJsonKindV0::Null)
    );
    assert_eq!(
        arena.bool_value(arena.array_at(root, 1).unwrap()),
        Some(false)
    );
    assert_eq!(arena.i64_value(arena.array_at(root, 2).unwrap()), Some(-1));
    assert_eq!(
        arena.kind(arena.array_at(root, 3).unwrap()),
        Some(StrictJsonKindV0::U64)
    );
    assert_eq!(
        arena.u64_value(arena.array_at(root, 3).unwrap()),
        Some(9_223_372_036_854_775_808)
    );
    assert_eq!(
        arena.kind(arena.array_at(root, 4).unwrap()),
        Some(StrictJsonKindV0::F64)
    );
    assert_eq!(StrictJsonNodeIdV0::from_i64(-1), None);
    assert_eq!(StrictJsonNodeIdV0::from_i64(0).unwrap().raw(), 0);
}

#[test]
fn strict_json_tree_keeps_duplicate_and_trailing_rejection() {
    for input in [r#"{"a":1,"\u0061":2}"#, r#"{"a":1} trailing"#] {
        assert!(
            StrictJsonArenaV0::parse(input).is_err(),
            "accepted: {input}"
        );
    }
}
