//! Sealed post-session capability for the bounded A-prime LLVM lane.
//!
//! This is deliberately a transport receipt, not a source or Recipe owner.
//! Its private issuer is reserved for the canonical physical session.  Until
//! that session is connected, normal functions leave the metadata field
//! empty and the LLVM loader keeps the legacy path unchanged.

use crate::mir::{BasicBlockId, ValueId};
use std::collections::BTreeSet;

pub(crate) const A_PRIME_I64_PHYSICAL_RECEIPT_SCHEMA_VERSION: u32 = 1;

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
    pub(crate) value_id: ValueId,
    pub(crate) lane: APrimeI64LaneV1,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct APrimeI64CallEdgeReceiptV1 {
    pub(crate) role: String,
    pub(crate) block: BasicBlockId,
    pub(crate) instruction_index: usize,
    pub(crate) target_fingerprint: String,
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
    DuplicateParameterRole,
    DuplicateParameterIndex,
    DuplicateParameterValue,
    ParameterRoleMismatch,
    ParameterLaneMismatch,
    MissingCallEdgeRows,
    WrongCallEdgeRows,
    DuplicateCallSite,
    CallRoleMismatch,
    EmptyCallTarget,
    EmptyCallArguments,
    CallResultLaneMismatch,
    MissingReturnRows,
    WrongReturnRows,
    EmptyReturnSite,
    DuplicateReturnSite,
    ReturnLaneMismatch,
}

/// A complete, post-session capability.  No public constructor is provided;
/// the canonical physical session will be the sole issuer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct APrimeI64PhysicalReceiptV1 {
    schema_version: u32,
    backend_family: APrimeI64BackendFamilyV1,
    parameters: Vec<APrimeI64ParameterReceiptV1>,
    call_edges: Vec<APrimeI64CallEdgeReceiptV1>,
    returns: Vec<APrimeI64ReturnReceiptV1>,
}

impl APrimeI64PhysicalReceiptV1 {
    /// Private issuer boundary for the canonical physical session.
    pub(crate) fn seal(
        backend_family: APrimeI64BackendFamilyV1,
        parameters: Vec<APrimeI64ParameterReceiptV1>,
        call_edges: Vec<APrimeI64CallEdgeReceiptV1>,
        returns: Vec<APrimeI64ReturnReceiptV1>,
    ) -> Result<Self, APrimeI64PhysicalReceiptRejectV1> {
        let receipt = Self {
            schema_version: A_PRIME_I64_PHYSICAL_RECEIPT_SCHEMA_VERSION,
            backend_family,
            parameters,
            call_edges,
            returns,
        };
        receipt.validate()?;
        Ok(receipt)
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
        for row in &self.call_edges {
            if !matches!(row.role.as_str(), "substring" | "index_of") {
                return Err(APrimeI64PhysicalReceiptRejectV1::CallRoleMismatch);
            }
            if !call_roles.insert(row.role.as_str()) {
                return Err(APrimeI64PhysicalReceiptRejectV1::CallRoleMismatch);
            }
            if !call_sites.insert((row.block.as_u32(), row.instruction_index)) {
                return Err(APrimeI64PhysicalReceiptRejectV1::DuplicateCallSite);
            }
            if row.target_fingerprint.is_empty() {
                return Err(APrimeI64PhysicalReceiptRejectV1::EmptyCallTarget);
            }
            if row.arguments.is_empty() {
                return Err(APrimeI64PhysicalReceiptRejectV1::EmptyCallArguments);
            }
            if row.result_lane != APrimeI64LaneV1::OpaqueHandle {
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
        Ok(())
    }

    pub(crate) fn schema_version(&self) -> u32 {
        self.schema_version
    }

    pub(crate) fn backend_family(&self) -> APrimeI64BackendFamilyV1 {
        self.backend_family
    }

    pub(crate) fn parameters(&self) -> &[APrimeI64ParameterReceiptV1] {
        &self.parameters
    }

    pub(crate) fn call_edges(&self) -> &[APrimeI64CallEdgeReceiptV1] {
        &self.call_edges
    }

    pub(crate) fn returns(&self) -> &[APrimeI64ReturnReceiptV1] {
        &self.returns
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_receipt() -> APrimeI64PhysicalReceiptV1 {
        APrimeI64PhysicalReceiptV1::seal(
            APrimeI64BackendFamilyV1::Llvm,
            vec![
                APrimeI64ParameterReceiptV1 {
                    role: "pos".to_string(),
                    formal_parameter_index: 0,
                    value_id: ValueId::new(10),
                    lane: APrimeI64LaneV1::ImmediateI64,
                },
                APrimeI64ParameterReceiptV1 {
                    role: "end".to_string(),
                    formal_parameter_index: 1,
                    value_id: ValueId::new(11),
                    lane: APrimeI64LaneV1::ImmediateI64,
                },
            ],
            vec![
                APrimeI64CallEdgeReceiptV1 {
                    role: "substring".to_string(),
                    block: BasicBlockId::new(1),
                    instruction_index: 3,
                    target_fingerprint: "substring/3".to_string(),
                    arguments: vec![APrimeI64CallArgumentReceiptV1 {
                        value_id: ValueId::new(12),
                        lane: APrimeI64LaneV1::OpaqueHandle,
                    }],
                    result_value_id: ValueId::new(20),
                    result_lane: APrimeI64LaneV1::OpaqueHandle,
                },
                APrimeI64CallEdgeReceiptV1 {
                    role: "index_of".to_string(),
                    block: BasicBlockId::new(1),
                    instruction_index: 4,
                    target_fingerprint: "indexOf/2".to_string(),
                    arguments: vec![APrimeI64CallArgumentReceiptV1 {
                        value_id: ValueId::new(20),
                        lane: APrimeI64LaneV1::OpaqueHandle,
                    }],
                    result_value_id: ValueId::new(21),
                    result_lane: APrimeI64LaneV1::OpaqueHandle,
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
        assert!(valid_receipt().validate().is_ok());
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
        receipt.call_edges[1].instruction_index = receipt.call_edges[0].instruction_index;
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
    }
}
