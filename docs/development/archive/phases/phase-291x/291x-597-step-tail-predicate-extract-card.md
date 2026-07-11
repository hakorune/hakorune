---
Status: Landed
Date: 2026-04-28
Scope: extract the duplicated step-tail matcher into the shared scan predicate owner
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/facts/scan_common_predicates.rs
  - src/mir/builder/control_flow/facts/loop_bundle_resolver_v0_shape_routes.rs
  - src/mir/builder/control_flow/facts/loop_collect_using_entries_v0_shape_routes.rs
---

# 291x-597: Step-Tail Predicate Extract

## Goal

Move the duplicated `extract_step_var_from_tail` matcher into the shared
`scan_common_predicates` owner so the scan-family shape routes consume one SSOT
for `loop_var = next_var` tail matching.

This is BoxShape-only cleanup. It does not change route acceptance.

## Boundaries

- Keep only the duplicated low-level AST matcher in the shared owner.
- Leave local-declaration checks in the shape-route owners.
- Do not change reject reasons or route-specific control-flow policy.

## Result

- Added shared `extract_step_var_from_tail()` to
  `facts/scan_common_predicates.rs`.
- Removed the duplicated local copies from:
  - `loop_bundle_resolver_v0_shape_routes.rs`
  - `loop_collect_using_entries_v0_shape_routes.rs`
- Updated both shape-route owners to import the shared predicate.

## Verification

```bash
cargo fmt -- --check
cargo check --release --bin hakorune -q
git diff --check
tools/checks/dev_gate.sh quick
```
