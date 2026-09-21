---
Status: fast__2026-09-21__WarningLiveLoopFactsImportTestFacade
Task: MIRBUILDER-WARNING-LIVE-LOOP-FACTS-IMPORT-TEST-FACADE-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i37-2026-09-21.md
Implementation permission: true for one cfg(test) live-loop-facts import facade only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I38
---

# Warning cleanup: live-loop-facts import test facade

## Six-line brief

```text
Decision: gate the two live-loop-facts imports used only by the test helper;
  keep the production LoopFacts path unchanged.
Source authority + canonical issuer: plan/facts/loop_builder.rs;
  the helper import scope is the only moved boundary.
Non-authority: cargo-fix, wildcard imports, loop facts, terminality semantics,
  production route selection, or warning guesses.
Fail-fast boundary: any production live-facts import consumer, compile error,
  changed test warning, or warning-count mismatch rejects the slice.
Smallest next slice: add cfg(test) to the import and run the fixed
  quick-profile gates once each.
Non-claims: no loop-facts redesign, suppression, or production cutover.
```

## Preconditions and acceptance

I37 records one lib-only unused-import diagnostic containing two names at
`src/mir/builder/control_flow/plan/facts/loop_builder.rs:48`. Both names are
used only by the `#[cfg(test)]` live-facts helper.

Acceptance requires the grouped import warning to drop lib warnings from
**1,795 to 1,794**, lib-test
warnings to remain **561**, and both fixed commands to exit 0:

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
```

Only the import annotation may change. If the consumer classification or
warning count differs, stop and return to design stop.

## Closeout evidence

`#[cfg(test)]` now gates the grouped `LiveLoopFactsV1` and
`bind_live_loop_facts_v1` import in
`src/mir/builder/control_flow/plan/facts/loop_builder.rs:48`. Rustc counts the
group as one diagnostic. The fixed gates completed sequentially and exited 0:

* `cargo check --profile quick --lib -j4`: 1,794 lib warnings,
  `/tmp/hakorune-warning-i0-live-loop-facts-import-lib-20260921.log`
* `cargo test --profile quick --lib --no-run -j4`: 561 lib-test warnings,
  `/tmp/hakorune-warning-i0-live-loop-facts-import-lib-test-20260921.log`

No production facts builder, route consumer, or source authority changed.
