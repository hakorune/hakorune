# CURRENT_TASK (root pointer)

Status: SSOT
Date: 2026-03-28
Scope: repo root の再起動入口。詳細の status / phase 進捗は `docs/development/current/main/` を正本とする。

## Purpose

- root から最短で current blocker / active lane / next fixed order に到達する。
- 本ファイルは薄い入口に保ち、長い phase 履歴や retired lane detail は phase README / design SSOT へ逃がす。

## Quick Restart Pointer

- `docs/development/current/main/05-Restart-Quick-Resume.md`
- `docs/development/current/main/15-Workstream-Map.md`
- `git status -sb`
- `tools/checks/dev_gate.sh quick`

## Current Lanes

### phase-29bq

- status: `active (failure-driven; blocker=none)`
- scope: selfhost `.hako` migration (`mirbuilder first / parser later`)
- current SSOT:
  - `docs/development/current/main/phases/phase-29bq/README.md`
  - `docs/development/current/main/phases/phase-29bq/29bq-90-selfhost-checklist.md`
  - `docs/development/current/main/phases/phase-29bq/29bq-91-mirbuilder-migration-progress-checklist.md`
  - `docs/development/current/main/phases/phase-29bq/29bq-92-parser-handoff-checklist.md`
- next exact leaf: `none` until the next blocker is captured

### phase-29x

- status: `active compare bridge retirement / archive decisions`
- scope: shrink the remaining compare bridge / archive wrapper surfaces
- current truth:
  - `archive-home is sufficient`
  - `delete-ready is none`
  - Hako front-door `env.codegen.compile_json_path` retirement is landed
  - launcher root-first transport cut is landed
  - builder-side `compile_json_path` recognition is retired
  - Rust runtime dispatcher `compile_json_path` branches are retired
  - route-env helper `lang/src/shared/backend/backend_route_env_box.hako` is retired from code
  - remaining live set is compare bridge / archive wrapper surfaces
  - dead wrapper `lang/src/shared/host_bridge/codegen_bridge_box.hako::compile_json_path_args` is retired in this slice
- fixed order:
  1. keep `.ll` as the Rust/LLVM tool seam
  2. thin compare bridge wrapper surfaces caller-by-caller
  3. review archive/delete only after the wrapper inventory reaches zero
- current prep SSOT:
  - `docs/development/current/main/design/backend-owner-cutover-ssot.md`
  - `docs/development/current/main/design/runtime-decl-manifest-v0.toml`
  - `docs/development/current/main/phases/phase-29x/29x-96-backend-owner-legacy-ledger-ssot.md`
  - `docs/development/current/main/phases/phase-29x/29x-97-compare-bridge-retirement-prep-ssot.md`

### phase-29ck

- status: `monitor/evidence only`
- current details stay in phase29ck docs

## Immediate Next Task

- thin the remaining compare bridge wrapper surfaces first:
  - `src/host_providers/llvm_codegen/ll_emit_bridge.rs`
  - `src/host_providers/llvm_codegen/hako_ll_driver.rs`
- keep `src/host_providers/llvm_codegen/route.rs` and `src/host_providers/llvm_codegen/ll_tool_driver.rs` as keep surfaces
- keep `src/host_providers/llvm_codegen/ll_emit_bridge.rs` and `src/host_providers/llvm_codegen/hako_ll_driver.rs` archive-later only

## Notes

- `compile_json_path` / `mir_json_to_object*` are no longer daily-facing.
- No new delete-ready surface is known.
