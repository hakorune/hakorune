---
Status: Task Pack
Decision: accepted
Date: 2026-03-26
Scope: `P33` after confirming the shell-helper trio is not homogeneous; inventory `tools/hakorune_emit_mir.sh` as the narrowest remaining helper-local keep for `MirBuilderBox.emit_from_program_json_v0(...)`.
Related:
  - docs/development/current/main/phases/phase-29cl/P21-BYN-MIN5-HARD-RETIRE-EXECUTION-PACK.md
  - docs/development/current/main/phases/phase-29cl/P33-BYN-MIN5-PROGRAM-JSON-SHELL-HELPER-INVENTORY.md
  - docs/development/current/main/phases/phase-29cl/README.md
  - docs/development/current/main/phases/phase-29ci/P3-SHARED-SHELL-HELPER-AUDIT.md
  - tools/hakorune_emit_mir.sh
  - tools/selfhost/selfhost_build.sh
  - tools/smokes/v2/lib/test_runner.sh
---

# P34: BYN-min5 Hakorune-Emit-MIR Helper Inventory

## Purpose

- inventory `tools/hakorune_emit_mir.sh` as the first exact helper-local bucket under the remaining Program(JSON) shell-helper keep
- keep `tools/selfhost/selfhost_build.sh` and `tools/smokes/v2/lib/test_runner.sh` out of this slice
- decide whether `tools/hakorune_emit_mir.sh` has one narrower direct-emit seam or should stay monitor-only as a single helper-local pipeline

## Current Truth

1. `docs/development/current/main/phases/phase-29ci/P3-SHARED-SHELL-HELPER-AUDIT.md` fixes the helper delete order as `tools/hakorune_emit_mir.sh` -> `tools/selfhost/selfhost_build.sh` -> `tools/smokes/v2/lib/test_runner.sh`
2. `tools/hakorune_emit_mir.sh` still owns a generated runner path that calls `MirBuilderBox.emit_from_program_json_v0(...)` behind helper-local `_emit_mir_checked(...)`
3. `tools/hakorune_emit_mir.sh` also keeps provider/delegate and direct `--program-json-to-mir` fallback routes in the same helper-local pipeline
4. `tools/selfhost/selfhost_build.sh` remains a broader build-contract helper with `--mir`, `--run`, and `--exe` consumers; do not mix it into this bucket
5. `tools/smokes/v2/lib/test_runner.sh` remains a shared smoke/runtime harness and must stay paired with the smoke-tail audit rather than this helper-local slice

## Next Exact Front

1. inventory `tools/hakorune_emit_mir.sh` itself and decide whether one exact direct-emit route can be isolated as the next narrow execution bucket
