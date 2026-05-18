# Hako Alloc Segment Arena Backing Requirement Matrix SSOT

Status: active
Date: 2026-05-19
Decision: accepted

## Purpose

Inventory the scalar requirement matrix for future segment arena backing after
the MIMAP-236A readiness inventory, MIMAP-237A diagnostics, and MIMAP-238A
closeout.

MIMAP-240A consumes readiness and diagnostics reports, then records whether a
candidate is still scalar/model-only or would require a closed substrate before
real backing can open.

## Owner

```text
lang/src/hako_alloc/memory/segment_arena_backing_requirement_matrix_box.hako
```

The owner may:

- observe MIMAP-236A readiness reports and MIMAP-237A diagnostics reports;
- publish scalar requirement flags for arena backing, raw pointer residence,
  real segment-map use, atomic bitmap use, OSVM/page-source use, worker use,
  provider activation, and backend matcher leaks;
- keep segment id, arena id, slice geometry, required alignment, and page size
  in scalar/model space;
- reject any row that requires a closed substrate.

The owner must not:

- allocate arena backing;
- use raw pointer residence;
- mutate a real segment-map;
- execute real segment allocation/free;
- execute atomic bitmap claims;
- call page-source or OSVM seams;
- schedule workers or expose source-level concurrency;
- activate providers, hooks, host allocator replacement, or backend matchers.

## Reason Vocabulary

| Reason | Meaning |
| --- | --- |
| `0` | scalar requirement matrix accepted |
| `1` | readiness report missing, rejected, or not marked present |
| `2` | diagnostics report missing, rejected, or not marked present |
| `3` | segment, arena, slice, alignment, or page-size geometry invalid |
| `4` | real arena backing allocation would be required |
| `5` | raw pointer residence would be required |
| `6` | real segment-map mutation would be required |
| `7` | atomic bitmap execution would be required |
| `8` | OSVM/page-source execution would be required |
| `9` | worker/TLS/source-level concurrency would be required |
| `10` | provider activation / hook / host allocator replacement would be required |
| `11` | backend matcher or `.inc` owner-name shortcut would be required |

## Validation

MIMAP-240A uses daily L2 validation:

```bash
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_requirement_matrix_guard.sh --level L2
```

The guard must:

- prove accepted scalar requirement matrix publication after readiness and
  diagnostics reports;
- prove readiness, diagnostics, geometry, and each closed substrate requirement
  reject reason;
- prove inactive execution flags remain zero;
- prove the MIR JSON has typed report fields and the expected route surface.

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
