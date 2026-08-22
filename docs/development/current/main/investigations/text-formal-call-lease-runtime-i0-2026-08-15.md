---
Status: closed caller-zero implementation row
Date: 2026-08-15
Work mode: fast
Classification: T2 BoxCount; runtime-only
Parent: TEXT-FORMAL-CALL-LEASE-RUNTIME-D0
---

# TEXT-FORMAL-CALL-LEASE-RUNTIME-I0

Implement only the accepted SlotTable call-lifetime state machine and opaque
lease-set API. This row keeps runtime/compiler production callers at zero.

## Six-line brief

```text
Decision: implement one per-slot Vacant/Active call-lifetime state, one atomic nonempty acquire-set transaction, one opaque non-Clone call-wide token, one consuming finish transaction, and one shared retirement terminal.
Source authority + canonical issuer: existing SlotTable payload/generation and exact StableText/StringBox classifier are the mechanical facts; the new call-lifetime child module is the sole pin/token/removal/free-list/DROP_EPOCH owner.
Non-authority: BindingRef/source/target/Recipe/Completion, TextFormalBorrowV1, DynamicV2 lease, raw handle alone, C/LLVM ABI, Builder/session, benchmark, fallback, and retry.
Fail-fast boundary: empty/zero/missing/stale/non-Text/Pending/overflow/token exhaustion and unknown/stale/underflow finish fail with mutation zero; no prefix pin, partial finish, foreign generation retirement, duplicate recycle, or implicit finish.
Smallest next slice: add the child modules and thin parent wiring, migrate both drop paths to the shared terminal, land focused state/API tests, update the runtime owner README and reusable guard, then close caller-zero only.
Non-claims: no compiler actual-origin, physical signature/call arity, C export, prologue/epilogue, Trap lowering, Canonical session, S6C/TextEq caller, production switch, fallback/retry, or main integration.
```

## Implementation files

```text
src/runtime/host_handles/call_lifetime.rs
  SlotCallLifetimeStateV1
  token records/table
  atomic acquire-set and finish-set
  shared retirement terminal

src/runtime/text_formal_call_lease.rs
  opaque public(crate) token/status façade
  no public constructor, Clone, Copy, or implicit Drop finish

src/runtime/host_handles/call_lifetime_tests.rs
src/runtime/text_formal_call_lease_tests.rs
  focused state/API tests

src/runtime/host_handles.rs
  child module wiring, parallel state initialization/allocation, thin delegates

src/runtime/host_handles/lease_identity.rs
  generation-matched drop delegates to shared retirement terminal

src/runtime/host_handles/README.md
  state invariants, authority, non-authority, caller-zero handoff
```

`host_handles.rs` is already near the 760-line design trigger. Move the old
drop implementation into the child; do not append the state machine to the
parent. Every Rust source must remain below 800 lines.

## Exact implementation contract

```text
slots.len = lease_generations.len = call_lifetimes.len
slots[index] == None <=> call_lifetimes[index] == Vacant
Pending => pins > 0
free-list entry => Vacant
active token record => matching payload/generation and pins > 0
```

Acquire-set under one write lock:

```text
reject empty
validate and group every pair by slot/generation
validate exact Text, Open state, generation, grouped checked pin increment
reserve opaque token record
only then commit all pins and publish token
```

Cardinality:

```text
one invocation × one ExactText formal occurrence = one pin
same pair in two formals = multiplicity two
caller forwarding = zero new pins
nested callee entry = a new token and new occurrence pins
```

Retirement:

```text
Open pins=0 -> remove/recycle and DROP_EPOCH +1
Open pins>0 -> Pending, payload/generation retained
Pending -> idempotent accepted request
Missing/generation mismatch -> mutation zero
```

`drop_handle` ignores the internal outcome. Generation-matched drop maps
RetiredNow/Deferred/AlreadyPending to success and Missing/Mismatch to false.
Only actual payload removal owns free-list insertion and `DROP_EPOCH`.

Finish consumes the token id, validates every grouped record first, then
decrements all multiplicities. A Pending slot is removed only when its last pin
reaches zero. Unknown/already-finished token, stale generation, invalid state,
or underflow produces no mutation.

## Required focused tests

```text
single pair acquire/finish
two distinct pairs
same pair multiplicity two
nested pin depth 1 -> 2 -> 1 -> 0
invalid second pair leaves first unpinned
empty/zero/missing/stale/non-Text/Pending acquire mutation zero
grouped overflow and token exhaustion mutation zero
raw drop while pinned defers without recycle/epoch
generation-matched drop while pinned defers and reports success
generation mismatch mutation zero
duplicate retirement request idempotent
last Pending finish removes/recycles/increments epoch exactly once
unknown/duplicate/stale/underflow finish mutation zero
token non-Clone/non-Copy/private constructor/consuming finish
pin=0 drop parity under existing allocation policies
existing DynamicV2 lease and TextFormalBorrow regression suites green
production caller census remains zero
```

## Acceptance

```text
one state owner
one atomic batch issuer
one move-only set token
one shared retirement terminal
all state/table invariants enforced
all failure paths mutation zero
same-pair/nested cardinality exact
free-list/DROP_EPOCH effects exact
existing pin=0 behavior preserved
runtime caller zero
compiler/session/C caller zero
focused tests, cargo check, formatting, reusable guard green
README and current pointer synchronized
```

## Stop conditions

Return to `NoSafeSlice::MissingTextFormalCallLeaseRuntimeOwner` if code needs
sequential public acquisition, prefix rollback, a second registry, Dynamic
lease authority, raw generation recapture, public pin mutation, token
Clone/Copy, implicit Drop finish, pending-slot reuse, multiple retirement
helpers, early `DROP_EPOCH`, compiler/source/Completion inputs, fallback, or
retry.

## Closeout evidence

```text
runtime call-lifetime focused       17/17 green
opaque facade focused                2/2 green
legacy host-handles regression      14/14 green
DynamicV2 lease regression           4/4 green
TextFormalBorrow regression          5/5 green
cargo check                           green
cargo fmt --all -- --check            green
Loop transfer authority guard        green
current-state pointer guard           green
git diff --check                      green
```

The registry carrier is move-only and visible only inside `runtime`; the
crate-facing façade is the only production-shaped acquire/finish surface.
Both surfaces remain caller-zero outside their owner/test modules. No C,
compiler, session, Completion, TextEq, or production edge was opened.
