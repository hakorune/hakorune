//! Disconnected exact-type preparation for sealed static `u16` loads.
//!
//! STATICLOAD0-S0 observes the existing MIR-owned static plan representation.
//! It owns no Builder state, index lowering, instruction emission, or commit.

use hakorune_mir_builder::lowering_facts::{
    PreparedTypeFactPublicationV1, TypeFactDecisionErrorV1, TypeFactDecisionV1,
};

use crate::mir::builder::type_context::TypeContext;
use crate::mir::function::StaticDataPlan;
use crate::mir::{MirType, ValueId};

/// Prepared Integer publication for one future successful static `u16` load.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct PreparedStaticU16LoadTypeV1 {
    publication: PreparedTypeFactPublicationV1,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) enum StaticU16LoadTypeErrorV1 {
    UnsupportedElement {
        source_name: String,
        element: String,
    },
    FactDecision(TypeFactDecisionErrorV1),
}

impl std::fmt::Display for StaticU16LoadTypeErrorV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnsupportedElement {
                source_name,
                element,
            } => write!(
                formatter,
                "[static-const/load-unsupported-element] {source_name} element={element}"
            ),
            Self::FactDecision(error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for StaticU16LoadTypeErrorV1 {}

impl PreparedStaticU16LoadTypeV1 {
    /// Prepares the exact Integer result fact for the existing sealed `u16`
    /// static-data representation without mutating a fact store.
    pub(super) fn prepare(
        plan: &StaticDataPlan,
        existing_destination: Option<&MirType>,
    ) -> Result<Self, StaticU16LoadTypeErrorV1> {
        if plan.element != "u16" {
            return Err(StaticU16LoadTypeErrorV1::UnsupportedElement {
                source_name: plan.source_name.clone(),
                element: plan.element.clone(),
            });
        }
        let publication =
            TypeFactDecisionV1::prepare(existing_destination, Some(&MirType::Integer))
                .map_err(StaticU16LoadTypeErrorV1::FactDecision)?;
        Ok(Self { publication })
    }

    /// Commits only the prepared exact transient type after successful load
    /// emission. Finalized function metadata remains a later snapshot owner.
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
    use super::{PreparedStaticU16LoadTypeV1, StaticU16LoadTypeErrorV1};
    use crate::mir::function::StaticDataPlan;
    use crate::mir::MirType;
    use hakorune_mir_builder::lowering_facts::{
        PreparedTypeFactPublicationV1, TypeFactDecisionErrorV1,
    };

    fn u16_plan() -> StaticDataPlan {
        StaticDataPlan {
            source_name: "SIZE_CLASS".to_string(),
            symbol: ".hako.static.SIZE_CLASS".to_string(),
            element: "u16".to_string(),
            align: 2,
            linkage: "private".to_string(),
            unnamed_addr: true,
            values: vec![8, 16],
        }
    }

    #[test]
    fn sealed_u16_plan_prepares_integer_for_missing_or_unknown_destination() {
        for existing in [None, Some(&MirType::Unknown)] {
            let prepared = PreparedStaticU16LoadTypeV1::prepare(&u16_plan(), existing).unwrap();
            assert_eq!(
                prepared.publication(),
                &PreparedTypeFactPublicationV1::Publish(MirType::Integer)
            );
        }
    }

    #[test]
    fn matching_integer_is_idempotent_and_conflict_rejects_before_any_commit() {
        let plan = u16_plan();
        let idempotent =
            PreparedStaticU16LoadTypeV1::prepare(&plan, Some(&MirType::Integer)).unwrap();
        assert_eq!(
            idempotent.publication(),
            &PreparedTypeFactPublicationV1::Idempotent(MirType::Integer)
        );
        assert_eq!(
            PreparedStaticU16LoadTypeV1::prepare(&plan, Some(&MirType::String)),
            Err(StaticU16LoadTypeErrorV1::FactDecision(
                TypeFactDecisionErrorV1::ConcreteFactConflict {
                    existing: MirType::String,
                    proposed: MirType::Integer,
                }
            ))
        );
    }

    #[test]
    fn unsupported_plan_element_is_not_a_result_type_fallback() {
        let mut plan = u16_plan();
        plan.element = "u8".to_string();
        assert_eq!(
            PreparedStaticU16LoadTypeV1::prepare(&plan, None),
            Err(StaticU16LoadTypeErrorV1::UnsupportedElement {
                source_name: "SIZE_CLASS".to_string(),
                element: "u8".to_string(),
            })
        );
    }
}
