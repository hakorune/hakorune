//! Shared public parser projections owned by the total postpass coordinator.
//!
//! String/build-config, instance-parser, and metadata callers all use the same
//! postpass finalizer. Explain-report capture remains a separate I0-C contract.

use crate::ast::ASTNode;
use crate::tokenizer::{NyashTokenizer, TokenType};

use super::callable_parameter_source::ParsedProgramWithCallableParameterSourceV1;
use super::postpass_envelope::{ExplainDemandV1, PostpassDemandV1};
use super::ParserMetadata;
use super::{normalize_logical_ops, NyashParser, ParseError, ParserBuildConfig};

pub(super) fn parse(
    input: String,
    fuel: Option<usize>,
    build_config: ParserBuildConfig,
) -> Result<ASTNode, ParseError> {
    let mut parser = parser_from_string(input, fuel, build_config)?;
    let completed = parser.parse_postpass_s0()?;
    Ok(completed.into_ast())
}

pub(crate) fn parse_postpass(
    input: String,
    fuel: Option<usize>,
    build_config: ParserBuildConfig,
) -> Result<super::postpass_envelope::CompletedParserPostpassV1, ParseError> {
    let mut parser = parser_from_string(input, fuel, build_config)?;
    parser.parse_postpass_s0()
}

pub(super) fn parse_normal_callable_program(
    input: String,
    fuel: Option<usize>,
    build_config: ParserBuildConfig,
) -> Result<super::normal_callable_program_source::ParsedNormalCallableProgramV1, ParseError> {
    let mut parser = parser_from_string(input, fuel, build_config)?;
    let completed = parser.parse_postpass_s0()?;
    let parameter_source = parser.finish_callable_parameter_source_for_normal()?;
    completed
        .into_normal_callable_program(parameter_source)
        .map_err(|error| ParseError::GrammarContract {
            stable_reject_tag: "parser/normal-callable-parameter-source",
            detail: format!("normal callable parameter source rejected: {error:?}"),
            line: 0,
        })
}

pub(super) fn parse_with_callable_parameter_source(
    input: String,
    fuel: Option<usize>,
    build_config: ParserBuildConfig,
) -> Result<ParsedProgramWithCallableParameterSourceV1, ParseError> {
    let mut parser = parser_from_string(input, fuel, build_config)?;
    let completed = parser.parse_postpass_s0()?;
    let catalog = parser.finish_callable_parameter_source_catalog()?;
    Ok(ParsedProgramWithCallableParameterSourceV1::new(
        completed, catalog,
    ))
}

pub(super) fn parse_existing(parser: &mut NyashParser) -> Result<ASTNode, ParseError> {
    Ok(parser.parse_postpass_s0()?.into_ast())
}

pub(super) fn parse_with_explain(
    input: String,
    fuel: Option<usize>,
    build_config: ParserBuildConfig,
) -> Result<(ASTNode, super::BuildGateExplainReport), ParseError> {
    let mut parser = parser_from_string(input, fuel, build_config)?;
    parser
        .parse_postpass_with_demand(PostpassDemandV1 {
            explain: ExplainDemandV1::Capture,
        })?
        .into_ast_and_explain()
        .map_err(|error| error.into_parse_error())
}

fn parser_from_string(
    input: String,
    fuel: Option<usize>,
    build_config: ParserBuildConfig,
) -> Result<NyashParser, ParseError> {
    let preprocessed = normalize_logical_ops(&input);
    let mut tokenizer =
        NyashTokenizer::with_grammar_profile(preprocessed, build_config.grammar_profile);
    let tokens = tokenizer.tokenize()?;
    reject_unsupported_self_identifier(&tokens)?;

    let mut parser = NyashParser::new(tokens);
    parser.debug_fuel = fuel;
    parser.build_config = build_config;
    Ok(parser)
}

pub(super) fn parse_with_metadata(
    parser: &mut NyashParser,
) -> Result<(ASTNode, ParserMetadata), ParseError> {
    Ok(parser.parse_postpass_s0()?.into_ast_and_metadata())
}

fn reject_unsupported_self_identifier(
    tokens: &[crate::tokenizer::Token],
) -> Result<(), ParseError> {
    for token in tokens {
        if let TokenType::IDENTIFIER(name) = &token.token_type {
            if name == "self" {
                return Err(ParseError::UnsupportedIdentifier {
                    name: name.clone(),
                    line: token.line,
                });
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn i0_a_ordinary_source_uses_total_postpass_edge() {
        let ast = parse(
            "box Plain { run() { return 1 } }\n".to_owned(),
            Some(100_000),
            ParserBuildConfig::default(),
        )
        .unwrap();
        assert!(matches!(ast, ASTNode::Program { .. }));
    }

    #[test]
    fn i0_a_compatibility_source_remains_successful_ast_transport() {
        let ast = parse(
            "static box StaticOnly {}\n".to_owned(),
            Some(100_000),
            ParserBuildConfig::default(),
        )
        .unwrap();
        assert!(matches!(ast, ASTNode::Program { .. }));
    }

    #[test]
    fn i0_a_interface_record_and_mixed_cohorts_remain_ast_transport() {
        for source in [
            "interface box Api { run() }\n",
            "record Data { value: i64 }\n",
            "box Plain {}\nstatic box StaticOnly {}\n",
        ] {
            let ast = parse(
                source.to_owned(),
                Some(100_000),
                ParserBuildConfig::default(),
            )
            .unwrap();
            assert!(matches!(ast, ASTNode::Program { .. }));
        }
    }

    #[test]
    fn i0_a_fuel_none_and_selected_gate_keep_single_invocation_contract() {
        let ast = parse(
            "gate Build.test { box Enabled {} } else { box Disabled {} }\n".to_owned(),
            None,
            ParserBuildConfig {
                mode: super::super::BuildMode::Test,
                ..ParserBuildConfig::default()
            },
        )
        .unwrap();
        let ASTNode::Program { statements, .. } = ast else {
            panic!("expected Program");
        };
        assert_eq!(statements.len(), 1);
        assert!(matches!(statements[0], ASTNode::BoxDeclaration { .. }));
    }

    #[test]
    fn i0_a_self_identifier_keeps_existing_diagnostic() {
        let error = parse(
            "box Bad { run() { return self } }\n".to_owned(),
            Some(100_000),
            ParserBuildConfig::default(),
        )
        .unwrap_err();
        assert!(matches!(error, ParseError::UnsupportedIdentifier { .. }));
    }

    #[test]
    fn i0_b_existing_parser_instance_uses_the_same_postpass_entry() {
        let tokens = NyashTokenizer::new("box Existing { run() { return 1 } }\n")
            .tokenize()
            .unwrap();
        let mut parser = NyashParser::new(tokens);
        let ast = parse_existing(&mut parser).unwrap();
        assert!(matches!(ast, ASTNode::Program { .. }));
    }

    #[test]
    fn i0_b_metadata_projection_moves_the_completed_sidecar_once() {
        let tokens =
            NyashTokenizer::new("static box Main { @rune Hint(inline) main() { return 0 } }\n")
                .tokenize()
                .unwrap();
        let mut parser = NyashParser::new(tokens);
        let (ast, metadata) = parse_with_metadata(&mut parser).unwrap();
        assert!(matches!(ast, ASTNode::Program { .. }));
        assert_eq!(metadata.runes.len(), 1);
        assert_eq!(metadata.runes[0].name, "Hint");
    }
}
