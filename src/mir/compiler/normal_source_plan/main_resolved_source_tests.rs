use super::*;
use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, Span};
use crate::mir::compiler::normal_source_plan::{
    NormalSourcePlanClassifierV1, PreparedNormalSourcePlanInputV1, SealedNormalScalarRootV1,
    SealedNormalSourcePlanV1,
};
use std::collections::HashMap;

fn literal(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn main_program(body: Vec<ASTNode>) -> ASTNode {
    let function = ASTNode::FunctionDeclaration {
        name: "main".to_owned(),
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
    methods.insert("main".to_owned(), function);
    ASTNode::Program {
        statements: vec![ASTNode::BoxDeclaration {
            name: "Main".to_owned(),
            fields: vec!["retained".to_owned()],
            field_decls: Vec::new(),
            public_fields: Vec::new(),
            private_fields: Vec::new(),
            methods,
            constructors: HashMap::new(),
            init_fields: Vec::new(),
            weak_fields: Vec::new(),
            delegates: Vec::new(),
            invariants: Vec::new(),
            transitions: Vec::new(),
            is_interface: false,
            is_sync: false,
            is_record: false,
            type_parameters: Vec::new(),
            extends: Vec::new(),
            implements: Vec::new(),
            is_static: true,
            static_init: None,
            attrs: DeclarationAttrs::default(),
            span: Span::unknown(),
        }],
        span: Span::unknown(),
    }
}

fn resolved_main(body: Vec<ASTNode>) -> VerifiedNormalMainResolvedSourceUnitV1 {
    let input =
        PreparedNormalSourcePlanInputV1::new(main_program(body), "main-resolved-source-test");
    let plan = NormalSourcePlanClassifierV1::seal(input).expect("valid Main0");
    let SealedNormalSourcePlanV1::ScalarRoot(SealedNormalScalarRootV1::Main0(main)) = plan else {
        panic!("expected Main0");
    };
    main.prepare_function_source()
        .expect("exact Main source")
        .prepare_embedded_resolved_main()
        .expect("embedded resolution")
}

#[test]
fn embedded_main_resolution_keeps_exact_program_owned_function_identity() {
    let resolved = resolved_main(vec![literal(7)]);
    let source_ptr = resolved.source_function_for_test() as *const ASTNode;
    let input = resolved.borrow_function_input().expect("function input");

    assert_eq!(source_ptr, input.source().root() as *const ASTNode);
    assert_eq!(input.owner(), input.function().owner());
    assert_eq!(input.owner(), input.source().owner());
    assert_eq!(resolved.role(), VerifiedNormalMainRoleV1::seal());
}

#[test]
fn embedded_main_resolution_reuses_nested_owner_forest_and_source_projection() {
    let lambda = ASTNode::Lambda {
        params: Vec::new(),
        body: vec![literal(1)],
        span: Span::unknown(),
    };
    let resolved = resolved_main(vec![ASTNode::Local {
        variables: vec!["f".to_owned()],
        initial_values: vec![Some(Box::new(lambda))],
        declared_type_names: vec![None],
        span: Span::unknown(),
    }]);
    let input = resolved.borrow_function_input().expect("function input");

    assert_eq!(input.forest().owner_count(), 2);
    assert_eq!(input.source().root().node_type(), "FunctionDeclaration");
}
