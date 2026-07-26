//! Backend-neutral published source-entry transport.
//!
//! This layer retains a complete family owner, an exact entry target, and the
//! already-sealed source-result contract. It does not execute a backend or own
//! process-exit policy.

use super::source_entry_result::UnitOriginV1;
use crate::mir::module_invocation_identity::ModuleInvocationBrandV1;
use crate::mir::resolved_semantics::FunctionOwnerIdV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum PublishedUnitPhysicalContractV1 {
    ExactVoid,
    CompatiblePayload,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum PublishedSourceEntryResultContractV1 {
    Unit {
        origin: UnitOriginV1,
        physical: PublishedUnitPhysicalContractV1,
    },
    Integer,
    Bool,
    Float,
    String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum PublishedSourceEntryMembershipV1 {
    Raw { brand: ModuleInvocationBrandV1 },
    CanonicalMain { source_owner: FunctionOwnerIdV1 },
}

#[derive(Debug)]
pub(in crate::mir) struct PendingPublishedSourceEntryTargetV1 {
    symbol: Box<str>,
    arity: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) enum PublishedSourceEntryTargetErrorV1 {
    EmptySymbol,
    ArityMismatch { actual: usize },
}

#[derive(Debug)]
pub(in crate::mir) struct RejectedPublishedSourceEntryTargetV1 {
    owner: PendingPublishedSourceEntryTargetV1,
    error: PublishedSourceEntryTargetErrorV1,
}

impl RejectedPublishedSourceEntryTargetV1 {
    pub(in crate::mir) fn error(&self) -> &PublishedSourceEntryTargetErrorV1 {
        &self.error
    }

    pub(in crate::mir) fn discard(self) {
        drop(self);
    }
}

#[derive(Debug)]
pub(in crate::mir) struct VerifiedPublishedSourceEntryTargetV1 {
    symbol: Box<str>,
    arity: usize,
    _seal: VerifiedPublishedSourceEntryTargetSealV1,
}

#[derive(Debug)]
struct VerifiedPublishedSourceEntryTargetSealV1;

impl PendingPublishedSourceEntryTargetV1 {
    pub(in crate::mir) fn new(symbol: impl Into<Box<str>>, arity: usize) -> Self {
        Self {
            symbol: symbol.into(),
            arity,
        }
    }

    pub(in crate::mir) fn seal(
        self,
    ) -> Result<VerifiedPublishedSourceEntryTargetV1, RejectedPublishedSourceEntryTargetV1> {
        let error = if self.symbol.is_empty() {
            Some(PublishedSourceEntryTargetErrorV1::EmptySymbol)
        } else if self.arity != 0 {
            Some(PublishedSourceEntryTargetErrorV1::ArityMismatch { actual: self.arity })
        } else {
            None
        };
        if let Some(error) = error {
            return Err(RejectedPublishedSourceEntryTargetV1 { owner: self, error });
        }
        Ok(VerifiedPublishedSourceEntryTargetV1 {
            symbol: self.symbol,
            arity: self.arity,
            _seal: VerifiedPublishedSourceEntryTargetSealV1,
        })
    }
}

impl VerifiedPublishedSourceEntryTargetV1 {
    pub(in crate::mir) fn symbol(&self) -> &str {
        &self.symbol
    }

    pub(in crate::mir) const fn arity(&self) -> usize {
        self.arity
    }
}

/// Move-only transport. `O` is the complete family-specific published owner.
#[derive(Debug)]
pub(in crate::mir) struct PublishedSourceEntryInvocationV1<O> {
    owner: O,
    target: VerifiedPublishedSourceEntryTargetV1,
    result: PublishedSourceEntryResultContractV1,
    membership: PublishedSourceEntryMembershipV1,
    _seal: PublishedSourceEntryInvocationSealV1,
}

#[derive(Debug)]
struct PublishedSourceEntryInvocationSealV1;

impl<O> PublishedSourceEntryInvocationV1<O> {
    /// Family adapters may call this only after validating their own pairing
    /// evidence. L0 has no production caller.
    pub(super) fn from_verified_parts(
        owner: O,
        target: VerifiedPublishedSourceEntryTargetV1,
        result: PublishedSourceEntryResultContractV1,
        membership: PublishedSourceEntryMembershipV1,
    ) -> Self {
        Self {
            owner,
            target,
            result,
            membership,
            _seal: PublishedSourceEntryInvocationSealV1,
        }
    }

    pub(in crate::mir) fn target(&self) -> &VerifiedPublishedSourceEntryTargetV1 {
        &self.target
    }

    pub(in crate::mir) const fn result(&self) -> PublishedSourceEntryResultContractV1 {
        self.result
    }

    pub(in crate::mir) const fn membership(&self) -> PublishedSourceEntryMembershipV1 {
        self.membership
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        O,
        VerifiedPublishedSourceEntryTargetV1,
        PublishedSourceEntryResultContractV1,
        PublishedSourceEntryMembershipV1,
    ) {
        (self.owner, self.target, self.result, self.membership)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::resolved_semantics::FunctionOwnerIssuerV1;

    #[derive(Debug, PartialEq, Eq)]
    struct DummyPublishedOwner(&'static str);

    fn target() -> VerifiedPublishedSourceEntryTargetV1 {
        PendingPublishedSourceEntryTargetV1::new("main", 0)
            .seal()
            .expect("exact target")
    }

    fn owner() -> FunctionOwnerIdV1 {
        FunctionOwnerIssuerV1::new_for_compilation()
            .expect("test owner issuer")
            .issue()
            .expect("test owner")
    }

    #[test]
    fn target_rejects_empty_symbol_and_nonzero_arity() {
        for (pending, expected) in [
            (
                PendingPublishedSourceEntryTargetV1::new("", 0),
                PublishedSourceEntryTargetErrorV1::EmptySymbol,
            ),
            (
                PendingPublishedSourceEntryTargetV1::new("main", 1),
                PublishedSourceEntryTargetErrorV1::ArityMismatch { actual: 1 },
            ),
        ] {
            let rejected = pending.seal().expect_err("target drift must reject");
            assert_eq!(rejected.error(), &expected);
            rejected.discard();
        }
    }

    #[test]
    fn invocation_retains_complete_owner_target_result_and_membership() {
        let membership = PublishedSourceEntryMembershipV1::CanonicalMain {
            source_owner: owner(),
        };
        let result = PublishedSourceEntryResultContractV1::Unit {
            origin: UnitOriginV1::ExplicitNull,
            physical: PublishedUnitPhysicalContractV1::ExactVoid,
        };
        let invocation = PublishedSourceEntryInvocationV1::from_verified_parts(
            DummyPublishedOwner("canonical"),
            target(),
            result,
            membership,
        );
        assert_eq!(invocation.target().symbol(), "main");
        assert_eq!(invocation.target().arity(), 0);
        assert_eq!(invocation.result(), result);
        assert_eq!(invocation.membership(), membership);
        let (owner, _, retained_result, retained_membership) = invocation.into_parts();
        assert_eq!(owner, DummyPublishedOwner("canonical"));
        assert_eq!(retained_result, result);
        assert_eq!(retained_membership, membership);
    }
}
