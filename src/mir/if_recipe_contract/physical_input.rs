//! One-shot pairing of a verified If artifact and its logical JoinSig.
//!
//! The pair remains caller-zero and physical-ID-free.  Consuming the artifact
//! here prevents a later consumer from mixing a signature from another recipe
//! or source receipt.

use super::join_sig::{IfJoinSigElaboratorV1, IfJoinSigRejectReasonV1, VerifiedIfJoinSigV1};
use super::verify::VerifiedIfRecipeArtifactV1;

#[derive(Debug)]
pub(crate) struct VerifiedIfPhysicalInputV1 {
    artifact: VerifiedIfRecipeArtifactV1,
    join_sig: VerifiedIfJoinSigV1,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum IfPhysicalInputRejectReasonV1 {
    JoinSig(IfJoinSigRejectReasonV1),
}

impl VerifiedIfPhysicalInputV1 {
    pub(crate) fn from_artifact(
        artifact: VerifiedIfRecipeArtifactV1,
    ) -> Result<Self, IfPhysicalInputRejectReasonV1> {
        let join_sig = IfJoinSigElaboratorV1::elaborate(artifact.recipe())
            .map_err(IfPhysicalInputRejectReasonV1::JoinSig)?;
        Ok(Self { artifact, join_sig })
    }

    pub(crate) fn artifact(&self) -> &VerifiedIfRecipeArtifactV1 {
        &self.artifact
    }

    pub(crate) fn join_sig(&self) -> &VerifiedIfJoinSigV1 {
        &self.join_sig
    }

    pub(crate) fn into_parts(self) -> (VerifiedIfRecipeArtifactV1, VerifiedIfJoinSigV1) {
        (self.artifact, self.join_sig)
    }
}
