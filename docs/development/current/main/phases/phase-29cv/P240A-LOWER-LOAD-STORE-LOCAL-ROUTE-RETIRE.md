---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P240a, LowerLoadStoreLocal active route retirement
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P239A-LOWER-LOOP-SIMPLE-DIRECT-JSON.md
  - lang/src/mir/builder/internal/lower_load_store_local_box.hako
---

# P240a: Lower Load/Store Local Route Retire

## Problem

P239a advances the source-exe probe to:

```text
target_shape_blocker_symbol=LowerLoadStoreLocalBox.try_lower/1
target_shape_blocker_reason=generic_string_unsupported_void_sentinel_const
backend_reason=missing_multi_function_emitter
```

The body already emits MIR JSON directly, but it is a broad legacy
Local/Assignment fallback that introduces a store/load-shaped MIR path into the
active fallback chain. It also carried nullable owner-local read helpers, making
the route classifier chase void-sentinel helper details that are not part of the
current source-exe keeper path.

## Decision

Do not teach Stage0 a new body shape for this legacy fallback.

Keep the file available for explicit future work, but remove
`LowerLoadStoreLocalBox.try_lower/1` from the active fallback chains:

```text
BuilderFallbackAuthorityBox._try_boxed_lowerers/1
BuilderRunnerMinBox.run/1
```

While touching the owner path, make its no-match contract string-only if it is
re-enabled later:

```text
match -> MIR JSON string
no-match -> ""
```

The owner path now reads the few required Program(JSON) facts through existing
`JsonFragBox` helpers and emits final MIR JSON directly from the owner.

## Non-Goals

- no new `GlobalCallTargetShape`
- no generic MapBox or object semantics
- no C body-specific emitter
- no change to the recognized load/store local pattern
- no fallback route
- no store/load semantics added to Stage0

## Acceptance

The source-exe probe should move past
`LowerLoadStoreLocalBox.try_lower/1`; a later blocker may remain. The active
fallback chain should no longer call this lowerer.

```bash
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p240a_lower_load_store_local_route_retire.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

Observed probe:

```text
target_shape_blocker_symbol=MirBuilderBox._emit_internal_program_json/3
target_shape_blocker_reason=generic_string_return_abi_not_handle_compatible
backend_reason=missing_multi_function_emitter
```

This confirms the active frontier moved past
`LowerLoadStoreLocalBox.try_lower/1`.

The remaining `LoopOptsBox.new_map/0` carrier inventory is still narrowed to:

```text
LowerLoopMultiCarrierBox.try_lower/2
```
