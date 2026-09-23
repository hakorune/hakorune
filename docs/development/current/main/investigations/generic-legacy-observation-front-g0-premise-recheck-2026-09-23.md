---
Status: superseded__2026-09-23__RetiredVmCompatibilityFront
Task: GENERIC-LEGACY-OBSERVATION-FRONT-G0
Date: 2026-09-23
Parent: JOINIR-LOOP-M8-GENERIC-RESIDUAL-S6E
PreviousCard: mir-call-parser-array-push-b3-loopcond-carrier-relation-d2-2026-09-23.md
NextCard: generic-loop-mainline-source-front-d0-2026-09-23.md
Implementation permission: false. This historical VM observation is outside the selected source-backed compiler lane; no corpus, fixture, route, or semantic-receipt changes are authorized here.
---

# Generic legacy observation front G0 — one-case premise recheck

## Six-line brief

```text
Decision: the pinned `generic_loop_continue_strict_shadow_vm` observation uses explicit retired `--backend vm` and cannot be the selected source-backed S6E acceptance front.
Source authority + canonical issuer: the normalized Generic corpus owner fixes membership; the selected compiler front must use MIR's parser/materializer and establish its actual SourceBacked or Compatibility outcome.
Non-authority: the old Aug-07 S0 task's stale `active implementation row`, the different int_to_str/1 canary, fixture/test names as route proof, VM as production authority, and unobserved cases as Declined.
Fail-fast boundary: preserve VM Keep's named pre-effect terminal and do not repair or reclassify it as a GenericLoop failure.
Smallest next slice: design the one-case MIR source-front observation in `GENERIC-LOOP-MAINLINE-SOURCE-FRONT-D0`.
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

The script historically expected exit `4` and either the strict-shadow Loop
tag or the planner-first `LoopSimpleWhile` tag. It hardcodes `--backend vm`,
which dispatches to explicit Legacy VM Keep and Compatibility AST admission;
that route is retired and is not a current source-backed acceptance front.
Historical Aug-07 observations are not current evidence. The former compile red at
`b48ecd9fb2c20be0d6eb4e7eec35b00f59300941` was repaired at
`fa80c9ccc2461b58e99a3c6b2c1bb5d52e74e74f`; the quick binary build passed.

At that exact SHA the wrapper exited `1`, not `4`. It printed
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

This is now classified as `RetiredVmCompatibilityRoute`, not as
`PostSelectionStaticCallTerminal` blocking the selected Generic source lane.
The caller/site and exact raw-port subtype remain unknown and are not needed
to keep the retired route fail-fast. Do not require the VM wrapper to exit `4`
for the selected compiler lane.

The Aug-07 callable handoff S0 schedule already exists in
`src/mir/builder/normal_callable_loop_handoff.rs` and its focused tests. Its
old task file still says `active implementation row`; that status is stale.
S0 is pre-effect evidence only, so do not select it again or treat it as the
missing S6E Recipe/route observation.

## One-case decision procedure

1. The historical pinned wrapper is classified as retired VM compatibility;
   its result is not a source-backed route disposition.
2. The selected successor is the one-case MIR source-front D0. It must observe
   the exact materializer outcome before selecting any implementation or
   route repair.
3. Keep S6E unaccepted until a selected MIR source-backed front reaches a
   named GenericLoop terminal with same-run evidence. Do not infer from the VM
   wrapper or a different canary.

## Finite outcomes and acceptance

| Outcome | Evidence | Allowed next step |
| --- | --- | --- |
| `LoopReached` | Current exact-case front exits 4 with one accepted Loop tag; SHA/profile and time are recorded in the owning card. | Select the next serial P1 route observation for the fixed corpus; no broad parallel census. |
| `PreLoopTerminal(owner)` | The exact current front stops before the Generic Loop route is selected at a named owner. | Keep the case unclassified; task only that owner through its existing authority. |
| `InLoopTerminal(owner)` | The exact failing callable is correlated to the selected Generic Loop, then exits nonzero at a named Loop carrier/lowering owner. | Keep the case unclassified; task only the named owner. A printed Loop/planner tag with nonzero exit is not enough to establish this relation. |
| `PostSelectionStaticCallTerminal(owner, method, arity)` | Loop/planner tags appeared earlier, then the run stopped in the generic static-call retirement terminal; caller and source site are not reported. | Keep the front unaccepted and resolve the exact source identity/ingress state through the existing owner. Do not attribute it to the Loop callable. |
| `RetiredVmCompatibilityFront` | The fixed wrapper selects explicit VM Keep and its post-macro request seals Compatibility AST. | Preserve the terminal as historical retired-route evidence; do not repair, require exit 4, or count it as selected GenericLoop evidence. |
| `BuildRed(owner)` | The compiler cannot produce the current binary and names a source owner. | Route to a separate current-change compile-repair card; do not label this `ObservationUnavailable` or a smoke result. |
| `ObservationUnavailable` | After a successful current build, the pinned front cannot run because of a concrete environment capability. | Record the missing capability and remain in design stop; do not call it Declined or use another backend. |

The current result is `RetiredVmCompatibilityFront` at
`fa80c9ccc2461b58e99a3c6b2c1bb5d52e74e74f`. Caller/site correlation is not
required for this non-selected route. Do not count it as `LoopReached`, an
in-Loop failure, S6E producer completion, or all-route coverage. The selected
MIR source-front D0 is the next design stop; S6E remains unaccepted and S6G,
M9, and M10 stay closed until their existing owner contracts pass.

## Worker consultation

The quick-build prerequisite is now closed by the linked repair card. A fresh
worker audit confirmed that `GenericCompatibility` reports only owner/method/
arity, and `Unavailable` may come from a raw-legacy port, absent context,
unlocated compatibility, or non-static cataloged lineage. No worker
correlated this stop to `int_to_str/1` or the printed Loop function. The
Aug-07 S0 schedule remains implemented pre-effect evidence only; it is not
reselected.
