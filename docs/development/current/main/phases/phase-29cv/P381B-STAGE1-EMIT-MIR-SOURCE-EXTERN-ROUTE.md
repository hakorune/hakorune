---
Status: Accepted
Decision: accepted
Date: 2026-05-04
Scope: phase-29cv P381B, Stage1 source-to-MIR extern route
Related:
  - docs/development/current/main/phases/phase-29cv/P381A-MODULE-GENERIC-STAGE1-EXTERN-PREPASS.md
  - docs/development/current/main/phases/phase-29cv/P380X-STAGE1-EMIT-PROGRAM-JSON-EXTERN-ROUTE.md
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - src/runner/json_v0_bridge/lowering/expr/call_ops.rs
  - src/mir/extern_call_route_plan.rs
---

# P381B: Stage1 Emit MIR Source Extern Route

## Problem

P381A advanced the phase29cg replay to the source-to-MIR dispatch path:

```text
reason=missing_multi_function_emitter
target_shape_blocker_symbol=Stage1EmitMirDispatchBox.run_emit_mir_mode/1
target_shape_blocker_reason=generic_string_unsupported_void_sentinel_const
```

The first unsupported call inside that path is the owner-local source authority:

```text
Stage1SourceMirAuthorityBox.emit_mir_from_source/2
  -> Stage1SourceMirAuthorityBox._emit_mir_from_source_checked/2
  -> MirBuilderBox.emit_from_source_v0(source, null)
```

The last call is currently emitted as a generic imported method/`boxcall`, which
would require Stage0 to understand the `MirBuilderBox` body or accept a wider
imported method-call surface.

## Decision

Lower the exact source-only MirBuilder call to an explicit extern route when
the options argument is statically `null`:

```text
MirBuilderBox.emit_from_source_v0(source, null)
  -> Extern("nyash.stage1.emit_mir_from_source_v0_h")(source)
```

Record this as MIR-owned `extern_call_routes` / `LoweringPlan` metadata and let
ny-llvmc consume that route fact.

This mirrors P380X for Program(JSON) while keeping the route source-owned:

```text
source -> Canonical MIR externcall -> LoweringPlan -> ny-llvmc emitter -> Rust kernel
```

## Non-Goals

- no new `GlobalCallTargetShape`
- no synthetic `MirBuilderBox` Hako function body
- no generic imported alias method acceptance
- no generic `boxcall` acceptance
- no acceptance when the second argument is not statically `null`
- no VM/compat fallback

## Acceptance

```bash
cargo test --release stage1_emit_mir_from_source_extern_route -- --nocapture
cargo test --release refresh_function_extern_call_routes_records_stage1_emit_mir_from_source_extern_route -- --nocapture
cargo test -p nyash_kernel --release stage1_emit_mir_from_source_v0_h -- --nocapture

bash tools/build_hako_llvmc_ffi.sh

rm -rf /tmp/p381b_phase29cg
KEEP_OUT_DIR=1 \
OUT_DIR=/tmp/p381b_phase29cg \
STAGE1_BIN=target/selfhost/hakorune.stage1_cli_env_seed \
NYASH_LLVM_SKIP_BUILD=1 \
bash tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh

bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected: the blocker moves beyond
`Stage1SourceMirAuthorityBox._emit_mir_from_source_checked/2` without adding a
body shape.

## Result

Accepted. The phase29cg replay moved beyond
`Stage1SourceMirAuthorityBox._emit_mir_from_source_checked/2` and routed
`Stage1SourceMirAuthorityBox.emit_mir_from_source/2` as DirectAbi without adding
a body shape.

The next blocker is:

```text
reason=module_generic_body_emit_failed
target_shape_blocker_symbol=Stage1MirPayloadContractBox._fail_invalid_mir_text/1
```

This keeps the P207A boundary intact: the exact source-only MirBuilder handoff
is now an explicit extern route, not a generic `MirBuilderBox` method body
clone.
