//! CUT0-I0-COMMIT0 focused fixture.

use super::capability::{CanonicalFirstFamilyPlanV1, CanonicalLoweringPreflightV1};
use super::module_postprocess::ModulePostprocessOwnerV1;
use super::external_commit::PostprocessEvidenceSealV1;
use super::source_bound_package::ExactCanonicalPreflightPlanV1;
use super::{MirCompiler, VerifiedResolvedSourceUnitV1};
use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, Span};
use crate::mir::verification::MirVerifier;

fn source() -> VerifiedResolvedSourceUnitV1 {
    VerifiedResolvedSourceUnitV1::resolve_function(ASTNode::FunctionDeclaration {
        name: "commit_fixture".into(),
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
    .expect("COMMIT0 source must resolve")
}

#[test]
fn paired_external_commit_consumes_builder_and_module_once() {
    let source = source();
    let plan = match CanonicalLoweringPreflightV1::verify(&source).unwrap() {
        CanonicalFirstFamilyPlanV1::TrivialBindingSsa(plan) => {
            ExactCanonicalPreflightPlanV1::BindingSsaTrivial(plan)
        }
        _ => panic!("COMMIT0 fixture must remain trivial SSA"),
    };
    let mut compiler = MirCompiler::new();
    let package = compiler.bind_canonical_source(plan).unwrap();
    let finalized = compiler
        .begin_canonical_invocation(package, Some("commit.hako"), "commit".into())
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
    let mut verifier = MirVerifier::new();
    let processed = ModulePostprocessOwnerV1::new(&mut verifier, false)
        .run(finalized)
        .unwrap();
    let prepared = compiler.prepare_module_external_commit(processed).unwrap();
    assert!(matches!(
        prepared.evidence(),
        PostprocessEvidenceSealV1::CanonicalSingle { .. }
    ));
    let result = compiler.commit_prepared_module(prepared);

    assert!(result.module.functions.contains_key("commit_fixture/0"));
    assert!(compiler.builder.current_module.is_none());
}
