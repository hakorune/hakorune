---
Status: Accepted
Decision: accepted
Date: 2026-05-04
Scope: phase-29cv P380Q, Program(JSON)->MIR bridge semantic metadata refresh
Related:
  - docs/development/current/main/phases/phase-29cv/P380P-STRUCTURED-CALL-GENERIC-MIR-OP.md
  - docs/development/current/main/design/stage0-llvm-line-shape-inventory-ssot.md
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - src/host_providers/mir_builder/handoff.rs
---

# P380Q: Program(JSON) Bridge Semantic Refresh

## Problem

P380P taught ny-llvmc to recognize structured `op:"call"` as a MIR op.
The next phase29cg output showed the correct backend stop:

```text
reason=missing_multi_function_emitter
target_shape_reason=structured_call_global_user_target
target_shape_blocker_symbol=Stage1ModeContractBox.resolve_mode/0
```

However, the Program(JSON)->MIR bridge output still carried empty MIR-owned
route facts:

```json
"global_call_routes": [],
"lowering_plan": []
```

That means ny-llvmc can only report a generic structured-call stop. If the C
shim starts reconstructing call target facts from raw function names, Stage0
becomes a second MIR classifier.

## Decision

Refresh module semantic metadata in the Rust `env.mirbuilder.emit` bridge before
serializing MIR JSON.

This keeps the ownership line:

```text
Program(JSON)
  -> Rust bridge module handoff
  -> Canonical MIR with MIR-owned route facts
  -> LoweringPlan JSON
  -> ny-llvmc consumes facts only
```

The bridge already owns the Rust-side Program(JSON)->MIR compatibility capsule.
It must emit the same semantic metadata contract as normal Rust MIR compiler
output.

## Non-Goals

- no new `GlobalCallTargetShape`
- no new C body-specific emitter
- no C raw callee-name classifier
- no `.hako` source helper workaround
- no change to the explicit Program(JSON)->MIR compat capsule status

## Acceptance

```bash
cargo test --release refreshes_global_call_routes

rm -rf /tmp/p380q_phase29cg
KEEP_OUT_DIR=1 \
OUT_DIR=/tmp/p380q_phase29cg \
STAGE1_BIN=target/selfhost/hakorune.stage1_cli_env_seed \
NYASH_LLVM_SKIP_BUILD=1 \
bash tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh

bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected: bridge MIR JSON carries `global_call_routes` / `lowering_plan` for
same-module structured calls. P380Q does not yet change the C structured-call
diagnostic consumer; that belongs to the next card.

## Result

Implemented.

Both Rust bridge entry points now refresh MIR semantic metadata before emitting
MIR JSON:

- `Stage1ProgramJsonModuleHandoff`
- runtime `env.mirbuilder.emit`

The actual phase29cg bridge output now contains route facts. Example from
`/tmp/p380q_phase29cg/stage1_cli_env.mir.json`:

```text
function=main
global_call_routes[0].callee_name=Stage1ModeContractBox.resolve_mode/0
global_call_routes[0].target_exists=true
global_call_routes[0].reason=missing_multi_function_emitter
global_call_routes[0].target_shape_reason=generic_string_unsupported_extern_call
```

The ny-llvmc stderr still reports the P380P hardcoded structured-call
diagnostic. The next card must make structured `op:"call"` reuse the same
LoweringPlan unsupported-detail consumer as `op:"mir_call"`.

Validation:

```text
cargo test --release refreshes_global_call_routes
-> PASS: 2 tests

KEEP_OUT_DIR=1 OUT_DIR=/tmp/p380q_phase29cg \
  STAGE1_BIN=target/selfhost/hakorune.stage1_cli_env_seed \
  NYASH_LLVM_SKIP_BUILD=1 \
  bash tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh
-> emit_program_rc=0 emit_mir_rc=0 llvm_rc=4
-> bridge MIR JSON now has global_call_routes/lowering_plan facts
```
