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

/// MemOps opened for MIR-FMEM-005 primary LLVM/object producer.
///
/// These are value-only operations with complete lowering semantics. Layout
/// and runtime-owner operations stay closed until their dedicated rows.
pub const FASTMEM_LLVM_VALUE_MEMOP_KINDS: &[MemOpKind] = &[
    MemOpKind::AddrOf,
    MemOpKind::LogicalShr,
    MemOpKind::BitAnd,
    MemOpKind::Add,
    MemOpKind::Sub,
];

pub fn is_fastmem_v0_memop_kind(kind: MemOpKind) -> bool {
    FASTMEM_V0_MEMOP_KINDS.contains(&kind)
}

pub fn is_fastmem_llvm_value_memop_kind(kind: MemOpKind) -> bool {
    FASTMEM_LLVM_VALUE_MEMOP_KINDS.contains(&kind)
}

/// Backend support is opened by dedicated rows. MIR-FMEM-005 opens only the
/// value-only MemOp subset for MIR JSON transport and LLVM lowering.
pub fn is_supported_memop_kind(backend: FastMemBackend, kind: MemOpKind) -> bool {
    if !is_fastmem_v0_memop_kind(kind) {
        return false;
    }
    match backend {
        FastMemBackend::MirJson | FastMemBackend::LlvmJson | FastMemBackend::LlvmNative => {
            is_fastmem_llvm_value_memop_kind(kind)
        }
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
        assert_eq!(FASTMEM_V0_MEMOP_KINDS.len(), 10);
    }

    #[test]
    fn fastmem_v0_memop_kinds_are_named() {
        for kind in FASTMEM_V0_MEMOP_KINDS {
            assert!(!memop_kind_name(*kind).is_empty());
            assert!(!kind.as_json_name().is_empty());
        }
    }

    #[test]
    fn llvm_value_memops_are_the_only_open_backend_subset() {
        for kind in FASTMEM_LLVM_VALUE_MEMOP_KINDS {
            assert!(is_supported_memop_kind(FastMemBackend::MirJson, *kind));
            assert!(is_supported_memop_kind(FastMemBackend::LlvmJson, *kind));
            assert!(is_supported_memop_kind(FastMemBackend::LlvmNative, *kind));
            assert!(!is_supported_memop_kind(FastMemBackend::Vm, *kind));
            assert!(!is_supported_memop_kind(FastMemBackend::CArtifact, *kind));
        }

        for kind in [
            MemOpKind::TableIndex,
            MemOpKind::FieldLoad,
            MemOpKind::FieldStore,
            MemOpKind::CurrentAllocOwnerId,
            MemOpKind::OwnerEq,
        ] {
            assert!(!is_supported_memop_kind(FastMemBackend::MirJson, kind));
            assert!(!is_supported_memop_kind(FastMemBackend::LlvmJson, kind));
            assert!(!is_supported_memop_kind(FastMemBackend::LlvmNative, kind));
        }
    }
}
