//! Stable Script semantic source authority shared by every receipt family.

use crate::ast::ASTNode;
use crate::mir::compiler::source_projection::VerifiedSourceProjectionV1;
use crate::mir::resolved_semantics::{SemanticOwnerRootProfileV1, VerifiedSemanticOwnerForestV1};

/// The stable shared owner products. Receipt families may not add authority
/// here: they only authorize exact lowering descendants around this core.
#[derive(Debug)]
pub(super) struct ScriptSemanticSourceCoreV1<'source> {
    source: &'source ASTNode,
    forest: VerifiedSemanticOwnerForestV1,
    projection: VerifiedSourceProjectionV1,
    runtime_source_indices: Box<[usize]>,
}

#[derive(Debug)]
pub(super) struct ScriptSemanticSourcePreEffectCorePartsV1 {
    pub(super) forest: VerifiedSemanticOwnerForestV1,
    pub(super) projection: VerifiedSourceProjectionV1,
    pub(super) runtime_source_indices: Box<[usize]>,
}

impl ScriptSemanticSourcePreEffectCorePartsV1 {
    pub(super) fn forest(&self) -> &VerifiedSemanticOwnerForestV1 {
        &self.forest
    }
}

impl<'source> ScriptSemanticSourceCoreV1<'source> {
    pub(super) fn seal(
        source: &'source ASTNode,
        forest: VerifiedSemanticOwnerForestV1,
        runtime_source_indices: Box<[usize]>,
    ) -> Result<Self, String> {
        let projection = VerifiedSourceProjectionV1::seal_with_root_profile(
            source,
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

    pub(super) fn source(&self) -> &ASTNode {
        self.source
    }

    pub(super) fn forest(&self) -> &VerifiedSemanticOwnerForestV1 {
        &self.forest
    }

    pub(super) fn bind_pre_effect_parts(
        source: &'source ASTNode,
        parts: ScriptSemanticSourcePreEffectCorePartsV1,
    ) -> Self {
        Self {
            source,
            forest: parts.forest,
            projection: parts.projection,
            runtime_source_indices: parts.runtime_source_indices,
        }
    }

    pub(super) fn into_pre_effect_parts(self) -> ScriptSemanticSourcePreEffectCorePartsV1 {
        ScriptSemanticSourcePreEffectCorePartsV1 {
            forest: self.forest,
            projection: self.projection,
            runtime_source_indices: self.runtime_source_indices,
        }
    }

    pub(super) fn projection(&self) -> &VerifiedSourceProjectionV1 {
        &self.projection
    }

    pub(super) fn runtime_source_indices(&self) -> &[usize] {
        &self.runtime_source_indices
    }
}
