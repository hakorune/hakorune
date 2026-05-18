# Hako Alloc Segment Arena Backing Readiness Inventory SSOT

Status: active
Date: 2026-05-19
Decision: accepted

## Purpose

Inventory arena backing readiness after lifecycle-keyed release apply/recycle
continuation closeout.

MIMAP-236A consumes the MIMAP-233A continuation diagnostics report and records
scalar facts that must be known before any real arena allocation, raw pointer
residence, real segment-map mutation, or atomic bitmap execution opens.

## Owner

```text
lang/src/hako_alloc/memory/segment_arena_backing_readiness_inventory_box.hako
```

The owner may:

- observe that lifecycle-keyed release apply/recycle continuation diagnostics
  were observed;
- publish scalar arena backing facts such as segment id, arena id, slice count,
  committed slice count, free slice count, required alignment, and page size;
- reject shapes that require real arena allocation, raw pointer residence, real
  segment-map mutation, atomic bitmap execution, OSVM, or provider activation.

The owner must not:

- allocate arena backing;
- use raw pointer residence;
- mutate a real segment-map;
- execute real segment allocation/free;
- execute atomic bitmap claims;
- call page-source or OSVM seams;
- infer anything from owner names or backend matchers.

## Reason Vocabulary

| Reason | Meaning |
| --- | --- |
| `0` | scalar arena backing readiness accepted |
| `1` | lifecycle-keyed continuation diagnostics missing or not applicable |
| `2` | invalid segment, arena, slice, alignment, or page-size shape |
| `3` | real arena backing allocation would be required |
| `4` | raw pointer residence would be required |
| `5` | real segment-map mutation would be required |
| `6` | atomic bitmap execution would be required |
| `7` | OSVM/page-source execution would be required |
| `8` | provider activation would be required |

## Validation

MIMAP-236A uses daily L2 validation:

```bash
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_readiness_inventory_guard.sh --level L2
```

The guard must:

- prove accepted scalar arena backing readiness after MIMAP-233A diagnostics;
- prove missing-continuation and invalid-shape rejects;
- prove arena allocation, raw pointer, real segment-map, atomic bitmap, OSVM,
  and provider requirements are rejected;
- prove inactive flags remain zero.

## Stop Lines

- No real arena backing allocation.
- No raw pointer residence or pointer-derived lookup.
- No real segment-map mutation.
- No real segment allocation/free execution.
- No atomic bitmap execution.
- No OSVM/page-source execution.
- No TLS, worker-local, worker scheduling, or source-level concurrency.
- No provider activation, host allocator replacement, hooks, or
  `#[global_allocator]`.
- No cross-function `Result` direct ABI or runtime sum materialization.
- No backend `.inc` matcher by app or owner name.
