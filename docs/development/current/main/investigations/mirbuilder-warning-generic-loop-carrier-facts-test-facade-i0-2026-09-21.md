---
Status: closed__2026-09-21__WarningGenericLoopCarrierFactsTestFacade
Task: MIRBUILDER-WARNING-GENERIC-LOOP-CARRIER-FACTS-TEST-FACADE-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i38-2026-09-21.md
Implementation permission: true for two cfg(test) generic-loop carrier facts re-exports only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I39
---

# Warning cleanup: generic-loop carrier facts test facade

## Six-line brief

```text
Decision: gate the two generic-loop carrier facts re-exports consumed only by
  tests; keep the generic-loop owner modules and production paths unchanged.
Source authority + canonical issuer: plan/generic_loop/facts and facts_types;
  parent facts export scope is the only moved boundary.
Non-authority: cargo-fix, wildcard imports, carrier semantics, route policy,
  production lowering, or warning guesses.
Fail-fast boundary: any production parent-facade consumer, compile error,
  changed test warning, or warning-count mismatch rejects the slice.
Smallest next slice: add cfg(test) to both parent re-exports and run the fixed
  quick-profile gates once each.
Non-claims: no generic-loop redesign, suppression, or production cutover.
```

## Preconditions and acceptance

I38 records two lib-only unused-import diagnostics at
`src/mir/builder/control_flow/plan/facts/mod.rs:46-47`. The parent names are
consumed only by test fixtures; generic-loop production code uses its owner
modules directly.

Acceptance requires lib warnings to drop from **1,794 to 1,793**, lib-test
warnings to remain **561**, and both fixed commands to exit 0:

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
```

Only the two re-export annotations may change. If the consumer classification
or warning count differs, stop and return to design stop.

## Closeout evidence

The two parent exports in
`src/mir/builder/control_flow/plan/facts/mod.rs:46-47` are now gated by
`#[cfg(test)]`. The fixed gates completed sequentially and exited 0:

* `cargo check --profile quick --lib -j4`: 1,793 lib warnings,
  `/tmp/hakorune-warning-i0-generic-loop-carrier-facts-lib-20260921.log`
* `cargo test --profile quick --lib --no-run -j4`: 561 lib-test warnings,
  `/tmp/hakorune-warning-i0-generic-loop-carrier-facts-lib-test-20260921.log`

No production generic-loop owner, facts type, or route consumer changed.
