---
Status: closed__Implementation__LoopCommonAfterConditionRelation__2026-09-11
Task: LOOP-COMMON-AFTER-CONDITION-RELATION-I0
Date: 2026-09-11
Priority: consume explicit Predicate relation in common After and isolate Callable header validation
Parent: mirbuilder-loop-common-after-condition-relation-d0-2026-09-11
NextCard: LOOP-G0-PRODUCTION-TERMINAL-I1
---

# Common Loop After condition relation I0

## Six-line brief

```text
Decision: remove recursive_after's first CompareI64.left -> Read inference; common After consumes only the existing explicit Predicate relation, while Callable Tail receives and validates its existing header Read receipt.
Source authority + canonical issuer: JoinSig physical-transfer binding owns Predicate condition/targets; the existing prepared Callable read row and operation dispatcher own the canonical header Read receipt.
Non-authority: Compare row first-match order, MIR ValueId numbering, names, route_loop, Generic implicit Read, fallback, retry, and a new semantic receipt/type/layer.
Fail-fast boundary: malformed Predicate relations reject at layout/common validation; missing or mismatched Callable header receipt rejects before Tail completion, publication, or retry.
Smallest next slice: remove header_current from common After's continuation, capture the existing Callable header-read relation at the Callable owner, pass it explicitly to Tail, and prove computed-left common After plus existing Callable/Generic discard behavior.
Non-claims: Callable source-shape widening, all Loop families, G0 physical lowering, source-to-exe acceptance, backend parity, or whole-MirBuilder completion.
```

## Authorized implementation cells

1. `recursive_after.rs`: remove only the `CompareI64.left`/Read scan and the
   `header_current` field/accessor. Keep explicit Predicate condition,
   placement, type, target, and duplicate validation.
2. `callable_lowerer.rs` and `tail_completion.rs`: use the existing prepared
   read row and `LoopOperationDispatchReceiptV1::Read` canonical receipt to
   validate and pass the Callable header relation explicitly. Do not add a
   new semantic product or physical owner.
3. Existing Callable and Generic production canaries: add computed-left
   common-After evidence, retain exact PHI and late-discard assertions, and
   add a typed missing/mismatched Callable header negative if the existing
   mutation seam can express it without a new fixture authority.
4. Update the loop physicalizer module README, this card, and the linked
   common physical-demand reference only for this ownership split.

## Finite acceptance

| state | required evidence |
| --- | --- |
| `PredicateReady` | common After emits from the explicit layout Predicate without scanning Compare left operands |
| `ComputedConditionLeft` | a focused common test proves `(i + 1) < limit` does not require a Read of the computed left value |
| `PredicateMissingOrInvalid` | typed pre-effect rejection remains green with no fallback |
| `CallableHeaderReadReady` | existing Callable canary reaches Tail using the explicit canonical Read receipt |
| `CallableHeaderReadUnavailableOrMismatch` | typed reject occurs before Tail completion and unpublished state is discarded |
| `LateFailure` | existing Callable/Generic fresh-session discard tests remain green |

**Census boundary:** existing `recursive_after` callers -> existing Callable
and Generic canaries -> existing Tail/Completion consumer; includes only the
selected common After/header-read split and excludes G0 I1, other Loop shapes,
backend parity, and source-to-exe.

## Source and ownership constraints

The common module must not rediscover source meaning from MIR operation order.
The Callable owner may cross-check the already-prepared
`PreparedLoopReadBindingRowV1` source binding/result against the dispatched
`CanonicalBindingReadReceiptV1`; that cross-check is a consumer of existing
products, not a new issuer. The current Callable profile has one declared
header `CompareI64`; its fixed condition-to-Read relation is validated at this
profile boundary and is not a common After selection rule. Any missing relation is a typed unpublished
rejection. No `Option` default, fallback, retry, or test-only constructor may
bridge the gap.

## Implementation evidence

The I0 slice is closed with the following bounded changes:

- `recursive_after.rs` no longer carries `header_current` and validates only
  the explicit `LoopPhysicalTransferV1::Predicate` condition/target relation.
- `tail_completion.rs` issues no new semantic product; it cross-checks the
  existing prepared Callable Read row and dispatched canonical Read receipt,
  with typed missing, duplicate, and binding-mismatch rejects.
- `callable_lowerer.rs` passes the existing header Read receipt explicitly to
  Tail/Completion. The Callable canary also asserts that the body Add result is
  an exact header-PHI backedge input.
- The common computed-left guard and Callable missing/duplicate Read mutation
  guards are green; each mutation discards the unpublished session.

Focused evidence on 2026-09-11:

```text
CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0 cargo test -j1 callable_production_canary --lib  -> 3 passed
CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0 cargo test -j1 recursive_after --lib               -> 2 passed
CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0 cargo test -j1 generic_production_canary --lib    -> 2 passed
CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0 cargo test -j1 loop_physical_prepare --lib        -> 7 passed
git diff --check                                                                                -> green
```

The whole-workspace `cargo fmt -- --check` remains red on pre-existing
unrelated formatting outside this slice; the changed Rust files were checked
without applying a broad formatting rewrite. G0 production lowering and
source-to-exe acceptance remain the next cards, while LocalSSA cache,
measurement-off cost, and view re-scan remain separate follow-ups.
