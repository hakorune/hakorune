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
pub const FASTMEM_V0_MEMOP_KINDS: &[MemOpKind] = &[
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

/// MIR-FMEM-002 only opens the vocabulary. Execution/transport support stays
/// closed until the dedicated JSON, verifier, and lowering rows.
pub fn is_supported_memop_kind(backend: FastMemBackend, kind: MemOpKind) -> bool {
    if !is_fastmem_v0_memop_kind(kind) {
        return false;
    }
    match backend {
        FastMemBackend::MirJson
        | FastMemBackend::Vm
        | FastMemBackend::LlvmJson
        | FastMemBackend::LlvmNative
        | FastMemBackend::CArtifact => false,
    }
}

pub fn memop_kind_name(kind: MemOpKind) -> &'static str {
    match kind {
        MemOpKind::AddrOf => "AddrOf",
        MemOpKind::LogicalShr => "LogicalShr",
        MemOpKind::BitAnd => "BitAnd",
        MemOpKind::Add => "Add",
        MemOpKind::Sub => "Sub",
        MemOpKind::TableIndex => "TableIndex",
        MemOpKind::FieldLoad => "FieldLoad",
        MemOpKind::FieldStore => "FieldStore",
        MemOpKind::CurrentAllocOwnerId => "CurrentAllocOwnerId",
        MemOpKind::OwnerEq => "OwnerEq",
    }
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
        }
    }

    #[test]
    fn fastmem_backends_are_closed_for_vocabulary_only_slice() {
        for backend in [
            FastMemBackend::MirJson,
            FastMemBackend::Vm,
            FastMemBackend::LlvmJson,
            FastMemBackend::LlvmNative,
            FastMemBackend::CArtifact,
        ] {
            for kind in FASTMEM_V0_MEMOP_KINDS {
                assert!(!is_supported_memop_kind(backend, *kind));
            }
        }
    }
}
