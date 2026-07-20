//! Disconnected exact-type preparation for `CheckExpr` accumulator Selects.
//!
//! CHECKSELECT0-S0 observes the fixed Integer accumulator pair established by
//! CONST0. It does not inspect a condition, allocate a value, emit MIR, or
//! commit a type fact.

use hakorune_mir_builder::lowering_facts::{
    PreparedTypeFactPublicationV1, TypeFactDecisionErrorV1, TypeFactDecisionV1,
};

use crate::mir::builder::type_context::TypeContext;
use crate::mir::{MirType, ValueId};

/// Prepared Integer publication for one future successful CheckExpr Select.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct PreparedCheckSelectIntegerTypeV1 {
    publication: PreparedTypeFactPublicationV1,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) enum CheckSelectIntegerTypeErrorV1 {
    FactDecision(TypeFactDecisionErrorV1),
}

impl std::fmt::Display for CheckSelectIntegerTypeErrorV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FactDecision(error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for CheckSelectIntegerTypeErrorV1 {}

impl PreparedCheckSelectIntegerTypeV1 {
    /// Prepares the fixed Integer result fact without mutating a fact store.
    pub(super) fn prepare(
        existing_destination: Option<&MirType>,
    ) -> Result<Self, CheckSelectIntegerTypeErrorV1> {
        let publication =
            TypeFactDecisionV1::prepare(existing_destination, Some(&MirType::Integer))
                .map_err(CheckSelectIntegerTypeErrorV1::FactDecision)?;
        Ok(Self { publication })
    }

    /// Commits only the prepared exact fact after the existing Select succeeds.
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
    use super::{CheckSelectIntegerTypeErrorV1, PreparedCheckSelectIntegerTypeV1};
    use crate::mir::MirType;
    use hakorune_mir_builder::lowering_facts::{
        PreparedTypeFactPublicationV1, TypeFactDecisionErrorV1,
    };

    #[test]
    fn missing_or_stored_unknown_prepares_integer_publication() {
        for existing in [None, Some(&MirType::Unknown)] {
            let prepared = PreparedCheckSelectIntegerTypeV1::prepare(existing).unwrap();
            assert_eq!(
                prepared.publication(),
                &PreparedTypeFactPublicationV1::Publish(MirType::Integer)
            );
        }
    }

    #[test]
    fn existing_integer_is_idempotent() {
        let prepared = PreparedCheckSelectIntegerTypeV1::prepare(Some(&MirType::Integer)).unwrap();
        assert_eq!(
            prepared.publication(),
            &PreparedTypeFactPublicationV1::Idempotent(MirType::Integer)
        );
    }

    #[test]
    fn conflicting_exact_destination_rejects_before_publication() {
        assert_eq!(
            PreparedCheckSelectIntegerTypeV1::prepare(Some(&MirType::String)),
            Err(CheckSelectIntegerTypeErrorV1::FactDecision(
                TypeFactDecisionErrorV1::ConcreteFactConflict {
                    existing: MirType::String,
                    proposed: MirType::Integer,
                }
            ))
        );
    }
}
