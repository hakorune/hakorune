//! Behavior-neutral canonical direct-call materialization facade.
//!
//! Only a verified callable header can construct the target projection. This
//! box never resolves source names, consults the MIR module table, or invokes
//! legacy call recovery.

use crate::mir::canonical_direct_call_contract::{
    VerifiedDirectCallEffectV1, VerifiedTrivialDirectCallTargetV1,
};
use crate::mir::resolved_semantics::VerifiedCallableHeaderV1;
use crate::mir::resolved_value_profile::VerifiedTrivialDirectCallV1;
use crate::mir::{Callee, Effect, EffectMask, MirInstruction, ValueId};

pub(crate) fn materialize_direct_call_effect_v1(effect: VerifiedDirectCallEffectV1) -> EffectMask {
    match effect {
        VerifiedDirectCallEffectV1::ConservativeBarrier => EffectMask::MUT
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

    pub(crate) fn from_verified_profile(row: &VerifiedTrivialDirectCallV1) -> Self {
        Self {
            target: row.target().clone(),
            effect: row.effect(),
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
            effects: materialize_direct_call_effect_v1(self.effect),
        })
    }
}

#[cfg(test)]
#[path = "canonical_direct_call_tests.rs"]
mod tests;
