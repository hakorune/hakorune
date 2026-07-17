//! Disconnected GenericLoop carrier representation decision.
//!
//! Facts provide one closed carrier role. This module verifies one selected
//! init value and its already-published transient MIR type. It does not read
//! Builder state, allocate values/blocks, infer types, or publish metadata.

use crate::mir::builder::control_flow::plan::generic_loop::facts_types::GenericLoopCarrierRoleV1;
use crate::mir::{MirType, ValueId};

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum GenericLoopCarrierRepresentationErrorV1 {
    MissingLoopVariableValue,
    MissingTransientType { init: ValueId },
    UnknownTransientType { init: ValueId },
    NumericRepresentationMismatch { init: ValueId, actual: MirType },
}

/// Single-use preparation result for the selected GenericLoop carrier.
///
/// This product is intentionally not `Clone`: later skeleton allocation must
/// consume one decision rather than duplicate representation authorities.
#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) struct PreparedGenericLoopCarrierRepresentationV1 {
    init: ValueId,
    exact_type: MirType,
}

impl PreparedGenericLoopCarrierRepresentationV1 {
    pub(in crate::mir::builder) fn init(&self) -> ValueId {
        self.init
    }

    pub(in crate::mir::builder) fn exact_type(&self) -> &MirType {
        &self.exact_type
    }
}

/// Purely verifies the role/representation pair.
///
/// `transient_type` must be the current lowering session's existing type fact
/// for `init`. Missing and `Unknown` are rejected; no default is synthesized.
pub(in crate::mir::builder) fn prepare_generic_loop_carrier_representation_v1(
    role: GenericLoopCarrierRoleV1,
    init: Option<ValueId>,
    transient_type: Option<&MirType>,
) -> Result<PreparedGenericLoopCarrierRepresentationV1, GenericLoopCarrierRepresentationErrorV1> {
    let init = init.ok_or(GenericLoopCarrierRepresentationErrorV1::MissingLoopVariableValue)?;
    let exact_type = transient_type
        .ok_or(GenericLoopCarrierRepresentationErrorV1::MissingTransientType { init })?;
    if exact_type == &MirType::Unknown {
        return Err(GenericLoopCarrierRepresentationErrorV1::UnknownTransientType { init });
    }
    if role == GenericLoopCarrierRoleV1::NumericProgression && exact_type != &MirType::Integer {
        return Err(
            GenericLoopCarrierRepresentationErrorV1::NumericRepresentationMismatch {
                init,
                actual: exact_type.clone(),
            },
        );
    }

    Ok(PreparedGenericLoopCarrierRepresentationV1 {
        init,
        exact_type: exact_type.clone(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() -> ValueId {
        ValueId::new(7)
    }

    fn prepare(
        role: GenericLoopCarrierRoleV1,
        ty: &MirType,
    ) -> Result<PreparedGenericLoopCarrierRepresentationV1, GenericLoopCarrierRepresentationErrorV1>
    {
        prepare_generic_loop_carrier_representation_v1(role, Some(init()), Some(ty))
    }

    #[test]
    fn numeric_progression_prepares_exact_integer() {
        let prepared = prepare(
            GenericLoopCarrierRoleV1::NumericProgression,
            &MirType::Integer,
        )
        .expect("Integer numeric carrier should prepare");

        assert_eq!(prepared.init(), init());
        assert_eq!(prepared.exact_type(), &MirType::Integer);
    }

    #[test]
    fn numeric_progression_rejects_non_integer_representations() {
        for ty in [MirType::Float, MirType::Box("JsonScanner".to_string())] {
            assert_eq!(
                prepare(GenericLoopCarrierRoleV1::NumericProgression, &ty),
                Err(
                    GenericLoopCarrierRepresentationErrorV1::NumericRepresentationMismatch {
                        init: init(),
                        actual: ty,
                    }
                )
            );
        }
    }

    #[test]
    fn body_managed_state_preserves_each_exact_representation() {
        for ty in [
            MirType::Integer,
            MirType::Bool,
            MirType::String,
            MirType::Box("JsonScanner".to_string()),
        ] {
            let prepared = prepare(GenericLoopCarrierRoleV1::BodyManagedState, &ty)
                .expect("exact body-managed carrier should prepare");
            assert_eq!(prepared.init(), init());
            assert_eq!(prepared.exact_type(), &ty);
        }
    }

    #[test]
    fn missing_loop_variable_rejects_before_type_consultation() {
        assert_eq!(
            prepare_generic_loop_carrier_representation_v1(
                GenericLoopCarrierRoleV1::BodyManagedState,
                None,
                None,
            ),
            Err(GenericLoopCarrierRepresentationErrorV1::MissingLoopVariableValue)
        );
    }

    #[test]
    fn missing_transient_type_rejects() {
        assert_eq!(
            prepare_generic_loop_carrier_representation_v1(
                GenericLoopCarrierRoleV1::BodyManagedState,
                Some(init()),
                None,
            ),
            Err(GenericLoopCarrierRepresentationErrorV1::MissingTransientType { init: init() })
        );
    }

    #[test]
    fn unknown_transient_type_rejects_without_default() {
        assert_eq!(
            prepare(
                GenericLoopCarrierRoleV1::BodyManagedState,
                &MirType::Unknown,
            ),
            Err(GenericLoopCarrierRepresentationErrorV1::UnknownTransientType { init: init() })
        );
    }
}
