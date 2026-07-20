//! Disconnected exact-type preparation for sealed trivial operations.
//!
//! RESOLVED-TRIVIAL-OP0-S0 owns only the representation-to-type decision.
//! The operation emitter remains the future sole instruction-success consumer;
//! this module has no Builder, ValueId allocation, instruction, or commit path.

use hakorune_mir_builder::lowering_facts::{
    PreparedTypeFactPublicationV1, TypeFactDecisionErrorV1, TypeFactDecisionV1,
};

use crate::mir::builder::type_context::TypeContext;
use crate::mir::resolved_value_profile::product::TrivialRepresentationV1;
use crate::mir::{MirType, ValueId};

/// Prepared exact type decision for one future successful trivial operation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct PreparedResolvedTrivialOperationTypeV1 {
    publication: PreparedTypeFactPublicationV1,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) enum ResolvedTrivialOperationTypeErrorV1 {
    FactDecision(TypeFactDecisionErrorV1),
}

impl std::fmt::Display for ResolvedTrivialOperationTypeErrorV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FactDecision(error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for ResolvedTrivialOperationTypeErrorV1 {}

impl PreparedResolvedTrivialOperationTypeV1 {
    /// Prepares the profile-sealed exact result type without mutating Builder
    /// state. A later I0 consumer may commit this only after instruction
    /// emission succeeds.
    pub(super) fn prepare(
        representation: TrivialRepresentationV1,
        existing_destination: Option<&MirType>,
    ) -> Result<Self, ResolvedTrivialOperationTypeErrorV1> {
        let candidate = exact_type_for_representation(representation);
        let publication = TypeFactDecisionV1::prepare(existing_destination, Some(&candidate))
            .map_err(ResolvedTrivialOperationTypeErrorV1::FactDecision)?;
        Ok(Self { publication })
    }

    /// Commits only a decision prepared before the selected operation emits.
    ///
    /// I0's sole production caller invokes this after `BinOp` or `Compare`
    /// emission succeeds. Idempotent decisions preserve an existing exact fact.
    pub(super) fn commit(self, destination: ValueId, type_ctx: &mut TypeContext) {
        if let PreparedTypeFactPublicationV1::Publish(ty) = self.publication {
            type_ctx.set_type(destination, ty);
        }
    }

    #[cfg(test)]
    fn publication(&self) -> &PreparedTypeFactPublicationV1 {
        &self.publication
    }
}

/// The sole resolved-trivial representation-to-MIR-type projection.
pub(super) const fn exact_type_for_representation(
    representation: TrivialRepresentationV1,
) -> MirType {
    match representation {
        TrivialRepresentationV1::InlineI64 => MirType::Integer,
        TrivialRepresentationV1::InlineBool => MirType::Bool,
        TrivialRepresentationV1::InlineF64 => MirType::Float,
        TrivialRepresentationV1::ExplicitVoidValue | TrivialRepresentationV1::NullSentinel => {
            MirType::Void
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        exact_type_for_representation, PreparedResolvedTrivialOperationTypeV1,
        ResolvedTrivialOperationTypeErrorV1,
    };
    use crate::mir::resolved_value_profile::product::TrivialRepresentationV1;
    use crate::mir::MirType;
    use hakorune_mir_builder::lowering_facts::{
        PreparedTypeFactPublicationV1, TypeFactDecisionErrorV1,
    };

    #[test]
    fn every_sealed_trivial_representation_has_one_exact_mir_type() {
        let cases = [
            (TrivialRepresentationV1::InlineI64, MirType::Integer),
            (TrivialRepresentationV1::InlineBool, MirType::Bool),
            (TrivialRepresentationV1::InlineF64, MirType::Float),
            (TrivialRepresentationV1::ExplicitVoidValue, MirType::Void),
            (TrivialRepresentationV1::NullSentinel, MirType::Void),
        ];

        for (representation, expected) in cases {
            assert_eq!(exact_type_for_representation(representation), expected);
            assert_ne!(expected, MirType::Unknown);
        }
    }

    #[test]
    fn missing_or_unknown_destination_prepares_the_exact_profile_type() {
        for existing in [None, Some(&MirType::Unknown)] {
            let prepared = PreparedResolvedTrivialOperationTypeV1::prepare(
                TrivialRepresentationV1::InlineBool,
                existing,
            )
            .unwrap();
            assert_eq!(
                prepared.publication(),
                &PreparedTypeFactPublicationV1::Publish(MirType::Bool)
            );
        }
    }

    #[test]
    fn matching_void_is_idempotent_for_both_void_representations() {
        for representation in [
            TrivialRepresentationV1::ExplicitVoidValue,
            TrivialRepresentationV1::NullSentinel,
        ] {
            let prepared = PreparedResolvedTrivialOperationTypeV1::prepare(
                representation,
                Some(&MirType::Void),
            )
            .unwrap();
            assert_eq!(
                prepared.publication(),
                &PreparedTypeFactPublicationV1::Idempotent(MirType::Void)
            );
        }
    }

    #[test]
    fn conflicting_concrete_destination_rejects_without_a_publication() {
        assert_eq!(
            PreparedResolvedTrivialOperationTypeV1::prepare(
                TrivialRepresentationV1::InlineF64,
                Some(&MirType::Integer),
            ),
            Err(ResolvedTrivialOperationTypeErrorV1::FactDecision(
                TypeFactDecisionErrorV1::ConcreteFactConflict {
                    existing: MirType::Integer,
                    proposed: MirType::Float,
                }
            ))
        );
    }
}
