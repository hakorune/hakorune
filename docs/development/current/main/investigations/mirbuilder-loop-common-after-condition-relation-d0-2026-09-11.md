---
Status: closed__DesignStop__LoopCommonAfterConditionRelation__2026-09-11
Task: LOOP-COMMON-AFTER-CONDITION-RELATION-D0
Date: 2026-09-11
Priority: remove Callable-specific condition Read inference from common After
Parent: mirbuilder-callable-loop-argument-phi-correction-i0-2026-09-11
NextCard: LOOP-COMMON-AFTER-CONDITION-RELATION-I0
---

# Common Loop After condition relation D0

## Six-line brief

```text
Decision: common After consumes the existing explicit LoopPhysicalTransferV1::Predicate relation and does not infer a header Read from the first CompareI64.left.
Source authority + canonical issuer: JoinSig-backed physical-transfer binding issues condition/on_true/on_false once; the existing operation Read issuer owns any physical binding receipt needed by a Callable consumer.
Non-authority: recursive_after Compare scanning, first-match order, MIR ValueId numbering, names, route_loop, Generic implicit Read, fallback, retry, or a new semantic receipt.
Fail-fast boundary: missing or inconsistent Predicate condition/branch placement rejects before effect; Callable header-binding mismatch remains a Callable-side validation before Tail completion.
Smallest next slice: remove common After's header_current inference, pass only the explicit relation through its existing continuation, and move the already-required Callable header check to the Callable owner using existing receipts.
Non-claims: Callable source-shape widening, all Loop families, G0 physical lowering, source-to-exe acceptance, backend parity, or whole-MirBuilder completion.
```

## Why implementation is stopped here

`recursive_after.rs` currently searches the first `CompareI64.left` in the
operation rows and requires a matching Read receipt. That makes a common After
consumer depend on the current Callable shape: a computed condition such as
`(i + 1) < limit` has a computed left operand even though the branch relation
itself is explicit and valid.

The existing `LoopPhysicalTransferV1::Predicate { condition, on_true,
on_false }` is already bound from JoinSig by
`physical_transfer::bind_predicate` and validated by common After for
condition presence, type, placement, and duplicates. That relation is the
common authority. Callable-specific header binding belongs at the Callable
source/input or Tail owner, where the existing resolver relation and
dispatched `CanonicalBindingReadReceiptV1` can be cross-checked before
completion.

No code, fixture, fallback, production switch, or new semantic `Verified*` or
`Prepared*` receipt is authorized by this D0. The next implementation row may
open only after the exact transport of the existing relation and the existing
Callable read receipt is named without composing two independent authorities.

## Finite relation state table

| state | authority / issuer | pre-effect behavior | terminal / fallback |
| --- | --- | --- | --- |
| `PredicateReady` | JoinSig `physical_transfer::bind_predicate` and existing prepared physical layout | common After validates the existing condition result and emits the already-bound branch relation | continue to existing After continuation; no fallback |
| `PredicateMissingOrInvalid` | JoinSig/layout binder; common After retains a typed defensive check | reject before operation emission when the source/layout product is malformed | typed reject; no retry or alternate transfer |
| `PredicateDuplicateOrMisplaced` | existing common After validation over layout, dispatch, and value ledger | reject the unpublished function session before Tail completion | typed reject and whole-candidate discard; no repair |
| `CallableHeaderReadReady` | existing prepared Callable read row plus dispatch `Read` receipt | Callable owner verifies binding, owner, and single-predecessor relation before Tail completion | continue to existing Tail/Completion; no new receipt |
| `CallableHeaderReadUnavailableOrMismatch` | Callable source/physical cross-check at the existing Tail boundary | reject before Tail completion and publication | typed reject and whole-candidate discard; no fallback |
| `ComputedConditionLeft` | common Predicate relation remains authoritative; Callable source profile remains its own shape owner | common After does not search for a Read derived from `CompareI64.left` | common path may continue; unsupported Callable shape rejects at its owner |

**Census boundary:** JoinSig transfer binder -> prepared physical layout ->
common After -> Callable Tail/Completion; includes the selected root Predicate
and its existing Callable header-read consumer, and excludes nested-loop
family expansion, G0 physical lowering, backend publication, and source-to-exe.

This Decision closes the materialization-relation design stop. The bounded I0
implementation is authorized in
`mirbuilder-loop-common-after-condition-relation-i0-2026-09-11.md`.

## Worker consultation and evidence

The read-only worker audit on 2026-09-11 reached the same bounded decision:

- `src/mir/builder/resolved_lowering/loop_recipe_physicalizer/recursive_after.rs`
  owns the disputed first-Compare/Read inference.
- `src/mir/loop_recipe_contract/physical_layout.rs` already represents the
  explicit predicate relation, and
  `src/mir/loop_recipe_contract/physical_transfer.rs` binds it from JoinSig.
- `src/mir/builder/resolved_lowering/loop_recipe_physicalizer/tail_completion.rs`
  is the existing Callable-side consumer that validates the After read
  relation before completion.
- Existing production and canary callers are `callable_lowerer.rs`,
  `callable_production_canary_tests.rs`, and `generic_production_canary_tests.rs`;
  no second issuer is needed.

## Bounded acceptance for the future I0

The implementation card must prove all of the following with focused
positive/negative evidence:

1. Common After accepts an explicit Predicate relation whose Compare left side
   is computed, without looking for a Read derived from that left operand.
2. Missing, duplicate, foreign, or mispositioned predicate relation rejects
   before effect and remains typed.
3. Callable validation still rejects a mismatched header binding before Tail
   completion using existing source/dispatch receipts.
4. Generic and Callable existing callers both compile and retain zero
   publication on late failure.

The separate `LOOP-G0-PRODUCTION-TERMINAL-I1` card remains queued after this
design stop. LocalSSA failure cache, measurement-off cost, and view rescan
remain separate follow-ups.
