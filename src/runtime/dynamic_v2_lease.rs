//! Neutral one-shot lease owner for strict DynamicV2 carriers.
//!
//! A lease token is deliberately distinct from a reusable HostHandle.  The
//! strict leaf publishes a token only after it has created the result handle;
//! the token owner is the only code allowed to consume the carrier and drop
//! that handle at the End cutpoint.

use std::collections::hash_map::{Entry, HashMap};
use std::collections::HashSet;
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
    TokenHandleMismatch,
    StaleHandleIdentity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum EndAuthorizedTextBorrowRejectV1 {
    UnknownOrAlreadyConsumed,
    TokenHandleMismatch,
    StaleHandleIdentity,
    NonTextPayload,
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

    /// Adopt a checked runtime result only when the wire handle and lease
    /// token name the same generation-branded identity.  The lease remains
    /// owned by this move-only value until `finish` consumes it.
    pub(crate) fn adopt(
        handle: u64,
        token: NonZeroU64,
    ) -> Result<Self, EndAuthorizedTextBorrowRejectV1> {
        let table = leases().lock().expect("dynamic v2 lease mutex poisoned");
        let identity = table
            .get(&token)
            .ok_or(EndAuthorizedTextBorrowRejectV1::UnknownOrAlreadyConsumed)?;
        if identity.handle() != handle {
            return Err(EndAuthorizedTextBorrowRejectV1::TokenHandleMismatch);
        }
        validate_text_lease_identity(identity)?;
        let mut adopted = adopted_leases()
            .lock()
            .expect("dynamic v2 adopted-lease mutex poisoned");
        if !adopted.insert(token) {
            return Err(EndAuthorizedTextBorrowRejectV1::UnknownOrAlreadyConsumed);
        }
        Ok(Self { handle, token })
    }

    /// Lend the result text only while the lease identity is validated under
    /// the host-handle registry read lock.  No raw pointer or handle escapes.
    pub(crate) fn with_text<R>(
        &self,
        callback: impl FnOnce(&str) -> R,
    ) -> Result<R, EndAuthorizedTextBorrowRejectV1> {
        let (handle, generation) = {
            let table = leases().lock().expect("dynamic v2 lease mutex poisoned");
            let identity = table
                .get(&self.token)
                .ok_or(EndAuthorizedTextBorrowRejectV1::UnknownOrAlreadyConsumed)?;
            if identity.handle() != self.handle {
                return Err(EndAuthorizedTextBorrowRejectV1::TokenHandleMismatch);
            }
            (identity.handle(), identity.generation())
        };
        host_handles::with_text_formal_wire(handle, generation, callback)
            .map_err(map_text_lookup_reject)
    }

    /// Consume the paired lease exactly once.  This is the only normal-path
    /// finish owner for a materialized End-authorized Text result.
    pub(crate) fn finish(self) -> Result<(), LeaseConsumeRejectV1> {
        consume_end_authorized_pair(self.handle, self.token)
    }
}

static NEXT_TOKEN: AtomicU64 = AtomicU64::new(1);
static LEASES: OnceLock<Mutex<HashMap<NonZeroU64, host_handles::HostHandleLeaseIdentityV1>>> =
    OnceLock::new();
static ADOPTED_LEASES: OnceLock<Mutex<HashSet<NonZeroU64>>> = OnceLock::new();
const TOKEN_BRAND: u64 = 1 << 63;

fn leases() -> &'static Mutex<HashMap<NonZeroU64, host_handles::HostHandleLeaseIdentityV1>> {
    LEASES.get_or_init(|| Mutex::new(HashMap::new()))
}

fn adopted_leases() -> &'static Mutex<HashSet<NonZeroU64>> {
    ADOPTED_LEASES.get_or_init(|| Mutex::new(HashSet::new()))
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
    consume_end_authorized_pair(0, token)
}

fn consume_end_authorized_pair(
    expected_handle: u64,
    token: NonZeroU64,
) -> Result<(), LeaseConsumeRejectV1> {
    let mut table = leases().lock().expect("dynamic v2 lease mutex poisoned");
    let identity = table
        .remove(&token)
        .ok_or(LeaseConsumeRejectV1::UnknownOrAlreadyConsumed)?;
    if expected_handle != 0 && identity.handle() != expected_handle {
        table.insert(token, identity);
        return Err(LeaseConsumeRejectV1::TokenHandleMismatch);
    }
    adopted_leases()
        .lock()
        .expect("dynamic v2 adopted-lease mutex poisoned")
        .remove(&token);
    if !host_handles::drop_if_lease_identity_matches(identity) {
        return Err(LeaseConsumeRejectV1::StaleHandleIdentity);
    }
    Ok(())
}

fn validate_text_lease_identity(
    identity: &host_handles::HostHandleLeaseIdentityV1,
) -> Result<(), EndAuthorizedTextBorrowRejectV1> {
    host_handles::with_text_formal_wire(identity.handle(), identity.generation(), |_| ())
        .map(|_| ())
        .map_err(map_text_lookup_reject)
}

fn map_text_lookup_reject(
    error: host_handles::TextFormalLookupRejectV1,
) -> EndAuthorizedTextBorrowRejectV1 {
    match error {
        host_handles::TextFormalLookupRejectV1::NonTextPayload => {
            EndAuthorizedTextBorrowRejectV1::NonTextPayload
        }
        host_handles::TextFormalLookupRejectV1::GenerationMismatch
        | host_handles::TextFormalLookupRejectV1::MissingSlot
        | host_handles::TextFormalLookupRejectV1::ZeroOrOutOfRangeSlot => {
            EndAuthorizedTextBorrowRejectV1::StaleHandleIdentity
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn with_lifo_policy<F: FnOnce()>(f: F) {
        let _guard = host_handles::test_host_handle_policy_lock()
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let previous = std::env::var("NYASH_HOST_HANDLE_ALLOC_POLICY").ok();
        std::env::set_var("NYASH_HOST_HANDLE_ALLOC_POLICY", "lifo");
        f();
        if let Some(value) = previous {
            std::env::set_var("NYASH_HOST_HANDLE_ALLOC_POLICY", value);
        } else {
            std::env::remove_var("NYASH_HOST_HANDLE_ALLOC_POLICY");
        }
    }

    #[test]
    fn lease_is_distinct_and_one_shot() {
        with_lifo_policy(|| {
            let handle = host_handles::to_handle_text("lease");
            let token = issue_end_authorized(handle).expect("live text handle admits");
            assert_ne!(token.get(), handle);
            assert_eq!(consume_end_authorized(token), Ok(()));
            assert_eq!(
                consume_end_authorized(token),
                Err(LeaseConsumeRejectV1::UnknownOrAlreadyConsumed)
            );
        });
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
        with_lifo_policy(|| {
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
        });
    }

    #[test]
    fn stale_end_cannot_drop_lifo_replacement() {
        with_lifo_policy(|| {
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
        });
    }

    #[test]
    fn adopted_end_lends_text_and_finishes_once() {
        with_lifo_policy(|| {
            let published = publish_end_authorized_text("substring").expect("publish lease");
            let handle = published.handle();
            let token = published.token();
            let adopted = EndAuthorizedTextV1::adopt(handle, token).expect("paired adoption");
            assert_eq!(adopted.with_text(str::to_owned), Ok("substring".to_owned()));
            assert_eq!(adopted.finish(), Ok(()));
            assert_eq!(
                EndAuthorizedTextV1::adopt(handle, token),
                Err(EndAuthorizedTextBorrowRejectV1::UnknownOrAlreadyConsumed)
            );
        });
    }

    #[test]
    fn adoption_rejects_foreign_handle_without_consuming_lease() {
        with_lifo_policy(|| {
            let first = publish_end_authorized_text("first").expect("first lease");
            let second = publish_end_authorized_text("second").expect("second lease");
            assert_eq!(
                EndAuthorizedTextV1::adopt(second.handle(), first.token()),
                Err(EndAuthorizedTextBorrowRejectV1::TokenHandleMismatch)
            );
            let adopted = EndAuthorizedTextV1::adopt(first.handle(), first.token())
                .expect("foreign attempt must preserve lease");
            adopted.finish().expect("first finish");
            second.finish().expect("second finish");
        });
    }

    #[test]
    fn adopted_end_rejects_stale_generation_before_lend() {
        with_lifo_policy(|| {
            let published = publish_end_authorized_text("old").expect("publish lease");
            let handle = published.handle();
            let token = published.token();
            host_handles::drop_handle(handle);
            let replacement = host_handles::to_handle_text("replacement");
            assert_eq!(replacement, handle);
            assert_eq!(
                EndAuthorizedTextV1::adopt(handle, token),
                Err(EndAuthorizedTextBorrowRejectV1::StaleHandleIdentity)
            );
            assert_eq!(
                consume_end_authorized(token),
                Err(LeaseConsumeRejectV1::StaleHandleIdentity)
            );
            host_handles::drop_handle(replacement);
        });
    }
}
