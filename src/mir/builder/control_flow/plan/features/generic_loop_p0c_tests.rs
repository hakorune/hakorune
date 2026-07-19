//! LOOP0-P0c disconnected evidence for the complete located GenericLoop.
//!
//! This module observes the already sealed plan only. It does not lower MIR,
//! claim source coverage, or introduce a second provenance authority.

use std::collections::{BTreeMap, BTreeSet};

use crate::mir::builder::control_flow::plan::{CoreEffectPlan, CorePlan, PlanVerifier};
use crate::mir::callable_result_representation::{
    actual_parser_add_fixture, VerifiedCallableResultLoopClaimScheduleV1,
};
use crate::mir::resolved_semantics::{SourceExprSiteV1, SourcePathSegmentV1};
use crate::mir::BasicBlockId;

use super::generic_loop_whole_parity_tests::{located_loop, run_located, run_raw};
use crate::mir::builder::control_flow::plan::generic_loop::facts::extract::test_support::with_default_and_strict_modes;

#[test]
fn p0c_seals_verifier_schedule_and_short_circuit_cfg_without_execution() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    with_default_and_strict_modes(|mode| {
        let activation = actual_parser_add_fixture::plan();
        let caller = actual_parser_add_fixture::caller(&activation);
        assert_eq!(
            activation.rows_for(&caller).expect("activation rows").len(),
            15,
            "whole activation carrier"
        );
        let (port, loop_root) = located_loop(&activation, &caller);
        let (_, schedule_root) = located_loop(&activation, &caller);
        let expected =
            VerifiedCallableResultLoopClaimScheduleV1::verify(&activation, &caller, schedule_root)
                .expect("Loop schedule")
                .sites_in_source_order()
                .cloned()
                .collect::<Vec<_>>();
        let raw = run_raw(mode, &port, &loop_root);
        let located = run_located(&activation, &caller, port, loop_root);

        PlanVerifier::verify(&raw.plan).expect("raw plan verifier");
        PlanVerifier::verify(&located.plan).expect("located plan verifier");
        assert_eq!(located.schedule.len(), 9, "Loop activation domain");

        assert_eq!(located.schedule, expected);
        assert_short_circuit_cfg(&located.plan, &located.schedule);
    });
}

fn assert_short_circuit_cfg(plan: &CorePlan, schedule: &[SourceExprSiteV1]) {
    let CorePlan::Loop(loop_plan) = plan else {
        panic!("P0c requires a CoreLoop plan")
    };

    let branches = loop_plan
        .frag
        .branches
        .iter()
        .map(|branch| (branch.from, branch.then_target, branch.else_target))
        .collect::<BTreeSet<_>>();
    assert_eq!(
        branches,
        BTreeSet::from([
            (BasicBlockId(1), BasicBlockId(5), BasicBlockId(4)),
            (BasicBlockId(5), BasicBlockId(2), BasicBlockId(6)),
            (BasicBlockId(6), BasicBlockId(2), BasicBlockId(4)),
        ])
    );

    let mut located_by_block = BTreeMap::<BasicBlockId, Vec<SourceExprSiteV1>>::new();
    for (block, effects) in &loop_plan.block_effects {
        for effect in effects {
            collect_effect_sites(effect, &mut located_by_block.entry(*block).or_default());
        }
    }
    let condition_blocks = [BasicBlockId(1), BasicBlockId(5), BasicBlockId(6)];
    for block in condition_blocks {
        let sites = located_by_block
            .get(&block)
            .expect("condition block effects");
        assert!(
            !sites.is_empty(),
            "short-circuit block must retain a call site"
        );
        assert!(sites.iter().all(|site| {
            site.node()
                .segments()
                .iter()
                .any(|segment| matches!(segment, SourcePathSegmentV1::LoopCondition))
        }));
    }

    let observed = located_by_block
        .values()
        .flatten()
        .filter(|site| {
            site.node()
                .segments()
                .iter()
                .any(|segment| matches!(segment, SourcePathSegmentV1::LoopCondition))
        })
        .cloned()
        .collect::<BTreeSet<_>>();
    let scheduled = schedule
        .iter()
        .filter(|site| {
            site.node()
                .segments()
                .iter()
                .any(|segment| matches!(segment, SourcePathSegmentV1::LoopCondition))
        })
        .cloned()
        .collect::<BTreeSet<_>>();
    assert_eq!(observed, scheduled);
    assert_eq!(observed.len(), 3, "&&/|| condition calls are distinct");
}

fn collect_effect_sites(effect: &CoreEffectPlan, out: &mut Vec<SourceExprSiteV1>) {
    match effect {
        CoreEffectPlan::MethodCall { source, .. }
        | CoreEffectPlan::GlobalCall { source, .. }
        | CoreEffectPlan::ValueCall { source, .. }
        | CoreEffectPlan::ExternCall { source, .. } => {
            if let crate::mir::builder::control_flow::plan::CoreCallSourceV1::LocatedMethodCall(
                site,
            ) = source
            {
                out.push(site.clone());
            }
        }
        CoreEffectPlan::IfEffect {
            then_effects,
            else_effects,
            ..
        } => {
            for child in then_effects {
                collect_effect_sites(child, out);
            }
            if let Some(children) = else_effects {
                for child in children {
                    collect_effect_sites(child, out);
                }
            }
        }
        _ => {}
    }
}
