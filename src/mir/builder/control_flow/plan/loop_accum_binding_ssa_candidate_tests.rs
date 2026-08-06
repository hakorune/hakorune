//! Caller-zero unpublished-candidate boundary for the DirectAccum observer.
//!
//! This adapter borrows the existing compile candidate and delegates all MIR
//! emission to the same borrowed emitter used by the owned session fixture.

#![cfg(test)]

use super::super::physical_digest_test_support::{
    observe_mir, AlphaFinalBindingWitnessV1, AlphaFunctionResultWitnessV1,
    AlphaPhysicalMirDigestV1, MirRoleWitnessV1,
};
use super::super::semantic_digest_test_support::{semantic_digest, AlphaPhysicalMirDigestV2};
use super::{
    emitter_tests, CanonicalLoopSsaStateV1, LoopBindingKeyV1, LoopOperationV1, LoopValueKeyV1,
    PhysicalRoleV1, VerifiedLoopOperationScheduleV1,
};
use crate::mir::builder::module_invocation_session::{
    BuilderCoreSeedPolicyV1, BuilderInvocationConfigV1, ModuleBuilderInvocationSessionV1,
};
use crate::mir::builder::MirBuilder;
use crate::mir::{BasicBlockId, MirInstruction, MirType, ValueId};
use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Eq)]
struct CandidateAlphaDigestV1 {
    header_phi_count: usize,
    header_compare_count: usize,
    body_const_count: usize,
    body_add_count: usize,
}

#[derive(Debug, PartialEq, Eq)]
struct CandidateObservationV1 {
    summary: CandidateAlphaDigestV1,
    alpha: AlphaPhysicalMirDigestV1,
    semantic: AlphaPhysicalMirDigestV2,
}

fn open_candidate(live: &MirBuilder) -> ModuleBuilderInvocationSessionV1 {
    let config = BuilderInvocationConfigV1::snapshot_with_policy(
        live,
        BuilderCoreSeedPolicyV1::ContinueLive,
    );
    ModuleBuilderInvocationSessionV1::open(live, config)
}

fn binding_label(binding: LoopBindingKeyV1) -> &'static str {
    match binding.raw() {
        0 => "phi:carrier:i",
        1 => "phi:carrier:sum",
        other => panic!("unexpected DirectAccum binding {other}"),
    }
}

fn logical_labels(
    schedule: &VerifiedLoopOperationScheduleV1,
) -> Result<BTreeMap<LoopValueKeyV1, String>, String> {
    let mut labels = BTreeMap::from([
        (
            LoopValueKeyV1::new(0),
            binding_label(LoopBindingKeyV1::new(0)).to_owned(),
        ),
        (
            LoopValueKeyV1::new(1),
            binding_label(LoopBindingKeyV1::new(1)).to_owned(),
        ),
    ]);
    for operation in schedule.condition.iter().chain(schedule.body.iter()) {
        match *operation {
            LoopOperationV1::ReadBinding { binding, result } => {
                labels.insert(result, binding_label(binding).to_owned());
            }
            LoopOperationV1::ConstI64 { result, value } => {
                labels.insert(result, format!("const:Integer({value})"));
            }
            LoopOperationV1::BinaryI64 {
                op,
                left,
                right,
                result,
            } => {
                let left = labels
                    .get(&left)
                    .ok_or_else(|| format!("missing logical binary lhs {left:?}"))?;
                let right = labels
                    .get(&right)
                    .ok_or_else(|| format!("missing logical binary rhs {right:?}"))?;
                let op = match op {
                    super::LoopBinaryI64OpV1::Add => "Add",
                    super::LoopBinaryI64OpV1::Sub => "Sub",
                };
                labels.insert(result, format!("bin:{op}:{left}:{right}"));
            }
            LoopOperationV1::CompareI64 {
                op,
                left,
                right,
                result,
            } => {
                let left = labels
                    .get(&left)
                    .ok_or_else(|| format!("missing logical compare lhs {left:?}"))?;
                let right = labels
                    .get(&right)
                    .ok_or_else(|| format!("missing logical compare rhs {right:?}"))?;
                let op = match op {
                    super::LoopCompareI64OpV1::Less => "Lt",
                    super::LoopCompareI64OpV1::LessEqual => "Le",
                    super::LoopCompareI64OpV1::Equal => "Eq",
                };
                labels.insert(result, format!("compare:{op}:{left}:{right}"));
            }
            LoopOperationV1::WriteBinding { .. } => {}
        }
    }
    Ok(labels)
}

fn candidate_alpha_digest(
    builder: &MirBuilder,
    schedule: &VerifiedLoopOperationScheduleV1,
    values: &BTreeMap<LoopValueKeyV1, ValueId>,
) -> Result<AlphaPhysicalMirDigestV1, String> {
    let function = builder
        .function_state
        .current_function
        .as_ref()
        .ok_or_else(|| "candidate function missing".to_owned())?;
    let header = function
        .get_block(BasicBlockId::new(1))
        .ok_or_else(|| "candidate header missing".to_owned())?;
    let preheader = function
        .get_block(BasicBlockId::new(0))
        .ok_or_else(|| "candidate preheader missing".to_owned())?;
    let mut physical_labels = BTreeMap::new();
    let mut entry_consts =
        preheader
            .instructions
            .iter()
            .filter_map(|instruction| match instruction {
                MirInstruction::Const { dst, .. } => Some(*dst),
                _ => None,
            });
    if let Some(value) = entry_consts.next() {
        physical_labels.insert(value, "binding:i".to_owned());
    }
    if let Some(value) = entry_consts.next() {
        physical_labels.insert(value, "binding:sum".to_owned());
    }
    for (binding, instruction) in schedule.header_reads.iter().zip(
        header
            .instructions
            .iter()
            .filter(|instruction| matches!(instruction, MirInstruction::Phi { .. })),
    ) {
        let MirInstruction::Phi { dst, .. } = instruction else {
            unreachable!()
        };
        physical_labels.insert(*dst, binding_label(*binding).to_owned());
    }
    let logical = logical_labels(schedule)?;
    for (key, value) in values {
        if let Some(label) = logical.get(key) {
            physical_labels.insert(*value, label.clone());
        }
    }
    let final_bindings = schedule
        .final_values
        .iter()
        .map(|(binding, value)| {
            let physical = values
                .get(value)
                .copied()
                .ok_or_else(|| format!("missing final physical value {value:?}"))?;
            let provenance = logical
                .get(value)
                .cloned()
                .ok_or_else(|| format!("missing final logical provenance {value:?}"))?;
            Ok(AlphaFinalBindingWitnessV1 {
                name: match binding.raw() {
                    0 => "i".to_owned(),
                    1 => "sum".to_owned(),
                    other => return Err(format!("unexpected final binding {other}")),
                },
                value: physical,
                provenance,
            })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let result = AlphaFunctionResultWitnessV1 {
        value: None,
        provenance: "unit".to_owned(),
        expected_type: MirType::Void,
    };
    observe_mir(
        function,
        &MirRoleWitnessV1::new(vec![
            ("P", BasicBlockId::new(0)),
            ("H", BasicBlockId::new(1)),
            ("B", BasicBlockId::new(2)),
            ("S", BasicBlockId::new(3)),
            ("A", BasicBlockId::new(4)),
        ])?,
        &physical_labels,
        &final_bindings,
        &result,
        &builder.function_state.type_ctx.value_types,
    )
}

fn observe_candidate(
    builder: &mut MirBuilder,
    schedule: &VerifiedLoopOperationScheduleV1,
    fail_after_first_effect: bool,
) -> Result<CandidateObservationV1, String> {
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
    let summary = CandidateAlphaDigestV1 {
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
    let alpha = candidate_alpha_digest(builder, schedule, &values)?;
    let semantic = semantic_digest(
        &alpha,
        &[
            "final:i:carrier:i:Integer",
            "final:sum:carrier:sum:Integer",
            "result:unit:Void",
        ],
    )?;
    builder.exit_function_for_test();
    Ok(CandidateObservationV1 {
        summary,
        alpha,
        semantic,
    })
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

#[test]
fn candidate_semantic_core_matches_legacy_and_auxiliary_is_explicit() {
    let live = MirBuilder::new();
    let schedule = VerifiedLoopOperationScheduleV1::from_direct_fixture(vec![
        LoopBindingKeyV1::new(0),
        LoopBindingKeyV1::new(1),
    ])
    .expect("verified direct schedule");
    let candidate = {
        let mut session = open_candidate(&live);
        let observation = observe_candidate(session.builder_mut(), &schedule, false)
            .expect("candidate semantic observation");
        let semantic = observation.semantic;
        drop(session);
        semantic
    };
    let legacy = super::super::physical_parity_tests::direct_legacy_semantic_digest();
    assert_eq!(candidate.semantic, legacy.semantic);
    assert!(candidate.legacy_aux.rows.is_empty());
    assert!(!legacy.legacy_aux.rows.is_empty());
}
