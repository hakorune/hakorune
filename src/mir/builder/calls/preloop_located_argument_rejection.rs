//! Typed rejection vocabulary for the disconnected pre-loop argument port.
//!
//! The port owns one source-sealed selected argument but has no lowering
//! ingress in PORT0. Keeping that boundary explicit prevents the ordinary raw
//! facade from accidentally activating the candidate route.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum PreloopLocatedArgumentPortErrorV1 {
    SelectedArgumentUnavailable { index: u32 },
    CandidateIngressPending,
}

impl PreloopLocatedArgumentPortErrorV1 {
    pub(in crate::mir::builder) fn bounded_message(self) -> String {
        match self {
            Self::SelectedArgumentUnavailable { index } => {
                format!(
                    "[preloop-located-argument-port/selected-argument-unavailable] index={index}"
                )
            }
            Self::CandidateIngressPending => {
                "[preloop-located-argument-port/candidate-ingress-pending]".to_string()
            }
        }
    }
}
