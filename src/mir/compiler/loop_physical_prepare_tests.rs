//! Focused evidence for the caller-zero Loop physical preparation boundary.
//!
//! The logical `loop_physical_prepare::tests` module stays unchanged while
//! its test-only body lives outside the production-shaped source file.

use super::*;
use crate::ast::{ASTNode, DeclarationAttrs, ParamDecl, Span};
use crate::mir::compiler::callable_single_loop_recipe_coseal::issue_callable_single_loop_recipe_v1;
use crate::mir::compiler::callable_single_loop_source_map::issue_callable_single_loop_source_map_v1;
use crate::mir::compiler::callable_single_loop_source_shapes::SourceReceiverShapeV1;
use crate::mir::compiler::callable_single_loop_static_fixture_tests::static_fixture_for_test;
use crate::mir::compiler::callable_single_loop_syntax_facts::issue_callable_single_loop_syntax_facts_v1;
use crate::mir::compiler::callable_single_loop_syntax_facts::tests::{
    input_loop_and_context, unit,
};
use crate::mir::compiler::resolved_callable_module::VerifiedResolvedCallableModuleV1;
use crate::mir::resolved_control_flow::verify_function_completion_v1;
use crate::mir::resolved_semantics::{
    CallableCatalogSealOutcomeV1, CallableSemanticSourceLedgerView, CanonicalCallableKeyV1,
    ExprChildRoleV1, OwnedExprSiteV1, VerifiedCallableHeaderSourceUnitV1,
    VerifiedOwnerFreeCallableCatalogSourceUnitV1,
};

fn variable(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.into(),
        span: Span::unknown(),
    }
}

fn scalar_function(name: &str, params: &[&str]) -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: name.into(),
        params: params.iter().map(|name| (*name).into()).collect(),
        param_decls: params
            .iter()
            .map(|name| ParamDecl {
                name: (*name).into(),
                declared_type_name: Some("i64".into()),
            })
            .collect(),
        return_type_name: Some("i64".into()),
        body: vec![ASTNode::Return {
            value: Some(Box::new(variable(params[0]))),
            span: Span::unknown(),
        }],
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

fn loop_function() -> ASTNode {
    let integer = |value: i64| ASTNode::Literal {
        value: crate::ast::LiteralValue::Integer(value),
        span: Span::unknown(),
    };
    let assignment = |name: &str, value: ASTNode| ASTNode::Assignment {
        target: Box::new(variable(name)),
        value: Box::new(value),
        span: Span::unknown(),
    };
    ASTNode::FunctionDeclaration {
        name: "int_to_str".into(),
        params: vec!["n".into(), "helper".into()],
        param_decls: vec![
            ParamDecl {
                name: "n".into(),
                declared_type_name: Some("i64".into()),
            },
            ParamDecl {
                name: "helper".into(),
                declared_type_name: Some("i64".into()),
            },
        ],
        return_type_name: Some("i64".into()),
        body: vec![
            ASTNode::Local {
                variables: vec!["value".into()],
                initial_values: vec![Some(Box::new(ASTNode::MethodCall {
                    object: Box::new(variable("helper")),
                    method: "to_i64".into(),
                    arguments: vec![variable("n")],
                    span: Span::unknown(),
                }))],
                declared_type_names: vec![None],
                span: Span::unknown(),
            },
            ASTNode::Local {
                variables: vec!["i".into()],
                initial_values: vec![Some(Box::new(integer(0)))],
                declared_type_names: vec![None],
                span: Span::unknown(),
            },
            ASTNode::Loop {
                condition: Box::new(ASTNode::BinaryOp {
                    operator: crate::ast::BinaryOperator::Less,
                    left: Box::new(variable("i")),
                    right: Box::new(integer(1)),
                    span: Span::unknown(),
                }),
                body: vec![assignment(
                    "i",
                    ASTNode::BinaryOp {
                        operator: crate::ast::BinaryOperator::Add,
                        left: Box::new(variable("i")),
                        right: Box::new(integer(1)),
                        span: Span::unknown(),
                    },
                )],
                span: Span::unknown(),
            },
            ASTNode::Return {
                value: Some(Box::new(variable("value"))),
                span: Span::unknown(),
            },
        ],
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

fn loop_module() -> VerifiedResolvedCallableModuleV1 {
    let source = VerifiedCallableHeaderSourceUnitV1::seal_header_surface(ASTNode::Program {
        statements: vec![scalar_function("helper", &["n"]), loop_function()],
        span: Span::unknown(),
    })
    .unwrap();
    let owner_free = VerifiedOwnerFreeCallableCatalogSourceUnitV1::seal(source).unwrap();
    let catalog = CallableCatalogSealOutcomeV1::seal(owner_free, 41).unwrap();
    VerifiedResolvedCallableModuleV1::resolve(catalog).unwrap()
}

fn loop_product<'a>(
    input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'a>,
) -> (
    CallableSemanticSourceLedgerView<'a>,
    super::super::callable_single_loop_recipe_coseal::VerifiedCallableSingleLoopRecipeProductV1,
) {
    let body = input.source().root_body().unwrap();
    let loop_stmt = input.source().body_stmt(&body, 2).unwrap();
    let ledger = input
        .forest()
        .callable_source_ledger(input.owner())
        .unwrap();
    let context = ledger.resolved_loop_source(loop_stmt.site()).unwrap();
    let syntax = issue_callable_single_loop_syntax_facts_v1(input, loop_stmt, context).unwrap();
    let map = issue_callable_single_loop_source_map_v1(&ledger, syntax).unwrap();
    let product = issue_callable_single_loop_recipe_v1(&ledger, map).unwrap();
    (ledger, product)
}

#[test]
fn demand_owns_the_co_seal_after_source_views_are_dropped() {
    let demand = {
        let unit = unit(
            None,
            ASTNode::Literal {
                value: crate::ast::LiteralValue::Integer(1),
                span: Span::unknown(),
            },
        );
        let (input, _, _) = input_loop_and_context(&unit);
        let (_, product) = loop_product(input);
        let (co_seal, _, _) = product.into_parts();
        VerifiedLoopPhysicalDemandV1::issue(co_seal)
    };
    assert_eq!(demand.co_seal().operations().len(), 7);
    assert_eq!(demand.co_seal().continuation().loop_key().raw(), 0);
}

#[test]
fn input_brand_rejects_a_root_view_before_any_product_is_opened() {
    let unit = unit(
        None,
        ASTNode::Literal {
            value: crate::ast::LiteralValue::Integer(1),
            span: Span::unknown(),
        },
    );
    let input = unit.root_function_input().unwrap();
    let module = loop_module();
    let key = CanonicalCallableKeyV1::free_static_for_test("int_to_str", 2);
    let index = module.source().catalog().index();
    let header = index.lookup(&key).unwrap();
    assert!(matches!(
        VerifiedCallableFunctionLoweringInputV1::issue(input, index, header),
        Err(LoopPhysicalPrepareRejectV1::NoSafeSlice(
            LoopPhysicalPrepareRejectReasonV1::MissingCallableIndex
        ))
    ));
}

#[test]
fn input_brand_accepts_the_exact_catalog_view() {
    let module = loop_module();
    let key = CanonicalCallableKeyV1::free_static_for_test("int_to_str", 2);
    let input = module.function_input(&key).unwrap();
    let index = module.source().catalog().index();
    let header = index.lookup(&key).unwrap();
    let brand = VerifiedCallableFunctionLoweringInputV1::issue(input, index, header)
        .expect("exact callable brand");
    assert_eq!(brand.owner(), header.callable().owner());
    assert_eq!(brand.header().source_key(), &key);
}

#[test]
fn input_brand_rejects_foreign_catalog_and_header_views() {
    let first = loop_module();
    let second = loop_module();
    let key = CanonicalCallableKeyV1::free_static_for_test("int_to_str", 2);
    let input = first.function_input(&key).unwrap();
    let first_index = first.source().catalog().index();
    let second_index = second.source().catalog().index();
    let first_header = first_index.lookup(&key).unwrap();
    let second_header = second_index.lookup(&key).unwrap();
    assert!(matches!(
        VerifiedCallableFunctionLoweringInputV1::issue(input, second_index, second_header),
        Err(LoopPhysicalPrepareRejectV1::NoSafeSlice(
            LoopPhysicalPrepareRejectReasonV1::ForeignCallableIndex
        ))
    ));
    assert!(matches!(
        VerifiedCallableFunctionLoweringInputV1::issue(input, first_index, second_header),
        Err(LoopPhysicalPrepareRejectV1::NoSafeSlice(
            LoopPhysicalPrepareRejectReasonV1::ForeignCallableHeader
        ))
    ));
    assert!(first_header.callable().owner() != second_header.callable().owner());
}

#[test]
fn current_method_call_fixture_is_a_typed_missing_target_boundary() {
    let module = loop_module();
    let key = CanonicalCallableKeyV1::free_static_for_test("int_to_str", 2);
    let input = module.function_input(&key).unwrap();
    let index = module.source().catalog().index();
    let header = index.lookup(&key).unwrap();
    let completion = verify_function_completion_v1(input).unwrap();
    let (_, product) = loop_product(input);
    assert!(matches!(
        issue_callable_loop_physicalization_v1(
            input,
            index,
            header,
            product,
            completion,
            SourceReceiverShapeV1::Other,
        ),
        Err(LoopPhysicalPrepareRejectV1::NoSafeSlice(
            LoopPhysicalPrepareRejectReasonV1::MissingPreludeTarget
        ))
    ));
}

#[test]
fn resolver_static_fixture_produces_declaration_backed_prepared_positive() {
    let module = static_fixture_for_test();
    let key = CanonicalCallableKeyV1::free_static_for_test("int_to_str", 1);
    let input = module.function_input(&key).unwrap();
    let index = module.source().catalog().index();
    let header = index.lookup(&key).unwrap();
    let completion = verify_function_completion_v1(input).unwrap();
    let (_, product) = loop_product(input);
    let prepared = issue_callable_loop_physicalization_v1(
        input,
        index,
        header,
        product,
        completion,
        SourceReceiverShapeV1::FreeStatic,
    )
    .expect("declaration-backed Prepared product");

    assert_eq!(
        prepared.prelude().result_abi(),
        ExactTrivialReturnAbiV1::I64
    );
    let arguments = prepared.prelude().arguments().rows();
    assert_eq!(arguments.len(), 1);
    assert_eq!(arguments[0].ordinal(), 0);
    assert_eq!(arguments[0].abi(), ExactTrivialReturnAbiV1::I64);
    assert_eq!(arguments[0].binding().owner(), input.owner());
    let call_site = OwnedExprSiteV1::new(input.owner(), prepared.prelude().site().clone());
    let call = input
        .source()
        .expr_at(&call_site)
        .expect("prepared call site");
    let argument = input
        .source()
        .child_expr_from_expr(&call, ExprChildRoleV1::CallArgument(0))
        .expect("prepared argument site");
    assert_eq!(arguments[0].site(), argument.site());
    assert_eq!(prepared.terminal().abi(), ExactTrivialReturnAbiV1::I64);
    assert_eq!(
        prepared
            .completion()
            .function_exit_contract()
            .declared_result(),
        &DeclaredFunctionResultContractV1::Annotated("i64".into())
    );
    assert_eq!(
        prepared.prelude().target(),
        module
            .source()
            .catalog()
            .index()
            .resolve_free_static_source_call("to_i64", 1)
            .unwrap()
            .callable()
    );
}
