//! Disconnected exact-type decisions for successful LocalSSA physical Copies.
//!
//! COPY0-S0 consumes the existing map-free `TypeFactDecisionV1`; it neither
//! reads nor writes `TypeContext`, allocates values, nor emits MIR.

use hakorune_mir_builder::lowering_facts::{
    PreparedTypeFactPublicationV1, TypeFactDecisionErrorV1, TypeFactDecisionV1,
};

use super::post_success::{LocalSsaMaterializationKindV1, LocalSsaSourceTypeEntryV1};
use crate::mir::MirType;

/// The physical-Copy-only decision prepared before a future successful commit.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PreparedLocalSsaPhysicalCopyTypeV1 {
    publication: PreparedTypeFactPublicationV1,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum LocalSsaPhysicalCopyTypeErrorV1 {
    NotPhysicalCopy(LocalSsaMaterializationKindV1),
    FactDecision(TypeFactDecisionErrorV1),
}

impl PreparedLocalSsaPhysicalCopyTypeV1 {
    /// Prepares an exact type fact only for the two already-classified physical
    /// Copy materializations. Missing and StoredUnknown make no proposal.
    pub(super) fn prepare(
        source_type: &LocalSsaSourceTypeEntryV1,
        materialization: LocalSsaMaterializationKindV1,
        existing_destination: Option<&MirType>,
    ) -> Result<Self, LocalSsaPhysicalCopyTypeErrorV1> {
        if !matches!(
            materialization,
            LocalSsaMaterializationKindV1::PhysicalCopy(_)
        ) {
            return Err(LocalSsaPhysicalCopyTypeErrorV1::NotPhysicalCopy(
                materialization,
            ));
        }

        let candidate = match source_type {
            LocalSsaSourceTypeEntryV1::Missing | LocalSsaSourceTypeEntryV1::StoredUnknown => None,
            LocalSsaSourceTypeEntryV1::Exact(ty) => Some(ty),
        };
        let publication = TypeFactDecisionV1::prepare(existing_destination, candidate)
            .map_err(LocalSsaPhysicalCopyTypeErrorV1::FactDecision)?;
        Ok(Self { publication })
    }

    #[cfg(test)]
    fn publication(&self) -> &PreparedTypeFactPublicationV1 {
        &self.publication
    }
}

#[cfg(test)]
mod tests {
    use super::{LocalSsaPhysicalCopyTypeErrorV1, PreparedLocalSsaPhysicalCopyTypeV1};
    use crate::mir::builder::ssa::local::post_success::{
        LocalSsaMaterializationKindV1, LocalSsaPhysicalCopyReasonV1, LocalSsaSourceTypeEntryV1,
    };
    use crate::mir::MirType;
    use hakorune_mir_builder::lowering_facts::{
        PreparedTypeFactPublicationV1, TypeFactDecisionErrorV1,
    };

    const FALLBACK_COPY: LocalSsaMaterializationKindV1 =
        LocalSsaMaterializationKindV1::PhysicalCopy(
            LocalSsaPhysicalCopyReasonV1::DominatingFallbackCopy,
        );
    const REMATERIALIZED_COPY: LocalSsaMaterializationKindV1 =
        LocalSsaMaterializationKindV1::PhysicalCopy(
            LocalSsaPhysicalCopyReasonV1::RematerializedCopy,
        );

    #[test]
    fn both_physical_copy_reasons_delegate_exact_types_to_the_existing_decision() {
        for materialization in [FALLBACK_COPY, REMATERIALIZED_COPY] {
            let prepared = PreparedLocalSsaPhysicalCopyTypeV1::prepare(
                &LocalSsaSourceTypeEntryV1::Exact(MirType::Integer),
                materialization,
                None,
            )
            .unwrap();
            assert_eq!(
                prepared.publication(),
                &PreparedTypeFactPublicationV1::Publish(MirType::Integer)
            );
        }

        let idempotent = PreparedLocalSsaPhysicalCopyTypeV1::prepare(
            &LocalSsaSourceTypeEntryV1::Exact(MirType::Integer),
            FALLBACK_COPY,
            Some(&MirType::Integer),
        )
        .unwrap();
        assert_eq!(
            idempotent.publication(),
            &PreparedTypeFactPublicationV1::Idempotent(MirType::Integer)
        );
    }

    #[test]
    fn missing_and_stored_unknown_do_not_propose_exact_type_facts() {
        for materialization in [FALLBACK_COPY, REMATERIALIZED_COPY] {
            for source in [
                LocalSsaSourceTypeEntryV1::Missing,
                LocalSsaSourceTypeEntryV1::StoredUnknown,
            ] {
                let prepared =
                    PreparedLocalSsaPhysicalCopyTypeV1::prepare(&source, materialization, None)
                        .unwrap();
                assert_eq!(
                    prepared.publication(),
                    &PreparedTypeFactPublicationV1::NoPublication
                );
            }
        }
    }

    #[test]
    fn non_copy_and_concrete_conflict_reject_before_any_commit() {
        assert_eq!(
            PreparedLocalSsaPhysicalCopyTypeV1::prepare(
                &LocalSsaSourceTypeEntryV1::Exact(MirType::Integer),
                LocalSsaMaterializationKindV1::RematerializedSelect,
                None,
            ),
            Err(LocalSsaPhysicalCopyTypeErrorV1::NotPhysicalCopy(
                LocalSsaMaterializationKindV1::RematerializedSelect
            ))
        );

        assert_eq!(
            PreparedLocalSsaPhysicalCopyTypeV1::prepare(
                &LocalSsaSourceTypeEntryV1::Exact(MirType::Integer),
                FALLBACK_COPY,
                Some(&MirType::String),
            ),
            Err(LocalSsaPhysicalCopyTypeErrorV1::FactDecision(
                TypeFactDecisionErrorV1::ConcreteFactConflict {
                    existing: MirType::String,
                    proposed: MirType::Integer,
                }
            ))
        );
    }

    #[test]
    fn failed_decision_has_no_publication_product() {
        let result = PreparedLocalSsaPhysicalCopyTypeV1::prepare(
            &LocalSsaSourceTypeEntryV1::Exact(MirType::Integer),
            REMATERIALIZED_COPY,
            Some(&MirType::String),
        );

        assert!(matches!(
            result,
            Err(LocalSsaPhysicalCopyTypeErrorV1::FactDecision(
                TypeFactDecisionErrorV1::ConcreteFactConflict { .. }
            ))
        ));
    }
}
