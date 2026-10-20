//! One physical operation order for an already-proven complete object End.
//! Callers retain source/destruction/availability authority and attach their
//! own original release subjects. This projection creates no Home or receipt.
use super::{OwnedFieldChildKindV1, OwnedFieldChildV1};
use crate::mir::instruction::{InvokeOperation, ObjectEndStepV1};
use crate::mir::ValueId;
use hakorune_mir_defs::{CanonicalFieldRefV1, CanonicalObjectIdV1};

/// One ordered projection for ordinary and conditional complete End. The
/// conditional owner retains this body, never reconstructing it from layout.
pub(super) fn steps_v1(
    children: &[OwnedFieldChildV1],
) -> impl Iterator<Item = ObjectEndStepV1> + '_ {
    children
        .iter()
        .rev()
        .map(|child| match child.kind {
            OwnedFieldChildKindV1::Residence(mode) => ObjectEndStepV1::FieldResidence { mode, field: child.field },
            OwnedFieldChildKindV1::Object(object) => ObjectEndStepV1::ObjectField {
                field: child.field,
                child: object,
            },
        })
        .chain(std::iter::once(ObjectEndStepV1::ReleaseOuter))
}

pub(super) fn operations_v1(
    object: CanonicalObjectIdV1,
    nullable: bool,
    children: &[OwnedFieldChildV1],
    value: ValueId,
) -> impl Iterator<Item = (Option<CanonicalFieldRefV1>, InvokeOperation)> + '_ {
    steps_v1(children).map(move |step| match step {
        ObjectEndStepV1::FieldResidence { mode, field } => (
            Some(field),
            InvokeOperation::OwnedFieldResidenceRelease { mode, field, base: value },
        ),
        ObjectEndStepV1::ObjectField { field, child } => (
            Some(field),
            InvokeOperation::OwnedObjectFieldRelease {
                field,
                base: value,
                child,
            },
        ),
        ObjectEndStepV1::ReleaseOuter => (
            None,
            if nullable {
                InvokeOperation::HomeReleaseIfLive { object, value }
            } else {
                InvokeOperation::HomeRelease { object, value }
            },
        ),
    })
}
