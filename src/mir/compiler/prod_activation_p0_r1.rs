//! CUT0-I0-P0-R1 POST failure evidence on the real canonical owner chain.

use super::capability::{CanonicalFirstFamilyPlanV1, CanonicalLoweringPreflightV1};
use super::canonical_finalization::{CanonicalFinalizationInputV1, CanonicalModuleFinalizerV1};
use super::module_postprocess::{
    ModulePostprocessErrorV1, ModulePostprocessOwnerV1, PostprocessFailureStageV1,
};
use super::source_bound_package::ExactCanonicalPreflightPlanV1;
use super::{MirCompiler, VerifiedResolvedSourceUnitV1};
use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, Span};
use crate::mir::verification::MirVerifier;
use crate::mir::BasicBlockId;

fn source() -> VerifiedResolvedSourceUnitV1 {
    VerifiedResolvedSourceUnitV1::resolve_function(ASTNode::FunctionDeclaration {
        name: "p0_r1_verifier_failure".into(),
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
    .expect("P0-R1 verifier source must resolve")
}

#[test]
fn p0_r1_final_verifier_failure_keeps_commit_zero() {
    let source = source();
    let plan = match CanonicalLoweringPreflightV1::verify(&source).unwrap() {
        CanonicalFirstFamilyPlanV1::TrivialBindingSsa(plan) => {
            ExactCanonicalPreflightPlanV1::BindingSsaTrivial(plan)
        }
        _ => panic!("P0-R1 verifier fixture must remain trivial SSA"),
    };
    let mut compiler = MirCompiler::with_options(false);
    let package = compiler.bind_canonical_source(plan).unwrap();
    let finalized = compiler
        .begin_canonical_invocation(
            package,
            Some("p0_r1_verifier_failure.hako"),
            "p0_r1_verifier_failure".into(),
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
    let mut finalized = CanonicalModuleFinalizerV1::finalize(finalized).unwrap();
    let CanonicalFinalizationInputV1::Single(input) = &mut finalized.input else {
        panic!("P0-R1 verifier fixture changed route shape")
    };
    let function = input
        .physical
        .module
        .functions
        .values_mut()
        .next()
        .expect("verifier fixture function");
    function
        .get_block_mut(function.entry_block)
        .expect("verifier fixture entry block")
        .set_jump_with_edge_args(BasicBlockId::new(9999), None);

    let mut verifier = MirVerifier::new();
    let rejected = ModulePostprocessOwnerV1::new(&mut verifier, false)
        .run(finalized)
        .expect_err("missing CFG target must fail canonical final verification");
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
