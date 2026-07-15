//! Pre-materialization contract for one verified canonical direct call.
//!
//! This module owns no MIR identity and performs no source lookup.  The target
//! projection is derived once from the source-unit callable index and can be
//! consumed by both the disconnected value profile and the later materializer.

use crate::mir::resolved_semantics::{
    CanonicalCallableSymbolV1, ExactTrivialCallableSignatureV1, ResolvedCallableRefV1,
    VerifiedCallableHeaderV1,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct VerifiedTrivialDirectCallTargetV1 {
    callable: ResolvedCallableRefV1,
    symbol: CanonicalCallableSymbolV1,
    signature: ExactTrivialCallableSignatureV1,
}

impl VerifiedTrivialDirectCallTargetV1 {
    pub(crate) fn from_header(header: &VerifiedCallableHeaderV1) -> Self {
        Self {
            callable: header.callable(),
            symbol: header.symbol().clone(),
            signature: header.signature().clone(),
        }
    }

    pub(crate) const fn callable(&self) -> ResolvedCallableRefV1 {
        self.callable
    }

    pub(crate) const fn symbol(&self) -> &CanonicalCallableSymbolV1 {
        &self.symbol
    }

    pub(crate) const fn signature(&self) -> &ExactTrivialCallableSignatureV1 {
        &self.signature
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum VerifiedDirectCallEffectV1 {
    ConservativeBarrier,
}
