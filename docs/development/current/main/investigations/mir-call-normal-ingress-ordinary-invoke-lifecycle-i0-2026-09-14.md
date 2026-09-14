---
Status: closed__fast__2026-09-14
Task: MIR-CALL-NORMAL-INGRESS-ORDINARY-INVOKE-LIFECYCLE-I0
Date: 2026-09-14
Priority: recover the two ParentPassCurrentFail normal-ingress reds at the existing lifecycle owner
Parent: mir-call-normal-pipeline-lifecycle-route-recovery-d0-2026-09-14.md
Implementation permission: true, limited to the selected normal finalization/lifecycle observer and its focused guards; landed in the test owner
NextCard: mir-call-resolver-if-expression-expressivity-d0-2026-09-14.md
---

# Ordinary invoke lifecycle observer recovery

## Decision

The source fixtures are valid and retain their `CanonicalTyped` expectation.
The regression is the old test observer re-entering through the generic
`PublishedMirBackendView::try_new` after `compile_normal()`. Since ordinary
static/free calls are now physical `MirInstruction::Invoke`, that generic view
must continue to stop at `UnsupportedBeforeObject`. The accepted route is the
existing selected published callback in normal finalization.

## Authority and boundary

```text
NormalDefaultRootCatalogLifecycle
 -> FinalizedRootHandoffV1 (CallReturn)
 -> compile_normal_with_published()
 -> bind_finalized_root_handoff()
 -> lifecycle_admission::admit_lifecycle()
 -> issue_lifecycle_physical_program()
 -> issue_lifecycle_compiled_entry_contract()
```

The source-backed normal callable package and normal finalization own the
facts. The lifecycle physical issuer owns the physical program and compiled
entry contract. The generic compatibility view, AST/name/arity inference,
VM, resolver-If work, and the two parent baseline failures are outside this
slice.

## Finite work items

1. Change both focused red tests to observe the selected published callback
   rather than calling `compile_normal()` and then generic `try_new` on the
   result module.
2. In the static `Main.helper/1` fixture, assert `CanonicalTyped`, a
   `CallReturn` handoff, exactly one ordinary call, and the matching
   `OrdinaryI64` physical function.
3. In the top-level `helper/1` fixture, assert the same relation with the
   `helper/1` target.
4. Keep a negative guard that the same module passed to generic
   `PublishedMirBackendView::try_new` remains `UnsupportedBeforeObject` and
   never retries through compatibility.
5. Re-run the two exact regressions and the normal-pipeline parent inventory;
   classify the two `published_consumer_*` failures as unchanged baseline
   debt and record the new receipt before closeout.

## Delete set and nonclaims

Delete the two generic-view re-entry edges from the tests. Do not change the
generic view route, remove fault-frame cleanup, add retry/fallback, alter the
fixtures, or claim all lifecycle instructions, all static/free calls,
publication cutover, legacy retirement, VM parity, Windows proof, or full-lib
green.

## Focused acceptance

The selected owner must make both finite source shapes pass without changing
their expected route. The negative generic-view assertion and the two known
parent baseline reds remain visible. A failure before the selected callback
is a named lifecycle terminal and keeps the row open; it is not reclassified
as compatibility success.

## Receipt

The two tests now consume `compile_normal_with_published` and assert the
selected callback's `CanonicalTyped` route, `CallReturn` handoff, one ordinary
call, and the matching `OrdinaryI64` physical target. They also assert that a
fresh generic view of the same module remains `UnsupportedBeforeObject`.

Focused command, with one quick Cargo process and four build jobs:

```text
CARGO_BUILD_JOBS=4 cargo test --profile quick --lib
  mir::compiler::normal_default_pipeline::tests::normal_ingress_preserves -- --nocapture
2 passed, 0 failed; 536 existing warnings.
```

The normal-pipeline inventory recheck ran 29 tests: 26 passed, 2 failed, and
1 was ignored. The only failures were the unchanged parent baseline
`published_consumer_runs_once_and_propagates_failure_without_retry` and
`published_consumer_does_not_consume_explicit_compatibility`; they remain
known baseline debt and were not weakened. The next design stop is the finite
resolver-If expression inventory.
