//! Source-origin retention for the selected construction-fault Reclaim.

use super::*;

/// Existing construction-fault obligation carried after exact New claim take.
/// It borrows source identity; it does not issue cleanup meaning from CFG.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ReclaimUnpublishedOriginV1 {
    pub(super) site: OwnedExprSiteV1,
    pub(super) constructor_source: crate::parser::ConstructorSourceIdV1,
    pub(super) constructor_owner: FunctionOwnerIdV1,
    pub(super) object: CanonicalObjectIdV1,
    /// Sealed owned `ArrayBox` children (declaration order) the
    /// construction-fault path releases before reclaiming storage.
    /// `None` on an `OwnedArrayFieldsNoHook` object means the proof was
    /// absent — the claim stays unavailable rather than reclaiming over
    /// live children.
    pub(super) array_children: Option<Box<[hakorune_mir_defs::CanonicalFieldRefV1]>>,
}

impl ReclaimUnpublishedOriginV1 {
    pub(crate) const fn object(&self) -> CanonicalObjectIdV1 {
        self.object
    }

    pub(crate) fn array_children(
        &self,
    ) -> Option<&[hakorune_mir_defs::CanonicalFieldRefV1]> {
        self.array_children.as_deref()
    }
}

#[derive(Debug)]
pub(super) struct ReclaimUnpublishedEmissionV1 {
    pub(super) origin: ReclaimUnpublishedOriginV1,
    pub(super) block: BasicBlockId,
    pub(super) instruction: MirInstruction,
}
