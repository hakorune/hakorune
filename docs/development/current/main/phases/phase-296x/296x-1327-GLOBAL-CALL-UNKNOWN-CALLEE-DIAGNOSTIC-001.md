# 296x-1327 Global Call Unknown Callee Diagnostic

Status: closed
Date: 2026-06-20

## Purpose

Improve diagnostics for `unknown_global_callee` without changing route
selection or backend lowering behavior.

## Problem

`unknown_global_callee` was a bare category string. It conflated multiple
debugging paths:

```text
callee missing because import/module root wiring did not bring it into MIR
callee missing because the source function/box method is actually undefined
```

The route already carried `callee_name`, but the reason itself did not provide
a diagnosis-oriented context or next check.

## Change

Keep `reason` stable for existing consumers, and add diagnostic fields next to
it:

```text
reason_detail
reason_hint
```

For `unknown_global_callee`, the detail includes the callee name and the hint
points to the imported static-box/module-root and import-bundle merge checks.

This deliberately does not make `GlobalCallRoute` read `hako.toml` directly.
MIR route metadata stays runner/config agnostic.

## Acceptance

```bash
cargo test -q global_call_route_plan::tests::core::refresh_function_global_call_routes_records_unsupported_global_call
cargo test -q mir_json_emit::tests::global_call_routes::core::build_mir_json_root_emits_global_call_routes_and_unsupported_plan
cargo check -q --lib
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Stop Line

```text
route selection changed=0
backend lowering changed=0
reason string renamed=0
hako.toml read from MIR metadata=0
```

## Next

Continue:

```text
CREAT-SUBSET-PILOT-SELECTION-001
```
