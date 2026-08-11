use super::*;
use crate::ast::ASTNode;
use crate::mir::builder::calls::CanonicalFunctionLoweringSessionV1;
use crate::mir::builder::resolved_lowering::canonical_ssa::CanonicalSsaFunctionSessionV2;
use crate::mir::builder::{
    issue_selected_dynamic_v2_emission_plan, CanonicalSameModuleCallableKeyV1, CompilationContext,
    MirBuilder, SelectedNormalCallableKeyV1,
};
use crate::mir::canonical_direct_static_call_capability::CanonicalDirectStaticCallCapabilityV1;
use crate::mir::compiler::a_prime_i64_physical_capability::issue_selected_a_prime_i64_physical_demand;
use crate::mir::function::MirParamDecl;
use crate::mir::normal_callable_semantic_package::issue_normal_callable_semantic_package_v1;
use crate::mir::resolved_control_flow::if_control::VerifiedResolvedFunctionIfControlV1;
use crate::mir::resolved_control_flow::verify_function_completion_v1;
use crate::mir::resolved_semantics::FunctionSemanticResolverSessionV1;
use crate::parser::{NyashParser, ParserBuildConfig};

fn open_canonical_session<'input, 'builder>(
    builder: &'builder mut MirBuilder,
    input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'input>,
    completion: crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1,
) -> (
    CanonicalFunctionLoweringSessionV1<'builder>,
    CanonicalSsaFunctionSessionV2<'input>,
) {
    let root = input.source().root();
    let ASTNode::FunctionDeclaration {
        name,
        params,
        param_decls,
        body,
        return_type_name,
        attrs,
        uses,
        ..
    } = root
    else {
        panic!("selected fixture root must be a function")
    };
    let function_name = format!("{name}/{}", params.len());
    let mut outer = builder.open_resolved_function_draft_seal_session_v1(&function_name);
    let canonical = {
        let draft_builder = outer.builder_view_mut_for_lowering();
        draft_builder
            .function_state
            .resolved_binding_state
            .install(input.function())
            .expect("resolver authority");
        draft_builder
            .create_function_skeleton(function_name, params, body)
            .expect("function skeleton");
        draft_builder.set_current_function_declared_signature(
            param_decls
                .iter()
                .map(|decl| MirParamDecl {
                    name: decl.name.clone(),
                    declared_type_name: decl.declared_type_name.clone(),
                    implicit_receiver: false,
                })
                .collect(),
            return_type_name.clone(),
        );
        draft_builder.set_current_function_runes(attrs);
        draft_builder.set_current_function_declared_capability_uses(uses);
        let function = draft_builder
            .function_state
            .current_function
            .as_mut()
            .expect("function installed");
        CanonicalDirectStaticCallCapabilityV1::install_for_function(
            &mut function.metadata.canonical_direct_static_call_capabilities,
            true,
        )
        .expect("direct-call capability");
        let if_control = VerifiedResolvedFunctionIfControlV1::empty_for_loop_profile(input)
            .expect("loop-only If control");
        CanonicalSsaFunctionSessionV2::new(input, if_control, completion, 0)
            .expect("canonical session")
    };
    (outer, canonical)
}

#[test]
fn i8_leaf_emits_one_immediate_i64_in_unpublished_session() {
    let source =
        include_str!("../../../../../lang/src/compiler/parser/scan/parser_scan_loop_box.hako");
    let parsed = NyashParser::parse_normal_callable_program_with_build_config(
        source,
        ParserBuildConfig::default(),
    )
    .expect("parser fixture");
    let transformed =
        crate::r#macro::transform_normal_callable_program_v1(parsed).expect("callable transform");
    let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) = transformed else {
        panic!("fixture must remain source-backed")
    };
    let mut resolver = FunctionSemanticResolverSessionV1::new(193).expect("resolver");
    let package =
        issue_normal_callable_semantic_package_v1(&mut resolver, source).expect("semantic package");
    let mut context = CompilationContext::new();
    let installed = package
        .prepare_install(&mut context)
        .expect("catalog install")
        .commit();
    let key = SelectedNormalCallableKeyV1::Cataloged(
        CanonicalSameModuleCallableKeyV1::test_static_box_method(
            "ParserScanLoopBox",
            "skip_while",
            4,
        ),
    );
    let mut port = installed.begin_lowering(&context).expect("loan");
    port.with_selected_lowering_input(&key, |input| {
        let completion = verify_function_completion_v1(input.source()).expect("completion");
        let demand = issue_selected_a_prime_i64_physical_demand(&input).expect("A-prime demand");
        let plan = issue_selected_dynamic_v2_emission_plan(demand).expect("V2 plan");
        let mut builder = MirBuilder::new();
        let (outer, canonical) = open_canonical_session(&mut builder, input.source(), completion);
        let mut session = DynamicV2PhysicalEmissionSessionV1::begin(plan, outer, canonical)
            .expect("unpublished canonical session");
        let receipt = session.emit_i8_const().expect("I8 receipt");
        receipt.with_value(|value| assert_eq!(value.as_u32(), 1));
        drop(receipt);
        assert_eq!(session.current_instruction_count(), 1);
        let error = session
            .emit_i8_const()
            .expect_err("duplicate I8 must reject");
        assert_eq!(error, DynamicV2I8EmitterRejectV1::DuplicateI8Emission);
        assert_eq!(session.current_instruction_count(), 1);
        session.discard_unpublished();
        assert!(builder.function_state.current_function.is_none());
    })
    .expect("selected loan");
}
