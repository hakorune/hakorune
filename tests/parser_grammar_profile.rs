use nyash_rust::parser::{
    parse_migration_transport_with_config, GrammarProfile, MigrationTransportKind, NyashParser,
    ParseError, ParserBuildConfig,
};
use nyash_rust::tokenizer::NyashTokenizer;

fn parse_with_profile(
    source: &str,
    grammar_profile: GrammarProfile,
) -> Result<nyash_rust::ast::ASTNode, nyash_rust::parser::ParseError> {
    NyashParser::parse_from_string_with_build_config(
        source,
        ParserBuildConfig {
            grammar_profile,
            ..ParserBuildConfig::default()
        },
    )
}

#[test]
fn tokenizer_retains_the_explicit_profile() {
    let tokenizer = NyashTokenizer::with_grammar_profile("try", GrammarProfile::Compat2025);
    assert_eq!(tokenizer.grammar_profile(), GrammarProfile::Compat2025);
}

#[test]
fn canonical_default_and_explicit_canonical_reject_statement_try() {
    let source = "try { local x = 1 } catch () { }";
    for result in [
        NyashParser::parse_from_string(source),
        parse_with_profile(source, GrammarProfile::Canonical),
    ] {
        let error = format!("{:?}", result.unwrap_err());
        assert!(error.contains("[parser/try_reserved]"), "{error}");
    }
}

#[test]
fn compat2025_accepts_only_the_closed_try_subset() {
    let accepted = "try { local x = 1 } catch () { } cleanup { local y = 2 }";
    assert!(parse_with_profile(accepted, GrammarProfile::Compat2025).is_ok());

    let rejected = "try { local x = 1 } catch (Error e) { }";
    let error = format!(
        "{:?}",
        parse_with_profile(rejected, GrammarProfile::Compat2025).unwrap_err()
    );
    assert!(error.contains("[parser/try_compat_not_normalizable]"));
}

#[test]
fn profile_plumbing_preserves_canonical_match() {
    let match_source = "local x = match 1 { 1 => 2, _ => 0 }";
    for profile in [GrammarProfile::Canonical, GrammarProfile::Compat2025] {
        assert!(parse_with_profile(match_source, profile).is_ok());
    }
}

#[test]
fn canonical_rejects_each_legacy_from_surface_before_ast_publication() {
    for (source, tag) in [
        ("box Child from Parent {}", "parser/from_inheritance_legacy"),
        ("from Parent.method()", "parser/from_call_legacy"),
    ] {
        for result in [
            NyashParser::parse_from_string(source),
            parse_with_profile(source, GrammarProfile::Canonical),
        ] {
            let error = format!("{:?}", result.expect_err("legacy from must reject"));
            assert!(error.contains(tag), "{error}");
        }
    }
}

#[test]
fn compat2025_semantic_parser_stops_from_before_ast_publication() {
    for (source, expected_kind) in [
        (
            "box Child from Parent {}",
            MigrationTransportKind::BoxFromInheritance,
        ),
        ("from Parent.method()", MigrationTransportKind::FromCall),
    ] {
        let error = parse_with_profile(source, GrammarProfile::Compat2025)
            .expect_err("transport-only syntax must not publish semantic AST");
        match error {
            ParseError::TransportOnly {
                profile,
                transport_kind,
                stable_reject_tag,
                ..
            } => {
                assert_eq!(profile, GrammarProfile::Compat2025);
                assert_eq!(transport_kind, expected_kind);
                assert_eq!(stable_reject_tag, "parser/from_compat_transport_only");
            }
            other => panic!("expected transport-only error, got {other:?}"),
        }
    }
}

#[test]
fn compat2025_migration_adapter_emits_span_free_transport_evidence() {
    let config = ParserBuildConfig {
        grammar_profile: GrammarProfile::Compat2025,
        ..ParserBuildConfig::default()
    };
    for (source, expected_kind) in [
        (
            "box Child from Parent {}",
            MigrationTransportKind::BoxFromInheritance,
        ),
        ("from Parent.method()", MigrationTransportKind::FromCall),
    ] {
        let bundle = parse_migration_transport_with_config(source, config.clone())
            .expect("closed legacy form must emit migration evidence");
        assert_eq!(bundle.transport.transport_kind, expected_kind);
        assert_eq!(
            bundle
                .witness
                .normalized_form
                .as_ref()
                .map(|form| form.kind.as_str()),
            Some("CompatibilityTransport")
        );
        assert_eq!(
            bundle.witness.migration_transport_ref.as_deref(),
            Some(bundle.transport.transport_id.as_str())
        );
        assert!(!bundle.transport.semantic_entry_allowed);
        assert!(!bundle.transport.ast_publication_allowed);
        assert!(!bundle.transport.mir_lowering_allowed);
        assert!(!bundle.transport.runtime_lowering_allowed);
        assert!(!bundle.transport.backend_lowering_allowed);
    }
}

#[test]
fn malformed_legacy_from_forms_fail_fast_instead_of_falling_back() {
    for source in [
        "box Child from Parent { local x = 1 }",
        "from Parent.method(1)",
    ] {
        let error = format!(
            "{:?}",
            parse_with_profile(source, GrammarProfile::Compat2025)
                .expect_err("non-closed legacy form must reject")
        );
        assert!(
            error.contains("parser/from_transport_not_closed_form"),
            "{error}"
        );
    }
}

#[test]
fn option_sugar_keeps_its_internal_from_call_representation() {
    for source in ["local x = none", "local x = some 1"] {
        assert!(
            parse_with_profile(source, GrammarProfile::Canonical).is_ok(),
            "Option sugar must not enter the source from transport boundary: {source}"
        );
    }
}

#[test]
fn peek_is_canonical_reject_and_compat2025_match_alias() {
    let peek_source = "local x = peek 1 { 1 => 2, _ => 0 }";
    let canonical_error = format!(
        "{:?}",
        parse_with_profile(peek_source, GrammarProfile::Canonical).unwrap_err()
    );
    assert!(canonical_error.contains("[parser/peek_legacy_replaced_by_match]"));

    let match_source = "local x = match 1 { 1 => 2, _ => 0 }";
    let peek_ast = parse_with_profile(peek_source, GrammarProfile::Compat2025).unwrap();
    let match_ast = parse_with_profile(match_source, GrammarProfile::Compat2025).unwrap();
    assert_eq!(
        nyash_rust::r#macro::ast_json::ast_to_json_roundtrip(&peek_ast),
        nyash_rust::r#macro::ast_json::ast_to_json_roundtrip(&match_ast)
    );
}

#[test]
fn compat2025_peek_rejects_non_match_shape_without_fallback() {
    let source = "local x = peek 1 { 1 => }";
    let error = format!(
        "{:?}",
        parse_with_profile(source, GrammarProfile::Compat2025).unwrap_err()
    );
    assert!(error.contains("[parser/peek_compat_not_normalizable]"));
}

#[test]
fn while_is_explicit_compat2025_alias() {
    let source = "while ready { break }";
    let canonical_error = format!(
        "{:?}",
        parse_with_profile(source, GrammarProfile::Canonical).unwrap_err()
    );
    assert!(
        canonical_error.contains("parser/while_legacy_replaced_by_loop_condition"),
        "{canonical_error}"
    );
    assert!(parse_with_profile(source, GrammarProfile::Compat2025).is_ok());
}

#[test]
fn noncanonical_loop_spellings_reject_in_every_profile() {
    for (source, stable_tag) in [
        ("for item in 0..1 { break }", "parser/for_loop_noncanonical"),
        ("do { break } while ready", "parser/do_while_noncanonical"),
        ("repeat { break }", "parser/repeat_loop_noncanonical"),
        ("until ready { break }", "parser/until_loop_noncanonical"),
    ] {
        for profile in [GrammarProfile::Canonical, GrammarProfile::Compat2025] {
            let error = format!("{:?}", parse_with_profile(source, profile).unwrap_err());
            assert!(error.contains(stable_tag), "{source}: {error}");
        }
    }
}

#[test]
fn typed_integer_suffix_is_not_a_language_v1_surface() {
    for profile in [GrammarProfile::Canonical, GrammarProfile::Compat2025] {
        let error = format!("{:?}", parse_with_profile("1usize", profile).unwrap_err());
        assert!(
            error.contains("parser/typed_integer_suffix_rust_evidence_only"),
            "{error}"
        );
    }
}

#[test]
fn percent_brace_map_literal_is_profile_independent() {
    for profile in [GrammarProfile::Canonical, GrammarProfile::Compat2025] {
        assert!(
            parse_with_profile("%{\"key\" => 1}", profile).is_ok(),
            "map literal must not depend on ambient syntax-sugar state"
        );
    }
}
