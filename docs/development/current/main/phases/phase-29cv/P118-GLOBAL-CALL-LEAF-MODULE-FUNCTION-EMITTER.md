---
Status: Active
Decision: accepted
Date: 2026-05-01
Scope: phase-29cv P118, ny-llvmc pure-first same-module user/global-call lowering
Related:
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - docs/development/current/main/phases/phase-29cv/P117-GLOBAL-CALL-DIRECT-TARGET-VALIDATOR.md
  - src/mir/global_call_route_plan.rs
  - lang/c-abi/shims/hako_llvmc_ffi_pure_compile.inc
  - lang/c-abi/shims/hako_llvmc_ffi_module_leaf_function_emit.inc
---

# P118: Global Call Leaf Module Function Emitter

## Problem

P117 proves that a `UserGlobalCall` site has a same-module target symbol and
matching arity, but the generic pure backend still emits only the selected
entry function body. Emitting a call to that symbol without also emitting the
callee body would silently externalize a same-module function.

That is not a clean compiler boundary. The backend may call a same-module
target only after the target body has been emitted in the same LLVM module.

## Decision

Add the first narrow module-function emission slice:

```text
Canonical MIR module
  -> global_call_routes target_shape = numeric_i64_leaf
  -> LoweringPlan tier = DirectAbi, emit_kind = direct_function_call
  -> ny-llvmc emits the leaf function definition
  -> ny-llvmc emits UserGlobalCall only to an emitted leaf symbol
```

This is intentionally not the full multi-function emitter. It is a sealed leaf
contract that proves same-module calls without externalizing the callee.

## Accepted Leaf Shape

A target function is `numeric_i64_leaf` only when all conditions hold:

- target exists in the same MIR module
- target arity matches the call arity
- target has one basic block
- target params and return are integer-shaped for the current i64 ABI slice
- target body uses only numeric `const`, `copy`, `binop`, and `ret`
- `binop` is one of `Add`, `Sub`, `Mul`, `Div`, or `Mod`
- no string constants, calls, allocation, lifecycle ops, PHI, or branches

The C emitter validates this body shape again before emitting the definition.
That validation is an emitter safety check, not a second semantic classifier.
The semantic permission still comes from MIR-owned `LoweringPlan` metadata.

## LoweringPlan v0 Fields

For a lowerable leaf global call:

```json
{
  "source": "global_call_routes",
  "source_route_id": "global.user_call",
  "core_op": "UserGlobalCall",
  "tier": "DirectAbi",
  "emit_kind": "direct_function_call",
  "target_shape": "numeric_i64_leaf",
  "target_symbol": "Helper.add/2",
  "symbol": "Helper.add/2",
  "proof": "typed_global_call_leaf_numeric_i64",
  "return_shape": "ScalarI64",
  "value_demand": "scalar_i64",
  "reason": null
}
```

For non-leaf same-module functions, the P117 stop-line remains:

```text
tier=Unsupported
reason=missing_multi_function_emitter
```

## ny-llvmc Rules

Allowed:

- emit `define i64 @"target"(...)` for a validated `numeric_i64_leaf`
- record the emitted leaf symbol in a module-local table
- emit `call i64 @"target"(...)` only when the plan is DirectAbi and the
  target symbol was emitted as a leaf definition

Forbidden:

- emitting a call to a same-module user function that has only a declaration
- treating `missing_multi_function_emitter` as permission to lower
- adding raw callee-name matchers in `.inc`
- expanding this card to string/env/branch/PHI functions

## Acceptance

- A generated MIR route for a numeric i64 leaf target reports:
  `tier=DirectAbi`, `emit_kind=direct_function_call`,
  `proof=typed_global_call_leaf_numeric_i64`, and
  `target_shape=numeric_i64_leaf`.
- ny-llvmc can emit an object for a canary where `main` calls a same-module
  numeric leaf function.
- The object contains a defined target symbol, proving the call was not
  externalized.
- `lang/src/runner/stage1_cli_env.hako` still stops at
  `missing_multi_function_emitter` because its target is not a leaf.
