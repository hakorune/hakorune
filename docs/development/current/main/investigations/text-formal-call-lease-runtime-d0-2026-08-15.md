---
Status: accepted BoxShape; caller-zero runtime I0 selected
Date: 2026-08-15
Work mode: fast
Classification: accepted T2 BoxShape; runtime state machine only
---

# TEXT-FORMAL-CALL-LEASE-RUNTIME-D0

This row fixes the caller-zero runtime substrate for the mandatory ExactText
callee-entry lease-set. It does not activate a compiler call edge, ABI caller,
Canonical session, TextEq route, or production selection.

## Six-line brief

```text
Decision: add one SlotTable-owned call-lifetime state per handle slot and one all-or-nothing callee-invocation lease-set transaction; every ExactText formal occurrence adds one pin, equal-pair aliases add multiplicity, and nested callee entries acquire independent sets.
Source authority + canonical issuer: the host-handle generation table and exact Text payload classifier are the mechanical source; `acquire_text_formal_call_leases_v1` is the sole batch pin issuer, `TextFormalCallLeaseSetTokenV1::finish` is the sole set discharge, and one shared retirement helper owns removal/free-list/DROP_EPOCH effects.
Non-authority: raw HostHandle, `TextFormalBorrowV1`, DynamicV2 lease, `retain_h`, Arc count, source BindingRef, signature lane role, Completion, MIR/ValueId, caller forwarding, benchmark, fallback, and retry.
Fail-fast boundary: under one SlotTable write lock, preflight every pair before mutation; reject zero/missing/stale/non-Text/retiring/overflow with no pin change, and never recycle or increment DROP_EPOCH until the final pin discharges a pending retirement.
Smallest next slice: after this D0 is accepted, implement a caller-zero BoxCount in `host_handles/text_formal_call_lease.rs`, converge both existing drop paths on the shared retirement terminal, and add exact state-transition tests.
Non-claims: no C export, compiler actual-origin, physical signature, call arity activation, prologue/epilogue, Trap lowering, Canonical session, S6C/TextEq caller, production switch, fallback, retry, or main integration.
```

## Current runtime facts

`SlotTable` currently owns dense payload slots, a reusable free list, and a
parallel generation table. It has no call-pin or pending-retirement state.
`drop_handle` removes and recycles immediately. The generation-matched Dynamic
lease drop independently removes and recycles immediately. Both increment
`DROP_EPOCH` at removal time. `TextFormalBorrowV1` validates a pair only for a
single read-lock closure; it is not a call-lifetime capability.

Therefore this row adds one state machine to the existing registry rather than
reusing Dynamic lease or inventing a second registry.

## Canonical state

```rust
enum SlotCallLifetimeStateV1 {
    Vacant,
    Active {
        call_pins: u32,
        retirement: SlotRetirementStateV1,
    },
}

enum SlotRetirementStateV1 {
    Open,
    Pending,
}
```

The state is parallel to `slots` and `lease_generations`; allocation initializes
`Active { call_pins = 0, retirement = Open }`, and actual removal sets `Vacant`
before reuse. Required invariants are:

```text
slots.len = lease_generations.len = call_lifetimes.len
slots[index] == None <=> call_lifetimes[index] == Vacant
Pending => call_pins > 0
free-list entry => Vacant
active token record => matching payload/generation and pins > 0
```

The observable states are:

```text
Vacant
Live             payload present, pins = 0, retirement = Open
Pinned           payload present, pins > 0, retirement = Open
RetirePending    payload present, pins > 0, retirement = Pending
```

`Pending` with zero pins is not stable: the transition that reaches zero must
remove/recycle atomically before releasing the write lock.

## Atomic lease-set acquisition

The production-shaped runtime authority is a set transaction, not repeated
single-pair acquisition:

```rust
pub(crate) fn acquire_text_formal_call_leases_v1(
    pairs: &[TextFormalBorrowV1],
) -> Result<TextFormalCallLeaseSetTokenV1, TextFormalLeaseAcquireRejectV1>;
```

The implementation performs two phases under one SlotTable write lock:

```text
preflight all occurrences
  - slot is nonzero/in range/present
  - generation matches
  - payload is exact Text
  - retirement is Open
  - grouped pin increment cannot overflow

reserve one token record
commit all grouped pin increments and publish the move-only token
```

If any preflight or token reservation fails, no slot is mutated and no rollback
token is needed. An empty pair set is `EmptyLeaseSet`; a no-Text callable uses
the separate signature-issued no-lease path and receives no fake capability.

## Pin cardinality

The exact unit is:

```text
one callee invocation × one ExactText formal occurrence = one pin
```

Consequences:

```text
f(text, text)             -> the same slot receives +2 pins
outer call active         -> outer set owns its pins
nested call entry         -> nested set adds new pins
caller lane forwarding    -> adds zero pins
```

The SlotTable retains an opaque token record of grouped
`{slot,generation,occurrences}` pins. The public Rust token contains only a
private nonzero token id, is `#[must_use]`, and exposes neither `Clone`/`Copy`
nor a public constructor, raw count, or prefix finish. `Drop` does not silently
finish; only explicit consuming finish is authority.

## Shared retirement terminal

Both existing drop paths must converge on one helper:

```rust
fn request_slot_retirement_v1(
    table: &mut SlotTable,
    slot: u64,
    expected_generation: Option<u64>,
) -> SlotRetirementOutcomeV1;
```

```rust
enum SlotRetirementOutcomeV1 {
    RetiredNow,
    DeferredByCallPins { pins: u32 },
    AlreadyPending,
    Missing,
    GenerationMismatch,
}
```

Rules:

```text
Live, pins=0       -> remove/recycle now
Pinned, pins>0     -> mark Pending, retain payload/generation
Pending            -> idempotent AlreadyPending
Vacant             -> Missing
generation mismatch-> no mutation
```

Only actual removal pushes the free list and increments `DROP_EPOCH`, exactly
once per removed slot. A pending slot is never reusable and its generation
never changes. `drop_handle` discards the internal outcome. The
generation-matched drop projects `RetiredNow`, `DeferredByCallPins`, and
`AlreadyPending` to success, while `Missing` and `GenerationMismatch` remain
false.

## Set finish

`TextFormalCallLeaseSetTokenV1::finish(self)` consumes the whole token under
one write lock. It validates each grouped slot/generation, decrements exact
multiplicity, and removes a `Pending` slot only when its final pin reaches zero.

Finish rejects or traps on foreign generation, underflow, missing slot, or a
token/table mismatch. It cannot retire a different generation. Repeated finish
is structurally unavailable because the token is move-only and has no public
constructor.

The current no-unwind contract means:

```text
acquire reject       -> no token, no cleanup
normal continuation  -> exactly one finish
noreturn trap        -> no post-trap finish callback
trap sentinel return -> finish before entering the trap terminal
```

Compiler epilogue coverage remains a later row.

## Transition table

| Operation | Live, pins=0 | Pinned | RetirePending | Vacant/stale |
|---|---|---|---|---|
| acquire-set | add multiplicity | add multiplicity | reject | reject |
| drop request | retire now | mark Pending | idempotent | missing/mismatch |
| finish-set | reject underflow | subtract | subtract; zero retires | reject |

Overflow, underflow, and any multi-pair validation failure are no-mutation
rejects.

## Typed outcomes

```text
Acquire:
  EmptyLeaseSet
  ZeroOrOutOfRangeSlot { formal_index }
  MissingSlot { formal_index }
  GenerationMismatch { formal_index }
  NonTextPayload { formal_index }
  RetirementPending { formal_index }
  PinCountOverflow { slot }
  TokenExhausted

Finish:
  UnknownOrAlreadyFinished
  MissingPinnedSlot
  PinnedGenerationMismatch
  PinCountUnderflow
  CallLifetimeStateMismatch
```

These are invariant/runtime statuses for a future canonical Trap owner, never
language Fault, alternate-route selection, fallback, or retry.

## Focused test matrix

```text
one valid pair acquire -> pins 1
finish without retirement -> pins 0, payload remains
two distinct pairs -> independent pins
same pair twice -> multiplicity 2
outer acquire / inner acquire / inner finish / outer finish -> 1/2/1/0
drop_handle while pinned -> Pending, payload retained, no free-list/DROP_EPOCH
generation-matched drop while pinned -> Pending
generation-mismatched drop -> no mutation
last finish of Pending -> remove once, recycle once, DROP_EPOCH +1
duplicate retirement request -> no duplicate recycle/epoch
zero/missing/stale/non-Text/retiring acquire -> no mutation
second pair invalid -> first pair remains unpinned
grouped pin overflow -> entire set rejected, no mutation
foreign-generation/underflow finish -> reject, no foreign retirement
token is non-Clone, constructor private, finish consumes self
existing pin=0 drop behavior remains unchanged
```

## File boundary and size ratchet

`src/runtime/host_handles.rs` is already above the 760-line design trigger, so
the implementation must not grow the state-machine body there.

```text
src/runtime/host_handles/call_lifetime.rs
  slot state, token records, atomic acquire/finish, shared retirement terminal
  target: 220-340 lines

src/runtime/text_formal_call_lease.rs
  opaque capability and typed acquire/finish façade; caller-zero
  target: 100-180 lines

src/runtime/host_handles/call_lifetime_tests.rs
  SlotTable transition/atomicity/multiplicity tests
  target: 220-360 lines

src/runtime/text_formal_call_lease_tests.rs
  opaque capability/API tests
  target: 100-180 lines

src/runtime/host_handles.rs
  module wiring, one parallel state field, allocation initialization,
  thin delegation only; remain below 800 hard stop

src/runtime/host_handles/lease_identity.rs
  generation identity/lookup only; matched drop delegates to shared helper

src/runtime/host_handles/README.md
  state invariants, mechanical authority, non-authority, future compiler seam
```

No new top-level guard is required. Extend the existing relevant runtime/Loop
guard with a file-size census and forbid public token construction/Clone if a
current reusable guard owns that boundary.

## Accepted D0

```text
one SlotTable lifetime state owner
slots/generations/lifetimes length and Vacant/Active invariants exact
one atomic all-pair acquire-set issuer
empty set rejected; token reservation included in the transaction
one pin per formal occurrence
same-pair alias multiplicity exact
nested callee pin cardinality exact
one move-only call-wide set token
one private token-record table with grouped occurrence counts
one shared retirement terminal for both drop paths
pending slots never recycled
actual removal owns free-list and DROP_EPOCH exactly once
finish consumes the set and cannot retire foreign generation
all validation/overflow failures are no-mutation
pin=0 legacy drop parity fixed
runtime/compiler/session responsibility boundary fixed
```

## Implementation stop line

Keep:

```text
NoSafeSlice::MissingTextFormalCallLeaseRuntimeOwner
```

if implementation requires sequential public acquire, prefix rollback, a
second registry, Dynamic lease authority, raw generation recapture, public
count mutation, token Clone, pending-slot reuse, multiple retirement helpers,
DROP_EPOCH before actual removal, compiler source/target inputs, MIR/ValueId,
Completion, fallback, or retry.

## Ordered follow-on

After this accepted D0:

```text
TEXT-FORMAL-CALL-LEASE-RUNTIME-I0
  runtime-only caller-zero BoxCount
  -> focused runtime tests and guard
  -> no compiler caller claim

CALLABLE-TEXT-FORMAL-PHYSICAL-SIGNATURE-R0/I0
  signature cohort without Completion dependency
  -> combined Installed Port + exact call-edge owner
  -> mapping-aware skeleton/call lanes

TEXT-FORMAL-ENTRY-NORMAL-EXIT-EPILOGUE-D0/I0
  Canonical entry lease-set ledger
  -> Completion-backed normal-exit finish coverage
  -> DraftSeal Return ordering
```

The runtime I0 proves only that a correct pair set can be pinned and retired
safely. It does not prove that the compiler supplies the correct pairs or that
all normal exits finish the set.
