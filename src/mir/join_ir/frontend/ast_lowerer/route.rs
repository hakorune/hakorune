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
        ("filter", FunctionRoute::LoopFrontend),
        ("print_tokens", FunctionRoute::LoopFrontend),
        ("map", FunctionRoute::LoopFrontend),
        ("reduce", FunctionRoute::LoopFrontend),
        ("fold", FunctionRoute::LoopFrontend),
        ("jsonparser_skip_ws_mini", FunctionRoute::LoopFrontend),
        ("jsonparser_skip_ws_real", FunctionRoute::LoopFrontend),
        ("jsonparser_atoi_mini", FunctionRoute::LoopFrontend),
        ("jsonparser_atoi_real", FunctionRoute::LoopFrontend),
        ("jsonparser_parse_number_real", FunctionRoute::LoopFrontend),
    ];

    if let Some((_, route)) = TABLE.iter().find(|(name, _)| *name == func_name) {
        return Ok(*route);
    }

    if func_name == "parse_loop" {
        if crate::config::env::joinir_dev_enabled()
            && crate::config::env::joinir_dev::nested_if_enabled()
        {
            return Ok(FunctionRoute::NestedIf);
        }
        return Err(
            "[joinir/frontend] 'parse_loop' requires HAKO_JOINIR_NESTED_IF=1 (dev only)"
                .to_string(),
        );
    }

    if func_name == "read_quoted_from" {
        if crate::config::env::joinir_dev_enabled()
            && crate::config::env::joinir_dev::read_quoted_enabled()
        {
            return Ok(FunctionRoute::ReadQuoted);
        }
        return Err(
            "[joinir/frontend] 'read_quoted_from' requires HAKO_JOINIR_READ_QUOTED=1 (dev only)"
                .to_string(),
        );
    }

    Err(format!(
        "[joinir/frontend] unsupported function '{}' (dev fixture not registered)",
        func_name
    ))
}

#[cfg(test)]
mod tests {
    use super::resolve_function_route;

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
}
