# Phase 216x: sum seed metadata helper consumer fold

Status: Landed

Purpose
- land the next `generic placement / effect` proving slice on the C shim boundary path
- make the current sum local seed metadata helper read folded `placement_effect_routes` first

Scope
- extend `placement_effect_routes` with the minimal generic `source_value` field required by current sum routes
- teach `hako_llvmc_ffi_sum_local_seed_metadata_helpers.inc` to read:
  - folded `thin_entry` rows
  - folded `sum_placement` local-aggregate rows
  - folded `agg_local_scalarization` sum-layout rows
- keep `thin_entry_selections`, `sum_placement_facts`, `sum_placement_selections`, and `sum_placement_layouts` as compatibility fallback
- pin the folded route path with focused metadata-bearing sum fixtures

Non-goals
- no sum lowering shape rewrite
- no deletion of legacy sum metadata lanes
- no broader generic backend-wide consumer switch
- no widening beyond the current sum local seed metadata helper

Acceptance
- current sum local seed metadata helper succeeds from `placement_effect_routes` first
- legacy sum metadata still keeps existing boundary fixtures green
- `git diff --check`

