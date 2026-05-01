---
Status: Active
Decision: accepted
Date: 2026-05-02
Scope: phase-29cv P143, return-profile string-or-void value union
Related:
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - docs/development/current/main/phases/phase-29cv/P142-GLOBAL-CALL-DIRECT-CHILD-BLOCKER-PROPAGATION.md
  - src/mir/global_call_route_plan.rs
---

# P143: Global Call String-Or-Void Return Union

## Problem

After P142, the active route advanced to:

```text
target_shape_blocker_symbol=BodyExtractionBox.extract_main_body/1
target_shape_blocker_reason=generic_string_return_abi_not_handle_compatible
```

The parent returns `null` on guard failures or returns the direct child
`BodyExtractionBox.extract_balanced_body/2`. That child is already classified as
`generic_string_or_void_sentinel_body`, but its MIR call result can still carry
the declared `void` value type. The return-profile scan needed a value class
that represents the route-owned `string_handle_or_null` shape instead of
collapsing `void` metadata and string evidence to `Other`.

## Decision

Add `StringOrVoid` to the return-profile value lattice:

- `generic_string_or_void_sentinel_body` child calls produce `StringOrVoid`
- `String + Void` merges produce `StringOrVoid`
- returning `StringOrVoid` counts as both string and void evidence
- existing MIR `value_types` still seed analysis, but direct child route facts
  may refine the return-profile class

This remains return-profile evidence only. Backend emission is still owned by
the direct child `target_shape` and the LoweringPlan route.

## Rules

Allowed:

- treat direct string-or-void child calls as `StringOrVoid`
- merge string/null return evidence without losing the sentinel shape
- expose the next real blocker after the return-profile ABI boundary

Forbidden:

- treating arbitrary `void` values as string-compatible
- accepting object/MapBox returns through this union
- lowering calls from return-profile evidence alone

## Expected Evidence

`BodyExtractionBox.extract_main_body/1` should move past
`generic_string_return_abi_not_handle_compatible` and expose the next child
blocker.

## Acceptance

- `cargo fmt --check` succeeds.
- `cargo test -q refresh_module_global_call_routes_accepts_void_typed_direct_sentinel_child_return` succeeds.
- `cargo test -q global_call_routes` succeeds.
- `target/release/hakorune --emit-mir-json ... stage1_cli_env.hako` exposes
  `BodyExtractionBox._find_pattern/3` as the next blocker.
