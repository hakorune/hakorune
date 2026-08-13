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
    let mut package_port = installed.begin_lowering(&context).expect("loan");
    let admission = NormalCatalogedBoxMethodDraftAdmissionV1::seal(match &key {
        SelectedNormalCallableKeyV1::Cataloged(source_key) => source_key.clone(),
        SelectedNormalCallableKeyV1::TopLevel(_) => unreachable!(),
    })
    .expect("catalog admission");
    let brand = crate::mir::module_invocation_identity::ModuleInvocationBrandV1::legacy_test();
    let mut builder = MirBuilder::new();
    let collector =
        crate::mir::builder::module_draft_collector::ModuleDraftCollectorV1::with_brand(brand);
    let mut invocation =
        crate::mir::builder::module_lowering_invocation::ModuleLoweringInvocationV1::with_collector(
            &mut builder,
            collector,
        );
    package_port.with_selected_cataloged_lowering_input(admission, |input| {
        invocation.with_module_port(|builder, module_port| {
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
        let capability = module_port
            .with_invocation_brand(|brand| {
                let capability = issue_selected_dynamic_v2_physical_capability_admission(plan, brand)
                    .expect("physical capability admission");
                assert_eq!(capability.aot_admission().plan_stamp(), brand);
                capability
            })
            .expect("collector brand must reach selected admission");
        let activation = capability
            .prepare_aot_activation()
            .expect("checked CallOut site-plan transport");
        let mut session = DynamicV2PhysicalEmissionSessionV1::begin(builder, activation)
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
            (12, immediate_i64),
            (13, immediate_bool),
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
        let i6_fault = function
            .get_block(session.i6_fault_block_for_test())
            .expect("I6 Fault landing");
        assert!(i6_fault.instructions.is_empty());
        assert!(matches!(
            i6_fault.terminator,
            Some(crate::mir::MirInstruction::CheckedCallOutFault {
                site_id: crate::mir::checked_callout::CheckedCallOutSiteIdV1(0),
            })
        ));
        assert!(i6_fault.successors.is_empty());
        let i7_fault = function
            .get_block(session.i7_fault_block_for_test())
            .expect("I7 Fault landing");
        assert!(matches!(
            i7_fault.instructions.as_slice(),
            [crate::mir::MirInstruction::CheckedCallOutEnd {
                site_id: crate::mir::checked_callout::CheckedCallOutSiteIdV1(0),
                lease_slot: crate::mir::checked_callout::CheckedCallOutLeaseSlotIdV1(0),
            }]
        ));
        assert!(matches!(
            i7_fault.terminator,
            Some(crate::mir::MirInstruction::CheckedCallOutFault {
                site_id: crate::mir::checked_callout::CheckedCallOutSiteIdV1(1),
            })
        ));
        assert!(i7_fault.successors.is_empty());
        let i7_normal_block = session.i7_normal_block_for_test();
        let i7_normal = function
            .get_block(i7_normal_block)
            .expect("I7 Normal landing");
        assert_eq!(i7_normal.predecessors.len(), 1);
        let normal_result_index = i7_normal
            .instructions
            .iter()
            .position(|instruction| {
                matches!(
                    instruction,
                    crate::mir::MirInstruction::CheckedCallOutNormalResult {
                        site_id: crate::mir::checked_callout::CheckedCallOutSiteIdV1(1),
                        ..
                    }
                )
            })
            .expect("I7 Normal projection");
        assert!(matches!(
            i7_normal.instructions.get(normal_result_index + 1),
            Some(crate::mir::MirInstruction::Const {
                value: crate::mir::ConstValue::Integer(0),
                ..
            })
        ));
        assert!(matches!(
            i7_normal.instructions.get(normal_result_index + 2),
            Some(crate::mir::MirInstruction::Compare {
                op: crate::mir::CompareOp::Lt,
                ..
            })
        ));
        assert!(matches!(
            i7_normal.terminator,
            Some(crate::mir::MirInstruction::Branch {
                then_bb,
                else_bb,
                ..
            }) if then_bb == target_blocks[3] && else_bb == target_blocks[4]
        ));
        let then_terminal = function
            .get_block(target_blocks[3])
            .expect("ThenTerminal");
        assert!(matches!(
            then_terminal.instructions.last(),
            Some(crate::mir::MirInstruction::CheckedCallOutEnd {
                site_id: crate::mir::checked_callout::CheckedCallOutSiteIdV1(0),
                lease_slot: crate::mir::checked_callout::CheckedCallOutLeaseSlotIdV1(0),
            })
        ));
        assert!(then_terminal.terminator.is_none());
        assert!(then_terminal.is_sealed());
        let continuation = function
            .get_block(target_blocks[4])
            .expect("Continuation");
        assert!(matches!(
            continuation.instructions.as_slice(),
            [
                crate::mir::MirInstruction::Const {
                    value: crate::mir::ConstValue::Integer(1),
                    ..
                },
                crate::mir::MirInstruction::BinOp {
                    op: crate::mir::BinaryOp::Add,
                    ..
                },
                crate::mir::MirInstruction::CheckedCallOutEnd {
                    site_id: crate::mir::checked_callout::CheckedCallOutSiteIdV1(0),
                    lease_slot: crate::mir::checked_callout::CheckedCallOutLeaseSlotIdV1(0),
                },
            ]
        ));
        assert!(matches!(
            continuation.terminator,
            Some(crate::mir::MirInstruction::Jump { target, .. })
                if target == target_blocks[1]
        ));
        assert!(continuation.is_sealed());
        assert_eq!(continuation.predecessors.len(), 1);
        assert!(continuation.predecessors.contains(&i7_normal_block));
        let header = function
            .get_block(target_blocks[1])
            .expect("Header");
        assert!(matches!(
            header.terminator,
            Some(crate::mir::MirInstruction::Branch {
                then_bb,
                else_bb,
                ..
            }) if then_bb == target_blocks[2] && else_bb == target_blocks[5]
        ));
        assert!(header.is_sealed());
        let phi = header
            .instructions
            .iter()
            .find_map(|instruction| match instruction {
                crate::mir::MirInstruction::Phi { inputs, .. } => Some(inputs),
                _ => None,
            })
            .expect("Header induction PHI");
        assert_eq!(phi.len(), 2);
        assert!(phi.contains(&(target_blocks[0], crate::mir::ValueId::new(1))));
        assert!(phi.iter().any(|(block, _)| *block == target_blocks[4]));
        assert!(session.current_instruction_count() >= 5);
        let completed = session
            .finish_unpublished_draft()
            .expect("profile close and exact-two DraftSeal");
        assert_eq!(completed.key().owner(), "ParserScanLoopBox");
        assert_eq!(completed.key().name(), "skip_while");
        assert_eq!(completed.draft().signature.name, "ParserScanLoopBox.skip_while/4");
        assert_eq!(completed.draft().signature.return_type, crate::mir::MirType::Integer);
        assert!(builder.function_state.current_function.is_none());
        let receipt = module_port
            .commit_cataloged_box_method_completed(completed)
            .expect("branded cataloged Box-method collector handoff");
        assert_eq!(receipt.brand(), brand);
        assert_eq!(receipt.payload().symbol(), "ParserScanLoopBox.skip_while/4");
        assert_eq!(receipt.payload().arity(), 4);
        assert_eq!(receipt.payload().policy(), crate::mir::builder::module_draft_collector::DraftPublicationPolicyV1::CanonicalRejectDuplicate);
        })
    })
    .expect("selected loan");
}
