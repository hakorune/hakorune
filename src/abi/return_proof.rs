/// Return value classes for future pointer/handle ownership proof rows.
///
/// This vocabulary is deliberately separate from LLVM attrs. Handle classes are
/// runtime value classes; only native pointer classes may later feed pointer
/// attrs after verifier/export gates.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReturnClass {
    ImmI64,
    HandleExistingBorrowed,
    HandleExistingOwnedRef,
    HandleFreshOwned,
    NativePtrNonnull,
    NativePtrNullable,
    NativePtrDereferenceable { len: String, align: u32 },
}

impl ReturnClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ImmI64 => "imm_i64",
            Self::HandleExistingBorrowed => "handle_existing_borrowed",
            Self::HandleExistingOwnedRef => "handle_existing_owned_ref",
            Self::HandleFreshOwned => "handle_fresh_owned",
            Self::NativePtrNonnull => "native_ptr_nonnull",
            Self::NativePtrNullable => "native_ptr_nullable",
            Self::NativePtrDereferenceable { .. } => "native_ptr_dereferenceable",
        }
    }

    pub fn is_handle_class(&self) -> bool {
        matches!(
            self,
            Self::HandleExistingBorrowed | Self::HandleExistingOwnedRef | Self::HandleFreshOwned
        )
    }

    pub fn is_native_pointer_class(&self) -> bool {
        matches!(
            self,
            Self::NativePtrNonnull
                | Self::NativePtrNullable
                | Self::NativePtrDereferenceable { .. }
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReturnProof {
    Fresh,
    NonNull,
    DereferenceableBytes,
    Alignment,
    NoAliasScope,
    NoRefcountMutation,
    NoRegistryWrite,
}

impl ReturnProof {
    pub const ALL: &'static [ReturnProof] = &[
        ReturnProof::Fresh,
        ReturnProof::NonNull,
        ReturnProof::DereferenceableBytes,
        ReturnProof::Alignment,
        ReturnProof::NoAliasScope,
        ReturnProof::NoRefcountMutation,
        ReturnProof::NoRegistryWrite,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Fresh => "fresh",
            Self::NonNull => "nonnull",
            Self::DereferenceableBytes => "dereferenceable_bytes",
            Self::Alignment => "alignment",
            Self::NoAliasScope => "noalias_scope",
            Self::NoRefcountMutation => "no_refcount_mutation",
            Self::NoRegistryWrite => "no_registry_write",
        }
    }

    pub fn can_feed_llvm_pointer_attr(self) -> bool {
        matches!(
            self,
            Self::Fresh
                | Self::NonNull
                | Self::DereferenceableBytes
                | Self::Alignment
                | Self::NoAliasScope
        )
    }
}

pub fn return_class_names() -> &'static [&'static str] {
    &[
        "imm_i64",
        "handle_existing_borrowed",
        "handle_existing_owned_ref",
        "handle_fresh_owned",
        "native_ptr_nonnull",
        "native_ptr_nullable",
        "native_ptr_dereferenceable",
    ]
}

pub fn return_proof_names() -> Vec<&'static str> {
    ReturnProof::ALL
        .iter()
        .copied()
        .map(ReturnProof::as_str)
        .collect()
}

pub fn may_export_llvm_pointer_attr(class: &ReturnClass, proof: ReturnProof) -> bool {
    if !proof.can_feed_llvm_pointer_attr() {
        return false;
    }
    match class {
        ReturnClass::NativePtrNonnull => matches!(proof, ReturnProof::NonNull),
        ReturnClass::NativePtrDereferenceable { .. } => matches!(
            proof,
            ReturnProof::NonNull | ReturnProof::DereferenceableBytes | ReturnProof::Alignment
        ),
        ReturnClass::NativePtrNullable => false,
        ReturnClass::ImmI64
        | ReturnClass::HandleExistingBorrowed
        | ReturnClass::HandleExistingOwnedRef
        | ReturnClass::HandleFreshOwned => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_proof_vocabulary_contains_m10c_pre_classes() {
        assert_eq!(
            return_class_names(),
            &[
                "imm_i64",
                "handle_existing_borrowed",
                "handle_existing_owned_ref",
                "handle_fresh_owned",
                "native_ptr_nonnull",
                "native_ptr_nullable",
                "native_ptr_dereferenceable",
            ]
        );
        assert_eq!(
            return_proof_names(),
            vec![
                "fresh",
                "nonnull",
                "dereferenceable_bytes",
                "alignment",
                "noalias_scope",
                "no_refcount_mutation",
                "no_registry_write",
            ]
        );
    }

    #[test]
    fn handle_return_classes_do_not_export_llvm_pointer_attrs() {
        let handle_classes = [
            ReturnClass::HandleExistingBorrowed,
            ReturnClass::HandleExistingOwnedRef,
            ReturnClass::HandleFreshOwned,
        ];
        for class in handle_classes {
            assert!(class.is_handle_class());
            for proof in ReturnProof::ALL {
                assert!(
                    !may_export_llvm_pointer_attr(&class, *proof),
                    "handle class {} must not feed LLVM pointer attrs for proof {}",
                    class.as_str(),
                    proof.as_str()
                );
            }
        }
    }

    #[test]
    fn native_pointer_classes_gate_pointer_attr_proofs_narrowly() {
        assert!(may_export_llvm_pointer_attr(
            &ReturnClass::NativePtrNonnull,
            ReturnProof::NonNull
        ));
        assert!(may_export_llvm_pointer_attr(
            &ReturnClass::NativePtrDereferenceable {
                len: "len".to_string(),
                align: 8,
            },
            ReturnProof::DereferenceableBytes
        ));
        assert!(!may_export_llvm_pointer_attr(
            &ReturnClass::NativePtrNullable,
            ReturnProof::NonNull
        ));
        assert!(!may_export_llvm_pointer_attr(
            &ReturnClass::NativePtrNonnull,
            ReturnProof::NoRefcountMutation
        ));
    }
}
