//! Pre-materialization contract for one verified canonical direct call.
//!
//! This module owns no MIR identity and performs no source lookup.  The target
//! projection is derived once from the source-unit callable index and can be
//! consumed by both the disconnected value profile and the later materializer.

use crate::mir::resolved_semantics::{
    CanonicalCallableKeyV1, CanonicalCallableSymbolV1, ExactTrivialCallableSignatureV1,
    ResolvedCallableRefV1, VerifiedCallableHeaderV1,
};
use hakorune_mir_defs::CanonicalSameModuleCallableKeyV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct VerifiedTrivialDirectCallTargetV1 {
    callable: ResolvedCallableRefV1,
    source_key: CanonicalCallableKeyV1,
    symbol: CanonicalCallableSymbolV1,
    signature: ExactTrivialCallableSignatureV1,
    /// Optional published definition identity.  FreeStatic source resolution
    /// is intentionally unqualified, while a source-backed static child may
    /// already have an exact same-module catalog key.  The package joins
    /// those two existing products once; materialization never re-resolves
    /// the source name or physical symbol.
    published_key: Option<CanonicalSameModuleCallableKeyV1>,
}

impl VerifiedTrivialDirectCallTargetV1 {
    pub(crate) fn from_header(header: &VerifiedCallableHeaderV1) -> Self {
        Self {
            callable: header.callable(),
            source_key: header.source_key().clone(),
            symbol: header.symbol().clone(),
            signature: header.signature().clone(),
            published_key: None,
        }
    }

    pub(crate) fn from_header_with_published_key(
        header: &VerifiedCallableHeaderV1,
        published_key: CanonicalSameModuleCallableKeyV1,
    ) -> Self {
        Self {
            callable: header.callable(),
            source_key: header.source_key().clone(),
            symbol: header.symbol().clone(),
            signature: header.signature().clone(),
            published_key: Some(published_key),
        }
    }

    pub(crate) const fn callable(&self) -> ResolvedCallableRefV1 {
        self.callable
    }

    pub(crate) const fn symbol(&self) -> &CanonicalCallableSymbolV1 {
        &self.symbol
    }

    pub(crate) const fn source_key(&self) -> &CanonicalCallableKeyV1 {
        &self.source_key
    }

    pub(crate) const fn signature(&self) -> &ExactTrivialCallableSignatureV1 {
        &self.signature
    }

    pub(crate) fn published_key(&self) -> Option<&CanonicalSameModuleCallableKeyV1> {
        self.published_key.as_ref()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum VerifiedDirectCallEffectV1 {
    ConservativeBarrier,
}
