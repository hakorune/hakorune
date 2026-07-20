//! Disconnected exact-Integer preparation for sealed direct-call results.
//!
//! RESOLVED-DIRECT-CALL0-S0 admits only the existing first direct-call profile:
//! a sealed `InlineI64` result. It owns no Builder, call materialization,
//! instruction emission, capability, source-site, or commit path.

use hakorune_mir_builder::lowering_facts::{
    PreparedTypeFactPublicationV1, TypeFactDecisionErrorV1, TypeFactDecisionV1,
};

use crate::mir::resolved_value_profile::product::TrivialRepresentationV1;
use crate::mir::MirType;

/// Prepared Integer fact for one future successful sealed direct call.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct PreparedResolvedDirectCallIntegerTypeV1 {
    publication: PreparedTypeFactPublicationV1,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) enum ResolvedDirectCallIntegerTypeErrorV1 {
    UnsupportedResultRepresentation { actual: TrivialRepresentationV1 },
    FactDecision(TypeFactDecisionErrorV1),
}

impl std::fmt::Display for ResolvedDirectCallIntegerTypeErrorV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnsupportedResultRepresentation { actual } => write!(
                formatter,
                "[freeze:contract][resolved_direct_call/result_representation] actual={actual:?}"
            ),
            Self::FactDecision(error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for ResolvedDirectCallIntegerTypeErrorV1 {}

impl PreparedResolvedDirectCallIntegerTypeV1 {
    /// Prepares only the profile-sealed direct-call Integer result fact.
    pub(super) fn prepare(
        representation: TrivialRepresentationV1,
        existing_destination: Option<&MirType>,
    ) -> Result<Self, ResolvedDirectCallIntegerTypeErrorV1> {
        if representation != TrivialRepresentationV1::InlineI64 {
            return Err(
                ResolvedDirectCallIntegerTypeErrorV1::UnsupportedResultRepresentation {
                    actual: representation,
                },
            );
        }
        let publication =
            TypeFactDecisionV1::prepare(existing_destination, Some(&MirType::Integer))
                .map_err(ResolvedDirectCallIntegerTypeErrorV1::FactDecision)?;
        Ok(Self { publication })
    }

    #[cfg(test)]
    fn publication(&self) -> &PreparedTypeFactPublicationV1 {
        &self.publication
    }
}

#[cfg(test)]
mod tests {
    use super::{PreparedResolvedDirectCallIntegerTypeV1, ResolvedDirectCallIntegerTypeErrorV1};
    use crate::mir::resolved_value_profile::product::TrivialRepresentationV1;
    use crate::mir::MirType;
    use hakorune_mir_builder::lowering_facts::{
        PreparedTypeFactPublicationV1, TypeFactDecisionErrorV1,
    };

    #[test]
    fn inline_i64_prepares_integer_for_missing_or_unknown_destination() {
        for existing in [None, Some(&MirType::Unknown)] {
            let prepared = PreparedResolvedDirectCallIntegerTypeV1::prepare(
                TrivialRepresentationV1::InlineI64,
                existing,
            )
            .unwrap();
            assert_eq!(
                prepared.publication(),
                &PreparedTypeFactPublicationV1::Publish(MirType::Integer)
            );
        }
    }

    #[test]
    fn matching_integer_is_idempotent_and_concrete_conflict_rejects() {
        let idempotent = PreparedResolvedDirectCallIntegerTypeV1::prepare(
            TrivialRepresentationV1::InlineI64,
            Some(&MirType::Integer),
        )
        .unwrap();
        assert_eq!(
            idempotent.publication(),
            &PreparedTypeFactPublicationV1::Idempotent(MirType::Integer)
        );
        assert_eq!(
            PreparedResolvedDirectCallIntegerTypeV1::prepare(
                TrivialRepresentationV1::InlineI64,
                Some(&MirType::Bool),
            ),
            Err(ResolvedDirectCallIntegerTypeErrorV1::FactDecision(
                TypeFactDecisionErrorV1::ConcreteFactConflict {
                    existing: MirType::Bool,
                    proposed: MirType::Integer,
                }
            ))
        );
    }

    #[test]
    fn no_other_trivial_representation_is_a_direct_call_integer_result() {
        for representation in [
            TrivialRepresentationV1::InlineBool,
            TrivialRepresentationV1::InlineF64,
            TrivialRepresentationV1::ExplicitVoidValue,
            TrivialRepresentationV1::NullSentinel,
        ] {
            assert_eq!(
                PreparedResolvedDirectCallIntegerTypeV1::prepare(representation, None),
                Err(
                    ResolvedDirectCallIntegerTypeErrorV1::UnsupportedResultRepresentation {
                        actual: representation,
                    }
                )
            );
        }
    }
}
