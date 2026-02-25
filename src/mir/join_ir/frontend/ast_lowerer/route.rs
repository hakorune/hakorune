//! Function routing for AST lowering
//!
//! Phase 89 リファクタリング:
//! - FunctionRoute の定義とルーティングロジックを集約
//! - normalized_dev fixture は SSOT (dev_fixtures.rs) から自動登録

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FunctionRoute {
    IfReturn,
    LoopFrontend,
    NestedIf,
    ReadQuoted,
}

pub(crate) fn resolve_function_route(func_name: &str) -> Result<FunctionRoute, String> {
    // Dev fixtures を SSOT から自動登録
    #[cfg(feature = "normalized_dev")]
    {
        use crate::mir::join_ir::normalized::dev_fixtures::ALL_DEV_FIXTURES;
        for fixture in ALL_DEV_FIXTURES {
            if func_name == fixture.function_name() {
                return Ok(fixture.route());
            }
        }
    }

    // 通常のルーティングテーブル
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
        ("pattern3_if_sum_multi_min", FunctionRoute::LoopFrontend),
        ("jsonparser_if_sum_min", FunctionRoute::LoopFrontend),
        ("selfhost_token_scan_p2", FunctionRoute::LoopFrontend),
        ("selfhost_token_scan_p2_accum", FunctionRoute::LoopFrontend),
        ("selfhost_args_parse_p2", FunctionRoute::LoopFrontend),
        ("selfhost_if_sum_p3", FunctionRoute::LoopFrontend),
        ("selfhost_if_sum_p3_ext", FunctionRoute::LoopFrontend),
        ("selfhost_stmt_count_p3", FunctionRoute::LoopFrontend),
        // Phase 54: selfhost P2/P3 shape growth
        ("selfhost_verify_schema_p2", FunctionRoute::LoopFrontend),
        ("selfhost_detect_format_p3", FunctionRoute::LoopFrontend),
        // Phase 88: JsonParser _unescape_string core (step2 + continue) minimal fixture
        (
            "jsonparser_unescape_string_step2_min",
            FunctionRoute::LoopFrontend,
        ),
        // Note: Phase 48-A/48-B/89-P1 fixtures are auto-registered from dev_fixtures.rs
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
