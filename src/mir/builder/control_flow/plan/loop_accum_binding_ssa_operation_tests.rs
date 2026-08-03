//! Caller-zero physical operation parity for the DirectAccum schedule.

#![cfg(test)]

use super::{
    CanonicalLoopSsaSessionV1, LoopBindingKeyV1, LoopValueKeyV1, PhysicalRoleV1,
    VerifiedLoopOperationScheduleV1,
};
use crate::mir::{BasicBlockId, BinaryOp, CompareOp, ConstValue, MirInstruction};

#[test]
fn direct_operation_schedule_emits_through_one_binding_ssa_owner() {
    let schedule = VerifiedLoopOperationScheduleV1::from_direct_fixture(vec![
        LoopBindingKeyV1::new(0),
        LoopBindingKeyV1::new(1),
    ])
    .expect("verified direct schedule");
    let mut session = CanonicalLoopSsaSessionV1::new();
    session.emit_jump(PhysicalRoleV1::Preheader, PhysicalRoleV1::Header);
    session.seal(PhysicalRoleV1::Preheader);

    let mut values = session.entry_values.clone();
    session.emit_header_carriers(&schedule, &mut values);
    let repeated_i = session.read_at(LoopBindingKeyV1::new(0), PhysicalRoleV1::Header);
    assert_eq!(repeated_i, values[&LoopValueKeyV1::new(0)]);
    let header_receipt = session
        .emit_operations(PhysicalRoleV1::Header, &schedule.condition, &mut values)
        .expect("condition operations");
    let condition = values[&schedule.condition_result];
    session.emit_branch(PhysicalRoleV1::Header, condition);

    session.seal(PhysicalRoleV1::Body);
    let body_receipt = session
        .emit_operations(PhysicalRoleV1::Body, &schedule.body, &mut values)
        .expect("body operations");
    let visible_sum = session.read_at(LoopBindingKeyV1::new(1), PhysicalRoleV1::Body);
    assert_eq!(visible_sum, values[&LoopValueKeyV1::new(7)]);
    session.emit_jump(PhysicalRoleV1::Body, PhysicalRoleV1::Step);
    session.seal(PhysicalRoleV1::Step);
    session.emit_jump(PhysicalRoleV1::Step, PhysicalRoleV1::Header);
    session.seal(PhysicalRoleV1::After);
    session.emit_return(PhysicalRoleV1::After);
    session.seal(PhysicalRoleV1::Header);

    assert_eq!(header_receipt.emitted.len(), 3);
    assert_eq!(body_receipt.emitted.len(), 8);
    assert_eq!(body_receipt.values.len(), 11);
    let (builder, receipt) = session.finish_with_builder();
    assert_eq!(receipt.reads.len(), 7);
    assert_eq!(receipt.defines.len(), 4);
    assert_eq!(receipt.header_phi_inputs.len(), 2);
    assert!(receipt
        .header_phi_inputs
        .iter()
        .all(|inputs| inputs.len() == 2));

    let function = builder
        .function_state
        .current_function
        .as_ref()
        .expect("candidate function");
    let header = function
        .get_block(BasicBlockId::new(1))
        .expect("header block");
    let body = function
        .get_block(BasicBlockId::new(2))
        .expect("body block");
    assert_eq!(
        header
            .instructions
            .iter()
            .filter(|instruction| matches!(instruction, MirInstruction::Phi { .. }))
            .count(),
        2
    );
    assert_eq!(
        header
            .instructions
            .iter()
            .filter(|instruction| matches!(
                instruction,
                MirInstruction::Const {
                    value: ConstValue::Integer(3),
                    ..
                }
            ))
            .count(),
        1
    );
    assert_eq!(
        header
            .instructions
            .iter()
            .filter(|instruction| matches!(
                instruction,
                MirInstruction::Compare {
                    op: CompareOp::Lt,
                    ..
                }
            ))
            .count(),
        1
    );
    assert_eq!(
        body.instructions
            .iter()
            .filter(|instruction| matches!(
                instruction,
                MirInstruction::Const {
                    value: ConstValue::Integer(1),
                    ..
                }
            ))
            .count(),
        2
    );
    assert_eq!(
        body.instructions
            .iter()
            .filter(|instruction| matches!(
                instruction,
                MirInstruction::BinOp {
                    op: BinaryOp::Add,
                    ..
                }
            ))
            .count(),
        2
    );
}
