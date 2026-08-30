//! Focused ingress tests for the retired Builder operator-call selectors.
//!
//! These tests deliberately exercise the public compiler seams rather than
//! the parser helper alone.  A rejected selector must leave the compiler
//! reusable and must not open a Builder module before the terminal.

use super::{MirCompiler, NormalCompileRequestV1, VerifiedResolvedSourceUnitV1};
use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, Span};

const SELECTORS: [(&str, Option<&str>); 3] = [
    ("NYASH_BUILDER_OPERATOR_BOX_ALL_CALL", None),
    ("NYASH_BUILDER_OPERATOR_BOX_ADD_CALL", None),
    ("NYASH_BUILDER_OPERATOR_BOX_COMPARE_CALL", None),
];

fn empty_program() -> ASTNode {
    ASTNode::Program {
        statements: Vec::new(),
        span: Span::unknown(),
    }
}

fn resolved_function() -> VerifiedResolvedSourceUnitV1 {
    VerifiedResolvedSourceUnitV1::resolve_function(ASTNode::FunctionDeclaration {
        name: "operator_retirement_fixture".into(),
        params: Vec::new(),
        param_decls: Vec::new(),
        return_type_name: None,
        body: vec![ASTNode::Return {
            value: Some(Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(1),
                span: Span::unknown(),
            })),
            span: Span::unknown(),
        }],
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    })
    .expect("operator retirement fixture resolves")
}

fn with_selectors(all: Option<&str>, add: Option<&str>, compare: Option<&str>, f: impl FnOnce()) {
    crate::test_support::with_env_vars(
        &[
            (SELECTORS[0].0, all),
            (SELECTORS[1].0, add),
            (SELECTORS[2].0, compare),
        ],
        f,
    );
}

#[test]
fn normal_ingress_rejects_retired_selector_before_builder_effects_and_reuses() {
    let mut compiler = MirCompiler::new();
    with_selectors(Some("1"), None, None, || {
        let request = NormalCompileRequestV1::for_mir_mode(
            empty_program(),
            Some("operator-retired.hako"),
            std::collections::HashMap::new(),
        )
        .expect("empty Program request");
        let error = compiler
            .compile_normal(request)
            .expect_err("retired Builder selector must fail closed");
        assert!(error.contains("[mir/operator-call/retired]"));
        assert!(compiler.builder.current_module.is_none());
    });
    with_selectors(None, None, None, || {
        let request = NormalCompileRequestV1::for_mir_mode(
            empty_program(),
            Some("operator-reuse.hako"),
            std::collections::HashMap::new(),
        )
        .expect("empty Program request");
        compiler
            .compile_normal(request)
            .expect("compiler is reusable after ingress rejection");
        assert!(compiler.builder.current_module.is_none());
    });
}

#[test]
fn raw_ingress_rejects_malformed_selector_before_source_binding() {
    with_selectors(None, Some(" 1 "), None, || {
        let mut compiler = MirCompiler::new();
        let error = compiler
            .compile_raw_with_source(empty_program(), Some("operator-invalid.hako"))
            .expect_err("malformed Builder selector must fail closed");
        assert!(error.contains("operator-call-ingress-rejected"));
        assert!(compiler.builder.current_module.is_none());
    });
}

#[test]
fn resolved_ingress_rejects_non_unicode_selector_before_candidate_open() {
    #[cfg(unix)]
    {
        use std::ffi::OsString;
        use std::os::unix::ffi::OsStringExt;

        let raw = OsString::from_vec(vec![0xff]);
        crate::test_support::with_env_vars(
            &[
                (SELECTORS[0].0, None),
                (SELECTORS[1].0, None),
                (SELECTORS[2].0, None),
            ],
            || {
                std::env::set_var(SELECTORS[2].0, &raw);
                let unit = resolved_function();
                let mut compiler = MirCompiler::new();
                let error = compiler
                    .compile_resolved(unit.lowering_input(), Some("operator-nonunicode.hako"))
                    .expect_err("non-Unicode selector must fail closed");
                assert!(error.to_string().contains("operator-call/non-unicode"));
                assert!(compiler.builder.current_module.is_none());
            },
        );
    }
}

#[test]
fn unset_selectors_keep_the_direct_compiler_route() {
    with_selectors(None, None, None, || {
        let mut compiler = MirCompiler::new();
        let request = NormalCompileRequestV1::for_mir_mode(
            empty_program(),
            Some("operator-direct.hako"),
            std::collections::HashMap::new(),
        )
        .expect("empty Program request");
        compiler
            .compile_normal(request)
            .expect("unset selectors retain direct MIR ingress");
        assert!(compiler.builder.current_module.is_none());
    });
}
