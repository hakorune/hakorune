---
Status: Accepted
Decision: accepted
Date: 2026-05-04
Scope: phase-29cv P381D, Stage1 Program(JSON)-to-MIR extern route
Related:
  - docs/development/current/main/phases/phase-29cv/P381B-STAGE1-EMIT-MIR-SOURCE-EXTERN-ROUTE.md
  - docs/development/current/main/phases/phase-29cv/P381C-MODULE-GENERIC-PRINT-ARG0.md
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - src/runner/json_v0_bridge/lowering/expr/call_ops.rs
  - src/mir/extern_call_route_plan.rs
---

# P381D: Stage1 Emit MIR Program(JSON) Extern Route

## Problem

P381C advanced the phase29cg replay to the explicit Program(JSON) compatibility
path:

```text
reason=missing_multi_function_emitter
target_shape_blocker_symbol=Stage1ProgramJsonMirCallerBox._emit_mir_from_program_json_text_checked/2
target_shape_blocker_reason=generic_string_unsupported_method_call
```

The body is the same Stage1 authority handoff shape as P381B, but for already
materialized Program(JSON):

```text
MirBuilderBox.emit_from_program_json_v0(program_json_text, null)
```

Leaving this as an imported method call would require a `MirBuilderBox` body
shape or generic imported method acceptance.

## Decision

Lower the exact Program(JSON)-to-MIR call to an explicit extern route when the
options argument is statically `null`:

```text
MirBuilderBox.emit_from_program_json_v0(program_json, null)
  -> Extern("nyash.stage1.emit_mir_from_program_json_v0_h")(program_json)
```

The route is recorded in `extern_call_routes` / `LoweringPlan` and consumed by
ny-llvmc as a runtime helper route.

## Non-Goals

- no new `GlobalCallTargetShape`
- no synthetic `MirBuilderBox` Hako body
- no generic imported alias method acceptance
- no generic `boxcall` acceptance
- no acceptance when the second argument is not statically `null`
- no VM/compat fallback

## Acceptance

```bash
cargo test --release stage1_emit_mir_from_program_json_extern_route -- --nocapture
cargo test --release refresh_function_extern_call_routes_records_stage1_emit_mir_from_program_json_extern_route -- --nocapture
cargo test -p nyash_kernel --release stage1_emit_mir_from_program_json_v0_h -- --nocapture

bash tools/build_hako_llvmc_ffi.sh

rm -rf /tmp/p381d_phase29cg
KEEP_OUT_DIR=1 \
OUT_DIR=/tmp/p381d_phase29cg \
STAGE1_BIN=target/selfhost/hakorune.stage1_cli_env_seed \
NYASH_LLVM_SKIP_BUILD=1 \
bash tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh

bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected: the blocker moves beyond
`Stage1ProgramJsonMirCallerBox._emit_mir_from_program_json_text_checked/2`.

## Result

Accepted.

The route and kernel helper tests passed:

```text
cargo test --release stage1_emit_mir_from_program_json_extern_route -- --nocapture
cargo test -p nyash_kernel --release stage1_emit_mir_from_program_json_v0_h -- --nocapture
bash tools/build_hako_llvmc_ffi.sh
```

The phase29cg replay moved beyond
`Stage1ProgramJsonMirCallerBox._emit_mir_from_program_json_text_checked/2`.
The next blocker is the Program(JSON) compat mode input guard:

```text
reason=missing_multi_function_emitter
target_shape_blocker_symbol=Stage1ProgramJsonTextGuardBox.coerce_text_checked/3
target_shape_blocker_reason=generic_string_unsupported_void_sentinel_const
```
