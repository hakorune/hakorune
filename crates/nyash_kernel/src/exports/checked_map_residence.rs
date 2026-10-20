//! Private SafeMutex residence for a source-authorized checked Map transfer.
//! Preparation observes identity only: rejecting an install drops this wrapper
//! without reclaiming the still-caller-owned payload. End consumes the attempt.
use super::typed_object::{DynamicFieldEndDescriptorV1, DynamicFieldStoredV1, DynamicFieldValueV1};
use super::typed_object_store_backend::{
    reclaim_checked_indexed, validate_checked_indexed_identity, CheckedStorageError,
    TypedObjectStoreBackend,
};
use nyash_rust::boxes::map_box::checked::{
    CanonicalMapResidence, CheckedMapPayload, MapEndContext, MapEndError,
};

struct IndexedResidence {
    handle: i64,
    type_id: i64,
    end: Option<&'static DynamicFieldEndDescriptorV1>,
}

/// No public numeric-handle constructor. Callers must retain the same source
/// obligation until install Normal; physical identity checking is not ownership.
pub(super) fn prepare(
    profile: TypedObjectStoreBackend,
    handle: i64,
    type_id: i64,
    end: Option<&'static DynamicFieldEndDescriptorV1>,
) -> Result<CheckedMapPayload, CheckedStorageError> {
    if profile != TypedObjectStoreBackend::SafeMutex {
        return Err(CheckedStorageError::ProfileMismatch);
    }
    validate_checked_indexed_identity(profile, handle, type_id)?;
    Ok(CheckedMapPayload::Residence(Box::new(IndexedResidence {
        handle,
        type_id,
        end,
    })))
}

impl CanonicalMapResidence for IndexedResidence {
    fn end(self: Box<Self>, context: &mut MapEndContext<'_>) {
        if let Some(descriptor) = self.end {
            let Some((frame, site)) = context.runtime() else {
                context.record(MapEndError::InvalidContract);
                return;
            };
            let stored = DynamicFieldStoredV1::Published {
                value: DynamicFieldValueV1 {
                    kind: 5,
                    reserved: 0,
                    value: self.handle,
                    token: 0,
                },
                end: Some(descriptor),
            };
            match unsafe { stored.end(frame, site) } {
                0 => {}
                1 => context.record(MapEndError::RecordedFault),
                _ => context.record(MapEndError::InvalidContract),
            }
            return;
        }
        if let Err(error) = reclaim_checked_indexed(
            TypedObjectStoreBackend::SafeMutex,
            self.handle,
            self.type_id,
        )
        .map_err(|error| match error {
            CheckedStorageError::ProfileMismatch => MapEndError::ProfileMismatch,
            CheckedStorageError::AllocationOrStorageUnavailable => MapEndError::StorageUnavailable,
            CheckedStorageError::InvalidLayout | CheckedStorageError::ObjectOrFieldMismatch => {
                MapEndError::InvalidIdentity
            }
        }) {
            context.record(error);
        }
    }
}

#[cfg(test)]
#[path = "checked_map_residence_tests.rs"]
mod tests;
