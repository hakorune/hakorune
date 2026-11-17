//! PARITY0-S0a: the first bounded Legacy-vs-Raw success witness.

use super::MirCompiler;
use crate::ast::{ASTNode, BinaryOperator, DeclarationAttrs, LiteralValue, Span};
use std::collections::HashMap;

fn empty_script() -> ASTNode {
    ASTNode::Program {
        statements: Vec::new(),
        span: Span::unknown(),
    }
}

fn literal_script(value: LiteralValue) -> ASTNode {
    ASTNode::Program {
        statements: vec![ASTNode::Literal {
            value,
            span: Span::unknown(),
        }],
        span: Span::unknown(),
    }
}

fn binary_script(left: LiteralValue, operator: BinaryOperator, right: LiteralValue) -> ASTNode {
    ASTNode::Program {
        statements: vec![ASTNode::BinaryOp {
            operator,
            left: Box::new(ASTNode::Literal {
                value: left,
                span: Span::unknown(),
            }),
            right: Box::new(ASTNode::Literal {
                value: right,
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }],
        span: Span::unknown(),
    }
}

fn app_with_body(body: Vec<ASTNode>) -> ASTNode {
    let main = ASTNode::FunctionDeclaration {
        name: "main".into(),
        params: Vec::new(),
        param_decls: Vec::new(),
        return_type_name: None,
        body,
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    };
    let mut methods = HashMap::new();
    methods.insert("main".into(), main);
    ASTNode::Program {
        statements: vec![ASTNode::BoxDeclaration {
            name: "Main".into(),
            fields: Vec::new(),
            field_decls: Vec::new(),
            public_fields: Vec::new(),
            private_fields: Vec::new(),
            methods: crate::ast::BoxMethodInventoryV1::from_legacy_ast_map(methods),
            constructors: HashMap::new(),
            init_fields: Vec::new(),
            weak_fields: Vec::new(),
            delegates: Vec::new(),
            invariants: Vec::new(),
            transitions: Vec::new(),
            is_interface: false,
            is_record: false,
            extends: Vec::new(),
            implements: Vec::new(),
            type_parameters: Vec::new(),
            is_sync: false,
            is_static: true,
            static_init: None,
            attrs: DeclarationAttrs::default(),
            span: Span::unknown(),
        }],
        span: Span::unknown(),
    }
}

fn empty_app() -> ASTNode {
    app_with_body(Vec::new())
}

fn scalar_app() -> ASTNode {
    app_with_body(vec![ASTNode::Literal {
        value: LiteralValue::Integer(9),
        span: Span::unknown(),
    }])
}

/// Raw and legacy compiler entry points recurse deeply enough in debug
/// builds to sit near the default 8 MiB test-thread stack boundary; run
/// these pins on a widened thread the same way the callable pipeline loop
/// pins do.
fn run_on_test_thread(name: &str, body: impl FnOnce() + Send + 'static) {
    std::thread::Builder::new()
        .name(name.to_owned())
        .stack_size(32 * 1024 * 1024)
        .spawn(body)
        .expect("spawn test thread")
        .join()
        .expect("test thread panicked");
}





#[test]
fn empty_app_raw_root_main_keeps_fixed_void_contract() {
    run_on_test_thread("raw-empty-app-void-contract", || {
        let mut raw = MirCompiler::new();
        let result = raw
            .compile_raw_with_source(empty_app(), Some("parity-app.hako"))
            .expect("Raw empty App should compile");
        assert_eq!(
            result.module.functions["main"].signature.return_type,
            crate::mir::MirType::Void
        );
    });
}

#[test]
fn scalar_app_raw_root_main_discards_tail_but_keeps_fixed_void_contract() {
    run_on_test_thread("raw-scalar-app-void-contract", || {
        let mut raw = MirCompiler::new();
        let result = raw
            .compile_raw_with_source(scalar_app(), Some("parity-app-scalar.hako"))
            .expect("Raw scalar App should complete BODY and ROOTBATCH");
        assert_eq!(
            result.module.functions["main"].signature.return_type,
            crate::mir::MirType::Void
        );
    });
}
