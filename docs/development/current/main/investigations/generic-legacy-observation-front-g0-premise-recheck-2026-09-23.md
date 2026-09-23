---
Status: design_stop__premise_reconciled_build_red_blocks_front
Task: GENERIC-LEGACY-OBSERVATION-FRONT-G0
Date: 2026-09-23
Parent: JOINIR-LOOP-M8-GENERIC-RESIDUAL-S6E
PreviousCard: mir-call-parser-array-push-b3-loopcond-carrier-relation-d2-2026-09-23.md
NextCard: same-row__serial_route_observation_only_after_loop_reached
Implementation permission: false for this observation card. The selected prerequisite compile repair is owned by `generic-loop-source-facts-accessor-compile-repair-i0-2026-09-23.md`; no corpus, fixture, route, or semantic-receipt changes are authorized here.
---

# Generic legacy observation front G0 — one-case premise recheck

## Six-line brief

```text
Decision: after B3 D2 reached family-local NoSafeSlice, return to the ordered M8 queue and recheck only the existing S6E observation front.
Source authority + canonical issuer: the normalized Generic corpus owner fixes membership; GENERIC-LEGACY-OBSERVATION-FRONT-G0 owns the fixed one-case direct-VM front and its terminal classification.
Non-authority: the old Aug-07 S0 task's stale `active implementation row`, the different int_to_str/1 canary, fixture/test names as route proof, VM as production authority, and unobserved cases as Declined.
Fail-fast boundary: the pinned case must reach Loop or retain its exact named pre-Loop terminal; no route/disposition row is issued from a failed or stale front.
Smallest next slice: reconcile current code, existing receipts, and the one pinned smoke for `generic_loop_continue_strict_shadow_vm`; run only that existing one-case front if no current exact-case evidence exists.
Non-claims: no 19-case census, S6E producer, new semantic receipt, source/fixture change, production selection, parity, or deletion.
```

## Fixed boundary

This task uses the existing normalized corpus and front. It does not rebuild
the inventory. The only case is:

```text
case       = generic_loop_continue_strict_shadow_vm
fixture    = apps/tests/phase29ca_generic_loop_continue_min.hako
profile    = vm-strict-planner-direct-v1
front      = tools/smokes/v2/profiles/integration/joinir/generic_loop_continue_strict_shadow_vm.sh
```

The script expects exit `4` and either the strict-shadow Loop tag or the
planner-first `LoopSimpleWhile` tag. The latest immutable exact-case evidence
is the Aug-07 S3 receipt: Generic Loop selection reaches carrier
representation, then exits `1` at `MissingTransientType { init: ValueId(3) }`.
The later note at `docs/reference/mir/generic-loop-stage-matrix.md:1267-1275` reports planner/
shadow tags before the same error. Neither is a passing front or a pre-Loop
terminal. Both predate the Sep-22/23 source and carrier changes and have no
tested current HEAD SHA.

At current HEAD `b48ecd9fb2c20be0d6eb4e7eec35b00f59300941`, the authorized
fresh front cannot yet run: `CARGO_BUILD_JOBS=4 cargo build --profile quick
--bin hakorune` exits 101 with four E0599 errors because production source
views call the `#[cfg(test)]`-only `CallableGenericLoopSourceFactsReceiptV1::pre_effect`
helper. This is classified as a current-change compile red and is selected
first by the linked I0 repair card. It is not a G0 runtime outcome.

When build recovery lands, re-run only the pinned direct-VM case.
`StringHelpers.int_to_str/1` reaching its first Loop remains a distinct
canary and cannot stand in for this exact case.

The Aug-07 callable handoff S0 schedule already exists in
`src/mir/builder/normal_callable_loop_handoff.rs` and its focused tests. Its
old task file still says `active implementation row`; that status is stale.
S0 is pre-effect evidence only, so do not select it again or treat it as the
missing S6E Recipe/route observation.

## One-case decision procedure

1. Read the existing normalized-case entry, G0 front receipt, fixed smoke,
   and subsequent exact-case evidence. Trace the present front to its first
   terminal; do not infer the result from a newer but different canary.
2. After the selected compile-repair card passes, use only the pinned one-case
   front above. Record its exact SHA, command, exit status, and first owner.
   The old receipts are historical and do not substitute for this run.
3. Do not classify the case as a route disposition here. If the front reaches
   Loop, the scheduler may select the next serial
   `GENERIC-LEGACY-ROUTE-OBSERVATION-P1` action. If it does not, keep S6E at
   design stop and name the one owner needed to repair or reselect the front.

## Finite outcomes and acceptance

| Outcome | Evidence | Allowed next step |
| --- | --- | --- |
| `LoopReached` | Current exact-case front exits 4 with one accepted Loop tag; SHA/profile and time are recorded in the owning card. | Select the next serial P1 route observation for the fixed corpus; no broad parallel census. |
| `PreLoopTerminal(owner)` | The exact current front stops before the Generic Loop route is selected at a named owner. | Keep the case unclassified; task only that owner through its existing authority. |
| `InLoopTerminal(owner)` | The exact current front selects/enters Generic Loop, then exits nonzero at a named carrier or lowering owner. | Keep the case unclassified; task only the named owner. A printed Loop/planner tag with nonzero exit is not `LoopReached`. |
| `BuildRed(owner)` | The compiler cannot produce the current binary and names a source owner. | Route to a separate current-change compile-repair card; do not label this `ObservationUnavailable` or a smoke result. |
| `ObservationUnavailable` | After a successful current build, the pinned front cannot run because of a concrete environment capability. | Record the missing capability and remain in design stop; do not call it Declined or use another backend. |

Acceptance is one current exact-case result in the table above, with the old
G0/S3 observations explicitly reconciled. The current result is presently
`BuildRed(owner)` and the selected next card repairs only that compile red.
Do not count an in-loop error or a printed tag as LoopReached, or this one-case
result as S6E producer completion/all-route coverage. After the front is
`LoopReached`, serial P1 observation and checked dispositions remain separate
tasks; S6G, M9, and M10 stay closed until their existing owner contracts pass.

## Worker consultation

Two current read-only reviews agree that the old pre-Loop premise is
superseded by the S3 in-loop carrier failure, but no exact-case receipt is
current for HEAD. Both also found that the card needed a distinct
`InLoopTerminal(owner)` outcome. The current quick build then exposed the
separate `BuildRed(owner)` prerequisite documented above. The Aug-07 S0
schedule remains implemented pre-effect evidence only; it is not reselected.
