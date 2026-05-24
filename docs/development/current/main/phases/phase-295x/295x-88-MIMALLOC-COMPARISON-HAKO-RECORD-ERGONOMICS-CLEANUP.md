---
Status: Landed
Date: 2026-05-25
Scope: apply accepted record ergonomics to one guarded `.hako` allocator-comparison owner.
Related:
  - docs/development/current/main/design/hako-alloc-wide-report-argument-cleanup-ssot.md
  - lang/src/hako_alloc/memory/allocator_comparison_benchmark_execution_preflight_inventory_box.hako
---

# 295x-88 Hako Record Ergonomics Cleanup

## Blocker

```text
MIMALLOC-COMPARISON-HAKO-RECORD-ERGONOMICS-CLEANUP-295X-001
```

## Decision

Apply the already accepted record ergonomics surface to a guarded
allocator-comparison owner before continuing with the `malloc-large` workload
alignment row.

Accepted surface:

```hako
record ReportFields {
    accepted: i64 = 0
    reason: i64 = 0
}

local fields = ReportFields {}
fields = fields with {
    accepted,
    reason
}
```

This row does not add new source syntax. It uses the existing Stage1 record
field defaults, empty record literals, same-name shorthand, and record-only
`with` updates.

## Owner

```text
lang/src/hako_alloc/memory/allocator_comparison_benchmark_execution_preflight_inventory_box.hako
```

The owner was chosen because it already has an L2 guard and contains an older
fully-expanded `ReportFields` construction site.

## Cleanup

- Add scalar defaults to
  `HakoAllocAllocatorComparisonBenchmarkExecutionPreflightInventoryReportFields`.
- Build owner-local report fields from `ReportFields {}` and a `with` update.
- Use same-name shorthand for parameters whose field names match exactly.
- Keep explicit zero stop-line fields in the `with` update so existing
  closed-seam guards remain readable.
- Realign the legacy MIMAP-436A guard with the current memory layer layout:
  owner membership is checked in `MODULE_INDEX.md`, while `README.md` remains
  the layer entry and style contract.

## Stop Line

This row does not introduce record spread / `...fields`, named arguments,
automatic record-to-box copy, ordinary-box `with`, runtime record materialization,
benchmark execution, provider activation, process allocator replacement, hooks,
backend matchers, `#[global_allocator]`, worker/TLS, atomics, remote-free stress,
or winner claims.

## Validation

```bash
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_benchmark_execution_preflight_inventory_guard.sh --level L2
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
