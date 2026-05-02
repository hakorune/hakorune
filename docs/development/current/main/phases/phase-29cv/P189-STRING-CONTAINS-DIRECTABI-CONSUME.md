---
Status: Accepted
Decision: accepted
Date: 2026-05-02
Scope: phase-29cv P189, String contains DirectAbi route consume
Related:
  - docs/development/current/main/phases/phase-29cv/P187-GENERIC-I64-SELECT-FLOW.md
  - docs/development/current/main/phases/phase-29cv/P188-STRING-LASTINDEXOF-CORE-METHOD-MANIFEST-SYNC.md
  - src/mir/generic_method_route_plan.rs
  - src/mir/global_call_route_plan/generic_string_body.rs
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_function_emit.inc
---

# P189: String Contains DirectAbi Consume

## Problem

After P187, the real `stage1_cli_env.hako` probe moved to:

```text
target_shape_blocker_symbol=CallMethodizeBox._find_func_name/3
target_shape_blocker_reason=generic_string_unsupported_method_call
```

The target helper returns `StringOrVoid` and uses:

```text
name.contains(".")
name2.contains(".")
```

This is a string predicate surface, not a JsonFrag normalizer shape. Adding it
as ad-hoc logic in the C shim would recreate backend-local classification, so
the route must be owned by MIR metadata.

## Decision

Add `StringBox.contains/1` as a CoreMethod-backed generic method route:

```text
route_id=generic_method.contains
core_op=StringContains
route_kind=string_contains
proof=contains_surface_policy
helper=nyash.string.contains_hh
return_shape=scalar_i64
value_demand=scalar_i64
publication_policy=no_publication
tier=DirectAbi
```

The Rust generic string/i64 body classifiers may observe `contains` as a
Bool-return string predicate. The C shim consumes the LoweringPlan method view
and emits the direct helper call; it does not infer the method surface from raw
body semantics.

## Result

`CallMethodizeBox._find_func_name/3` classifies and emits through the direct
string predicate route. The real `stage1_cli_env.hako` probe moved past the
previous blocker and now stops at:

```text
target_shape_blocker_symbol=PatternRegistryBox.candidates/0
target_shape_blocker_reason=generic_string_return_object_abi_not_handle_compatible
```

## Acceptance

```bash
cargo test -q string_contains --lib
cargo test -q global_call_routes --lib
cargo test -q manifest_core_ops_are_known_by_mir_carrier --lib
cargo fmt --check
bash tools/build_hako_llvmc_ffi.sh
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p189_contains_probe.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
