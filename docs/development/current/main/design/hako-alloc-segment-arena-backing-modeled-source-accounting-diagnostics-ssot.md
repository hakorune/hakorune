# Hako Alloc Segment Arena Backing Modeled Source Accounting Diagnostics SSOT

Status: active
Date: 2026-05-19
Decision: accepted

## Purpose

Observe MIMAP-264A modeled source accounting inventory counters and publish
observer-only scalar diagnostic facts before closeout.

The diagnostics row reads inventory counters and the last source accounting
report. It does not record new modeled source accounting rows and does not open
real pointer residence, pointer-derived lookup, or arena backing.

## Owner

```text
lang/src/hako_alloc/memory/segment_arena_backing_modeled_source_accounting_diagnostic_box.hako
```

The owner may:

- observe modeled source accounting inventory counters;
- publish reject-category seen bits;
- mirror the last report reason, accounting facts, and closed-substrate
  blocker count;
- reject missing inventory / missing report presence.

The owner must not:

- call `recordSourceAccounting`;
- mutate modeled source accounting inventory counters;
- create real raw pointer residence;
- perform pointer-derived lookup or dereference;
- allocate real arena backing;
- mutate a real segment-map;
- execute atomic bitmap claims;
- call page-source or OSVM seams;
- infer anything from owner names or backend matchers.

## Reason Vocabulary

| Reason | Meaning |
| --- | --- |
| `0` | modeled source accounting diagnostics observed |
| `1` | modeled source accounting inventory/report missing |

When observed, the diagnostic report mirrors the last modeled source accounting
report reason in its `reason` field so closeout can see the terminal reject
category.

## Validation

MIMAP-265A uses daily L2 validation:

```bash
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_source_accounting_diagnostics_guard.sh --level L2
```

The guard must:

- prove observer-only diagnostic publication;
- prove all MIMAP-264A reject categories are surfaced as seen bits;
- prove empty inventory is rejected;
- prove inactive execution flags remain zero;
- prove the MIR JSON has typed diagnostic report fields and the expected route
  surface.

## Stop Lines

- No new source accounting rows.
- No real raw pointer residence.
- No pointer-derived lookup or dereference.
- No real arena backing allocation.
- No real segment-map mutation.
- No real segment allocation/free execution.
- No atomic bitmap execution.
- No OSVM/page-source execution.
- No TLS, worker-local, worker scheduling, or source-level concurrency.
- No provider activation, host allocator replacement, hooks, or
  `#[global_allocator]`.
- No cross-function `Result` direct ABI or runtime sum materialization.
- No backend `.inc` matcher by app or owner name.
