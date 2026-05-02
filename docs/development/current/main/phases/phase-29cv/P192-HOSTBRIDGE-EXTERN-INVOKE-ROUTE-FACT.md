---
Status: Active
Decision: accepted
Date: 2026-05-02
Scope: phase-29cv P192, hostbridge extern-invoke route fact and AOT fail-fast boundary
Related:
  - docs/development/current/main/phases/phase-29cv/P191-UNKNOWN-RETURN-STRING-OR-VOID-WRAPPER.md
  - lang/src/mir/builder/internal/delegate_provider_box.hako
  - src/mir/extern_call_route_plan.rs
  - src/mir/global_call_route_plan/generic_string_body.rs
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_shell.inc
---

# P192: HostBridge Extern Invoke Route Fact

## Problem

P191 moved the source-execution probe to the delegate provider trampoline:

```text
target_shape_blocker_symbol=hostbridge.extern_invoke/3
target_shape_blocker_reason=generic_string_global_target_missing
```

The source body is the compatibility delegate route:

```hako
return hostbridge.extern_invoke("env.mirbuilder", "emit", args)
```

This is not a same-module global function and must not become a generic string
body rule by itself.

## Decision

Treat `hostbridge.extern_invoke` as an extern route fact, even when legacy MIR
still records the callee as a `Global("hostbridge.extern_invoke/3")`.

```text
route_id=extern.hostbridge.extern_invoke
core_op=HostBridgeExternInvoke
tier=ColdRuntime
proof=extern_registry
return_shape=string_handle_or_null
```

The generic string/global body classifier may consume this route fact only as a
known extern string-or-void result. It must not inspect provider semantics or
turn `hostbridge.extern_invoke` into a same-module DirectAbi target.

## AOT Boundary

`env.mirbuilder.emit` delegate is compatibility-only in the current mainline.
The stage1 bridge runtime defaults keep it disabled:

```text
HAKO_SELFHOST_NO_DELEGATE=1
HAKO_MIR_BUILDER_DELEGATE=0
```

For ny-llvmc source-execution, the C shim must read the lowering plan and emit a
fail-fast trap for `extern.hostbridge.extern_invoke`. Silent null fallback is
forbidden, and the C shim must not rediscover the route from raw callee names.

## Forbidden

- adding `hostbridge.extern_invoke` as a same-module global target
- adding by-name delegate provider acceptance inside generic string body
- silently returning null when delegate is enabled
- adding backend-local raw callee classification instead of reading
  `lowering_plan`

## Acceptance

```bash
cargo test -q hostbridge_extern_invoke --lib
cargo test -q global_call_routes --lib
cargo fmt --check
bash tools/build_hako_llvmc_ffi.sh
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p192_hostbridge_extern_invoke_route_probe.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

The `--emit-exe` command remains a next-blocker probe.

## Probe Result

Observed on 2026-05-02:

```text
target_shape_blocker_symbol=BuilderFinalizeChainBox._methodize_if_enabled_checked/1
target_shape_blocker_reason=generic_string_unsupported_void_sentinel_const
```

This confirms that the `hostbridge.extern_invoke/3` route fact is no longer the
active source-execution blocker. The next card should re-check the methodize
wrapper in the newly exposed parent path.
