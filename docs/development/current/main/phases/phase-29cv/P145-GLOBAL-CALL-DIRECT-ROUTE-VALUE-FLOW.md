---
Status: Active
Decision: accepted
Date: 2026-05-02
Scope: phase-29cv P145, direct route value evidence through stale void metadata
Related:
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - docs/development/current/main/phases/phase-29cv/P144-GLOBAL-CALL-STRING-SCAN-I64-BODY.md
  - src/mir/global_call_route_plan.rs
---

# P145: Global Call Direct Route Value Flow

## Problem

After P144, the next route was:

```text
BuildBox._parse_program_json_from_scan_src/1 -> BuildBox._resolve_parse_src/1
target_shape_reason=generic_string_unsupported_void_sentinel_const
```

`BuildBox._resolve_parse_src/1` calls the already-direct
`BodyExtractionBox.extract_main_body/1`, but stale MIR value metadata can still
mark that call result and its exact copies as `void`. The generic string scan
was therefore reporting the stale metadata instead of the route-owned direct
child evidence.

## Decision

Let direct route facts and exact flow instructions override stale `void`
metadata in the generic string value scan:

- a direct string/string-or-null child route can prove the call result is a
  string-class value for the string scan
- a direct i64 child route can prove the call result is i64
- exact `copy` and all-string/all-i64 PHI flow may carry that proven class
  through stale `void` destinations
- this override is limited to `void -> proven class`; conflicting non-void
  classes remain unchanged

This is analysis evidence only. It does not make declarations lowerable and
does not infer function parameters as string by default.

## Expected Evidence

`stage1_cli_env.hako` should move from stale void-sentinel blocker evidence to
the next real blocker:

```text
BuildBox._parse_program_json_from_scan_src/1 -> BuildBox._resolve_parse_src/1
target_shape_reason=generic_string_return_not_string
```

`BuildBox._resolve_parse_src/1 -> BodyExtractionBox.extract_main_body/1` must
remain direct:

```text
target_shape=generic_string_or_void_sentinel_body
proof=typed_global_call_generic_string_or_void_sentinel
```

## Acceptance

- `cargo fmt --check` succeeds.
- `cargo test -q refresh_module_global_call_routes_uses_direct_child_route_over_void_metadata` succeeds.
- `cargo test -q global_call_routes` succeeds.
- `cargo build --release --bin hakorune` succeeds.
- `target/release/hakorune --emit-mir-json ... stage1_cli_env.hako` exposes
  `BuildBox._resolve_parse_src/1` as `generic_string_return_not_string`.
- `git diff --check` succeeds.
- `tools/checks/current_state_pointer_guard.sh` succeeds.
