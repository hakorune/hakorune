use crate::mir::{BasicBlockId, ValueId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserBoxKnownReceiverMethodSeedKind {
    CounterStepLocalI64,
    CounterStepCopyLocalI64,
    CounterStepChainLocalI64,
    CounterStepChainMicro,
    CounterStepMicro,
    PointSumLocalI64,
    PointSumCopyLocalI64,
    PointSumMicro,
}

impl std::fmt::Display for UserBoxKnownReceiverMethodSeedKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CounterStepLocalI64 => f.write_str("counter_step_local_i64"),
            Self::CounterStepCopyLocalI64 => f.write_str("counter_step_copy_local_i64"),
            Self::CounterStepChainLocalI64 => f.write_str("counter_step_chain_local_i64"),
            Self::CounterStepChainMicro => f.write_str("counter_step_chain_micro"),
            Self::CounterStepMicro => f.write_str("counter_step_micro"),
            Self::PointSumLocalI64 => f.write_str("point_sum_local_i64"),
            Self::PointSumCopyLocalI64 => f.write_str("point_sum_copy_local_i64"),
            Self::PointSumMicro => f.write_str("point_sum_micro"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum UserBoxKnownReceiverMethodSeedProof {
    CounterStepLocalI64Seed,
    CounterStepChainLocalI64Seed,
    CounterStepChainMicroSeed,
    CounterStepMicroSeed,
    PointSumLocalI64Seed,
    PointSumMicroSeed,
}

impl UserBoxKnownReceiverMethodSeedProof {
    pub(super) fn as_str(self) -> &'static str {
        match self {
            Self::CounterStepLocalI64Seed => "userbox_counter_step_local_i64_seed",
            Self::CounterStepChainLocalI64Seed => "userbox_counter_step_chain_local_i64_seed",
            Self::CounterStepChainMicroSeed => "userbox_counter_step_chain_micro_seed",
            Self::CounterStepMicroSeed => "userbox_counter_step_micro_seed",
            Self::PointSumLocalI64Seed => "userbox_point_sum_local_i64_seed",
            Self::PointSumMicroSeed => "userbox_point_sum_micro_seed",
        }
    }
}

impl std::fmt::Display for UserBoxKnownReceiverMethodSeedProof {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UserBoxKnownReceiverMethodSeedPayload {
    CounterStepI64 {
        base_i64: i64,
        delta_i64: i64,
    },
    PointSumI64 {
        x_i64: i64,
        y_i64: i64,
    },
    CounterStepLoopMicro {
        base_i64: i64,
        delta_i64: i64,
        ops: i64,
        step_i64: i64,
        known_receiver_count: usize,
        field_set_count: usize,
    },
    CounterStepChainI64 {
        base_i64: i64,
        delta_i64: i64,
        leaf_method_function: String,
        leaf_method_block_count: usize,
        leaf_method_block: BasicBlockId,
        ops: Option<i64>,
        known_receiver_count: usize,
        field_set_count: usize,
    },
    PointSumLoopMicro {
        x_i64: i64,
        y_i64: i64,
        ops: i64,
        sum_i64: i64,
        known_receiver_count: usize,
        field_set_count: usize,
        compare_lt_count: usize,
        branch_count: usize,
        jump_count: usize,
        ret_count: usize,
        add_count: usize,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserBoxKnownReceiverMethodSeedRoute {
    pub(super) kind: UserBoxKnownReceiverMethodSeedKind,
    pub(super) box_name: String,
    pub(super) method: String,
    pub(super) method_function: String,
    pub(super) block_count: usize,
    pub(super) method_block_count: usize,
    pub(super) block: BasicBlockId,
    pub(super) method_block: BasicBlockId,
    pub(super) newbox_instruction_index: usize,
    pub(super) copy_instruction_index: Option<usize>,
    pub(super) call_instruction_index: usize,
    pub(super) box_value: ValueId,
    pub(super) copy_value: Option<ValueId>,
    pub(super) result_value: ValueId,
    pub(super) proof: UserBoxKnownReceiverMethodSeedProof,
    pub(super) payload: UserBoxKnownReceiverMethodSeedPayload,
}

impl UserBoxKnownReceiverMethodSeedRoute {
    pub fn kind(&self) -> UserBoxKnownReceiverMethodSeedKind {
        self.kind
    }

    pub fn box_name(&self) -> &str {
        &self.box_name
    }

    pub fn method(&self) -> &str {
        &self.method
    }

    pub fn method_function(&self) -> &str {
        &self.method_function
    }

    pub fn block_count(&self) -> usize {
        self.block_count
    }

    pub fn method_block_count(&self) -> usize {
        self.method_block_count
    }

    pub fn block(&self) -> BasicBlockId {
        self.block
    }

    pub fn method_block(&self) -> BasicBlockId {
        self.method_block
    }

    pub fn newbox_instruction_index(&self) -> usize {
        self.newbox_instruction_index
    }

    pub fn copy_instruction_index(&self) -> Option<usize> {
        self.copy_instruction_index
    }

    pub fn call_instruction_index(&self) -> usize {
        self.call_instruction_index
    }

    pub fn box_value(&self) -> ValueId {
        self.box_value
    }

    pub fn copy_value(&self) -> Option<ValueId> {
        self.copy_value
    }

    pub fn result_value(&self) -> ValueId {
        self.result_value
    }

    pub fn proof(&self) -> &'static str {
        self.proof.as_str()
    }

    pub fn payload(&self) -> &UserBoxKnownReceiverMethodSeedPayload {
        &self.payload
    }
}
