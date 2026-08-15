---
Status: closed caller-zero implementation row; next design stop is the installed S6C/common-V2 composition
Date: 2026-08-15
Parent: `docs/development/current/main/investigations/callable-physical-text-parameter-abi-d0-2026-08-15.md`
Classification: closed T2 BoxCount
Authority: `TextFormalBorrowV1` Rust validator and its fixed C wire projection
---

# CALLABLE-PHYSICAL-TEXT-PARAMETER-ABI-I0

This row implements only the accepted physical Text formal lane. It validates
an existing host-handle slot plus its generation and lends the live Text
payload through a closure-scoped Rust borrow. The C function is a status
projection of the same validator; it is not a new semantic classifier.

## Six-line brief

```text
Decision: issue one non-Clone TextFormalBorrowV1 with {slot,generation}; validate live Text and lend it only through a closure; expose one fixed C status projection.
Source authority + canonical issuer: host-handle slot/generation table + Text payload classifier; issue_text_formal_borrow_v1 is the only runtime issuer and the package/header binder remains the only compiler co-seal issuer.
Non-authority: raw HostHandle without generation, DynamicV2 lease tokens, BorrowedHandleBox, TextScan, eq_hh, MirType, StringBox spelling, TextEq, AST/MIR, fallback, retry.
Fail-fast boundary: zero/missing slot, generation mismatch, non-Text payload, or invalid C status is a typed invariant Trap candidate before body effects; no language Fault or truthy Bool conversion.
Smallest next slice: add one runtime child module, one fixed C header/export, focused positive/negative tests, and guards; production callable, S6C, Builder, and session callers remain zero.
Non-claims: no retain/release, no public Text object publication, no TextEq route, no residence across a physical session, no ReadyEntry, MIR/CFG/SSA/PHI, selector, fallback, retry, or legacy retirement.
```

## Ownership and API shape

```text
runtime::text_formal_abi
  -> TextFormalBorrowV1 { slot, generation }       // move-only
       -> validate(self)                           // typed status
       -> with_text(self, |&str| ...)              // read-lock scoped borrow

The validator uses the existing host-handle generation table as a mechanical
substrate, but the formal lane owns the exact Text classification: only the
registry's StableText and the language StringBox representation are admitted.
Generic `as_str_fast` plugin payloads, raw HostHandle values, and DynamicV2
lease identities are not formal lanes.
```

The module may read the existing private slot table and generation vector, but
it does not expose the table, `HandlePayload`, or a raw `Registry` getter. The
borrow owns no `Arc`, does not retain/release, and cannot escape the closure.
The caller is responsible for keeping the strong source owner alive until the
call returns. A future physical session must issue a separate session
residence product if it needs a borrow that spans multiple operations.

The C projection is:

```c
typedef struct NyrtTextFormalBorrowV1 {
    uint64_t slot;
    uint64_t generation;
} NyrtTextFormalBorrowV1;

uint32_t hako_text_formal_validate_v1(uint64_t slot, uint64_t generation);
```

The status values are fixed and exhaustive:

```text
0 Valid
1 ZeroOrOutOfRangeSlot
2 MissingSlot
3 GenerationMismatch
4 NonTextPayload
```

Unknown status values are rejected by the C/Rust parity test. No result is a
language Fault, and no invalid input is retried through a generic handle or
`eq_hh` route.

## Focused acceptance

Positive:

```text
live StringBox/Text handle -> capture -> validate -> closure reads exact text
live StableText with generation N -> replacement generation N+1 rejects old pair
C status 0 == Rust Valid
```

Negative:

```text
slot 0                       -> ZeroOrOutOfRangeSlot
unknown slot                 -> MissingSlot
same slot / old generation  -> GenerationMismatch
live IntegerBox/other box   -> NonTextPayload
raw handle without generation -> API unavailable
invalid status / retry      -> structural guard failure
```

## Scope and guards

Keep the owner in `src/runtime/text_formal_abi.rs`, with the narrow generation
lookup helper in `src/runtime/host_handles/lease_identity.rs`, a separate test
module, the fixed `include/nyrt_text_formal_v1.h`, and one kernel export module.
Do not grow `host_handles.rs` or an existing export file past the 760-line
design trigger. The existing Loop physical-transfer guard now requires these
files and rejects DynamicV2/eq_hh/fallback/retry references in the validator.
The new export has no production compiler caller; its only I0 callers are
focused tests.

## Implementation evidence

The caller-zero lane is implemented and remains deliberately detached from
the compiler and S6C. `TextFormalBorrowV1` is non-`Clone`/non-`Copy`, carries
the published slot generation, validates under one registry read lock, and
lends `&str` only through `with_text`. The host table rejects zero/out-of-range
slots, missing slots, generation mismatch, and non-Text payloads. The C bridge
`hako_text_formal_validate_v1` returns the same fixed status values and does
not recapture a generation from a raw handle.

Focused evidence:

```text
cargo test --lib -q text_formal_abi       # 5 passed
cargo test -p nyash_kernel text_formal     # 1 passed
cargo check -q                             # green; inherited warning census
loop_physical_transfer_authority_guard.sh  # green
current-state pointer guard + diff check   # green
```

No production callable, S6C, TextEq, ReadyEntry, Builder/session, fallback,
retry, or legacy retirement is claimed by this row.

## Exit and next row

I0 closes only when Rust/C status parity, stale-generation rejection, Text
classification, closure-scoped borrowing, file budgets, and caller-zero
guards are green. Then the pointer advances to
`CALLABLE-S6C-INSTALLED-CHILD-COMPOSITION-D0`. If a stable residence, trap
consumer, or source/header re-pair is required here, stop and return to the
parent D0 instead of adding a hidden capability.
