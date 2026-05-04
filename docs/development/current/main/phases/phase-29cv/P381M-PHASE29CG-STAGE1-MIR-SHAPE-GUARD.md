---
Status: Done
Decision: accepted
Date: 2026-05-04
Scope: phase-29cv P381M, phase29cg bridge keeper MIR-shape guard
Related:
  - docs/development/current/main/phases/phase-29cv/P105-PHASE29CG-STAGE1-ARTIFACT-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P106-PHASE29CG-MIR-FIRST-REPLACEMENT-BLOCKER.md
  - docs/development/current/main/phases/phase-29cv/P372A-PROGRAM-JSON-MIR-BRIDGE-CALLER-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P381L-BOOLEAN-PHI-INPUT-HELPER-SSOT.md
  - tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh
  - tools/dev/README.md
---

# P381M: Phase29cg Stage1 MIR Shape Guard

## Problem

P381K/P381L made the current phase29cg bridge proof green again, but the
MIR-first replacement is not green yet. A direct
`stage1_contract_exec_mode ... emit-mir` probe against the current
`stage1_cli_env_seed` artifact still returns a tiny `main -> 97` MIR payload.

The existing phase29cg script validates only that a MIR file is non-empty before
passing it to ny-llvmc. That is fine for the current bridge path, but it is too
weak as a future replacement boundary: a reduced/stub MIR could accidentally be
treated as a replacement candidate.

## Decision

Keep the Program(JSON)->MIR bridge keeper for now, but require the MIR payload
used by phase29cg to contain the Stage1 env owner functions that prove it is the
full `stage1_cli_env.hako` MIR, not a reduced run-only stub.

This is a guard/contract cleanup. It does not remove the bridge caller and does
not add Stage0 lowering surface.

## Boundary

Allowed:

- add a phase29cg-local MIR shape guard after MIR payload creation
- update active dev-surface wording for the bridge keeper
- keep the existing bridge route unchanged

Not allowed:

- delete `program_json_mir_bridge_emit`
- replace phase29cg with Rust direct `--emit-mir-json`
- treat stub MIR as MIR-first replacement success
- add backend body shapes or C shim emitters

## Acceptance

```bash
bash -n tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh

rm -rf /tmp/p381m_phase29cg
KEEP_OUT_DIR=1 OUT_DIR=/tmp/p381m_phase29cg \
STAGE1_BIN=target/selfhost/hakorune.stage1_cli_env_seed \
NYASH_LLVM_SKIP_BUILD=1 \
bash tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh

bash tools/checks/program_json_mir_bridge_caller_guard.sh
bash tools/checks/tools_dev_surface_inventory_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected:

- phase29cg remains green through the explicit bridge keeper
- the MIR payload contains Stage1 env owner functions
- active Program(JSON)->MIR bridge caller count does not grow

## Result

Done:

- added a phase29cg-local MIR shape guard for full Stage1 env owner functions
- kept the existing Program(JSON)->MIR bridge keeper route unchanged
- updated `tools/dev` owner wording so the removal condition is full Stage1 env
  MIR, not a marker-only stub
- phase29cg remains green through the explicit bridge keeper
