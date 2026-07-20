//! Prepared lifecycle for one FastMem FieldLoad.
//!
//! FastMem FieldLoad intentionally differs from ordinary FieldGet: its layout
//! access-site and declared destination type are pre-emission reservations,
//! while missing-declared Integer compatibility and result origin complete only
//! after the physical MemOp succeeds. This module records that split without
//! holding Builder state or connecting a production consumer.

use crate::mir::MirType;

/// Immutable site inputs for the existing pre-emission FastMem reservation.
#[derive(Debug, Eq, PartialEq)]
pub(super) struct FastMemFieldLoadSiteReservationV1 {
    receiver_box_name: Option<String>,
    field: String,
}

/// Existing declared-type reservation behavior.
#[derive(Debug, Eq, PartialEq)]
pub(super) enum FastMemFieldLoadTypeReservationV1 {
    AbsentDeclaredType,
    Declared(MirType),
}

/// Existing missing-declared compatibility completion behavior.
#[derive(Debug, Eq, PartialEq)]
pub(super) enum FastMemFieldLoadTypeCompletionV1 {
    Inactive,
    PublishIntegerCompatibility,
}

/// Existing post-success field-result origin behavior.
#[derive(Debug, Eq, PartialEq)]
pub(super) enum FastMemFieldLoadOriginCompletionV1 {
    Absent,
    Publish(String),
}

/// One Builder-free, non-Clone description of the FieldLoad timing split.
#[derive(Debug)]
pub(super) struct PreparedFastMemFieldLoadLifecycleV1 {
    site: FastMemFieldLoadSiteReservationV1,
    type_reservation: FastMemFieldLoadTypeReservationV1,
    type_completion: FastMemFieldLoadTypeCompletionV1,
    origin_completion: FastMemFieldLoadOriginCompletionV1,
}

impl PreparedFastMemFieldLoadLifecycleV1 {
    /// Snapshots already-resolved inputs without publishing a fact or site.
    pub(super) fn prepare(
        declared_type: Option<&MirType>,
        receiver_box_name: Option<&str>,
        field: &str,
        field_result_origin: Option<&str>,
    ) -> Self {
        let (type_reservation, type_completion) = match declared_type {
            Some(ty) => (
                FastMemFieldLoadTypeReservationV1::Declared(ty.clone()),
                FastMemFieldLoadTypeCompletionV1::Inactive,
            ),
            None => (
                FastMemFieldLoadTypeReservationV1::AbsentDeclaredType,
                FastMemFieldLoadTypeCompletionV1::PublishIntegerCompatibility,
            ),
        };
        Self {
            site: FastMemFieldLoadSiteReservationV1 {
                receiver_box_name: receiver_box_name.map(str::to_string),
                field: field.to_string(),
            },
            type_reservation,
            type_completion,
            origin_completion: match field_result_origin {
                Some(origin) => FastMemFieldLoadOriginCompletionV1::Publish(origin.to_string()),
                None => FastMemFieldLoadOriginCompletionV1::Absent,
            },
        }
    }

    #[cfg(test)]
    fn site(&self) -> &FastMemFieldLoadSiteReservationV1 {
        &self.site
    }

    #[cfg(test)]
    fn type_reservation(&self) -> &FastMemFieldLoadTypeReservationV1 {
        &self.type_reservation
    }

    #[cfg(test)]
    fn type_completion(&self) -> &FastMemFieldLoadTypeCompletionV1 {
        &self.type_completion
    }

    #[cfg(test)]
    fn origin_completion(&self) -> &FastMemFieldLoadOriginCompletionV1 {
        &self.origin_completion
    }
}

#[cfg(test)]
mod tests {
    use super::{
        FastMemFieldLoadOriginCompletionV1, FastMemFieldLoadTypeCompletionV1,
        FastMemFieldLoadTypeReservationV1, PreparedFastMemFieldLoadLifecycleV1,
    };
    use crate::mir::MirType;

    #[test]
    fn declared_type_is_reserved_before_emission_without_integer_completion() {
        let prepared = PreparedFastMemFieldLoadLifecycleV1::prepare(
            Some(&MirType::Box("ArrayBox".to_string())),
            Some("OwnerBox"),
            "items",
            Some("ArrayBox"),
        );

        assert_eq!(
            prepared.site().receiver_box_name.as_deref(),
            Some("OwnerBox")
        );
        assert_eq!(prepared.site().field, "items");
        assert_eq!(
            prepared.type_reservation(),
            &FastMemFieldLoadTypeReservationV1::Declared(MirType::Box("ArrayBox".to_string()))
        );
        assert_eq!(
            prepared.type_completion(),
            &FastMemFieldLoadTypeCompletionV1::Inactive
        );
        assert_eq!(
            prepared.origin_completion(),
            &FastMemFieldLoadOriginCompletionV1::Publish("ArrayBox".to_string())
        );
    }

    #[test]
    fn missing_declared_type_defers_integer_compatibility_until_success() {
        let prepared = PreparedFastMemFieldLoadLifecycleV1::prepare(None, None, "owner_id", None);

        assert_eq!(
            prepared.type_reservation(),
            &FastMemFieldLoadTypeReservationV1::AbsentDeclaredType
        );
        assert_eq!(
            prepared.type_completion(),
            &FastMemFieldLoadTypeCompletionV1::PublishIntegerCompatibility
        );
        assert_eq!(
            prepared.origin_completion(),
            &FastMemFieldLoadOriginCompletionV1::Absent
        );
    }

    #[test]
    fn declared_unknown_remains_a_legacy_pre_emission_reservation() {
        let prepared = PreparedFastMemFieldLoadLifecycleV1::prepare(
            Some(&MirType::Unknown),
            None,
            "dynamic",
            None,
        );

        assert_eq!(
            prepared.type_reservation(),
            &FastMemFieldLoadTypeReservationV1::Declared(MirType::Unknown)
        );
        assert_eq!(
            prepared.type_completion(),
            &FastMemFieldLoadTypeCompletionV1::Inactive
        );
    }
}
