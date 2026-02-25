use crate::mir::join_ir::frontend::AstToJoinIrLowerer;
use crate::mir::join_ir::JoinModule;
use crate::runtime::get_global_ring0;
use crate::{config::env::joinir_dev_enabled, config::env::joinir_test_debug_enabled};

/// Pattern2 ブレークループ（fixture ベース）を Structured で組み立てるヘルパー。
///
/// Fixture: docs/private/roadmap2/phases/phase-34-joinir-frontend/fixtures/loop_frontend_break.program.json
pub fn build_pattern2_break_fixture_structured() -> JoinModule {
    const FIXTURE: &str = include_str!(
        "../../../../../docs/private/roadmap2/phases/phase-34-joinir-frontend/fixtures/loop_frontend_break.program.json"
    );

    let program_json: serde_json::Value =
        serde_json::from_str(FIXTURE).expect("fixture JSON should be valid");

    let mut lowerer = AstToJoinIrLowerer::new();
    lowerer.lower_program_json(&program_json)
}

/// JsonParser 由来のミニ P2 ループ（空白スキップ相当）を Structured で組み立てるヘルパー。
///
/// Fixture: docs/private/roadmap2/phases/normalized_dev/fixtures/jsonparser_skip_ws_mini.program.json
pub fn build_jsonparser_skip_ws_structured_for_normalized_dev() -> JoinModule {
    const FIXTURE: &str = include_str!(
        "../../../../../docs/private/roadmap2/phases/normalized_dev/fixtures/jsonparser_skip_ws_mini.program.json"
    );

    let program_json: serde_json::Value =
        serde_json::from_str(FIXTURE).expect("jsonparser skip_ws fixture should be valid JSON");

    let mut lowerer = AstToJoinIrLowerer::new();
    lowerer.lower_program_json(&program_json)
}

/// JsonParser _skip_whitespace 本体相当の P2 ループを Structured で組み立てるヘルパー。
///
/// Fixture: docs/private/roadmap2/phases/normalized_dev/fixtures/jsonparser_skip_ws_real.program.json
pub fn build_jsonparser_skip_ws_real_structured_for_normalized_dev() -> JoinModule {
    const FIXTURE: &str = include_str!(
        "../../../../../docs/private/roadmap2/phases/normalized_dev/fixtures/jsonparser_skip_ws_real.program.json"
    );

    let program_json: serde_json::Value = serde_json::from_str(FIXTURE)
        .expect("jsonparser skip_ws real fixture should be valid JSON");

    let mut lowerer = AstToJoinIrLowerer::new();
    let module = lowerer.lower_program_json(&program_json);

    if joinir_dev_enabled() && joinir_test_debug_enabled() {
        get_global_ring0().log.debug(&format!(
            "[joinir/normalized-dev] jsonparser_skip_ws_real structured module: {:#?}",
            module
        ));
    }

    module
}

/// JsonParser _parse_number 本体相当の P2 ループを Structured で組み立てるヘルパー。
///
/// Fixture: docs/private/roadmap2/phases/normalized_dev/fixtures/jsonparser_parse_number_real.program.json
pub fn build_jsonparser_parse_number_real_structured_for_normalized_dev() -> JoinModule {
    const FIXTURE: &str = include_str!(
        "../../../../../docs/private/roadmap2/phases/normalized_dev/fixtures/jsonparser_parse_number_real.program.json"
    );

    let program_json: serde_json::Value = serde_json::from_str(FIXTURE)
        .expect("jsonparser parse_number real fixture should be valid JSON");

    let mut lowerer = AstToJoinIrLowerer::new();
    let module = lowerer.lower_program_json(&program_json);

    if joinir_dev_enabled() && joinir_test_debug_enabled() {
        get_global_ring0().log.debug(&format!(
            "[joinir/normalized-dev] jsonparser_parse_number_real structured module: {:#?}",
            module
        ));
    }

    module
}

/// selfhost token-scan 系の P2 ループを Structured で組み立てるヘルパー。
///
/// Fixture: docs/private/roadmap2/phases/normalized_dev/fixtures/selfhost_token_scan_p2.program.json
pub fn build_selfhost_token_scan_p2_structured_for_normalized_dev() -> JoinModule {
    const FIXTURE: &str = include_str!(
        "../../../../../docs/private/roadmap2/phases/normalized_dev/fixtures/selfhost_token_scan_p2.program.json"
    );

    let program_json: serde_json::Value =
        serde_json::from_str(FIXTURE).expect("selfhost token_scan P2 fixture should be valid JSON");

    let mut lowerer = AstToJoinIrLowerer::new();
    let module = lowerer.lower_program_json(&program_json);

    if joinir_dev_enabled() && joinir_test_debug_enabled() {
        get_global_ring0().log.debug(&format!(
            "[joinir/normalized-dev] selfhost_token_scan_p2 structured module: {:#?}",
            module
        ));
    }

    module
}

/// selfhost token-scan 系の P2 ループ（accum 拡張）を Structured で組み立てるヘルパー。
///
/// Fixture: docs/private/roadmap2/phases/normalized_dev/fixtures/selfhost_token_scan_p2_accum.program.json
pub fn build_selfhost_token_scan_p2_accum_structured_for_normalized_dev() -> JoinModule {
    const FIXTURE: &str = include_str!(
        "../../../../../docs/private/roadmap2/phases/normalized_dev/fixtures/selfhost_token_scan_p2_accum.program.json"
    );

    let program_json: serde_json::Value = serde_json::from_str(FIXTURE)
        .expect("selfhost token_scan P2 accum fixture should be valid JSON");

    let mut lowerer = AstToJoinIrLowerer::new();
    let module = lowerer.lower_program_json(&program_json);

    if joinir_dev_enabled() && joinir_test_debug_enabled() {
        get_global_ring0().log.debug(&format!(
            "[joinir/normalized-dev] selfhost_token_scan_p2_accum structured module: {:#?}",
            module
        ));
    }

    module
}

/// selfhost if-sum P3 を Structured で組み立てるヘルパー。
///
/// Fixture: docs/private/roadmap2/phases/normalized_dev/fixtures/selfhost_if_sum_p3.program.json
pub fn build_selfhost_if_sum_p3_structured_for_normalized_dev() -> JoinModule {
    const FIXTURE: &str = include_str!(
        "../../../../../docs/private/roadmap2/phases/normalized_dev/fixtures/selfhost_if_sum_p3.program.json"
    );

    let program_json: serde_json::Value =
        serde_json::from_str(FIXTURE).expect("selfhost if_sum P3 fixture should be valid JSON");

    let mut lowerer = AstToJoinIrLowerer::new();
    let module = lowerer.lower_program_json(&program_json);

    if joinir_dev_enabled() && joinir_test_debug_enabled() {
        get_global_ring0().log.debug(&format!(
            "[joinir/normalized-dev] selfhost_if_sum_p3 structured module: {:#?}",
            module
        ));
    }

    module
}

/// selfhost if-sum P3（ext 拡張）を Structured で組み立てるヘルパー。
///
/// Fixture: docs/private/roadmap2/phases/normalized_dev/fixtures/selfhost_if_sum_p3_ext.program.json
pub fn build_selfhost_if_sum_p3_ext_structured_for_normalized_dev() -> JoinModule {
    const FIXTURE: &str = include_str!(
        "../../../../../docs/private/roadmap2/phases/normalized_dev/fixtures/selfhost_if_sum_p3_ext.program.json"
    );

    let program_json: serde_json::Value =
        serde_json::from_str(FIXTURE).expect("selfhost if_sum P3 ext fixture should be valid JSON");

    let mut lowerer = AstToJoinIrLowerer::new();
    let module = lowerer.lower_program_json(&program_json);

    if joinir_dev_enabled() && joinir_test_debug_enabled() {
        get_global_ring0().log.debug(&format!(
            "[joinir/normalized-dev] selfhost_if_sum_p3_ext structured module: {:#?}",
            module
        ));
    }

    module
}

/// selfhost args-parse P2（Phase 53）を Structured で組み立てるヘルパー。
///
/// Fixture: docs/private/roadmap2/phases/normalized_dev/fixtures/selfhost_args_parse_p2.program.json
pub fn build_selfhost_args_parse_p2_structured_for_normalized_dev() -> JoinModule {
    const FIXTURE: &str = include_str!(
        "../../../../../docs/private/roadmap2/phases/normalized_dev/fixtures/selfhost_args_parse_p2.program.json"
    );

    let program_json: serde_json::Value =
        serde_json::from_str(FIXTURE).expect("selfhost args_parse P2 fixture should be valid JSON");

    let mut lowerer = AstToJoinIrLowerer::new();
    let module = lowerer.lower_program_json(&program_json);

    if joinir_dev_enabled() && joinir_test_debug_enabled() {
        get_global_ring0().log.debug(&format!(
            "[joinir/normalized-dev] selfhost_args_parse_p2 structured module: {:#?}",
            module
        ));
    }

    module
}

/// selfhost stmt-count P3（Phase 53）を Structured で組み立てるヘルパー。
///
/// Fixture: docs/private/roadmap2/phases/normalized_dev/fixtures/selfhost_stmt_count_p3.program.json
pub fn build_selfhost_stmt_count_p3_structured_for_normalized_dev() -> JoinModule {
    const FIXTURE: &str = include_str!(
        "../../../../../docs/private/roadmap2/phases/normalized_dev/fixtures/selfhost_stmt_count_p3.program.json"
    );

    let program_json: serde_json::Value =
        serde_json::from_str(FIXTURE).expect("selfhost stmt_count P3 fixture should be valid JSON");

    let mut lowerer = AstToJoinIrLowerer::new();
    let module = lowerer.lower_program_json(&program_json);

    if joinir_dev_enabled() && joinir_test_debug_enabled() {
        get_global_ring0().log.debug(&format!(
            "[joinir/normalized-dev] selfhost_stmt_count_p3 structured module: {:#?}",
            module
        ));
    }

    module
}

/// JsonParser _atoi 相当のミニ P2 ループを Structured で組み立てるヘルパー。
///
/// Fixture: docs/private/roadmap2/phases/normalized_dev/fixtures/jsonparser_atoi_mini.program.json
pub fn build_jsonparser_atoi_structured_for_normalized_dev() -> JoinModule {
    const FIXTURE: &str = include_str!(
        "../../../../../docs/private/roadmap2/phases/normalized_dev/fixtures/jsonparser_atoi_mini.program.json"
    );

    let program_json: serde_json::Value =
        serde_json::from_str(FIXTURE).expect("jsonparser atoi fixture should be valid JSON");

    let mut lowerer = AstToJoinIrLowerer::new();
    let module = lowerer.lower_program_json(&program_json);

    if joinir_dev_enabled() && joinir_test_debug_enabled() {
        get_global_ring0().log.debug(&format!(
            "[joinir/normalized-dev] jsonparser_atoi_mini structured module: {:#?}",
            module
        ));
    }

    module
}

/// JsonParser _atoi 本体相当の P2 ループを Structured で組み立てるヘルパー。
///
/// Fixture: docs/private/roadmap2/phases/normalized_dev/fixtures/jsonparser_atoi_real.program.json
pub fn build_jsonparser_atoi_real_structured_for_normalized_dev() -> JoinModule {
    const FIXTURE: &str = include_str!(
        "../../../../../docs/private/roadmap2/phases/normalized_dev/fixtures/jsonparser_atoi_real.program.json"
    );

    let program_json: serde_json::Value =
        serde_json::from_str(FIXTURE).expect("jsonparser atoi real fixture should be valid JSON");

    let mut lowerer = AstToJoinIrLowerer::new();
    let module = lowerer.lower_program_json(&program_json);

    if joinir_dev_enabled() && joinir_test_debug_enabled() {
        get_global_ring0().log.debug(&format!(
            "[joinir/normalized-dev] jsonparser_atoi_real structured module: {:#?}",
            module
        ));
    }

    module
}

/// Pattern4 continue minimal ループ（pattern4_continue_min 相当）を Structured で組み立てるヘルパー。
///
/// Phase 48-A: P4 Normalized の最小ケース検証用（dev-only）。
/// 単純な continue パターン（i == 2 でスキップ）。
///
/// Fixture: docs/private/roadmap2/phases/normalized_dev/fixtures/pattern4_continue_min.program.json
pub fn build_pattern4_continue_min_structured_for_normalized_dev() -> JoinModule {
    use super::super::dev_fixtures::NormalizedDevFixture;
    NormalizedDevFixture::Pattern4ContinueMinimal.load_and_lower()
}

/// JsonParser _parse_array の whitespace continue ループを Structured で組み立てるヘルパー（dev-only）。
///
/// Fixture: docs/private/roadmap2/phases/normalized_dev/fixtures/jsonparser_parse_array_continue_skip_ws.program.json
pub fn build_jsonparser_parse_array_continue_skip_ws_structured_for_normalized_dev() -> JoinModule {
    use super::super::dev_fixtures::NormalizedDevFixture;
    NormalizedDevFixture::Pattern4JsonParserParseArrayContinueSkipWs.load_and_lower()
}

/// JsonParser _parse_object の whitespace continue ループを Structured で組み立てるヘルパー（dev-only）。
///
/// Fixture: docs/private/roadmap2/phases/normalized_dev/fixtures/jsonparser_parse_object_continue_skip_ws.program.json
pub fn build_jsonparser_parse_object_continue_skip_ws_structured_for_normalized_dev() -> JoinModule
{
    use super::super::dev_fixtures::NormalizedDevFixture;
    NormalizedDevFixture::Pattern4JsonParserParseObjectContinueSkipWs.load_and_lower()
}

/// JsonParser _unescape_string の「i+=2 + continue」コアを Structured で組み立てるヘルパー（dev-only）。
///
/// 実ループ（`tools/hako_shared/json_parser.hako::_unescape_string`）から、
/// 文字列処理を除いて制御構造（continue + 可変ステップ更新）だけを抽出した最小フィクスチャ。
///
/// Fixture: docs/private/roadmap2/phases/normalized_dev/fixtures/jsonparser_unescape_string_step2_min.program.json
pub fn build_jsonparser_unescape_string_step2_min_structured_for_normalized_dev() -> JoinModule {
    const FIXTURE: &str = include_str!(
        "../../../../../docs/private/roadmap2/phases/normalized_dev/fixtures/jsonparser_unescape_string_step2_min.program.json"
    );

    let program_json: serde_json::Value = serde_json::from_str(FIXTURE)
        .expect("jsonparser_unescape_string_step2_min fixture should be valid JSON");

    let mut lowerer = AstToJoinIrLowerer::new();
    let module = lowerer.lower_program_json(&program_json);

    if joinir_dev_enabled() && joinir_test_debug_enabled() {
        get_global_ring0().log.debug(&format!(
            "[joinir/normalized-dev] jsonparser_unescape_string_step2_min structured module: {:#?}",
            module
        ));
    }

    module
}

/// Pattern Continue + Return minimal を Structured で組み立てるヘルパー
///
/// Fixture: docs/private/roadmap2/phases/normalized_dev/fixtures/pattern_continue_return_min.program.json
pub fn build_pattern_continue_return_min_structured_for_normalized_dev() -> JoinModule {
    use super::super::dev_fixtures::NormalizedDevFixture;
    NormalizedDevFixture::PatternContinueReturnMin.load_and_lower()
}

/// Parse String Composite minimal を Structured で組み立てるヘルパー
///
/// Fixture: docs/private/roadmap2/phases/normalized_dev/fixtures/parse_string_composite_min.program.json
pub fn build_parse_string_composite_min_structured_for_normalized_dev() -> JoinModule {
    use super::super::dev_fixtures::NormalizedDevFixture;
    NormalizedDevFixture::ParseStringCompositeMin.load_and_lower()
}

/// Parse Array minimal を Structured で組み立てるヘルパー
///
/// Fixture: docs/private/roadmap2/phases/normalized_dev/fixtures/parse_array_min.program.json
pub fn build_parse_array_min_structured_for_normalized_dev() -> JoinModule {
    use super::super::dev_fixtures::NormalizedDevFixture;
    NormalizedDevFixture::ParseArrayMin.load_and_lower()
}

/// Parse Object minimal を Structured で組み立てるヘルパー
///
/// Fixture: docs/private/roadmap2/phases/normalized_dev/fixtures/parse_object_min.program.json
pub fn build_parse_object_min_structured_for_normalized_dev() -> JoinModule {
    use super::super::dev_fixtures::NormalizedDevFixture;
    NormalizedDevFixture::ParseObjectMin.load_and_lower()
}
