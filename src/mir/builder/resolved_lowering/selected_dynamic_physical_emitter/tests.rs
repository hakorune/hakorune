use super::*;
use crate::mir::builder::{
    issue_selected_dynamic_v2_emission_plan,
    issue_selected_dynamic_v2_physical_capability_admission, CanonicalSameModuleCallableKeyV1,
    CompilationContext, MirBuilder, NormalCatalogedBoxMethodDraftAdmissionV1,
    SelectedNormalCallableKeyV1,
};
use crate::mir::compiler::a_prime_i64_physical_capability::issue_selected_a_prime_i64_physical_demand;
use crate::mir::normal_callable_semantic_package::issue_normal_callable_semantic_package_v1;
use crate::mir::resolved_semantics::FunctionSemanticResolverSessionV1;
use crate::parser::{NyashParser, ParserBuildConfig};

#[test]
fn combined_corridor_emits_typed_prerequisites_and_callouts_in_unpublished_session() {
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
    let admission = NormalCatalogedBoxMethodDraftAdmissionV1::seal(match &key {
        SelectedNormalCallableKeyV1::Cataloged(source_key) => source_key.clone(),
        SelectedNormalCallableKeyV1::TopLevel(_) => unreachable!(),
    })
    .expect("catalog admission");
    port.with_selected_cataloged_lowering_input(admission, |input| {
        let demand = issue_selected_a_prime_i64_physical_demand(input).expect("A-prime demand");
        let plan = issue_selected_dynamic_v2_emission_plan(demand).expect("V2 plan");
        let target = |item| {
            plan.schedule_rows()
                .iter()
                .find(|row| row.item().raw() == item)
                .map(|row| row.target())
                .expect("scheduled operation")
        };
        assert_eq!(target(0), DynamicV2PhysicalBlockTargetV1::Header);
        assert_eq!(target(8), DynamicV2PhysicalBlockTargetV1::BodyPrelude);
        assert_eq!(target(11), DynamicV2PhysicalBlockTargetV1::ThenTerminal);
        assert_eq!(target(13), DynamicV2PhysicalBlockTargetV1::Continuation);
        plan.with_ledger(|ledger| {
            assert_eq!(
                ledger.outer_tail_target(),
                DynamicV2PhysicalBlockTargetV1::After
            );
        });
        let capability = issue_selected_dynamic_v2_physical_capability_admission(
            plan,
            std::num::NonZeroU64::new(1).expect("test registry generation"),
            crate::mir::module_invocation_identity::ModuleInvocationBrandV1::legacy_test(),
        )
        .expect("physical capability admission");
        let activation = capability
            .prepare_aot_activation()
            .expect("checked CallOut site-plan transport");
        let mut builder = MirBuilder::new();
        let mut session = DynamicV2PhysicalEmissionSessionV1::begin(&mut builder, activation)
            .expect("unpublished canonical session");
        assert_eq!(session.lifecycle.i6_site().0, 0);
        assert_eq!(session.lifecycle.i7_site().0, 1);
        assert_eq!(session.lifecycle.lease_slot().0, 0);
        assert_eq!(session.lifecycle.end_cutpoints().len(), 3);
        let target_blocks = session.target_blocks_for_test();
        assert_eq!(target_blocks.len(), 6);
        assert_eq!(
            target_blocks
                .iter()
                .collect::<std::collections::BTreeSet<_>>()
                .len(),
            6
        );
        let function = session
            .outer
            .as_ref()
            .expect("outer session")
            .builder_view()
            .function_state
            .current_function
            .as_ref()
            .expect("canonical skeleton");
        assert_eq!(target_blocks[0], function.entry_block);
        assert_ne!(target_blocks[0], target_blocks[1]);
        assert_eq!(function.signature.name, "ParserScanLoopBox.skip_while/4");
        assert_eq!(function.signature.params.len(), 4);
        assert_eq!(function.signature.params[1], crate::mir::MirType::Integer);
        assert_eq!(function.signature.params[2], crate::mir::MirType::Integer);
        assert_eq!(function.signature.return_type, crate::mir::MirType::Integer);
        assert_eq!(function.signature.effects, crate::mir::EffectMask::READ);
        assert!(function
            .metadata
            .checked_callout_plan(crate::mir::checked_callout::CheckedCallOutSiteIdV1(0))
            .is_some());
        assert!(function
            .metadata
            .checked_callout_plan(crate::mir::checked_callout::CheckedCallOutSiteIdV1(1))
            .is_some());
        let formal_header = &session.formal_header;
        assert_eq!(
            formal_header
                .formals()
                .iter()
                .map(|row| row.ordinal())
                .collect::<Vec<_>>(),
            vec![0, 1, 2, 3]
        );
        assert_eq!(
            formal_header
                .formals()
                .iter()
                .map(|row| row.recipe_value().raw())
                .collect::<Vec<_>>(),
            vec![0, 1, 2, 3]
        );
        assert_eq!(
            formal_header
                .formals()
                .iter()
                .map(|row| row.value().as_u32())
                .collect::<Vec<_>>(),
            vec![0, 1, 2, 3]
        );
        assert_eq!(formal_header.enter(), target_blocks[0]);
        assert_eq!(formal_header.header(), target_blocks[1]);
        assert_eq!(
            formal_header.header_current().physical_block(),
            target_blocks[1]
        );
        let immediate_i64 = crate::mir::builder::resolved_lowering::
            selected_dynamic_physical_capability::DynamicV2PhysicalRepresentationV1::ImmediateI64;
        let immediate_bool = crate::mir::builder::resolved_lowering::
            selected_dynamic_physical_capability::DynamicV2PhysicalRepresentationV1::ImmediateBool;
        let handle = crate::mir::builder::resolved_lowering::
            selected_dynamic_physical_capability::DynamicV2PhysicalRepresentationV1::EndAuthorizedHandle {
                lease_slot: crate::mir::checked_callout::CheckedCallOutLeaseSlotIdV1(0),
            };
        for (result, representation) in [
            (4, immediate_i64),
            (5, immediate_bool),
            (6, immediate_i64),
            (7, immediate_i64),
            (8, immediate_i64),
            (9, immediate_i64),
            (10, handle),
            (11, immediate_i64),
        ] {
            session
                .with_physical_value_for_test_as(
                    crate::mir::loop_recipe_contract::LoopValueKeyV1::new(result),
                    representation,
                    |_| (),
                )
                .expect("combined corridor value must be ledger-published");
        }
        let function = session
            .outer
            .as_ref()
            .expect("outer session")
            .builder_view()
            .function_state
            .current_function
            .as_ref()
            .expect("function remains unpublished");
        let callout_count = function
            .blocks
            .values()
            .filter(|block| {
                matches!(
                    block.terminator,
                    Some(crate::mir::MirInstruction::CheckedCallOut { .. })
                )
            })
            .count();
        assert_eq!(callout_count, 2);
        assert!(session.current_instruction_count() >= 5);
        session.discard_unpublished();
        assert!(builder.function_state.current_function.is_none());
    })
    .expect("selected loan");
}
