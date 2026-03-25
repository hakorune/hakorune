---
Status: Task Pack
Decision: accepted
Date: 2026-03-25
Scope: `phase-29ck` の pre-perf runway を `W1..W4` の固定順で task 化し、`phase-21_5` / `kilo` を reopen する前に閉じるべき mainline fronts を commit 粒度まで分解する。
Related:
  - docs/development/current/main/phases/phase-29ck/README.md
  - docs/development/current/main/phases/phase-29ck/P3-THIN-BACKEND-CUTOVER-LOCK.md
  - docs/development/current/main/phases/phase-29ck/P5-COMPAT-PURE-PACK-LOCK.md
  - docs/development/current/main/design/backend-recipe-route-profile-ssot.md
  - docs/development/current/main/design/de-rust-backend-zero-boundary-lock-ssot.md
  - docs/development/current/main/design/stage2-aot-native-thin-path-design-note.md
  - docs/development/current/main/design/perf-optimization-method-ssot.md
---

# P7: Pre-Perf Runway Task Pack

## Purpose

- `perf/kilo` reopen 前に閉じるべき `ny-llvm` mainline fronts を 1 枚で固定する。
- `phase-29ck` の remaining work を `recipe seam -> fallback reliance -> Rust glue -> llvmlite demotion` の 4 wave に固定する。
- runtime proof / native subset widening / `phase-29cl` を evidence/adjacent lane として残し、pre-perf mainline と混ぜない。
- 各 wave を `1 series = 1 front` の運用で切れるように、owner / non-goals / acceptance / exit condition を先に lock する。

## Current Runway State

- `W1` `.hako` recipe seam close-sync is landed.
- `W2a` forwarder / explicit compat replay inventory lock is landed.
- `W2b` pure-first lane now fail-fasts without explicit compat replay; unsupported keep seeds require explicit `compat_replay=harness`.
- `W2c` generic export / historical alias keep-only sync is landed.
- `W3a` `llvm_codegen.rs` normalization/default helper concentration is landed.
- `W3b` `boundary_driver*.rs` thin-floor is landed.
- `W3c` generic compile symbol branch keep-only lock is landed.
- `W4a` keep-lane owner inventory sync is landed.
- `W4b` acceptance/readme/test split is landed.
- current active pre-perf front is `W4c` pre-perf exit check and close-sync.
- `perf/kilo` remains parked until `W3..W4` are also closed.

## Fixed Order

| wave | goal | owner | acceptance focus | do not mix |
| --- | --- | --- | --- | --- |
| `W1` | `.hako` recipe seam close-sync | `lang/src/shared/backend/backend_recipe_box.hako` + route-profile docs | visible acceptance rows / recipe-policy ownership / probe sync | support widening / C transport edits |
| `W2` | boundary fallback reliance reduction | `lang/c-abi/shims/hako_llvmc_ffi.c` 周辺 + fallback docs | daily default route と explicit compat replay の分離 | Rust glue thinning / llvmlite demotion |
| `W3` | Rust glue thinning | `src/host_providers/llvm_codegen.rs` + `crates/nyash-llvm-compiler/src/boundary_driver*.rs` | transport-only / symbol selection / boundary glue thin-floor | owner logic の再吸収 / recipe widening |
| `W4` | `llvmlite` demotion completion | `src/llvm_py/**` + `tools/llvmlite_harness.py` docs/tests | explicit compat/probe keep lock | perf reopen / hot-path obligations |

## Wave Notes

### `W1` `.hako` recipe seam close-sync

- current landed evidence:
  - supported pure rows
  - `method-call-only-small-compat-v1`
- current landed state:
  - recipe seam is now the stable visible evidence owner
  - widening is no longer the active backend-zero front
  - reopen only when a new narrow evidence row is required by `phase-29ck`
- non-goals:
  - no new pure-first support
  - no C export/default change
  - no `llvmlite` keep-lane edit
- acceptance:
  - `tools/dev/phase29ck_backend_recipe_profile_probe.sh`
  - `tools/smokes/v2/profiles/integration/phase29ck_boundary/entry/phase29ck_boundary_compat_keep_min.sh`
  - `tools/smokes/v2/profiles/integration/phase29ck_boundary/entry/phase29ck_boundary_pure_first_min.sh`

### `W2` boundary fallback reliance reduction

- current landed state:
  - `W2a` is landed
  - default forwarder / explicit compat replay / pure-first lane now have distinct helper owners in the C shim
  - raw compat replay callsites are reduced to the explicit compat replay helper
  - `W2b` is landed
  - unsupported pure-first shapes now fail fast unless explicit `HAKO_BACKEND_COMPAT_REPLAY=harness` is present
  - `W2c` is landed
  - generic `hako_llvmc_compile_json` and `HAKO_CAPI_PURE=1` are now locked as historical keep surfaces only; explicit recipe names win when both are present
- exact target:
  - shrink unsupported-shape reliance on `hako_llvmc_ffi.c -> ny-llvmc --driver harness`
  - keep `harness` / `native` as explicit replay lanes only
- split order:
  - `W2a` forwarder / explicit compat replay inventory lock
  - `W2b` daily default route reduction slice
  - `W2c` generic export / historical alias keep-only sync
- non-goals:
  - no recipe-policy widening
  - no perf retune
  - no Python builder work
- acceptance:
  - `tools/dev/phase29ck_boundary_fallback_inventory_probe.sh`
  - `tools/dev/phase29ck_boundary_explicit_compat_probe.sh`
  - `tools/dev/phase29ck_boundary_historical_alias_probe.sh`
  - `phase29ck_boundary_*` default/compat keep smokes
  - broken `NYASH_NY_LLVM_COMPILER` exact proofs for supported seeds

### `W3` Rust glue thinning

- exact target:
  - keep Rust boundary code on payload decode / symbol selection / boundary glue only
  - do not let owner truth drift back out of `.hako`
- current landed state:
  - `W3a` is landed
  - `src/host_providers/llvm_codegen.rs` now delegates boundary-default recipe/compat defaults and FFI library candidate ownership into `src/host_providers/llvm_codegen/defaults.rs`
  - the parent file now reads as `Opts + public facade entrypoints`, while `normalize.rs` / `route.rs` / `transport.rs` / `defaults.rs` own helper-local truth
  - `W3b` is landed
  - `crates/nyash-llvm-compiler/src/boundary_driver_ffi.rs` now delegates compile symbol selection and FFI library candidate resolution into `crates/nyash-llvm-compiler/src/boundary_driver_defaults.rs`
  - the FFI file now reads more as call/link transport glue, while route-symbol and library-candidate truth is isolated for the final keep-only lock
  - `W3c` is landed
  - boundary driver now fails fast on the selected compile symbol instead of silently falling back from `hako_llvmc_compile_json_pure_first` to the generic export
  - the generic `hako_llvmc_compile_json` branch is now explicit keep-only inside the Rust boundary driver, matching the already-landed C-side historical keep lock
- split order:
  - `W3a` `llvm_codegen.rs` normalization/default helper concentration
  - `W3b` `boundary_driver*.rs` thin-floor
  - `W3c` generic compile symbol branch keep-only lock
- non-goals:
  - no new route-profile evidence rows
  - no `llvmlite` hot-path rescue
  - no stage2 fast-leaf reopen
- acceptance:
  - Rust unit tests around boundary default / symbol selection
  - `phase29ck_llvm_backend_box_capi_link_min.sh`
  - `phase29ck_boundary_*` supported/compat controls

### `W4` `llvmlite` demotion completion

- exact target:
  - lock `src/llvm_py/**` and `tools/llvmlite_harness.py` as explicit compat/probe keep only
  - remove remaining ambiguity that `llvmlite` might still be a mainline owner
- current landed state:
  - `W4a` is landed
  - `tools/llvmlite_harness.py` now self-identifies with `[llvmlite-keep]`
  - `src/llvm_py/README.md` now pins the Python lane as explicit compat/probe keep in the code-side entry doc
  - `W4b` is landed
  - mainline acceptance and keep-lane acceptance are now read separately
  - keep-lane identity is pinned by `tools/smokes/v2/profiles/integration/phase29ck_boundary/entry/phase29ck_llvmlite_keep_identity_min.sh`
- split order:
  - `W4a` keep-lane owner inventory sync
  - `W4b` acceptance/readme/test split between mainline and keep
  - `W4c` pre-perf exit check and close-sync
- non-goals:
  - no new hot-path optimization in `llvm_py`
  - no perf reopen in the same patch
- acceptance:
  - mainline:
    - `tools/smokes/v2/profiles/integration/apps/phase29ck_llvm_backend_box_capi_link_min.sh`
    - `tools/smokes/v2/profiles/integration/phase29ck_boundary/entry/phase29ck_boundary_pure_first_min.sh`
  - keep:
    - `tools/smokes/v2/profiles/integration/phase29ck_boundary/entry/phase29ck_llvmlite_keep_identity_min.sh`
    - `PYTHONPATH=src/llvm_py:. python3 -m unittest src.llvm_py.tests.test_strlen_fast`

## Pre-Perf Exit Condition

`phase-21_5` / `kilo` may reopen only when all of these are true.

1. `BackendRecipeBox` is the stable visible recipe/policy owner.
2. boundary default route is stable and does not silently merge with compat replay.
3. Rust boundary code is transport/boundary glue only.
4. `llvmlite` is documented and tested as explicit compat/probe keep only.

If any one of these is still open, the perf lane stays parked.

## Reopen Rules

- reopen `W1` only for a new narrow evidence row required by a real `phase-29ck` blocker
- reopen `W2` only when the daily default route still depends on explicit compat replay
- reopen `W3` only when owner logic leaks back into Rust boundary glue
- reopen `W4` only when a mainline acceptance path still reads `llvmlite` as owner
- do not reopen `phase-21_5` / `kilo` until `W1..W4` are closed
