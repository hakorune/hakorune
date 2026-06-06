use crate::mir::instruction::MemOpKind;

/// Backend/profile surfaces that may consume FastMemory MemOps.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FastMemBackend {
    MirJson,
    Vm,
    LlvmJson,
    LlvmNative,
    CArtifact,
}

/// V0 MemOpKind vocabulary. This is the dialect allowlist SSOT.
pub const FASTMEM_V0_MEMOP_KINDS: &[MemOpKind] = MemOpKind::ALL;

/// Value-only MemOps opened by MIR-FMEM-005.
pub const FASTMEM_LLVM_VALUE_MEMOP_KINDS: &[MemOpKind] = &[
    MemOpKind::AddrOf,
    MemOpKind::LogicalShr,
    MemOpKind::BitAnd,
    MemOpKind::Add,
    MemOpKind::Sub,
];

/// Layout/table MemOps opened by MIR-FMEM-008B/008C.
pub const FASTMEM_LLVM_LAYOUT_TABLE_MEMOP_KINDS: &[MemOpKind] = &[
    MemOpKind::TableIndex,
    MemOpKind::FieldLoad,
    MemOpKind::FieldStore,
];

/// Owner-runtime MemOps opened by MIR-FMEM-008D.
pub const FASTMEM_LLVM_OWNER_RUNTIME_MEMOP_KINDS: &[MemOpKind] =
    &[MemOpKind::CurrentAllocOwnerId, MemOpKind::OwnerEq];

/// Free-list MemOps selected by MIM-PORT-FMEM-009.
///
/// These are visible in MIR/JSON as vocabulary, but LLVM lowering remains
/// closed until verifier-owned local free-list plans land.
pub const FASTMEM_FREE_LIST_MEMOP_KINDS: &[MemOpKind] = &[
    MemOpKind::LocalFreePush,
    MemOpKind::LocalFreePop,
    MemOpKind::FreeHeadPop,
];

/// Complete MemOp set accepted by the current MIR-to-LLVM/object producer.
///
/// AtomicRemoteHead, TLS backing transfer, owner slot reuse, and allocator
/// activation are intentionally not represented by v0 MemOpKind entries.
pub const FASTMEM_LLVM_OPEN_MEMOP_KINDS: &[MemOpKind] = &[
    MemOpKind::AddrOf,
    MemOpKind::LogicalShr,
    MemOpKind::BitAnd,
    MemOpKind::Add,
    MemOpKind::Sub,
    MemOpKind::TableIndex,
    MemOpKind::FieldLoad,
    MemOpKind::FieldStore,
    MemOpKind::CurrentAllocOwnerId,
    MemOpKind::OwnerEq,
];

pub fn is_fastmem_v0_memop_kind(kind: MemOpKind) -> bool {
    FASTMEM_V0_MEMOP_KINDS.contains(&kind)
}

pub fn is_fastmem_llvm_value_memop_kind(kind: MemOpKind) -> bool {
    FASTMEM_LLVM_VALUE_MEMOP_KINDS.contains(&kind)
}

pub fn is_fastmem_llvm_layout_table_memop_kind(kind: MemOpKind) -> bool {
    FASTMEM_LLVM_LAYOUT_TABLE_MEMOP_KINDS.contains(&kind)
}

pub fn is_fastmem_llvm_owner_runtime_memop_kind(kind: MemOpKind) -> bool {
    FASTMEM_LLVM_OWNER_RUNTIME_MEMOP_KINDS.contains(&kind)
}

pub fn is_fastmem_free_list_memop_kind(kind: MemOpKind) -> bool {
    FASTMEM_FREE_LIST_MEMOP_KINDS.contains(&kind)
}

pub fn is_fastmem_llvm_open_memop_kind(kind: MemOpKind) -> bool {
    FASTMEM_LLVM_OPEN_MEMOP_KINDS.contains(&kind)
}

/// Backend support is opened by dedicated rows. MIR JSON/LLVM accept the v0
/// MemOp dialect as the MIR-to-LLVM producer surface. VM and C artifact
/// consumers stay closed until explicitly reopened.
pub fn is_supported_memop_kind(backend: FastMemBackend, kind: MemOpKind) -> bool {
    if !is_fastmem_v0_memop_kind(kind) {
        return false;
    }
    match backend {
        FastMemBackend::MirJson | FastMemBackend::LlvmJson => true,
        FastMemBackend::LlvmNative => is_fastmem_llvm_open_memop_kind(kind),
        FastMemBackend::Vm | FastMemBackend::CArtifact => false,
    }
}

pub fn memop_kind_name(kind: MemOpKind) -> &'static str {
    kind.display_name()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fastmem_v0_memop_kind_count_is_intentional() {
        assert_eq!(FASTMEM_V0_MEMOP_KINDS.len(), 13);
    }

    #[test]
    fn fastmem_v0_memop_kinds_are_named() {
        for kind in FASTMEM_V0_MEMOP_KINDS {
            assert!(!memop_kind_name(*kind).is_empty());
            assert!(!kind.as_json_name().is_empty());
        }
    }

    #[test]
    fn llvm_open_memops_are_json_and_llvm_supported() {
        assert_eq!(FASTMEM_LLVM_OPEN_MEMOP_KINDS.len(), 10);
        for kind in FASTMEM_LLVM_OPEN_MEMOP_KINDS {
            assert!(is_supported_memop_kind(FastMemBackend::MirJson, *kind));
            assert!(is_supported_memop_kind(FastMemBackend::LlvmJson, *kind));
            assert!(is_supported_memop_kind(FastMemBackend::LlvmNative, *kind));
            assert!(!is_supported_memop_kind(FastMemBackend::Vm, *kind));
            assert!(!is_supported_memop_kind(FastMemBackend::CArtifact, *kind));
        }
    }

    #[test]
    fn free_list_memops_are_transport_only_until_lowering_row() {
        for kind in FASTMEM_FREE_LIST_MEMOP_KINDS {
            assert!(is_fastmem_v0_memop_kind(*kind));
            assert!(is_fastmem_free_list_memop_kind(*kind));
            assert!(is_supported_memop_kind(FastMemBackend::MirJson, *kind));
            assert!(is_supported_memop_kind(FastMemBackend::LlvmJson, *kind));
            assert!(!is_supported_memop_kind(FastMemBackend::LlvmNative, *kind));
            assert!(!is_supported_memop_kind(FastMemBackend::Vm, *kind));
            assert!(!is_supported_memop_kind(FastMemBackend::CArtifact, *kind));
        }
    }

    #[test]
    fn llvm_open_memop_subsets_are_named() {
        assert!(is_fastmem_llvm_value_memop_kind(MemOpKind::AddrOf));
        assert!(is_fastmem_llvm_layout_table_memop_kind(
            MemOpKind::TableIndex
        ));
        assert!(is_fastmem_llvm_owner_runtime_memop_kind(
            MemOpKind::CurrentAllocOwnerId
        ));
    }
}
