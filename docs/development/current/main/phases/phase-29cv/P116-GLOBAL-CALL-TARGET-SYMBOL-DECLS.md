---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: expose same-module global-call target symbols in LoweringPlan and emit
  module function declarations in ny-llvmc generic pure.
Related:
  - docs/development/current/main/phases/phase-29cv/P113-GLOBAL-CALL-TARGET-CONTRACT-INVENTORY.md
  - docs/development/current/main/phases/phase-29cv/P114-GENERIC-PURE-MODULE-VIEW-ENTRY-SPLIT.md
  - docs/development/current/main/phases/phase-29cv/P115-GLOBAL-CALL-LOWERING-PLAN-VIEW.md
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - src/mir/global_call_route_plan.rs
  - lang/c-abi/shims/hako_llvmc_ffi_pure_compile.inc
---

# P116 Global Call Target Symbol Declarations

## Stop Line

P115 gave ny-llvmc a typed view for `global_call_routes`, but the next emitter
still needs a symbol contract and module declarations. Without that, the first
tempting implementation would be a raw `callee_name` call emitter.

## Change

`global_call_routes` now exposes:

```text
target_symbol = callee_name when target_exists
target_symbol = null when the callee is unknown
```

The C generic pure module view now emits `declare i64 @"<function>"(...)` rows
for every non-entry MIR function before the selected entry definition. Function
parameters are read from JSON `params[]` and represented as `i64` in this v0
ABI.

This is still not the multi-function emitter. `UserGlobalCall` remains
`tier=Unsupported`, and the expected Stage1 env stop remains
`missing_multi_function_emitter`.

## Rules

- Call emission must use `target_symbol` from the plan view.
- The backend must not derive symbols from concrete global names at the call
  site.
- Quoted LLVM symbols are accepted only when they contain no quote, backslash,
  or control byte. P116 declaration emission skips invalid names; the later
  call-emitter card must fail-fast if an invalid planned symbol is required.

## Acceptance

```bash
cargo test -q global_call_routes
bash tools/build_hako_llvmc_ffi.sh
target/release/hakorune --emit-mir-json /tmp/p116_stage1_cli_env.mir.json \
  lang/src/runner/stage1_cli_env.hako
jq '[.functions[] | select(.name == "main")
  | .metadata.lowering_plan[]?
  | select(.source == "global_call_routes")
  | select(.callee_name == "Stage1ModeContractBox.resolve_mode/0")
  | select(.target_symbol == "Stage1ModeContractBox.resolve_mode/0")] | length == 1' \
  /tmp/p116_stage1_cli_env.mir.json
HAKO_BACKEND_COMPILE_RECIPE=pure-first \
HAKO_BACKEND_COMPAT_REPLAY=none \
NYASH_LLVM_ROUTE_TRACE=1 \
target/release/ny-llvmc --in /tmp/p116_stage1_cli_env.mir.json \
  --emit obj --out /tmp/p116_stage1_cli_env.o
```

The last command still fails by design:

```text
reason=missing_multi_function_emitter
```

