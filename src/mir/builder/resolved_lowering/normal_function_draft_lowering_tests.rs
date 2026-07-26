use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, Span};
use crate::mir::compiler::capability::{CanonicalFirstFamilyPlanV1, CanonicalLoweringPreflightV1};
use crate::mir::compiler::VerifiedResolvedSourceUnitV1;
use crate::mir::resolved_control_flow::verify_function_completion_v1;
use crate::mir::resolved_semantics::FunctionSemanticResolverSessionV1;
use crate::mir::{BasicBlockId, MirBuilder, MirInstruction};

use super::completion_consumption::ResolvedFunctionCompletionConsumptionV1;
use super::draft_seal::ReadyFunctionDraftSealV1;
use super::draft_seal_owner::FunctionDraftSealStageV1;
use super::{
    reject_after_session_discard, reject_draft_seal_typed, NormalFunctionDraftLoweringCauseV1,
    NormalFunctionDraftLoweringStageV1,
};

fn function(name: &str) -> ASTNode {
    function_with_body(
        name,
        vec![ASTNode::Return {
            value: Some(Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(7),
                span: Span::unknown(),
            })),
            span: Span::unknown(),
        }],
    )
}

fn function_with_body(name: &str, body: Vec<ASTNode>) -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: name.to_owned(),
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
    }
}

fn with_plan<R>(
    name: &str,
    use_plan: impl FnOnce(crate::mir::compiler::capability::CanonicalTrivialBindingSsaPlanV1<'_>) -> R,
) -> R {
    let source = VerifiedResolvedSourceUnitV1::resolve_function(function(name)).unwrap();
    let CanonicalFirstFamilyPlanV1::TrivialBindingSsa(plan) =
        CanonicalLoweringPreflightV1::verify(&source).unwrap()
    else {
        panic!("fixture must use trivial Binding SSA")
    };
    use_plan(plan)
}

#[test]
fn retaining_terminal_keeps_compatibility_success_shape() {
    let mut typed_builder = MirBuilder::new();
    let typed = with_plan("typed", |plan| {
        typed_builder
            .lower_resolved_trivial_function_draft_retaining_failure_v1(plan)
            .unwrap()
    });
    assert_eq!(typed.signature.name, "typed/0");
    assert!(typed_builder.function_state.current_function.is_none());
    assert!(typed_builder.function_state.current_block.is_none());

    let mut compatibility_builder = MirBuilder::new();
    let compatibility = with_plan("compatibility", |plan| {
        compatibility_builder
            .lower_resolved_trivial_function_draft(plan)
            .unwrap()
    });
    assert_eq!(
        compatibility.signature.return_type,
        typed.signature.return_type
    );
    assert!(compatibility_builder
        .function_state
        .current_function
        .is_none());
    assert!(compatibility_builder.function_state.current_block.is_none());
}

#[test]
fn typed_body_lowering_rejection_has_restoration_receipt_without_string_stage_parse() {
    let mut builder = MirBuilder::new();
    let session = builder.open_resolved_function_draft_seal_session_v1("body_failure/0");
    let rejected = reject_after_session_discard(
        session,
        NormalFunctionDraftLoweringStageV1::BodyLowering,
        "injected body failure".to_owned(),
    );

    assert_eq!(
        rejected.stage(),
        NormalFunctionDraftLoweringStageV1::BodyLowering
    );
    assert!(matches!(
        rejected.cause(),
        NormalFunctionDraftLoweringCauseV1::BuilderContract(detail)
            if detail.as_ref() == "injected body failure"
    ));
    assert!(rejected.has_restoration_receipt());
    assert!(builder.function_state.current_function.is_none());
    assert!(builder.function_state.current_block.is_none());
}

#[test]
fn typed_draft_seal_rejection_keeps_exact_inner_stage() {
    let source = VerifiedResolvedSourceUnitV1::resolve_function(function_with_body(
        "seal_failure",
        vec![ASTNode::Return {
            value: None,
            span: Span::unknown(),
        }],
    ))
    .unwrap();
    let input = source.root_function_input().unwrap();
    let body = input.source().root_body().unwrap();
    let region = input.function().lowering_roots().function_pair().region();
    let completion = verify_function_completion_v1(input).unwrap();
    let site = completion.explicit_site().unwrap().clone();
    let mut consumption =
        ResolvedFunctionCompletionConsumptionV1::new(input.owner(), completion).unwrap();
    consumption.claim_explicit_unit(&site, region).unwrap();
    let ready = consumption
        .finish(body.site(), body.statements().len() as u32, region)
        .unwrap();
    let product = FunctionSemanticResolverSessionV1::new(0)
        .unwrap()
        .resolve(
            crate::mir::resolved_semantics::FunctionSyntaxViewV1::from_ast(input.source().root())
                .unwrap(),
        )
        .unwrap();
    let owner = product.owner();
    let mut builder = MirBuilder::new();
    let session = builder.open_resolved_function_draft_seal_session_v1("seal_failure/0");
    let mut open = ReadyFunctionDraftSealV1::new(ready, BasicBlockId::new(0)).open(session);
    open.builder_mut()
        .function_state
        .resolved_binding_state
        .install(&product)
        .unwrap();
    open.builder_mut()
        .function_state
        .resolved_binding_state
        .finish(owner)
        .unwrap();
    open.builder_mut()
        .enter_function_for_test("seal_failure/0".to_owned());
    open.builder_mut()
        .function_state
        .current_function
        .as_mut()
        .unwrap()
        .get_block_mut(BasicBlockId::new(0))
        .unwrap()
        .set_terminator(MirInstruction::Return { value: None });

    let seal_rejection = match open.prepare() {
        Ok(_) => panic!("preterminated draft must reject before commit"),
        Err(rejected) => rejected,
    };
    let rejected = reject_draft_seal_typed(seal_rejection);
    assert_eq!(
        rejected.stage(),
        NormalFunctionDraftLoweringStageV1::DraftSeal(FunctionDraftSealStageV1::Exit)
    );
    assert!(matches!(
        rejected.cause(),
        NormalFunctionDraftLoweringCauseV1::DraftSeal(_)
    ));
    assert!(rejected.has_restoration_receipt());
    assert!(builder.function_state.current_function.is_none());
    assert!(builder.function_state.current_block.is_none());
}
