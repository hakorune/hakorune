---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P207e, JsonFragBox.read_bool_from direct scalar/null return
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P207D-LOWER-LOOP-LOCAL-RETURN-VAR-GUARD-SPLIT.md
  - docs/development/current/main/CURRENT_STATE.toml
  - lang/src/shared/json/utils/json_frag.hako
---

# P207e: JsonFrag read_bool_from Direct Scalar

## Problem

P207d moved the source-execution probe to:

```text
target_shape_blocker_symbol=JsonFragBox.read_bool_from/2
target_shape_blocker_reason=generic_string_return_abi_not_handle_compatible
backend_reason=missing_multi_function_emitter
```

`read_bool_from/2` is a scalar/null helper:

```text
"t" -> 1
"f" -> 0
otherwise -> null
```

The source currently delegates the final character decision to
`_to_bool10/1`. That callee also returns `null` on unknown input, and the MIR
metadata for the call result is currently `void`. This hides the scalar return
from the caller body and prevents the existing generic-i64 scalar lane from
classifying `read_bool_from/2`.

## Decision

Do not widen `generic_i64_body` or add a JsonFrag-specific bool reader shape.

Inline the final `t/f/null` return split inside `read_bool_from/2`:

```text
if ch == "t" { return 1 }
if ch == "f" { return 0 }
return null
```

Keep `_to_bool10/1` in place for existing callers; this card only removes the
callee-result dependency from `read_bool_from/2`.

## Boundary

This card may only change `JsonFragBox.read_bool_from/2`.

It must not:

- change `_to_bool10/1`
- change `read_bool_after/2`
- change integer/string/float read helpers
- add or widen a body classifier
- add ny-llvmc semantics

## Probe Contract

Before this card, the stage probe stopped at:

```text
JsonFragBox.read_bool_from/2
generic_string_return_abi_not_handle_compatible
```

After this card, that blocker should disappear. A later stop is the next
blocker, not a regression.

## Probe Result

The `--emit-exe` probe no longer stops at
`JsonFragBox.read_bool_from/2`. The next observed stop is:

```text
target_shape_blocker_symbol=BoxTypeInspectorBox.is_map/1
target_shape_blocker_reason=generic_string_global_target_shape_unknown
backend_reason=missing_multi_function_emitter
```

This is not the P207c `unsupported_method_call` failure. The current stop is a
transitive target-shape blocker in a deeper MIR/JSON emit path.

## Acceptance

```bash
cargo test -q global_call_routes --lib
cargo fmt --check
bash tools/build_hako_llvmc_ffi.sh
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p207e_jsonfrag_read_bool_direct_scalar.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
