---
Status: design_stop__post_selection_static_terminal_unattributed
Task: GENERIC-LEGACY-OBSERVATION-FRONT-G0
Date: 2026-09-23
Parent: JOINIR-LOOP-M8-GENERIC-RESIDUAL-S6E
PreviousCard: mir-call-parser-array-push-b3-loopcond-carrier-relation-d2-2026-09-23.md
NextCard: generic-post-selection-static-call-site-identity-d0-2026-09-23.md
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
planner-first `LoopSimpleWhile` tag. Historical Aug-07 observations are not
current evidence. The former compile red at
`b48ecd9fb2c20be0d6eb4e7eec35b00f59300941` was repaired at
`fa80c9ccc2461b58e99a3c6b2c1bb5d52e74e74f`; the quick binary build passed.

At that exact SHA the authorized wrapper exited `1`, not `4`. It printed
`LoopCondContinueOnly` / shadow adoption tags and then stopped at:

```text
[freeze:contract][static-call/legacy-fallback-retired]
owner=StringHelpers method=to_i64 arity=1
```

This is recorded as `PostSelectionStaticCallTerminal`: the planner/shadow
tags prove selection happened earlier in the run, but the terminal diagnostic
does not identify its enclosing callable or `SourceExprSiteV1`. The source
fixture itself is an integer loop; the worker audit found no evidence joining
this static-call terminal to that Loop function. In particular, the known
`StringHelpers.int_to_str/1 Body(0).Initializer(0)` publication row is not
assumed to be this occurrence. The next card resolves only that identity
question through existing transport/ingress owners.

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
| `InLoopTerminal(owner)` | The exact failing callable is correlated to the selected Generic Loop, then exits nonzero at a named Loop carrier/lowering owner. | Keep the case unclassified; task only the named owner. A printed Loop/planner tag with nonzero exit is not enough to establish this relation. |
| `PostSelectionStaticCallTerminal(owner, method, arity)` | Loop/planner tags appeared earlier, then the run stopped in the generic static-call retirement terminal; caller and source site are not reported. | Keep the front unaccepted and resolve the exact source identity/ingress state through the existing owner. Do not attribute it to the Loop callable. |
| `BuildRed(owner)` | The compiler cannot produce the current binary and names a source owner. | Route to a separate current-change compile-repair card; do not label this `ObservationUnavailable` or a smoke result. |
| `ObservationUnavailable` | After a successful current build, the pinned front cannot run because of a concrete environment capability. | Record the missing capability and remain in design stop; do not call it Declined or use another backend. |

Acceptance is one current exact-case result in the table above, with the old
G0/S3 observations explicitly reconciled. The current result is
`PostSelectionStaticCallTerminal(StringHelpers, to_i64, 1)` at
`fa80c9ccc2461b58e99a3c6b2c1bb5d52e74e74f`; caller/site correlation is the
selected next design stop. Do not count this terminal as `LoopReached`, an
in-Loop failure, S6E producer completion, or all-route coverage. After the
front reaches exit 4, serial P1 observation and checked dispositions remain
separate tasks; S6G, M9, and M10 stay closed until their existing owner
contracts pass.

## Worker consultation

The quick-build prerequisite is now closed by the linked repair card. A fresh
worker audit confirmed that `GenericCompatibility` reports only owner/method/
arity, and `Unavailable` may come from a raw-legacy port, absent context,
unlocated compatibility, or non-static cataloged lineage. No worker
correlated this stop to `int_to_str/1` or the printed Loop function. The
Aug-07 S0 schedule remains implemented pre-effect evidence only; it is not
reselected.
