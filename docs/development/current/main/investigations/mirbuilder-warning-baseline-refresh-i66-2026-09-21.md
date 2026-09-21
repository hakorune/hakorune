---
Status: closed__2026-09-21__WarningBaselineRefreshI66__SelectedFunctionControlTestFacade
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I66
Date: 2026-09-21
Parent: mirbuilder-warning-generic-g0-demand-test-facade-i0-2026-09-21.md
Implementation permission: false; selection recorded, successor must enter fast mode before code
NextCard: MIRBUILDER-WARNING-FUNCTION-CONTROL-TEST-FACADE-I0
---

# MirBuilder warning baseline refresh I66

## Six-line brief

```text
Decision: refresh both warning surfaces after the Generic G0 test-facade
  deletion before selecting another cleanup row.
Source authority + canonical issuer: fixed quick-profile Cargo diagnostics and
  the checked-in warning classification policy.
Non-authority: historical counts, cargo-fix, blanket allow, visibility edits,
  dead-code guesses, or semantic interpretation of lint output.
Fail-fast boundary: command drift, new failure name, unclassified test-only
  reference, or a warning family outside the inventory stops selection.
Smallest next slice: compare current lib/lib-test inventories against
  1,754/559 and select at most one caller-zero import cohort.
Non-claims: no broad warning deletion, suppression, semantic refactor, or
  production route change.
```

## Acceptance

Run these commands once each, sequentially, and record warning lint, file and
line, owner, and production/test/compat/generated role:

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
```

Select one finite caller-zero import cohort or write `NoSafeSlice`. Keep
dead-code and private-interface rows with their owners. No code edit is
permitted until the selection is recorded and the pointer enters its bounded
successor. The current fixed baseline is lib **1,754** and lib-test **559**.

## Selection evidence

The fixed gates completed sequentially with exit 0 at the current head:

| surface | warnings | evidence |
| --- | ---: | --- |
| lib | 1,754 | `/tmp/hakorune-warning-i66-lib-20260921.log` |
| lib test | 559 | `/tmp/hakorune-warning-i66-lib-test-20260921.log` |

The selected cohort is the two-layer cfg(test)-only re-export chain for
`verify_function_completion_with_new_homes_v1` at
`src/mir/resolved_control_flow/mod.rs:21` and
`src/mir/resolved_control_flow/function_control.rs:423`. A complete source
census finds only the helper definition and these unused re-exports; no caller
remains. The successor may remove only this chain, with expected lib unchanged
at **1,754** and lib-test **559 → 558**. The parent-only attempt was rejected
as incomplete because the warning moved to the owner layer.

Other imports remain owned rows: dynamic and operator facades have local test
consumers, backend and If recipe names have external consumers, and
dead-code/private-interface warnings remain informational owner rows. No
visibility or semantic control-flow change is selected.
