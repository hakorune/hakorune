use crate::bid::{BidError, BidResult};
use std::sync::{PoisonError, RwLockReadGuard, RwLockWriteGuard};

#[inline]
pub fn from_rwlock_write<'a, T>(
    r: Result<RwLockWriteGuard<'a, T>, PoisonError<RwLockWriteGuard<'a, T>>>,
) -> BidResult<RwLockWriteGuard<'a, T>> {
    r.map_err(|_| BidError::PluginError)
}

#[inline]
pub fn from_rwlock_read<'a, T>(
    r: Result<RwLockReadGuard<'a, T>, PoisonError<RwLockReadGuard<'a, T>>>,
) -> BidResult<RwLockReadGuard<'a, T>> {
    r.map_err(|_| BidError::PluginError)
}
