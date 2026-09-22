use crate::mir::builder::control_flow::edgecfg::api::Frag;
use crate::mir::builder::control_flow::plan::features::generic_loop_context::GenericLoopV1SourceLoweringContextV1;
use crate::mir::builder::control_flow::plan::lowering_context::PlanLoweringContext;
use crate::mir::builder::control_flow::plan::{
    CoreLoopFinalValuesV1, CoreLoopPlan, CorePlan, LoopStepMode,
};
use crate::mir::builder::normal_callable_loop_source_facts::source_final_values_for_test;
use crate::mir::builder::MirBuilder;
use crate::mir::{BasicBlockId, ValueId};
use std::collections::BTreeMap;

#[test]
fn source_final_values_follow_real_plan_remap_without_changing_identity() {
    let original = source_final_values_for_test();
    let CoreLoopFinalValuesV1::Source(before) = &original else {
        panic!("source witness");
    };
    let plan = plan_with_final_values(original.clone());
    let map = BTreeMap::from([
        (ValueId::new(100), ValueId::new(200)),
        (ValueId::new(101), ValueId::new(201)),
    ]);
    let CorePlan::Loop(remapped) = super::remapper::remap_plan(
        &mut MirBuilder::new(),
        &mut BTreeMap::new(),
        &map,
        CorePlan::Loop(plan),
    ) else {
        panic!("loop transport");
    };
    let CoreLoopFinalValuesV1::Source(after) = &remapped.final_values else {
        panic!("no raw downgrade");
    };
    assert_eq!(before.owner(), after.owner());
    assert_eq!(before.site(), after.site());
    assert_eq!(before.rows().len(), after.rows().len());
    for (old, new) in before.rows().iter().zip(after.rows()) {
        assert_eq!(old.slot(), new.slot());
        assert_eq!(old.binding(), new.binding());
        assert_eq!(new.value(), map[&old.value()]);
    }
    assert!(remapped
        .final_values
        .raw_rows()
        .unwrap_err()
        .contains("source-at-raw-consumer"));
    assert!(super::verifier::find_unremapped_value_id(&CorePlan::Loop(remapped), &map).is_none());
}

#[test]
fn source_final_values_require_completion_capability_even_after_clone() {
    let values = source_final_values_for_test().clone();
    let CoreLoopFinalValuesV1::Source(source) = values else {
        panic!("source witness");
    };
    let context = GenericLoopV1SourceLoweringContextV1::new(false, false);
    assert!(context
        .validate_source_loop_completion(&source)
        .unwrap_err()
        .contains("source-completion-unavailable"));
    assert!(context
        .publish_source_loop_completion(&source)
        .unwrap_err()
        .contains("source-completion-unavailable"));
}

fn plan_with_final_values(final_values: CoreLoopFinalValuesV1) -> CoreLoopPlan {
    let block = BasicBlockId::new(1);
    CoreLoopPlan {
        preheader_bb: block,
        preheader_is_fresh: false,
        header_bb: block,
        body_bb: block,
        step_bb: block,
        continue_target: block,
        after_bb: block,
        found_bb: block,
        body: vec![],
        cond_loop: ValueId::new(1),
        cond_match: ValueId::new(1),
        block_effects: vec![],
        phis: vec![],
        frag: Frag::new(block),
        final_values,
        step_mode: LoopStepMode::ExtractToStepBb,
        has_explicit_step: true,
    }
}

#[test]
fn source_final_values_stop_before_physical_effects_without_completion_owner() {
    use crate::mir::builder::control_flow::plan::lowerer::PlanLowerer;
    let plan = CorePlan::Loop(plan_with_final_values(source_final_values_for_test()));
    let mut builder = MirBuilder::new();
    let context = GenericLoopV1SourceLoweringContextV1::new(false, false);
    let error = PlanLowerer::lower(&mut builder, plan, &context).unwrap_err();
    assert!(error.contains("source-completion-unavailable"), "{error}");
    assert!(builder.function_state.current_function.is_none());
    assert!(builder.function_state.current_block.is_none());
}
