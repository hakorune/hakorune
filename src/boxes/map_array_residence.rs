//! Physical owner for the bounded Map -> Array -> Map read lane.
//!
//! This module deliberately owns only arrays whose elements are checked Maps.
//! Scalar and mixed Array shapes remain outside the selected T2-alpha lane and
//! are rejected before a Map payload is committed.
use super::{
    CheckedMap, CheckedMapError, CheckedMapTextRead, CheckedMapTextViewRead, MapEndError,
    MapKeyDomain,
};
use std::sync::Arc;

/// Faults produced by an ArrayIndex against a checked Array residence.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CheckedMapArrayReadError {
    InvalidState,
    StorageUnavailable,
    Missing,
    Bounds,
    NonArray,
    NonMap,
}

/// A read-only Map view borrowed from an Array root. An owned residence keeps
/// the child live with an Arc; a borrowed residence relies on the caller's
/// co-sealed MapLocal lifetime. Neither form grants End authority.
#[derive(Clone)]
pub struct CheckedMapReadView {
    map: CheckedMapReadViewStorage,
}

#[derive(Clone)]
enum CheckedMapReadViewStorage {
    Owned(Arc<CheckedMap>),
    Borrowed(*const CheckedMap),
}

impl CheckedMapReadView {
    fn map(&self) -> &CheckedMap {
        match &self.map {
            CheckedMapReadViewStorage::Owned(map) => map.as_ref(),
            CheckedMapReadViewStorage::Borrowed(ptr) => {
                // The borrowed residence validates liveness before issuing a
                // view, and its caller-owned root remains live until the
                // synchronous next read consumes this descriptor.
                unsafe { &**ptr }
            }
        }
    }

    pub fn read_text(&self, key: &MapKeyDomain) -> Result<CheckedMapTextRead, CheckedMapError> {
        self.map().read_text(key)
    }

    pub fn read_text_view(
        &self,
        key: &MapKeyDomain,
    ) -> Result<CheckedMapTextViewRead, CheckedMapError> {
        self.map().read_text_view(key)
    }

    /// Borrowed pointer for the synchronous kernel descriptor. The parent
    /// Array residence keeps this child Map alive until the view is consumed.
    pub fn borrowed_map_ptr(&self) -> *const CheckedMap {
        match &self.map {
            CheckedMapReadViewStorage::Owned(map) => Arc::as_ptr(map),
            CheckedMapReadViewStorage::Borrowed(ptr) => *ptr,
        }
    }

    fn borrowed(map: *const CheckedMap) -> Self {
        Self {
            map: CheckedMapReadViewStorage::Borrowed(map),
        }
    }
}

// The pointer is valid only during the caller-owned synchronous read window;
// the physical ABI never stores this view beyond that window.
unsafe impl Send for CheckedMapReadView {}
unsafe impl Sync for CheckedMapReadView {}

/// Canonical physical owner for an Array stored inside a checked Map.
/// ArrayIndex returns a borrowed view; only the owner can end the root.
pub trait CanonicalMapArrayResidence: Send + Sync {
    fn read_map(&self, index: i64) -> Result<CheckedMapReadView, CheckedMapArrayReadError>;
    fn end(self: Box<Self>) -> Result<(), MapEndError>;
}

/// The first concrete T2-alpha residence. Elements and release roots are
/// kept separately so duplicate `[main, main]` references release `main` once.
pub struct OwnedMapArrayResidence {
    elements: Vec<Arc<CheckedMap>>,
    release_roots: Vec<Arc<CheckedMap>>,
}

impl OwnedMapArrayResidence {
    pub fn builder(capacity: usize) -> Result<OwnedMapArrayResidenceBuilder, MapEndError> {
        OwnedMapArrayResidenceBuilder::with_capacity(capacity)
    }

    fn release_roots(mut roots: Vec<Arc<CheckedMap>>) -> Result<(), MapEndError> {
        let mut first = None;
        while let Some(root) = roots.pop() {
            match root.end() {
                Ok(report) => {
                    if let Some(error) = report.first {
                        first.get_or_insert(error);
                    }
                }
                Err(error) => {
                    first.get_or_insert(match error {
                        CheckedMapError::StorageUnavailable => MapEndError::StorageUnavailable,
                        _ => MapEndError::InvalidIdentity,
                    });
                }
            }
            if root.require_disposable().is_err() {
                first.get_or_insert(MapEndError::InvalidIdentity);
            }
        }
        first.map_or(Ok(()), Err)
    }
}

impl CanonicalMapArrayResidence for OwnedMapArrayResidence {
    fn read_map(&self, index: i64) -> Result<CheckedMapReadView, CheckedMapArrayReadError> {
        let index = usize::try_from(index).map_err(|_| CheckedMapArrayReadError::Bounds)?;
        let map = self
            .elements
            .get(index)
            .ok_or(CheckedMapArrayReadError::Bounds)?;
        map.require_live().map_err(|error| match error {
            CheckedMapError::StorageUnavailable => CheckedMapArrayReadError::StorageUnavailable,
            _ => CheckedMapArrayReadError::InvalidState,
        })?;
        Ok(CheckedMapReadView {
            map: CheckedMapReadViewStorage::Owned(Arc::clone(map)),
        })
    }

    fn end(self: Box<Self>) -> Result<(), MapEndError> {
        Self::release_roots(self.release_roots)
    }
}

/// A caller-owned Array residence. It retains only validated child pointers;
/// the caller's MapLocal cleanup remains the sole child End owner.
pub struct BorrowedMapArrayResidence {
    elements: Box<[*const CheckedMap]>,
}

impl BorrowedMapArrayResidence {
    pub fn try_from_elements(
        elements: Vec<*const CheckedMap>,
    ) -> Result<Self, CheckedMapArrayReadError> {
        if elements.is_empty() || elements.iter().any(|element| element.is_null()) {
            return Err(CheckedMapArrayReadError::InvalidState);
        }
        Ok(Self {
            elements: elements.into_boxed_slice(),
        })
    }
}

// Child liveness is sealed by the caller's MapLocal cleanup, not by this
// residence. Pointer access is synchronous and guarded by require_live().
unsafe impl Send for BorrowedMapArrayResidence {}
unsafe impl Sync for BorrowedMapArrayResidence {}

impl CanonicalMapArrayResidence for BorrowedMapArrayResidence {
    fn read_map(&self, index: i64) -> Result<CheckedMapReadView, CheckedMapArrayReadError> {
        let index = usize::try_from(index).map_err(|_| CheckedMapArrayReadError::Bounds)?;
        let ptr = *self
            .elements
            .get(index)
            .ok_or(CheckedMapArrayReadError::Bounds)?;
        let map = unsafe { ptr.as_ref() }.ok_or(CheckedMapArrayReadError::InvalidState)?;
        map.require_live().map_err(|error| match error {
            CheckedMapError::StorageUnavailable => CheckedMapArrayReadError::StorageUnavailable,
            _ => CheckedMapArrayReadError::InvalidState,
        })?;
        Ok(CheckedMapReadView::borrowed(ptr))
    }

    fn end(self: Box<Self>) -> Result<(), MapEndError> {
        // The borrowed roots are ended by the caller's MapLocal cleanup.
        drop(self);
        Ok(())
    }
}

/// Stages owned Map roots before the Array payload becomes a Map candidate.
/// A failed append immediately releases the already acquired prefix in reverse
/// order; an unaccepted candidate remains caller-owned.
pub struct OwnedMapArrayResidenceBuilder {
    elements: Vec<Arc<CheckedMap>>,
    release_roots: Vec<Arc<CheckedMap>>,
}

impl OwnedMapArrayResidenceBuilder {
    fn after_failure(&mut self, error: MapEndError) -> MapEndError {
        self.release_prefix().err().unwrap_or(error)
    }

    pub fn with_capacity(capacity: usize) -> Result<Self, MapEndError> {
        let mut elements = Vec::new();
        elements
            .try_reserve(capacity)
            .map_err(|_| MapEndError::StorageUnavailable)?;
        let mut release_roots = Vec::new();
        release_roots
            .try_reserve(capacity)
            .map_err(|_| MapEndError::StorageUnavailable)?;
        Ok(Self {
            elements,
            release_roots,
        })
    }

    pub fn push_map(&mut self, map: Arc<CheckedMap>) -> Result<(), MapEndError> {
        if let Err(error) = map.require_live() {
            let error = match error {
                CheckedMapError::StorageUnavailable => MapEndError::StorageUnavailable,
                _ => MapEndError::InvalidIdentity,
            };
            return Err(self.after_failure(error));
        }
        if let Err(error) = self.elements.try_reserve(1) {
            let _ = error;
            return Err(self.after_failure(MapEndError::StorageUnavailable));
        }
        self.elements.push(Arc::clone(&map));
        if !self
            .release_roots
            .iter()
            .any(|root| Arc::ptr_eq(root, &map))
        {
            if let Err(error) = self.release_roots.try_reserve(1) {
                let _ = error;
                let _ = self.elements.pop();
                return Err(self.after_failure(MapEndError::StorageUnavailable));
            }
            self.release_roots.push(map);
        }
        Ok(())
    }

    pub fn finish(mut self) -> OwnedMapArrayResidence {
        OwnedMapArrayResidence {
            elements: std::mem::take(&mut self.elements),
            release_roots: std::mem::take(&mut self.release_roots),
        }
    }

    fn release_prefix(&mut self) -> Result<(), MapEndError> {
        let roots = std::mem::take(&mut self.release_roots);
        self.elements.clear();
        OwnedMapArrayResidence::release_roots(roots)
    }
}

impl Drop for OwnedMapArrayResidenceBuilder {
    fn drop(&mut self) {
        // A builder that is abandoned has not transferred its prefix to a
        // Map. Best-effort cleanup preserves the same reverse-root contract;
        // explicit push errors already report the first failure to the caller.
        let _ = self.release_prefix();
    }
}
