---
Status: Accepted
Decision: accepted
Date: 2026-05-02
Scope: phase-29cv P147, global-call route return-profile acceptance
Related:
  - docs/development/current/main/phases/phase-29cv/P145-GLOBAL-CALL-DIRECT-ROUTE-VALUE-FLOW.md
  - src/mir/global_call_route_plan.rs
  - src/mir/global_call_route_plan/generic_string_body.rs
  - src/mir/global_call_route_plan/string_return_profile.rs
  - src/mir/global_call_route_plan/tests/void_sentinel.rs
---

# P147: Global-Call Unknown-Return Void-Sentinel Body

## Problem

`BodyExtractionBox.extract_balanced_body/2` is emitted with an unknown (`?`)
signature return type, but its return profile is structurally a
string-or-void-sentinel body: early failure exits return the void/null sentinel,
while the success path returns a substring string handle.

Before P147, the route classifier only entered the
`generic_string_or_void_sentinel_body` path when the signature return type was
`void`. That made the body fall through to the generic pure string scanner,
where the sentinel returns were rejected as
`generic_string_unsupported_void_sentinel_const`. The rejection propagated to
`BodyExtractionBox.extract_main_body/1` and then to
`BuildBox._resolve_parse_src/1`, masking the P145 evidence.

## Decision

Allow the existing void-sentinel return-profile scanner to run for both
`void` and `?` return signatures.

This adds exactly one acceptance shape:

- `return_type == ?`
- return profile proves at least one string return and one void/null sentinel
  return
- body scanner accepts only the existing generic string/void-sentinel
  vocabulary
- no propagated target blocker is present

This is not a by-name exception for `BodyExtractionBox`. The target name is not
consulted; the existing return-profile and body scanners remain the SSOT.

## Evidence

After rebuilding `target/release/hakorune` and emitting
`lang/src/runner/stage1_cli_env.hako`:

```text
BodyExtractionBox.extract_main_body/1 -> BodyExtractionBox.extract_balanced_body/2
  tier=DirectAbi
  target_shape=generic_string_or_void_sentinel_body
  proof=typed_global_call_generic_string_or_void_sentinel

BuildBox._resolve_parse_src/1 -> BodyExtractionBox.extract_main_body/1
  tier=DirectAbi
  target_shape=generic_string_or_void_sentinel_body
  proof=typed_global_call_generic_string_or_void_sentinel

BuildBox._parse_program_json_from_scan_src/1 -> BuildBox._resolve_parse_src/1
  tier=Unsupported
  target_shape_reason=generic_string_return_not_string
```

The last line restores the P145 intended blocker boundary: the remaining
unsupported edge is the actual `_resolve_parse_src` return-shape mismatch, not
the nested body-extraction sentinel route.

## Acceptance

```bash
cargo test -q refresh_module_global_call_routes_accepts_unknown_return_void_sentinel_body
cargo test -q refresh_module_global_call_routes_uses_direct_child_route_over_void_metadata
cargo test -q global_call_routes
cargo fmt --check
cargo build --release --bin hakorune
target/release/hakorune --emit-mir-json /tmp/hakorune_p147_stage1_cli_env_after.mir.json lang/src/runner/stage1_cli_env.hako
```
