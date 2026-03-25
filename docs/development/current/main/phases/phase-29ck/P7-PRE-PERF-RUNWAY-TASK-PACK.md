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
- exact next work:
  - close the recipe seam as an active widening front
  - move the current state from `active widening` to `stable evidence owner`
  - pin reopen rule: only reopen when a new narrow evidence row is required by `phase-29ck`
- non-goals:
  - no new pure-first support
  - no C export/default change
  - no `llvmlite` keep-lane edit
- acceptance:
  - `tools/dev/phase29ck_backend_recipe_profile_probe.sh`
  - `tools/smokes/v2/profiles/integration/phase29ck_boundary/entry/phase29ck_boundary_compat_keep_min.sh`
  - `tools/smokes/v2/profiles/integration/phase29ck_boundary/entry/phase29ck_boundary_pure_first_min.sh`

### `W2` boundary fallback reliance reduction

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
  - `phase29ck_boundary_*` default/compat keep smokes
  - broken `NYASH_NY_LLVM_COMPILER` exact proofs for supported seeds

### `W3` Rust glue thinning

- exact target:
  - keep Rust boundary code on payload decode / symbol selection / boundary glue only
  - do not let owner truth drift back out of `.hako`
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
- split order:
  - `W4a` keep-lane owner inventory sync
  - `W4b` acceptance/readme/test split between mainline and keep
  - `W4c` pre-perf exit check and close-sync
- non-goals:
  - no new hot-path optimization in `llvm_py`
  - no perf reopen in the same patch
- acceptance:
  - keep-lane docs/tests stay green
  - `phase-29ck` mainline route proofs do not require `llvmlite`

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
