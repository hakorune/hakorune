# 293x-413 DOCS-SLIM-006 M10c Runtime Decl Resolver

Status: landed
Date: 2026-05-15

## Decision

Adopt the phase-card resolver helper in the M10c runtime-decl guard family.

This row removes direct phase-card paths from the M10c runtime-decl guards,
including Python-side card path constants, without moving cards or changing the
runtime-decl contracts.

## TODO

- [x] Convert the M10c runtime-decl return proof row guard card path to
  `guard_require_phase293x_card`.
- [x] Convert the M10c native pointer declare-type guard card path to
  `guard_require_phase293x_card`.
- [x] Convert the hako_mem realloc / call-arg / free card paths to
  `guard_require_phase293x_card`.
- [x] Add a guard proving the converted scripts no longer contain direct
  phase-293x card paths.

## Scope

- Guard path cleanup only.
- M10c runtime-decl guard family only.
- No card movement.

## Stop Lines

- Do not move numbered cards in this row.
- Do not convert unrelated direct-reference guards in this row.
- Do not change runtime-decl manifests, generated defaults, or return-proof
  semantics in this row.
- Do not wire the resolver helper into `dev_gate.sh` or allocator-wide.

## Required Evidence

```text
bash tools/checks/docs_slim_006_m10c_runtime_decl_resolver_guard.sh
bash tools/checks/docs_slim_005_production_closeout_resolver_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```

## Implementation

- Converted `k2_wide_runtime_decl_return_proof_row_guard.sh`.
- Converted `k2_wide_native_ptr_decl_type_guard.sh`.
- Converted `k2_wide_hako_mem_runtime_decl_guard.sh`.
- Added `docs_slim_006_m10c_runtime_decl_resolver_guard.sh`.

## Evidence

```text
bash tools/checks/docs_slim_006_m10c_runtime_decl_resolver_guard.sh
bash tools/checks/docs_slim_005_production_closeout_resolver_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```
