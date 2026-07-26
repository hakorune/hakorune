use super::*;
use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, Span};
use crate::mir::compiler::capability::{
    CanonicalLoweringPreflightV1, CanonicalTrivialBindingSsaPlanV1,
};
use crate::mir::compiler::lowering_input::VerifiedResolvedSourceUnitV1;
use crate::mir::resolved_control_flow::{
    DeclaredFunctionResultContractV1, FunctionExitCoverageV1, FunctionUnitOriginV1,
    SealedFunctionExitDispositionV1,
};
use std::collections::HashMap;

use super::super::{
    NormalSourcePlanClassifierV1, PreparedNormalSourcePlanInputV1, SealedNormalScalarRootV1,
    SealedNormalSourcePlanV1, VerifiedNormalMainFunctionSourceUnitV1,
};

fn literal(value: LiteralValue) -> ASTNode {
    ASTNode::Literal {
        value,
        span: Span::unknown(),
    }
}

fn return_(value: Option<LiteralValue>) -> ASTNode {
    ASTNode::Return {
        value: value.map(|value| Box::new(literal(value))),
        span: Span::unknown(),
    }
}

fn function(name: &str, params: Vec<String>, result: Option<&str>, body: Vec<ASTNode>) -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: name.to_owned(),
        params,
        param_decls: Vec::new(),
        return_type_name: result.map(str::to_owned),
        body,
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

fn main_program(result: Option<&str>, body: Vec<ASTNode>) -> ASTNode {
    let mut methods = HashMap::new();
    methods.insert(
        "main".to_owned(),
        function("main", Vec::new(), result, body),
    );
    ASTNode::Program {
        statements: vec![ASTNode::BoxDeclaration {
            name: "Main".to_owned(),
            fields: Vec::new(),
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

fn main_source(result: Option<&str>, body: Vec<ASTNode>) -> VerifiedNormalMainFunctionSourceUnitV1 {
    let input =
        PreparedNormalSourcePlanInputV1::new(main_program(result, body), "main-f1-plan-test");
    let plan = NormalSourcePlanClassifierV1::seal(input).expect("valid Main0");
    let SealedNormalSourcePlanV1::ScalarRoot(SealedNormalScalarRootV1::Main0(main)) = plan else {
        panic!("expected Main0");
    };
    main.prepare_function_source().expect("exact Main source")
}

fn resolved_main(
    result: Option<&str>,
    body: Vec<ASTNode>,
) -> VerifiedNormalMainResolvedSourceUnitV1 {
    main_source(result, body)
        .prepare_embedded_resolved_main()
        .expect("embedded Main resolution")
}

fn with_plan<R>(
    result: Option<&str>,
    body: Vec<ASTNode>,
    inspect: impl FnOnce(&VerifiedNormalMainFunctionPlanV1<'_>) -> R,
) -> R {
    let unit = resolved_main(result, body);
    let plan = NormalMainFunctionPreflightV1::seal(&unit).expect("Main F1 plan");
    inspect(&plan)
}

fn contract<'plan, 'unit>(
    plan: &'plan VerifiedNormalMainFunctionPlanV1<'unit>,
) -> &'plan crate::mir::resolved_control_flow::SealedFunctionExitContractV1 {
    plan.completion().function_exit_contract()
}

#[test]
fn main_f1_seals_empty_fallthrough_and_expression_statement_as_unit() {
    for (body, expected_origin, expected_end) in [
        (Vec::new(), FunctionUnitOriginV1::EmptyBody, 0),
        (
            vec![literal(LiteralValue::Integer(1))],
            FunctionUnitOriginV1::ImplicitFallthrough,
            1,
        ),
    ] {
        with_plan(None, body, |plan| {
            assert!(matches!(
                contract(plan).disposition(),
                SealedFunctionExitDispositionV1::ImplicitUnit {
                    body_end,
                    origin,
                    ..
                } if *body_end == expected_end && *origin == expected_origin
            ));
            assert_eq!(
                contract(plan).coverage(),
                FunctionExitCoverageV1::ExactZeroExitRootBody
            );
        });
    }
}

#[test]
fn main_f1_preserves_explicit_unit_origins() {
    for (value, origin) in [
        (None, FunctionUnitOriginV1::BareReturn),
        (Some(LiteralValue::Void), FunctionUnitOriginV1::ExplicitVoid),
        (Some(LiteralValue::Null), FunctionUnitOriginV1::ExplicitNull),
    ] {
        with_plan(None, vec![return_(value)], |plan| {
            assert!(matches!(
                contract(plan).disposition(),
                SealedFunctionExitDispositionV1::ExplicitUnit {
                    origin: actual,
                    ..
                } if *actual == origin
            ));
            assert_eq!(
                contract(plan).coverage(),
                FunctionExitCoverageV1::ExactOneTerminalRootReturn
            );
        });
    }
}

#[test]
fn main_f1_admits_exact_scalar_value_carriers() {
    for value in [
        LiteralValue::Integer(7),
        LiteralValue::Bool(true),
        LiteralValue::Float(1.5),
    ] {
        with_plan(None, vec![return_(Some(value))], |plan| {
            assert!(plan.completion().returns_value());
            assert!(matches!(
                contract(plan).disposition(),
                SealedFunctionExitDispositionV1::ExplicitValue { .. }
            ));
        });
    }
}

#[test]
fn main_f1_admits_void_and_exact_i64_declared_contracts() {
    with_plan(
        Some("void"),
        vec![return_(Some(LiteralValue::Void))],
        |plan| {
            assert_eq!(
                contract(plan).declared_result(),
                &DeclaredFunctionResultContractV1::Void
            );
        },
    );

    with_plan(
        Some("i64"),
        vec![return_(Some(LiteralValue::Integer(42)))],
        |plan| {
            assert_eq!(
                contract(plan).declared_result(),
                &DeclaredFunctionResultContractV1::Annotated("i64".into())
            );
        },
    );
}

#[test]
fn main_f1_rejects_contract_mismatch_and_unsupported_carrier_before_lowering() {
    for unit in [
        resolved_main(Some("void"), vec![return_(Some(LiteralValue::Integer(1)))]),
        resolved_main(Some("i64"), Vec::new()),
        resolved_main(
            Some("Integer"),
            vec![return_(Some(LiteralValue::Integer(1)))],
        ),
        resolved_main(
            None,
            vec![return_(Some(LiteralValue::String("text".to_owned())))],
        ),
    ] {
        let rejected =
            NormalMainFunctionPreflightV1::seal(&unit).expect_err("typed Main rejection");
        assert!(matches!(
            rejected.error(),
            NormalMainFunctionPlanErrorV1::CanonicalPreflight(_)
        ));
        assert!(std::ptr::eq(rejected.owner_for_test(), &unit));
        rejected.discard();
    }
}

#[test]
fn main_f1_rejects_multiple_nested_and_nonterminal_returns() {
    let nested = ASTNode::If {
        condition: Box::new(literal(LiteralValue::Bool(true))),
        then_body: vec![return_(Some(LiteralValue::Integer(1)))],
        else_body: None,
        span: Span::unknown(),
    };
    for body in [
        vec![
            return_(Some(LiteralValue::Integer(1))),
            return_(Some(LiteralValue::Integer(2))),
        ],
        vec![nested],
        vec![
            return_(Some(LiteralValue::Integer(1))),
            literal(LiteralValue::Integer(2)),
        ],
    ] {
        let unit = resolved_main(None, body);
        assert!(NormalMainFunctionPreflightV1::seal(&unit).is_err());
    }
}

#[test]
fn main_f1_rejects_direct_call_and_nested_owner_before_lowering() {
    let direct_call = main_source(
        None,
        vec![ASTNode::FunctionCall {
            name: "helper".to_owned(),
            arguments: Vec::new(),
            span: Span::unknown(),
        }],
    )
    .prepare_embedded_resolved_main()
    .expect("resolution retains the unresolved call shape");
    let direct_call =
        NormalMainFunctionPreflightV1::seal(&direct_call).expect_err("Main call rejects in F1");
    assert!(matches!(
        direct_call.error(),
        NormalMainFunctionPlanErrorV1::CanonicalPreflight(_)
    ));
    direct_call.discard();

    let nested_owner = resolved_main(
        None,
        vec![ASTNode::Local {
            variables: vec!["f".to_owned()],
            initial_values: vec![Some(Box::new(ASTNode::Lambda {
                params: Vec::new(),
                body: vec![literal(LiteralValue::Integer(1))],
                span: Span::unknown(),
            }))],
            declared_type_names: vec![None],
            span: Span::unknown(),
        }],
    );
    let rejected =
        NormalMainFunctionPreflightV1::seal(&nested_owner).expect_err("nested owner rejects");
    assert!(matches!(
        rejected.error(),
        NormalMainFunctionPlanErrorV1::CanonicalPreflight(_)
    ));
    rejected.discard();
}

#[test]
fn ordinary_preflight_still_rejects_standalone_main() {
    let standalone = VerifiedResolvedSourceUnitV1::resolve_function(function(
        "main",
        Vec::new(),
        None,
        vec![return_(Some(LiteralValue::Integer(1)))],
    ))
    .expect("standalone test source resolves");

    assert!(CanonicalLoweringPreflightV1::verify(&standalone).is_err());
}

#[test]
fn main_plan_retains_exact_role_unit_and_consumable_trivial_plan() {
    let unit = resolved_main(None, vec![return_(Some(LiteralValue::Integer(9)))]);
    let plan = NormalMainFunctionPreflightV1::seal(&unit).expect("Main F1 plan");

    assert_eq!(plan.role(), unit.role());
    assert!(std::ptr::eq(plan.owner_for_test(), &unit));
    let lowering: CanonicalTrivialBindingSsaPlanV1<'_> = plan.into_lowering();
    assert!(lowering.completion().returns_value());
}
