//! Disconnected exact-type preparation for canonical `Const` emission.
//!
//! CONST0-S0 owns only the representation law:
//! `ConstValue -> exact MirType -> TypeFactDecisionV1::prepare`.  It owns no
//! Builder state, ValueId, instruction emission, or fact-store commit.

use hakorune_mir_builder::lowering_facts::{
    PreparedTypeFactPublicationV1, TypeFactDecisionErrorV1, TypeFactDecisionV1,
};

use crate::mir::{ConstValue, MirType};

/// A prepared exact type fact for one future successful canonical Const.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PreparedCanonicalConstTypeV1 {
    publication: PreparedTypeFactPublicationV1,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum CanonicalConstTypeErrorV1 {
    FactDecision(TypeFactDecisionErrorV1),
}

impl std::fmt::Display for CanonicalConstTypeErrorV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FactDecision(error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for CanonicalConstTypeErrorV1 {}

impl PreparedCanonicalConstTypeV1 {
    /// Prepares the canonical Const result fact without mutating a fact store.
    pub(crate) fn prepare(
        value: &ConstValue,
        existing_destination: Option<&MirType>,
    ) -> Result<Self, CanonicalConstTypeErrorV1> {
        let candidate = exact_type_for_const(value);
        let publication = TypeFactDecisionV1::prepare(existing_destination, Some(&candidate))
            .map_err(CanonicalConstTypeErrorV1::FactDecision)?;
        Ok(Self { publication })
    }

    #[cfg(test)]
    fn publication(&self) -> &PreparedTypeFactPublicationV1 {
        &self.publication
    }
}

/// The exact result representation of the six canonical MIR constant variants.
pub(crate) fn exact_type_for_const(value: &ConstValue) -> MirType {
    match value {
        ConstValue::Integer(_) => MirType::Integer,
        ConstValue::Float(_) => MirType::Float,
        ConstValue::Bool(_) => MirType::Bool,
        ConstValue::String(_) => MirType::String,
        ConstValue::Null | ConstValue::Void => MirType::Void,
    }
}

#[cfg(test)]
mod tests {
    use super::{exact_type_for_const, CanonicalConstTypeErrorV1, PreparedCanonicalConstTypeV1};
    use crate::mir::{ConstValue, MirType};
    use hakorune_mir_builder::lowering_facts::{
        PreparedTypeFactPublicationV1, TypeFactDecisionErrorV1,
    };

    #[test]
    fn every_canonical_const_variant_has_one_exact_type() {
        let cases = [
            (ConstValue::Integer(7), MirType::Integer),
            (ConstValue::Float(1.5), MirType::Float),
            (ConstValue::Bool(true), MirType::Bool),
            (ConstValue::String("text".to_string()), MirType::String),
            (ConstValue::Null, MirType::Void),
            (ConstValue::Void, MirType::Void),
        ];

        for (value, expected) in cases {
            assert_eq!(exact_type_for_const(&value), expected);
            assert_ne!(exact_type_for_const(&value), MirType::Unknown);
        }
    }

    #[test]
    fn missing_or_stored_unknown_destination_prepares_exact_publication() {
        for existing in [None, Some(&MirType::Unknown)] {
            let prepared =
                PreparedCanonicalConstTypeV1::prepare(&ConstValue::Integer(7), existing).unwrap();
            assert_eq!(
                prepared.publication(),
                &PreparedTypeFactPublicationV1::Publish(MirType::Integer)
            );
        }
    }

    #[test]
    fn matching_exact_destination_is_idempotent() {
        let prepared =
            PreparedCanonicalConstTypeV1::prepare(&ConstValue::Void, Some(&MirType::Void)).unwrap();
        assert_eq!(
            prepared.publication(),
            &PreparedTypeFactPublicationV1::Idempotent(MirType::Void)
        );
    }

    #[test]
    fn conflicting_exact_destination_rejects_without_prepared_publication() {
        assert_eq!(
            PreparedCanonicalConstTypeV1::prepare(
                &ConstValue::String("text".to_string()),
                Some(&MirType::Integer),
            ),
            Err(CanonicalConstTypeErrorV1::FactDecision(
                TypeFactDecisionErrorV1::ConcreteFactConflict {
                    existing: MirType::Integer,
                    proposed: MirType::String,
                }
            ))
        );
    }
}
