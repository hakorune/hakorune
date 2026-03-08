//! Function routing for AST lowering
//!
//! Phase 89 リファクタリング:
//! - FunctionRoute の定義とルーティングロジックを集約

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FunctionRoute {
    IfReturn,
    LoopFrontend,
    NestedIf,
    ReadQuoted,
}

pub(crate) fn resolve_function_route(func_name: &str) -> Result<FunctionRoute, String> {
    // By-name allowlist for current Program JSON frontend entrypoints.
    // Historical normalized-dev fixture names are retired and tracked in the retirement SSOT.
    const TABLE: &[(&str, FunctionRoute)] = &[
        ("test", FunctionRoute::IfReturn),
        ("local", FunctionRoute::IfReturn),
        ("_read_value_from_pair", FunctionRoute::IfReturn),
        ("simple", FunctionRoute::LoopFrontend),
    ];
    const NESTED_IF_KEYS: &[&str] = &["nested_if_merge"];
    const READ_QUOTED_KEYS: &[&str] = &["read_quoted"];

    if let Some((_, route)) = TABLE.iter().find(|(name, _)| *name == func_name) {
        return Ok(*route);
    }

    if NESTED_IF_KEYS.contains(&func_name) {
        if crate::config::env::joinir_dev_enabled()
            && crate::config::env::joinir_dev::nested_if_enabled()
        {
            return Ok(FunctionRoute::NestedIf);
        }
        return Err(format!(
            "[joinir/frontend] '{}' requires HAKO_JOINIR_NESTED_IF=1 (dev only; current key: nested_if_merge)",
            func_name
        ));
    }

    if READ_QUOTED_KEYS.contains(&func_name) {
        if crate::config::env::joinir_dev_enabled()
            && crate::config::env::joinir_dev::read_quoted_enabled()
        {
            return Ok(FunctionRoute::ReadQuoted);
        }
        return Err(format!(
            "[joinir/frontend] '{}' requires HAKO_JOINIR_READ_QUOTED=1 (dev only; current key: read_quoted)",
            func_name
        ));
    }

    Err(format!(
        "[joinir/frontend] unsupported function '{}' (dev fixture not registered)",
        func_name
    ))
}

#[cfg(test)]
mod tests {
    use super::{resolve_function_route, FunctionRoute};

    #[test]
    fn current_program_json_route_keys_resolve_to_expected_routes() {
        for (name, expected) in [
            ("test", FunctionRoute::IfReturn),
            ("local", FunctionRoute::IfReturn),
            ("_read_value_from_pair", FunctionRoute::IfReturn),
            ("simple", FunctionRoute::LoopFrontend),
        ] {
            let route =
                resolve_function_route(name).expect("current accepted Program JSON key must stay live");
            assert_eq!(
                route, expected,
                "current accepted Program JSON key must resolve to the frozen route bucket: {name}"
            );
        }
    }

    #[test]
    fn retired_legacy_if_phi_join_fixture_keys_are_rejected() {
        for name in [
            "pattern3_if_sum_multi_min",
            "jsonparser_if_sum_min",
            "selfhost_if_sum_p3",
            "selfhost_if_sum_p3_ext",
        ] {
            let err = resolve_function_route(name).expect_err("legacy key must be retired");
            assert!(
                err.contains("unsupported function"),
                "legacy key should fail via unsupported-function path: {name} => {err}"
            );
        }
    }

    #[test]
    fn retired_unused_selfhost_fixture_keys_are_rejected() {
        for name in [
            "selfhost_token_scan_p2",
            "selfhost_token_scan_p2_accum",
            "selfhost_args_parse_p2",
            "selfhost_stmt_count_p3",
            "if_phi_join_multi_min",
            "jsonparser_if_phi_join_min",
            "selfhost_if_phi_join",
            "selfhost_if_phi_join_ext",
            "selfhost_verify_schema_p2",
            "selfhost_detect_format_p3",
            "jsonparser_unescape_string_step2_min",
        ] {
            let err = resolve_function_route(name).expect_err("unused legacy key must be retired");
            assert!(
                err.contains("unsupported function"),
                "retired selfhost key should fail via unsupported-function path: {name} => {err}"
            );
        }
    }

    #[test]
    fn retired_program_json_loop_frontend_compat_keys_are_rejected() {
        for name in ["filter", "print_tokens", "map", "reduce", "fold"] {
            let err = resolve_function_route(name).expect_err("compat key must be retired");
            assert!(
                err.contains("unsupported function"),
                "retired Program JSON key should fail via unsupported-function path: {name} => {err}"
            );
        }
    }

    #[test]
    fn nested_if_dev_keys_fail_fast_without_env() {
        for name in ["nested_if_merge"] {
            let err = resolve_function_route(name).expect_err("dev-gated key must fail without env");
            assert!(
                err.contains("HAKO_JOINIR_NESTED_IF=1"),
                "nested-if dev key should fail via env guard path: {name} => {err}"
            );
        }
    }

    #[test]
    fn read_quoted_dev_keys_fail_fast_without_env() {
        for name in ["read_quoted"] {
            let err = resolve_function_route(name).expect_err("dev-gated key must fail without env");
            assert!(
                err.contains("HAKO_JOINIR_READ_QUOTED=1"),
                "read_quoted dev key should fail via env guard path: {name} => {err}"
            );
        }
    }

    #[test]
    fn retired_read_quoted_compat_key_is_rejected() {
        let err =
            resolve_function_route("read_quoted_from").expect_err("compat key must be retired");
        assert!(
            err.contains("unsupported function"),
            "retired read_quoted compat key should fail via unsupported-function path: {err}"
        );
    }

    #[test]
    fn retired_parse_loop_compat_key_is_rejected() {
        let err = resolve_function_route("parse_loop").expect_err("compat key must be retired");
        assert!(
            err.contains("unsupported function"),
            "retired parse_loop compat key should fail via unsupported-function path: {err}"
        );
    }

    #[test]
    fn retired_historical_jsonparser_loop_frontend_keys_are_rejected() {
        for name in [
            "jsonparser_skip_ws_mini",
            "jsonparser_skip_ws_real",
            "jsonparser_atoi_mini",
            "jsonparser_atoi_real",
            "jsonparser_parse_number_real",
        ] {
            let err = resolve_function_route(name).expect_err("historical key must be retired");
            assert!(
                err.contains("unsupported function"),
                "retired jsonparser historical key should fail via unsupported-function path: {name} => {err}"
            );
        }
    }
}
