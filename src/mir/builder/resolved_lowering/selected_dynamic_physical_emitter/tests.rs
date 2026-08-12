use super::*;
use crate::mir::builder::{
    issue_selected_dynamic_v2_emission_plan, CanonicalSameModuleCallableKeyV1, CompilationContext,
    MirBuilder, SelectedNormalCallableKeyV1,
};
use crate::mir::compiler::a_prime_i64_physical_capability::issue_selected_a_prime_i64_physical_demand;
use crate::mir::normal_callable_semantic_package::issue_normal_callable_semantic_package_v1;
use crate::mir::resolved_semantics::FunctionSemanticResolverSessionV1;
use crate::parser::{NyashParser, ParserBuildConfig};

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
        let demand = issue_selected_a_prime_i64_physical_demand(&input).expect("A-prime demand");
        let plan = issue_selected_dynamic_v2_emission_plan(demand).expect("V2 plan");
        let mut builder = MirBuilder::new();
        let mut session = DynamicV2PhysicalEmissionSessionV1::begin(&mut builder, plan)
            .expect("unpublished canonical session");
        let receipt = session.emit_i8_const().expect("I8 receipt");
        receipt.with_value(|value| assert_ne!(value.as_u32(), 0));
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
