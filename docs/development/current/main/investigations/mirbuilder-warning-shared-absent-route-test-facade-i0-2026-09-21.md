---
Status: closed__2026-09-21__WarningSharedAbsentRouteTestFacade
Task: MIRBUILDER-WARNING-SHARED-ABSENT-ROUTE-TEST-FACADE-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i36-2026-09-21.md
Implementation permission: true for one cfg(test) shared-absent route re-export only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I37
---

# Warning cleanup: shared-absent route test facade

## Six-line brief

```text
Decision: gate the route-registry parent re-export consumed by no production
  caller; keep the direct types owner and all route consumers unchanged.
Source authority + canonical issuer: route_entry/registry/types.rs;
  parent registry export scope is the only moved boundary.
Non-authority: cargo-fix, wildcard imports, route semantics, handlers,
  execution witness, production lowering, or warning guesses.
Fail-fast boundary: any production parent-export consumer, compile error,
  changed test warning, or warning-count mismatch rejects the slice.
Smallest next slice: add cfg(test) to the parent re-export and run the fixed
  quick-profile gates once each.
Non-claims: no route-policy redesign, suppression, or production cutover.
```

## Preconditions and acceptance

I36 records one lib-only unused-import diagnostic at
`src/mir/builder/control_flow/joinir/route_entry/registry/mod.rs:53`. The
production handlers and execution witness import the type owner directly;
the parent re-export has no production caller.

Acceptance requires lib warnings to drop from **1,796 to 1,795**, lib-test
warnings to remain **561**, and both fixed commands to exit 0:

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
```

Only the parent re-export declaration may change. If the consumer
classification or warning count differs, stop and return to design stop.

## Closeout evidence

The parent export in
`src/mir/builder/control_flow/joinir/route_entry/registry/mod.rs:53` is now
gated by `#[cfg(test)]`; the `Entry` and `RouterEnv` exports remain
unconditional. The fixed gates completed sequentially and exited 0:

* `cargo check --profile quick --lib -j4`: 1,795 lib warnings,
  `/tmp/hakorune-warning-i0-shared-absent-route-lib-20260921.log`
* `cargo test --profile quick --lib --no-run -j4`: 561 lib-test warnings,
  `/tmp/hakorune-warning-i0-shared-absent-route-lib-test-20260921.log`

No production route consumer, handler, or policy changed.
