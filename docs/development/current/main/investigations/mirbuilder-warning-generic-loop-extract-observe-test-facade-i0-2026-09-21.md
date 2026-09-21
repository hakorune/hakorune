---
Status: closed__2026-09-21__WarningGenericLoopExtractObserveTestFacade
Task: MIRBUILDER-WARNING-GENERIC-LOOP-EXTRACT-OBSERVE-TEST-FACADE-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i39-2026-09-21.md
Implementation permission: true for one cfg(test) generic-loop extract observe re-export only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I40
---

# Warning cleanup: generic-loop extract observe test facade

## Six-line brief

```text
Decision: gate the generic-loop extract observe re-export with no callers;
  keep collection and v1 extraction owners unchanged.
Source authority + canonical issuer: plan/generic_loop/facts/extract/collection.rs;
  the parent extract export is the only moved boundary.
Non-authority: cargo-fix, wildcard imports, carrier semantics, v1 extraction,
  production lowering, or warning guesses.
Fail-fast boundary: any production parent-facade consumer, compile error,
  changed test warning, or warning-count mismatch rejects the slice.
Smallest next slice: add cfg(test) to the parent re-export and run the fixed
  quick-profile gates once each.
Non-claims: no generic-loop redesign, suppression, or production cutover.
```

## Preconditions and acceptance

I39 records one lib-only unused-import diagnostic at
`src/mir/builder/control_flow/plan/generic_loop/facts/extract/mod.rs:13`.
Production v1 uses the collection owner directly; the parent extract export
has no caller.

Acceptance requires lib warnings to drop from **1,793 to 1,792**, lib-test
warnings to remain **561**, and both fixed commands to exit 0:

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
```

Only the parent re-export annotation may change. If the consumer
classification or warning count differs, stop and return to design stop.


## Closeout evidence

The sole permitted source edit was applied:
`src/mir/builder/control_flow/plan/generic_loop/facts/extract/mod.rs:13` now
exports `observe_generic_loop_carrier_observation` only under `#[cfg(test)]`.
The fixed gates completed sequentially with exit 0:

* lib: 1,792 warnings — `/tmp/hakorune-warning-i0-generic-loop-extract-observe-lib-20260921.log`
* lib test: 561 warnings — `/tmp/hakorune-warning-i0-generic-loop-extract-observe-lib-test-20260921.log`

No production owner, extraction path, or semantic route changed.
