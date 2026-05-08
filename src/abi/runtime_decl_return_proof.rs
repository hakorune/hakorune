use super::return_proof::{may_export_llvm_pointer_attr, ReturnClass, ReturnProof};

/// Backend-private return proof row for runtime declarations.
///
/// This is a proof schema, not an LLVM attr emitter. Strong attrs remain
/// disabled until a later verifier/export gate consumes eligible native pointer
/// rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReturnProofExportMode {
    Disabled,
    VerifierRequired,
    Exported,
}

impl ReturnProofExportMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Disabled => "disabled",
            Self::VerifierRequired => "verifier_required",
            Self::Exported => "exported",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeDeclReturnProofRow {
    pub symbol: String,
    pub return_class: ReturnClass,
    pub proofs: Vec<ReturnProof>,
    pub export_mode: ReturnProofExportMode,
}

impl RuntimeDeclReturnProofRow {
    pub fn new(
        symbol: impl Into<String>,
        return_class: ReturnClass,
        proofs: Vec<ReturnProof>,
        export_mode: ReturnProofExportMode,
    ) -> Self {
        Self {
            symbol: symbol.into(),
            return_class,
            proofs,
            export_mode,
        }
    }
}

pub fn validate_runtime_decl_return_proof_row(
    row: &RuntimeDeclReturnProofRow,
) -> Result<(), String> {
    if row.symbol.trim().is_empty() {
        return Err("[runtime-decl/return-proof] missing symbol".to_string());
    }

    if matches!(row.export_mode, ReturnProofExportMode::Exported) {
        return Err(format!(
            "[runtime-decl/return-proof] {}: strong attr export is still blocked",
            row.symbol
        ));
    }

    if matches!(row.export_mode, ReturnProofExportMode::VerifierRequired) {
        if !row.return_class.is_native_pointer_class() {
            return Err(format!(
                "[runtime-decl/return-proof] {}: verifier-required pointer attrs require native pointer return class",
                row.symbol
            ));
        }
        for proof in &row.proofs {
            if !may_export_llvm_pointer_attr(&row.return_class, *proof) {
                return Err(format!(
                    "[runtime-decl/return-proof] {}: proof {} cannot feed pointer attrs for {}",
                    row.symbol,
                    proof.as_str(),
                    row.return_class.as_str()
                ));
            }
        }
    }

    if let ReturnClass::NativePtrDereferenceable { len, align } = &row.return_class {
        if len.trim().is_empty() || *align == 0 {
            return Err(format!(
                "[runtime-decl/return-proof] {}: dereferenceable native pointer requires len and non-zero align",
                row.symbol
            ));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn disabled_handle_return_proof_row_is_schema_only() {
        let row = RuntimeDeclReturnProofRow::new(
            "fixture.handle.borrowed",
            ReturnClass::HandleExistingBorrowed,
            vec![
                ReturnProof::NoRefcountMutation,
                ReturnProof::NoRegistryWrite,
            ],
            ReturnProofExportMode::Disabled,
        );
        validate_runtime_decl_return_proof_row(&row).unwrap();
        for proof in &row.proofs {
            assert!(!may_export_llvm_pointer_attr(&row.return_class, *proof));
        }
    }

    #[test]
    fn verifier_required_rejects_handle_return_classes() {
        let row = RuntimeDeclReturnProofRow::new(
            "fixture.handle.bad",
            ReturnClass::HandleFreshOwned,
            vec![ReturnProof::Fresh],
            ReturnProofExportMode::VerifierRequired,
        );
        let err = validate_runtime_decl_return_proof_row(&row).unwrap_err();
        assert!(err.contains("require native pointer return class"));
    }

    #[test]
    fn verifier_required_native_nonnull_accepts_only_nonnull_proof() {
        let good = RuntimeDeclReturnProofRow::new(
            "fixture.native.nonnull",
            ReturnClass::NativePtrNonnull,
            vec![ReturnProof::NonNull],
            ReturnProofExportMode::VerifierRequired,
        );
        validate_runtime_decl_return_proof_row(&good).unwrap();

        let bad = RuntimeDeclReturnProofRow::new(
            "fixture.native.bad",
            ReturnClass::NativePtrNonnull,
            vec![ReturnProof::NoRefcountMutation],
            ReturnProofExportMode::VerifierRequired,
        );
        let err = validate_runtime_decl_return_proof_row(&bad).unwrap_err();
        assert!(err.contains("cannot feed pointer attrs"));
    }

    #[test]
    fn exported_mode_remains_blocked_before_m10c() {
        let row = RuntimeDeclReturnProofRow::new(
            "fixture.native.exported",
            ReturnClass::NativePtrNonnull,
            vec![ReturnProof::NonNull],
            ReturnProofExportMode::Exported,
        );
        let err = validate_runtime_decl_return_proof_row(&row).unwrap_err();
        assert!(err.contains("strong attr export is still blocked"));
    }
}
