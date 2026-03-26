---
Status: Task Pack
Decision: accepted
Date: 2026-03-26
Scope: `P35` after landing the generated selfhost builder runner seam split in `tools/hakorune_emit_mir.sh`; inventory the broader build-contract helper `tools/selfhost/selfhost_build.sh` as the next exact bucket.
Related:
  - docs/development/current/main/phases/phase-29cl/P21-BYN-MIN5-HARD-RETIRE-EXECUTION-PACK.md
  - docs/development/current/main/phases/phase-29cl/P35-BYN-MIN5-EMIT-MIR-SELFHOST-RUNNER-SEAM-INVENTORY.md
  - docs/development/current/main/phases/phase-29cl/README.md
  - docs/development/current/main/phases/phase-29ci/P3-SHARED-SHELL-HELPER-AUDIT.md
  - tools/selfhost/selfhost_build.sh
  - tools/smokes/v2/lib/test_runner.sh
---

# P36: BYN-min5 Selfhost-Build Helper Inventory

## Purpose

- inventory `tools/selfhost/selfhost_build.sh` as the next broader helper-local keep after `tools/hakorune_emit_mir.sh`
- keep `tools/smokes/v2/lib/test_runner.sh` out of this slice
- decide whether one exact build-contract seam can be isolated without mixing `--mir`, `--run`, and `--exe` consumers

## Current Truth

1. `tools/selfhost/selfhost_build.sh` is a broader build-contract helper, not a narrow direct `MirBuilderBox.emit_from_program_json_v0(...)` pipeline
2. the helper still owns `BuildBox.emit_program_json_v0(...)` keep, Program(JSON)->MIR conversion, Core-direct run, and Program(JSON)->EXE routing
3. this bucket must stay separate from `tools/smokes/v2/lib/test_runner.sh`, which remains the last helper audit bucket
4. current judgment: this helper is the next exact front only because `tools/hakorune_emit_mir.sh` is now near-thin-floor under `P35`

## Next Exact Front

1. inventory `tools/selfhost/selfhost_build.sh` and decide whether one exact build-contract seam is narrower than the rest
