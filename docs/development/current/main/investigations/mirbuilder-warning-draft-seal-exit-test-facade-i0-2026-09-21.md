---
Status: closed__2026-09-21__WarningDraftSealExitTestFacade
Task: MIRBUILDER-WARNING-DRAFT-SEAL-EXIT-TEST-FACADE-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i44-2026-09-21.md
Implementation permission: true for one cfg(test) draft-seal exit re-export only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I45
---

# Warning cleanup: draft-seal exit test facade

## Six-line brief

```text
Decision: gate the draft-seal exit re-export used only by completion-consumption
  tests; keep production exit_projection imports unchanged.
Source authority + canonical issuer: draft_seal/multi_site_exit.rs; production
  exit projection owns the error and claim types.
Non-authority: cargo-fix, wildcard imports, exit redesign, draft-seal ownership,
  or warning guesses.
Fail-fast boundary: any production facade consumer, compile error, changed test
  warning, or warning-count mismatch rejects the slice.
Smallest next slice: add cfg(test) to the grouped re-export and run fixed gates.
Non-claims: no exit semantics change, suppression, or production cutover.
```

## Preconditions and acceptance

I44 records one lib-only grouped unused-import diagnostic at
`src/mir/builder/resolved_lowering/draft_seal.rs:38`. The only facade consumer is
test-only; production exit projection uses the child module directly.

Acceptance requires lib warnings to drop from **1,786 to 1,785**, lib-test
warnings to remain **561**, and both fixed commands to exit 0:

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
```

Only the re-export cfg annotation may change.

## Closeout evidence

The sole permitted source edit gated the grouped re-export of
`DetachedFunctionExitClaimSetV1` and `MultiSiteExitPreparationErrorV1` under
`#[cfg(test)]`. The fixed gates completed sequentially with exit 0:

* lib: 1,785 warnings — `/tmp/hakorune-warning-i0-draft-seal-exit-test-facade-lib-20260921.log`
* lib test: 561 warnings — `/tmp/hakorune-warning-i0-draft-seal-exit-test-facade-lib-test-20260921.log`

No draft-seal exit semantics or production owner changed.
