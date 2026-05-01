---
Status: Active
Decision: accepted
Date: 2026-05-02
Scope: phase-29cv P144, string scanner helpers as generic i64 bodies
Related:
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - docs/development/current/main/phases/phase-29cv/P143-GLOBAL-CALL-STRING-OR-VOID-RETURN-UNION.md
  - src/mir/global_call_route_plan.rs
---

# P144: Global Call String Scan I64 Body

## Problem

P143 moved the active route to:

```text
target_shape_blocker_symbol=BodyExtractionBox._find_pattern/3
target_shape_blocker_reason=generic_string_unsupported_method_call
```

That blocker is a string scanner that returns an i64 index. It is not a string
body, but it uses string receiver methods such as `length()` and
`substring(i64, i64)` while computing scalar bounds. The existing
`generic_i64_body` scan was too narrow: it seeded unknown parameters as string
and rejected `substring` while scalar argument facts were still pending.

## Decision

Keep this in the existing `generic_i64_body` shape and make its value-class
fixpoint precise enough for scanner helpers:

- seed known value classes from MIR signatures and `metadata.value_types`
- leave unknown parameters unknown until an instruction proves their class
- infer string receivers for accepted string `length` / `substring` calls
- let `length()` produce i64
- let `substring(i64, i64)` produce string only after both args resolve i64
- defer pending substring scalar arguments during the fixpoint instead of
  rejecting them as unsupported methods
- iterate target classification in sorted function-name order for stable
  same-module fixpoint results

This is still a narrow shape proof. Backend emission remains authorized only by
the LoweringPlan route with `target_shape=generic_i64_body`.

## Rules

Allowed:

- copy/PHI propagation from known typed evidence
- relational comparisons to prove scalar i64 operands
- `Eq` / `Ne` comparisons to prove string operands when the opposite side is
  already string
- `RuntimeDataBox.length()` / `StringBox.length()` on known string receivers
- `RuntimeDataBox.substring(i64, i64)` / `StringBox.substring(i64, i64)` on
  known string receivers with scalar bounds

Forbidden:

- treating every unknown parameter as string by default
- accepting arbitrary string methods through the i64 body shape
- accepting substring bounds that resolve to non-i64 values
- externalizing same-module calls through declaration-only fallbacks

## Expected Evidence

`stage1_cli_env.hako` should expose these direct routes:

```text
BodyExtractionBox.find_main_position/1 -> BodyExtractionBox._find_pattern/3
target_shape=generic_i64_body
proof=typed_global_call_generic_i64
return_shape=ScalarI64

BodyExtractionBox.extract_main_body/1 -> BodyExtractionBox.find_char_skipping_strings/3
target_shape=generic_i64_body
proof=typed_global_call_generic_i64
return_shape=ScalarI64
```

The string sentinel body must remain direct:

```text
BodyExtractionBox.extract_main_body/1 -> BodyExtractionBox.extract_balanced_body/2
target_shape=generic_string_or_void_sentinel_body
proof=typed_global_call_generic_string_or_void_sentinel
return_shape=string_handle_or_null
```

The next visible blocker advances above the scanner helpers to:

```text
BuildBox._parse_program_json_from_scan_src/1 -> BuildBox._resolve_parse_src/1
target_shape_reason=generic_string_unsupported_void_sentinel_const
```

## Acceptance

- `cargo fmt --check` succeeds.
- `cargo test -q refresh_module_global_call_routes_marks_string_scan_generic_i64_body` succeeds.
- `cargo test -q global_call_routes` succeeds.
- `cargo build --release --bin hakorune` succeeds.
- `target/release/hakorune --emit-mir-json ... stage1_cli_env.hako` exposes
  the scanner helpers as `generic_i64_body` and keeps
  `extract_balanced_body/2` as `generic_string_or_void_sentinel_body`.
- `git diff --check` succeeds.
- `tools/checks/current_state_pointer_guard.sh` succeeds.
