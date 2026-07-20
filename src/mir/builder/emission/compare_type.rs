//! Disconnected exact-Bool preparation for one future physical Compare receipt.
//!
//! COMPAREEMIT0-S0 fixes only `MirInstruction::Compare -> Bool`. It owns no
//! operand inference, Builder, ValueId allocation, instruction emission, or
//! production commit consumer.

use hakorune_mir_builder::lowering_facts::{
    PreparedTypeFactPublicationV1, TypeFactDecisionErrorV1, TypeFactDecisionV1,
};

use crate::mir::builder::type_context::TypeContext;
use crate::mir::{MirType, ValueId};

/// Prepared Bool fact for a future successfully emitted physical Compare.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct PreparedCanonicalCompareBoolTypeV1 {
    publication: PreparedTypeFactPublicationV1,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) enum CanonicalCompareBoolTypeErrorV1 {
    FactDecision(TypeFactDecisionErrorV1),
}

impl std::fmt::Display for CanonicalCompareBoolTypeErrorV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FactDecision(error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for CanonicalCompareBoolTypeErrorV1 {}

impl PreparedCanonicalCompareBoolTypeV1 {
    /// Prepares the fixed Compare result fact without mutating a fact store.
    pub(super) fn prepare(
        existing_destination: Option<&MirType>,
    ) -> Result<Self, CanonicalCompareBoolTypeErrorV1> {
        let publication = TypeFactDecisionV1::prepare(existing_destination, Some(&MirType::Bool))
            .map_err(CanonicalCompareBoolTypeErrorV1::FactDecision)?;
        Ok(Self { publication })
    }

    /// Commits only a Bool fact prepared before a checked Compare receipt.
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

#[cfg(test)]
mod tests {
    use super::{CanonicalCompareBoolTypeErrorV1, PreparedCanonicalCompareBoolTypeV1};
    use crate::mir::MirType;
    use hakorune_mir_builder::lowering_facts::{
        PreparedTypeFactPublicationV1, TypeFactDecisionErrorV1,
    };

    #[test]
    fn missing_or_stored_unknown_prepares_bool_publication() {
        for existing in [None, Some(&MirType::Unknown)] {
            let prepared = PreparedCanonicalCompareBoolTypeV1::prepare(existing).unwrap();
            assert_eq!(
                prepared.publication(),
                &PreparedTypeFactPublicationV1::Publish(MirType::Bool)
            );
        }
    }

    #[test]
    fn existing_bool_is_idempotent() {
        let prepared = PreparedCanonicalCompareBoolTypeV1::prepare(Some(&MirType::Bool)).unwrap();
        assert_eq!(
            prepared.publication(),
            &PreparedTypeFactPublicationV1::Idempotent(MirType::Bool)
        );
    }

    #[test]
    fn conflicting_exact_destination_rejects_before_publication() {
        assert_eq!(
            PreparedCanonicalCompareBoolTypeV1::prepare(Some(&MirType::Integer)),
            Err(CanonicalCompareBoolTypeErrorV1::FactDecision(
                TypeFactDecisionErrorV1::ConcreteFactConflict {
                    existing: MirType::Integer,
                    proposed: MirType::Bool,
                }
            ))
        );
    }
}
