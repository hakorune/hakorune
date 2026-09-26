//! Existing expression-port value and NoValue source projections.
use crate::mir::builder::CanonicalSameModuleCallableKeyV1;
use crate::mir::{EffectMask, MirType, ValueId};
/// Exact source-call projection consumed by the normalizer for one
/// resolver-issued CoreMethod contract.  It carries physical values only;
/// semantic identity remains in the source contract owner.
#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct ExactSourceMethodCallV1 {
    receiver: Option<ValueId>,
    static_target: Option<CanonicalSameModuleCallableKeyV1>,
    result_type: MirType,
    effects: EffectMask,
}

impl ExactSourceMethodCallV1 {
    pub(in crate::mir::builder) const fn new(
        receiver: ValueId,
        result_type: MirType,
        effects: EffectMask,
    ) -> Self {
        Self {
            receiver: Some(receiver),
            static_target: None,
            result_type,
            effects,
        }
    }

    pub(in crate::mir::builder) const fn static_publication(
        target: CanonicalSameModuleCallableKeyV1,
        result_type: MirType,
        effects: EffectMask,
    ) -> Self {
        Self {
            receiver: None,
            static_target: Some(target),
            result_type,
            effects,
        }
    }

    pub(in crate::mir::builder) fn receiver(&self) -> Option<ValueId> {
        self.receiver
    }

    pub(in crate::mir::builder) fn static_target(
        &self,
    ) -> Option<&CanonicalSameModuleCallableKeyV1> {
        self.static_target.as_ref()
    }

    pub(in crate::mir::builder) fn result_type(&self) -> MirType {
        self.result_type.clone()
    }

    pub(in crate::mir::builder) fn effects(&self) -> EffectMask {
        self.effects
    }
}

/// Exact source projection for one resolver-issued declared-instance
/// (`me.method`) locator row.  It carries the canonical same-module target
/// key and the exact receiver value already bound by the ledger; the
/// normalizer must never fall back to dynamic method dispatch once this
/// projection exists.
#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct ExactSourceDeclaredInstanceCallV1 {
    key: CanonicalSameModuleCallableKeyV1,
    receiver: ValueId,
}

impl ExactSourceDeclaredInstanceCallV1 {
    pub(in crate::mir::builder) const fn new(
        key: CanonicalSameModuleCallableKeyV1,
        receiver: ValueId,
    ) -> Self {
        Self { key, receiver }
    }

    pub(in crate::mir::builder) const fn key(&self) -> &CanonicalSameModuleCallableKeyV1 {
        &self.key
    }

    pub(in crate::mir::builder) const fn receiver(&self) -> ValueId {
        self.receiver
    }
}

#[derive(Debug)]
pub(in crate::mir::builder) enum ExactSourceStatementCallV1 {
    ArrayPush {
        receiver: ValueId,
        emission: crate::mir::normal_callable_semantic_package::NamedArrayWriteEmissionPortV1,
    },
}
