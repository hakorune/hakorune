use super::*;
use crate::parser::{NyashParser, ParserBuildConfig};

#[test]
fn default_derive_origin_rejects_wrong_kind_arity() {
    let parsed = NyashParser::parse_from_string_with_source_seal(
        "box Plain {}",
        ParserBuildConfig::default(),
    )
    .unwrap();
    let seal = &parsed.source_seals()[0];
    for kind in [DefaultDeriveKindV1::Equals, DefaultDeriveKindV1::ToString] {
        let syntax = kind.build("Plain", &Vec::new());
        let mut inventory = seal.inventory().clone();
        let placement = inventory
            .try_push_generated(
                kind.name(),
                syntax.clone(),
                kind.provenance(),
                crate::ast::Span::unknown(),
            )
            .unwrap();
        let origin = GeneratedDefaultDeriveOriginV1::issue(
            seal.box_site().clone(),
            kind,
            placement,
            &syntax,
        )
        .unwrap();
        assert!(origin.validates(&syntax));
        let mut wrong = syntax;
        let opposite = match kind {
            DefaultDeriveKindV1::Equals => DefaultDeriveKindV1::ToString,
            DefaultDeriveKindV1::ToString => DefaultDeriveKindV1::Equals,
        }
        .build("Plain", &Vec::new());
        let ASTNode::FunctionDeclaration {
            params: other_params,
            param_decls: other_decls,
            ..
        } = opposite
        else {
            panic!("declaration")
        };
        let ASTNode::FunctionDeclaration {
            params,
            param_decls,
            ..
        } = &mut wrong
        else {
            panic!("declaration")
        };
        *params = other_params;
        *param_decls = other_decls;
        assert!(!origin.validates(&wrong));
        assert!(GeneratedDefaultDeriveOriginV1::issue(
            seal.box_site().clone(),
            kind,
            placement,
            &wrong,
        )
        .is_err());
    }
}
