---
Status: Active
Decision: accepted
Date: 2026-05-02
Scope: phase-29cv P139, transitive returned-child blocker evidence
Related:
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - docs/development/current/main/phases/phase-29cv/P137-GLOBAL-CALL-VOID-SENTINEL-RETURN-BLOCKER.md
  - docs/development/current/main/phases/phase-29cv/P138-GLOBAL-CALL-OBJECT-RETURN-ABI-BLOCKER.md
  - src/mir/global_call_route_plan.rs
  - src/runner/mir_json_emit/tests/global_call_routes.rs
---

# P139: Global Call Transitive Return Blocker

## Problem

P138 exposes the object-return leaf blocker:

```text
BuildBox._new_prepare_scan_src_result/1
target_shape_reason=generic_string_return_object_abi_not_handle_compatible
```

But the top source-execution trace can still stop at an intermediate returned
global wrapper such as:

```text
Stage1SourceProgramAuthorityBox._emit_program_json_from_source_checked/2
target_shape_blocker_reason=generic_string_global_target_shape_unknown
```

That is correct but less actionable than the deeper object boundary.

## Decision

For void/string-or-null sentinel return-profile diagnostics, propagate an
unknown returned child global's existing blocker evidence when it has one.

This is still diagnostic-only. It does not make the parent route lowerable and
does not add object lowering.

## Rules

Allowed:

- forward `target_shape_blocker_symbol` / `target_shape_blocker_reason` from a
  returned unknown child global
- keep `target_shape=unknown` and `tier=Unsupported`
- mirror the propagated blocker through MIR JSON and LoweringPlan

Forbidden:

- treating propagated blocker evidence as backend permission
- accepting intermediate wrappers as new target shapes
- adding MapBox/object lowering in this card

## Expected Evidence

After this card, the source-execution unsupported-shape trace should move closer
to the leaf object boundary:

```text
target_shape_blocker_symbol=BuildBox._new_prepare_scan_src_result/1
target_shape_blocker_reason=generic_string_return_object_abi_not_handle_compatible
```

## Acceptance

- `cargo fmt --check` succeeds.
- `cargo test -q global_call_routes` succeeds.
- `cargo test -q build_mir_json_root_emits_transitive_void_sentinel_return_child_blocker` succeeds.
- `target/release/hakorune --emit-mir-json ... stage1_cli_env.hako` reports the
  transitive object-return blocker on the source-execution route.
