//! Private SafeMutex residence for a source-authorized checked Map transfer.
//! Preparation observes identity only: rejecting an install drops this wrapper
//! without reclaiming the still-caller-owned payload. End consumes the attempt.
use super::typed_object_store_backend::{
    reclaim_checked_indexed, validate_checked_indexed_identity, CheckedStorageError,
    TypedObjectStoreBackend,
};
use nyash_rust::boxes::map_box::checked::{CanonicalMapResidence, MapEndError};

struct IndexedResidence {
    handle: i64,
    type_id: i64,
}

/// No public numeric-handle constructor. Callers must retain the same source
/// obligation until install Normal; physical identity checking is not ownership.
pub(super) fn prepare(
    profile: TypedObjectStoreBackend,
    handle: i64,
    type_id: i64,
) -> Result<Box<dyn CanonicalMapResidence>, CheckedStorageError> {
    if profile != TypedObjectStoreBackend::SafeMutex {
        return Err(CheckedStorageError::ProfileMismatch);
    }
    validate_checked_indexed_identity(profile, handle, type_id)?;
    Ok(Box::new(IndexedResidence { handle, type_id }))
}

impl CanonicalMapResidence for IndexedResidence {
    fn end(self: Box<Self>) -> Result<(), MapEndError> {
        reclaim_checked_indexed(
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
        })
    }
}

#[cfg(test)]
#[path = "checked_map_residence_tests.rs"]
mod tests;
