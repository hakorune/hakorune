Status: landed fast caller-zero physical contract; strict writer not connected
Task: MIR-LOOP-COMPARE-RESULT-LEDGER-P0
Date: 2026-08-22
Priority: next bounded physical contract
Parent: MIR-EMIT-CANONICAL-STRICTNESS-D0
PreviousCard: MIR-LOOP-COMPARE-SAME-BLOCK-OPERANDS-P0
NextCard: MIR-LOOP-COMPARE-STRICT-WRITER-P0
---

# Loop Compare result ledger P0

## Six-line brief

```text
Decision: make one owner-bound result key move through map-vacant -> Reserved -> Published, with Poisoned as the terminal state for an uncommitted pending token.
Source authority + canonical issuer: LoopOperationValueLedgerV1 owns result-slot lifecycle; new_for_owner issues the strict owner binding, and the future strict Compare issuer is the sole reservation/commit caller.
Non-authority: post-append map insertion, duplicate checks after MIR mutation, raw ValueId commit input, type_ctx, final verifier, and legacy publish helpers.
Fail-fast boundary: reject an unbound/wrong-owner or occupied slot, then reserve only after all Compare preparation succeeds and before the strict writer append; commit is infallible.
Smallest next slice: add owner-bound full receipt lookup plus one-shot result reservation/commit without connecting a writer or Compare caller.
Non-claims: strict append, destination allocation, Bool publication, operation dispatch, fallback removal, CONNECT0, production I0/R0, and performance.
```

## Fixed boundary

The preceding operand P0 now co-seals a full Published Loop receipt with a
canonical same-session, same-block, uniquely-defined Integer witness. This
card adds only the result lifecycle needed to make the next strict Compare
append failure-atomic:

```text
all fallible Compare preparation
    -> reserve vacant result key
    -> strict append in the next card
    -> Pending.commit(definition) is infallible
```

The ledger is the sole authority for whether a logical result key is map-vacant,
reserved, poisoned, or already published. Strict reservation is available only
from `LoopOperationValueLedgerV1::new_for_owner`; the existing `Default` ledger
remains legacy-unbound and cannot open a strict reservation. The ledger does
not allocate physical `ValueId`s and does not inspect MIR. The strict Compare
issuer owns the ordering and must not call the old `publish()` after an append.

## Allowed files

```text
src/mir/builder/resolved_lowering/loop_recipe_physicalizer/operation_ledger.rs
src/mir/builder/resolved_lowering/loop_recipe_physicalizer/operation_family_tests.rs
src/mir/builder/resolved_lowering/loop_recipe_physicalizer/compare_result_ledger.rs
```

Keep `operation_dispatcher.rs`, `builder_emit.rs`, `operation_emitter.rs`, and
all Compare callers unchanged. The new pending token is private to the
ledger/strict-issuer boundary and must not be returned as a public product.

## State contract

| State | Owner | Allowed operation | Effect |
| --- | --- | --- | --- |
| `Vacant` (map entry absent) | ledger | `reserve_result` | create one pending slot |
| `Reserved` | private pending token | `commit(definition)` | publish exactly once |
| `Published` | ledger | full receipt read | no mutation |
| `Poisoned` | ledger/session discard | reject all reuse | no rollback |

`reserve_result` is the last fallible operation in this card's contract. A
pending token owns the exact slot and carries the key; the slot owns the owner,
class, producer item, and target fixed at reservation. `commit` rechecks
nothing and cannot return a `Result`; a dropped token poisons the slot so the
unpublished function session must be discarded rather than repaired or retried.

The current ledger map has no pending state. Adding the explicit
`Reserved`/`Published`/`Poisoned` slot states and treating an absent map entry
as `Vacant` is a bounded BoxShape/ownership change, not a hidden `Option`
merge. The pending token holds only `&mut` to the exact slot, is non-`Clone`,
and cannot be used to reborrow the ledger while alive. Existing non-strict
operation families may retain their old path until their caller-zero cleanup
card; this P0 must not silently route them through the new pending token.

`commit` consumes a writer-owned `LoopOperationValueDefinitionSourceV1`. That
mechanical trait is only a handoff of the writer's already-prepared physical
value; it is not a source/target/type issuer. The ledger performs no post-append
validation and has no raw-`ValueId` reservation API.

## Acceptance

- unbound/wrong-owner, duplicate, foreign-owner, and already-published result
  keys reject before any slot mutation; an already-reserved key is guarded by
  the affine slot borrow and has a defensive typed arm;
- a successful reservation returns a non-`Clone` private pending token;
- `commit` consumes the token and publishes one full receipt infallibly;
- dropping an uncommitted token marks the slot `Poisoned` and does not restore
  `Vacant`;
- published receipt reads retain owner, key, class, producer item, target, and
  physical value together;
- tests prove ledger state is unchanged on rejected reservation and that
  second commit/reuse is impossible by ownership/type;
- no writer, destination, Bool type, dispatcher, fallback, or production edge
  is added.

## Evidence

```text
compare_result_ledger focused tests: 4 passed
loop_recipe_physicalizer regression suite: 32 passed
cargo check --lib: passed (baseline warnings only)
operation_ledger.rs: 306 lines
compare_result_ledger.rs: 153 lines
strict Compare caller connection: 0
legacy publish/get/receipt callers: unchanged
```

The strict ledger remains caller-zero. The next design task must make the
writer-owned definition source concrete and connect one strict append without
opening dispatcher or production selection.

## NoSafeSlice

Return to the strictness D0 if strict reservation cannot be owner-bound, if
reservation cannot be the last fallible step, if `commit` still needs
duplicate/type/owner checks after append, if a pending token can be cloned or
converted back to `Vacant`, if the ledger must inspect MIR or accept raw
`ValueId` as its commit authority, or if existing operation families require
the new lifecycle before their own bounded caller-zero card.
