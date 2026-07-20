//! LocalSSA materialization failure policy.

use crate::mir::ValueId;

/// Checked failures; legacy facades recover only block/emission failures.
pub(super) enum LocalSsaMaterializationErrorV1 {
    Contract(String),
    BlockCreation(String),
    InstructionEmission(String),
}

impl LocalSsaMaterializationErrorV1 {
    fn message(&self) -> &str {
        match self {
            Self::Contract(message)
            | Self::BlockCreation(message)
            | Self::InstructionEmission(message) => message,
        }
    }
}

#[derive(Clone, Copy)]
#[allow(dead_code)] // Checked is reserved for COPY0's first selected consumer.
pub(super) enum LocalSsaFailurePolicyV1 {
    LegacyFacade,
    Checked,
}

impl LocalSsaFailurePolicyV1 {
    pub(super) fn resolve(
        self,
        original: ValueId,
        error: LocalSsaMaterializationErrorV1,
    ) -> Result<ValueId, LocalSsaMaterializationErrorV1> {
        match self {
            Self::LegacyFacade => {
                let _ = error.message();
                Ok(original)
            }
            Self::Checked => Err(error),
        }
    }
}
