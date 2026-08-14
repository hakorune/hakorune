//! Neutral one-shot lease owner for strict DynamicV2 carriers.
//!
//! A lease token is deliberately distinct from a reusable HostHandle.  The
//! strict leaf publishes a token only after it has created the result handle;
//! the token owner is the only code allowed to consume the carrier and drop
//! that handle at the End cutpoint.

use std::collections::hash_map::{Entry, HashMap};
use std::num::NonZeroU64;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Mutex, OnceLock};

use super::host_handles;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LeaseIssueRejectV1 {
    InvalidHandle,
    Exhausted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LeaseConsumeRejectV1 {
    UnknownOrAlreadyConsumed,
    StaleHandleIdentity,
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
static LEASES: OnceLock<Mutex<HashMap<NonZeroU64, host_handles::HostHandleLeaseIdentityV1>>> =
    OnceLock::new();
const TOKEN_BRAND: u64 = 1 << 63;

fn leases() -> &'static Mutex<HashMap<NonZeroU64, host_handles::HostHandleLeaseIdentityV1>> {
    LEASES.get_or_init(|| Mutex::new(HashMap::new()))
}

fn next_token() -> Result<NonZeroU64, LeaseIssueRejectV1> {
    let raw = NEXT_TOKEN
        .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |value| {
            value.checked_add(1).filter(|next| *next < TOKEN_BRAND)
        })
        .map_err(|_| LeaseIssueRejectV1::Exhausted)?;
    NonZeroU64::new(TOKEN_BRAND | raw).ok_or(LeaseIssueRejectV1::Exhausted)
}

fn insert_lease_in_table(
    table: &mut HashMap<NonZeroU64, host_handles::HostHandleLeaseIdentityV1>,
    token: NonZeroU64,
    identity: host_handles::HostHandleLeaseIdentityV1,
) -> Result<(), (LeaseIssueRejectV1, host_handles::HostHandleLeaseIdentityV1)> {
    match table.entry(token) {
        Entry::Vacant(entry) => {
            entry.insert(identity);
            Ok(())
        }
        Entry::Occupied(_) => Err((LeaseIssueRejectV1::Exhausted, identity)),
    }
}

fn insert_lease_identity(
    identity: host_handles::HostHandleLeaseIdentityV1,
) -> Result<NonZeroU64, (LeaseIssueRejectV1, host_handles::HostHandleLeaseIdentityV1)> {
    let token = match next_token() {
        Ok(token) => token,
        Err(error) => return Err((error, identity)),
    };
    let mut table = leases().lock().expect("dynamic v2 lease mutex poisoned");
    insert_lease_in_table(&mut table, token, identity).map(|()| token)
}

/// Admit one captured live text handle into the one-shot End-authorized lane.
fn issue_end_authorized(handle: u64) -> Result<NonZeroU64, LeaseIssueRejectV1> {
    let identity = host_handles::capture_text_lease_identity(handle)
        .ok_or(LeaseIssueRejectV1::InvalidHandle)?;
    match insert_lease_identity(identity) {
        Ok(token) => Ok(token),
        Err((error, identity)) => {
            let _ = host_handles::drop_if_lease_identity_matches(identity);
            Err(error)
        }
    }
}

/// Create a fresh text result and admit its End lease as one aggregate.  The
/// rollback stays inside this owner so a strict leaf never calls `drop_handle`
/// or publishes a handle without its matching lease.
pub fn publish_end_authorized_text(
    text: impl Into<String>,
) -> Result<EndAuthorizedTextV1, LeaseIssueRejectV1> {
    let (handle, identity) = host_handles::to_handle_text_with_lease_identity(text);
    let token = match insert_lease_identity(identity) {
        Ok(token) => token,
        Err((error, identity)) => {
            let _ = host_handles::drop_if_lease_identity_matches(identity);
            return Err(error);
        }
    };
    Ok(EndAuthorizedTextV1 { handle, token })
}

/// Consume a lease exactly once and release its associated result handle.
pub fn consume_end_authorized(token: NonZeroU64) -> Result<(), LeaseConsumeRejectV1> {
    let identity = leases()
        .lock()
        .expect("dynamic v2 lease mutex poisoned")
        .remove(&token)
        .ok_or(LeaseConsumeRejectV1::UnknownOrAlreadyConsumed)?;
    if !host_handles::drop_if_lease_identity_matches(identity) {
        return Err(LeaseConsumeRejectV1::StaleHandleIdentity);
    }
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
        assert_eq!(
            consume_end_authorized(token),
            Err(LeaseConsumeRejectV1::UnknownOrAlreadyConsumed)
        );
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

    #[test]
    fn token_collision_preserves_existing_lease() {
        let first = host_handles::to_handle_text("first");
        let second = host_handles::to_handle_text("second");
        let first_identity =
            host_handles::capture_text_lease_identity(first).expect("first identity");
        let second_identity =
            host_handles::capture_text_lease_identity(second).expect("second identity");
        let token = NonZeroU64::new(TOKEN_BRAND | 777).expect("branded token");
        let mut table = HashMap::new();

        assert!(insert_lease_in_table(&mut table, token, first_identity).is_ok());
        let (error, second_identity) =
            insert_lease_in_table(&mut table, token, second_identity).expect_err("collision");
        assert_eq!(error, LeaseIssueRejectV1::Exhausted);
        assert_eq!(table.len(), 1);
        assert_eq!(
            host_handles::with_str_handle_ready(first, str::to_owned),
            Some("first".to_owned())
        );
        assert_eq!(
            host_handles::with_str_handle_ready(second, str::to_owned),
            Some("second".to_owned())
        );
        let preserved = table.remove(&token).expect("original lease preserved");
        assert!(host_handles::drop_if_lease_identity_matches(preserved));
        assert!(host_handles::drop_if_lease_identity_matches(
            second_identity
        ));
        assert_eq!(
            host_handles::with_str_handle_ready(first, str::to_owned),
            None
        );
        assert_eq!(
            host_handles::with_str_handle_ready(second, str::to_owned),
            None
        );
    }

    #[test]
    fn stale_end_cannot_drop_lifo_replacement() {
        let _guard = host_handles::test_host_handle_policy_lock()
            .lock()
            .expect("host handle policy lock");
        let previous = std::env::var("NYASH_HOST_HANDLE_ALLOC_POLICY").ok();
        std::env::set_var("NYASH_HOST_HANDLE_ALLOC_POLICY", "lifo");

        let published = publish_end_authorized_text("leased").expect("publish lease");
        let old_handle = published.handle();
        let token = published.token();
        host_handles::drop_handle(old_handle);
        let replacement = host_handles::to_handle_text("replacement");
        assert_eq!(replacement, old_handle);

        assert_eq!(
            consume_end_authorized(token),
            Err(LeaseConsumeRejectV1::StaleHandleIdentity)
        );
        assert_eq!(
            host_handles::with_str_handle_ready(replacement, str::to_owned),
            Some("replacement".to_owned())
        );
        host_handles::drop_handle(replacement);

        if let Some(value) = previous {
            std::env::set_var("NYASH_HOST_HANDLE_ALLOC_POLICY", value);
        } else {
            std::env::remove_var("NYASH_HOST_HANDLE_ALLOC_POLICY");
        }
    }
}
