---
Status: Accepted
Decision: accepted
Date: 2026-05-04
Scope: phase-29cv P380O, phase29cg LLVM failure diagnostic surface
Related:
  - docs/development/current/main/phases/phase-29cv/P380N-PROGRAM-JSON-MIR-BRIDGE-DELEGATE-ENV.md
  - tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh
  - tools/ny_mir_builder.sh
---

# P380O: phase29cg LLVM Diagnostic Surface

## Problem

After P380N, the phase29cg keeper reaches:

```text
[phase29cg] emit_program_rc=0 emit_mir_rc=0 llvm_rc=4 verify_rc=1
```

But the stored LLVM log only says:

```text
error: ny-llvmc failed
```

`ny_mir_builder.sh --quiet` suppresses backend diagnostics unless
`NYASH_LLVM_ROUTE_TRACE=1` is set. That leaves the keeper one step too far from
the real blocker.

## Decision

Keep `ny_mir_builder.sh` unchanged. Make the phase29cg keeper opt into route
trace by default for its local LLVM probe and print bounded LLVM/verify logs on
failure.

This is diagnostic-only: it does not change route selection, backend behavior,
or acceptance.

## Non-Goals

- no ny-llvmc behavior change
- no route/classifier/body-shape change
- no Stage0 change
- no bridge behavior change

## Acceptance

```bash
rm -rf /tmp/p380o_phase29cg
KEEP_OUT_DIR=1 \
OUT_DIR=/tmp/p380o_phase29cg \
STAGE1_BIN=target/selfhost/hakorune.stage1_cli_env_seed \
NYASH_LLVM_SKIP_BUILD=1 \
bash tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh

bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected: if LLVM still fails, the keeper prints the real `ny-llvmc`
diagnostic, including `llvm-pure/unsupported-shape`.

## Result

Implemented.

The phase29cg keeper now opts its local LLVM probe into route tracing and
prints bounded LLVM/verify stderr on failure. Remeasure output now exposes the
real next blocker:

```text
[phase29cg] emit_program_rc=0 emit_mir_rc=0 llvm_rc=4 verify_rc=1 verify_count=
[llvm-route/select] owner=boundary recipe=pure-first compat_replay=none symbol=hako_llvmc_compile_json_pure_first
[llvm-route/trace] stage=generic_walk result=seen reason=call extra=ii=1 dst=2 op=call
[llvm-route/replay] lane=none reason=unsupported_pure_shape
[llvm-pure/unsupported-shape] recipe=pure-first first_block=0 first_inst=1 first_op=call owner_hint=mir_normalizer reason=unknown_op target_return_type=- target_shape_reason=- target_shape_blocker_symbol=- target_shape_blocker_reason=-
```

The missing LLVM IR verify message is now clearly secondary:

```text
opt: /tmp/p380o_phase29cg/stage1_cli_env.ll: error: Could not open input file: No such file or directory
```

Next implementation should address the exposed MIR/LLVM boundary, not the
bridge capsule or output/log plumbing.
