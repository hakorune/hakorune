---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: document `HAKO_CAPI_PURE` alias retirement inventory and task order.
Related:
  - docs/development/current/main/design/backend-recipe-route-profile-ssot.md
  - docs/reference/environment-variables.md
  - lang/c-abi/shims/hako_llvmc_ffi_common.inc
  - src/config/env/llvm_provider_flags.rs
  - crates/nyash-llvm-compiler/src/boundary_driver_ffi.rs
---

# P96 HAKO_CAPI_PURE Alias Retirement Inventory

## Goal

Prepare retirement of the historical `HAKO_CAPI_PURE=1` spelling without
changing behavior in the same card.

The canonical route spelling is:

```text
HAKO_BACKEND_COMPILE_RECIPE=pure-first
HAKO_BACKEND_COMPAT_REPLAY=none
```

`HAKO_CAPI_PURE=1` remains live for compat/pure-keep callers until the active
caller inventory is replaced. It must not be used as new mainline proof.

## Decision

- Do not add warning or fail-fast behavior in this card.
- Treat `HAKO_CAPI_PURE=1` as a live historical alias, not a policy owner.
- Replace active callers with `HAKO_BACKEND_COMPILE_RECIPE=pure-first` before
  changing runtime behavior.
- Keep archive-only callers as evidence, but do not let them block mainline
  route naming cleanup.

## Current Implementation Points

| Layer | File | Current read | Retirement posture |
| --- | --- | --- | --- |
| C shim generic export | `lang/c-abi/shims/hako_llvmc_ffi_common.inc` | `capi_pure_enabled()` uses `HAKO_CAPI_PURE` only when no explicit recipe is set | replace callers first, then remove alias fallback |
| Rust env catalog/helper | `src/config/env/llvm_provider_flags.rs` | `backend_recipe_requests_pure_first()` still treats `HAKO_CAPI_PURE` as pure-first request | remove after callers stop relying on it |
| ny-llvmc FFI boundary | `crates/nyash-llvm-compiler/src/boundary_driver_ffi.rs` | reads alias and passes it into boundary symbol selection | can be deleted after symbol selection no longer accepts alias |
| env docs | `docs/reference/environment-variables.md` | documented as compat-only pure-lowering | mark retire-target and point to canonical recipe spelling |

## Active Caller Inventory

| Caller | Kind | Current use | Next action |
| --- | --- | --- | --- |
| `tools/smokes/v2/lib/phase2120_boundary_pure_helper.sh` | active helper | exports `HAKO_CAPI_PURE=1` and requires it | P97: switch helper to canonical recipe spelling while preserving coverage |
| `tools/smokes/v2/profiles/integration/compat/pure-keep/run_pure_keep.sh` | active suite wrapper | exports `HAKO_CAPI_PURE=1` | P97/P98: route through helper policy or canonical recipe spelling |
| `tools/dev/phase29ck_boundary_historical_alias_probe.sh` | explicit alias probe | intentionally tests historical alias behavior | keep until warning/fail-fast card, then update expected behavior |
| `tools/dev/phase29ck_boundary_explicit_compat_probe.sh` | replacement reference | explicitly clears `HAKO_CAPI_PURE` and calls pure-first export | use as reference for non-alias compat coverage |
| `tools/smokes/v2/profiles/integration/proof/pure-legacy-cluster/README.md` | proof index | states active pure C-API keep pins require alias | update after active helper replacement |

## Archive / Historical References

Archive-only docs and scripts may keep `HAKO_CAPI_PURE=1` as historical
evidence. They do not define current route policy:

- `tools/archive/legacy-selfhost/compat-codegen/*`
- `docs/development/current/main/phases/phase-29x/*`
- `docs/development/current/main/phases/phase-29ck/*`
- `docs/development/current/main/design/de-rust-*`

If an archive script is reactivated into an integration suite, it must first be
migrated to the canonical recipe spelling or explicitly marked compat-only.

## Task Order

1. P96 docs lock
   - Inventory active callers and update route/env docs.
   - No behavior change.
2. P97 active helper replacement
   - Change the active pure helper/suite to use
     `HAKO_BACKEND_COMPILE_RECIPE=pure-first`.
   - Keep tests green before changing alias semantics.
3. P98 alias probe split
   - Keep one historical alias probe that proves old spelling behavior, or
     convert it into a warning/fail-fast expectation if P99 chooses that path.
4. P99 alias warning
   - Add a stable warning only after active callers are off the alias.
   - Update log contract if a new stable tag is added.
5. P100 alias fail-fast or no-op
   - Remove C/Rust alias branches after warning soak and proof migration.

## Stop Line

- Do not delete `HAKO_CAPI_PURE` in this card.
- Do not change `HAKO_BACKEND_COMPAT_REPLAY=harness`.
- Do not change `stageb-delegate` defaults.
- Do not touch `boxcall` / `externcall` legacy dialects.

## Acceptance

```bash
rg -n "HAKO_CAPI_PURE" . -g '!target' -g '!docs/private/**'
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
