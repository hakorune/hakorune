//! Call-scoped Text-formal pins and pin-aware slot retirement.
//!
//! This module is a mechanical runtime owner. It knows slot generations,
//! exact Text payloads, pin multiplicity, and retirement. It does not know
//! source bindings, callable signatures, MIR, Completion, or TextEq routes.

use std::collections::BTreeMap;
use std::num::NonZeroU64;
use std::sync::atomic::Ordering;

use crate::config::env::HostHandleAllocPolicyMode;
use crate::runtime::host_handles_policy;
use crate::runtime::text_formal_abi::TextFormalWirePairV1;

use super::lease_identity::{exact_text_ref, HostHandleLeaseIdentityV1};
use super::{HandlePayload, Registry, SlotTable, DROP_EPOCH};

#[inline(always)]
fn stable_text_ref(payload: &HandlePayload) -> Option<&str> {
    match payload {
        HandlePayload::StableText(text) => Some(text.as_str()),
        HandlePayload::StableBox(_) => None,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum SlotRetirementStateV1 {
    Open,
    Pending,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum SlotCallLifetimeStateV1 {
    Vacant,
    Active {
        call_pins: u32,
        retirement: SlotRetirementStateV1,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TextFormalCallPinRecordV1 {
    slot: u64,
    generation: u64,
    occurrences: u32,
}

pub(super) struct CallLifetimeTableV1 {
    states: Vec<SlotCallLifetimeStateV1>,
    tokens: BTreeMap<NonZeroU64, Box<[TextFormalCallPinRecordV1]>>,
    next_token: Option<NonZeroU64>,
}

impl CallLifetimeTableV1 {
    pub(super) fn new() -> Self {
        Self {
            states: vec![SlotCallLifetimeStateV1::Vacant],
            tokens: BTreeMap::new(),
            next_token: NonZeroU64::new(1),
        }
    }

    pub(super) fn activate_slot_or_panic(&mut self, idx: usize) {
        if idx == self.states.len() {
            self.states.push(SlotCallLifetimeStateV1::Active {
                call_pins: 0,
                retirement: SlotRetirementStateV1::Open,
            });
            return;
        }
        match self.states.get_mut(idx) {
            Some(state @ SlotCallLifetimeStateV1::Vacant) => {
                *state = SlotCallLifetimeStateV1::Active {
                    call_pins: 0,
                    retirement: SlotRetirementStateV1::Open,
                };
            }
            Some(SlotCallLifetimeStateV1::Active { .. }) => {
                super::host_handle_panic(
                    "[host_handles] allocation reached active call-lifetime slot",
                );
            }
            None => super::host_handle_panic(
                "[host_handles] call-lifetime state is behind slot allocation",
            ),
        }
    }

    fn reserve_token(
        &self,
    ) -> Result<RegistryTextFormalCallLeaseSetV1, TextFormalLeaseAcquireRejectV1> {
        let raw = self
            .next_token
            .ok_or(TextFormalLeaseAcquireRejectV1::TokenExhausted)?;
        if self.tokens.contains_key(&raw) {
            return Err(TextFormalLeaseAcquireRejectV1::TokenExhausted);
        }
        Ok(RegistryTextFormalCallLeaseSetV1(raw))
    }

    fn advance_token(&mut self, token: &RegistryTextFormalCallLeaseSetV1) {
        self.next_token = token.0.get().checked_add(1).and_then(NonZeroU64::new);
    }
}

#[derive(Debug)]
pub(in crate::runtime) struct RegistryTextFormalCallLeaseSetV1(NonZeroU64);

impl RegistryTextFormalCallLeaseSetV1 {
    pub(in crate::runtime) fn raw_token(&self) -> u64 {
        self.0.get()
    }
}

#[derive(Debug, Clone, Copy)]
pub(in crate::runtime) struct TextFormalRootDescriptorV1 {
    pub(in crate::runtime) ptr: *const u8,
    pub(in crate::runtime) byte_len: u64,
}

#[derive(Debug)]
pub(in crate::runtime) struct RegistryTextFormalCallResidenceV1 {
    lease: RegistryTextFormalCallLeaseSetV1,
    roots: Box<[TextFormalRootDescriptorV1]>,
}

impl RegistryTextFormalCallResidenceV1 {
    pub(in crate::runtime) fn root_count(&self) -> usize {
        self.roots.len()
    }

    pub(in crate::runtime) fn root(&self, index: usize) -> Option<TextFormalRootDescriptorV1> {
        self.roots.get(index).copied()
    }

    pub(in crate::runtime) fn finish(self) -> Result<(), TextFormalLeaseFinishRejectV1> {
        super::reg().finish_text_formal_call_lease_set(self.lease)
    }

    pub(in crate::runtime) fn into_raw_parts(self) -> (u64, Box<[TextFormalRootDescriptorV1]>) {
        let Self { lease, roots } = self;
        (lease.raw_token(), roots)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TextFormalLeaseAcquireRejectV1 {
    EmptyLeaseSet,
    ZeroOrOutOfRangeSlot { formal_index: usize },
    MissingSlot { formal_index: usize },
    GenerationMismatch { formal_index: usize },
    NonTextPayload { formal_index: usize },
    RetirementPending { formal_index: usize },
    PinCountOverflow { slot: u64 },
    ByteLengthOutOfRange { formal_index: usize },
    TokenExhausted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TextFormalLeaseFinishRejectV1 {
    UnknownOrAlreadyFinished,
    MissingPinnedSlot,
    PinnedGenerationMismatch,
    PinCountUnderflow,
    CallLifetimeStateMismatch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SlotRetirementOutcomeV1 {
    RetiredNow,
    DeferredByCallPins { pins: u32 },
    AlreadyPending,
    Missing,
    GenerationMismatch,
}

fn retire_slot_now(
    table: &mut SlotTable,
    policy: HostHandleAllocPolicyMode,
    idx: usize,
    slot: u64,
) {
    table.slots[idx] = None;
    table.call_lifetime.states[idx] = SlotCallLifetimeStateV1::Vacant;
    host_handles_policy::recycle_handle(policy, &mut table.free, slot);
    DROP_EPOCH.fetch_add(1, Ordering::Relaxed);
}

fn request_slot_retirement_v1(
    table: &mut SlotTable,
    policy: HostHandleAllocPolicyMode,
    slot: u64,
    expected_generation: Option<u64>,
) -> SlotRetirementOutcomeV1 {
    let Ok(idx) = usize::try_from(slot) else {
        return SlotRetirementOutcomeV1::Missing;
    };
    let Some(payload) = table.slots.get(idx) else {
        return SlotRetirementOutcomeV1::Missing;
    };
    if payload.is_none() {
        return SlotRetirementOutcomeV1::Missing;
    }
    if expected_generation
        .is_some_and(|generation| table.lease_generations.get(idx).copied() != Some(generation))
    {
        return SlotRetirementOutcomeV1::GenerationMismatch;
    }
    match table.call_lifetime.states.get(idx).copied() {
        Some(SlotCallLifetimeStateV1::Active {
            call_pins: 0,
            retirement: SlotRetirementStateV1::Open,
        }) => {
            retire_slot_now(table, policy, idx, slot);
            SlotRetirementOutcomeV1::RetiredNow
        }
        Some(SlotCallLifetimeStateV1::Active {
            call_pins,
            retirement: SlotRetirementStateV1::Open,
        }) => {
            table.call_lifetime.states[idx] = SlotCallLifetimeStateV1::Active {
                call_pins,
                retirement: SlotRetirementStateV1::Pending,
            };
            SlotRetirementOutcomeV1::DeferredByCallPins { pins: call_pins }
        }
        Some(SlotCallLifetimeStateV1::Active {
            call_pins,
            retirement: SlotRetirementStateV1::Pending,
        }) if call_pins > 0 => SlotRetirementOutcomeV1::AlreadyPending,
        _ => SlotRetirementOutcomeV1::Missing,
    }
}

impl Registry {
    fn acquire_text_formal_call_lease_set_with_roots(
        &self,
        pairs: &[TextFormalWirePairV1],
        stable_text_only: bool,
    ) -> Result<
        (
            RegistryTextFormalCallLeaseSetV1,
            Box<[TextFormalRootDescriptorV1]>,
        ),
        TextFormalLeaseAcquireRejectV1,
    > {
        if pairs.is_empty() {
            return Err(TextFormalLeaseAcquireRejectV1::EmptyLeaseSet);
        }

        let mut table = self.table.write();
        let token = table.call_lifetime.reserve_token()?;
        let mut grouped = BTreeMap::<(u64, u64), (usize, usize, u32)>::new();
        let mut roots = Vec::with_capacity(pairs.len());

        for (formal_index, pair) in pairs.iter().copied().enumerate() {
            if pair.slot == 0 || pair.generation == 0 {
                return Err(TextFormalLeaseAcquireRejectV1::ZeroOrOutOfRangeSlot { formal_index });
            }
            let idx = usize::try_from(pair.slot).map_err(|_| {
                TextFormalLeaseAcquireRejectV1::ZeroOrOutOfRangeSlot { formal_index }
            })?;
            if idx >= table.slots.len() {
                return Err(TextFormalLeaseAcquireRejectV1::ZeroOrOutOfRangeSlot { formal_index });
            }
            let payload = table
                .slots
                .get(idx)
                .and_then(Option::as_ref)
                .ok_or(TextFormalLeaseAcquireRejectV1::MissingSlot { formal_index })?;
            if table.lease_generations.get(idx).copied() != Some(pair.generation) {
                return Err(TextFormalLeaseAcquireRejectV1::GenerationMismatch { formal_index });
            }
            let text = if stable_text_only {
                stable_text_ref(payload)
            } else {
                exact_text_ref(payload)
            }
            .ok_or(TextFormalLeaseAcquireRejectV1::NonTextPayload { formal_index })?;
            let byte_len = u64::try_from(text.len()).map_err(|_| {
                TextFormalLeaseAcquireRejectV1::ByteLengthOutOfRange { formal_index }
            })?;
            if byte_len > i64::MAX as u64 {
                return Err(TextFormalLeaseAcquireRejectV1::ByteLengthOutOfRange { formal_index });
            }
            roots.push(TextFormalRootDescriptorV1 {
                ptr: text.as_ptr(),
                byte_len,
            });
            match table.call_lifetime.states.get(idx).copied() {
                Some(SlotCallLifetimeStateV1::Active {
                    retirement: SlotRetirementStateV1::Open,
                    ..
                }) => {}
                Some(SlotCallLifetimeStateV1::Active {
                    retirement: SlotRetirementStateV1::Pending,
                    ..
                }) => {
                    return Err(TextFormalLeaseAcquireRejectV1::RetirementPending { formal_index });
                }
                _ => {
                    return Err(TextFormalLeaseAcquireRejectV1::MissingSlot { formal_index });
                }
            }
            let grouped_entry =
                grouped
                    .entry((pair.slot, pair.generation))
                    .or_insert((idx, formal_index, 0));
            grouped_entry.2 = grouped_entry
                .2
                .checked_add(1)
                .ok_or(TextFormalLeaseAcquireRejectV1::PinCountOverflow { slot: pair.slot })?;
        }

        for (&(slot, _), &(idx, formal_index, occurrences)) in &grouped {
            let SlotCallLifetimeStateV1::Active {
                call_pins,
                retirement: SlotRetirementStateV1::Open,
            } = table.call_lifetime.states[idx]
            else {
                return Err(TextFormalLeaseAcquireRejectV1::RetirementPending { formal_index });
            };
            call_pins
                .checked_add(occurrences)
                .ok_or(TextFormalLeaseAcquireRejectV1::PinCountOverflow { slot })?;
        }

        let records = grouped
            .into_iter()
            .map(
                |((slot, generation), (_, _, occurrences))| TextFormalCallPinRecordV1 {
                    slot,
                    generation,
                    occurrences,
                },
            )
            .collect::<Vec<_>>()
            .into_boxed_slice();

        for record in records.iter().copied() {
            let idx = usize::try_from(record.slot)
                .expect("validated Text formal call lease slot must fit usize");
            let SlotCallLifetimeStateV1::Active {
                call_pins,
                retirement,
            } = table.call_lifetime.states[idx]
            else {
                super::host_handle_panic("[host_handles] validated call-lifetime slot vanished");
            };
            table.call_lifetime.states[idx] = SlotCallLifetimeStateV1::Active {
                call_pins: call_pins + record.occurrences,
                retirement,
            };
        }
        table.call_lifetime.tokens.insert(token.0, records);
        table.call_lifetime.advance_token(&token);
        Ok((token, roots.into_boxed_slice()))
    }

    fn acquire_text_formal_call_lease_set(
        &self,
        pairs: &[TextFormalWirePairV1],
    ) -> Result<RegistryTextFormalCallLeaseSetV1, TextFormalLeaseAcquireRejectV1> {
        self.acquire_text_formal_call_lease_set_with_roots(pairs, false)
            .map(|(token, _)| token)
    }

    fn acquire_text_formal_call_residence(
        &self,
        pairs: &[TextFormalWirePairV1],
    ) -> Result<RegistryTextFormalCallResidenceV1, TextFormalLeaseAcquireRejectV1> {
        let (lease, roots) = self.acquire_text_formal_call_lease_set_with_roots(pairs, true)?;
        Ok(RegistryTextFormalCallResidenceV1 { lease, roots })
    }

    fn finish_text_formal_call_lease_set(
        &self,
        token: RegistryTextFormalCallLeaseSetV1,
    ) -> Result<(), TextFormalLeaseFinishRejectV1> {
        let policy = self.alloc_policy_mode();
        let mut table = self.table.write();
        let records = table
            .call_lifetime
            .tokens
            .get(&token.0)
            .ok_or(TextFormalLeaseFinishRejectV1::UnknownOrAlreadyFinished)?;

        for record in records.iter().copied() {
            let idx = usize::try_from(record.slot)
                .map_err(|_| TextFormalLeaseFinishRejectV1::MissingPinnedSlot)?;
            if table.slots.get(idx).map_or(true, Option::is_none) {
                return Err(TextFormalLeaseFinishRejectV1::MissingPinnedSlot);
            }
            if table.lease_generations.get(idx).copied() != Some(record.generation) {
                return Err(TextFormalLeaseFinishRejectV1::PinnedGenerationMismatch);
            }
            match table.call_lifetime.states.get(idx).copied() {
                Some(SlotCallLifetimeStateV1::Active { call_pins, .. })
                    if call_pins >= record.occurrences => {}
                Some(SlotCallLifetimeStateV1::Active { .. }) => {
                    return Err(TextFormalLeaseFinishRejectV1::PinCountUnderflow);
                }
                _ => return Err(TextFormalLeaseFinishRejectV1::CallLifetimeStateMismatch),
            }
        }

        let records = table
            .call_lifetime
            .tokens
            .remove(&token.0)
            .expect("validated call-lifetime token must remain present");
        for record in records.iter().copied() {
            let idx = usize::try_from(record.slot)
                .expect("validated Text formal call lease slot must fit usize");
            let SlotCallLifetimeStateV1::Active {
                call_pins,
                retirement,
            } = table.call_lifetime.states[idx]
            else {
                super::host_handle_panic("[host_handles] validated call-lifetime state vanished");
            };
            let remaining = call_pins - record.occurrences;
            if remaining == 0 && retirement == SlotRetirementStateV1::Pending {
                retire_slot_now(&mut table, policy, idx, record.slot);
            } else {
                table.call_lifetime.states[idx] = SlotCallLifetimeStateV1::Active {
                    call_pins: remaining,
                    retirement,
                };
            }
        }
        Ok(())
    }

    #[inline(always)]
    pub(super) fn drop_handle(&self, slot: u64) {
        let policy = self.alloc_policy_mode();
        let mut table = self.table.write();
        let _ = request_slot_retirement_v1(&mut table, policy, slot, None);
    }

    #[inline(always)]
    pub(super) fn drop_if_lease_identity_matches(
        &self,
        identity: HostHandleLeaseIdentityV1,
    ) -> bool {
        let policy = self.alloc_policy_mode();
        let mut table = self.table.write();
        matches!(
            request_slot_retirement_v1(
                &mut table,
                policy,
                identity.handle,
                Some(identity.generation),
            ),
            SlotRetirementOutcomeV1::RetiredNow
                | SlotRetirementOutcomeV1::DeferredByCallPins { .. }
                | SlotRetirementOutcomeV1::AlreadyPending
        )
    }
}

pub(in crate::runtime) fn acquire_text_formal_call_lease_set_v1(
    pairs: &[TextFormalWirePairV1],
) -> Result<RegistryTextFormalCallLeaseSetV1, TextFormalLeaseAcquireRejectV1> {
    super::reg().acquire_text_formal_call_lease_set(pairs)
}

pub(in crate::runtime) fn acquire_text_formal_call_residence_v1(
    pairs: &[TextFormalWirePairV1],
) -> Result<RegistryTextFormalCallResidenceV1, TextFormalLeaseAcquireRejectV1> {
    super::reg().acquire_text_formal_call_residence(pairs)
}

pub(in crate::runtime) fn finish_text_formal_call_lease_set_v1(
    token: RegistryTextFormalCallLeaseSetV1,
) -> Result<(), TextFormalLeaseFinishRejectV1> {
    super::reg().finish_text_formal_call_lease_set(token)
}

pub(in crate::runtime) fn finish_text_formal_call_lease_set_raw_v1(
    raw_token: u64,
) -> Result<(), TextFormalLeaseFinishRejectV1> {
    let Some(token) = NonZeroU64::new(raw_token) else {
        return Err(TextFormalLeaseFinishRejectV1::UnknownOrAlreadyFinished);
    };
    super::reg().finish_text_formal_call_lease_set(RegistryTextFormalCallLeaseSetV1(token))
}

#[cfg(test)]
#[path = "call_lifetime_tests.rs"]
mod tests;
