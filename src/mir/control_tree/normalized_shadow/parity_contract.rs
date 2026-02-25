//! Parity verification between shadow and existing router (contracts only)
//!
//! ## Responsibility
//!
//! - Compare exit/writes contracts extracted from StepTree
//! - Do not inspect generated JoinIR; purely contract parity

use crate::mir::control_tree::step_tree_contract_box::StepTreeContract;

/// Mismatch classification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MismatchKind {
    /// Exit contract mismatch
    ExitMismatch,
    /// Writes contract mismatch
    WritesMismatch,
    /// Unsupported kind (should not happen for if-only)
    UnsupportedKind,
    /// Phase 129-C: Structure mismatch (e.g., post_k form vs if-as-last)
    StructureMismatch,
}

impl MismatchKind {
    /// Get human-readable description
    pub fn description(&self) -> &'static str {
        match self {
            MismatchKind::ExitMismatch => "exit contract mismatch",
            MismatchKind::WritesMismatch => "writes contract mismatch",
            MismatchKind::UnsupportedKind => "unsupported pattern for parity check",
            MismatchKind::StructureMismatch => "structure mismatch (post_k vs if-as-last)",
        }
    }
}

/// Result of parity check
#[derive(Debug, Clone)]
pub struct ShadowParityResult {
    /// Whether parity check passed
    pub ok: bool,
    /// Mismatch kind if not ok
    pub mismatch_kind: Option<MismatchKind>,
    /// Hint for debugging (must be non-empty if not ok)
    pub hint: Option<String>,
}

impl ShadowParityResult {
    /// Create successful parity result
    pub fn ok() -> Self {
        Self {
            ok: true,
            mismatch_kind: None,
            hint: None,
        }
    }

    /// Create failed parity result with hint
    pub fn mismatch(kind: MismatchKind, hint: String) -> Self {
        assert!(!hint.is_empty(), "hint must not be empty for mismatch");
        Self {
            ok: false,
            mismatch_kind: Some(kind),
            hint: Some(hint),
        }
    }
}

/// Compare exit contracts between shadow and existing path
pub fn compare_exit_contracts(
    shadow: &StepTreeContract,
    existing: &StepTreeContract,
) -> ShadowParityResult {
    if shadow.exits != existing.exits {
        let hint = format!(
            "exit mismatch: shadow={:?}, existing={:?}",
            shadow.exits, existing.exits
        );
        return ShadowParityResult::mismatch(MismatchKind::ExitMismatch, hint);
    }
    ShadowParityResult::ok()
}

/// Compare writes contracts between shadow and existing path
pub fn compare_writes_contracts(
    shadow: &StepTreeContract,
    existing: &StepTreeContract,
) -> ShadowParityResult {
    if shadow.writes != existing.writes {
        let hint = format!(
            "writes mismatch: shadow={:?}, existing={:?}",
            shadow.writes, existing.writes
        );
        return ShadowParityResult::mismatch(MismatchKind::WritesMismatch, hint);
    }
    ShadowParityResult::ok()
}

/// Full parity check (exits + writes)
pub fn check_full_parity(
    shadow: &StepTreeContract,
    existing: &StepTreeContract,
) -> ShadowParityResult {
    let exit_result = compare_exit_contracts(shadow, existing);
    if !exit_result.ok {
        return exit_result;
    }

    let writes_result = compare_writes_contracts(shadow, existing);
    if !writes_result.ok {
        return writes_result;
    }

    ShadowParityResult::ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::control_tree::step_tree::ExitKind;

    fn make_contract(exits: Vec<ExitKind>, writes: Vec<&str>) -> StepTreeContract {
        StepTreeContract {
            exits: exits.into_iter().collect(),
            writes: writes.into_iter().map(String::from).collect(),
            reads: Default::default(), // Phase 124
            required_caps: Default::default(),
            cond_sig: Default::default(),
        }
    }

    #[test]
    fn test_exit_parity_match() {
        let c1 = make_contract(vec![ExitKind::Return], vec!["x"]);
        let c2 = make_contract(vec![ExitKind::Return], vec!["x"]);
        let result = compare_exit_contracts(&c1, &c2);
        assert!(result.ok);
    }

    #[test]
    fn test_exit_parity_mismatch() {
        let c1 = make_contract(vec![ExitKind::Return], vec!["x"]);
        let c2 = make_contract(vec![ExitKind::Break], vec!["x"]);
        let result = compare_exit_contracts(&c1, &c2);
        assert!(!result.ok);
        assert_eq!(result.mismatch_kind, Some(MismatchKind::ExitMismatch));
        assert!(result.hint.is_some());
    }

    #[test]
    fn test_writes_parity_match() {
        let c1 = make_contract(vec![ExitKind::Return], vec!["x", "y"]);
        let c2 = make_contract(vec![ExitKind::Return], vec!["x", "y"]);
        let result = compare_writes_contracts(&c1, &c2);
        assert!(result.ok);
    }

    #[test]
    fn test_writes_parity_mismatch() {
        let c1 = make_contract(vec![ExitKind::Return], vec!["x"]);
        let c2 = make_contract(vec![ExitKind::Return], vec!["x", "y"]);
        let result = compare_writes_contracts(&c1, &c2);
        assert!(!result.ok);
        assert_eq!(result.mismatch_kind, Some(MismatchKind::WritesMismatch));
        assert!(result.hint.is_some());
    }

    #[test]
    fn test_full_parity_ok() {
        let c1 = make_contract(vec![ExitKind::Return], vec!["x"]);
        let c2 = make_contract(vec![ExitKind::Return], vec!["x"]);
        let result = check_full_parity(&c1, &c2);
        assert!(result.ok);
    }

    #[test]
    fn test_full_parity_exit_mismatch() {
        let c1 = make_contract(vec![ExitKind::Return], vec!["x"]);
        let c2 = make_contract(vec![ExitKind::Break], vec!["x"]);
        let result = check_full_parity(&c1, &c2);
        assert!(!result.ok);
        assert_eq!(result.mismatch_kind, Some(MismatchKind::ExitMismatch));
    }

    #[test]
    fn test_full_parity_writes_mismatch() {
        let c1 = make_contract(vec![ExitKind::Return], vec!["x"]);
        let c2 = make_contract(vec![ExitKind::Return], vec!["x", "y"]);
        let result = check_full_parity(&c1, &c2);
        assert!(!result.ok);
        assert_eq!(result.mismatch_kind, Some(MismatchKind::WritesMismatch));
    }
}
