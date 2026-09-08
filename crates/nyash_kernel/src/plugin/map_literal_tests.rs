use super::*;
use std::sync::Arc;

fn handle(value: impl NyashBox + 'static) -> i64 {
    handles::to_handle_arc(Arc::new(value)) as i64
}
fn key(text: &str) -> i64 {
    handle(StringBox::new(text))
}
fn read(map: i64, key: &str) -> Box<dyn NyashBox> {
    let object = handles::get(map as u64).unwrap();
    object
        .as_any()
        .downcast_ref::<MapBox>()
        .unwrap()
        .get_opt_key_str(key)
        .unwrap()
}
fn put(map: i64, text: &str, kind: u32, payload: u64) {
    assert_eq!(
        nyash_map_literal_store_v1(map, key(text), kind, payload),
        NYRT_MAP_LITERAL_OK
    );
}

#[test]
fn scalar_classes_do_not_alias_live_handles() {
    let map = handle(MapBox::new());
    let live = key("this is not the integer value");
    put(map, "integer", NYRT_MAP_LITERAL_I64, live as u64);
    put(map, "negative", NYRT_MAP_LITERAL_I64, i64::MIN as u64);
    put(map, "bool", NYRT_MAP_LITERAL_BOOL, 1);
    put(map, "void", NYRT_MAP_LITERAL_VOID, 0);
    assert_eq!(
        read(map, "integer")
            .as_any()
            .downcast_ref::<IntegerBox>()
            .unwrap()
            .value,
        live
    );
    assert_eq!(
        read(map, "negative")
            .as_any()
            .downcast_ref::<IntegerBox>()
            .unwrap()
            .value,
        i64::MIN
    );
    assert!(
        read(map, "bool")
            .as_any()
            .downcast_ref::<BoolBox>()
            .unwrap()
            .value
    );
    assert!(read(map, "void").as_any().is::<VoidBox>());
}

#[test]
fn float_payload_preserves_negative_zero_and_nan_bits() {
    let map = handle(MapBox::new());
    for bits in [0x8000000000000000, 0x7ff8000000000123, 1.25f64.to_bits()] {
        put(map, "float", NYRT_MAP_LITERAL_F64, bits);
        assert_eq!(
            read(map, "float")
                .as_any()
                .downcast_ref::<FloatBox>()
                .unwrap()
                .value
                .to_bits(),
            bits
        );
    }
}

#[test]
fn invalid_contract_does_not_replace_or_insert() {
    let map = handle(MapBox::new());
    let k = key("kept");
    put(map, "kept", NYRT_MAP_LITERAL_I64, 17);
    for (kind, payload) in [
        (0, 0),
        (99, 0),
        (NYRT_MAP_LITERAL_BOOL, 2),
        (NYRT_MAP_LITERAL_VOID, 1),
        (NYRT_MAP_LITERAL_HANDLE, 0),
        (NYRT_MAP_LITERAL_HANDLE, u64::MAX),
    ] {
        assert_eq!(
            nyash_map_literal_store_v1(map, k, kind, payload),
            NYRT_MAP_LITERAL_INVALID_CONTRACT
        );
    }
    let dead = key("removed");
    handles::drop_handle(dead as u64);
    assert_eq!(
        nyash_map_literal_store_v1(map, k, NYRT_MAP_LITERAL_HANDLE, dead as u64),
        2
    );
    for (receiver, bad_key) in [(0, k), (k, k), (map, 0), (map, handle(IntegerBox::new(3)))] {
        assert_eq!(
            nyash_map_literal_store_v1(receiver, bad_key, NYRT_MAP_LITERAL_I64, 99),
            2
        );
    }
    assert_eq!(
        read(map, "kept")
            .as_any()
            .downcast_ref::<IntegerBox>()
            .unwrap()
            .value,
        17
    );
    let object = handles::get(map as u64).unwrap();
    assert_eq!(object.as_any().downcast_ref::<MapBox>().unwrap().len(), 1);
}

#[test]
fn string_value_survives_source_handle_drop() {
    let map = handle(MapBox::new());
    let text = key("retained\0猫");
    put(map, "text", NYRT_MAP_LITERAL_HANDLE, text as u64);
    handles::drop_handle(text as u64);
    assert_eq!(read(map, "text").to_string_box().value, "retained\0猫");
}

#[test]
fn nested_map_keeps_existing_clone_policy_and_self_store_does_not_deadlock() {
    let outer = handle(MapBox::new());
    let inner = handle(MapBox::new());
    put(inner, "n", NYRT_MAP_LITERAL_I64, 1);
    put(outer, "child", NYRT_MAP_LITERAL_HANDLE, inner as u64);
    put(inner, "n", NYRT_MAP_LITERAL_I64, 2);
    let nested = read(outer, "child");
    assert_eq!(
        nested
            .as_any()
            .downcast_ref::<MapBox>()
            .unwrap()
            .get_scalar_i64_key_str("n"),
        Some(1)
    );
    put(outer, "self", NYRT_MAP_LITERAL_HANDLE, outer as u64);
    assert!(read(outer, "self").as_any().is::<MapBox>());
}

#[test]
fn keys_preserve_bytes_and_duplicate_replacement() {
    let map = handle(MapBox::new());
    for (text, value) in [("", 1), ("猫\0尾", 2), ("1", 3), ("01", 4), ("1", 5)] {
        put(map, text, NYRT_MAP_LITERAL_I64, value);
    }
    for (text, expected) in [("", 1), ("猫\0尾", 2), ("1", 5), ("01", 4)] {
        assert_eq!(
            read(map, text)
                .as_any()
                .downcast_ref::<IntegerBox>()
                .unwrap()
                .value,
            expected
        );
    }
    let object = handles::get(map as u64).unwrap();
    assert_eq!(object.as_any().downcast_ref::<MapBox>().unwrap().len(), 4);
}

#[test]
fn string_view_materializes_and_survives_both_handle_drops() {
    use crate::exports::string_view::StringViewBox;
    let map = handle(MapBox::new());
    let base = key("[view\0text]");
    let view = handle(StringViewBox::new(
        base,
        handles::get(base as u64).unwrap(),
        1,
        10,
    ));
    put(map, "view", NYRT_MAP_LITERAL_HANDLE, view as u64);
    handles::drop_handle(view as u64);
    handles::drop_handle(base as u64);
    let stored = read(map, "view");
    assert!(stored.as_any().is::<StringBox>());
    assert_eq!(stored.to_string_box().value, "view\0text");
}

#[test]
fn exported_symbols_compose_length_aware_key_with_store() {
    extern "C" {
        #[link_name = "nyash.box.from_i8_string_const_len_v1"]
        fn literal_text(bytes: *const u8, len: u64) -> i64;
        #[link_name = "nyash.map.literal_store_v1"]
        fn literal_store(map: i64, key: i64, kind: u32, payload: u64) -> u32;
    }
    let map = handle(MapBox::new());
    let bytes = "ffi\0猫".as_bytes();
    let text = unsafe { literal_text(bytes.as_ptr(), bytes.len() as u64) };
    assert!(text > 0);
    assert_eq!(
        unsafe { literal_store(map, text, NYRT_MAP_LITERAL_I64, 30) },
        0
    );
    assert_eq!(
        read(map, "ffi\0猫")
            .as_any()
            .downcast_ref::<IntegerBox>()
            .unwrap()
            .value,
        30
    );
}
