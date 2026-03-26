---
Status: Closed Task Pack
Decision: accepted
Date: 2026-03-26
Scope: `P34` after confirming `tools/hakorune_emit_mir.sh` is still a live helper-local bucket; inventory only the generated selfhost builder runner seam around `MirBuilderBox.emit_from_program_json_v0(...)`.
Related:
  - docs/development/current/main/phases/phase-29cl/P21-BYN-MIN5-HARD-RETIRE-EXECUTION-PACK.md
  - docs/development/current/main/phases/phase-29cl/P34-BYN-MIN5-HAKORUNE-EMIT-MIR-HELPER-INVENTORY.md
  - docs/development/current/main/phases/phase-29cl/P36-BYN-MIN5-SELFHOST-BUILD-HELPER-INVENTORY.md
  - docs/development/current/main/phases/phase-29cl/README.md
  - docs/development/current/main/phases/phase-29ci/P3-SHARED-SHELL-HELPER-AUDIT.md
  - tools/hakorune_emit_mir.sh
---

# P35: BYN-min5 Emit-MIR Selfhost Runner Seam Inventory

## Purpose

- inventory only the generated selfhost builder runner seam inside `tools/hakorune_emit_mir.sh`
- keep provider/delegate fallback and legacy `--program-json-to-mir` routes out of this slice
- decide whether the helper-local `_emit_mir_checked(...)` path can be isolated as the next narrow execution bucket
- make this the last inventory step for the helper-local seam before code or monitor-only judgment

## Current Truth

1. `render_selfhost_builder_runner_hako(...)` still generates the wrapper-local `_emit_mir_checked(...)` path that calls `MirBuilderBox.emit_from_program_json_v0(...)`
2. the render / execute / marker-check / capture / cleanup lifecycle is now localized behind `prepare_selfhost_builder_runner_context(...)`, `run_rendered_selfhost_builder_runner(...)`, and `try_selfhost_builder_once(...)`
3. `try_selfhost_builder(...)` now owns only the exact route-choice policy for direct force, selected builder box, and optional builder-min retry
4. `emit_mir_json_via_delegate_routes(...)` and `emit_mir_json_via_non_direct_routes(...)` remain outside this seam and were not widened by this slice
5. current result: execution code landed; the generated selfhost builder runner seam is now near-thin-floor / monitor-only under the current helper-local scope
6. the next helper bucket is `tools/selfhost/selfhost_build.sh`, not another nested `tools/hakorune_emit_mir.sh` inventory

## Next Exact Front

1. `P36-BYN-MIN5-SELFHOST-BUILD-HELPER-INVENTORY.md`
