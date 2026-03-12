use nyash_rust::parser::NyashParser;
use std::sync::{Mutex, OnceLock};

fn env_guard() -> &'static Mutex<()> {
    static GUARD: OnceLock<Mutex<()>> = OnceLock::new();
    GUARD.get_or_init(|| Mutex::new(()))
}

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
    let _lock = env_guard().lock().unwrap_or_else(|e| e.into_inner());
    ensure_ring0_initialized_for_alias_warning();
    let prev_features = std::env::var("NYASH_FEATURES").ok();
    let prev_parser_stage3 = std::env::var("NYASH_PARSER_STAGE3").ok();
    let prev_hako_stage3 = std::env::var("HAKO_PARSER_STAGE3").ok();

    // Phase 73: Unified to NYASH_FEATURES=stage3
    // Legacy aliases (NYASH_PARSER_STAGE3 / HAKO_PARSER_STAGE3) still gate on/off.
    match features {
        Some(v) => std::env::set_var("NYASH_FEATURES", v),
        None => std::env::remove_var("NYASH_FEATURES"),
    }
    match parser_stage3 {
        Some(v) => std::env::set_var("NYASH_PARSER_STAGE3", v),
        None => std::env::remove_var("NYASH_PARSER_STAGE3"),
    }
    match hako_stage3 {
        Some(v) => std::env::set_var("HAKO_PARSER_STAGE3", v),
        None => std::env::remove_var("HAKO_PARSER_STAGE3"),
    }

    f();

    match prev_features {
        Some(v) => std::env::set_var("NYASH_FEATURES", v),
        None => std::env::remove_var("NYASH_FEATURES"),
    }
    match prev_parser_stage3 {
        Some(v) => std::env::set_var("NYASH_PARSER_STAGE3", v),
        None => std::env::remove_var("NYASH_PARSER_STAGE3"),
    }
    match prev_hako_stage3 {
        Some(v) => std::env::set_var("HAKO_PARSER_STAGE3", v),
        None => std::env::remove_var("HAKO_PARSER_STAGE3"),
    }
}

#[test]
fn stage3_default_enabled_accepts_try_and_rejects_throw() {
    with_stage3_env(None, None, None, || {
        let code_try = "try { local x = 1 } catch () { }";
        let res_try = NyashParser::parse_from_string(code_try);
        assert!(
            res_try.is_ok(),
            "try should parse when Stage-3 is default-enabled: {:?}",
            res_try.err()
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
            "try should be rejected with no-try-compat"
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
fn stage3_enabled_accepts_try_catch_variants() {
    with_stage3_env(Some("stage3"), None, None, || {
        // (Type var)
        let code1 = r#"
            try { local a = 1 }
            catch (Error e) { local b = 2 }
            cleanup { local z = 3 }
        "#;
        assert!(NyashParser::parse_from_string(code1).is_ok());

        // (var) only
        let code2 = r#"
            try { local a = 1 }
            catch (e) { local b = 2 }
        "#;
        assert!(NyashParser::parse_from_string(code2).is_ok());

        // () empty
        let code3 = r#"
            try { local a = 1 }
            catch () { local b = 2 }
        "#;
        assert!(NyashParser::parse_from_string(code3).is_ok());
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
        let res = NyashParser::parse_from_string(code);
        assert!(
            res.is_err(),
            "finally must be rejected; use cleanup: {:?}",
            res.err()
        );
    });
}
