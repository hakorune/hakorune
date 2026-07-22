//! CUT0-S0-COMPAT0: sealed callable-Main compatibility policy.
//!
//! The environment is read only at module ingress.  Body lowering consumes
//! this typed snapshot and never re-reads the ambient toggle.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum CallableMainCompatibilityPolicyV1 {
    Omitted,
    Required,
}

impl CallableMainCompatibilityPolicyV1 {
    pub(in crate::mir::builder) fn snapshot_from_legacy_ingress() -> Self {
        if crate::config::env::builder_build_static_main_entry() {
            Self::Required
        } else {
            Self::Omitted
        }
    }

    pub(in crate::mir::builder) const fn is_required(self) -> bool {
        matches!(self, Self::Required)
    }
}

#[cfg(test)]
mod tests {
    use super::CallableMainCompatibilityPolicyV1;

    #[test]
    fn policy_has_exclusive_dispositions() {
        assert!(!CallableMainCompatibilityPolicyV1::Omitted.is_required());
        assert!(CallableMainCompatibilityPolicyV1::Required.is_required());
    }
}
