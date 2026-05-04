---
Status: Done
Decision: accepted
Date: 2026-05-04
Scope: phase-29cv P381N, Stage1 env MIR shape guard SSOT
Related:
  - docs/development/current/main/phases/phase-29cv/P381M-PHASE29CG-STAGE1-MIR-SHAPE-GUARD.md
  - tools/selfhost/lib/stage1_contract.sh
  - tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh
---

# P381N: Stage1 Env MIR Shape Guard SSOT

## Problem

P381M added a phase29cg-local guard so the bridge keeper cannot accidentally
treat reduced/stub MIR as the P106 MIR-first replacement.

The guard itself is not phase29cg-specific. The exact contract is:

- a full Stage1 env MIR payload must contain the owner functions that prove it
  came from `lang/src/runner/stage1_cli_env.hako`
- a tiny `main -> 97` payload is not a MIR-first replacement candidate

Keeping that pattern list only inside one dev probe creates a new shell-side
partial truth.

## Decision

Move the Stage1 env MIR shape check into `tools/selfhost/lib/stage1_contract.sh`
as a reusable contract helper, then make phase29cg consume that helper.

This remains a shell contract cleanup. It does not widen Stage0, does not add a
body shape, and does not remove the Program(JSON)->MIR bridge keeper.

## Boundary

Allowed:

- add a named `stage1_contract_*` guard helper for full Stage1 env MIR shape
- keep the same required owner-function patterns from P381M
- make phase29cg call the shared helper

Not allowed:

- make generic `emit-mir` validation require Stage1 env owner functions
- reject simple fixture MIR emitted through the bootstrap capability helper
- change Program(JSON)->MIR bridge routing
- add ny-llvmc body-specific lowering

## Acceptance

```bash
bash -n tools/selfhost/lib/stage1_contract.sh
bash -n tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh

rm -rf /tmp/p381n_phase29cg
KEEP_OUT_DIR=1 OUT_DIR=/tmp/p381n_phase29cg \
STAGE1_BIN=target/selfhost/hakorune.stage1_cli_env_seed \
NYASH_LLVM_SKIP_BUILD=1 \
bash tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh

bash tools/checks/program_json_mir_bridge_caller_guard.sh
bash tools/checks/tools_dev_surface_inventory_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected:

- phase29cg remains green
- Stage1 env MIR shape patterns live in one shell helper
- generic emit-output validation stays marker-only for non-Stage1-env fixture
  payloads

## Result

Done:

- added `stage1_contract_require_stage1_env_mir_shape()`
- made phase29cg consume the shared guard helper
- kept Program(JSON)->MIR bridge route unchanged
