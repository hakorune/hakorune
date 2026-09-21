---
Status: closed__2026-09-21__WarningBaselineRefreshI27__ScalarOperandRecipeArgumentTestFacade
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I27
Date: 2026-09-21
Parent: mirbuilder-warning-direct-static-physical-input-row-test-facade-i0-2026-09-21.md
Implementation permission: true for one cfg(test) scalar-operand recipe argument re-export only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I28
---

# MirBuilder warning baseline refresh I27

## Six-line brief

```text
Decision: refresh both warning surfaces after the I26 row-facade cohort before
  selecting another cleanup row.
Source authority + canonical issuer: fixed quick-profile Cargo diagnostics and
  the checked-in warning classification policy.
Non-authority: historical counts, cargo-fix, blanket allow, visibility edits,
  dead-code guesses, or semantic interpretation of lint output.
Fail-fast boundary: command drift, new failure name, unclassified test-only
  reference, or a warning family outside the inventory stops selection.
Smallest next slice: compare current lib/lib-test inventories against
  1,804/561 and select at most one caller-zero import cohort.
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

The fixed commands completed sequentially at the current head:

| surface | warnings | evidence |
| --- | ---: | --- |
| lib | 1,804 | `/tmp/hakorune-warning-i27-lib-20260921.log` |
| lib test | 561 | `/tmp/hakorune-warning-i27-lib-test-20260921.log` |

The selected caller-zero cohort is the scalar operand recipe argument test
facade:

* `src/mir/builder/normal_script_direct_static_join_handoff.rs:302` —
  `ScalarOperandRecipeArgumentV1` is re-exported for the
  `script_physical_exit` test module only. Production `physical_input.rs` uses
  the owner module directly; no production caller uses this re-export.

The bounded execution slice gates only this re-export with `#[cfg(test)]`.
The scalar recipe owner, physical-input product, direct-static lowering, and
all semantic behavior remain unchanged. Any production consumer, compile
failure, changed test warning, or count mismatch returns the row to design
stop.

## Closeout evidence

The selected re-export was split so that only
`ScalarOperandRecipeArgumentV1` is `#[cfg(test)]`; the scalar operator and node
exports remain available as before. The fixed commands were run sequentially
and both exited 0:

| command | result | evidence |
| --- | --- | --- |
| `cargo check --profile quick --lib -j4` | lib warnings **1,803** | `/tmp/hakorune-warning-i0-scalar-operand-recipe-argument-lib-20260921.log` |
| `cargo test --profile quick --lib --no-run -j4` | lib-test warnings **561**, test executable built | `/tmp/hakorune-warning-i0-scalar-operand-recipe-argument-lib-test-20260921.log` |

The only source change is the re-export declaration in
`normal_script_direct_static_join_handoff.rs`; scalar recipe construction,
physical input, lowering, and semantic behavior are unchanged.
