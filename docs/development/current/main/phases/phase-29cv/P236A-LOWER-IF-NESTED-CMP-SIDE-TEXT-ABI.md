---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P236a, LowerIfNested compare-side numeric text ABI
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P235A-COMPAT-MIR-EXTERNCALL-DIRECT-JSON.md
  - lang/src/mir/builder/internal/lower_if_nested_box.hako
---

# P236a: Lower If Nested Compare-Side Text ABI

## Problem

P235a advances the source-exe probe to:

```text
target_shape_blocker_symbol=LowerIfNestedBox._read_cmp_side_int/4
target_shape_blocker_reason=generic_string_return_not_string
backend_reason=missing_multi_function_emitter
```

`_read_cmp_side_int/4` is used by a generic string MIR(JSON) emitter path, but
its success values can arrive from numeric scalar routes:

```text
JsonFragBox.read_int_after(...)
PatternUtilBox.find_local_int_before(...)
```

Returning those directly creates scalar/null flow at a string caller boundary.

## Decision

Keep the fix source-owned. `_read_cmp_side_int/4` should expose the same shape
as its consumer needs: numeric text or null. Coerce successful numeric reads to
text before returning and leave null as the explicit unsupported/no-match
sentinel.

Also split the helper's compound guards into nested `if` statements so Stage0
does not need extra boolean expression shapes for this owner-local probe.

## Non-Goals

- no new scalar-to-string route fact
- no new `GlobalCallTargetShape`
- no C body-specific emitter
- no generic boolean-expression acceptance expansion
- no change to the nested-if recognition policy

## Acceptance

Probe result should move past `LowerIfNestedBox._read_cmp_side_int/4`.
Observed next blocker after the source cleanup:

```text
target_shape_blocker_symbol=LowerIfNestedBox._read_int_or_var_node_after/3
target_shape_blocker_reason=generic_string_return_abi_not_handle_compatible
backend_reason=missing_multi_function_emitter
```

Verification:

```bash
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p236a_lower_if_nested_cmp_side_text_abi.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
