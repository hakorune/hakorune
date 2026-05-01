---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: add the ny-llvmc C LoweringPlan view for `global_call_routes`.
Related:
  - docs/development/current/main/phases/phase-29cv/P113-GLOBAL-CALL-TARGET-CONTRACT-INVENTORY.md
  - docs/development/current/main/phases/phase-29cv/P114-GENERIC-PURE-MODULE-VIEW-ENTRY-SPLIT.md
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - lang/c-abi/shims/hako_llvmc_ffi_lowering_plan_metadata.inc
---

# P115 Global Call LoweringPlan View

## Stop Line

P113 made `global_call_routes` carry target facts. P114 made the generic pure
program reader module-shaped. The remaining C-side gap before a real
multi-function emitter is that `global_call_routes` was still consumed at the
failure site by hand-reading `source`, `tier`, and `reason`.

That is too close to backend-local raw JSON interpretation.

## Change

Add `LoweringPlanGlobalCallView` beside the existing generic-method and extern
views:

```text
source = global_call_routes
proof = typed_global_call_contract_missing
core_op = UserGlobalCall
tier = Unsupported
callee_name / arity / target facts / reason
```

The generic pure unsupported path now reads this view and surfaces
`global_view.reason`. It no longer has a local `source == "global_call_routes"`
ladder at the failure site.

This is still diagnostic-only. It does not make user/global calls lowerable.

## Next Implementation Boundary

The multi-function emitter should extend this same view rather than adding a
new raw reader:

```text
read_lowering_plan_global_call_view(...)
  -> validate target_exists && arity_matches
  -> validate supported tier/emit_kind
  -> emit quoted call
```

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
target/release/hakorune --emit-mir-json /tmp/p115_stage1_cli_env.mir.json \
  lang/src/runner/stage1_cli_env.hako
HAKO_BACKEND_COMPILE_RECIPE=pure-first \
HAKO_BACKEND_COMPAT_REPLAY=none \
NYASH_LLVM_ROUTE_TRACE=1 \
target/release/ny-llvmc --in /tmp/p115_stage1_cli_env.mir.json \
  --emit obj --out /tmp/p115_stage1_cli_env.o
```

The expected failure remains:

```text
reason=missing_multi_function_emitter
```

