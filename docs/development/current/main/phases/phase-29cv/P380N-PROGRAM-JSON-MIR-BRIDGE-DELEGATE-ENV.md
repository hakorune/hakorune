---
Status: Accepted
Decision: accepted
Date: 2026-05-04
Scope: phase-29cv P380N, Program(JSON)->MIR bridge delegate env boundary
Related:
  - docs/development/current/main/phases/phase-29cv/P372A-PROGRAM-JSON-MIR-BRIDGE-CALLER-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P380M-PHASE29CG-OUT-DIR-GUARD.md
  - tools/selfhost/lib/program_json_mir_bridge.sh
  - tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh
---

# P380N: Program(JSON)->MIR Bridge Delegate Env Boundary

## Problem

P380M fixed caller-provided `OUT_DIR` setup for the phase29cg bridge keeper.
The helper then reached the real bridge boundary and failed here:

```text
[phase29cg] env.mirbuilder.emit wrapper failed (rc=1)
[ERROR] [vm/error] Invalid instruction: [freeze:contract][mirbuilder/delegate-forbidden] env.mirbuilder.emit blocked (HAKO_SELFHOST_NO_DELEGATE=1)
```

`stage1_contract` intentionally defaults selfhost Stage1 execution to:

```text
HAKO_SELFHOST_NO_DELEGATE=1
HAKO_MIR_BUILDER_DELEGATE=0
```

That is correct for the mainline source->MIR contract, but the
Program(JSON)->MIR bridge is an explicit compat capsule whose only job is to
run `env.mirbuilder.emit` until the P106 MIR-first replacement is green.

## Decision

Keep the mainline Stage1 defaults unchanged. Make the compat bridge helper own
its delegate env locally while invoking the wrapper MIR:

```text
HAKO_SELFHOST_NO_DELEGATE=0
HAKO_MIR_BUILDER_DELEGATE=1
```

This prevents caller env from accidentally disabling the bridge capsule while
keeping delegate use visible and confined to `program_json_mir_bridge_emit`.

## Non-Goals

- no new Program(JSON)->MIR bridge caller
- no Stage1 runner default change
- no route/classifier/body-shape change
- no C shim change
- no bridge retirement

## Acceptance

```bash
rm -rf /tmp/p380n_phase29cg
KEEP_OUT_DIR=1 \
OUT_DIR=/tmp/p380n_phase29cg \
STAGE1_BIN=target/selfhost/hakorune.stage1_cli_env_seed \
NYASH_LLVM_SKIP_BUILD=1 \
bash tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh

bash tools/checks/program_json_mir_bridge_caller_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected: phase29cg no longer fails with `mirbuilder/delegate-forbidden`.
The remaining result, pass or fail, is the next real MIR/LLVM keeper boundary.

## Result

Implemented.

`program_json_mir_bridge_emit` now owns the delegate env only for its wrapper
MIR invocation:

```text
HAKO_SELFHOST_NO_DELEGATE=0
HAKO_MIR_BUILDER_DELEGATE=1
```

The phase29cg keeper was remeasured with the default no-delegate Stage1
contract still in force:

```text
[phase29cg] out_dir=/tmp/p380n_phase29cg
[phase29cg] emit_program_rc=0 emit_mir_rc=0 llvm_rc=4 verify_rc=1 verify_count=
[FAIL] phase29cg_stage2_bootstrap_phi_verify llvm_rc=4 verify_rc=1 verify_count=
```

The prior `mirbuilder/delegate-forbidden` failure is gone. The bridge produced
MIR successfully, and the next boundary is the `ny-llvmc failed` result. The
verify error is secondary because no LLVM IR file was produced:

```text
opt: /tmp/p380n_phase29cg/stage1_cli_env.ll: error: Could not open input file: No such file or directory
```

Follow-up should improve the keeper's LLVM failure diagnostics or advance the
next MIR/LLVM blocker; do not add new bridge callers or Stage0 body shapes.
