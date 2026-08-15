use super::*;
use crate::box_trait::IntegerBox;
use crate::runtime::host_handles::HandlePayload;
use std::sync::Arc;

fn test_lock() -> std::sync::MutexGuard<'static, ()> {
    super::super::test_host_handle_policy_lock()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

struct LifoPolicyGuard(Option<String>);

impl LifoPolicyGuard {
    fn install() -> Self {
        let previous = std::env::var("NYASH_HOST_HANDLE_ALLOC_POLICY").ok();
        std::env::set_var("NYASH_HOST_HANDLE_ALLOC_POLICY", "lifo");
        Self(previous)
    }
}

impl Drop for LifoPolicyGuard {
    fn drop(&mut self) {
        if let Some(previous) = self.0.take() {
            std::env::set_var("NYASH_HOST_HANDLE_ALLOC_POLICY", previous);
        } else {
            std::env::remove_var("NYASH_HOST_HANDLE_ALLOC_POLICY");
        }
    }
}

fn pair(registry: &Registry, slot: u64) -> TextFormalWirePairV1 {
    let (slot, generation) = registry
        .capture_text_formal_pair(slot)
        .expect("exact Text pair");
    TextFormalWirePairV1 { slot, generation }
}

fn state(registry: &Registry, slot: u64) -> SlotCallLifetimeStateV1 {
    registry.table.read().call_lifetime.states[slot as usize]
}

fn pins(registry: &Registry, slot: u64) -> u32 {
    match state(registry, slot) {
        SlotCallLifetimeStateV1::Active { call_pins, .. } => call_pins,
        SlotCallLifetimeStateV1::Vacant => 0,
    }
}

fn assert_acquire_reject(
    result: Result<RegistryTextFormalCallLeaseSetV1, TextFormalLeaseAcquireRejectV1>,
    expected: TextFormalLeaseAcquireRejectV1,
) {
    assert_eq!(result.err(), Some(expected));
}

#[test]
fn valid_set_pins_then_finishes_without_retiring_payload() {
    let _guard = test_lock();
    let registry = Registry::new();
    let slot = registry.alloc_text("entry".to_owned());
    let token = registry
        .acquire_text_formal_call_lease_set(&[pair(&registry, slot)])
        .expect("acquire");

    assert_eq!(pins(&registry, slot), 1);
    registry
        .finish_text_formal_call_lease_set(token)
        .expect("finish");
    assert_eq!(pins(&registry, slot), 0);
    assert!(registry.table.read().slots[slot as usize].is_some());
}

#[test]
fn same_pair_multiplicity_and_nested_entries_are_exact() {
    let _guard = test_lock();
    let registry = Registry::new();
    let slot = registry.alloc_text("alias".to_owned());
    let text = pair(&registry, slot);
    let outer = registry
        .acquire_text_formal_call_lease_set(&[text, text])
        .expect("outer alias acquire");
    assert_eq!(pins(&registry, slot), 2);

    let inner = registry
        .acquire_text_formal_call_lease_set(&[text])
        .expect("inner acquire");
    assert_eq!(pins(&registry, slot), 3);
    registry
        .finish_text_formal_call_lease_set(inner)
        .expect("inner finish");
    assert_eq!(pins(&registry, slot), 2);
    registry
        .finish_text_formal_call_lease_set(outer)
        .expect("outer finish");
    assert_eq!(pins(&registry, slot), 0);
}

#[test]
fn invalid_second_formal_leaves_first_unpinned() {
    let _guard = test_lock();
    let registry = Registry::new();
    let first_slot = registry.alloc_text("first".to_owned());
    let first = pair(&registry, first_slot);
    let missing = TextFormalWirePairV1 {
        slot: 0,
        generation: 1,
    };

    assert_acquire_reject(
        registry.acquire_text_formal_call_lease_set(&[first, missing]),
        TextFormalLeaseAcquireRejectV1::ZeroOrOutOfRangeSlot { formal_index: 1 },
    );
    assert_eq!(pins(&registry, first_slot), 0);
    assert!(registry.table.read().call_lifetime.tokens.is_empty());
}

#[test]
fn out_of_range_and_in_range_vacant_slots_remain_distinct() {
    let _guard = test_lock();
    let registry = Registry::new();
    assert_acquire_reject(
        registry.acquire_text_formal_call_lease_set(&[TextFormalWirePairV1 {
            slot: u64::MAX,
            generation: 1,
        }]),
        TextFormalLeaseAcquireRejectV1::ZeroOrOutOfRangeSlot { formal_index: 0 },
    );

    let slot = registry.alloc_text("vacant".to_owned());
    let old = pair(&registry, slot);
    registry.drop_handle(slot);
    assert_acquire_reject(
        registry.acquire_text_formal_call_lease_set(&[old]),
        TextFormalLeaseAcquireRejectV1::MissingSlot { formal_index: 0 },
    );
}

#[test]
fn stale_generation_acquire_is_mutation_free() {
    let _guard = test_lock();
    let registry = Registry::new();
    let slot = registry.alloc_text("generation".to_owned());
    let stale = pair(&registry, slot);
    registry.table.write().lease_generations[slot as usize] += 1;

    assert_acquire_reject(
        registry.acquire_text_formal_call_lease_set(&[stale]),
        TextFormalLeaseAcquireRejectV1::GenerationMismatch { formal_index: 0 },
    );
    assert_eq!(pins(&registry, slot), 0);
}

#[test]
fn non_text_formal_rejects_without_mutation() {
    let _guard = test_lock();
    let registry = Registry::new();
    let slot = registry.alloc(Arc::new(IntegerBox::new(7)));
    let generation = registry.table.read().lease_generations[slot as usize];

    assert_acquire_reject(
        registry.acquire_text_formal_call_lease_set(&[TextFormalWirePairV1 { slot, generation }]),
        TextFormalLeaseAcquireRejectV1::NonTextPayload { formal_index: 0 },
    );
    assert_eq!(pins(&registry, slot), 0);
}

#[test]
fn raw_drop_defers_retirement_until_final_finish() {
    let _guard = test_lock();
    let _policy = LifoPolicyGuard::install();
    let registry = Registry::new();
    let slot = registry.alloc_text("pending".to_owned());
    let token = registry
        .acquire_text_formal_call_lease_set(&[pair(&registry, slot)])
        .expect("acquire");
    let before_epoch = DROP_EPOCH.load(Ordering::Relaxed);

    registry.drop_handle(slot);
    assert_eq!(
        state(&registry, slot),
        SlotCallLifetimeStateV1::Active {
            call_pins: 1,
            retirement: SlotRetirementStateV1::Pending,
        }
    );
    {
        let table = registry.table.read();
        assert!(table.slots[slot as usize].is_some());
        assert!(!table.free.contains(&slot));
    }
    assert_eq!(DROP_EPOCH.load(Ordering::Relaxed), before_epoch);

    registry
        .finish_text_formal_call_lease_set(token)
        .expect("finish pending");
    let table = registry.table.read();
    assert!(table.slots[slot as usize].is_none());
    assert_eq!(table.free.iter().filter(|entry| **entry == slot).count(), 1);
    assert_eq!(DROP_EPOCH.load(Ordering::Relaxed), before_epoch + 1);
}

#[test]
fn repeated_retirement_request_is_idempotent() {
    let _guard = test_lock();
    let _policy = LifoPolicyGuard::install();
    let registry = Registry::new();
    let (slot, identity) = registry.alloc_text_with_lease_identity("pending".to_owned());
    let token = registry
        .acquire_text_formal_call_lease_set(&[pair(&registry, slot)])
        .expect("acquire");
    let before_epoch = DROP_EPOCH.load(Ordering::Relaxed);

    assert!(
        registry.drop_if_lease_identity_matches(HostHandleLeaseIdentityV1 {
            handle: identity.handle,
            generation: identity.generation,
        })
    );
    assert!(registry.drop_if_lease_identity_matches(identity));
    assert_eq!(DROP_EPOCH.load(Ordering::Relaxed), before_epoch);

    registry
        .finish_text_formal_call_lease_set(token)
        .expect("finish pending");
    let table = registry.table.read();
    assert_eq!(table.free.iter().filter(|entry| **entry == slot).count(), 1);
    assert_eq!(DROP_EPOCH.load(Ordering::Relaxed), before_epoch + 1);
}

#[test]
fn pending_slot_rejects_new_entry_without_extra_pin() {
    let _guard = test_lock();
    let registry = Registry::new();
    let slot = registry.alloc_text("retiring".to_owned());
    let text = pair(&registry, slot);
    let token = registry
        .acquire_text_formal_call_lease_set(&[text])
        .expect("acquire");
    registry.drop_handle(slot);

    assert_acquire_reject(
        registry.acquire_text_formal_call_lease_set(&[text]),
        TextFormalLeaseAcquireRejectV1::RetirementPending { formal_index: 0 },
    );
    assert_eq!(pins(&registry, slot), 1);
    registry
        .finish_text_formal_call_lease_set(token)
        .expect("finish");
}

#[test]
fn generation_mismatch_retirement_and_duplicate_finish_do_not_mutate() {
    let _guard = test_lock();
    let registry = Registry::new();
    let (slot, identity) = registry.alloc_text_with_lease_identity("stable".to_owned());
    let token = registry
        .acquire_text_formal_call_lease_set(&[pair(&registry, slot)])
        .expect("acquire");
    let raw_token = token.0;

    assert!(
        !registry.drop_if_lease_identity_matches(HostHandleLeaseIdentityV1 {
            handle: identity.handle,
            generation: identity.generation + 1,
        })
    );
    assert_eq!(pins(&registry, slot), 1);
    registry
        .finish_text_formal_call_lease_set(token)
        .expect("finish");
    assert_eq!(
        registry.finish_text_formal_call_lease_set(RegistryTextFormalCallLeaseSetV1(raw_token)),
        Err(TextFormalLeaseFinishRejectV1::UnknownOrAlreadyFinished)
    );
    assert!(registry.table.read().slots[slot as usize].is_some());
}

#[test]
fn empty_set_is_rejected_without_token() {
    let _guard = test_lock();
    let registry = Registry::new();
    assert_acquire_reject(
        registry.acquire_text_formal_call_lease_set(&[]),
        TextFormalLeaseAcquireRejectV1::EmptyLeaseSet,
    );
    assert!(registry.table.read().call_lifetime.tokens.is_empty());
}

#[test]
fn pin_overflow_rejects_the_complete_set_without_mutation() {
    let _guard = test_lock();
    let registry = Registry::new();
    let slot = registry.alloc_text("overflow".to_owned());
    let text = pair(&registry, slot);
    registry.table.write().call_lifetime.states[slot as usize] = SlotCallLifetimeStateV1::Active {
        call_pins: u32::MAX,
        retirement: SlotRetirementStateV1::Open,
    };

    assert_acquire_reject(
        registry.acquire_text_formal_call_lease_set(&[text]),
        TextFormalLeaseAcquireRejectV1::PinCountOverflow { slot },
    );
    assert_eq!(pins(&registry, slot), u32::MAX);
    assert!(registry.table.read().call_lifetime.tokens.is_empty());
}

#[test]
fn token_exhaustion_rejects_before_any_pin() {
    let _guard = test_lock();
    let registry = Registry::new();
    let slot = registry.alloc_text("token".to_owned());
    let text = pair(&registry, slot);
    registry.table.write().call_lifetime.next_token = None;

    assert_acquire_reject(
        registry.acquire_text_formal_call_lease_set(&[text]),
        TextFormalLeaseAcquireRejectV1::TokenExhausted,
    );
    assert_eq!(pins(&registry, slot), 0);
}

#[test]
fn finish_generation_drift_rejects_without_partial_unpin() {
    let _guard = test_lock();
    let registry = Registry::new();
    let first_slot = registry.alloc_text("first".to_owned());
    let second_slot = registry.alloc_text("second".to_owned());
    let token = registry
        .acquire_text_formal_call_lease_set(&[
            pair(&registry, first_slot),
            pair(&registry, second_slot),
        ])
        .expect("acquire");
    registry.table.write().lease_generations[second_slot as usize] += 1;

    assert_eq!(
        registry.finish_text_formal_call_lease_set(token),
        Err(TextFormalLeaseFinishRejectV1::PinnedGenerationMismatch)
    );
    assert_eq!(pins(&registry, first_slot), 1);
    assert_eq!(pins(&registry, second_slot), 1);
    assert_eq!(registry.table.read().call_lifetime.tokens.len(), 1);
}

#[test]
fn finish_underflow_rejects_without_partial_unpin() {
    let _guard = test_lock();
    let registry = Registry::new();
    let first_slot = registry.alloc_text("first".to_owned());
    let second_slot = registry.alloc_text("second".to_owned());
    let token = registry
        .acquire_text_formal_call_lease_set(&[
            pair(&registry, first_slot),
            pair(&registry, second_slot),
        ])
        .expect("acquire");
    registry.table.write().call_lifetime.states[second_slot as usize] =
        SlotCallLifetimeStateV1::Active {
            call_pins: 0,
            retirement: SlotRetirementStateV1::Open,
        };

    assert_eq!(
        registry.finish_text_formal_call_lease_set(token),
        Err(TextFormalLeaseFinishRejectV1::PinCountUnderflow)
    );
    assert_eq!(pins(&registry, first_slot), 1);
    assert_eq!(pins(&registry, second_slot), 0);
    assert_eq!(registry.table.read().call_lifetime.tokens.len(), 1);
}

#[test]
fn finish_state_mismatch_rejects_without_partial_unpin() {
    let _guard = test_lock();
    let registry = Registry::new();
    let first_slot = registry.alloc_text("first".to_owned());
    let second_slot = registry.alloc_text("second".to_owned());
    let token = registry
        .acquire_text_formal_call_lease_set(&[
            pair(&registry, first_slot),
            pair(&registry, second_slot),
        ])
        .expect("acquire");
    registry.table.write().call_lifetime.states[second_slot as usize] =
        SlotCallLifetimeStateV1::Vacant;

    assert_eq!(
        registry.finish_text_formal_call_lease_set(token),
        Err(TextFormalLeaseFinishRejectV1::CallLifetimeStateMismatch)
    );
    assert_eq!(pins(&registry, first_slot), 1);
    assert_eq!(registry.table.read().call_lifetime.tokens.len(), 1);
}

#[test]
fn fresh_allocation_reactivates_only_a_vacant_slot() {
    let _guard = test_lock();
    let registry = Registry::new();
    let slot = registry.alloc_payload(HandlePayload::StableText("old".to_owned()));
    registry.drop_handle(slot);
    assert_eq!(state(&registry, slot), SlotCallLifetimeStateV1::Vacant);

    let replacement = registry.alloc_payload(HandlePayload::StableText("new".to_owned()));
    if replacement == slot {
        assert_eq!(
            state(&registry, replacement),
            SlotCallLifetimeStateV1::Active {
                call_pins: 0,
                retirement: SlotRetirementStateV1::Open,
            }
        );
    }
}
