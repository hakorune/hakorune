//! Pure lowering-time PHI destination-type decision.
//!
//! This module owns no PHI insertion or Builder lifecycle entry. It prepares
//! one type-only decision from logical incoming values; lifecycle producers
//! may consume that decision only in the later TYPE-PUBLISH0-I0 row.

mod commit;
mod decision;

use crate::mir::{BasicBlockId, MirType, ValueId};

// Disconnected through S0; the later I0 row connects this prepared commit.
#[allow(unused_imports)]
pub(in crate::mir::builder) use commit::commit_prepared_phi_type;

/// Sole Builder-lowering owner for the pure PHI type decision.
#[derive(Debug)]
pub(in crate::mir::builder) struct PhiTransientTypeDecisionV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) enum PhiTypeFactSiteV1 {
    ExistingDestination,
    ExplicitTypeHint,
    Incoming {
        predecessor: BasicBlockId,
        value: ValueId,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) struct PhiConcreteTypeConflictV1 {
    pub(in crate::mir::builder) dst: ValueId,
    pub(in crate::mir::builder) first_site: PhiTypeFactSiteV1,
    pub(in crate::mir::builder) first_type: MirType,
    pub(in crate::mir::builder) second_site: PhiTypeFactSiteV1,
    pub(in crate::mir::builder) second_type: MirType,
}

impl std::fmt::Display for PhiConcreteTypeConflictV1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[freeze:contract][phi_type_publication/concrete_fact_conflict] \
             dst={} first_site={:?} first_type={:?} second_site={:?} second_type={:?}",
            self.dst, self.first_site, self.first_type, self.second_site, self.second_type
        )
    }
}

impl std::error::Error for PhiConcreteTypeConflictV1 {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) enum PhiTypeNoPublicationReasonV1 {
    EmptyInputs,
    MissingInputType {
        predecessor: BasicBlockId,
        value: ValueId,
    },
    UnknownInputType {
        predecessor: BasicBlockId,
        value: ValueId,
    },
    HeterogeneousInputTypes,
}

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum PreparedPhiTypePublicationV1 {
    Publish(MirType),
    Idempotent(MirType),
    PreserveExisting {
        existing: MirType,
        reason: PhiTypeNoPublicationReasonV1,
    },
    NoPublication(PhiTypeNoPublicationReasonV1),
}

#[cfg(test)]
mod tests;
