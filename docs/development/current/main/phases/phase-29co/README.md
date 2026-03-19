---
Status: Active
Decision: provisional
Date: 2026-03-20
Scope: `stage1-cli` bootstrap rebuild の `missing schema_version` を、C ABI FFI contract と bootstrap rebuild handoff の整合で解消するための専用 phase。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/design/selfhost-bootstrap-route-ssot.md
  - docs/development/current/main/design/selfhost-bootstrap-route-evidence-and-legacy-lanes.md
  - docs/development/current/main/phases/phase-29cn/README.md
  - tools/selfhost/build_stage1.sh
  - tools/build_hako_llvmc_ffi.sh
  - lang/c-abi/shims/hako_json_v1.c
  - lang/c-abi/shims/hako_llvmc_ffi.c
  - tools/ny_mir_builder.sh
---

# Phase 29co: Stage1 Bootstrap C ABI Schema Contract Repair

> Update 2026-03-20: the C ABI schema seam was rebuilt, and the reduced stage1-cli artifact is now treated as runnable bootstrap output. The remaining payload proof lives on the stage0 bootstrap route, so the next handoff cleanup sits in `phase-29cp`.

## Goal

- `stage1-cli` bootstrap rebuild の `ny-llvmc failed to link exe` を、C ABI FFI contract の整合で解消する。
- 具体的には `missing schema_version` を bootstrap handoff で吸収しつつ、stage1 artifact の実行 contract を変えない。
- shell 側の JSON rewrite ではなく、FFI shim の validation / rebuild contract を正しい位置に戻す。

## Boundary

- in scope:
  - `lang/c-abi/shims/hako_json_v1.c` の schema validation contract
  - `tools/build_hako_llvmc_ffi.sh` による FFI library rebuild
  - `tools/selfhost/build_stage1.sh` の stage1-cli bootstrap handoff / freshness boundary
  - `tools/ny_mir_builder.sh` の pass-through contract 保持
- out of scope:
  - `stage1_cli_env.hako` の authority redesign
  - backend-zero / `.hako` authoring wave
  - Program(JSON v0) retirement body work
  - kernel migration refactors

## Fixed Order

1. exact failure point を C ABI / rebuild boundary で inventory 化する
2. `libhako_llvmc_ffi` を stage1 bootstrap handoff に確実に乗せる
3. `build_stage1.sh --artifact-kind stage1-cli --force-rebuild` を green に戻す
4. stage1 env route の identity / smoke / docs を同じ wording にそろえる

## Current Blocker

- `tools/selfhost/build_stage1.sh --artifact-kind stage1-cli --force-rebuild` now gets past the C ABI rebuild seam, but the reduced artifact is validated as runnable bootstrap output while payload proof stays on the stage0 bootstrap route
- the shipped `libhako_llvmc_ffi` has already been rebuilt from `lang/c-abi/shims/hako_json_v1.c`
- the remaining mismatch is the bootstrap child/env bridge contract, which is now handled in `phase-29cp`

## Acceptance

- `bash tools/build_hako_llvmc_ffi.sh`
- `bash tools/selfhost/build_stage1.sh --artifact-kind stage1-cli --force-rebuild` PASS
- `bash tools/selfhost_identity_check.sh --mode full --skip-build --bin-stage1 target/selfhost/hakorune.stage1_cli --bin-stage2 target/selfhost/hakorune.stage1_cli.stage2` remains green
- stage1 env route still emits the same Program/MIR identities without shell-side JSON rewrite

## Non-Goals

- changing `stage1_cli_env.hako` authority semantics
- reopening `phase-29cg` / `phase-29ch` / `phase-29ci` / `phase-29cj`
- mixing the bootstrap schema fix with backend-zero or `.hako` authoring
