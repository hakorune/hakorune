//! String/build-config parser entry owned by the total postpass coordinator.
//!
//! This module contains only the selected public AST edge. Grammar-evidence,
//! metadata, `NyashParser::parse`, and explain-report entries remain separate
//! contracts until their own rows are opened.

use crate::ast::ASTNode;
use crate::tokenizer::{NyashTokenizer, TokenType};

use super::postpass_envelope::PostpassDemandV1;
use super::{normalize_logical_ops, NyashParser, ParseError, ParserBuildConfig};

pub(super) fn parse(
    input: String,
    fuel: Option<usize>,
    build_config: ParserBuildConfig,
) -> Result<ASTNode, ParseError> {
    let preprocessed = normalize_logical_ops(&input);
    let mut tokenizer =
        NyashTokenizer::with_grammar_profile(preprocessed, build_config.grammar_profile);
    let tokens = tokenizer.tokenize()?;
    reject_unsupported_self_identifier(&tokens)?;

    let mut parser = NyashParser::new(tokens);
    parser.debug_fuel = fuel;
    parser.build_config = build_config;
    let ast = parser.parse_program()?;
    let product = parser.open_postpass_product(ast);
    let completed = product.finish_total_s0(&parser, PostpassDemandV1::default())?;
    Ok(completed.into_ast())
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
}
