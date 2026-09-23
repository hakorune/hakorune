---
Status: design_stop__one_case_observation_front_recheck
Task: GENERIC-LEGACY-OBSERVATION-FRONT-G0
Date: 2026-09-23
Parent: JOINIR-LOOP-M8-GENERIC-RESIDUAL-S6E
PreviousCard: mir-call-parser-array-push-b3-loopcond-carrier-relation-d2-2026-09-23.md
NextCard: same-row__serial_route_observation_only_after_loop_reached
Implementation permission: false; reconcile one existing observation front only. No source, fixture, corpus, or semantic-receipt changes.
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
planner-first `LoopSimpleWhile` tag. Those tags describe this observation
front only; neither selects a production backend or proves a portable Recipe.
The earlier G0 record naming a raw-structured pre-Loop stop must be reconciled
with later source handoff, static-call publication, and callable-loop
source-Facts/Recipe/physical-adapter evidence. In particular,
`StringHelpers.int_to_str/1` reaching its first Loop is a distinct canary and
cannot stand in for this exact case.

The Aug-07 callable handoff S0 schedule already exists in
`src/mir/builder/normal_callable_loop_handoff.rs` and its focused tests. Its
old task file still says `active implementation row`; that status is stale.
S0 is pre-effect evidence only, so do not select it again or treat it as the
missing S6E Recipe/route observation.

## One-case decision procedure

1. Read the existing normalized-case entry, G0 front receipt, fixed smoke,
   and subsequent exact-case evidence. Trace the present front to its first
   terminal; do not infer the result from a newer but different canary.
2. If no current exact-case receipt exists, use only the pinned one-case front
   above. Record either `LoopReached` or the exact pre-Loop owner and terminal.
   Preserve failure as unclassified when the terminal cannot be named.
3. Do not classify the case as a route disposition here. If the front reaches
   Loop, the scheduler may select the next serial
   `GENERIC-LEGACY-ROUTE-OBSERVATION-P1` action. If it does not, keep S6E at
   design stop and name the one owner needed to repair or reselect the front.

## Finite outcomes and acceptance

| Outcome | Evidence | Allowed next step |
| --- | --- | --- |
| `LoopReached` | Current exact-case front exits 4 with one accepted Loop tag; SHA/profile and time are recorded in the owning card. | Select the next serial P1 route observation for the fixed corpus; no broad parallel census. |
| `PreLoopTerminal(owner)` | The exact current front stops before Loop at a named owner. | Keep the case unclassified; task only that owner through its existing authority. |
| `ObservationUnavailable` | No current exact-case run/receipt and the pinned front cannot be run under the selected environment. | Record the concrete missing capability and remain in design stop; do not call it Declined or use another backend. |

Acceptance is one current exact-case result or one exact named pre-Loop
terminal, with the old G0 observation explicitly reconciled. If that is not
available, the missing evidence stays open. Do not count this one-case result
as S6E producer completion or as all-route coverage. After the front is
`LoopReached`, serial P1 observation and checked dispositions remain separate
tasks; S6G, M9, and M10 stay closed until their existing owner contracts pass.

## Worker consultation

Three read-only reviews found: B3 has no safe operation-outcome slice; the
old callable-loop S0 schedule is already implemented and its task frontmatter
is stale; and the authorized S6E successor is the exact strict-shadow VM case
above. The reviews did not execute a build or test and did not issue a
semantic receipt.
