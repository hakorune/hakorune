---
Status: Landed
Date: 2026-04-25
Scope: Add the StringIndexOf CoreMethod route carrier and metadata consumers without pruning legacy indexOf fallback rows.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-212-string-indexof-coremethod-carrier-design-card.md
  - src/mir/generic_method_route_plan.rs
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_route_policy.inc
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_need_policy.inc
  - apps/tests/mir_shape_guard/string_indexof_ascii_min_v1.mir.json
---

# 291x-217 String IndexOf Carrier Consumer Card

## Goal

Add the missing metadata path for the already-manifested `StringIndexOf`
CoreMethod operation:

```text
StringBox.indexOf(needle)
  -> generic_method.indexOf
  -> core_method.op = StringIndexOf
  -> route_kind = string_indexof
  -> nyash.string.indexOf_hh
```

This is the consumer/carrier card only. The legacy `indexOf` method-surface
fallback remains pinned for a follow-up prune card.

## Boundary

- Support only the current one-argument String indexOf shape.
- Reuse existing `StringIndexOf`; do not add new CoreMethod vocabulary.
- Do not touch ArrayBox.indexOf.
- Do not add two-argument `indexOf(search, start)` metadata in this card.
- Do not prune `.inc` rows in this card.

## Acceptance

```bash
cargo test -q records_direct_indexof_core_method_route
cargo test -q records_runtime_data_indexof_from_string_origin
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_string_indexof_min.sh
bash tools/smokes/v2/profiles/integration/archive/phase29ck_boundary/string/phase29ck_boundary_pure_string_indexof_min.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

Landed.

- Added the `generic_method.indexOf` route carrier for one-argument
  `StringBox.indexOf(needle)` and `RuntimeDataBox.indexOf(needle)` when receiver
  origin is `StringBox`.
- Routed the carrier as `core_method.op = StringIndexOf`,
  `route_kind = string_indexof`, `return_shape = scalar_i64`,
  `value_demand = scalar_i64`, and `publication_policy = no_publication`.
- Added `.inc` metadata consumers for route and need selection. The legacy
  `indexOf` method-surface fallback rows intentionally remain pinned.
- `core_method_contract_inc_no_growth_guard.sh` remains at
  `classifiers=12 rows=12`.

Validated with:

```bash
cargo test -q records_direct_indexof_core_method_route
cargo test -q records_runtime_data_indexof_from_string_origin
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_string_indexof_min.sh
bash tools/smokes/v2/profiles/integration/archive/phase29ck_boundary/string/phase29ck_boundary_pure_string_indexof_min.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Residual risk:

- `phase29ck_boundary_pure_indexof_line_min.sh` still fails with the existing
  `unsupported pure shape for current backend recipe` exact-seed boundary. That
  fixture is outside this carrier/consumer card and should be handled as a
  follow-up exact-route metadata card before any `indexOf` prune attempt.
