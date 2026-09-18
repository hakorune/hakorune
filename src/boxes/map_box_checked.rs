//! Non-NyashBox Map lifecycle storage. No Clone, host publication or value carrier.
//! Callers supply source-authorized payloads; this owner never issues a Home.
//! ABI placement/disposal and compiler activation are separate consumers.
use super::table::MapTable;
use crate::boxes::map_key_domain::MapKeyDomain;
use std::sync::Mutex;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MapEndError {
    ProfileMismatch,
    StorageUnavailable,
    InvalidIdentity,
}

/// A physical obligation, not a NyashBox. End consumes the attempt on either outcome.
pub trait CanonicalMapResidence: Send + Sync {
    fn end(self: Box<Self>) -> Result<(), MapEndError>;
}

/// One slot payload. Trivial values carry no child-Home obligation. Owned
/// text keeps its own bytes — never an interned or shared handle. An empty
/// array entry owns the empty-array meaning itself: no host handle or
/// backing object is minted, so a read lane materializes a fresh ArrayBox
/// on projection. A borrowed handle stores a non-consuming snapshot — the
/// map never owns the target, end is a no-op, and a scalar read Faults
/// rather than conflating handle bits with `I64`. Residences remain
/// non-Clone and are ended exactly once.
pub enum CheckedMapPayload {
    I64(i64),
    Bool(bool),
    Text(Box<str>),
    EmptyArray,
    BorrowedHandle(i64),
    Residence(Box<dyn CanonicalMapResidence>),
}
impl CheckedMapPayload {
    fn end(self) -> Result<(), MapEndError> {
        match self {
            Self::I64(_) | Self::Bool(_) | Self::Text(_) | Self::EmptyArray
            | Self::BorrowedHandle(_) => Ok(()),
            Self::Residence(value) => value.end(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CheckedMapError {
    InvalidState,
    StorageUnavailable,
    CapacityUnavailable,
    OrderExhausted,
    ProjectionUnavailable,
}

/// Bounded scalar-read outcome for the checked lane. `Missing` is a Normal
/// outcome (the lane contract maps it to `0`); `NonScalar` means an entry
/// exists but is not an `I64` payload — surfaced as a Fault by the ABI.
pub enum CheckedMapI64Read {
    Missing,
    Value(i64),
    NonScalar,
}

#[must_use = "a rejected candidate still belongs to its prior owner"]
pub struct MapInstallFailure {
    pub error: CheckedMapError,
    pub candidate: CheckedMapPayload,
}

#[must_use = "consume the detached outcome even when no old entry existed"]
/// Caller must consume this outcome; dropping it is not semantic end. The
/// eventual opaque ABI rejects disposal while its detached outcome is Ready.
pub struct DetachedMapEntry(Option<CheckedMapPayload>);
impl DetachedMapEntry {
    pub fn end(self) -> Result<(), MapEndError> {
        match self.0 {
            Some(value) => value.end(),
            None => Ok(()),
        }
    }
}

/// Finite physical failures; the kernel translates them to its FaultFrame.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct MapEndReport {
    pub first: Option<MapEndError>,
    pub suppressed: [Option<MapEndError>; 8],
    pub suppressed_count: usize,
}
impl MapEndReport {
    fn record(&mut self, error: MapEndError) {
        if self.first.is_none() {
            self.first = Some(error);
        } else {
            if self.suppressed_count < self.suppressed.len() {
                self.suppressed[self.suppressed_count] = Some(error);
            }
            self.suppressed_count = self.suppressed_count.saturating_add(1);
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Phase {
    Unissued,
    Live,
    Ending,
    Ended,
}
struct Entry {
    order: u64,
    payload: CheckedMapPayload,
}
struct State {
    phase: Phase,
    next_order: u64,
    entries: MapTable<Entry>,
    // Empty while Live. Capacity is reserved before install, so end can move
    // every entry out and sort without allocating or duplicating a payload.
    end_buffer: Vec<Entry>,
}

/// Runtime placement owner must end every acquired lifetime, then require
/// disposable state before destruction. Drop does not run semantic cleanup.
/// Abandoning Live/Ending storage violates that caller protocol.
pub struct CheckedMap {
    state: Mutex<State>,
}
impl CheckedMap {
    pub fn unissued() -> Self {
        Self {
            state: Mutex::new(State {
                phase: Phase::Unissued,
                next_order: 0,
                entries: MapTable::new(),
                end_buffer: Vec::new(),
            }),
        }
    }

    /// Native bookkeeping exists before semantic acquisition Normal.
    pub fn acquire(&self) -> Result<(), CheckedMapError> {
        let mut state = self
            .state
            .lock()
            .map_err(|_| CheckedMapError::StorageUnavailable)?;
        if state.phase != Phase::Unissued {
            return Err(CheckedMapError::InvalidState);
        }
        state.phase = Phase::Live;
        Ok(())
    }

    /// Inspect lifetime even after poison; mutation still reports unavailable.
    pub fn require_live(&self) -> Result<(), CheckedMapError> {
        let state = self
            .state
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if state.phase == Phase::Live {
            Ok(())
        } else {
            Err(CheckedMapError::InvalidState)
        }
    }

    /// The key is already prepared. Failure returns the untouched candidate.
    pub fn install(
        &self,
        key: MapKeyDomain,
        candidate: CheckedMapPayload,
    ) -> Result<DetachedMapEntry, MapInstallFailure> {
        let mut state = match self.state.lock() {
            Ok(state) => state,
            Err(_) => {
                return Err(MapInstallFailure {
                    error: CheckedMapError::StorageUnavailable,
                    candidate,
                })
            }
        };
        let ready = (|| {
            if state.phase != Phase::Live {
                return Err(CheckedMapError::InvalidState);
            }
            let next = state
                .next_order
                .checked_add(1)
                .ok_or(CheckedMapError::OrderExhausted)?;
            let extra = usize::from(!state.entries.contains_key(&key));
            let count = state
                .entries
                .len()
                .checked_add(extra)
                .ok_or(CheckedMapError::CapacityUnavailable)?;
            state
                .entries
                .try_reserve(extra)
                .map_err(|_| CheckedMapError::CapacityUnavailable)?;
            state
                .end_buffer
                .try_reserve(count)
                .map_err(|_| CheckedMapError::CapacityUnavailable)?;
            Ok(next)
        })();
        let next = match ready {
            Ok(next) => next,
            Err(error) => return Err(MapInstallFailure { error, candidate }),
        };
        // Reservation/key preparation finished. No hooks or fallible work from
        // the commit through returned outcome publication.
        let entry = Entry {
            order: state.next_order,
            payload: candidate,
        };
        let old = state.entries.insert(key, entry);
        state.next_order = next;
        drop(state);
        Ok(DetachedMapEntry(old.map(|entry| entry.payload)))
    }

    /// No self-contained native projection of an Owned residence is authorized.
    /// Missing is distinct from refusal; this API publishes no clone or handle.
    pub fn observe_native(
        &self,
        key: &MapKeyDomain,
    ) -> Result<Option<Box<dyn crate::box_trait::NyashBox>>, CheckedMapError> {
        let state = self
            .state
            .lock()
            .map_err(|_| CheckedMapError::StorageUnavailable)?;
        if state.phase != Phase::Live {
            return Err(CheckedMapError::InvalidState);
        }
        if state.entries.contains_key(key) {
            Err(CheckedMapError::ProjectionUnavailable)
        } else {
            Ok(None)
        }
    }

    /// Bounded scalar read for the checked lane. Read-only: the lease and
    /// entry order are untouched, so borrowed callers can read without
    /// owning the map. `Missing` and `NonScalar` are distinct outcomes —
    /// the ABI maps them to `0`-Normal and Fault respectively.
    pub fn read_i64(&self, key: &MapKeyDomain) -> Result<CheckedMapI64Read, CheckedMapError> {
        let state = self
            .state
            .lock()
            .map_err(|_| CheckedMapError::StorageUnavailable)?;
        if state.phase != Phase::Live {
            return Err(CheckedMapError::InvalidState);
        }
        Ok(match state.entries.get(key) {
            None => CheckedMapI64Read::Missing,
            Some(entry) => match &entry.payload {
                CheckedMapPayload::I64(value) => CheckedMapI64Read::Value(*value),
                _ => CheckedMapI64Read::NonScalar,
            },
        })
    }

    /// Both returned outcomes consume the end attempt. No storage lock spans end.
    pub fn end(&self) -> Result<MapEndReport, CheckedMapError> {
        let mut report = MapEndReport::default();
        let mut entries = {
            let mut state = self.state.lock().unwrap_or_else(|poison| {
                report.record(MapEndError::StorageUnavailable);
                poison.into_inner()
            });
            if state.phase != Phase::Live {
                return Err(CheckedMapError::InvalidState);
            }
            state.phase = Phase::Ending;
            let mut entries = std::mem::take(&mut state.end_buffer);
            entries.extend(state.entries.drain().map(|(_, entry)| entry));
            entries
        };
        // Unstable sort allocates no scratch storage; orders are unique.
        entries.sort_unstable_by(|a, b| b.order.cmp(&a.order));
        for entry in entries {
            if let Err(error) = entry.payload.end() {
                report.record(error);
            }
        }
        self.state
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .phase = Phase::Ended;
        Ok(report)
    }

    /// Placement owner calls this before native destruction. This does not end a Home.
    pub fn require_disposable(&self) -> Result<(), CheckedMapError> {
        let state = self
            .state
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        match state.phase {
            Phase::Unissued | Phase::Ended => Ok(()),
            Phase::Live | Phase::Ending => Err(CheckedMapError::InvalidState),
        }
    }
}

#[cfg(test)]
#[path = "map_box_checked_tests.rs"]
mod tests;
