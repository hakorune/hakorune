//! CUT0-I0-CANON-FIXTURE0-S0.
//!
//! This fixture is the first aggregate proof of the compiler-owned canonical
//! bridge.  Each row starts from an exact preflight plan and consumes the same
//! by-value chain through physical completion.  It deliberately stops before
//! DRAIN0, finalization, and external publication.

use super::canonical_physical_completion::CanonicalPhysicalCompleteInvocationV1;
use super::capability::{CanonicalFirstFamilyPlanV1, CanonicalLoweringPreflightV1};
use super::source_bound_package::ExactCanonicalPreflightPlanV1;
use super::{MirCompiler, VerifiedResolvedCallableProgramV1, VerifiedResolvedSourceUnitV1};
use crate::ast::{ASTNode, BinaryOperator, DeclarationAttrs, LiteralValue, ParamDecl, Span};
use crate::mir::module_invocation_identity::{ModuleInvocationBrandV1, ModuleInvocationFamilyV1};
use crate::mir::verification::MirVerifier;
use crate::mir::MirCompileResult;

fn literal(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn bool_literal(value: bool) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Bool(value),
        span: Span::unknown(),
    }
}

fn variable(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.into(),
        span: Span::unknown(),
    }
}

fn local(name: &str, value: ASTNode) -> ASTNode {
    ASTNode::Local {
        variables: vec![name.into()],
        initial_values: vec![Some(Box::new(value))],
        declared_type_names: vec![None],
        span: Span::unknown(),
    }
}

fn assignment(name: &str, value: i64) -> ASTNode {
    ASTNode::Assignment {
        target: Box::new(variable(name)),
        value: Box::new(literal(value)),
        span: Span::unknown(),
    }
}

fn add(left: ASTNode, right: ASTNode) -> ASTNode {
    ASTNode::BinaryOp {
        operator: BinaryOperator::Add,
        left: Box::new(left),
        right: Box::new(right),
        span: Span::unknown(),
    }
}

fn if_stmt(
    condition: ASTNode,
    then_body: Vec<ASTNode>,
    else_body: Option<Vec<ASTNode>>,
) -> ASTNode {
    ASTNode::If {
        condition: Box::new(condition),
        then_body,
        else_body,
        span: Span::unknown(),
    }
}

fn named_function(name: &str, params: Vec<&str>, body: Vec<ASTNode>) -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: name.into(),
        params: params.into_iter().map(str::to_owned).collect(),
        param_decls: Vec::new(),
        return_type_name: None,
        body,
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

fn typed_callable(name: &str, value: ASTNode) -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: name.into(),
        params: vec!["x".into()],
        param_decls: vec![ParamDecl {
            name: "x".into(),
            declared_type_name: Some("i64".into()),
        }],
        return_type_name: Some("i64".into()),
        body: vec![ASTNode::Return {
            value: Some(Box::new(value)),
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

fn call(name: &str) -> ASTNode {
    ASTNode::FunctionCall {
        name: name.into(),
        arguments: vec![variable("x")],
        span: Span::unknown(),
    }
}

fn a_plus_source() -> VerifiedResolvedSourceUnitV1 {
    VerifiedResolvedSourceUnitV1::resolve_function(named_function(
        "a_plus_fixture",
        vec!["arg"],
        vec![
            ASTNode::Local {
                variables: vec!["x".into()],
                initial_values: vec![Some(Box::new(add(variable("arg"), literal(1))))],
                declared_type_names: vec![None],
                span: Span::unknown(),
            },
            ASTNode::Assignment {
                target: Box::new(variable("x")),
                value: Box::new(add(variable("x"), literal(1))),
                span: Span::unknown(),
            },
            ASTNode::Outbox {
                variables: vec!["result".into()],
                initial_values: vec![None],
                span: Span::unknown(),
            },
            ASTNode::Return {
                value: Some(Box::new(variable("x"))),
                span: Span::unknown(),
            },
        ],
    ))
    .unwrap()
}

fn condition_fn_source() -> VerifiedResolvedSourceUnitV1 {
    VerifiedResolvedSourceUnitV1::resolve_function(named_function(
        "condition_fn",
        Vec::new(),
        vec![
            local("x", literal(0)),
            if_stmt(
                bool_literal(true),
                vec![assignment("x", 1)],
                Some(vec![assignment("x", 2)]),
            ),
            ASTNode::Return {
                value: Some(Box::new(variable("x"))),
                span: Span::unknown(),
            },
        ],
    ))
    .unwrap()
}

fn trivial_source() -> VerifiedResolvedSourceUnitV1 {
    VerifiedResolvedSourceUnitV1::resolve_function(named_function(
        "trivial_fixture",
        Vec::new(),
        vec![
            local("x", literal(0)),
            if_stmt(
                bool_literal(true),
                vec![assignment("x", 1)],
                Some(vec![assignment("x", 2)]),
            ),
            ASTNode::Return {
                value: Some(Box::new(variable("x"))),
                span: Span::unknown(),
            },
        ],
    ))
    .unwrap()
}

fn callable_program(functions: Vec<ASTNode>) -> VerifiedResolvedCallableProgramV1 {
    VerifiedResolvedCallableProgramV1::resolve(ASTNode::Program {
        statements: functions,
        span: Span::unknown(),
    })
    .unwrap()
}

fn acyclic_source() -> VerifiedResolvedCallableProgramV1 {
    callable_program(vec![
        typed_callable("caller", call("callee")),
        typed_callable("callee", variable("x")),
    ])
}

fn recursive_source() -> VerifiedResolvedCallableProgramV1 {
    callable_program(vec![typed_callable("loop", call("loop"))])
}

fn assert_single_completion(
    completion: CanonicalPhysicalCompleteInvocationV1<'_>,
    expected_family: ModuleInvocationFamilyV1,
    expected_brand: ModuleInvocationBrandV1,
) {
    let CanonicalPhysicalCompleteInvocationV1::Single(product) = completion else {
        panic!("canonical callable route unexpectedly entered single completion")
    };
    assert_eq!(product.token.family(), expected_family);
    assert_eq!(product.token.brand(), expected_brand);
    assert_eq!(product.session.brand(), expected_brand);
    assert_eq!(product.physical.brand(), expected_brand);
    assert_eq!(product.physical.receipt_brand(), expected_brand);
}

fn assert_callable_completion(
    completion: CanonicalPhysicalCompleteInvocationV1<'_>,
    expected_family: ModuleInvocationFamilyV1,
    expected_brand: ModuleInvocationBrandV1,
) {
    let CanonicalPhysicalCompleteInvocationV1::Callable(product) = completion else {
        panic!("canonical single route unexpectedly entered callable completion")
    };
    assert_eq!(product.token.family(), expected_family);
    assert_eq!(product.token.brand(), expected_brand);
    assert_eq!(product.session.brand(), expected_brand);
    assert_eq!(product.physical.brand(), expected_brand);
    assert_eq!(product.physical.receipt_brand(), expected_brand);
    assert_eq!(product.capability.brand(), expected_brand);
    assert_eq!(product.capability.family(), expected_family);
}

fn finish_canonical_route<'a>(
    compiler: &mut MirCompiler,
    plan: ExactCanonicalPreflightPlanV1<'a>,
    source_file: &str,
    module_name: &str,
) -> (ModuleInvocationBrandV1, MirCompileResult) {
    let package = compiler.bind_canonical_source(plan).unwrap();
    let brand = package.brand();
    let finalized = compiler
        .begin_canonical_invocation(package, Some(source_file), module_name.to_owned())
        .unwrap()
        .lower()
        .unwrap()
        .collect()
        .unwrap()
        .complete()
        .unwrap()
        .prepare_drain()
        .unwrap()
        .drain()
        .prepare_finalization()
        .unwrap();
    let finalized = super::canonical_finalization::CanonicalModuleFinalizerV1::finalize(finalized)
        .unwrap();
    let mut verifier = MirVerifier::new();
    let processed = super::module_postprocess::ModulePostprocessOwnerV1::new(&mut verifier, false)
        .run(finalized)
        .unwrap();
    let prepared = compiler.prepare_module_external_commit(processed).unwrap();
    let result = compiler.commit_prepared_module(prepared);
    (brand, result)
}

#[test]
fn canonical_bridge_fixture0_four_route_aggregate() {
    let mut compiler = MirCompiler::with_options(false);

    let a_plus = a_plus_source();
    let plan = match CanonicalLoweringPreflightV1::verify(&a_plus).unwrap() {
        CanonicalFirstFamilyPlanV1::CurrentCanonicalAPlus(plan) => {
            ExactCanonicalPreflightPlanV1::APlus(plan)
        }
        _ => panic!("A+ aggregate row changed preflight family"),
    };
    let package = compiler.bind_canonical_source(plan).unwrap();
    let a_plus_brand = package.brand();
    let completion = compiler
        .begin_canonical_invocation(
            package,
            Some("fixture_a_plus.hako"),
            "fixture_a_plus".into(),
        )
        .unwrap()
        .lower()
        .unwrap()
        .collect()
        .unwrap()
        .complete()
        .unwrap();
    assert_single_completion(
        completion,
        ModuleInvocationFamilyV1::CanonicalAPlus,
        a_plus_brand,
    );
    assert!(compiler.builder.current_module.is_none());

    let trivial = trivial_source();
    let plan = match CanonicalLoweringPreflightV1::verify(&trivial).unwrap() {
        CanonicalFirstFamilyPlanV1::TrivialBindingSsa(plan) => {
            ExactCanonicalPreflightPlanV1::BindingSsaTrivial(plan)
        }
        _ => panic!("trivial aggregate row changed preflight family"),
    };
    let package = compiler.bind_canonical_source(plan).unwrap();
    let trivial_brand = package.brand();
    let completion = compiler
        .begin_canonical_invocation(
            package,
            Some("fixture_trivial.hako"),
            "fixture_trivial".into(),
        )
        .unwrap()
        .lower()
        .unwrap()
        .collect()
        .unwrap()
        .complete()
        .unwrap();
    assert_single_completion(
        completion,
        ModuleInvocationFamilyV1::BindingSsaTrivial,
        trivial_brand,
    );
    assert!(compiler.builder.current_module.is_none());

    let acyclic = acyclic_source();
    let plan = super::acyclic_callable_module_plan::VerifiedAcyclicCallableModulePlanV1::verify(
        acyclic.module(),
    )
    .unwrap();
    let package = compiler
        .bind_canonical_source(ExactCanonicalPreflightPlanV1::BindingSsaAcyclic(plan))
        .unwrap();
    let acyclic_brand = package.brand();
    let completion = compiler
        .begin_canonical_invocation(
            package,
            Some("fixture_acyclic.hako"),
            "fixture_acyclic".into(),
        )
        .unwrap()
        .lower()
        .unwrap()
        .collect()
        .unwrap()
        .complete()
        .unwrap();
    assert_callable_completion(
        completion,
        ModuleInvocationFamilyV1::BindingSsaAcyclic,
        acyclic_brand,
    );
    assert!(compiler.builder.current_module.is_none());

    let recursive = recursive_source();
    let plan =
        super::recursive_callable_module_plan::VerifiedRecursiveCallableModulePlanV1::verify(
            recursive.module(),
        )
        .unwrap();
    let package = compiler
        .bind_canonical_source(ExactCanonicalPreflightPlanV1::BindingSsaRecursive(plan))
        .unwrap();
    let recursive_brand = package.brand();
    let completion = compiler
        .begin_canonical_invocation(
            package,
            Some("fixture_recursive.hako"),
            "fixture_recursive".into(),
        )
        .unwrap()
        .lower()
        .unwrap()
        .collect()
        .unwrap()
        .complete()
        .unwrap();
    assert_callable_completion(
        completion,
        ModuleInvocationFamilyV1::BindingSsaRecursive,
        recursive_brand,
    );
    assert!(compiler.builder.current_module.is_none());

    assert_ne!(a_plus_brand, trivial_brand);
    assert_ne!(trivial_brand, acyclic_brand);
    assert_ne!(acyclic_brand, recursive_brand);
}

#[test]
fn canonical_bridge_fixture0_condition_fn_spelling_is_canonical() {
    let source = condition_fn_source();
    let plan = CanonicalLoweringPreflightV1::verify(&source).unwrap();
    let CanonicalFirstFamilyPlanV1::TrivialBindingSsa(plan) = plan else {
        panic!("condition_fn fixture must retain its trivial plan")
    };
    let header = plan.seal_resolved_owner_header_v1().unwrap();
    assert_eq!(header.symbol().as_mir_name(), "condition_fn/0");

    let mut compiler = MirCompiler::with_options(false);
    let package = compiler
        .bind_canonical_source(ExactCanonicalPreflightPlanV1::BindingSsaTrivial(plan))
        .unwrap();
    let completion = compiler
        .begin_canonical_invocation(package, Some("condition_fn.hako"), "condition_fn".into())
        .unwrap()
        .lower()
        .unwrap()
        .collect()
        .unwrap()
        .complete()
        .unwrap();
    assert!(matches!(
        completion,
        CanonicalPhysicalCompleteInvocationV1::Single(_)
    ));
    assert!(compiler.builder.current_module.is_none());
}

#[test]
fn canonical_bridge_fixture0_foreign_pairing_is_rejected() {
    let first = trivial_source();
    let foreign = a_plus_source();
    let first_plan = CanonicalLoweringPreflightV1::verify(&first).unwrap();
    let foreign_plan = CanonicalLoweringPreflightV1::verify(&foreign).unwrap();
    let header = first_plan.seal_resolved_owner_header_v1().unwrap();
    assert!(header.require_same_plan(&foreign_plan).is_err());
}

#[test]
fn canonical_bridge_fixture0_recursive_acyclic_witness_parity() {
    let mut compiler = MirCompiler::with_options(false);

    let acyclic = acyclic_source();
    let acyclic_plan =
        super::acyclic_callable_module_plan::VerifiedAcyclicCallableModulePlanV1::verify(
            acyclic.module(),
        )
        .unwrap();
    let acyclic_package = compiler
        .bind_canonical_source(ExactCanonicalPreflightPlanV1::BindingSsaAcyclic(
            acyclic_plan,
        ))
        .unwrap();
    let acyclic_complete = compiler
        .begin_canonical_invocation(
            acyclic_package,
            Some("witness_acyclic.hako"),
            "witness_acyclic".into(),
        )
        .unwrap()
        .lower()
        .unwrap()
        .collect()
        .unwrap()
        .complete()
        .unwrap();
    let CanonicalPhysicalCompleteInvocationV1::Callable(acyclic_product) = acyclic_complete else {
        panic!("acyclic route did not produce callable completion")
    };
    assert_eq!(
        acyclic_product.capability.brand(),
        acyclic_product.token.brand()
    );
    assert_eq!(
        acyclic_product.capability.family(),
        ModuleInvocationFamilyV1::BindingSsaAcyclic
    );

    let recursive = recursive_source();
    let recursive_plan =
        super::recursive_callable_module_plan::VerifiedRecursiveCallableModulePlanV1::verify(
            recursive.module(),
        )
        .unwrap();
    let recursive_package = compiler
        .bind_canonical_source(ExactCanonicalPreflightPlanV1::BindingSsaRecursive(
            recursive_plan,
        ))
        .unwrap();
    let recursive_complete = compiler
        .begin_canonical_invocation(
            recursive_package,
            Some("witness_recursive.hako"),
            "witness_recursive".into(),
        )
        .unwrap()
        .lower()
        .unwrap()
        .collect()
        .unwrap()
        .complete()
        .unwrap();
    let CanonicalPhysicalCompleteInvocationV1::Callable(recursive_product) = recursive_complete
    else {
        panic!("recursive route did not produce callable completion")
    };
    assert_eq!(
        recursive_product.capability.brand(),
        recursive_product.token.brand()
    );
    assert_eq!(
        recursive_product.capability.family(),
        ModuleInvocationFamilyV1::BindingSsaRecursive
    );
}

#[test]
fn p0_r1_canonical_four_route_real_authority_chain() {
    let mut compiler = MirCompiler::with_options(false);

    let a_plus = a_plus_source();
    let a_plus_plan = match CanonicalLoweringPreflightV1::verify(&a_plus).unwrap() {
        CanonicalFirstFamilyPlanV1::CurrentCanonicalAPlus(plan) => {
            ExactCanonicalPreflightPlanV1::APlus(plan)
        }
        _ => panic!("P0-R1 A+ route changed preflight family"),
    };
    let (a_plus_brand, a_plus_result) =
        finish_canonical_route(&mut compiler, a_plus_plan, "p0_r1_a_plus.hako", "p0_r1_a_plus");
    assert!(a_plus_result
        .module
        .functions
        .contains_key("a_plus_fixture/1"));
    assert!(a_plus_result.verification_result.is_ok());

    let trivial = trivial_source();
    let trivial_plan = match CanonicalLoweringPreflightV1::verify(&trivial).unwrap() {
        CanonicalFirstFamilyPlanV1::TrivialBindingSsa(plan) => {
            ExactCanonicalPreflightPlanV1::BindingSsaTrivial(plan)
        }
        _ => panic!("P0-R1 trivial route changed preflight family"),
    };
    let (trivial_brand, trivial_result) = finish_canonical_route(
        &mut compiler,
        trivial_plan,
        "p0_r1_trivial.hako",
        "p0_r1_trivial",
    );
    assert!(trivial_result
        .module
        .functions
        .contains_key("trivial_fixture/0"));
    assert!(trivial_result.verification_result.is_ok());

    let acyclic = acyclic_source();
    let acyclic_plan = super::acyclic_callable_module_plan::VerifiedAcyclicCallableModulePlanV1::verify(
        acyclic.module(),
    )
    .unwrap();
    let (acyclic_brand, acyclic_result) = finish_canonical_route(
        &mut compiler,
        ExactCanonicalPreflightPlanV1::BindingSsaAcyclic(acyclic_plan),
        "p0_r1_acyclic.hako",
        "p0_r1_acyclic",
    );
    assert!(acyclic_result.module.functions.contains_key("caller/1"));
    assert!(acyclic_result.module.functions.contains_key("callee/1"));
    assert!(acyclic_result.verification_result.is_ok());

    let recursive = recursive_source();
    let recursive_plan =
        super::recursive_callable_module_plan::VerifiedRecursiveCallableModulePlanV1::verify(
            recursive.module(),
        )
        .unwrap();
    let (recursive_brand, recursive_result) = finish_canonical_route(
        &mut compiler,
        ExactCanonicalPreflightPlanV1::BindingSsaRecursive(recursive_plan),
        "p0_r1_recursive.hako",
        "p0_r1_recursive",
    );
    assert!(recursive_result.module.functions.contains_key("loop/1"));
    assert!(recursive_result.verification_result.is_ok());

    assert_ne!(a_plus_brand, trivial_brand);
    assert_ne!(trivial_brand, acyclic_brand);
    assert_ne!(acyclic_brand, recursive_brand);
    assert!(compiler.builder.current_module.is_none());
}
