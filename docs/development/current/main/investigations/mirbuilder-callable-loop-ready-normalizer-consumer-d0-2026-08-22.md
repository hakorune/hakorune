---
Status: D0 accepted after worker audit; implementation remains design_stop
Task: MIR-CALLABLE-LOOP-READY-NORMALIZER-CONSUMER-D0
Date: 2026-08-22
Priority: connect one claimed source-facts receipt to one existing structural normalizer boundary without re-observation
Parent: MIR-CALLABLE-LOOP-ORDINARY-READY-D0
PreviousCard: mirbuilder-callable-loop-source-facts-issuer-d0-2026-08-22.md
NextCard: none-until-Decision
---

# Callable Loop Ready normalizer consumer D0

## Six-line brief

Decision: accept one source-bound, zero-effect handoff; the claimed source
receipt is moved into the same existing structural owner that lends the
callback-scoped port, and one HRTB callback receives the receipt view and port
together. The current diagnostic-only port is not an authority by itself.

Source authority + canonical issuer: `CallableGenericLoopSourceFactsIssuerV1`
issues the source-located Facts/Recipe outcome and
`CallableGenericLoopSourceFactsReceiptV1` retains the pre-effect receipt.
The existing `cf_loop_joinir_impl` structural owner is the only owner allowed
to bind that receipt to its structural traversal view. No second semantic
issuer is introduced.

Non-authority: an independent `CallableLoopStructuralPortV1`, a directly
passed `LoopRouteContext`, `route_loop`, registry selection, `PlanLowerer`,
Builder state, AST/name/ordinal/pointer pairing, and any raw `ValueId` or
physical receipt.

Fail-fast boundary: before the callback, validate owner, loop site, exact
parent/condition/body source relation, selected route seal, and retained
pre-effect receipt. A foreign receipt/port pair, missing site, dropped receipt,
second Facts/Recipe/selection run, or callback borrow escape rejects before any
Builder effect.

Smallest next slice: add one private source-bound handoff operation and one
named zero-effect normalizer consumer. It may expose only a callback-scoped
receipt view plus the existing diagnostic structural port. It must not lower,
select a registry route, or call `route_loop`.

Non-claims: ordinary plan normalization, `CorePlan`, `PlanLowerer`, registry
or preflight continuation, Builder/ledger/ValueId/physical/publication work,
raw Ready cutover, fallback/retry, body-only rebind, nested loops, and
production switch.

## Worker-audited decision

The existing structural lease is only a shape seam:

```text
LoopRouteContext
  -> diagnostic-only CallableLoopStructuralPortV1
```

It does not prove that an independently supplied source receipt belongs to the
same loop. Passing the port and receipt as separate arguments would recreate
the pairing risk that the source-facts lane was designed to remove.

The safe relation is scoped and move-only:

```text
CallableGenericLoopSourceFactsReceiptV1
  -> same structural owner receives the receipt
  -> one source-bound structural handoff
  -> HRTB callback(receipt_view, CallableLoopStructuralPortV1)
  -> callback return only
```

The receipt's already-owned condition/body source is allowed to be used once
as the structural owner's input. It is not exposed as a second AST authority,
reparsed, or used to rebuild Facts/Recipe/selection. If the existing context
constructor would reclassify the route, the handoff must use a route-neutral
structural input or remain `NoSafeSlice`; do not hide a second
`choose_route_kind` observation in the source issuer.

`CallableGenericLoopSourceFactsConsumedV1` and
`CallableGenericLoopSourceFactsTerminalConsumerV1` remain caller-zero
historical experiments. The new handoff consumes the retained receipt in
place; it must not create a parallel `Ready`, `Facts`, `Consumed`, or
`Receipt` authority.

## Proposed private boundary

This is a contract sketch, not an implementation authorization:

```rust
fn consume_source_bound_structural_handoff<R>(
    receipt: CallableGenericLoopSourceFactsReceiptV1<'_>,
    use_view: impl for<'view> FnOnce(
        CallableGenericLoopSourceFactsReceiptView<'view>,
        CallableLoopStructuralPortV1<'view>,
    ) -> R,
) -> Result<R, CallableLoopStructuralHandoffRejectV1>;
```

The exact owner and construction point must be the existing
`cf_loop_joinir_impl` structural owner or a private sibling owned by it. The
operation must not accept separately supplied condition/body AST, function
name, route kind, registry rows, or Builder state as a pairing key. If a
receipt view type is needed, it is an opaque borrowed view of already-issued
fields only; it does not issue new semantic facts.

The callback must be unable to return the borrowed view or port. The source
receipt itself is consumed exactly once, and the retained pre-effect receipt
must remain reachable through the handoff until the callback returns.

## Finite state

| State | Owner | Effect | Allowed next step |
| --- | --- | ---: | --- |
| `Ready` | source-facts issuer | 0 | one `claim_all()` move |
| `ClaimedReceipt` | source-facts owner | 0 | source-bound handoff validation |
| `HandoffPrepared` | existing structural owner | 0 | one HRTB callback |
| `BorrowedNormalizerView` | named consumer | 0 | callback return only |
| `RejectedBeforeEffect` | typed handoff validator | 0 | terminal discard |
| `Consumed` | named normalizer consumer | 0 | later plan/lower decision; not this slice |

There is no `Ready -> lower_loop_or_freeze_v1` transition in this state
machine. The current raw edge remains explicitly caller-zero for this design
card; it is not silently reclassified as a successful consumer.

## Implementation task sequence after Decision

1. **Census the exact owner and relation.** Locate the one existing
   `cf_loop_joinir_impl` context owner, the one source receipt ingress, and the
   exact source lineage fields that can prove same-loop membership. Record
   caller counts for `route_loop`, `lower_loop_or_freeze_v1`, registry
   selection, and `PlanLowerer` from the new consumer.
2. **If the relation is closed, add the private handoff.** Move the existing
   receipt, validate owner/sites/selection/pre-effect, and mint the existing
   callback-scoped structural port inside the same owner. Do not add a
   semantic `Verified*`/`Prepared*` product merely to transport these fields.
3. **Add one zero-effect named consumer.** Consume the callback view and return
   an owned observation/terminal result. It must not invoke route planning,
   registry selection, `route_loop`, `PlanLowerer`, or Builder mutation.
4. **Add focused evidence and a reusable guard.** Prove positive same-owner
   handoff, foreign owner/site rejection, missing-site rejection, receipt
   retention, callback non-escape, and zero Builder effect. Prove exactly one
   source-facts/selection extraction and zero old-route calls from the new
   consumer.
5. **Close the card before opening physical work.** Update the module README
   and current pointer. Keep raw Ready production cutover, plan/lower,
   fallback retirement, and publication as a later Decision.

If step 1 cannot prove the relation without a second Context, direct AST
pairing, pointer/name/ordinal keys, or route reclassification, stop and record
`NoSafeSlice` instead of adding an adapter.

## Acceptance / guards

```text
source receipt -> source-bound handoff = 1 named path
source receipt -> lower_loop_or_freeze_v1 = 0
source receipt -> route_loop = 0
source consumer -> registry selection/preflight = 0
source consumer -> PlanLowerer = 0
source consumer -> Builder/ValueId/physical = 0
Facts extraction = 1
Recipe/selection extraction = 1
pre_effect discard = 0
foreign receipt/port pairing = 0
AST/source borrow escapes callback = 0
terminal ConsumedV1 production callers = 0
old raw Ready edge remains explicitly unclaimed until later R0
```

Positive evidence must show that the same source receipt, not a reconstructed
AST or a separately supplied context, reaches the callback. Negative evidence
must show no effect and no fallback on every reject.

## NoSafeSlice / non-claims

Remain in `design_stop` if any implementation needs:

```text
second LoopRouteContext built from independent inputs
direct &LoopRouteContext passed beside an unbound receipt
AST pointer/name/ordinal/digest pairing
route_loop or registry re-selection
Facts/Recipe/selection re-extraction
dropped pre-effect receipt
borrowed view escaping the HRTB callback
Builder or physical effect before handoff completion
parallel source-facts or terminal authority
```

This card is a handoff task, not a claim that the ordinary Loop consumer,
physical lowerer, publication, fallback retirement, or production switch is
complete.
