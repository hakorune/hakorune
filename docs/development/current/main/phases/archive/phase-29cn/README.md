---
Status: Superseded
Decision: rejected
Date: 2026-03-20
Scope: `stage1-cli` bootstrap rebuild の `missing schema_version` を、bootstrap handoff の shell-side schema normalization で解消する仮説を検証した phase。最終的にこの仮説は採用せず、C ABI schema contract repair へ移行した。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/design/selfhost-bootstrap-route-ssot.md
  - docs/development/current/main/design/selfhost-bootstrap-route-evidence-and-legacy-lanes.md
  - docs/development/current/main/phases/phase-29cg/README.md
  - docs/development/current/main/phases/phase-29ch/README.md
  - docs/development/current/main/phases/phase-29ci/README.md
  - docs/development/current/main/phases/phase-29cj/README.md
  - tools/selfhost/build_stage1.sh
  - tools/ny_mir_builder.sh
  - crates/nyash-llvm-compiler/src/boundary_driver.rs
  - crates/nyash-llvm-compiler/src/compile_input.rs
  - src/host_providers/llvm_codegen/normalize.rs
  - lang/src/runner/stage1_cli_env.hako
---

# Phase 29cn: Stage1 Bootstrap Schema Normalization

> Superseded by `phase-29co`: shell-side normalization changed the final stage1 artifact semantics, so the actual fix moved to the C ABI schema contract boundary.

## Goal

- `stage1-cli` bootstrap の `emit-mir` / `build_stage1.sh --artifact-kind stage1-cli` 経路で、`ny-llvmc` crate backend が受け取れる schema に MIR を正規化する。
- 具体的には `missing schema_version` で `ny-llvmc failed to link exe` にならない状態を作る。
- `stage1_cli_env.hako` の authority / identity contract は維持し、bootstrap handoff 側で schema mismatch を吸収する。

## Boundary

- in scope:
  - `tools/selfhost/build_stage1.sh` の stage1-cli bootstrap handoff
  - `tools/ny_mir_builder.sh` の crate backend input normalization
  - `crates/nyash-llvm-compiler/src/boundary_driver.rs` / `compile_input.rs` の backend input contract
- out of scope:
  - `stage1_cli_env.hako` の authority redesign
  - backend-zero / `.hako` authoring wave
  - `Program(JSON v0)` retirement body work
  - kernel migration refactors

## Fixed Order

1. exact failure point を inventory 化する
2. `stage1-cli` から crate backend に渡る MIR payload の schema normalization を 1 箇所に固定する
3. `build_stage1.sh --artifact-kind stage1-cli --force-rebuild` を green に戻す
4. identity route / bootstrap route の docs を同じ wording にそろえる

## Current Blocker

- `tools/selfhost/build_stage1.sh --artifact-kind stage1-cli --force-rebuild` currently fails in `stageb-delegate` / `ny-llvmc failed to link exe` with `missing schema_version`
- the current payload path still uses `lang/src/runner/stage1_cli_env.hako::Stage1ProgramJsonMirCallerBox -> MirBuilderBox.emit_from_program_json_v0(...)`
- this is a bootstrap handoff blocker, not a backend-zero or `.hako` authoring blocker

## Acceptance

- `bash tools/selfhost/build_stage1.sh --artifact-kind stage1-cli --force-rebuild` PASS
- `bash tools/selfhost_identity_check.sh --mode full --skip-build --bin-stage1 target/selfhost/hakorune.stage1_cli --bin-stage2 target/selfhost/hakorune.stage1_cli.stage2` remains green
- bootstrap normalization stays transparent to the stage1 authority contract

## Non-Goals

- changing `stage1_cli_env.hako` authority semantics
- reopening `phase-29cg` / `phase-29ch` / `phase-29ci` / `phase-29cj` solved buckets
- mixing the bootstrap schema fix with backend-zero or `.hako` authoring
