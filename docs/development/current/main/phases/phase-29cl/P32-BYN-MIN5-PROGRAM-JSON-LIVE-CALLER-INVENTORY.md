---
Status: Closed Task Pack
Decision: accepted
Date: 2026-03-26
Scope: `P31` after confirming `emit_from_program_json_v0` stays live compat; inventory the remaining live/bootstrap caller set before any program-json seam retirement judgment.
Related:
  - docs/development/current/main/phases/phase-29cl/P21-BYN-MIN5-HARD-RETIRE-EXECUTION-PACK.md
  - docs/development/current/main/phases/phase-29cl/P31-BYN-MIN5-MIRBUILDER-PROGRAM-JSON-SEAM-INVENTORY.md
  - docs/development/current/main/phases/phase-29cl/P33-BYN-MIN5-PROGRAM-JSON-SHELL-HELPER-INVENTORY.md
  - docs/development/current/main/phases/phase-29cl/README.md
  - lang/src/runner/stage1_cli.hako
  - lang/src/runner/stage1_cli_env.hako
  - lang/src/runner/launcher.hako
  - lang/src/mir/builder/MirBuilderBox.hako
  - docs/development/current/main/phases/phase-29ci/P0-PROGRAM-JSON-V0-CONSUMER-INVENTORY.md
---

# P32: BYN-min5 Program-JSON Live Caller Inventory

## Purpose

- inventory the remaining live/bootstrap callers of `MirBuilderBox.emit_from_program_json_v0(...)`
- keep the kernel/module-string seam judgment from `P31` closed and separate
- decide which caller family is the narrowest next execution/inventory bucket

## Current Truth

1. `lang/src/runner/stage1_cli.hako` still calls `MirBuilderBox.emit_from_program_json_v0(...)`
2. `lang/src/runner/stage1_cli_env.hako` still calls `MirBuilderBox.emit_from_program_json_v0(...)`
3. `lang/src/runner/launcher.hako` still calls `MirBuilderBox.emit_from_program_json_v0(...)`
4. `lang/src/mir/builder/MirBuilderBox.hako` still exposes the Program(JSON) route on the language side
5. `docs/development/current/main/phases/phase-29ci/P0-PROGRAM-JSON-V0-CONSUMER-INVENTORY.md` already treats these as live/bootstrap callers, separate from shell-helper and diagnostics keep
6. `docs/development/current/main/phases/phase-29ci/P2-LIVE-CALLER-DELETE-ORDER.md` already treats the 4 `.hako` owners as near-thin-floor / monitor-only and warns against mixing them with shell-helper keep
7. current judgment: the `.hako` live/bootstrap caller set remains live, but it is not the narrowest next execution bucket
8. do not mix shell-helper keep or diagnostics/probe keep into this first caller inventory slice

## Next Exact Front

1. `P33-BYN-MIN5-PROGRAM-JSON-SHELL-HELPER-INVENTORY.md`
