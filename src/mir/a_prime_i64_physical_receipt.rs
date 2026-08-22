//! Sealed post-session capability for the bounded A-prime LLVM lane.
//!
//! This is deliberately a transport receipt, not a source or Recipe owner.
//! Its private issuer is reserved for the canonical physical session.  Until
//! that session is connected, normal functions leave the metadata field
//! empty and the LLVM loader keeps the legacy path unchanged.

use crate::mir::checked_callout::CheckedCallOutSiteIdV1;
use crate::mir::linear_metadata_slot::LinearSlotObservation;
use crate::mir::{BasicBlockId, ValueId};
use std::collections::BTreeSet;

pub(crate) const A_PRIME_I64_PHYSICAL_RECEIPT_SCHEMA_VERSION: u32 = 2;
pub(crate) const A_PRIME_I64_FORMAL_PARAMETER_COUNT: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum APrimeI64BackendFamilyV1 {
    Llvm,
}

impl APrimeI64BackendFamilyV1 {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Llvm => "llvm",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum APrimeI64LaneV1 {
    ImmediateI64,
    OpaqueHandle,
}

impl APrimeI64LaneV1 {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::ImmediateI64 => "immediate_i64",
            Self::OpaqueHandle => "opaque_handle",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct APrimeI64ParameterReceiptV1 {
    pub(crate) role: String,
    pub(crate) formal_parameter_index: usize,
    pub(crate) value_id: ValueId,
    pub(crate) lane: APrimeI64LaneV1,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct APrimeI64CallArgumentReceiptV1 {
    pub(crate) ordinal: usize,
    pub(crate) role: String,
    pub(crate) value_id: ValueId,
    pub(crate) lane: APrimeI64LaneV1,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct APrimeI64CallEdgeReceiptV1 {
    /// Canonical physical identity.  Role/fingerprint remain diagnostics and
    /// cross-checks; they never select a call edge.
    pub(crate) site_id: CheckedCallOutSiteIdV1,
    pub(crate) role: String,
    pub(crate) target_fingerprint: String,
    pub(crate) receiver_role: String,
    pub(crate) receiver_value_id: ValueId,
    pub(crate) receiver_lane: APrimeI64LaneV1,
    pub(crate) arguments: Vec<APrimeI64CallArgumentReceiptV1>,
    pub(crate) result_value_id: ValueId,
    pub(crate) result_lane: APrimeI64LaneV1,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct APrimeI64ReturnReceiptV1 {
    pub(crate) site: String,
    pub(crate) block: BasicBlockId,
    pub(crate) value_id: ValueId,
    pub(crate) lane: APrimeI64LaneV1,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum APrimeI64PhysicalReceiptRejectV1 {
    WrongSchemaVersion(u32),
    UnsupportedBackend,
    MissingParameterRows,
    WrongParameterRows,
    WrongFormalParameterCount,
    DuplicateParameterRole,
    DuplicateParameterIndex,
    DuplicateParameterValue,
    ParameterRoleIndexMismatch,
    ParameterRoleMismatch,
    ParameterLaneMismatch,
    MissingCallEdgeRows,
    WrongCallEdgeRows,
    DuplicateCallSite,
    CallSiteRoleMismatch,
    CallRoleMismatch,
    EmptyCallTarget,
    CallTargetFingerprintMismatch,
    CallReceiverMismatch,
    CallArgumentMismatch,
    DuplicateCallResult,
    CallResultLaneMismatch,
    MissingReturnRows,
    WrongReturnRows,
    EmptyReturnSite,
    DuplicateReturnSite,
    ReturnSiteMismatch,
    ReturnLaneMismatch,
}

/// A complete, post-session capability.  No public constructor is provided;
/// the canonical physical session will be the sole issuer.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct APrimeI64PhysicalReceiptV1 {
    schema_version: u32,
    backend_family: APrimeI64BackendFamilyV1,
    formal_parameter_count: usize,
    parameters: Vec<APrimeI64ParameterReceiptV1>,
    call_edges: Vec<APrimeI64CallEdgeReceiptV1>,
    returns: Vec<APrimeI64ReturnReceiptV1>,
}

impl APrimeI64PhysicalReceiptV1 {
    /// Private issuer boundary for the canonical physical session.
    pub(in crate::mir) fn seal(
        backend_family: APrimeI64BackendFamilyV1,
        formal_parameter_count: usize,
        parameters: Vec<APrimeI64ParameterReceiptV1>,
        call_edges: Vec<APrimeI64CallEdgeReceiptV1>,
        returns: Vec<APrimeI64ReturnReceiptV1>,
    ) -> Result<Self, APrimeI64PhysicalReceiptRejectV1> {
        let receipt = Self {
            schema_version: A_PRIME_I64_PHYSICAL_RECEIPT_SCHEMA_VERSION,
            backend_family,
            formal_parameter_count,
            parameters,
            call_edges,
            returns,
        };
        receipt.validate()?;
        Ok(receipt)
    }

    #[cfg(test)]
    pub(crate) fn seal_for_test(
        backend_family: APrimeI64BackendFamilyV1,
        formal_parameter_count: usize,
        parameters: Vec<APrimeI64ParameterReceiptV1>,
        call_edges: Vec<APrimeI64CallEdgeReceiptV1>,
        returns: Vec<APrimeI64ReturnReceiptV1>,
    ) -> Result<Self, APrimeI64PhysicalReceiptRejectV1> {
        Self::seal(
            backend_family,
            formal_parameter_count,
            parameters,
            call_edges,
            returns,
        )
    }

    pub(crate) fn validate(&self) -> Result<(), APrimeI64PhysicalReceiptRejectV1> {
        if self.schema_version != A_PRIME_I64_PHYSICAL_RECEIPT_SCHEMA_VERSION {
            return Err(APrimeI64PhysicalReceiptRejectV1::WrongSchemaVersion(
                self.schema_version,
            ));
        }
        if self.backend_family != APrimeI64BackendFamilyV1::Llvm {
            return Err(APrimeI64PhysicalReceiptRejectV1::UnsupportedBackend);
        }
        if self.formal_parameter_count != A_PRIME_I64_FORMAL_PARAMETER_COUNT {
            return Err(APrimeI64PhysicalReceiptRejectV1::WrongFormalParameterCount);
        }

        if self.parameters.is_empty() {
            return Err(APrimeI64PhysicalReceiptRejectV1::MissingParameterRows);
        }
        if self.parameters.len() != 2 {
            return Err(APrimeI64PhysicalReceiptRejectV1::WrongParameterRows);
        }
        let mut parameter_roles = BTreeSet::new();
        let mut parameter_indices = BTreeSet::new();
        let mut parameter_values = BTreeSet::new();
        for row in &self.parameters {
            if !matches!(row.role.as_str(), "pos" | "end") {
                return Err(APrimeI64PhysicalReceiptRejectV1::ParameterRoleMismatch);
            }
            let expected_index = match row.role.as_str() {
                "pos" => 1,
                "end" => 2,
                _ => unreachable!("parameter role checked above"),
            };
            if row.formal_parameter_index != expected_index {
                return Err(APrimeI64PhysicalReceiptRejectV1::ParameterRoleIndexMismatch);
            }
            if !parameter_roles.insert(row.role.as_str()) {
                return Err(APrimeI64PhysicalReceiptRejectV1::DuplicateParameterRole);
            }
            if !parameter_indices.insert(row.formal_parameter_index) {
                return Err(APrimeI64PhysicalReceiptRejectV1::DuplicateParameterIndex);
            }
            if !parameter_values.insert(row.value_id.as_u32()) {
                return Err(APrimeI64PhysicalReceiptRejectV1::DuplicateParameterValue);
            }
            if row.lane != APrimeI64LaneV1::ImmediateI64 {
                return Err(APrimeI64PhysicalReceiptRejectV1::ParameterLaneMismatch);
            }
        }
        if parameter_roles.len() != 2 {
            return Err(APrimeI64PhysicalReceiptRejectV1::WrongParameterRows);
        }

        if self.call_edges.is_empty() {
            return Err(APrimeI64PhysicalReceiptRejectV1::MissingCallEdgeRows);
        }
        if self.call_edges.len() != 2 {
            return Err(APrimeI64PhysicalReceiptRejectV1::WrongCallEdgeRows);
        }
        let mut call_roles = BTreeSet::new();
        let mut call_sites = BTreeSet::new();
        let mut call_results = BTreeSet::new();
        for row in &self.call_edges {
            if !matches!(row.role.as_str(), "substring" | "index_of") {
                return Err(APrimeI64PhysicalReceiptRejectV1::CallRoleMismatch);
            }
            if !call_roles.insert(row.role.as_str()) {
                return Err(APrimeI64PhysicalReceiptRejectV1::CallRoleMismatch);
            }
            if !call_sites.insert(row.site_id.as_u32()) {
                return Err(APrimeI64PhysicalReceiptRejectV1::DuplicateCallSite);
            }
            if row.target_fingerprint.is_empty() {
                return Err(APrimeI64PhysicalReceiptRejectV1::EmptyCallTarget);
            }
            let (expected_target, expected_receiver, expected_arguments) = match row.role.as_str() {
                "substring" => (
                    "substring/2",
                    "src",
                    &[(0usize, "start"), (1usize, "end")][..],
                ),
                "index_of" => ("indexOf/1", "pred_chars", &[(0usize, "ch")][..]),
                _ => unreachable!("call role checked above"),
            };
            let expected_site = if row.role == "substring" { 0 } else { 1 };
            if row.site_id.as_u32() != expected_site {
                return Err(APrimeI64PhysicalReceiptRejectV1::CallSiteRoleMismatch);
            }
            if row.target_fingerprint != expected_target {
                return Err(APrimeI64PhysicalReceiptRejectV1::CallTargetFingerprintMismatch);
            }
            if row.receiver_role != expected_receiver
                || row.receiver_lane != APrimeI64LaneV1::OpaqueHandle
            {
                return Err(APrimeI64PhysicalReceiptRejectV1::CallReceiverMismatch);
            }
            if row.arguments.len() != expected_arguments.len()
                || row.arguments.iter().zip(expected_arguments.iter()).any(
                    |(actual, (ordinal, role))| {
                        actual.ordinal != *ordinal
                            || actual.role != *role
                            || actual.lane
                                != if row.role == "substring" {
                                    APrimeI64LaneV1::ImmediateI64
                                } else {
                                    APrimeI64LaneV1::OpaqueHandle
                                }
                    },
                )
            {
                return Err(APrimeI64PhysicalReceiptRejectV1::CallArgumentMismatch);
            }
            if !call_results.insert(row.result_value_id.as_u32()) {
                return Err(APrimeI64PhysicalReceiptRejectV1::DuplicateCallResult);
            }
            let expected_result_lane = if row.role == "substring" {
                APrimeI64LaneV1::OpaqueHandle
            } else {
                APrimeI64LaneV1::ImmediateI64
            };
            if row.result_lane != expected_result_lane {
                return Err(APrimeI64PhysicalReceiptRejectV1::CallResultLaneMismatch);
            }
        }

        if self.returns.is_empty() {
            return Err(APrimeI64PhysicalReceiptRejectV1::MissingReturnRows);
        }
        if self.returns.len() != 2 {
            return Err(APrimeI64PhysicalReceiptRejectV1::WrongReturnRows);
        }
        let mut return_sites = BTreeSet::new();
        for row in &self.returns {
            if row.site.is_empty() {
                return Err(APrimeI64PhysicalReceiptRejectV1::EmptyReturnSite);
            }
            if !return_sites.insert(row.site.as_str()) {
                return Err(APrimeI64PhysicalReceiptRejectV1::DuplicateReturnSite);
            }
            if row.lane != APrimeI64LaneV1::ImmediateI64 {
                return Err(APrimeI64PhysicalReceiptRejectV1::ReturnLaneMismatch);
            }
        }
        if return_sites != BTreeSet::from(["inner", "outer"]) {
            return Err(APrimeI64PhysicalReceiptRejectV1::ReturnSiteMismatch);
        }
        Ok(())
    }

    pub(crate) fn schema_version(&self) -> u32 {
        self.schema_version
    }

    pub(crate) fn backend_family(&self) -> APrimeI64BackendFamilyV1 {
        self.backend_family
    }

    pub(crate) fn formal_parameter_count(&self) -> usize {
        self.formal_parameter_count
    }

    pub(crate) fn parameters(&self) -> &[APrimeI64ParameterReceiptV1] {
        &self.parameters
    }

    pub(crate) fn call_edges(&self) -> &[APrimeI64CallEdgeReceiptV1] {
        &self.call_edges
    }

    pub(crate) fn call_edge(
        &self,
        site_id: CheckedCallOutSiteIdV1,
    ) -> Option<&APrimeI64CallEdgeReceiptV1> {
        self.call_edges.iter().find(|edge| edge.site_id == site_id)
    }

    pub(crate) fn returns(&self) -> &[APrimeI64ReturnReceiptV1] {
        &self.returns
    }
}

#[derive(Debug, PartialEq, Eq)]
enum APrimeI64PhysicalReceiptSlotState {
    Empty,
    Occupied(APrimeI64PhysicalReceiptV1),
    Consumed,
}

/// Linear storage boundary for the post-session receipt.
///
/// `FunctionMetadata` remains cloneable for the broad MIR compatibility
/// surface, but cloning metadata must never duplicate this capability. An
/// occupied slot therefore becomes `Consumed` in the clone. The only live
/// consumer will use `take_once`; JSON transport only borrows the receipt.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct APrimeI64PhysicalReceiptSlotV1 {
    state: APrimeI64PhysicalReceiptSlotState,
}

impl Default for APrimeI64PhysicalReceiptSlotV1 {
    fn default() -> Self {
        Self {
            state: APrimeI64PhysicalReceiptSlotState::Empty,
        }
    }
}

impl Clone for APrimeI64PhysicalReceiptSlotV1 {
    fn clone(&self) -> Self {
        Self {
            state: match self.state {
                APrimeI64PhysicalReceiptSlotState::Empty => {
                    APrimeI64PhysicalReceiptSlotState::Empty
                }
                APrimeI64PhysicalReceiptSlotState::Occupied(_)
                | APrimeI64PhysicalReceiptSlotState::Consumed => {
                    APrimeI64PhysicalReceiptSlotState::Consumed
                }
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum APrimeI64PhysicalReceiptSlotRejectV1 {
    AlreadyOccupied,
    AlreadyConsumed,
    Missing,
}

impl APrimeI64PhysicalReceiptSlotV1 {
    pub(crate) fn observe(&self) -> LinearSlotObservation<'_, APrimeI64PhysicalReceiptV1> {
        match &self.state {
            APrimeI64PhysicalReceiptSlotState::Empty => LinearSlotObservation::Empty,
            APrimeI64PhysicalReceiptSlotState::Occupied(receipt) => {
                LinearSlotObservation::Occupied(receipt)
            }
            APrimeI64PhysicalReceiptSlotState::Consumed => LinearSlotObservation::Scrubbed,
        }
    }

    pub(crate) fn borrow(&self) -> Option<&APrimeI64PhysicalReceiptV1> {
        match &self.state {
            APrimeI64PhysicalReceiptSlotState::Occupied(receipt) => Some(receipt),
            APrimeI64PhysicalReceiptSlotState::Empty
            | APrimeI64PhysicalReceiptSlotState::Consumed => None,
        }
    }

    pub(in crate::mir) fn install(
        &mut self,
        receipt: APrimeI64PhysicalReceiptV1,
    ) -> Result<(), APrimeI64PhysicalReceiptSlotRejectV1> {
        match self.state {
            APrimeI64PhysicalReceiptSlotState::Empty => {
                self.state = APrimeI64PhysicalReceiptSlotState::Occupied(receipt);
                Ok(())
            }
            APrimeI64PhysicalReceiptSlotState::Occupied(_) => {
                Err(APrimeI64PhysicalReceiptSlotRejectV1::AlreadyOccupied)
            }
            APrimeI64PhysicalReceiptSlotState::Consumed => {
                Err(APrimeI64PhysicalReceiptSlotRejectV1::AlreadyConsumed)
            }
        }
    }

    pub(in crate::mir) fn take_once(
        &mut self,
    ) -> Result<APrimeI64PhysicalReceiptV1, APrimeI64PhysicalReceiptSlotRejectV1> {
        match std::mem::replace(&mut self.state, APrimeI64PhysicalReceiptSlotState::Consumed) {
            APrimeI64PhysicalReceiptSlotState::Occupied(receipt) => Ok(receipt),
            APrimeI64PhysicalReceiptSlotState::Empty => {
                self.state = APrimeI64PhysicalReceiptSlotState::Empty;
                Err(APrimeI64PhysicalReceiptSlotRejectV1::Missing)
            }
            APrimeI64PhysicalReceiptSlotState::Consumed => {
                Err(APrimeI64PhysicalReceiptSlotRejectV1::AlreadyConsumed)
            }
        }
    }

    #[cfg(test)]
    pub(crate) fn install_for_test(
        &mut self,
        receipt: APrimeI64PhysicalReceiptV1,
    ) -> Result<(), APrimeI64PhysicalReceiptSlotRejectV1> {
        self.install(receipt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_receipt() -> APrimeI64PhysicalReceiptV1 {
        APrimeI64PhysicalReceiptV1::seal(
            APrimeI64BackendFamilyV1::Llvm,
            A_PRIME_I64_FORMAL_PARAMETER_COUNT,
            vec![
                APrimeI64ParameterReceiptV1 {
                    role: "pos".to_string(),
                    formal_parameter_index: 1,
                    value_id: ValueId::new(11),
                    lane: APrimeI64LaneV1::ImmediateI64,
                },
                APrimeI64ParameterReceiptV1 {
                    role: "end".to_string(),
                    formal_parameter_index: 2,
                    value_id: ValueId::new(12),
                    lane: APrimeI64LaneV1::ImmediateI64,
                },
            ],
            vec![
                APrimeI64CallEdgeReceiptV1 {
                    site_id: CheckedCallOutSiteIdV1::from_test(0),
                    role: "substring".to_string(),
                    target_fingerprint: "substring/2".to_string(),
                    receiver_role: "src".to_string(),
                    receiver_value_id: ValueId::new(10),
                    receiver_lane: APrimeI64LaneV1::OpaqueHandle,
                    arguments: vec![
                        APrimeI64CallArgumentReceiptV1 {
                            ordinal: 0,
                            role: "start".to_string(),
                            value_id: ValueId::new(12),
                            lane: APrimeI64LaneV1::ImmediateI64,
                        },
                        APrimeI64CallArgumentReceiptV1 {
                            ordinal: 1,
                            role: "end".to_string(),
                            value_id: ValueId::new(13),
                            lane: APrimeI64LaneV1::ImmediateI64,
                        },
                    ],
                    result_value_id: ValueId::new(20),
                    result_lane: APrimeI64LaneV1::OpaqueHandle,
                },
                APrimeI64CallEdgeReceiptV1 {
                    site_id: CheckedCallOutSiteIdV1::from_test(1),
                    role: "index_of".to_string(),
                    target_fingerprint: "indexOf/1".to_string(),
                    receiver_role: "pred_chars".to_string(),
                    receiver_value_id: ValueId::new(14),
                    receiver_lane: APrimeI64LaneV1::OpaqueHandle,
                    arguments: vec![APrimeI64CallArgumentReceiptV1 {
                        ordinal: 0,
                        role: "ch".to_string(),
                        value_id: ValueId::new(20),
                        lane: APrimeI64LaneV1::OpaqueHandle,
                    }],
                    result_value_id: ValueId::new(21),
                    result_lane: APrimeI64LaneV1::ImmediateI64,
                },
            ],
            vec![
                APrimeI64ReturnReceiptV1 {
                    site: "inner".to_string(),
                    block: BasicBlockId::new(2),
                    value_id: ValueId::new(30),
                    lane: APrimeI64LaneV1::ImmediateI64,
                },
                APrimeI64ReturnReceiptV1 {
                    site: "outer".to_string(),
                    block: BasicBlockId::new(3),
                    value_id: ValueId::new(31),
                    lane: APrimeI64LaneV1::ImmediateI64,
                },
            ],
        )
        .expect("valid receipt")
    }

    #[test]
    fn validates_complete_receipt() {
        let receipt = valid_receipt();
        assert_eq!(receipt.parameters[0].formal_parameter_index, 1);
        assert_eq!(receipt.parameters[1].formal_parameter_index, 2);
        assert!(receipt.validate().is_ok());
    }

    #[test]
    fn rejects_missing_and_duplicate_rows() {
        let mut receipt = valid_receipt();
        receipt.parameters.pop();
        assert_eq!(
            receipt.validate(),
            Err(APrimeI64PhysicalReceiptRejectV1::WrongParameterRows)
        );

        let mut receipt = valid_receipt();
        receipt.call_edges[1].site_id = receipt.call_edges[0].site_id;
        assert_eq!(
            receipt.validate(),
            Err(APrimeI64PhysicalReceiptRejectV1::DuplicateCallSite)
        );

        let mut receipt = valid_receipt();
        receipt.returns[1].site = receipt.returns[0].site.clone();
        assert_eq!(
            receipt.validate(),
            Err(APrimeI64PhysicalReceiptRejectV1::DuplicateReturnSite)
        );
    }

    #[test]
    fn rejects_wrong_lane() {
        let mut receipt = valid_receipt();
        receipt.parameters[0].lane = APrimeI64LaneV1::OpaqueHandle;
        assert_eq!(
            receipt.validate(),
            Err(APrimeI64PhysicalReceiptRejectV1::ParameterLaneMismatch)
        );

        let mut receipt = valid_receipt();
        receipt.call_edges[1].result_lane = APrimeI64LaneV1::OpaqueHandle;
        assert_eq!(
            receipt.validate(),
            Err(APrimeI64PhysicalReceiptRejectV1::CallResultLaneMismatch)
        );
    }

    #[test]
    fn rejects_swapped_parameter_role_indices() {
        let mut receipt = valid_receipt();
        receipt.parameters[0].formal_parameter_index = 0;
        assert_eq!(
            receipt.validate(),
            Err(APrimeI64PhysicalReceiptRejectV1::ParameterRoleIndexMismatch)
        );

        let mut receipt = valid_receipt();
        receipt.parameters[1].formal_parameter_index = 1;
        assert_eq!(
            receipt.validate(),
            Err(APrimeI64PhysicalReceiptRejectV1::ParameterRoleIndexMismatch)
        );
    }

    #[test]
    fn rejects_transport_shape_drift() {
        let mut receipt = valid_receipt();
        receipt.formal_parameter_count = 3;
        assert_eq!(
            receipt.validate(),
            Err(APrimeI64PhysicalReceiptRejectV1::WrongFormalParameterCount)
        );

        let mut receipt = valid_receipt();
        receipt.returns[1].site = "cleanup".to_string();
        assert_eq!(
            receipt.validate(),
            Err(APrimeI64PhysicalReceiptRejectV1::ReturnSiteMismatch)
        );

        let mut receipt = valid_receipt();
        receipt.call_edges[0].target_fingerprint = "indexOf/1".to_string();
        assert_eq!(
            receipt.validate(),
            Err(APrimeI64PhysicalReceiptRejectV1::CallTargetFingerprintMismatch)
        );

        let mut receipt = valid_receipt();
        receipt.call_edges[0].arguments[1].ordinal = 0;
        assert_eq!(
            receipt.validate(),
            Err(APrimeI64PhysicalReceiptRejectV1::CallArgumentMismatch)
        );

        let mut receipt = valid_receipt();
        receipt.call_edges[0].site_id = CheckedCallOutSiteIdV1::from_test(1);
        assert_eq!(
            receipt.validate(),
            Err(APrimeI64PhysicalReceiptRejectV1::CallSiteRoleMismatch)
        );
    }

    #[test]
    fn receipt_slot_is_linear_and_clone_scrubs_capability() {
        let mut slot = APrimeI64PhysicalReceiptSlotV1::default();
        assert!(slot.borrow().is_none());
        assert_eq!(slot.observe(), LinearSlotObservation::Empty);
        slot.install_for_test(valid_receipt())
            .expect("first receipt install");
        assert!(slot.borrow().is_some());
        assert!(matches!(slot.observe(), LinearSlotObservation::Occupied(_)));

        let mut cloned = slot.clone();
        assert!(cloned.borrow().is_none());
        assert_eq!(cloned.observe(), LinearSlotObservation::Scrubbed);
        assert_eq!(
            cloned.take_once(),
            Err(APrimeI64PhysicalReceiptSlotRejectV1::AlreadyConsumed)
        );

        let receipt = slot.take_once().expect("one-shot receipt take");
        assert!(receipt.validate().is_ok());
        assert!(slot.borrow().is_none());
        assert_eq!(
            slot.take_once(),
            Err(APrimeI64PhysicalReceiptSlotRejectV1::AlreadyConsumed)
        );
        assert_eq!(
            slot.install_for_test(valid_receipt()),
            Err(APrimeI64PhysicalReceiptSlotRejectV1::AlreadyConsumed)
        );
    }

    #[test]
    fn function_metadata_clone_does_not_duplicate_receipt() {
        let mut metadata = crate::mir::function::FunctionMetadata::default();
        metadata
            .install_a_prime_i64_physical_receipt_for_test(valid_receipt())
            .expect("receipt install");

        let cloned = metadata.clone();
        assert!(cloned.a_prime_i64_physical_receipt().is_none());
        assert!(metadata.a_prime_i64_physical_receipt().is_some());
    }
}
