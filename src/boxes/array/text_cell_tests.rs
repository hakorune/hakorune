use super::{append_text_suffix, text_contains_literal, ArrayTextCell};

#[test]
fn text_contains_literal_matches_str_contains() {
    let values = [
        "",
        "line-seed",
        "xxline-seed",
        "seed-line",
        "naive cafe",
        "東京line大阪",
        "abc日本語def",
    ];
    let needles = [
        "", "l", "li", "line", "seed", "cafe", "東京", "日本", "absent",
    ];
    for value in values {
        for needle in needles {
            assert_eq!(
                text_contains_literal(value, needle),
                value.contains(needle),
                "value={value:?} needle={needle:?}"
            );
        }
    }
}

#[test]
fn append_text_suffix_matches_push_str() {
    let suffixes = ["", "l", "ln", "東京", "🙂", "abcdefghi"];
    for suffix in suffixes {
        let mut actual = String::from("line-seed");
        append_text_suffix(&mut actual, suffix);

        let mut expected = String::from("line-seed");
        expected.push_str(suffix);
        assert_eq!(actual, expected, "suffix={suffix:?}");
    }
}

#[test]
fn mid_gap_lenhalf_insert_matches_flat_string() {
    let mut cell = ArrayTextCell::flat("line-seed".to_string());
    let mut expected = "line-seed".to_string();

    for step in 0..128 {
        let out = cell.insert_const_mid_lenhalf("xx");
        let split = expected.len() / 2;
        expected.insert_str(split, "xx");
        assert_eq!(out, expected.len() as i64, "step={step}");

        if step % 8 == 0 {
            cell.append_suffix("ln");
            expected.push_str("ln");
        }

        assert_eq!(cell.len(), expected.len(), "step={step}");
        assert_eq!(cell.to_visible_string(), expected, "step={step}");
    }
}

#[test]
fn byte_boundary_safe_lenhalf_insert_matches_checked_ascii() {
    let mut checked = ArrayTextCell::flat("line-seed".to_string());
    let mut fast = ArrayTextCell::flat("line-seed".to_string());

    for step in 0..128 {
        let checked_len = checked.insert_const_mid_lenhalf("xx");
        let fast_len = fast.insert_const_mid_lenhalf_byte_boundary_safe("xx");
        assert_eq!(fast_len, checked_len, "step={step}");
        assert_eq!(
            fast.to_visible_string(),
            checked.to_visible_string(),
            "step={step}"
        );

        if step % 8 == 0 {
            checked.append_suffix("ln");
            fast.append_suffix("ln");
            assert_eq!(
                fast.to_visible_string(),
                checked.to_visible_string(),
                "append step={step}"
            );
        }
    }
}

#[test]
fn mid_gap_contains_literal_checks_boundary() {
    let mut cell = ArrayTextCell::flat("ab".to_string());
    assert_eq!(cell.insert_const_mid_lenhalf("XY"), 4);

    assert!(cell.contains_literal("aX"));
    assert!(cell.contains_literal("XY"));
    assert!(cell.contains_literal("Yb"));
    assert!(!cell.contains_literal("Ya"));
}

#[test]
fn four_byte_literal_word_matches_generic_contains() {
    let needles = ["line", "seed", "🙂", "none"];
    let cells = [
        ArrayTextCell::flat("line-seed🙂".to_string()),
        ArrayTextCell::MidGap {
            left: "ab".to_string(),
            right: "cd-line".to_string(),
            right_start: 0,
        },
        ArrayTextCell::MidGap {
            left: "prefix-line".to_string(),
            right: "tail".to_string(),
            right_start: 0,
        },
    ];

    for cell in cells {
        for needle in needles {
            let word = ArrayTextCell::four_byte_literal_word(needle).unwrap();
            assert_eq!(
                cell.contains_four_byte_literal(word),
                cell.contains_literal(needle),
                "cell={cell:?} needle={needle:?}"
            );
        }
    }
}

#[test]
fn mid_gap_as_mut_string_materializes_explicitly() {
    let mut cell = ArrayTextCell::flat("abcd".to_string());
    assert_eq!(cell.insert_const_mid_lenhalf("XY"), 6);

    cell.as_mut_string().push_str("!");
    assert_eq!(cell.to_visible_string(), "abXYcd!");
}
