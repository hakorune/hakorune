---
Status: Active
Decision: accepted
Date: 2026-05-02
Scope: phase-29cv P141, substring sentinel returns with mixed scalar/string scan state
Related:
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - docs/development/current/main/phases/phase-29cv/P134-GLOBAL-CALL-STRING-SUBSTRING-METHOD.md
  - docs/development/current/main/phases/phase-29cv/P136-GLOBAL-CALL-STRING-VOID-SENTINEL-BODY.md
  - docs/development/current/main/phases/phase-29cv/P140-BUILDBOX-NULL-OPTS-DIRECT-SCAN.md
  - src/mir/global_call_route_plan.rs
  - src/mir/generic_method_route_plan.rs
  - src/runner/mir_json_emit/tests/global_call_routes.rs
---

# P141: Global Call Sentinel Substring Mixed State

## Problem

After P140, the active source-execution blocker advances to:

```text
target_shape_blocker_symbol=BodyExtractionBox.extract_balanced_body/2
target_shape_blocker_reason=generic_string_return_abi_not_handle_compatible
```

That function returns either `null` or `s.substring(...)`. The source parameter
is coerced with `"" + src`, while the brace positions flow as i64 values through
loop PHIs. The old return/body scans initialized every parameter as string, so
the numeric position parameter polluted substring bound classification and the
return-profile scan stopped at the broad ABI reason.

## Decision

Teach the generic string and string-or-void sentinel scans to seed value classes
from existing MIR `value_types` plus declared signature types. Unknown
parameters remain unknown until the body proves a value:

- `"" + x` proves the result is a string surface
- `value_types=i64` proves scalar substring bounds
- relational comparisons against typed i64 values may prove copied scalar
  position values
- copy propagation is bidirectional inside this analysis so scalar/string
  evidence can flow through compiler-generated temporaries
- `RuntimeDataBox.substring(i64, i64)` / `StringBox.substring(i64, i64)` may be
  a string return only when receiver and bounds are already classified

Generic-method routing now also propagates string concat origins so the
LoweringPlan `generic_method.substring` entry remains the backend emission
authority.

## Rules

Allowed:

- use substring return classification as return-profile evidence
- seed scan state from existing MIR value-type metadata
- classify string concat results as string surfaces
- defer method rejection while value classes are still changing
- promote bodies that already pass the generic string body scan to
  `generic_string_or_void_sentinel_body`

Forbidden:

- accepting arbitrary methods as string returns
- treating all unknown parameters as string
- inferring backend emission from raw method names
- adding a backend-local substring matcher

## Expected Evidence

`BodyExtractionBox.extract_balanced_body/2` should move from broad return ABI
incompatibility to the existing string-or-void sentinel body shape.

## Acceptance

- `cargo fmt --check` succeeds.
- `cargo test -q global_call_routes` succeeds.
- `cargo test -q generic_method_route` succeeds.
- `cargo test -q build_mir_json_root_emits_substring_string_or_void_sentinel_direct_route` succeeds.
- `target/release/hakorune --emit-mir-json ... stage1_cli_env.hako` advances
  past the `BodyExtractionBox.extract_balanced_body/2` blocker.
