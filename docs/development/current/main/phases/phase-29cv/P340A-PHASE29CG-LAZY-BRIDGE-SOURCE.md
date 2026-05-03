---
Status: Completed
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv phase29cg bridge capsule source cleanup
Related:
  - docs/development/current/main/phases/phase-29cv/P105-PHASE29CG-STAGE1-ARTIFACT-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P106-PHASE29CG-MIR-FIRST-REPLACEMENT-BLOCKER.md
  - tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh
  - docs/development/current/main/CURRENT_STATE.toml
---

# P340A: Phase29cg Lazy Bridge Source

## Problem

P105 added a reduced-artifact guard to
`tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh`, but the script still
sourced `program_json_mir_bridge.sh` before that guard.

That kept the Program(JSON)->MIR bridge capsule present even on the common
fail-fast path where the selected Stage1 artifact is the reduced run-only
`stage1-cli` artifact and cannot emit Program/MIR payloads.

## Boundary

Do not remove `program_json_mir_bridge_emit()`.

Do not archive `phase29cg_stage2_bootstrap_phi_verify.sh`.

Do not replace the proof with Rust direct `--emit-mir-json` output.

This is a BoxShape cleanup only: the compatibility bridge is loaded only after
the artifact has passed the reduced-artifact guard and `emit-program` has
produced a Program(JSON v0) payload.

## Implementation

- move `source tools/selfhost/lib/program_json_mir_bridge.sh` below the
  reduced-artifact guard and below the `emit-program` payload check
- add a local comment tying the remaining bridge source to the P106 replacement
  proof condition

## Acceptance

```text
bash -n tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh
-> ok
```

```text
OUT_DIR=/tmp/p340_phase29cg_guard KEEP_OUT_DIR=1 \
  bash tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh
-> rc != 0
-> stderr contains "reduced run-only stage1-cli artifact cannot emit Program/MIR payloads"
```

```text
bash tools/checks/current_state_pointer_guard.sh
-> ok

git diff --check
-> ok
```
