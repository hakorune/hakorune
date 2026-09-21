---
Status: closed__2026-09-21__WarningLoopPhiIndexCfg
Task: MIRBUILDER-WARNING-LOOP-PHI-INDEX-CFG-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i93-2026-09-21.md
Implementation permission: true for the mixed-cfg loop-index warning only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I94
---

# Warning cleanup: LoopPhi materializer mixed-cfg index

## Six-line brief

```text
Decision: rename the production-unused loop index to `_index`; retain its
test-only failure-injection comparison and all PHI transaction behavior.
Source authority + canonical issuer: loop_phi_materializer.rs implementation
and the I93 quick-profile warning census.
Non-authority: PHI layout, transaction lifecycle, failure injection semantics,
warning suppression, or a new physical/semantic receipt.
Fail-fast boundary: any focused red, changed test behavior, or warning-count
mismatch rejects the rename.
Smallest next slice: one identifier rename, loop_phi_materializer tests, and
the fixed warning refresh.
Non-claims: no LoopBreak retirement, route switch, source expansion, or ABI
change.
```

## Caller/cfg census

I93 records the current warning surface as lib **1,711** and lib-test **552**.
At `src/mir/builder/control_flow/plan/loop_phi_materializer.rs:467`, `index`
is consumed only by the `#[cfg(test)]` `fail_after` comparison. The non-test
build still enumerates the same rows but has no failure-injection branch, so
the identifier is unused there. The rename keeps the test branch's value and
comparison intact and removes only the mixed-cfg diagnostic.

## Acceptance

Run sequentially with one Cargo process:

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
cargo test --profile quick --lib mir::builder::control_flow::plan::loop_phi_materializer::tests
```

The focused filter must execute nonzero tests and pass. The stable refresh
must reduce lib warnings from **1,711** to **1,710**, keep lib-test at **552**,
and preserve the provisional-failure rollback and deterministic-reuse tests.
Run fmt, diff, and the current-state pointer guard before closeout. Do not
touch the compatibility route or add an allow.

## Execution evidence

The loop index is now `_index`; the `#[cfg(test)]` failure-injection branch
still compares the same value. Sequential acceptance passed: lib check
**1,710** warnings, lib-test build **552**, and the focused LoopPhi filter
**36/36**. `cargo fmt --all -- --check`, `git diff --check`, and the current
state pointer guard also passed. No PHI transaction, rollback, route, or
production behavior changed.
