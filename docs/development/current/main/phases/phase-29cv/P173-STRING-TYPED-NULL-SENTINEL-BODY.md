---
Status: Accepted
Decision: accepted
Date: 2026-05-02
Scope: phase-29cv P173, string-typed null sentinel body
Related:
  - docs/development/current/main/phases/phase-29cv/P172-GENERIC-STRING-ORDERED-COMPARE.md
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - src/mir/global_call_route_plan.rs
  - src/mir/global_call_route_plan/generic_string_body.rs
---

# P173: String-Typed Null Sentinel Body

## Problem

After P172, the source-execution probe advances to:

```text
target_shape_blocker_symbol=JsonFragBox._decode_escapes/1
target_shape_blocker_reason=generic_string_unsupported_void_sentinel_const
```

`JsonFragBox._decode_escapes/1` has a string-handle signature but begins with a
null guard:

```hako
if s == null { return null }
```

The runtime ABI for a string handle and a null sentinel is still an `i64`
handle value, with `0` as null. MIR already has the dedicated
`generic_string_or_void_sentinel_body` target shape and ny-llvmc already
validates its `string_handle_or_null` return shape. The classifier only tried
that shape for `void` / `unknown` signatures, so a typed string helper fell
through to `generic_pure_string_body` and rejected the null return sentinel.

## Decision

Allow the existing `generic_string_or_void_sentinel_body` classifier path to run
for string-handle signatures (`str` / `StringBox`) in addition to `void` and
`unknown`, when return-profile evidence proves the body returns only strings or
null/void sentinels.

This card reuses the existing shape/proof:

```text
target_shape=generic_string_or_void_sentinel_body
proof=typed_global_call_generic_string_or_void_sentinel
return_shape=string_handle_or_null
```

This does not allow arbitrary void returns in pure string bodies, does not
change method-call acceptance, and does not add a backend fallback.

## Acceptance

```bash
cargo test -q refresh_module_global_call_routes_accepts_string_typed_null_sentinel_body --lib
cargo test -q global_call_routes --lib
cargo fmt --check
cargo build -q --release --bin hakorune
bash tools/build_hako_llvmc_ffi.sh
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p173_stage1_cli_env.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

The final `--emit-exe` command is an advance-to-next-blocker probe.

## Result

`JsonFragBox._decode_escapes/1` now routes as a direct
`generic_string_or_void_sentinel_body` target when its typed string body returns
either a string handle or null sentinel.

The probe advances to:

```text
target_shape_blocker_symbol=BuilderUnsupportedTailBox._log_fail_reason/2
target_shape_blocker_reason=generic_string_return_abi_not_handle_compatible
```

Treat the unsupported tail logging helper as the next card. Do not fold it into
string-typed null sentinel acceptance.
