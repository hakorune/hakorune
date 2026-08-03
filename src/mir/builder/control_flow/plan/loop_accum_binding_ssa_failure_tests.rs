//! Caller-zero late-failure and shared-PHI-transaction proof.

#![cfg(test)]

use super::{
    CanonicalLoopSsaSessionV1, LoopOperationV1, LoopValueKeyV1, PhysicalRoleV1,
    VerifiedLoopOperationScheduleV1,
};
use crate::mir::builder::MirBuilder;
use crate::mir::{BasicBlockId, MirInstruction};

fn header_phi_count(builder: &MirBuilder) -> usize {
    builder
        .function_state
        .current_function
        .as_ref()
        .expect("candidate function")
        .get_block(BasicBlockId::new(1))
        .expect("header block")
        .instructions
        .iter()
        .filter(|instruction| matches!(instruction, MirInstruction::Phi { .. }))
        .count()
}

#[test]
fn operation_failure_aborts_shared_phi_transaction_and_fresh_session_reuses() {
    let schedule = VerifiedLoopOperationScheduleV1::from_direct_fixture(vec![
        super::LoopBindingKeyV1::new(0),
        super::LoopBindingKeyV1::new(1),
    ])
    .expect("verified direct schedule");
    let mut session = CanonicalLoopSsaSessionV1::new();
    session.emit_jump(PhysicalRoleV1::Preheader, PhysicalRoleV1::Header);
    session.seal(PhysicalRoleV1::Preheader);
    let mut values = session.entry_values().clone();
    session.emit_header_carriers(&schedule, &mut values);
    assert_eq!(header_phi_count(&session.builder), 2);

    let failed = session
        .emit_operations(
            PhysicalRoleV1::Header,
            &[
                LoopOperationV1::ConstI64 {
                    result: LoopValueKeyV1::new(11),
                    value: 1,
                },
                LoopOperationV1::BinaryI64 {
                    op: super::LoopBinaryI64OpV1::Add,
                    left: LoopValueKeyV1::new(99),
                    right: LoopValueKeyV1::new(11),
                    result: LoopValueKeyV1::new(12),
                },
            ],
            &mut values,
        )
        .expect_err("missing operation input must fail after a prior MIR effect");
    assert!(failed.contains("missing binary lhs"));
    assert!(failed.contains("txn_abort"));

    let builder = session.into_builder();
    assert_eq!(header_phi_count(&builder), 0);

    let mut fresh = CanonicalLoopSsaSessionV1::new();
    fresh.emit_jump(PhysicalRoleV1::Preheader, PhysicalRoleV1::Header);
    fresh.seal(PhysicalRoleV1::Preheader);
    let mut fresh_values = fresh.entry_values().clone();
    fresh.emit_header_carriers(&schedule, &mut fresh_values);
    assert_eq!(header_phi_count(&fresh.builder), 2);
}
