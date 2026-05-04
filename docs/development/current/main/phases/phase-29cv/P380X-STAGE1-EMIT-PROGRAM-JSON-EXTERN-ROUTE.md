---
Status: Accepted
Decision: accepted
Date: 2026-05-04
Scope: phase-29cv P380X, Stage1 emit Program(JSON) extern route
Related:
  - docs/development/current/main/phases/phase-29cv/P380W-BUILDBOX-EMIT-PROGRAM-JSON-CANONICAL-CALL.md
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - src/mir/extern_call_route_plan.rs
  - src/runner/json_v0_bridge/lowering/expr/call_ops.rs
---

# P380X: Stage1 Emit Program(JSON) Extern Route

## Problem

P380W canonicalized the Program(JSON v0) bridge output for:

```hako
BuildBox.emit_program_json_v0(source_text, null)
```

from a runtime `boxcall` to the canonical static global call:

```text
BuildBox.emit_program_json_v0/2
```

That moved the previous method-call blocker, but the module still does not
contain a `BuildBox.emit_program_json_v0/2` function definition:

```text
target_shape_blocker_symbol=BuildBox.emit_program_json_v0/2
target_shape_blocker_reason=generic_string_global_target_missing
```

Adding a synthetic Hako body or another `GlobalCallTargetShape` would teach
Stage0 more compiler body semantics. The actual owner is already the Rust
kernel helper used by the existing Program(JSON) capsule:

```text
nyash.stage1.emit_program_json_v0_h(source_handle) -> program_json_handle
```

## Decision

Lower the source-only BuildBox entry to an explicit extern route when the opts
argument is statically `null`:

```text
BuildBox.emit_program_json_v0(source, null)
  -> Extern("nyash.stage1.emit_program_json_v0_h")(source)
```

Add a dedicated extern route kind for this helper. This keeps the route in
Canonical MIR / LoweringPlan metadata and lets ny-llvmc consume an explicit
runtime helper route without accepting arbitrary imported method calls or
inventing a missing Hako function body.

## Non-Goals

- no new `GlobalCallTargetShape`
- no synthetic `BuildBox` Hako function body
- no generic `boxcall` acceptance
- no generic imported alias method acceptance
- no VM/compat fallback
- no acceptance when the second argument is not statically `null`

## Acceptance

```bash
cargo test --release stage1_emit_program_json_extern_route -- --nocapture
cargo test --release imported_alias_qualified_call -- --nocapture

bash tools/build_hako_llvmc_ffi.sh

rm -rf /tmp/p380x_phase29cg
KEEP_OUT_DIR=1 \
OUT_DIR=/tmp/p380x_phase29cg \
STAGE1_BIN=target/selfhost/hakorune.stage1_cli_env_seed \
NYASH_LLVM_SKIP_BUILD=1 \
bash tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh

bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected: the blocker moves beyond `BuildBox.emit_program_json_v0/2` missing
target without adding a body shape.

## Result

Accepted.

Verified:

```bash
cargo test --release stage1_emit_program_json_extern_route -- --nocapture
cargo test --release imported_alias_qualified_call -- --nocapture

bash tools/build_hako_llvmc_ffi.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

The phase29cg replay now passes the Program(JSON) emit route without teaching
Stage0 a `BuildBox.emit_program_json_v0/2` body shape. The remaining blocker
moved to the next owner-local stage1 validation helper:

```text
target_shape_blocker_symbol=Stage1ProgramResultValidationBox.finalize_emit_result/1
target_shape_blocker_reason=generic_string_return_not_string
backend_reason=missing_multi_function_emitter
```

This preserves the P207A rule: the bridge emits an explicit Canonical MIR
extern call, LoweringPlan records the route fact, and ny-llvmc consumes that
fact instead of rediscovering compiler body semantics.
