//! Neutral one-shot lease owner for strict DynamicV2 carriers.
//!
//! A lease token is deliberately distinct from a reusable HostHandle.  The
//! strict leaf publishes a token only after it has created the result handle;
//! the token owner is the only code allowed to consume the carrier and drop
//! that handle at the End cutpoint.

use std::collections::HashMap;
use std::num::NonZeroU64;
use std::sync::{Mutex, OnceLock};
use std::sync::atomic::{AtomicU64, Ordering};

use super::host_handles;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LeaseIssueRejectV1 {
    InvalidHandle,
    Exhausted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LeaseConsumeRejectV1 {
    UnknownOrAlreadyConsumed,
    HandleMissing,
}

#[derive(Debug, PartialEq, Eq)]
pub struct EndAuthorizedTextV1 {
    handle: u64,
    token: NonZeroU64,
}

impl EndAuthorizedTextV1 {
    pub fn handle(&self) -> u64 {
        self.handle
    }

    pub fn token(&self) -> NonZeroU64 {
        self.token
    }
}

static NEXT_TOKEN: AtomicU64 = AtomicU64::new(1);
static LEASES: OnceLock<Mutex<HashMap<NonZeroU64, u64>>> = OnceLock::new();
const TOKEN_BRAND: u64 = 1 << 63;

fn leases() -> &'static Mutex<HashMap<NonZeroU64, u64>> {
    LEASES.get_or_init(|| Mutex::new(HashMap::new()))
}

fn next_token() -> Result<NonZeroU64, LeaseIssueRejectV1> {
    let raw = NEXT_TOKEN
        .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |value| {
            value
                .checked_add(1)
                .filter(|next| *next < TOKEN_BRAND)
        })
        .map_err(|_| LeaseIssueRejectV1::Exhausted)?;
    NonZeroU64::new(TOKEN_BRAND | raw).ok_or(LeaseIssueRejectV1::Exhausted)
}

/// Admit one live text handle into the one-shot End-authorized lane.
pub fn issue_end_authorized(handle: u64) -> Result<NonZeroU64, LeaseIssueRejectV1> {
    if host_handles::with_str_handle_ready(handle, |_| ()).is_none() {
        return Err(LeaseIssueRejectV1::InvalidHandle);
    }
    let token = next_token()?;
    let mut table = leases().lock().expect("dynamic v2 lease mutex poisoned");
    if table.insert(token, handle).is_some() {
        return Err(LeaseIssueRejectV1::Exhausted);
    }
    Ok(token)
}

/// Create a fresh text result and admit its End lease as one aggregate.  The
/// rollback stays inside this owner so a strict leaf never calls `drop_handle`
/// or publishes a handle without its matching lease.
pub fn publish_end_authorized_text(
    text: impl Into<String>,
) -> Result<EndAuthorizedTextV1, LeaseIssueRejectV1> {
    let handle = host_handles::to_handle_text(text);
    let token = match issue_end_authorized(handle) {
        Ok(token) => token,
        Err(error) => {
            host_handles::drop_handle(handle);
            return Err(error);
        }
    };
    Ok(EndAuthorizedTextV1 { handle, token })
}

/// Consume a lease exactly once and release its associated result handle.
pub fn consume_end_authorized(token: NonZeroU64) -> Result<(), LeaseConsumeRejectV1> {
    let handle = leases()
        .lock()
        .expect("dynamic v2 lease mutex poisoned")
        .remove(&token)
        .ok_or(LeaseConsumeRejectV1::UnknownOrAlreadyConsumed)?;
    if host_handles::with_str_handle_ready(handle, |_| ()).is_none() {
        return Err(LeaseConsumeRejectV1::HandleMissing);
    }
    host_handles::drop_handle(handle);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lease_is_distinct_and_one_shot() {
        let handle = host_handles::to_handle_text("lease");
        let token = issue_end_authorized(handle).expect("live text handle admits");
        assert_ne!(token.get(), handle);
        assert_eq!(consume_end_authorized(token), Ok(()));
        assert_eq!(consume_end_authorized(token), Err(LeaseConsumeRejectV1::UnknownOrAlreadyConsumed));
    }

    #[test]
    fn foreign_and_stale_inputs_reject() {
        let foreign = NonZeroU64::new(u64::MAX).expect("nonzero");
        assert_eq!(
            consume_end_authorized(foreign),
            Err(LeaseConsumeRejectV1::UnknownOrAlreadyConsumed)
        );
        assert_eq!(
            issue_end_authorized(u64::MAX),
            Err(LeaseIssueRejectV1::InvalidHandle)
        );
    }
}
