---
Status: Task Pack
Decision: accepted
Date: 2026-03-26
Scope: `P34` after confirming `tools/hakorune_emit_mir.sh` is still a live helper-local bucket; inventory only the generated selfhost builder runner seam around `MirBuilderBox.emit_from_program_json_v0(...)`.
Related:
  - docs/development/current/main/phases/phase-29cl/P21-BYN-MIN5-HARD-RETIRE-EXECUTION-PACK.md
  - docs/development/current/main/phases/phase-29cl/P34-BYN-MIN5-HAKORUNE-EMIT-MIR-HELPER-INVENTORY.md
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

1. `render_selfhost_builder_runner_hako(...)` generates a wrapper that calls `MirBuilderBox.emit_from_program_json_v0(...)` via helper-local `_emit_mir_checked(...)`
2. `try_selfhost_builder(...)` owns the render -> execute -> capture lifecycle for that runner, plus the explicit builder-min retry path
3. `emit_mir_json_via_delegate_routes(...)` and `emit_mir_json_via_non_direct_routes(...)` belong to the broader helper route ladder and must stay outside this slice
4. this seam is narrower than `tools/selfhost/selfhost_build.sh` and `tools/smokes/v2/lib/test_runner.sh`, which remain later helper buckets
5. current judgment: the generated selfhost builder runner seam is the first exact helper-local seam worth opening after `P34`
6. this pack must not fan out into `P36` / `P37` style sub-inventories before an execution judgment is recorded

## Execution Judgment Contract

1. next result must be one of these two outcomes:
   - land a code slice that thins the generated selfhost builder runner seam without reopening the broader route ladder
   - or close this seam as `monitor-only / near-thin-floor` and advance to the next helper bucket
2. do not open another inventory pack for provider/delegate, legacy `--program-json-to-mir`, or shell-helper broadening until one of the two outcomes above is recorded
3. if the seam is judged `monitor-only`, the next helper bucket is `tools/selfhost/selfhost_build.sh`

## Next Exact Front

1. execution judgment on `_emit_mir_checked(...)` plus its render/execute/capture wrapper
