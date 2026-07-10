use nyash_rust::parser::{GrammarProfile, NyashParser, ParserBuildConfig};
use nyash_rust::test_support::with_env_vars;

fn ensure_ring0_initialized_for_alias_warning() {
    use nyash_rust::runtime::ring0::{default_ring0, init_global_ring0};
    let _ = std::panic::catch_unwind(|| {
        init_global_ring0(default_ring0());
    });
}

fn with_stage3_env<F: FnOnce()>(
    features: Option<&str>,
    parser_stage3: Option<&str>,
    hako_stage3: Option<&str>,
    f: F,
) {
    ensure_ring0_initialized_for_alias_warning();
    // Phase 73: Unified to NYASH_FEATURES=stage3.
    // Legacy aliases (NYASH_PARSER_STAGE3 / HAKO_PARSER_STAGE3) still gate on/off.
    with_env_vars(
        &[
            ("NYASH_FEATURES", features),
            ("NYASH_PARSER_STAGE3", parser_stage3),
            ("HAKO_PARSER_STAGE3", hako_stage3),
        ],
        f,
    );
}

fn parse_compat2025(
    code: &str,
) -> Result<nyash_rust::ast::ASTNode, nyash_rust::parser::ParseError> {
    let config = ParserBuildConfig {
        grammar_profile: GrammarProfile::Compat2025,
        ..ParserBuildConfig::default()
    };
    NyashParser::parse_from_string_with_build_config(code, config)
}

#[test]
fn canonical_default_rejects_try_and_throw() {
    with_stage3_env(None, None, None, || {
        let code_try = "try { local x = 1 } catch () { }";
        let res_try = NyashParser::parse_from_string(code_try);
        assert!(
            format!("{:?}", res_try.err()).contains("[parser/try_reserved]"),
            "Canonical default must reject statement try"
        );

        let code_throw = "throw 1";
        let res_throw = NyashParser::parse_from_string(code_throw);
        assert!(
            res_throw.is_err(),
            "throw should be reserved/prohibited by default"
        );
    });
}

#[test]
fn stage3_disabled_rejects_try_and_throw() {
    with_stage3_env(None, Some("0"), Some("0"), || {
        let code_try = "try { local x = 1 } catch () { }";
        let res_try = NyashParser::parse_from_string(code_try);
        assert!(res_try.is_err(), "try should be rejected when gate is off");

        let code_throw = "throw 1";
        let res_throw = NyashParser::parse_from_string(code_throw);
        assert!(
            res_throw.is_err(),
            "throw should be rejected when gate is off"
        );
    });
}

#[test]
fn stage3_enabled_without_compat_rejects_throw() {
    with_stage3_env(Some("stage3"), None, None, || {
        let code = "throw (1 + 2)";
        let res = NyashParser::parse_from_string(code);
        assert!(
            res.is_err(),
            "throw should stay prohibited without throw-compat"
        );
    });
}

#[test]
fn throw_compat_feature_still_rejects_throw() {
    with_stage3_env(Some("stage3,throw-compat"), None, None, || {
        let code = "throw (1 + 2)";
        let res = NyashParser::parse_from_string(code);
        assert!(
            res.is_err(),
            "throw must stay reserved/prohibited even when legacy feature flag is set"
        );
    });
}

#[test]
fn no_try_compat_feature_rejects_try_with_freeze_tag() {
    with_stage3_env(Some("stage3,no-try-compat"), None, None, || {
        let code_try = "try { local x = 1 } catch () { }";
        let res_try = NyashParser::parse_from_string(code_try);
        assert!(
            res_try.is_err(),
            "Canonical should reject independently of no-try-compat"
        );
        let err = format!("{:?}", res_try.err());
        assert!(
            err.contains("[freeze:contract][parser/try_reserved]"),
            "missing try freeze tag: {}",
            err
        );
    });
}

#[test]
fn compat2025_accepts_only_normalizable_try_shape() {
    with_stage3_env(Some("stage3"), None, None, || {
        // (Type var)
        let code1 = r#"
            try { local a = 1 }
            catch (Error e) { local b = 2 }
            cleanup { local z = 3 }
        "#;
        let err1 = format!("{:?}", parse_compat2025(code1).err());
        assert!(err1.contains("[parser/try_compat_not_normalizable]"));

        // (var) only
        let code2 = r#"
            try { local a = 1 }
            catch (e) { local b = 2 }
        "#;
        let err2 = format!("{:?}", parse_compat2025(code2).err());
        assert!(err2.contains("[parser/try_compat_not_normalizable]"));

        // () empty
        let code3 = r#"
            try { local a = 1 }
            catch () { local b = 2 }
        "#;
        assert!(parse_compat2025(code3).is_ok());

        let code4 = r#"
            try { local a = 1 }
            catch () { local b = 2 }
            cleanup { local z = 3 }
        "#;
        assert!(parse_compat2025(code4).is_ok());
    });
}

#[test]
fn stage3_rejects_finally_alias_keyword() {
    with_stage3_env(Some("stage3"), None, None, || {
        let code = r#"
            try { local a = 1 }
            catch () { local b = 2 }
            finally { local z = 3 }
        "#;
        let res = parse_compat2025(code);
        assert!(
            res.is_err(),
            "finally must be rejected; use cleanup: {:?}",
            res.err()
        );
    });
}
