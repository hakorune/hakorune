use super::*;

pub(crate) fn counter_step_copy_local_i64() -> UserBoxKnownReceiverMethodSeedRoute {
    UserBoxKnownReceiverMethodSeedRoute {
        kind: UserBoxKnownReceiverMethodSeedKind::CounterStepCopyLocalI64,
        box_name: "Counter".to_string(),
        method: "step".to_string(),
        method_function: "Counter.step/1".to_string(),
        block_count: 1,
        method_block_count: 1,
        block: BasicBlockId::new(0),
        method_block: BasicBlockId::new(1),
        newbox_instruction_index: 1,
        copy_instruction_index: Some(3),
        call_instruction_index: 4,
        box_value: ValueId::new(2),
        copy_value: Some(ValueId::new(3)),
        result_value: ValueId::new(4),
        proof: UserBoxKnownReceiverMethodSeedProof::CounterStepLocalI64Seed,
        payload: UserBoxKnownReceiverMethodSeedPayload::CounterStepI64 {
            base_i64: 41,
            delta_i64: 2,
        },
    }
}
