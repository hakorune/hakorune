---
Status: closed__2026-09-21__WarningRawLegacyExpressionTestFacade
Task: MIRBUILDER-WARNING-RAW-LEGACY-EXPRESSION-TEST-FACADE-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i35-2026-09-21.md
Implementation permission: true for one cfg(test) raw legacy expression re-export only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I36
---

# Warning cleanup: raw legacy expression test facade

## Six-line brief

```text
Decision: gate the raw legacy expression re-export consumed only by tests;
  keep body/statement ports and the legacy port trait unconditional.
Source authority + canonical issuer: recursive_child_lowering/legacy_port.rs;
  expression export scope is the only moved boundary.
Non-authority: cargo-fix, wildcard imports, legacy semantics, body/statement
  behavior, production lowering, or warning guesses.
Fail-fast boundary: any production expression consumer, compile error, changed
  test warning, or warning-count mismatch rejects the slice.
Smallest next slice: split the parent use and add cfg(test) to expression,
  then run the fixed quick-profile gates once each.
Non-claims: no legacy route redesign, suppression, or production cutover.
```

## Preconditions and acceptance

I35 records one lib-only unused-import diagnostic at
`src/mir/builder/recursive_child_lowering.rs:48`. The expression helper is
consumed only by test code; body/statement helpers remain production-capable.

Acceptance requires lib warnings to drop from **1,797 to 1,796**, lib-test
warnings to remain **561**, and both fixed commands to exit 0:

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
```

Only the expression re-export split may change. If the consumer classification
or warning count differs, stop and return to design stop.

## Closeout evidence

The parent export in `src/mir/builder/recursive_child_lowering.rs:48` was
split so `drive_raw_legacy_expression_v1` is gated by `#[cfg(test)]` while the
body/statement helpers and legacy-port trait remain unconditional. The fixed
gates completed sequentially and exited 0:

* `cargo check --profile quick --lib -j4`: 1,796 lib warnings,
  `/tmp/hakorune-warning-i0-raw-legacy-expression-lib-20260921.log`
* `cargo test --profile quick --lib --no-run -j4`: 561 lib-test warnings,
  `/tmp/hakorune-warning-i0-raw-legacy-expression-lib-test-20260921.log`

No production expression consumer, legacy body/statement owner, or route
policy changed.
