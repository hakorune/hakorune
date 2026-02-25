use crate::mir::function::MirFunction;
use crate::mir::verification_types::VerificationError;

/// Reject legacy instructions that should be rewritten to Core-15 equivalents
/// Skips check when NYASH_VERIFY_ALLOW_LEGACY=1
pub fn check_no_legacy_ops(function: &MirFunction) -> Result<(), Vec<VerificationError>> {
    if std::env::var("NYASH_VERIFY_ALLOW_LEGACY").ok().as_deref() == Some("1") {
        return Ok(());
    }
    let mut errors = Vec::new();
    for (bid, block) in &function.blocks {
        for (idx, sp) in block.all_spanned_instructions_enumerated() {
            let legacy_name = crate::mir::contracts::backend_core_ops::lowered_away_tag(sp.inst);
            if let Some(name) = legacy_name {
                errors.push(VerificationError::UnsupportedLegacyInstruction {
                    block: *bid,
                    instruction_index: idx,
                    name: name.to_string(),
                });
            }
        }
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
