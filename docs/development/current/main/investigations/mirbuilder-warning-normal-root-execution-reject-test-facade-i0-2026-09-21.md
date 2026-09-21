---
Status: closed__2026-09-21__WarningNormalRootExecutionRejectTestFacade
Task: MIRBUILDER-WARNING-NORMAL-ROOT-EXECUTION-REJECT-TEST-FACADE-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i33-2026-09-21.md
Implementation permission: true for one cfg(test) normal-root reject re-export only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I34
---

# Warning cleanup: normal-root execution reject test facade

## Six-line brief

```text
Decision: gate the normal-root execution reject re-export consumed only by
  tests; keep the successful consumer in the production export.
Source authority + canonical issuer: normal_root_execution/consumer.rs;
  the split export is the only moved boundary.
Non-authority: cargo-fix, wildcard imports, consumer semantics, route policy,
  production lowering, or warning guesses.
Fail-fast boundary: any production reject-type consumer, compile error, changed
  test warning, or warning-count mismatch rejects the slice.
Smallest next slice: split the parent use and add cfg(test) to the reject type,
  then run the fixed quick-profile gates once each.
Non-claims: no normal-root redesign, suppression, or production cutover.
```

## Preconditions and acceptance

I33 records one lib-only unused-import diagnostic at
`src/mir/builder.rs:190`. The reject type is consumed only by test code; the
successful `NormalRootExecutionConsumerV1` export has a production consumer
and remains unconditional.

Acceptance requires lib warnings to drop from **1,798 to 1,797**, lib-test
warnings to remain **561**, and both fixed commands to exit 0:

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
```

Only the parent re-export split may change. If the consumer classification or
warning count differs, stop and return to design stop.

## Closeout evidence

The parent export in `src/mir/builder.rs:190` was split so
`NormalRootExecutionConsumerRejectV1` is gated by `#[cfg(test)]` while
`NormalRootExecutionConsumerV1` remains unconditional. The fixed gates
completed sequentially and exited 0:

* `cargo check --profile quick --lib -j4`: 1,797 lib warnings,
  `/tmp/hakorune-warning-i0-normal-root-execution-reject-lib-20260921.log`
* `cargo test --profile quick --lib --no-run -j4`: 561 lib-test warnings,
  `/tmp/hakorune-warning-i0-normal-root-execution-reject-lib-test-20260921.log`

No production consumer, normal-root owner, or route policy changed.
