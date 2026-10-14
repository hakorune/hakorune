//! One physical operation order for an already-proven complete object End.
//! Callers retain source/destruction/availability authority and attach their
//! own original release subjects. This projection creates no Home or receipt.
use super::{OwnedFieldChildKindV1, OwnedFieldChildV1};
use crate::mir::instruction::InvokeOperation;
use crate::mir::ValueId;
use hakorune_mir_defs::{CanonicalFieldRefV1, CanonicalObjectIdV1};

pub(super) fn operations_v1(
    object: CanonicalObjectIdV1,
    nullable: bool,
    children: &[OwnedFieldChildV1],
    value: ValueId,
) -> impl Iterator<Item = (Option<CanonicalFieldRefV1>, InvokeOperation)> + '_ {
    let release = if nullable {
        InvokeOperation::HomeReleaseIfLive { object, value }
    } else {
        InvokeOperation::HomeRelease { object, value }
    };
    children.iter().rev().map(move |child| {
        let operation = match child.kind {
            OwnedFieldChildKindV1::Array => InvokeOperation::OwnedFieldResidenceRelease {
                field: child.field, base: value,
            },
            OwnedFieldChildKindV1::Object(object) => InvokeOperation::OwnedObjectFieldRelease {
                field: child.field, base: value, child: object,
            },
        };
        (Some(child.field), operation)
    }).chain(std::iter::once((None, release)))
}
