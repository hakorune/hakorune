---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P237a, LowerIfNested int-or-var node numeric text ABI
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P236A-LOWER-IF-NESTED-CMP-SIDE-TEXT-ABI.md
  - lang/src/mir/builder/internal/lower_if_nested_box.hako
---

# P237a: Lower If Nested Int-Or-Var Text ABI

## Problem

P236a advances the source-exe probe to:

```text
target_shape_blocker_symbol=LowerIfNestedBox._read_int_or_var_node_after/3
target_shape_blocker_reason=generic_string_return_abi_not_handle_compatible
backend_reason=missing_multi_function_emitter
```

This helper is the ternary value-side companion of
`_read_cmp_side_int/4`. It also returns successful numeric reads directly from:

```text
JsonFragBox.read_int_after(...)
PatternUtilBox.find_local_int_before(...)
```

That leaks scalar/null flow into a generic string caller boundary.

## Decision

Keep the fix source-owned. `_read_int_or_var_node_after/3` should return
numeric text or null. Coerce successful numeric reads to text and split the
compound local guards into nested `if` statements.

## Non-Goals

- no new scalar-to-string route fact
- no new `GlobalCallTargetShape`
- no C body-specific emitter
- no generic boolean-expression acceptance expansion
- no change to nested ternary recognition policy

## Acceptance

Probe result should move past `LowerIfNestedBox._read_int_or_var_node_after/3`;
observed next blocker after the source cleanup:

```text
target_shape_blocker_symbol=LoopOptsBox.new_map/0
target_shape_blocker_reason=generic_string_return_object_abi_not_handle_compatible
backend_reason=missing_multi_function_emitter
```

Verification:

```bash
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p237a_lower_if_nested_int_or_var_text_abi.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
