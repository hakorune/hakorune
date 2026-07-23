//! CUT0-I0-POST0 focused fixtures.

use super::capability::{CanonicalFirstFamilyPlanV1, CanonicalLoweringPreflightV1};
use super::module_postprocess::{
    ModulePostprocessErrorV1, ModulePostprocessOwnerV1, ModulePostprocessScheduleV1,
    PostprocessFailureStageV1, RcInsertionScheduleV1, VerificationBarrierV1,
};
use super::source_bound_package::ExactCanonicalPreflightPlanV1;
use super::{MirCompiler, VerifiedResolvedSourceUnitV1};
use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, Span};
use crate::mir::module_invocation_identity::ModuleInvocationFamilyV1;
use crate::mir::BasicBlockId;

fn source() -> VerifiedResolvedSourceUnitV1 {
    VerifiedResolvedSourceUnitV1::resolve_function(ASTNode::FunctionDeclaration {
        name: "postprocess_fixture".into(),
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
    .expect("POST0 source must resolve")
}

#[test]
fn postprocess_schedule_is_family_owned() {
    assert_eq!(
        ModulePostprocessScheduleV1::for_family(ModuleInvocationFamilyV1::Raw).rc(),
        RcInsertionScheduleV1::Run
    );
    assert_eq!(
        ModulePostprocessScheduleV1::for_family(ModuleInvocationFamilyV1::Raw).verifier(),
        VerificationBarrierV1::ReportPreTransformOnly
    );
    assert_eq!(
        ModulePostprocessScheduleV1::for_family(ModuleInvocationFamilyV1::CanonicalAPlus).rc(),
        RcInsertionScheduleV1::Run
    );
    for family in [
        ModuleInvocationFamilyV1::BindingSsaTrivial,
        ModuleInvocationFamilyV1::BindingSsaAcyclic,
        ModuleInvocationFamilyV1::BindingSsaRecursive,
    ] {
        let schedule = ModulePostprocessScheduleV1::for_family(family);
        assert_eq!(schedule.rc(), RcInsertionScheduleV1::Skip);
        assert_eq!(schedule.verifier(), VerificationBarrierV1::RequireFinal);
    }
}

#[test]
fn postprocess_consumes_finalized_single_without_publication() {
    let source = source();
    let plan = match CanonicalLoweringPreflightV1::verify(&source).unwrap() {
        CanonicalFirstFamilyPlanV1::TrivialBindingSsa(plan) => {
            ExactCanonicalPreflightPlanV1::BindingSsaTrivial(plan)
        }
        _ => panic!("POST0 fixture must remain trivial SSA"),
    };
    let mut compiler = MirCompiler::new();
    let package = compiler.bind_canonical_source(plan).unwrap();
    let expected_brand = package.brand();
    let finalized = compiler
        .begin_canonical_invocation(package, Some("postprocess.hako"), "postprocess".into())
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
    let finalized =
        super::canonical_finalization::CanonicalModuleFinalizerV1::finalize(finalized).unwrap();
    let processed = ModulePostprocessOwnerV1::new(&mut compiler.verifier, false)
        .run(finalized)
        .unwrap();
    assert_eq!(
        processed.family(),
        ModuleInvocationFamilyV1::BindingSsaTrivial
    );
    assert_eq!(processed.brand(), expected_brand);
    assert!(processed
        .module()
        .functions
        .contains_key("postprocess_fixture/0"));
    assert!(compiler.builder.current_module.is_none());
}

#[test]
fn postprocess_final_verifier_failure_retains_discard_only_owner() {
    let source = source();
    let plan = match CanonicalLoweringPreflightV1::verify(&source).unwrap() {
        CanonicalFirstFamilyPlanV1::TrivialBindingSsa(plan) => {
            ExactCanonicalPreflightPlanV1::BindingSsaTrivial(plan)
        }
        _ => panic!("POST0 rejection fixture must remain trivial SSA"),
    };
    let mut compiler = MirCompiler::with_options(false);
    let package = compiler.bind_canonical_source(plan).unwrap();
    let finalized = compiler
        .begin_canonical_invocation(
            package,
            Some("postprocess-reject.hako"),
            "postprocess".into(),
        )
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
    let mut finalized =
        super::canonical_finalization::CanonicalModuleFinalizerV1::finalize(finalized).unwrap();
    let super::canonical_finalization::CanonicalFinalizationInputV1::Single(input) =
        &mut finalized.input
    else {
        panic!("POST0 rejection fixture changed route shape")
    };
    let function = input
        .physical
        .module
        .functions
        .values_mut()
        .next()
        .expect("POST0 rejection fixture function");
    function
        .get_block_mut(function.entry_block)
        .expect("POST0 rejection fixture entry block")
        .set_jump_with_edge_args(BasicBlockId::new(9999), None);

    let mut verifier = super::super::verification::MirVerifier::new();
    let rejected = ModulePostprocessOwnerV1::new(&mut verifier, false)
        .run(finalized)
        .expect_err("POST0 final verification must reject");
    assert_eq!(
        rejected.stage(),
        PostprocessFailureStageV1::FinalVerification
    );
    assert!(matches!(
        rejected.error(),
        ModulePostprocessErrorV1::FinalVerification(_)
    ));
    rejected.discard();
    assert!(compiler.builder.current_module.is_none());
}
