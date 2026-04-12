use super::*;

#[test]
fn parser_accepts_legacy_annotations_as_rune_metadata_on_callable_declarations() {
    with_features(Some("stage3,rune"), || {
        let src = r#"
static box Main {
  @hint(hot)
  @contract(no_alloc)
  @intrinsic_candidate("StringBox.length/0")
  main() {
    return 0
  }
}
"#;
        let (ast, metadata) = NyashParser::parse_from_string_with_metadata(src)
            .expect("parse with legacy declaration metadata");
        let runes = find_runes(&metadata);
        assert_eq!(
            runes,
            vec![
                ("Hint".to_string(), vec!["hot".to_string()]),
                ("Contract".to_string(), vec!["no_alloc".to_string()]),
                (
                    "IntrinsicCandidate".to_string(),
                    vec!["StringBox.length/0".to_string()]
                ),
            ]
        );
        let (_box_runes, method_runes) = find_box_and_method_runes(&ast, "Main", "main");
        assert_eq!(method_runes, runes);
    });
}

#[test]
fn parser_accepts_canonical_optimization_runes_and_preserves_metadata() {
    with_features(Some("rune"), || {
        let src = r#"
static box Main {
  @rune Hint(inline)
  @rune Contract(no_alloc)
  @rune IntrinsicCandidate("StringBox.length/0")
  main() {
    return 0
  }
}
"#;
        let (ast, metadata) = NyashParser::parse_from_string_with_metadata(src)
            .expect("parse with canonical rune families");
        let runes = find_runes(&metadata);
        assert_eq!(
            runes,
            vec![
                ("Hint".to_string(), vec!["inline".to_string()]),
                ("Contract".to_string(), vec!["no_alloc".to_string()]),
                (
                    "IntrinsicCandidate".to_string(),
                    vec!["StringBox.length/0".to_string()]
                ),
            ]
        );

        let (_box_runes, method_runes) = find_box_and_method_runes(&ast, "Main", "main");
        assert_eq!(
            method_runes,
            vec![
                ("Hint".to_string(), vec!["inline".to_string()]),
                ("Contract".to_string(), vec!["no_alloc".to_string()]),
                (
                    "IntrinsicCandidate".to_string(),
                    vec!["StringBox.length/0".to_string()]
                ),
            ]
        );
    });
}

#[test]
fn parser_accepts_mixed_legacy_aliases_and_canonical_runes_on_same_declaration() {
    with_features(Some("rune"), || {
        let src = r#"
static box Main {
  @rune Hint(hot)
  @contract(no_alloc)
  @intrinsic_candidate("StringBox.length/0")
  @rune Symbol("main_sym")
  @rune CallConv("c")
  main() {
    return 0
  }
}
"#;
        let ast = NyashParser::parse_from_string(src).expect("parse mixed metadata preamble");
        let (_box_runes, method_runes) = find_box_and_method_runes(&ast, "Main", "main");
        assert_eq!(
            method_runes,
            vec![
                ("Hint".to_string(), vec!["hot".to_string()]),
                ("Contract".to_string(), vec!["no_alloc".to_string()]),
                (
                    "IntrinsicCandidate".to_string(),
                    vec!["StringBox.length/0".to_string()]
                ),
                ("Symbol".to_string(), vec!["main_sym".to_string()]),
                ("CallConv".to_string(), vec!["c".to_string()]),
            ]
        );
    });
}

#[test]
fn parser_accepts_canonical_rune_surface_under_opt_annotations_gate() {
    with_features(Some("stage3,opt-annotations"), || {
        let src = r#"
static box Main {
  @rune Hint(hot)
  main() { return 0 }
}
"#;
        let ast = NyashParser::parse_from_string(src)
            .expect("canonical rune surface should parse under compat gate");
        let (_box_runes, method_runes) = find_box_and_method_runes(&ast, "Main", "main");
        assert_eq!(
            method_runes,
            vec![("Hint".to_string(), vec!["hot".to_string()])]
        );
    });
}

#[test]
fn parser_accepts_canonical_rune_control_plane_surface_and_roundtrips_ast_json() {
    with_features(Some("rune"), || {
        let src = r#"
@rune Public
static box Main {
  @rune FfiSafe
  @rune ReturnsOwned
  @rune FreeWith("cleanup_main")
  @rune Symbol("main_sym")
  @rune CallConv("c")
  @rune Hint(inline)
  @rune Contract(no_alloc)
  @rune IntrinsicCandidate("Main.main/0")
  main() {
    return 0
  }
}
"#;
        let (ast, metadata) = NyashParser::parse_from_string_with_metadata(src)
            .expect("parse canonical rune surface");
        let runes = find_runes(&metadata);
        assert_eq!(
            runes,
            vec![
                ("Public".to_string(), vec![]),
                ("FfiSafe".to_string(), vec![]),
                ("ReturnsOwned".to_string(), vec![]),
                ("FreeWith".to_string(), vec!["cleanup_main".to_string()]),
                ("Symbol".to_string(), vec!["main_sym".to_string()]),
                ("CallConv".to_string(), vec!["c".to_string()]),
                ("Hint".to_string(), vec!["inline".to_string()]),
                ("Contract".to_string(), vec!["no_alloc".to_string()]),
                (
                    "IntrinsicCandidate".to_string(),
                    vec!["Main.main/0".to_string()]
                ),
            ]
        );

        let (box_runes, method_runes) = find_box_and_method_runes(&ast, "Main", "main");
        assert_eq!(box_runes, vec![("Public".to_string(), vec![])]);
        assert_eq!(
            method_runes,
            vec![
                ("FfiSafe".to_string(), vec![]),
                ("ReturnsOwned".to_string(), vec![]),
                ("FreeWith".to_string(), vec!["cleanup_main".to_string()]),
                ("Symbol".to_string(), vec!["main_sym".to_string()]),
                ("CallConv".to_string(), vec!["c".to_string()]),
                ("Hint".to_string(), vec!["inline".to_string()]),
                ("Contract".to_string(), vec!["no_alloc".to_string()]),
                (
                    "IntrinsicCandidate".to_string(),
                    vec!["Main.main/0".to_string()]
                ),
            ]
        );

        let roundtrip = json_to_ast(&ast_to_json_roundtrip(&ast)).expect("ast roundtrip");
        let (roundtrip_box_runes, roundtrip_method_runes) =
            find_box_and_method_runes(&roundtrip, "Main", "main");
        assert_eq!(roundtrip_box_runes, box_runes);
        assert_eq!(roundtrip_method_runes, method_runes);
    });
}

#[test]
fn parser_accepts_rune_annotations_and_preserves_metadata() {
    with_features(Some("rune"), || {
        let src = r#"
@rune Public
static box Main {
  @rune Ownership(owned)
  main() {
    return 0
  }
}
"#;
        let (ast, metadata) =
            NyashParser::parse_from_string_with_metadata(src).expect("parse with rune");
        let runes = find_runes(&metadata);
        assert_eq!(
            runes,
            vec![
                ("Public".to_string(), vec![]),
                ("Ownership".to_string(), vec!["owned".to_string()])
            ]
        );
        let (box_runes, method_runes) = find_box_and_method_runes(&ast, "Main", "main");
        assert_eq!(box_runes, vec![("Public".to_string(), vec![])]);
        assert_eq!(
            method_runes,
            vec![("Ownership".to_string(), vec!["owned".to_string()])]
        );
    });
}
