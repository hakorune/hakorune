//! Caller-zero unpublished-candidate boundary for the DirectAccum observer.

#![cfg(test)]

use super::{
    LoopBindingKeyV1, LoopOperationV1, LoopValueKeyV1, PhysicalRoleV1, TestExitOwnerV1,
    VerifiedLoopBindingProjectionV1, VerifiedLoopOperationScheduleV1,
};
use crate::mir::builder::emission::phi_lifecycle::{PhiToken, PhiTxn};
use crate::mir::builder::module_invocation_session::{
    BuilderCoreSeedPolicyV1, BuilderInvocationConfigV1, ModuleBuilderInvocationSessionV1,
};
use crate::mir::builder::resolved_lowering::canonical_cfg::CanonicalCfgSessionV1;
use crate::mir::builder::ssa::binding::{BindingSsaBuilderV1, MirBindingSsaAdapterV1};
use crate::mir::builder::MirBuilder;
use crate::mir::resolved_semantics::{BindingRefV1, FunctionOwnerIssuerV1};
use crate::mir::{
    BasicBlockId, BinaryOp, BindingId, CompareOp, ConstValue, MirInstruction, MirType, ValueId,
};
use std::collections::BTreeMap;

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

fn block(role: PhysicalRoleV1) -> BasicBlockId {
    match role {
        PhysicalRoleV1::Preheader => BasicBlockId::new(0),
        PhysicalRoleV1::Header => BasicBlockId::new(1),
        PhysicalRoleV1::Body => BasicBlockId::new(2),
        PhysicalRoleV1::Step => BasicBlockId::new(3),
        PhysicalRoleV1::After => BasicBlockId::new(4),
    }
}

fn install_blocks(builder: &mut MirBuilder) -> Result<(), String> {
    builder.enter_function_for_test("accum_candidate_observer/0".to_owned());
    for id in 0..=4 {
        builder.ensure_block_exists(BasicBlockId::new(id))?;
    }
    Ok(())
}

fn projection() -> VerifiedLoopBindingProjectionV1 {
    let owner = FunctionOwnerIssuerV1::new_for_compilation()
        .expect("owner issuer")
        .issue()
        .expect("function owner");
    VerifiedLoopBindingProjectionV1::try_new(
        owner,
        vec![
            (
                LoopBindingKeyV1::new(0),
                BindingRefV1::new(owner, BindingId::new(0)),
            ),
            (
                LoopBindingKeyV1::new(1),
                BindingRefV1::new(owner, BindingId::new(1)),
            ),
        ],
    )
    .expect("binding projection")
}

fn emit_const(builder: &mut MirBuilder, block: BasicBlockId, value: i64) -> ValueId {
    builder.start_new_block(block).expect("select const block");
    let dst = builder.alloc_value_for_test();
    builder
        .emit_for_test(MirInstruction::Const {
            dst,
            value: ConstValue::Integer(value),
        })
        .expect("emit const");
    builder
        .function_state
        .type_ctx
        .value_types
        .insert(dst, MirType::Integer);
    dst
}

fn emit_add(builder: &mut MirBuilder, block: BasicBlockId, lhs: ValueId, rhs: ValueId) -> ValueId {
    builder.start_new_block(block).expect("select add block");
    let dst = builder.alloc_value_for_test();
    builder
        .emit_for_test(MirInstruction::BinOp {
            dst,
            op: BinaryOp::Add,
            lhs,
            rhs,
        })
        .expect("emit add");
    builder
        .function_state
        .type_ctx
        .value_types
        .insert(dst, MirType::Integer);
    dst
}

fn emit_compare(
    builder: &mut MirBuilder,
    block: BasicBlockId,
    lhs: ValueId,
    rhs: ValueId,
) -> ValueId {
    builder
        .start_new_block(block)
        .expect("select compare block");
    let dst = builder.alloc_value_for_test();
    builder
        .emit_for_test(MirInstruction::Compare {
            dst,
            op: CompareOp::Lt,
            lhs,
            rhs,
        })
        .expect("emit compare");
    builder
        .function_state
        .type_ctx
        .value_types
        .insert(dst, MirType::Bool);
    dst
}

fn read_binding(
    builder: &mut MirBuilder,
    phis: &mut PhiTxn,
    ssa: &mut BindingSsaBuilderV1<PhiToken>,
    projection: &VerifiedLoopBindingProjectionV1,
    binding: LoopBindingKeyV1,
    block: BasicBlockId,
) -> Result<ValueId, String> {
    let binding = projection.resolve(binding);
    let mut adapter = MirBindingSsaAdapterV1::new(builder, phis);
    ssa.read(&mut adapter, binding, block)
        .map_err(|error| format!("{error:?}"))
}

fn define_binding(
    ssa: &mut BindingSsaBuilderV1<PhiToken>,
    projection: &VerifiedLoopBindingProjectionV1,
    binding: LoopBindingKeyV1,
    block: BasicBlockId,
    value: ValueId,
) -> Result<(), String> {
    ssa.define(projection.resolve(binding), block, value)
        .map_err(|error| format!("{error:?}"))
}

fn emit_jump(
    builder: &mut MirBuilder,
    cfg: &mut CanonicalCfgSessionV1,
    from: PhysicalRoleV1,
    to: PhysicalRoleV1,
) -> Result<(), String> {
    let function = builder
        .function_state
        .current_function
        .as_mut()
        .ok_or_else(|| "candidate function missing".to_owned())?;
    cfg.emit_jump(function, block(from), block(to))
        .map_err(|error| format!("{error:?}"))
}

fn seal(
    builder: &mut MirBuilder,
    cfg: &mut CanonicalCfgSessionV1,
    ssa: &mut BindingSsaBuilderV1<PhiToken>,
    phis: &mut PhiTxn,
    role: PhysicalRoleV1,
) -> Result<(), String> {
    let target = block(role);
    let witness = {
        let function = builder
            .function_state
            .current_function
            .as_mut()
            .ok_or_else(|| "candidate function missing".to_owned())?;
        cfg.seal_block(function, target)
            .map_err(|error| format!("{error:?}"))?
    };
    let mut adapter = MirBindingSsaAdapterV1::new(builder, phis);
    ssa.seal(&mut adapter, target, &witness)
        .map_err(|error| format!("{error:?}"))
}

fn emit_operations(
    builder: &mut MirBuilder,
    phis: &mut PhiTxn,
    ssa: &mut BindingSsaBuilderV1<PhiToken>,
    projection: &VerifiedLoopBindingProjectionV1,
    role: PhysicalRoleV1,
    operations: &[LoopOperationV1],
    values: &mut BTreeMap<LoopValueKeyV1, ValueId>,
    fail_after_first_effect: bool,
) -> Result<(), String> {
    for (index, operation) in operations.iter().copied().enumerate() {
        match operation {
            LoopOperationV1::ReadBinding { binding, result } => {
                let value = read_binding(builder, phis, ssa, projection, binding, block(role))?;
                values.insert(result, value);
            }
            LoopOperationV1::ConstI64 { result, value } => {
                values.insert(result, emit_const(builder, block(role), value));
            }
            LoopOperationV1::BinaryI64 {
                left,
                right,
                result,
                ..
            } => {
                let lhs = values
                    .get(&left)
                    .copied()
                    .ok_or_else(|| format!("missing binary lhs {left:?}"))?;
                let rhs = values
                    .get(&right)
                    .copied()
                    .ok_or_else(|| format!("missing binary rhs {right:?}"))?;
                values.insert(result, emit_add(builder, block(role), lhs, rhs));
            }
            LoopOperationV1::CompareI64 {
                left,
                right,
                result,
                ..
            } => {
                let lhs = values
                    .get(&left)
                    .copied()
                    .ok_or_else(|| format!("missing compare lhs {left:?}"))?;
                let rhs = values
                    .get(&right)
                    .copied()
                    .ok_or_else(|| format!("missing compare rhs {right:?}"))?;
                values.insert(result, emit_compare(builder, block(role), lhs, rhs));
            }
            LoopOperationV1::WriteBinding { binding, value } => {
                let value = values
                    .get(&value)
                    .copied()
                    .ok_or_else(|| "missing write value".to_owned())?;
                define_binding(ssa, projection, binding, block(role), value)?;
            }
        }
        if fail_after_first_effect && index == 0 {
            return Err("injected operation failure".to_owned());
        }
    }
    Ok(())
}

fn observe_candidate(
    builder: &mut MirBuilder,
    schedule: &VerifiedLoopOperationScheduleV1,
    fail_after_first_effect: bool,
) -> Result<CandidateAlphaDigestV1, String> {
    install_blocks(builder)?;
    let projection = projection();
    let mut cfg = CanonicalCfgSessionV1::new();
    let mut ssa = BindingSsaBuilderV1::new(projection.owner);
    let mut phis = PhiTxn::begin("accum_candidate_observer");
    let preheader = block(PhysicalRoleV1::Preheader);
    let initial_i = emit_const(builder, preheader, 0);
    let initial_sum = emit_const(builder, preheader, 0);
    define_binding(
        &mut ssa,
        &projection,
        LoopBindingKeyV1::new(0),
        preheader,
        initial_i,
    )?;
    define_binding(
        &mut ssa,
        &projection,
        LoopBindingKeyV1::new(1),
        preheader,
        initial_sum,
    )?;
    emit_jump(
        builder,
        &mut cfg,
        PhysicalRoleV1::Preheader,
        PhysicalRoleV1::Header,
    )?;
    seal(
        builder,
        &mut cfg,
        &mut ssa,
        &mut phis,
        PhysicalRoleV1::Preheader,
    )?;

    let mut values = BTreeMap::from([
        (LoopValueKeyV1::new(0), initial_i),
        (LoopValueKeyV1::new(1), initial_sum),
    ]);
    for binding in schedule.header_reads.iter().copied() {
        let value = read_binding(
            builder,
            &mut phis,
            &mut ssa,
            &projection,
            binding,
            block(PhysicalRoleV1::Header),
        )?;
        values.insert(LoopValueKeyV1::new(binding.raw()), value);
    }
    if let Err(error) = emit_operations(
        builder,
        &mut phis,
        &mut ssa,
        &projection,
        PhysicalRoleV1::Header,
        &schedule.condition,
        &mut values,
        fail_after_first_effect,
    ) {
        let abort = phis.abort_on_err(builder, error.clone());
        return Err(format!("{error}; {abort}"));
    }
    let condition = values[&schedule.condition_result];
    {
        let function = builder
            .function_state
            .current_function
            .as_mut()
            .ok_or_else(|| "candidate function missing".to_owned())?;
        cfg.emit_branch(
            function,
            block(PhysicalRoleV1::Header),
            condition,
            block(PhysicalRoleV1::Body),
            block(PhysicalRoleV1::After),
        )
        .map_err(|error| format!("{error:?}"))?;
    }
    seal(builder, &mut cfg, &mut ssa, &mut phis, PhysicalRoleV1::Body)?;
    emit_operations(
        builder,
        &mut phis,
        &mut ssa,
        &projection,
        PhysicalRoleV1::Body,
        &schedule.body,
        &mut values,
        false,
    )?;
    emit_jump(
        builder,
        &mut cfg,
        PhysicalRoleV1::Body,
        PhysicalRoleV1::Step,
    )?;
    seal(builder, &mut cfg, &mut ssa, &mut phis, PhysicalRoleV1::Step)?;
    emit_jump(
        builder,
        &mut cfg,
        PhysicalRoleV1::Step,
        PhysicalRoleV1::Header,
    )?;
    seal(
        builder,
        &mut cfg,
        &mut ssa,
        &mut phis,
        PhysicalRoleV1::After,
    )?;
    TestExitOwnerV1::emit_unit_return(builder, block(PhysicalRoleV1::After));
    seal(
        builder,
        &mut cfg,
        &mut ssa,
        &mut phis,
        PhysicalRoleV1::Header,
    )?;
    ssa.finish().map_err(|error| format!("{error:?}"))?;
    let function = builder
        .function_state
        .current_function
        .as_ref()
        .ok_or_else(|| "candidate function missing".to_owned())?;
    cfg.finish(function).map_err(|error| format!("{error:?}"))?;
    phis.commit(builder).map_err(|error| error.to_string())?;
    let function = builder
        .function_state
        .current_function
        .as_ref()
        .expect("candidate function");
    let header = function.get_block(block(PhysicalRoleV1::Header)).unwrap();
    let body = function.get_block(block(PhysicalRoleV1::Body)).unwrap();
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
