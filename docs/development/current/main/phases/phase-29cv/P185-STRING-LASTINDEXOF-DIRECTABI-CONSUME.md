---
Status: Accepted
Decision: accepted
Date: 2026-05-02
Scope: phase-29cv P185, String lastIndexOf DirectAbi route consume
Related:
  - docs/development/current/main/phases/phase-29cv/P184-JSONFRAG-NORMALIZER-DIRECT-SHAPE-EMIT-WIRING.md
  - src/mir/core_method_op.rs
  - src/mir/generic_method_route_plan.rs
  - src/mir/global_call_route_plan/generic_i64_body.rs
  - src/mir/global_call_route_plan/generic_string_body.rs
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_function_emit.inc
---

# P185: String lastIndexOf DirectAbi Consume

## Problem

P184 moved the real `stage1_cli_env.hako` probe past the JsonFrag normalizer
collection body. The next blocker was a separate reverse string-scan helper:

```text
target_shape_blocker_symbol=JsonFragBox.last_index_of_from/3
target_shape_blocker_reason=generic_string_unsupported_method_call
```

The target body uses the existing string surface:

```text
RuntimeDataBox.length()
RuntimeDataBox.substring(start, end)
RuntimeDataBox.lastIndexOf(needle)
```

This is not normalizer collection semantics. It must not be added to
`generic_string_body.rs` as a JsonFrag-specific body rule.

## Decision

Add `lastIndexOf(needle)` as a MIR-owned generic method route:

```text
route_id=generic_method.lastIndexOf
core_op=StringLastIndexOf
route_kind=string_last_indexof
proof=lastindexof_surface_policy
helper=nyash.string.lastIndexOf_hh
return_shape=scalar_i64
value_demand=scalar_i64
publication_policy=no_publication
tier=DirectAbi
```

The Rust route classifier owns the eligibility decision. The C shim consumes
the LoweringPlan method view and emits the direct helper call without
reclassifying body semantics.

Only the one-argument surface is accepted in this card. The two-argument
`lastIndexOf(needle, start)` surface has different clamp semantics and should
be routed by a separate card if it becomes a blocker.

## Result

`JsonFragBox.last_index_of_from/3` classifies as a generic scalar i64 body
when it implements the reverse scan through `substring(...).lastIndexOf(...)`.

This keeps the P182 boundary intact:

- `generic_string_body.rs` may observe string-scan surfaces.
- `jsonfrag_normalizer_body.rs` remains the collection normalizer shape.
- The backend emits from LoweringPlan route facts.

The real `stage1_cli_env.hako` probe moved past the previous blocker and now
stops at the next methodize body:

```text
target_shape_blocker_symbol=CallMethodizeBox.methodize_calls_in_mir/1
target_shape_blocker_reason=generic_string_unsupported_instruction
```

## Acceptance

```bash
cargo test -q lastindexof --lib
cargo test -q global_call_routes --lib
cargo fmt --check
bash tools/build_hako_llvmc_ffi.sh
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p185_lastindexof_probe.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
