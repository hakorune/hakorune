---
Status: Investigation
Date: 2026-06-08
Scope: archive for oversized active mimalloc workstream ledgers.
Related:
  - docs/development/current/main/workstreams/mimalloc-current.md
  - docs/development/current/main/investigations/current-docs-length-audit-2026-06-06.md
---

# Mimalloc Current Docs Slim Archive 2026-06-08

This note collects the historical ledger material that was removed from the
active mimalloc workstream to keep restart pointers compact.

## Archived Algorithm Port Coverage

```text
.hako model/policy coverage:
  size_class_policy=modeled
  page_local_free_stack=modeled
  same_thread_local_free=modeled
  object_lifecycle_hot_core=modeled
  page_map/realloc/huge/osvm/remote_free=policy_or_seam_modeled

benchmark-only replacement front execution:
  fixed_slot_native_free_stack=executed
  matched_fixed_slot_size=executed for selected fixtures
  hako_size_class_good_size_slot=executed only when
    --replacement-front-match-hako-size-class is passed
  product_bins=benchmark_native_bins_v0 when
    --replacement-front-native-bins-mode is passed
  product_pages=report_only_plan
  in_place_realloc_within_fixed_slot=executed
  thread_local_arena_remote_free_bridge=executed

not yet bridged:
  size_class_policy_to_product_replacement_pages
  measured DirectArrayI64 source route in product-like execution
  selected next structural owner after HotCore/PageModel measurement
  general page queue / segment / OSVM product allocator front
```

Stable report entry:

```bash
python3 tools/allocator/hako_mimalloc_algorithm_coverage.py
```

The detailed bridge probes, keeper/non-keeper decisions, and refresh report
paths remain archived in the workstream history and phase cards.

## Archived Evidence Anchors

The historical workstream evidence anchors that were previously inline are
still valid, but they are no longer restart-critical:

```text
MIM-116..MIM-123:
  state bucket / record-state residence / hako_check state-explain /
  source-surface responsibility cleanup

MIM-124..MIM-134:
  owner-first perf refreshes, body-timer control, workload matrix,
  direct-exact app perf/asm tooling, observer-light closeout

MIM-135..MIM-146:
  provider package explicit measurement, hakmem and hakozuna LD_PRELOAD
  bridge, generated global allocator smoke, provider export bundle,
  same-machine Hakozuna mixed-ws C mimalloc comparison
```

The provider ABI / shim boundary and replacement-front fidelity evidence are
now owned by the dedicated design docs and phase cards.

## Archived Current Task Ledger

The active workstream now keeps only a compact restart pointer. The long
historic task order, route-order notes, and benchmark-side ledgers are archived
here instead of being duplicated in the restart card.

The current active task ordering is now tracked in:

```text
docs/development/current/main/CURRENT_STATE.toml
docs/development/current/main/workstreams/mimalloc-current.md
```

## Purpose

This archive is for reference only. Do not re-expand it back into the active
workstream unless the restart card becomes sparse again or a new investigation
needs the old ledgers.
