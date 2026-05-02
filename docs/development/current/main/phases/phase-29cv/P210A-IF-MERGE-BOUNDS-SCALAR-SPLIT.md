---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P210a, split LowerIfMergeLocalReturnVar bounds object into scalars
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P209C-MIR-JSON-STATIC-FIELD-READ-INLINE.md
  - lang/src/mir/builder/internal/lower_if_merge_local_return_var_box.hako
---

# P210a: If-Merge Bounds Scalar Split

## Problem

P209c advances the source-exe probe to:

```text
target_shape_blocker_symbol=LowerIfMergeLocalReturnVarBox._parse_bounds/1
target_shape_blocker_reason=generic_string_return_object_abi_not_handle_compatible
backend_reason=missing_multi_function_emitter
```

`_parse_bounds/1` parses a `"lb:rb"` text and returns a temporary map:

```hako
return %{
  "lb" => lb,
  "rb" => rb
}
```

The caller immediately reads `then_bounds.get("lb")`,
`then_bounds.get("rb")`, `else_bounds.get("lb")`, and
`else_bounds.get("rb")`. Teaching Stage0 this helper as an object-return body
would add another selfhost compiler helper shape.

## Decision

Keep the bounds parse local to `try_lower` and expose only scalar locals:

```text
then_bounds_text -> then_lb / then_rb
else_bounds_text -> else_lb / else_rb
```

This removes the object-return helper and avoids map construction/get routes for
data that never needs to cross a helper boundary.

## Non-Goals

- no new object-return `GlobalCallTargetShape`
- no generic map constructor/get acceptance for this helper
- no C body-specific emitter
- no change to `PatternUtilBox.then_array_bounds` /
  `else_array_bounds_after_then`

## Acceptance

Probe result should move past `_parse_bounds/1`; a later blocker may remain:

```bash
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p210a_bounds_scalar.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

