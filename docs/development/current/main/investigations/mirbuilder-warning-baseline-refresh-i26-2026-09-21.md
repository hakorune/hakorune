---
Status: closed__2026-09-21__WarningBaselineRefreshI26__DirectStaticPhysicalInputRowTestFacade
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I26
Date: 2026-09-21
Parent: mirbuilder-warning-direct-static-physical-input-test-facade-i0-2026-09-21.md
Implementation permission: true for one cfg(test) direct-static physical-input row re-export only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I27
---

# MirBuilder warning baseline refresh I26

## Six-line brief

```text
Decision: refresh both warning surfaces after the direct-static physical-input
  test-facade cohort before selecting another cleanup row.
Source authority + canonical issuer: fixed quick-profile Cargo diagnostics and
  the checked-in warning classification policy.
Non-authority: historical counts, cargo-fix, blanket allow, visibility edits,
  dead-code guesses, or semantic interpretation of lint output.
Fail-fast boundary: command drift, new failure name, unclassified test-only
  reference, or a warning family outside the inventory stops selection.
Smallest next slice: compare current lib/lib-test inventories against
  1,805/561 and select at most one caller-zero import cohort.
Non-claims: no broad warning deletion, suppression, semantic refactor, or
  production route change.
```

## Acceptance

Run `cargo check --profile quick --lib -j4` and
`cargo test --profile quick --lib --no-run -j4` once each. Record lint,
file:line, owner, and production/test/compat/generated role, then select one
cohort or write `NoSafeSlice`. Keep dead-code/private-interface rows with
their owners. No code edit is permitted until the selection is recorded.

## Selection evidence

The fixed commands completed sequentially with the current warning baseline:

| surface | warnings | evidence |
| --- | ---: | --- |
| lib | 1,805 | `/tmp/hakorune-warning-i26-lib-20260921.log` |
| lib test | 561 | `/tmp/hakorune-warning-i26-lib-test-20260921.log` |

The selected caller-zero cohort is the direct-static physical-input row test
facade:

* `src/mir/builder/normal_script_direct_static_join_handoff.rs:294` —
  `VerifiedScriptDirectStaticPhysicalInputRowV1` is consumed only by the
  `script_physical_exit` test fixture. Production consumers use the aggregate
  `VerifiedScriptDirectStaticPhysicalInputV1`.

The bounded execution slice gates only this row re-export with `#[cfg(test)]`.
The physical-input product, direct-static lowering, visibility of the aggregate
input, and all semantic behavior remain unchanged. Any production consumer,
compile failure, changed test warning, or count mismatch returns the row to
design stop.

## Closeout evidence

The selected re-export was split so that only
`VerifiedScriptDirectStaticPhysicalInputRowV1` is `#[cfg(test)]`; the aggregate
`VerifiedScriptDirectStaticPhysicalInputV1` remains available to production.
The fixed commands were run sequentially and both exited 0:

| command | result | evidence |
| --- | --- | --- |
| `cargo check --profile quick --lib -j4` | lib warnings **1,804** | `/tmp/hakorune-warning-i0-direct-static-physical-input-row-lib-20260921.log` |
| `cargo test --profile quick --lib --no-run -j4` | lib-test warnings **561**, test executable built | `/tmp/hakorune-warning-i0-direct-static-physical-input-row-lib-test-20260921.log` |

The only source change is the grouped re-export declaration in
`normal_script_direct_static_join_handoff.rs`; physical-input construction,
lowering, and test behavior are unchanged.
