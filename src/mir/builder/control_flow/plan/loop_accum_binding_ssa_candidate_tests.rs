//! Caller-zero unpublished-candidate boundary for the DirectAccum observer.
//!
//! This adapter borrows the existing compile candidate and delegates all MIR
//! emission to the same borrowed emitter used by the owned session fixture.

#![cfg(test)]

use super::{
    emitter_tests, CanonicalLoopSsaStateV1, LoopBindingKeyV1, PhysicalRoleV1,
    VerifiedLoopOperationScheduleV1,
};
use crate::mir::builder::module_invocation_session::{
    BuilderCoreSeedPolicyV1, BuilderInvocationConfigV1, ModuleBuilderInvocationSessionV1,
};
use crate::mir::builder::MirBuilder;
use crate::mir::{BasicBlockId, MirInstruction};

#[derive(Debug, PartialEq, Eq)]
struct CandidateAlphaDigestV1 {
    header_phi_count: usize,
    header_compare_count: usize,
    body_const_count: usize,
    body_add_count: usize,
}

fn open_candidate(live: &MirBuilder) -> ModuleBuilderInvocationSessionV1 {
    let config = BuilderInvocationConfigV1::snapshot_with_policy(
        live,
        BuilderCoreSeedPolicyV1::ContinueLive,
    );
    ModuleBuilderInvocationSessionV1::open(live, config)
}

fn observe_candidate(
    builder: &mut MirBuilder,
    schedule: &VerifiedLoopOperationScheduleV1,
    fail_after_first_effect: bool,
) -> Result<CandidateAlphaDigestV1, String> {
    builder.enter_function_for_test("accum_candidate_observer/0".to_owned());
    let mut state: CanonicalLoopSsaStateV1 = emitter_tests::new_state(builder);
    let mut emitter = emitter_tests::CanonicalLoopSsaEmitterV1::new(builder, &mut state);
    emitter.seed_entries();
    emitter.emit_jump(PhysicalRoleV1::Preheader, PhysicalRoleV1::Header);
    emitter.seal(PhysicalRoleV1::Preheader);

    let mut values = emitter.entry_values().clone();
    emitter.emit_header_carriers(schedule, &mut values);
    if fail_after_first_effect {
        emitter.emit_operations(
            PhysicalRoleV1::Header,
            &schedule.condition[..1],
            &mut values,
        )?;
        return Err(emitter.abort_with_error("injected operation failure".to_owned()));
    }
    emitter.emit_operations(PhysicalRoleV1::Header, &schedule.condition, &mut values)?;
    let condition = values[&schedule.condition_result];
    emitter.emit_branch(PhysicalRoleV1::Header, condition);
    emitter.seal(PhysicalRoleV1::Body);
    emitter.emit_operations(PhysicalRoleV1::Body, &schedule.body, &mut values)?;
    emitter.emit_jump(PhysicalRoleV1::Body, PhysicalRoleV1::Step);
    emitter.seal(PhysicalRoleV1::Step);
    emitter.emit_jump(PhysicalRoleV1::Step, PhysicalRoleV1::Header);
    emitter.seal(PhysicalRoleV1::After);
    emitter.emit_return(PhysicalRoleV1::After);
    emitter.seal(PhysicalRoleV1::Header);
    emitter.finish()?;
    drop(emitter);
    let function = builder
        .function_state
        .current_function
        .as_ref()
        .ok_or_else(|| "candidate function missing".to_owned())?;
    let header = function
        .get_block(BasicBlockId::new(1))
        .ok_or_else(|| "candidate header missing".to_owned())?;
    let body = function
        .get_block(BasicBlockId::new(2))
        .ok_or_else(|| "candidate body missing".to_owned())?;
    let digest = CandidateAlphaDigestV1 {
        header_phi_count: header
            .instructions
            .iter()
            .filter(|instruction| matches!(instruction, MirInstruction::Phi { .. }))
            .count(),
        header_compare_count: header
            .instructions
            .iter()
            .filter(|instruction| matches!(instruction, MirInstruction::Compare { .. }))
            .count(),
        body_const_count: body
            .instructions
            .iter()
            .filter(|instruction| matches!(instruction, MirInstruction::Const { .. }))
            .count(),
        body_add_count: body
            .instructions
            .iter()
            .filter(|instruction| matches!(instruction, MirInstruction::BinOp { .. }))
            .count(),
    };
    builder.exit_function_for_test();
    Ok(digest)
}

#[test]
fn candidate_observer_failure_drops_candidate_and_fresh_success_matches() {
    let live = MirBuilder::new();
    let before = live.loop_candidate_test_fingerprint();
    let schedule = VerifiedLoopOperationScheduleV1::from_direct_fixture(vec![
        LoopBindingKeyV1::new(0),
        LoopBindingKeyV1::new(1),
    ])
    .expect("verified direct schedule");

    let baseline = {
        let mut candidate = open_candidate(&live);
        let digest = observe_candidate(candidate.builder_mut(), &schedule, false)
            .expect("baseline candidate");
        drop(candidate);
        digest
    };
    assert_eq!(live.loop_candidate_test_fingerprint(), before);

    {
        let mut failed = open_candidate(&live);
        let error = observe_candidate(failed.builder_mut(), &schedule, true)
            .expect_err("injected operation failure");
        assert!(error.contains("injected operation failure"));
        assert!(error.contains("txn_abort"));
        drop(failed);
    }
    assert_eq!(live.loop_candidate_test_fingerprint(), before);

    let fresh = {
        let mut candidate = open_candidate(&live);
        let digest =
            observe_candidate(candidate.builder_mut(), &schedule, false).expect("fresh candidate");
        drop(candidate);
        digest
    };
    assert_eq!(fresh, baseline);
    assert_eq!(live.loop_candidate_test_fingerprint(), before);
}
