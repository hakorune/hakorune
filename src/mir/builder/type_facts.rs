//! Type inference facts for MIR operands
//!
//! Provides type classification and inference for binary operations.
//!
//! Phase 136 follow-up: Extracted from ops.rs to improve testability
//! and responsibility separation.

use crate::mir::{MirType, ValueId};
use std::collections::BTreeMap;

/// Operand type classification for binary operation type inference
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperandTypeClass {
    Integer,
    String,
    Unknown,
}

/// Type facts box for type inference
///
/// Provides type classification services for MIR operands during lowering.
/// Immutable by design - operates on borrowed type maps.
pub struct TypeFactsBox<'a> {
    value_types: &'a BTreeMap<ValueId, MirType>,
    value_origin_newbox: &'a BTreeMap<ValueId, String>,
}

impl<'a> TypeFactsBox<'a> {
    /// Create a new TypeFacts Box with borrowed type information
    pub fn new(
        value_types: &'a BTreeMap<ValueId, MirType>,
        value_origin_newbox: &'a BTreeMap<ValueId, String>,
    ) -> Self {
        Self {
            value_types,
            value_origin_newbox,
        }
    }

    /// Classify operand type for binary operation inference
    ///
    /// Phase 131-11-E: TypeFacts - operand type classification
    /// (Extracted from ops.rs:23-46)
    ///
    /// # Classification Strategy
    /// 1. Check explicit type annotation first (value_types)
    /// 2. Infer from NewBox origin (value_origin_newbox)
    /// 3. Default to Unknown
    ///
    /// # Returns
    /// - `OperandTypeClass::Integer` - Integer or Bool type
    /// - `OperandTypeClass::String` - String or StringBox type
    /// - `OperandTypeClass::Unknown` - Unable to infer
    pub fn classify_operand_type(&self, vid: ValueId) -> OperandTypeClass {
        let result = match self.value_types.get(&vid) {
            Some(MirType::String) => OperandTypeClass::String,
            Some(MirType::Box(bt)) if bt == "StringBox" => OperandTypeClass::String,
            Some(MirType::Integer) => OperandTypeClass::Integer,
            Some(MirType::Bool) => OperandTypeClass::Integer, // Bool can be used as integer
            _ => {
                // Check value_origin_newbox for StringBox
                if self
                    .value_origin_newbox
                    .get(&vid)
                    .map(|s| s == "StringBox")
                    .unwrap_or(false)
                {
                    return OperandTypeClass::String;
                }
                OperandTypeClass::Unknown
            }
        };

        if crate::config::env::builder_typefacts_debug() {
            crate::runtime::get_global_ring0()
                .log
                .debug(&format!("[typefacts] classify {:?} -> {:?}", vid, result));
        }

        result
    }
}
