---
Status: closed__2026-09-21__WarningBaselineRefreshI32__ModuleDeclarationShellPrepareErrorTestFacade
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I32
Date: 2026-09-21
Parent: mirbuilder-warning-live-loop-facts-qualification-test-facade-i0-2026-09-21.md
Implementation permission: true for one cfg(test) declaration-shell error re-export; execution closed
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I33
---

# MirBuilder warning baseline refresh I32

## Six-line brief

```text
Decision: refresh both warning surfaces after the I31 live-loop qualification
  facade cohort before selecting another cleanup row.
Source authority + canonical issuer: fixed quick-profile Cargo diagnostics and
  the checked-in warning classification policy.
Non-authority: historical counts, cargo-fix, blanket allow, visibility edits,
  dead-code guesses, or semantic interpretation of lint output.
Fail-fast boundary: command drift, new failure name, unclassified test-only
  reference, or a warning family outside the inventory stops selection.
Smallest next slice: compare current lib/lib-test inventories against
  1,799/561 and select at most one caller-zero import cohort.
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
| lib | 1,799 | `/tmp/hakorune-warning-i32-lib-20260921.log` |
| lib test | 561 | `/tmp/hakorune-warning-i32-lib-test-20260921.log` |

The selected caller-zero cohort is the test-only declaration-shell error
facade:

* `src/mir/builder/module_lowering_shell.rs:17` —
  `ModuleDeclarationFactShellPrepareErrorV1` is re-exported only for the
  `#[cfg(test)]` `module_declaration_fact_shell_commit_p0` fixture. There is
  no production caller of this parent re-export.

The bounded execution slice gates only this re-export with `#[cfg(test)]`.
Shell ownership, declaration-fact validation, and production lowering remain
unchanged. Any production consumer, compile failure, changed test warning, or
count mismatch returns the row to design stop.

## Closeout

The parent re-export is now test-only. The fixed gates completed sequentially:

| surface | warnings | evidence |
| --- | ---: | --- |
| lib | 1,798 | `/tmp/hakorune-warning-i0-module-declaration-shell-prepare-error-lib-20260921.log` |
| lib test | 561 | `/tmp/hakorune-warning-i0-module-declaration-shell-prepare-error-lib-test-20260921.log` |

Both commands exited 0. The production shell owner and declaration-fact
behavior are unchanged; I33 is the next design-stop baseline at 1,798/561.
