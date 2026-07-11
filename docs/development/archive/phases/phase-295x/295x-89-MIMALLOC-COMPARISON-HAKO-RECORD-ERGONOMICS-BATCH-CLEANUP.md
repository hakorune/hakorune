---
Status: Landed
Date: 2026-05-25
Scope: batch-apply accepted record ergonomics to allocator-comparison `.hako` owners.
Related:
  - docs/development/current/main/design/hako-alloc-wide-report-argument-cleanup-ssot.md
  - docs/development/current/main/phases/phase-295x/295x-88-MIMALLOC-COMPARISON-HAKO-RECORD-ERGONOMICS-CLEANUP.md
---

# 295x-89 Hako Record Ergonomics Batch Cleanup

## Blocker

```text
MIMALLOC-COMPARISON-HAKO-RECORD-ERGONOMICS-BATCH-CLEANUP-295X-001
```

## Decision

Extend the 295x-88 record ergonomics pattern across allocator-comparison owners
that still had both:

- `ReportFields` records without scalar defaults;
- direct `local fields = ReportFields { ... }` construction.

This row keeps the language surface unchanged. It only uses the already accepted
record defaults, empty record literals, same-name shorthand, and record-only
`with` updates.

## Owners

The batch converted these owner-local `ReportFields` construction sites:

```text
lang/src/hako_alloc/memory/allocator_comparison_baseline_inventory_box.hako
lang/src/hako_alloc/memory/allocator_comparison_benchmark_execution_preflight_diagnostic_box.hako
lang/src/hako_alloc/memory/allocator_comparison_c_mimalloc_execution_diagnostic_box.hako
lang/src/hako_alloc/memory/allocator_comparison_c_mimalloc_execution_inventory_box.hako
lang/src/hako_alloc/memory/allocator_comparison_c_mimalloc_explicit_runner_evidence_diagnostic_box.hako
lang/src/hako_alloc/memory/allocator_comparison_c_mimalloc_explicit_runner_execution_pilot_box.hako
lang/src/hako_alloc/memory/allocator_comparison_c_mimalloc_result_ledger_box.hako
lang/src/hako_alloc/memory/allocator_comparison_controlled_benchmark_execution_inventory_box.hako
lang/src/hako_alloc/memory/allocator_comparison_measurement_plan_inventory_box.hako
lang/src/hako_alloc/memory/allocator_comparison_representative_benchmark_execution_pilot_box.hako
lang/src/hako_alloc/memory/allocator_comparison_workload_matrix_inventory_box.hako
```

## Cleanup

- Add `= 0` scalar defaults to each selected `ReportFields` record.
- Replace direct full record literals with:

```hako
local fields = ReportFields {}
fields = fields with {
    accepted,
    reason,
    ...
}
```

- Use same-name shorthand only when the local variable and field name are
  identical.
- Keep explicit assignments for derived values, owner counters, and closed
  stop-line evidence.
- Realign selected legacy guards with the current memory layer layout:
  owner membership is checked in `MODULE_INDEX.md`, while `README.md` remains
  the layer entry and style contract.

## Result

The selected allocator-comparison owner group no longer has a no-default
`ReportFields` plus direct literal construction pattern. Remaining cleanup work
should focus on true data-shape decomposition or guard/test wrapper generation,
not on adding wider record-copy sugar.

## Stop Line

This row does not add record spread / `...fields`, named arguments, automatic
record-to-box copy, ordinary-box `with`, runtime record materialization,
benchmark execution, provider activation, process allocator replacement, hooks,
backend matchers, `#[global_allocator]`, worker/TLS, atomics, remote-free stress,
or winner claims.

## Validation

```bash
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_baseline_inventory_guard.sh --level L2
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_benchmark_execution_preflight_diagnostics_guard.sh --level L2
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_execution_diagnostics_guard.sh --level L2
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_execution_inventory_guard.sh --level L2
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_explicit_runner_evidence_diagnostics_guard.sh --level L2
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_explicit_runner_execution_pilot_guard.sh --level L2
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_ledger_guard.sh --level L2
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_controlled_benchmark_execution_inventory_guard.sh --level L2
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_measurement_plan_inventory_guard.sh --level L2
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_representative_benchmark_execution_pilot_guard.sh --level L2
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_workload_matrix_inventory_guard.sh --level L2
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_benchmark_execution_preflight_inventory_guard.sh --level L2
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
