//! Versioned physical transport for the selected object lifecycle consumer.
//!
//! V2 carries one backend-owned storage profile and typed row arrays borrowed
//! for one synchronous C call. It does not issue source meaning, object
//! identity, layout, or lifecycle permission; those remain owned by the final
//! published view. V1 stays layout-compatible for its existing cohorts.

use std::ffi::CString;
use std::os::raw::c_char;

use hakorune_mir_defs::SameModuleCallableNamespaceV1;

use crate::mir::function::{ObjectDestructionDispositionV1, TypedObjectFieldStorage};
use crate::mir::instruction::{FaultFrameMode, InvokeOperation};
use crate::mir::{Callee, MirInstruction};

use super::{
    lifecycle_schema::{
        CONTROL_KIND_RETURN, DEFINITION_ROLE_BIRTH_UNIT, DEFINITION_ROLE_ROOT_I64,
        DEFINITION_ROLE_ROOT_UNIT, PUBLISHED_LIFECYCLE_ABSENT_U32_V2, RESULT_KIND_I64,
        RESULT_KIND_UNIT,
    },
    CompiledEntryCleanupKindV1, CompiledEntryContractV1, CompiledEntryFormalKindV1,
    CompiledEntryRootResultV1, PublishedLifecyclePhysicalFunctionRoleV1, PublishedMirBackendView,
    PublishedStaticMethodCFrameV1, PublishedStaticMethodCallCRowV1,
};

#[path = "published_backend_view_lifecycle_c_transport/body_projection.rs"]
mod body_projection;

pub(crate) const PUBLISHED_LIFECYCLE_ABI_REVISION_V2: u32 = 2;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PublishedObjectStorageProfileV1 {
    SafeMutex = 1,
    SingleThreadExact = 2,
}

impl PublishedObjectStorageProfileV1 {
    pub(crate) fn from_runtime_name(value: Option<&str>) -> Result<Self, String> {
        match value {
            None | Some("") | Some("safe_mutex") => Ok(Self::SafeMutex),
            Some("single_thread_exact") => Ok(Self::SingleThreadExact),
            Some(other) => Err(format!(
                "[freeze:contract][published-lifecycle/storage-profile] unsupported profile: {other}"
            )),
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct PublishedLifecycleDefinitionCRowV2 {
    pub(crate) function_name: *const c_char,
    pub(crate) target_symbol: *const c_char,
    pub(crate) role: u32,
    pub(crate) source_arity: u32,
    pub(crate) receiver_formal: u32,
    pub(crate) object_id: u32,
    pub(crate) result_kind: u32,
    pub(crate) frame_mode: u32,
    pub(crate) flags: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct PublishedLifecycleFormalCRowV2 {
    pub(crate) definition_index: u32,
    pub(crate) source_ordinal: u32,
    pub(crate) physical_ordinal: u32,
    pub(crate) value_id: u32,
    pub(crate) wire_revision: u32,
    pub(crate) input_kind: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct PublishedLifecycleOperationCRowV2 {
    pub(crate) function_name: *const c_char,
    pub(crate) block_id: u32,
    pub(crate) instruction_index: u32,
    pub(crate) kind: u32,
    pub(crate) definition_index: u32,
    pub(crate) fault_frame: u32,
    pub(crate) normal_landing: u32,
    pub(crate) fault_landing: u32,
    pub(crate) object_id: u32,
    pub(crate) field_ordinal: u32,
    pub(crate) base: u32,
    pub(crate) value: u32,
    pub(crate) receiver: u32,
    pub(crate) operand_count: u32,
    pub(crate) flags: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct PublishedLifecycleOperandCRowV2 {
    pub(crate) operation_index: u32,
    pub(crate) ordinal: u32,
    pub(crate) value_id: u32,
    pub(crate) kind: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct PublishedLifecycleControlCRowV2 {
    pub(crate) function_name: *const c_char,
    pub(crate) block_id: u32,
    pub(crate) instruction_index: u32,
    pub(crate) kind: u32,
    pub(crate) operand: u32,
    pub(crate) origin_block: u32,
    pub(crate) mode: u32,
    pub(crate) flags: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct PublishedLifecycleLayoutCRowV2 {
    pub(crate) object_id: u32,
    pub(crate) runtime_type_id: u32,
    pub(crate) field_count: u32,
    pub(crate) destruction_kind: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct PublishedLifecycleFieldCRowV2 {
    pub(crate) object_id: u32,
    pub(crate) declaration_ordinal: u32,
    pub(crate) runtime_slot: u32,
    pub(crate) storage_kind: u32,
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct PublishedLifecycleBodySiteCRowV1 {
    pub(crate) function_name: *const c_char,
    pub(crate) block_id: u32,
    pub(crate) instruction_index: u32,
    pub(crate) normal_result: u32,
    pub(crate) fault_frame: u32,
    pub(crate) normal_landing: u32,
    pub(crate) fault_landing: u32,
    pub(crate) object_id: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct PublishedLifecycleCFrameHeaderV2 {
    pub(crate) abi_revision: u32,
    pub(crate) storage_profile: u32,
    pub(crate) call_rows: *const PublishedStaticMethodCallCRowV1,
    pub(crate) call_row_count: usize,
    pub(crate) definitions: *const PublishedLifecycleDefinitionCRowV2,
    pub(crate) definition_count: usize,
    pub(crate) formals: *const PublishedLifecycleFormalCRowV2,
    pub(crate) formal_count: usize,
    pub(crate) operations: *const PublishedLifecycleOperationCRowV2,
    pub(crate) operation_count: usize,
    pub(crate) operands: *const PublishedLifecycleOperandCRowV2,
    pub(crate) operand_count: usize,
    pub(crate) controls: *const PublishedLifecycleControlCRowV2,
    pub(crate) control_count: usize,
    pub(crate) layouts: *const PublishedLifecycleLayoutCRowV2,
    pub(crate) layout_count: usize,
    pub(crate) fields: *const PublishedLifecycleFieldCRowV2,
    pub(crate) field_count: usize,
}

/// Owns every pointer reachable from the V2 header for one synchronous call.
#[derive(Debug)]
pub(crate) struct PublishedLifecycleCFrameV2 {
    calls: PublishedStaticMethodCFrameV1,
    strings: Vec<CString>,
    definitions: Vec<PublishedLifecycleDefinitionCRowV2>,
    formals: Vec<PublishedLifecycleFormalCRowV2>,
    operations: Vec<PublishedLifecycleOperationCRowV2>,
    operands: Vec<PublishedLifecycleOperandCRowV2>,
    controls: Vec<PublishedLifecycleControlCRowV2>,
    layouts: Vec<PublishedLifecycleLayoutCRowV2>,
    fields: Vec<PublishedLifecycleFieldCRowV2>,
    body_sites: Vec<PublishedLifecycleBodySiteCRowV1>,
    header: PublishedLifecycleCFrameHeaderV2,
}

impl PublishedLifecycleCFrameV2 {
    /// S0 constructor. Empty lifecycle arrays never admit execution; the same
    /// series populates them from the published view before V2 is callable.
    pub(crate) fn from_call_frame(
        profile: PublishedObjectStorageProfileV1,
        calls: PublishedStaticMethodCFrameV1,
    ) -> Self {
        let header = PublishedLifecycleCFrameHeaderV2 {
            abi_revision: PUBLISHED_LIFECYCLE_ABI_REVISION_V2,
            storage_profile: profile as u32,
            call_rows: calls.as_ptr(),
            call_row_count: calls.len(),
            definitions: std::ptr::null(),
            definition_count: 0,
            formals: std::ptr::null(),
            formal_count: 0,
            operations: std::ptr::null(),
            operation_count: 0,
            operands: std::ptr::null(),
            operand_count: 0,
            controls: std::ptr::null(),
            control_count: 0,
            layouts: std::ptr::null(),
            layout_count: 0,
            fields: std::ptr::null(),
            field_count: 0,
        };
        Self {
            calls,
            strings: Vec::new(),
            definitions: Vec::new(),
            formals: Vec::new(),
            operations: Vec::new(),
            operands: Vec::new(),
            controls: Vec::new(),
            layouts: Vec::new(),
            fields: Vec::new(),
            body_sites: Vec::new(),
            header,
        }
    }

    /// Projects only the lifecycle coordinates retained by the final view.
    /// It never rescans the module or derives source identity from names.
    pub(crate) fn from_view(view: &PublishedMirBackendView<'_>) -> Result<Self, String> {
        let profile = view
            .lifecycle_storage_profile()
            .ok_or_else(|| fault("profile-not-issued"))?;
        let calls =
            PublishedStaticMethodCFrameV1::from_view(view).map_err(|error| error.to_string())?;
        let contract = view.issue_lifecycle_compiled_entry_contract()?;
        let mut frame = Self::from_call_frame(profile, calls);
        frame.populate(view.module(), &contract)?;
        Ok(frame)
    }

    pub(crate) const fn header(&self) -> &PublishedLifecycleCFrameHeaderV2 {
        &self.header
    }

    pub(crate) fn call_rows(&self) -> &[PublishedStaticMethodCallCRowV1] {
        self.calls.as_slice()
    }
    pub(crate) fn body_sites(&self) -> &[PublishedLifecycleBodySiteCRowV1] {
        &self.body_sites
    }

    pub(crate) fn definition_rows(&self) -> &[PublishedLifecycleDefinitionCRowV2] {
        &self.definitions
    }
}

fn fault(reason: &str) -> String {
    format!("[freeze:contract][published-lifecycle/{reason}]")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn storage_profile_is_finite_and_rejects_legacy_backends() {
        assert_eq!(
            PublishedObjectStorageProfileV1::from_runtime_name(None).unwrap(),
            PublishedObjectStorageProfileV1::SafeMutex
        );
        assert_eq!(
            PublishedObjectStorageProfileV1::from_runtime_name(Some("single_thread_exact"))
                .unwrap(),
            PublishedObjectStorageProfileV1::SingleThreadExact
        );
        for rejected in ["direct_slot_exact", "pinned_arena_exact", "unknown"] {
            assert!(PublishedObjectStorageProfileV1::from_runtime_name(Some(rejected)).is_err());
        }
    }

    #[test]
    fn v2_rows_have_the_fixed_lp64_c_layout() {
        use std::mem::{align_of, size_of};

        assert_eq!(size_of::<PublishedLifecycleDefinitionCRowV2>(), 48);
        assert_eq!(size_of::<PublishedLifecycleFormalCRowV2>(), 24);
        assert_eq!(size_of::<PublishedLifecycleOperationCRowV2>(), 64);
        assert_eq!(size_of::<PublishedLifecycleOperandCRowV2>(), 16);
        assert_eq!(size_of::<PublishedLifecycleControlCRowV2>(), 40);
        assert_eq!(size_of::<PublishedLifecycleLayoutCRowV2>(), 16);
        assert_eq!(size_of::<PublishedLifecycleFieldCRowV2>(), 16);
        assert_eq!(size_of::<PublishedLifecycleBodySiteCRowV1>(), 40);
        assert_eq!(size_of::<PublishedLifecycleCFrameHeaderV2>(), 136);
        assert_eq!(align_of::<PublishedLifecycleCFrameHeaderV2>(), 8);
    }
}
