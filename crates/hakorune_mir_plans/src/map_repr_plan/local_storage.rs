use hakorune_mir_core::{BasicBlockId, ValueId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalMapStorageRealizationPlan {
    receiver_value: ValueId,
    representation: &'static str,
    candidate_set_count: usize,
    candidate_scalar_get_count: usize,
    publication_materialization_required: bool,
    backend_lowering_enabled: bool,
    runtime_helper_enabled: bool,
}

impl LocalMapStorageRealizationPlan {
    pub fn local_i64_key_map(
        receiver_value: ValueId,
        candidate_set_count: usize,
        candidate_scalar_get_count: usize,
    ) -> Self {
        Self {
            receiver_value,
            representation: "local_i64_key_map",
            candidate_set_count,
            candidate_scalar_get_count,
            publication_materialization_required: true,
            backend_lowering_enabled: false,
            runtime_helper_enabled: false,
        }
    }

    pub fn receiver_value(&self) -> ValueId {
        self.receiver_value
    }

    pub fn representation(&self) -> &'static str {
        self.representation
    }

    pub fn candidate_set_count(&self) -> usize {
        self.candidate_set_count
    }

    pub fn candidate_scalar_get_count(&self) -> usize {
        self.candidate_scalar_get_count
    }

    pub fn publication_materialization_required(&self) -> bool {
        self.publication_materialization_required
    }

    pub fn backend_lowering_enabled(&self) -> bool {
        self.backend_lowering_enabled
    }

    pub fn runtime_helper_enabled(&self) -> bool {
        self.runtime_helper_enabled
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalI64MapDirectStoragePlan {
    receiver_value: ValueId,
    representation: &'static str,
    known_i64_key_set_count: usize,
    scalar_get_count: usize,
    entry_value_tracking_enabled: bool,
    publication_materialization_required: bool,
    backend_lowering_enabled: bool,
    runtime_helper_enabled: bool,
}

impl LocalI64MapDirectStoragePlan {
    pub fn closed_world_i64_key_value_table(
        receiver_value: ValueId,
        known_i64_key_set_count: usize,
        scalar_get_count: usize,
    ) -> Self {
        Self {
            receiver_value,
            representation: "closed_world_i64_key_value_table",
            known_i64_key_set_count,
            scalar_get_count,
            entry_value_tracking_enabled: false,
            publication_materialization_required: true,
            backend_lowering_enabled: false,
            runtime_helper_enabled: false,
        }
    }

    pub fn receiver_value(&self) -> ValueId {
        self.receiver_value
    }

    pub fn representation(&self) -> &'static str {
        self.representation
    }

    pub fn known_i64_key_set_count(&self) -> usize {
        self.known_i64_key_set_count
    }

    pub fn scalar_get_count(&self) -> usize {
        self.scalar_get_count
    }

    pub fn entry_value_tracking_enabled(&self) -> bool {
        self.entry_value_tracking_enabled
    }

    pub fn publication_materialization_required(&self) -> bool {
        self.publication_materialization_required
    }

    pub fn backend_lowering_enabled(&self) -> bool {
        self.backend_lowering_enabled
    }

    pub fn runtime_helper_enabled(&self) -> bool {
        self.runtime_helper_enabled
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalI64MapEntryValueTrackingPlan {
    receiver_value: ValueId,
    set_block: BasicBlockId,
    set_instruction_index: usize,
    key_value: ValueId,
    value_value: ValueId,
    key_const_if_known: Option<i64>,
    value_const_if_known: Option<i64>,
    backend_lowering_enabled: bool,
    runtime_helper_enabled: bool,
}

impl LocalI64MapEntryValueTrackingPlan {
    pub fn from_parts(
        receiver_value: ValueId,
        set_block: BasicBlockId,
        set_instruction_index: usize,
        key_value: ValueId,
        value_value: ValueId,
        key_const_if_known: Option<i64>,
        value_const_if_known: Option<i64>,
    ) -> Self {
        Self {
            receiver_value,
            set_block,
            set_instruction_index,
            key_value,
            value_value,
            key_const_if_known,
            value_const_if_known,
            backend_lowering_enabled: false,
            runtime_helper_enabled: false,
        }
    }

    pub fn receiver_value(&self) -> ValueId {
        self.receiver_value
    }

    pub fn set_block(&self) -> BasicBlockId {
        self.set_block
    }

    pub fn set_instruction_index(&self) -> usize {
        self.set_instruction_index
    }

    pub fn key_value(&self) -> ValueId {
        self.key_value
    }

    pub fn value_value(&self) -> ValueId {
        self.value_value
    }

    pub fn key_const_if_known(&self) -> Option<i64> {
        self.key_const_if_known
    }

    pub fn value_const_if_known(&self) -> Option<i64> {
        self.value_const_if_known
    }

    pub fn backend_lowering_enabled(&self) -> bool {
        self.backend_lowering_enabled
    }

    pub fn runtime_helper_enabled(&self) -> bool {
        self.runtime_helper_enabled
    }
}
