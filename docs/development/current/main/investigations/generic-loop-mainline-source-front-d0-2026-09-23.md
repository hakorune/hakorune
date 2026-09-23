---
Status: design_stop__mainline_source_front_unproven
Task: GENERIC-LOOP-MAINLINE-SOURCE-FRONT-D0
Date: 2026-09-23
Parent: generic-post-selection-static-call-site-identity-d0-2026-09-23.md
PreviousCard: generic-legacy-observation-front-g0-premise-recheck-2026-09-23.md
NextCard: none__design_decision_required
Implementation permission: false; one exact MIR-front/source-admission design only. No code, test, fixture, smoke, route, receipt, production switch, or fallback change.
---

# Generic Loop mainline source-front D0

## Six-line brief

```text
Decision: the failed fixed G0 wrapper is a retired VM Compatibility observation; reselect one front on the canonical MIR entry and prove the actual materializer outcome before calling it source-backed acceptance.
Source authority + canonical issuer: `prepare_normal_source_with_imports` supplies code/imports/merged lineage; `materialize_normal_callable_program_with_identity_and_lineage_v1` issues either `SourceBacked` or explicit `Compatibility`; only the former enters the callable source lifecycle and existing GenericLoop Facts/Recipe owners.
Non-authority: `--backend mir` by itself, the retired VM result, Loop tags without same-run route evidence, the old VM exit-4 assertion, direct test products without matching prepared bytes/lineage, or the separate `generic_g0` helper profile.
Fail-fast boundary: `Compatibility(origin)` stays on its explicit branch; do not retry it as source-backed or repair the retired VM terminal.
Smallest next slice: for the exact existing continue fixture, identify an existing-owner evidence path that observes materializer outcome, source admission, and the first GenericLoop terminal on the same prepared input.
Non-claims: no new language shape, GenericLoop semantic change, caller/site attribution for the retired VM call, production selection, backend parity, or old-edge deletion.
```

## Why this is the next design stop

The pinned wrapper
`tools/smokes/v2/profiles/integration/joinir/generic_loop_continue_strict_shadow_vm.sh`
hardcodes `--backend vm`. That dispatch selects explicit Legacy VM Keep;
`for_vm_keep_post_macro` seals a Compatibility AST root. Its
`StringHelpers.to_i64/1` terminal is therefore outside the selected source
frontier. The prior card records this as
[`RetiredVmCompatibilityRoute`](generic-post-selection-static-call-site-identity-d0-2026-09-23.md).

MIR is the selected compiler entry, but it is not synonymous with
`SourceBacked`. [`execute_mir_mode`](../../../../../src/runner/modes/mir.rs)
prepares normal source and lineage, then branches on
`NormalCallableMaterializationOutcomeV1`: `SourceBacked` enters
`for_mir_mode_callable_source`; `Compatibility` enters
`for_mir_mode_compatibility`. No existing observation found so far proves which
branch this exact fixture takes through the production preparation path.

The exact candidate is unchanged:

```text
fixture: apps/tests/phase29ca_generic_loop_continue_min.hako
source shape: static box Main; main; integer GenericLoop with continue; return i
candidate command: target/quick/hakorune --backend mir <fixture>
```

The candidate command is not yet an acceptance command. The existing VM
wrapper's expected exit and tags cannot be copied: MIR route admission and its
interpreter result must be established independently.

## Finite outcomes

| Result | Required evidence | Consequence |
| --- | --- | --- |
| `SourceBackedFrontObservable` | One existing owner path sees the exact prepared bytes and lineage, observes `SourceBacked`, carries the source-admission witness, and identifies the first same-run GenericLoop terminal or selected owner. | Select one fast slice to connect that evidence to a MIR-specific acceptance invocation. Keep the source/API map unchanged. |
| `CompatibilityFront(reason)` | The exact materializer outcome is `Compatibility` with its existing reason. | Do not count the fixture as source-backed Generic acceptance; return to the family scheduler for one already-inventoried front. |
| `NoExistingObserver(owner, missing_product)` | No existing test/trace can observe the materializer outcome and selected GenericLoop terminal for the same prepared input without adding instrumentation or a new semantic receipt. | Close as NoSafeSlice; name the missing observation owner and return to the family scheduler. Do not guess from backend name or tags. |

## Audit boundary and acceptance

```text
includes: exact continue fixture -> normal source preparation -> merged root lineage -> callable materializer outcome -> source admission -> first GenericLoop terminal/owner
excludes: other corpus cases, VM Keep/VM-Hako, LLVM, other GenericLoop shapes, production selection, S6E producer, M9/M10 cutover, parity, and retirement
```

Audit only existing code and receipts. In particular, compare the runner path
in `src/runner/modes/mir.rs` with the existing preparation/materializer tests
in `src/runner/modes/common_util/source_hint_normal_tests.rs` and
`normal_callable.rs`. If a test observes only a hand-built source product,
different bytes, or a lineage-free parse, it does not prove the production
front. Do not edit those tests in this D0.

Acceptance is one finite outcome above, exact owner/API references, and one
successor decision. If the observable path exists, the successor must keep the
same fixture and define MIR-specific route/result assertions. If it does not,
record the single missing owner and stop. Do not add a diagnostic or broaden
the corpus to make the outcome look complete.

## Worker consultation

Two read-only audits confirmed that `--backend mir` can still choose either
materializer outcome and that no current receipt proves the exact fixture's
MIR outcome. They found no exact fixture evidence tying the earlier VM
`to_i64/1` terminal to the GenericLoop. No code, test, fixture, or build was
run or changed by the workers.
