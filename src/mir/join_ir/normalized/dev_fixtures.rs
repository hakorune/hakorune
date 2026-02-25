//! Normalized dev fixture の SSOT (Single Source of Truth)
//!
//! Phase 89 リファクタリング:
//! - fixture 名・パス・ルーティング先を一箇所で管理
//! - 散在する文字列リテラルを減らし、typo・不一致を防止

#![cfg(feature = "normalized_dev")]

use super::super::frontend::ast_lowerer::route::FunctionRoute;

/// Normalized dev fixture の列挙型（SSOT）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NormalizedDevFixture {
    /// Pattern 4: Continue minimal (Phase 48-A)
    Pattern4ContinueMinimal,
    /// Pattern 4: JsonParser parse_array continue skip_ws (Phase 48-B)
    Pattern4JsonParserParseArrayContinueSkipWs,
    /// Pattern 4: JsonParser parse_object continue skip_ws (Phase 48-B)
    Pattern4JsonParserParseObjectContinueSkipWs,
    /// Pattern Continue + Early Return minimal (Phase 89 P1)
    PatternContinueReturnMin,
    /// Parse String Composite minimal (Phase 90 P0)
    ParseStringCompositeMin,
    /// Refactor-B: ContinueReturn multi minimal (multiple return-if with same value)
    /// Note: This also tests Null literal support from Refactor-A
    ContinueReturnMultiMin,
    /// Parse Array minimal (Phase Next: _parse_array homomorphic fixture)
    ParseArrayMin,
    /// Parse Object minimal (Phase Next: _parse_object homomorphic fixture)
    ParseObjectMin,
}

impl NormalizedDevFixture {
    /// 関数名（allowlist・ルーティング用）
    pub fn function_name(&self) -> &'static str {
        match self {
            Self::Pattern4ContinueMinimal => "pattern4_continue_minimal",
            Self::Pattern4JsonParserParseArrayContinueSkipWs => {
                "jsonparser_parse_array_continue_skip_ws"
            }
            Self::Pattern4JsonParserParseObjectContinueSkipWs => {
                "jsonparser_parse_object_continue_skip_ws"
            }
            Self::PatternContinueReturnMin => "pattern_continue_return_minimal",
            Self::ParseStringCompositeMin => "parse_string_composite_minimal",
            Self::ContinueReturnMultiMin => "continue_return_multi_minimal",
            Self::ParseArrayMin => "parse_array_minimal",
            Self::ParseObjectMin => "parse_object_minimal",
        }
    }

    /// include_str! パス（ドキュメント内 fixture）
    pub fn fixture_path(&self) -> &'static str {
        match self {
            Self::Pattern4ContinueMinimal => {
                "../../../../docs/private/roadmap2/phases/normalized_dev/fixtures/pattern4_continue_min.program.json"
            }
            Self::Pattern4JsonParserParseArrayContinueSkipWs => {
                "../../../../docs/private/roadmap2/phases/normalized_dev/fixtures/jsonparser_parse_array_continue_skip_ws.program.json"
            }
            Self::Pattern4JsonParserParseObjectContinueSkipWs => {
                "../../../../docs/private/roadmap2/phases/normalized_dev/fixtures/jsonparser_parse_object_continue_skip_ws.program.json"
            }
            Self::PatternContinueReturnMin => {
                "../../../../docs/private/roadmap2/phases/normalized_dev/fixtures/pattern_continue_return_min.program.json"
            }
            Self::ParseStringCompositeMin => {
                "../../../../docs/private/roadmap2/phases/normalized_dev/fixtures/parse_string_composite_min.program.json"
            }
            Self::ContinueReturnMultiMin => {
                "../../../../docs/private/roadmap2/phases/normalized_dev/fixtures/continue_return_multi_min.program.json"
            }
            Self::ParseArrayMin => {
                "../../../../docs/private/roadmap2/phases/normalized_dev/fixtures/parse_array_min.program.json"
            }
            Self::ParseObjectMin => {
                "../../../../docs/private/roadmap2/phases/normalized_dev/fixtures/parse_object_min.program.json"
            }
        }
    }

    /// ルーティング先
    pub fn route(&self) -> FunctionRoute {
        match self {
            // すべて LoopFrontend ルーティング
            Self::Pattern4ContinueMinimal
            | Self::Pattern4JsonParserParseArrayContinueSkipWs
            | Self::Pattern4JsonParserParseObjectContinueSkipWs
            | Self::PatternContinueReturnMin
            | Self::ParseStringCompositeMin
            | Self::ContinueReturnMultiMin
            | Self::ParseArrayMin
            | Self::ParseObjectMin => FunctionRoute::LoopFrontend,
        }
    }

    /// fixture の内容文字列を取得（include_str! ラッパー）
    ///
    /// Note: include_str! はコンパイル時展開なので、このメソッドは直接呼ばず
    /// fixtures.rs の各 builder 関数内で使用される想定
    pub fn fixture_content(&self) -> &'static str {
        match self {
            Self::Pattern4ContinueMinimal => include_str!(
                "../../../../docs/private/roadmap2/phases/normalized_dev/fixtures/pattern4_continue_min.program.json"
            ),
            Self::Pattern4JsonParserParseArrayContinueSkipWs => include_str!(
                "../../../../docs/private/roadmap2/phases/normalized_dev/fixtures/jsonparser_parse_array_continue_skip_ws.program.json"
            ),
            Self::Pattern4JsonParserParseObjectContinueSkipWs => include_str!(
                "../../../../docs/private/roadmap2/phases/normalized_dev/fixtures/jsonparser_parse_object_continue_skip_ws.program.json"
            ),
            Self::PatternContinueReturnMin => include_str!(
                "../../../../docs/private/roadmap2/phases/normalized_dev/fixtures/pattern_continue_return_min.program.json"
            ),
            Self::ParseStringCompositeMin => include_str!(
                "../../../../docs/private/roadmap2/phases/normalized_dev/fixtures/parse_string_composite_min.program.json"
            ),
            Self::ContinueReturnMultiMin => include_str!(
                "../../../../docs/private/roadmap2/phases/normalized_dev/fixtures/continue_return_multi_min.program.json"
            ),
            Self::ParseArrayMin => include_str!(
                "../../../../docs/private/roadmap2/phases/normalized_dev/fixtures/parse_array_min.program.json"
            ),
            Self::ParseObjectMin => include_str!(
                "../../../../docs/private/roadmap2/phases/normalized_dev/fixtures/parse_object_min.program.json"
            ),
        }
    }

    /// fixture を読み込んで JoinModule に変換
    pub fn load_and_lower(&self) -> super::super::JoinModule {
        use super::super::frontend::ast_lowerer::AstToJoinIrLowerer;

        let fixture_json = self.fixture_content();
        let program_json: serde_json::Value =
            serde_json::from_str(fixture_json).unwrap_or_else(|e| {
                panic!(
                    "{} fixture should be valid JSON: {}",
                    self.function_name(),
                    e
                )
            });

        let mut lowerer = AstToJoinIrLowerer::new();
        lowerer.lower_program_json(&program_json)
    }
}

/// すべての normalized dev fixtures を列挙
pub const ALL_DEV_FIXTURES: &[NormalizedDevFixture] = &[
    NormalizedDevFixture::Pattern4ContinueMinimal,
    NormalizedDevFixture::Pattern4JsonParserParseArrayContinueSkipWs,
    NormalizedDevFixture::Pattern4JsonParserParseObjectContinueSkipWs,
    NormalizedDevFixture::PatternContinueReturnMin,
    NormalizedDevFixture::ParseStringCompositeMin,
    NormalizedDevFixture::ContinueReturnMultiMin,
    NormalizedDevFixture::ParseArrayMin,
    NormalizedDevFixture::ParseObjectMin,
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_fixtures_have_unique_names() {
        use std::collections::HashSet;
        let names: HashSet<_> = ALL_DEV_FIXTURES.iter().map(|f| f.function_name()).collect();
        assert_eq!(
            names.len(),
            ALL_DEV_FIXTURES.len(),
            "Fixture names must be unique"
        );
    }

    #[test]
    fn test_all_fixtures_have_valid_paths() {
        for fixture in ALL_DEV_FIXTURES {
            let path = fixture.fixture_path();
            assert!(
                path.ends_with(".program.json"),
                "Fixture path must end with .program.json: {}",
                path
            );
            assert!(
                path.contains("normalized_dev/fixtures/"),
                "Fixture path must be in normalized_dev/fixtures/: {}",
                path
            );
        }
    }

    #[test]
    fn test_all_fixtures_route_to_loop_frontend() {
        for fixture in ALL_DEV_FIXTURES {
            assert_eq!(
                fixture.route(),
                FunctionRoute::LoopFrontend,
                "{} should route to LoopFrontend",
                fixture.function_name()
            );
        }
    }
}
