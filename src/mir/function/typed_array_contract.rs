use super::types::ArrayStateTermId;
use crate::mir::{LocalSlotId, ValueId};
use crate::typed_array_contract_spec::ArrayElementContractSpec;

pub const TYPED_ARRAY_EXACT_NUMERIC_CAPABILITY: &str = "typed_array_exact_numeric_state_guard_v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TypedArrayContractBoundary {
    LocalInit,
    LocalReassign,
    ParameterEntry,
    ReturnExit,
    BoxFieldWrite,
    RecordConstruct,
    RecordWithUpdate,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TypedArrayContractSourceIdentity {
    LocalSlot(LocalSlotId),
    Parameter {
        formal_index: usize,
    },
    Return,
    BoxField {
        box_name: String,
        field_index: usize,
    },
    RecordField {
        schema_fingerprint: String,
        field_index: usize,
        update: bool,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TypedArrayBoundaryValue {
    Value(ValueId),
    FinalReturn,
}

/// Source-owned contract claim evidence. Builders publish these rows, while
/// semantic refresh rebuilds executable carriers from them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypedArrayContractSource {
    pub contract_id: String,
    pub boundary: TypedArrayContractBoundary,
    pub source_identity: TypedArrayContractSourceIdentity,
    pub boundary_value: TypedArrayBoundaryValue,
    pub element_spec: ArrayElementContractSpec,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypedArrayContractDisposition {
    RuntimeCheckedContract,
}

/// Refreshed semantic carrier. Runtime identity values are deliberately not
/// represented; `state_term` is parser/backend-neutral relation evidence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypedArrayElementContract {
    pub contract_id: String,
    pub boundary: TypedArrayContractBoundary,
    pub source_identity: TypedArrayContractSourceIdentity,
    pub boundary_value: TypedArrayBoundaryValue,
    pub state_term: Option<ArrayStateTermId>,
    pub element_spec: ArrayElementContractSpec,
    pub disposition: TypedArrayContractDisposition,
    pub runtime_check_required: bool,
    pub proof_elision_allowed: bool,
    pub backend_capability_required: String,
}
