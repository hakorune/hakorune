---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: classify unsupported user/global calls in LoweringPlan JSON v0.
Related:
  - docs/development/current/main/phases/phase-29cv/P111-STAGE1-ENV-MAIN-DISPATCHER-THINNING.md
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - lang/src/runner/stage1_cli_env.hako
  - src/mir/global_call_route_plan.rs
  - lang/c-abi/shims/hako_llvmc_ffi_pure_compile_generic_lowering.inc
---

# P112 Global Call Unsupported Lowering Plan

## Goal

Move the next full Stage1 env stop from backend-local discovery to
plan-builder classification.

After P111, the first pure-first stop is:

```text
mir_call Global Stage1ModeContractBox.resolve_mode/0
```

This is not a Program(JSON v0) bridge leak. It is the broader typed
user/global-call family. The clean next step is not to emit it yet. The clean
step is to put the unsupported shape into `metadata.lowering_plan` so the
backend can fail-fast because the plan says unsupported.

## Decision

- Add MIR-owned `global_call_routes` metadata.
- Add `LoweringPlan` entries with:
  - `source = "global_call_routes"`
  - `core_op = "UserGlobalCall"`
  - `tier = "Unsupported"`
  - `emit_kind = "unsupported"`
  - `proof = "typed_global_call_contract_missing"`
- Exclude `Global print`, because the current backend already has a narrow
  print surface.
- ny-llvmc must not lower this plan. It only reports the plan-backed
  unsupported reason.

## Acceptance

```bash
cargo test -q global_call_routes

target/release/hakorune \
  --emit-mir-json /tmp/p112_stage1_cli_env.mir.json \
  lang/src/runner/stage1_cli_env.hako

jq -e '
  [.functions[]
   | select(.name == "main")
   | .metadata.lowering_plan[]?
   | select(.source == "global_call_routes")
   | select(.core_op == "UserGlobalCall")
   | select(.tier == "Unsupported")
   | select(.callee_name == "Stage1ModeContractBox.resolve_mode/0")]
  | length == 1
' /tmp/p112_stage1_cli_env.mir.json

HAKO_BACKEND_COMPILE_RECIPE=pure-first \
HAKO_BACKEND_COMPAT_REPLAY=none \
NYASH_LLVM_ROUTE_TRACE=1 \
target/release/ny-llvmc \
  --in /tmp/p112_stage1_cli_env.mir.json \
  --emit obj \
  --out /tmp/p112_stage1_cli_env.o
```

The last command is expected to fail until typed user/global-call lowering is
implemented. The accepted failure reason is:

```text
reason=lowering_plan_unsupported_global_call
```
