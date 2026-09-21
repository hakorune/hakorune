---
Status: closed__2026-09-21__WarningBaselineRefreshI45__SelectedDynamicEmitterTestFacade
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I45
Date: 2026-09-21
Parent: mirbuilder-warning-draft-seal-exit-test-facade-i0-2026-09-21.md
Implementation permission: selection recorded; execution delegated to MIRBUILDER-WARNING-SELECTED-DYNAMIC-EMITTER-TEST-FACADE-I0
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I46
---

# MirBuilder warning baseline refresh I45

## Six-line brief

```text
Decision: refresh both warning surfaces after the I44 draft-seal exit test
  facade cohort before selecting another cleanup row.
Source authority + canonical issuer: fixed quick-profile Cargo diagnostics and
  the checked-in warning classification policy.
Non-authority: historical counts, cargo-fix, blanket allow, visibility edits,
  dead-code guesses, or semantic interpretation of lint output.
Fail-fast boundary: command drift, new failure name, unclassified test-only
  reference, or a warning family outside the inventory stops selection.
Smallest next slice: compare current lib/lib-test inventories against
  1,785/561 and select at most one caller-zero import cohort.
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

The fixed gates completed sequentially at the current head:

| surface | warnings | evidence |
| --- | ---: | --- |
| lib | 1,785 | `/tmp/hakorune-warning-i45-lib-20260921.log` |
| lib test | 561 | `/tmp/hakorune-warning-i45-lib-test-20260921.log` |

The selected caller-zero cohort is the selected dynamic physical emitter test
facade:

* `selected_dynamic_physical_emitter/mod.rs:39` imports
  `DynamicV2PhysicalRepresentationV1`; its uses in this module are only the
  `#[cfg(test)]` representation helpers.
* `:55-56` imports the value-ledger view and reject types alongside the
  production `DynamicV2PhysicalValueLedgerV1`; only the view/reject pair is
  test-only.
* `:599` re-exports `assemble_unpublished_selected_dynamic_w6` and its
  `_from_parts` sibling; production uses only `_from_parts`, while the shorter
  helper is test-only.

The bounded execution slice gates these three test-only import/re-export groups
under `#[cfg(test)]`; dynamic physical ownership and production assembly remain
unchanged.

## Closeout evidence

The delegated selected-emitter test-facade slice completed with the fixed gates,
sequentially:

| surface | warnings | evidence |
| --- | ---: | --- |
| lib | 1,782 | `/tmp/hakorune-warning-i0-selected-dynamic-emitter-test-facade-lib-20260921.log` |
| lib test | 561 | `/tmp/hakorune-warning-i0-selected-dynamic-emitter-test-facade-lib-test-20260921.log` |

Both commands exited 0. The representation/view/reject imports and short
assembly helper are now test-only; production ledger and `_from_parts` assembly
remain unchanged. I45 is closed, and I46 is the next design-stop baseline refresh.
