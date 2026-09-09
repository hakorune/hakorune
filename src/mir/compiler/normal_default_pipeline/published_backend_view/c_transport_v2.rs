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
    NamedAliasOperandZero = 8,
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
    NamedAliasOperandZero,
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
            | Self::Formal(_)
            | Self::NamedAliasOperandZero => false,
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
            Self::NamedAliasOperandZero => ActionKind::NamedAliasOperandZero,
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
pub(crate) struct FrameHeader {
    pub(super) revision: u32,
    pub(super) byte_size: u32,
    pub(super) calls: *const PublishedStaticMethodCallCRowV1,
    pub(super) call_count: u64,
    pub(super) map_operations: *const MapOperationRow,
    pub(super) map_operation_count: u64,
    pub(super) values: *const ValueProjectionRow,
    pub(super) value_count: u64,
    pub(super) expanded_functions: *const ExpandedFunctionRow,
    pub(super) expanded_function_count: u64,
}

#[cfg(test)]
#[path = "c_transport_v2_tests.rs"]
mod tests;

/// Owns the complete synchronous wire projection. The existing call frame is
/// reused as backing, not reissued as another call graph. This is a candidate
/// until the C consumer validates coverage, ingress and actual materialization.
#[derive(Debug)]
pub(crate) struct PublishedStaticMethodCFrameV2 {
    calls: super::c_transport::PublishedStaticMethodCFrameV1,
    strings: Vec<std::ffi::CString>,
    map_operations: Vec<MapOperationRow>,
    values: Vec<ValueProjectionRow>,
    expanded_functions: Vec<ExpandedFunctionRow>,
}

impl PublishedStaticMethodCFrameV2 {
    pub(super) fn from_view<'m>(
        view: &super::PublishedMirBackendView<'m>,
        observations: impl IntoIterator<
            Item = (
                super::map_body_index::Site<'m>,
                super::map_named_allocations::NamedAllocationConsumer,
            ),
        >,
    ) -> Result<Self, String> {
        use super::map_body_index::MapBodyIndex;
        let index = MapBodyIndex::from_view(view)?.with_named_allocations(observations)?;
        Self::from_index(view, index)
    }

    pub(crate) fn from_view_with_query<'m>(
        view: &super::PublishedMirBackendView<'m>,
        mut query: impl FnMut(&str, u32, u32) -> Result<Option<super::map_named_allocations::NamedAllocationConsumer>, String>,
    ) -> Result<Self, String> {
        let index = super::map_body_index::MapBodyIndex::from_view(view)?;
        let mut observations = Vec::new();
        for (&site, instruction) in &index.instructions {
            if matches!(instruction, crate::mir::MirInstruction::NewBox {
                target: crate::mir::ConstructionTarget::Named(_), ..
            }) {
                if let Some(consumer) = query(site.0, site.1, site.2)? {
                    observations.push((site, consumer));
                }
            }
        }
        Self::from_index(view, index.with_named_allocations(observations)?)
    }

    fn from_index<'m>(
        view: &super::PublishedMirBackendView<'m>,
        index: super::map_body_index::MapBodyIndex<'m>,
    ) -> Result<Self, String> {
        use std::collections::{BTreeMap, BTreeSet};
        use std::ffi::CString;
        let (actions, original) = index.map_frame_projection()?;
        let expanded: BTreeSet<_> = actions
            .iter()
            .filter_map(|(key, action)| {
                matches!(action, ProjectionAction::Formal(_)).then_some(key.0)
            })
            .collect();
        let mut frame = Self {
            calls: super::c_transport::PublishedStaticMethodCFrameV1::from_view(view)
                .map_err(|error| error.to_string())?,
            strings: Vec::new(),
            map_operations: Vec::with_capacity(index.map_operations.len()),
            values: Vec::with_capacity(actions.len()),
            expanded_functions: Vec::with_capacity(expanded.len()),
        };
        let names: BTreeSet<_> = actions
            .keys()
            .map(|key| key.0)
            .chain(index.map_operations.keys().map(|site| site.0))
            .collect();
        let mut name_ptrs = BTreeMap::new();
        for name in names {
            let owned = CString::new(name)
                .map_err(|_| "[freeze:contract][map-frame/function-name-nul]".to_string())?;
            name_ptrs.insert(name, owned.as_ptr());
            frame.strings.push(owned);
        }
        for (site, kind) in &index.map_operations {
            frame.map_operations.push(MapOperationRow {
                function_name: name_ptrs[site.0],
                block_id: site.1,
                instruction_index: site.2,
                kind: *kind as u32,
                reserved: 0,
            });
        }
        for (key, action) in &actions {
            frame
                .values
                .push(action.row(name_ptrs[key.0], key.1.as_u32(), original.contains(key)));
        }
        // Deterministic physical names; never infer a semantic target from them.
        // Skip collisions with every original definition, including unused ones.
        let mut serial = 0u64;
        for function in expanded {
            let target = loop {
                let candidate = format!("__hako_map_expanded_v2_{serial}");
                serial = serial
                    .checked_add(1)
                    .ok_or("[freeze:contract][map-frame/internal-symbol-overflow]")?;
                if !index.functions.contains_key(candidate.as_str()) {
                    break candidate;
                }
            };
            let target = CString::new(target).expect("generated identifier has no NUL");
            frame.expanded_functions.push(ExpandedFunctionRow {
                function_name: name_ptrs[function],
                internal_target: target.as_ptr(),
            });
            frame.strings.push(target);
        }
        Ok(frame)
    }

    /// Header borrows all pointers from this frame for one synchronous call.
    /// Moving the frame preserves CString/Vec allocations; mutation is private.
    pub(crate) fn header(&self) -> FrameHeader {
        fn pointer<T>(rows: &[T]) -> *const T {
            if rows.is_empty() {
                std::ptr::null()
            } else {
                rows.as_ptr()
            }
        }
        let calls = self.calls.as_slice();
        FrameHeader {
            revision: FRAME_REVISION,
            byte_size: std::mem::size_of::<FrameHeader>() as u32,
            calls: pointer(calls),
            call_count: calls.len() as u64,
            map_operations: pointer(&self.map_operations),
            map_operation_count: self.map_operations.len() as u64,
            values: pointer(&self.values),
            value_count: self.values.len() as u64,
            expanded_functions: pointer(&self.expanded_functions),
            expanded_function_count: self.expanded_functions.len() as u64,
        }
    }
}
