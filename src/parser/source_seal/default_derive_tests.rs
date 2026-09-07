use super::*;
use crate::parser::callable_source_anchor::{
    PreparedCallableSourceV1, PreparedGeneratedCallableSourceV1,
};
use crate::parser::initial_callable_program_source::InitialCallableProgramSourceRejectV1;
use crate::parser::ParserBuildConfig;

fn generated() -> OpenParserPostpassProductV1 {
    crate::test_support::with_env_vars(
        &[
            ("NYASH_MACRO_ENABLE", Some("1")),
            ("NYASH_MACRO_DISABLE", Some("0")),
            ("NYASH_MACRO_DERIVE", None),
            ("NYASH_MACRO_DERIVE_ALL", None),
        ],
        || {
            let mut tokenizer = crate::tokenizer::NyashTokenizer::with_grammar_profile(
                "box Plain {}",
                ParserBuildConfig::default().grammar_profile,
            );
            let mut parser = NyashParser::new(tokenizer.tokenize().unwrap());
            parser.build_config = ParserBuildConfig::default();
            let ast = parser.parse_program().unwrap();
            parser
                .open_postpass_product(ast)
                .unwrap()
                .prune_build_gates(&parser)
                .unwrap()
                .lower_delegates()
                .unwrap()
                .issue_default_derives(Some(&crate::r#macro::NormalMacroPolicyV1::capture()))
                .unwrap()
        },
    )
}

#[test]
fn default_derive_finalization_rejects_body_and_parameter_drift() {
    for change_body in [true, false] {
        let mut product = generated();
        let ASTNode::Program { statements, .. } = &mut product.ast else {
            panic!("program")
        };
        let ASTNode::BoxDeclaration { methods, .. } = &mut statements[0] else {
            panic!("box")
        };
        *methods = methods
            .clone()
            .map_declarations(|mut node| {
                if let ASTNode::FunctionDeclaration {
                    name,
                    body,
                    params,
                    param_decls,
                    ..
                } = &mut node
                {
                    if name == "equals" {
                        if change_body {
                            body.clear();
                        } else {
                            params.clear();
                            param_decls.clear();
                        }
                    }
                }
                node
            })
            .unwrap();
        assert!(matches!(
            product.finalize(),
            Err(SourceSealFinalizationErrorV1::InventoryPrefixMismatch { .. })
        ));
    }
}

#[test]
fn default_derive_finalization_rejects_missing_duplicate_and_foreign_rows() {
    let mut missing = generated();
    missing.source_session.callable_rows.pop().unwrap();
    assert!(matches!(
        missing.finalize(),
        Err(SourceSealFinalizationErrorV1::InitialCallableProgramSource(
            _
        ))
    ));

    let mut duplicate = generated();
    let row = duplicate.source_session.callable_rows[0]
        .generated()
        .unwrap();
    let extra = PreparedGeneratedCallableSourceV1::issue(
        row.parser_brand().clone(),
        row.origin().clone(),
        row.diagnostic_name(),
    );
    duplicate
        .source_session
        .callable_rows
        .push(PreparedCallableSourceV1::Generated(extra));
    assert!(matches!(
        duplicate.finalize(),
        Err(SourceSealFinalizationErrorV1::InitialCallableProgramSource(
            InitialCallableProgramSourceRejectV1::DuplicateFinalCallableSlot
        ))
    ));

    let mut foreign = generated();
    foreign.source_session.callable_rows[0] = generated().source_session.callable_rows.remove(0);
    assert!(matches!(
        foreign.finalize(),
        Err(SourceSealFinalizationErrorV1::InitialCallableProgramSource(
            InitialCallableProgramSourceRejectV1::ForeignParser
        ))
    ));
}

#[test]
fn default_derive_finalization_rejects_unissued_extra_declaration() {
    let mut product = generated();
    let ASTNode::Program { statements, .. } = &mut product.ast else {
        panic!("program")
    };
    let ASTNode::BoxDeclaration { methods, .. } = &mut statements[0] else {
        panic!("box")
    };
    let mut extra = methods.get_declaration("equals").unwrap().clone();
    let ASTNode::FunctionDeclaration { name, .. } = &mut extra else {
        panic!("method")
    };
    *name = "unissued".into();
    methods
        .try_push_generated(
            "unissued",
            extra,
            BoxMethodGeneratedProvenanceV1::MacroOrImport {
                generator: "macro-derive-equals".into(),
            },
            crate::ast::Span::unknown(),
        )
        .unwrap();
    assert!(matches!(
        product.finalize(),
        Err(SourceSealFinalizationErrorV1::UnexpectedGeneratedRow { .. })
    ));
}

#[test]
fn default_derive_finalization_rejects_wrong_placement() {
    use crate::parser::callable_source_anchor::GeneratedCallableOriginV1;
    use crate::parser::default_derive_source::GeneratedDefaultDeriveOriginV1;
    let mut product = generated();
    let row = product.source_session.callable_rows[0].generated().unwrap();
    let GeneratedCallableOriginV1::DefaultDerive(origin) = row.origin() else {
        panic!("origin")
    };
    let GeneratedCallableOriginV1::DefaultDerive(other) = product.source_session.callable_rows[1]
        .generated()
        .unwrap()
        .origin()
    else {
        panic!("origin")
    };
    let ASTNode::Program { statements, .. } = &product.ast else {
        panic!("program")
    };
    let ASTNode::BoxDeclaration { methods, .. } = &statements[0] else {
        panic!("box")
    };
    let wrong = GeneratedDefaultDeriveOriginV1::issue(
        origin.parent().clone(),
        origin.kind(),
        other.placement(),
        methods.get_declaration("equals").unwrap(),
    )
    .unwrap();
    product.source_session.callable_rows[0] =
        PreparedCallableSourceV1::Generated(PreparedGeneratedCallableSourceV1::issue(
            row.parser_brand().clone(),
            GeneratedCallableOriginV1::DefaultDerive(wrong),
            row.diagnostic_name(),
        ));
    assert!(matches!(
        product.finalize(),
        Err(SourceSealFinalizationErrorV1::InitialCallableProgramSource(
            _
        ))
    ));
}
