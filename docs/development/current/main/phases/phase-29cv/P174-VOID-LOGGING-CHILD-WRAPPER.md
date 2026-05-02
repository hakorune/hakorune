---
Status: Accepted
Decision: accepted
Date: 2026-05-02
Scope: phase-29cv P174, void logging child wrapper
Related:
  - docs/development/current/main/phases/phase-29cv/P173-STRING-TYPED-NULL-SENTINEL-BODY.md
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - src/mir/global_call_route_plan/generic_string_body.rs
---

# P174: Void Logging Child Wrapper

## Problem

After P173, the source-execution probe advances to:

```text
target_shape_blocker_symbol=BuilderUnsupportedTailBox._log_fail_reason/2
target_shape_blocker_reason=generic_string_return_abi_not_handle_compatible
```

`BuilderUnsupportedTailBox._log_fail_reason/2` is a void/null sentinel logging
wrapper. It does not call `print` directly; it branches to the existing
`BuilderFinalizeChainBox.log_fail/1` helper and returns `null` after each
logging path.

The existing `generic_string_void_logging_body` shape already lowers the child
logging helper, but its evidence required a direct `print` call in the same
body. That made logging wrappers look like generic string failures even though
the logging authority was already direct.

## Decision

Allow `generic_string_void_logging_body` evidence to be satisfied by either:

- a direct supported backend `print` call
- a same-module child target already classified as
  `generic_string_void_logging_body`

The body must still build a string surface and return only void/null sentinel
values. This does not accept general void helpers, non-logging children, or
string-or-void return unions.

## Acceptance

```bash
cargo test -q refresh_module_global_call_routes_accepts_void_logging_child_wrapper --lib
cargo test -q global_call_routes --lib
cargo fmt --check
cargo build -q --release --bin hakorune
bash tools/build_hako_llvmc_ffi.sh
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p174_stage1_cli_env.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

The final `--emit-exe` command is an advance-to-next-blocker probe.

## Result

`BuilderUnsupportedTailBox._log_fail_reason/2` now routes as a direct
`generic_string_void_logging_body` target by using the already-direct
`BuilderFinalizeChainBox.log_fail/1` child call as logging evidence.

The probe advances to:

```text
target_shape_blocker_symbol=JsonFragNormalizerBox._normalize_instructions_array/1
target_shape_blocker_reason=generic_string_unsupported_method_call
```

Treat the normalizer method surface as the next card. Do not fold it into void
logging child-wrapper evidence.
