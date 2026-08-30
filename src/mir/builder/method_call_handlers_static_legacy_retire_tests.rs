use super::*;
use crate::ast::{ASTNode, LiteralValue, Span};
use crate::mir::builder::callable_declaration_catalog::VerifiedSameModuleCallableDeclarationCatalogV1;
use crate::mir::builder::calls::{AssociatedMethodCallArgumentsV1, RawLegacyMethodCallInputV1};
use crate::mir::builder::recursive_child_lowering::RawLegacyChildLoweringPortV1;
use crate::parser::NyashParser;

fn integer(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn variable(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.to_owned(),
        span: Span::unknown(),
    }
}

fn instruction_count(builder: &MirBuilder) -> usize {
    builder
        .function_state
        .current_function
        .as_ref()
        .unwrap()
        .blocks
        .values()
        .map(|block| block.instructions.len())
        .sum()
}

fn builder(name: &str) -> MirBuilder {
    let mut builder = MirBuilder::new();
    let root =
        NyashParser::parse_from_string("static box RouteCatalogSentinel { noop() { return 0 } }")
            .unwrap();
    let catalog = VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&root).unwrap();
    builder
        .comp_ctx
        .install_callable_declaration_catalog(catalog)
        .unwrap();
    builder.enter_function_for_test(name.to_owned());
    builder
}

fn input(receiver: ASTNode, method: &str, arguments: Vec<ASTNode>) -> RawLegacyMethodCallInputV1 {
    RawLegacyMethodCallInputV1::new(receiver, method.to_owned(), arguments)
}

#[test]
fn unissued_static_route_retires_before_argument_descent() {
    let call = input(
        variable("LegacyStatic"),
        "run",
        vec![integer(1), integer(2)],
    );
    let mut port = RawLegacyChildLoweringPortV1;
    let mut builder = builder("legacy_static_route/0");
    let before = instruction_count(&builder);

    let error = builder
        .build_method_call_from_input_v1(&mut port, &call)
        .expect_err("unissued generic static compatibility must retire");

    assert!(error.starts_with("[freeze:contract][static-call/legacy-fallback-retired]"));
    assert_eq!(instruction_count(&builder), before);
}

#[test]
fn qualified_math_static_route_keeps_compatibility_owner() {
    let call = input(variable("Math"), "abs", vec![integer(-1)]);
    let mut port = RawLegacyChildLoweringPortV1;
    let mut builder = builder("math_static_route/0");
    let before = instruction_count(&builder);

    builder
        .build_method_call_from_input_v1(&mut port, &call)
        .expect("qualified Math remains on its existing compatibility owner");

    assert!(instruction_count(&builder) > before);
}

#[test]
fn static_this_retires_before_argument_descent() {
    let call = input(
        ASTNode::This {
            span: Span::unknown(),
        },
        "run",
        vec![integer(1)],
    );
    let mut port = RawLegacyChildLoweringPortV1;
    let mut builder = builder("legacy_static_this/0");
    builder.comp_ctx.current_static_box = Some("LegacyStatic".to_owned());
    let before = instruction_count(&builder);

    let error = builder
        .build_method_call_from_input_v1(&mut port, &call)
        .expect_err("static this without an exact issuer must retire");

    assert!(error.starts_with("[freeze:contract][static-call/legacy-fallback-retired]"));
    assert_eq!(instruction_count(&builder), before);
}

#[test]
fn me_static_fallback_retires_before_argument_descent() {
    let call = input(
        ASTNode::Me {
            span: Span::unknown(),
        },
        "run",
        vec![integer(1)],
    );
    let mut port = RawLegacyChildLoweringPortV1;
    let mut builder = builder("LegacyStatic.caller/0");
    let before = instruction_count(&builder);

    let error = builder
        .build_method_call_from_input_v1(&mut port, &call)
        .expect_err("receiverless me fallback must retire");

    assert!(error.starts_with("[freeze:contract][static-call/legacy-fallback-retired]"));
    assert_eq!(instruction_count(&builder), before);
}

#[test]
fn lowered_global_static_retires_before_argument_descent() {
    let call = input(
        ASTNode::Me {
            span: Span::unknown(),
        },
        "run",
        vec![integer(1)],
    );
    let mut port = RawLegacyChildLoweringPortV1;
    let mut builder = builder("LegacyStatic.caller/0");
    let mut descent = AssociatedMethodCallArgumentsV1::new(&mut port, &call);
    let prepared = PreparedMeCallExecutionV1::LoweredGlobal {
        owner: "LegacyStatic".to_owned(),
        prepared: crate::mir::builder::me_call_header_observation::PreparedMeLoweredCallV1::from_test_parts(
            1,
            PreparedMeReceiverV1::Static,
        ),
    };
    let before = instruction_count(&builder);

    let error =
        MeCallPolicyBox::execute(&mut builder, "run", &[integer(1)], &mut descent, prepared)
            .expect_err("unissued lowered static must retire before descent");

    assert!(error.starts_with("[freeze:contract][static-call/legacy-fallback-retired]"));
    assert_eq!(instruction_count(&builder), before);
}
