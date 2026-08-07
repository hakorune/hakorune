use crate::ast::{ASTNode, BinaryOperator, DeclarationAttrs, LiteralValue, ParamDecl, Span};
use crate::mir::resolved_semantics::{
    CallableCatalogSealOutcomeV1, VerifiedCallableHeaderSourceUnitV1,
    VerifiedOwnerFreeCallableCatalogSourceUnitV1,
};

use super::callable_single_loop_source_map::{
    issue_callable_single_loop_source_map_v1, CallableSourceMapRejectV1,
};
use super::callable_single_loop_source_shapes::SourceCallKindV1;
use super::callable_single_loop_syntax_facts::issue_callable_single_loop_syntax_facts_v1;
use super::callable_single_loop_syntax_facts::tests as syntax_tests;
use super::resolved_callable_module::VerifiedResolvedCallableModuleV1;

fn variable(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.into(),
        span: Span::unknown(),
    }
}

fn integer(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn function(name: &str, body: Vec<ASTNode>) -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: name.into(),
        params: vec!["n".into()],
        param_decls: vec![ParamDecl {
            name: "n".into(),
            declared_type_name: Some("i64".into()),
        }],
        return_type_name: Some("i64".into()),
        body,
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

fn static_fixture_with_brand(brand: u32) -> VerifiedResolvedCallableModuleV1 {
    let int_to_str = function(
        "int_to_str",
        vec![
            ASTNode::Local {
                variables: vec!["value".into()],
                initial_values: vec![Some(Box::new(ASTNode::FunctionCall {
                    name: "to_i64".into(),
                    arguments: vec![variable("n")],
                    span: Span::unknown(),
                }))],
                declared_type_names: vec![Some("i64".into())],
                span: Span::unknown(),
            },
            ASTNode::Local {
                variables: vec!["i".into()],
                initial_values: vec![Some(Box::new(integer(0)))],
                declared_type_names: vec![Some("i64".into())],
                span: Span::unknown(),
            },
            ASTNode::Loop {
                condition: Box::new(ASTNode::BinaryOp {
                    operator: BinaryOperator::Less,
                    left: Box::new(variable("i")),
                    right: Box::new(integer(1)),
                    span: Span::unknown(),
                }),
                body: vec![ASTNode::Assignment {
                    target: Box::new(variable("i")),
                    value: Box::new(ASTNode::BinaryOp {
                        operator: BinaryOperator::Add,
                        left: Box::new(variable("i")),
                        right: Box::new(integer(1)),
                        span: Span::unknown(),
                    }),
                    span: Span::unknown(),
                }],
                span: Span::unknown(),
            },
            ASTNode::Return {
                value: Some(Box::new(variable("value"))),
                span: Span::unknown(),
            },
        ],
    );
    let to_i64 = function(
        "to_i64",
        vec![ASTNode::Return {
            value: Some(Box::new(variable("n"))),
            span: Span::unknown(),
        }],
    );
    let source = VerifiedCallableHeaderSourceUnitV1::seal_header_surface(ASTNode::Program {
        statements: vec![int_to_str, to_i64],
        span: Span::unknown(),
    })
    .expect("static fixture header source");
    let owner_free = VerifiedOwnerFreeCallableCatalogSourceUnitV1::seal(source)
        .expect("static fixture owner-free catalog");
    let catalog = CallableCatalogSealOutcomeV1::seal(owner_free, brand)
        .expect("static fixture callable catalog");
    VerifiedResolvedCallableModuleV1::resolve(catalog).expect("static fixture resolver")
}

pub(crate) fn static_fixture_for_test() -> VerifiedResolvedCallableModuleV1 {
    static_fixture_with_brand(53)
}

fn facts_for(
    module: &VerifiedResolvedCallableModuleV1,
) -> (
    crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
    super::callable_single_loop_syntax_facts::VerifiedSourceSyntaxFactsV1,
) {
    let header = module
        .source()
        .catalog()
        .index()
        .resolve_free_static_source_call("int_to_str", 1)
        .expect("int_to_str header");
    let input = module
        .function_input(header.source_key())
        .expect("int_to_str input");
    let body = input.source().root_body().expect("int_to_str body");
    let loop_stmt = input.source().body_stmt(&body, 2).expect("loop statement");
    let context = input
        .forest()
        .callable_source_ledger(input.owner())
        .expect("int_to_str ledger")
        .resolved_loop_source(loop_stmt.site())
        .expect("loop context");
    let facts = issue_callable_single_loop_syntax_facts_v1(input, loop_stmt, context)
        .expect("static syntax facts");
    (input, facts)
}

#[test]
fn resolver_backed_free_static_prefix_is_observed_without_target_injection() {
    let module = static_fixture_for_test();
    let (input, facts) = facts_for(&module);

    assert_eq!(facts.prefix().call().kind(), SourceCallKindV1::FreeStatic);
    assert_eq!(facts.prefix().call().argument_count(), 1);

    let targets = input.function().direct_call_targets().collect::<Vec<_>>();
    let [(site, target)] = targets.as_slice() else {
        panic!("expected one resolver-issued static target");
    };
    assert_eq!(**site, *facts.prefix().initializer_site());
    assert_eq!(
        target.callable().owner().compilation_brand(),
        input.owner().compilation_brand()
    );
    assert_ne!(target.callable().owner(), input.owner());
}

#[test]
fn same_brand_static_target_maps_without_owner_identity_equality() {
    let module = static_fixture_for_test();
    let (input, facts) = facts_for(&module);
    let ledger = input
        .forest()
        .callable_source_ledger(input.owner())
        .expect("static fixture ledger");
    let map = issue_callable_single_loop_source_map_v1(&ledger, facts)
        .expect("same-brand static source map");
    let (_, _, Some(mapped_target)) = map.prefix().target().prefix().expect("prefix target") else {
        panic!("expected resolver-issued static target");
    };
    let expected_target = module
        .source()
        .catalog()
        .index()
        .resolve_free_static_source_call("to_i64", 1)
        .expect("to_i64 header")
        .callable();
    assert_eq!(mapped_target, expected_target);
    assert_ne!(mapped_target.owner(), input.owner());
}

#[test]
fn foreign_compilation_brand_rejects_before_source_map_effects() {
    let first = static_fixture_with_brand(53);
    let second = static_fixture_with_brand(54);
    let (_, facts) = facts_for(&first);
    let other_header = second
        .source()
        .catalog()
        .index()
        .resolve_free_static_source_call("int_to_str", 1)
        .expect("foreign int_to_str header");
    let other_input = second
        .function_input(other_header.source_key())
        .expect("foreign static fixture input");
    let other_ledger = other_input
        .forest()
        .callable_source_ledger(other_input.owner())
        .expect("foreign static fixture ledger");
    assert_ne!(
        facts.owner().compilation_brand(),
        other_input.owner().compilation_brand()
    );
    assert_eq!(
        issue_callable_single_loop_source_map_v1(&other_ledger, facts),
        Err(CallableSourceMapRejectV1::ForeignOwner)
    );
}

#[test]
fn existing_method_call_fixture_remains_a_method_shape_negative() {
    let unit = syntax_tests::unit(None, integer(1));
    let (input, loop_stmt, context) = syntax_tests::input_loop_and_context(&unit);
    let facts = issue_callable_single_loop_syntax_facts_v1(input, loop_stmt, context)
        .expect("existing method syntax facts");

    assert!(matches!(
        facts.prefix().call().kind(),
        SourceCallKindV1::Method(_)
    ));
    assert!(input.function().direct_call_targets().next().is_none());
}
