---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: enrich `global_call_routes` with target-contract facts before ny-llvmc
  learns multi-function emission.
Related:
  - docs/development/current/main/phases/phase-29cv/P112-GLOBAL-CALL-UNSUPPORTED-LOWERING-PLAN.md
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - src/mir/global_call_route_plan.rs
  - src/runner/mir_json_emit/root.rs
  - lang/c-abi/shims/hako_llvmc_ffi_pure_compile_generic_lowering.inc
---

# P113 Global Call Target Contract Inventory

## Stop Line

P112 moved `mir_call Global ...` failures from backend-local route discovery
into MIR-owned `global_call_routes` / `LoweringPlan tier=Unsupported`.

The next blocker is not a new method surface:

```text
main
  mir_call Global Stage1ModeContractBox.resolve_mode/0
```

`Stage1ModeContractBox.resolve_mode/0` is present in the same MIR module. The
generic pure emitter still emits only the selected entry function body, so it
cannot yet compile typed user/global calls without a multi-function emitter.

Do not fix this by adding a `.inc` matcher for
`Stage1ModeContractBox.resolve_mode/0`.

## Contract

`global_call_routes` now records target facts from the MIR module:

- `target_exists`
- `target_arity`
- `arity_matches`
- `reason`

The valid v0 reasons are:

| reason | meaning |
| --- | --- |
| `missing_multi_function_emitter` | the target function exists and arity matches, but ny-llvmc generic pure still emits only the entry function |
| `global_call_arity_mismatch` | the target function exists but call arity does not match the MIR function arity |
| `unknown_global_callee` | the target is not a function in the current MIR module |

All three reasons remain `tier=Unsupported`. This card is diagnostic and
contract inventory only; it does not make global calls lowerable.

## Backend Rule

ny-llvmc may surface the plan `reason` in
`[llvm-pure/unsupported-shape]`, but it must not classify raw global names.

The first implementation that makes this shape lowerable must be a
multi-function emitter card:

```text
MIR module functions
  -> per-function generic pure state
  -> quoted LLVM function definitions
  -> global call sites emit typed i64 calls by plan
```

The emitter may validate target arity and symbol spelling from the plan. It
must not rediscover semantics from `Stage1ModeContractBox.*` names.

## Acceptance

```bash
cargo test -q global_call_routes
cargo build --release --bin hakorune
bash tools/build_hako_llvmc_ffi.sh
target/release/hakorune --emit-mir-json /tmp/p113_stage1_cli_env.mir.json \
  lang/src/runner/stage1_cli_env.hako
jq '[.functions[] | select(.name == "main")
  | .metadata.lowering_plan[]?
  | select(.source == "global_call_routes")
  | select(.callee_name == "Stage1ModeContractBox.resolve_mode/0")
  | select(.reason == "missing_multi_function_emitter")] | length == 1' \
  /tmp/p113_stage1_cli_env.mir.json
HAKO_BACKEND_COMPILE_RECIPE=pure-first \
HAKO_BACKEND_COMPAT_REPLAY=none \
NYASH_LLVM_ROUTE_TRACE=1 \
target/release/ny-llvmc --in /tmp/p113_stage1_cli_env.mir.json \
  --emit obj --out /tmp/p113_stage1_cli_env.o
```

The last command is expected to fail until the multi-function emitter lands.
The expected failure reason is:

```text
reason=missing_multi_function_emitter
```

