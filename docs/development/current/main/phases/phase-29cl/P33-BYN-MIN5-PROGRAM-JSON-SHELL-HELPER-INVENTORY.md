---
Status: Closed Task Pack
Decision: accepted
Date: 2026-03-26
Scope: `P32` after confirming the 4 `.hako` live/bootstrap callers are monitor-only / near-thin-floor; inventory the remaining shell-helper keep for `MirBuilderBox.emit_from_program_json_v0(...)`.
Related:
  - docs/development/current/main/phases/phase-29cl/P21-BYN-MIN5-HARD-RETIRE-EXECUTION-PACK.md
  - docs/development/current/main/phases/phase-29cl/P32-BYN-MIN5-PROGRAM-JSON-LIVE-CALLER-INVENTORY.md
  - docs/development/current/main/phases/phase-29cl/P34-BYN-MIN5-HAKORUNE-EMIT-MIR-HELPER-INVENTORY.md
  - docs/development/current/main/phases/phase-29cl/README.md
  - docs/development/current/main/phases/phase-29ci/P0-PROGRAM-JSON-V0-CONSUMER-INVENTORY.md
  - docs/development/current/main/phases/phase-29ci/P2-LIVE-CALLER-DELETE-ORDER.md
  - tools/hakorune_emit_mir.sh
  - tools/selfhost/selfhost_build.sh
  - tools/smokes/v2/lib/test_runner.sh
---

# P33: BYN-min5 Program-JSON Shell-Helper Inventory

## Purpose

- inventory the remaining shell-helper keep for `MirBuilderBox.emit_from_program_json_v0(...)`
- keep the `.hako` live/bootstrap caller set frozen as monitor-only / near-thin-floor
- decide whether one helper owner is the narrowest next bucket or whether the helper set must stay paired

## Current Truth

1. `docs/development/current/main/phases/phase-29ci/P0-PROGRAM-JSON-V0-CONSUMER-INVENTORY.md` treats `tools/hakorune_emit_mir.sh`, `tools/selfhost/selfhost_build.sh`, and `tools/smokes/v2/lib/test_runner.sh` as the shell-helper keep set
2. `docs/development/current/main/phases/phase-29ci/P2-LIVE-CALLER-DELETE-ORDER.md` says the `.hako` owner 4 file should stay monitor-only / near-thin-floor while the helper set is audited separately
3. this bucket must stay separate from diagnostics/probe keep
4. this bucket must not reopen the kernel/module-string seam judgments from `P30` or `P31`
5. `tools/hakorune_emit_mir.sh` is the narrowest helper-local pipeline in the trio
6. `tools/selfhost/selfhost_build.sh` remains a broader build-contract helper and stays after `tools/hakorune_emit_mir.sh`
7. `tools/smokes/v2/lib/test_runner.sh` remains the last helper bucket because its direct builder-module runner is coupled to the shared smoke/runtime tail

## Next Exact Front

1. `P34-BYN-MIN5-HAKORUNE-EMIT-MIR-HELPER-INVENTORY.md`
