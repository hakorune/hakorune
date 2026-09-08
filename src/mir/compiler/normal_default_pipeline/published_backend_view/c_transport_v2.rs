//! Private v2 wire vocabulary for the existing published frame owner.
//!
//! Rows project one body graph; they contain no copied operands or CFG edges.
//! This schema alone grants no admission or executable backend capability.
use std::os::raw::c_char;

use super::c_transport::PublishedStaticMethodCallCRowV1;

pub(super) const FRAME_REVISION: u32 = 2;
pub(super) const ORIGINAL_REQUIRED: u32 = 1;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum MapOperationKind {
    Allocate = 1,
    Write = 2,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub(super) struct MapOperationRow {
    pub function_name: *const c_char,
    pub block_id: u32,
    pub instruction_index: u32,
    pub kind: u32,
    pub reserved: u32,
}

// Runtime literal_store_v1 tags. Handle is never an ExactBits literal.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ValueKind {
    I64 = 1,
    Bool = 2,
    F64 = 3,
    Void = 4,
    Handle = 5,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
enum ActionKind {
    ExactBits = 1,
    OriginalValue = 2,
    Copy = 3,
    Phi = 4,
    Select = 5,
    Formal = 6,
    Operation = 7,
}

/// Selected physical consumer, not a source type or another operand graph.
/// The planner must prove its input domain; C checks the matched body opcode.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum PhysicalOperation {
    I64Binary = 1,
    I64Compare = 2,
    BoolCompare = 3,
    StringCompare = 4,
    StringConcat = 5,
    I64Not = 6,
    BoolNot = 7,
}

impl PhysicalOperation {
    fn result(self) -> (ValueKind, OriginalEncoding) {
        match self {
            Self::I64Binary => (ValueKind::I64, OriginalEncoding::I64Bits),
            Self::StringConcat => (ValueKind::Handle, OriginalEncoding::I64Bits),
            Self::I64Compare
            | Self::BoolCompare
            | Self::StringCompare
            | Self::I64Not
            | Self::BoolNot => (ValueKind::Bool, OriginalEncoding::BoolI1ZeroExtend),
        }
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
enum OriginalEncoding {
    I64Bits = 1,
    BoolI1ZeroExtend = 2,
}

/// Physically proved actions, issued only by the frame's pre-JSON planner.
/// There is deliberately no raw-kind constructor or exact Handle literal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ProjectionAction {
    ExactI64(i64),
    ExactBool(bool),
    ExactF64(u64),
    ExactVoid,
    OriginalI64,
    OriginalBoolI64,
    OriginalBoolI1,
    OriginalHandle,
    Copy,
    Phi,
    Select,
    Formal(u32),
    Operation(PhysicalOperation),
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub(super) struct ValueProjectionRow {
    pub function_name: *const c_char,
    pub value_id: u32,
    pub action: u32,
    pub value_kind: u32,
    pub encoding: u32,
    pub flags: u32,
    pub source_ordinal: u32,
    pub operation: u32,
    pub payload: u64,
}

impl ProjectionAction {
    pub(super) fn requires_original_producer(self) -> bool {
        match self {
            Self::OriginalI64
            | Self::OriginalBoolI64
            | Self::OriginalBoolI1
            | Self::OriginalHandle
            | Self::Operation(_) => true,
            Self::ExactI64(_)
            | Self::ExactBool(_)
            | Self::ExactF64(_)
            | Self::ExactVoid
            | Self::Copy
            | Self::Phi
            | Self::Select
            | Self::Formal(_) => false,
        }
    }

    pub(super) fn row(
        self,
        function_name: *const c_char,
        value_id: u32,
        original_required: bool,
    ) -> ValueProjectionRow {
        let mut row = ValueProjectionRow {
            function_name,
            value_id,
            action: 0,
            value_kind: 0,
            encoding: 0,
            flags: if original_required || self.requires_original_producer() {
                ORIGINAL_REQUIRED
            } else {
                0
            },
            source_ordinal: 0,
            operation: 0,
            payload: 0,
        };
        if let Self::Operation(operation) = self {
            let (kind, encoding) = operation.result();
            row.action = ActionKind::Operation as u32;
            row.value_kind = kind as u32;
            row.encoding = encoding as u32;
            row.operation = operation as u32;
            return row;
        }
        let exact = match self {
            Self::ExactI64(value) => Some((ValueKind::I64, value as u64)),
            Self::ExactBool(value) => Some((ValueKind::Bool, u64::from(value))),
            Self::ExactF64(bits) => Some((ValueKind::F64, bits)),
            Self::ExactVoid => Some((ValueKind::Void, 0)),
            _ => None,
        };
        if let Some((kind, payload)) = exact {
            row.action = ActionKind::ExactBits as u32;
            row.value_kind = kind as u32;
            row.payload = payload;
            return row;
        }
        let original = match self {
            Self::OriginalI64 => Some((ValueKind::I64, OriginalEncoding::I64Bits)),
            Self::OriginalBoolI64 => Some((ValueKind::Bool, OriginalEncoding::I64Bits)),
            Self::OriginalBoolI1 => Some((ValueKind::Bool, OriginalEncoding::BoolI1ZeroExtend)),
            Self::OriginalHandle => Some((ValueKind::Handle, OriginalEncoding::I64Bits)),
            _ => None,
        };
        if let Some((kind, encoding)) = original {
            row.action = ActionKind::OriginalValue as u32;
            row.value_kind = kind as u32;
            row.encoding = encoding as u32;
            return row;
        }
        row.action = match self {
            Self::Copy => ActionKind::Copy,
            Self::Phi => ActionKind::Phi,
            Self::Select => ActionKind::Select,
            Self::Formal(ordinal) => {
                row.source_ordinal = ordinal;
                ActionKind::Formal
            }
            _ => unreachable!("exact and original actions returned above"),
        } as u32;
        row
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub(super) struct ExpandedFunctionRow {
    pub function_name: *const c_char,
    pub internal_target: *const c_char,
}

/// Borrowed header; backing strings and arrays belong to the synchronous frame.
/// Counts cover complete arrays, including rows for nested function emission.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub(super) struct FrameHeader {
    pub revision: u32,
    pub byte_size: u32,
    pub calls: *const PublishedStaticMethodCallCRowV1,
    pub call_count: u64,
    pub map_operations: *const MapOperationRow,
    pub map_operation_count: u64,
    pub values: *const ValueProjectionRow,
    pub value_count: u64,
    pub expanded_functions: *const ExpandedFunctionRow,
    pub expanded_function_count: u64,
}

#[cfg(test)]
#[path = "c_transport_v2_tests.rs"]
mod tests;
