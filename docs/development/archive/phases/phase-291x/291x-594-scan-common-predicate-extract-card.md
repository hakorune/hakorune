---
Status: Landed
Date: 2026-04-28
Scope: extract duplicated scan-family AST predicates into a shared facts owner module
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/facts/mod.rs
  - src/mir/builder/control_flow/facts/scan_common_predicates.rs
  - src/mir/builder/control_flow/facts/loop_scan_methods_v0.rs
  - src/mir/builder/control_flow/facts/loop_scan_methods_block_v0_helpers.rs
  - src/mir/builder/control_flow/facts/loop_scan_phi_vars_v0_helpers.rs
  - src/mir/builder/control_flow/facts/loop_bundle_resolver_v0_helpers.rs
  - src/mir/builder/control_flow/facts/loop_collect_using_entries_v0_helpers.rs
  - src/mir/builder/control_flow/plan/loop_scan_v0/facts.rs
---

# 291x-594: Scan Common Predicate Extract

## Goal

Move duplicated scan-family AST predicate helpers into one shared facts owner so
the remaining v0 scan families read the same low-level vocabulary from a single
place.

This is BoxShape-only cleanup. It does not widen accepted syntax or change any
route decision policy.

## Extracted predicates

The new shared owner is:

- `facts/scan_common_predicates.rs`

It now holds the duplicated low-level AST helpers:

- `as_var_name`
- `is_int_lit`
- `is_var_plus_one`
- `is_var_plus_expr`
- `is_loop_cond_var_lt_var`

## Boundaries

- Keep scan-family route-specific helpers local.
- Do not move higher-level recipe or route policy into the shared module.
- Let helper-family modules keep thin wrappers only where that preserves their
  existing outward surface.

## Result

- Centralized shared scan predicates under `facts::scan_common_predicates`.
- Removed repeated AST matching logic from the scan-methods, phi-vars,
  bundle-resolver, collect-using, and plan-side scan facts shelves.
- Left route-specific helpers such as range conditions and local-declaration
  checks in their current owners.

## Verification

```bash
cargo fmt -- --check
cargo check --release --bin hakorune -q
git diff --check
tools/checks/dev_gate.sh quick
```
