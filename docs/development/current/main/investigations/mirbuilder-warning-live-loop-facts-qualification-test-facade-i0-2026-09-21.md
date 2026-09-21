---
Status: closed__2026-09-21__WarningLiveLoopFactsQualificationTestFacade
Task: MIRBUILDER-WARNING-LIVE-LOOP-FACTS-QUALIFICATION-TEST-FACADE-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i31-2026-09-21.md
Implementation permission: true for one cfg(test) live-loop-facts qualification re-export only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I32
---

# Warning cleanup: live-loop-facts qualification test facade

## Six-line brief

```text
Decision: gate the live-loop-facts qualification re-export consumed only by
  terminality tests; keep transaction.rs as its owner.
Source authority + canonical issuer: live_ordered_terminality/transaction.rs;
  cfg(test) export scope is the only moved boundary.
Non-authority: cargo-fix, wildcard imports, loop facts, terminality semantics,
  production route selection, or warning guesses.
Fail-fast boundary: any production consumer, compile error, changed test
  warning, or warning-count mismatch rejects the slice.
Smallest next slice: add cfg(test) to the parent re-export and run the fixed
  quick-profile gates once each.
Non-claims: no loop-route redesign, suppression, or production cutover.
```

## Preconditions and acceptance

I31 records one lib-only unused-import diagnostic at
`src/mir/builder/control_flow/joinir/route_entry/registry/live_ordered_terminality/mod.rs:13`.
The re-export is consumed by test modules in `logical_product.rs` and
`transaction.rs`; production route code does not use it.

Acceptance requires lib warnings to drop from **1,800 to 1,799**, lib-test
warnings to remain **561**, and both fixed commands to exit 0:

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
```

Only the re-export declaration may change. If the consumer classification or
warning count differs, stop and return to design stop.

## Closeout evidence

`#[cfg(test)]` was added to the parent re-export at
`src/mir/builder/control_flow/joinir/route_entry/registry/live_ordered_terminality/mod.rs:13`.
The fixed gates completed sequentially and exited 0:

* `cargo check --profile quick --lib -j4`: 1,799 lib warnings,
  `/tmp/hakorune-warning-i0-live-loop-facts-qualification-lib-20260921.log`
* `cargo test --profile quick --lib --no-run -j4`: 561 lib-test warnings,
  `/tmp/hakorune-warning-i0-live-loop-facts-qualification-lib-test-20260921.log`

No production consumer, route, or terminality semantics changed.
