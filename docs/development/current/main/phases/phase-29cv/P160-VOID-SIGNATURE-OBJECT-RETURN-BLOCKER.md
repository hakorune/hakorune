---
Status: Accepted
Decision: accepted
Date: 2026-05-02
Scope: phase-29cv P160, void-signature object return blocker evidence
Related:
  - docs/development/current/main/phases/phase-29cv/P159-I64-STRING-OR-VOID-GUARD.md
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - src/mir/global_call_route_plan/string_return_profile.rs
  - src/mir/global_call_route_plan/generic_string_body.rs
---

# P160: Void-Signature Object Return Blocker

## Problem

P159 moved the source-execution blocker to the collect-using owner:

```text
target_shape_blocker_symbol=Stage1UsingResolverBox._collect_using_entries/1
target_shape_blocker_reason=generic_string_return_abi_not_handle_compatible
```

Inspecting the target showed a `void` signature whose observed return values are
`void|null` or a locally constructed `ArrayBox`. The broad return ABI reason hid
the actual ownership boundary.

## Decision

Split that case into the existing object-return diagnostic:

```text
generic_string_return_object_abi_not_handle_compatible
```

The evidence is return-profile only:

- local `NewBox` / value-type evidence for non-`StringBox` objects
- propagation through `Copy` and `Phi`
- no permission for MapBox/ArrayBox lowering
- no collapse of child-global blockers into parent object evidence

Child global calls still use the existing deepest-blocker propagation contract.

## Evidence

The source-execution probe now reports the real collect-using object boundary:

```text
target_shape_reason=generic_string_global_target_shape_unknown
target_shape_blocker_symbol=Stage1UsingResolverBox._collect_using_entries/1
target_shape_blocker_reason=generic_string_return_object_abi_not_handle_compatible
```

This is still an unsupported pure shape. The change only improves the MIR-owned
diagnostic so the next implementation slice does not accidentally treat
`_collect_using_entries/1` as a string/void sentinel body.

## Acceptance

```bash
cargo test -q refresh_module_global_call_routes_marks_void_signature_object_or_void_return_reason --lib
cargo test -q refresh_module_global_call_routes_propagates_return_child_blocker_transitively --lib
cargo test -q global_call_routes --lib
cargo fmt --check
git diff --check
cargo build -q --release --bin hakorune
target/release/hakorune --emit-mir-json /tmp/hakorune_p160_void_object_boundary.mir.json lang/src/runner/stage1_cli_env.hako
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p160_stage1_cli_env.exe lang/src/runner/stage1_cli_env.hako
```

The final `--emit-exe` command is accepted as an advance-to-next-blocker probe,
not a green source-execution gate.
