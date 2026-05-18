# Hako Alloc Segment Map Modeled Consume Ledger Diagnostics SSOT

Status: accepted
Decision: accepted
Date: 2026-05-18

## Purpose

MIMAP-158A adds stable diagnostics around the MIMAP-157A accepted readiness ->
modeled consume ledger boundary.

The diagnostic vocabulary is intentionally scalar and local to the modeled
lane:

```text
0 ok
1 blocked
2 duplicate
3 stale
```

## Diagnostic Mapping

```text
blocked:
  accepted readiness reached modeled consume,
  but consume rejected an unsupported substrate requirement.

duplicate:
  modeled consume succeeded,
  but ledger rejected the token as already live.

stale:
  readiness input was rejected because the explicit-ID lookup saw stale
  generation.
```

The row must preserve the MIMAP-157A accepted path and add only diagnostic
counters / report fields.

## Owner

```text
lang/src/hako_alloc/memory/segment_map_accepted_readiness_modeled_consume_ledger_box.hako
```

The owner remains a composition owner. It does not own a real segment map,
arena, bitmap, OSVM, or provider.

## Validation

MIMAP-158A stays on L2 validation:

```text
bash tools/checks/k2_wide_hako_alloc_segment_map_accepted_readiness_modeled_consume_ledger_guard.sh
```

L3 EXE evidence remains deferred to the future consume-ledger closeout pack.

## Stop Lines

- No raw pointer residence or pointer-derived lookup.
- No real segment-map mutation.
- No real segment allocation/free.
- No arena backing allocation.
- No atomic bitmap execution.
- No OSVM/page-source execution.
- No TLS, worker-local, worker scheduling, or source-level concurrency.
- No provider activation, host allocator replacement, hooks, or
  `#[global_allocator]`.
- No cross-function `Result` direct ABI or runtime sum materialization.
- No backend `.inc` matcher by app or owner name.

## Next

The next selected row is:

```text
MIMAP-159A segment-map modeled consume ledger closeout pack
```

MIMAP-159A should close the MIMAP-157A/158A pack and carry representative L3
EXE evidence.
