---
Status: Active
Decision: accepted
Date: 2026-05-02
Scope: phase-29cv P190, static String ArrayBox factory body shape
Related:
  - docs/development/current/main/phases/phase-29cv/P189-STRING-CONTAINS-DIRECTABI-CONSUME.md
  - docs/development/current/main/phases/phase-29cv/P138-GLOBAL-CALL-OBJECT-RETURN-ABI-BLOCKER.md
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - lang/src/mir/builder/pattern_registry.hako
  - src/mir/global_call_route_plan.rs
---

# P190: Static String Array Body Shape

## Problem

P189 moved the source-execution blocker to the MirBuilder registry candidate
factory:

```text
target_shape_blocker_symbol=PatternRegistryBox.candidates/0
target_shape_blocker_reason=generic_string_return_object_abi_not_handle_compatible
```

`PatternRegistryBox.candidates/0` is not a string body. It creates one
`ArrayBox`, pushes static `StringBox` constants, and returns the same array
handle.

Treating this as `generic_pure_string_body` would violate the P138 object-return
boundary and would make the generic string classifier continue growing into a
second compiler.

## Decision

Add a dedicated MIR-owned target shape:

```text
target_shape=static_string_array_body
proof=typed_global_call_static_string_array
return_shape=array_handle
value_demand=runtime_i64_or_handle
```

This shape is an object-handle DirectAbi body, not a string body.

## Acceptance Shape

The classifier accepts only a narrow static factory:

- zero parameters
- one `ArrayBox` birth
- string constants
- `ArrayBox.push` / `RuntimeDataBox.push` calls that push string values into
  the born array
- copies preserving the array or string values
- return of the born array handle

## Forbidden

- accepting arbitrary ArrayBox-return helpers
- accepting MapBox/object factories through this shape
- treating array handles as `string_handle`
- adding normalizer, registry policy, or by-name `PatternRegistryBox` logic
- adding new behavior to `generic_string_body.rs`

## Expected Evidence

After this card, the route for `PatternRegistryBox.candidates/0` should become
direct:

```text
target_shape=static_string_array_body
proof=typed_global_call_static_string_array
return_shape=array_handle
```

The source-execution probe should advance to the next blocker after
`PatternRegistryBox.candidates/0`.

## Probe Result

P190 advanced beyond `PatternRegistryBox.candidates/0`. The next blocker is:

```text
target_shape_blocker_symbol=BuilderFinalizeChainBox._methodize_if_enabled_checked/1
target_shape_blocker_reason=generic_string_unsupported_void_sentinel_const
```

This keeps P190 scoped to the static array factory and leaves the new
void-sentinel blocker for the next card.

## Acceptance

```bash
cargo test -q static_string_array --lib
cargo test -q global_call_routes --lib
cargo test -q build_mir_json_root_emits_direct_plan_for_static_string_array_body --lib
cargo fmt --check
bash tools/build_hako_llvmc_ffi.sh
cargo build --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p190_static_string_array_probe.exe lang/src/runner/stage1_cli_env.hako
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

The `--emit-exe` command is a next-blocker probe, not a full green
source-execution gate.
