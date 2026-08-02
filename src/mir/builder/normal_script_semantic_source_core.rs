//! Stable Script semantic source authority shared by every receipt family.

use super::normal_default_root_catalog_lifecycle::PreparedNormalDefaultProgramRootV1;
use crate::mir::compiler::source_projection::VerifiedSourceProjectionV1;
use crate::mir::resolved_semantics::{SemanticOwnerRootProfileV1, VerifiedSemanticOwnerForestV1};

/// The stable shared owner products. Receipt families may not add authority
/// here: they only authorize exact lowering descendants around this core.
#[derive(Debug)]
pub(super) struct ScriptSemanticSourceCoreV1<'source> {
    source: &'source PreparedNormalDefaultProgramRootV1,
    forest: VerifiedSemanticOwnerForestV1,
    projection: VerifiedSourceProjectionV1,
    runtime_source_indices: Box<[usize]>,
}

impl<'source> ScriptSemanticSourceCoreV1<'source> {
    pub(super) fn seal(
        source: &'source PreparedNormalDefaultProgramRootV1,
        forest: VerifiedSemanticOwnerForestV1,
        runtime_source_indices: Box<[usize]>,
    ) -> Result<Self, String> {
        let projection = VerifiedSourceProjectionV1::seal_with_root_profile(
            source.source_ast(),
            &forest,
            SemanticOwnerRootProfileV1::Script,
        )
        .map_err(|error| format!("[mir/script-semantic/projection] {error}"))?;
        Ok(Self {
            source,
            forest,
            projection,
            runtime_source_indices,
        })
    }

    pub(super) fn source(&self) -> &PreparedNormalDefaultProgramRootV1 {
        self.source
    }

    pub(super) fn forest(&self) -> &VerifiedSemanticOwnerForestV1 {
        &self.forest
    }

    pub(super) fn projection(&self) -> &VerifiedSourceProjectionV1 {
        &self.projection
    }

    pub(super) fn runtime_source_indices(&self) -> &[usize] {
        &self.runtime_source_indices
    }
}
