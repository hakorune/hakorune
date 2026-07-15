//! Behavior-neutral canonical direct-call materialization facade.
//!
//! Only a verified callable header can construct the target projection. This
//! box never resolves source names, consults the MIR module table, or invokes
//! legacy call recovery.

#![allow(dead_code)] // Passive until P0c-S0 supplies the first sealed call row.

use crate::mir::resolved_semantics::{
    CanonicalCallableSymbolV1, ExactTrivialCallableSignatureV1, ResolvedCallableRefV1,
    VerifiedCallableHeaderV1,
};
use crate::mir::{Callee, Effect, EffectMask, MirInstruction, ValueId};

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

impl VerifiedDirectCallEffectV1 {
    pub(crate) fn mir_effects(self) -> EffectMask {
        match self {
            Self::ConservativeBarrier => EffectMask::MUT
                .union(EffectMask::IO)
                .union(EffectMask::WRITE)
                .add(Effect::Control)
                .add(Effect::P2P)
                .add(Effect::FFI)
                .add(Effect::Panic)
                .add(Effect::Alloc)
                .add(Effect::Global)
                .add(Effect::Async)
                .add(Effect::Unsafe)
                .add(Effect::Debug)
                .add(Effect::Barrier),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct VerifiedCanonicalDirectCallEmissionV1 {
    target: VerifiedTrivialDirectCallTargetV1,
    effect: VerifiedDirectCallEffectV1,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum DirectCallEmissionErrorV1 {
    ArgumentCardinality { expected: usize, actual: usize },
}

impl VerifiedCanonicalDirectCallEmissionV1 {
    pub(crate) fn conservative_from_header(header: &VerifiedCallableHeaderV1) -> Self {
        Self {
            target: VerifiedTrivialDirectCallTargetV1::from_header(header),
            effect: VerifiedDirectCallEffectV1::ConservativeBarrier,
        }
    }

    pub(crate) const fn target(&self) -> &VerifiedTrivialDirectCallTargetV1 {
        &self.target
    }

    pub(crate) const fn effect(&self) -> VerifiedDirectCallEffectV1 {
        self.effect
    }

    pub(crate) fn materialize(
        self,
        dst: ValueId,
        args: Vec<ValueId>,
    ) -> Result<MirInstruction, DirectCallEmissionErrorV1> {
        let expected = self.target.signature().arity();
        if args.len() != expected {
            return Err(DirectCallEmissionErrorV1::ArgumentCardinality {
                expected,
                actual: args.len(),
            });
        }
        Ok(MirInstruction::Call {
            dst: Some(dst),
            func: ValueId::INVALID,
            callee: Some(Callee::Global(
                self.target.symbol().as_mir_name().to_string(),
            )),
            args,
            effects: self.effect.mir_effects(),
        })
    }
}

#[cfg(test)]
#[path = "canonical_direct_call_tests.rs"]
mod tests;
