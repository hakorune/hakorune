---
Status: Active
Date: 2026-05-24
Scope: SSOT for the post-294x mimalloc comparison execution lane.
Related:
  - docs/development/current/main/phases/phase-295x/README.md
  - docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md
  - docs/development/current/main/phases/phase-294x/294x-270-PHASE-294X-USIZE-COMPARISON-CLOSEOUT.md
---

# Mimalloc Comparison Execution SSOT

## Decision

Phase-295x resumes mimalloc-facing work from a comparison execution seam.

The goal is not provider activation, DLL packaging, host allocator replacement,
or native allocator replacement. The goal is a narrow, reproducible comparison
between:

- the existing `.hako` / `hako_alloc` vertical-slice evidence; and
- the explicit C mimalloc runner evidence surface.

## Boundary

Open:

- explicit runner output/evidence shape selection;
- stable workload and metric contract checks;
- comparison ledger/presentation rows that consume existing evidence;
- `.hako` port rows that directly improve the comparison workload.

Keep closed:

- provider package / DLL generation;
- provider activation and provider API execution;
- process allocator replacement, hooks, backend matchers, and
  `#[global_allocator]`;
- worker/TLS, true threads, atomics, remote-free stress, abandoned heap stress,
  and native allocator replacement claims;
- broad production `usize` field migration not required by the comparison
  workload.

## Validation Cadence

- Planning/inventory rows: docs/static guard.
- `.hako` model/evidence rows: VM + MIR JSON + route preflight.
- First execution or closeout rows: representative L3 where the row explicitly
  requires it.
- Heavy allocator-wide packs: explicit only.

## Comparison Method

Phase-295x compares in layers. Do not collapse these layers into one winner
claim.

### Layer 1: Contract / Schema

Compare whether both sides publish stable evidence:

- output contract id;
- workload id;
- allocation/free counts;
- requested bytes;
- memory-use evidence presence;
- stop-line fields for replacement / hook / backend matcher / provider package.

This layer answers:

```text
Can the `.hako` evidence and C mimalloc evidence be consumed by one ledger?
```

It does not answer:

```text
Which allocator is faster or more memory efficient?
```

### Layer 2: Semantic / Workload Shape

Compare only fields that mean the same thing:

- operation family;
- request-size family;
- allocation/free/reuse/realloc/huge behavior category;
- failure/reject reason category;
- live handle / released handle evidence where the schema defines it.

If the workload ids or operation families differ, the row may compare schema
compatibility and evidence availability, but it must not compare speed or RSS as
an allocator-quality conclusion.

Current status:

- `.hako` V5 vertical-slice evidence combines small, realloc/aligned, and
  huge/OSVM model slices.
- the C mimalloc explicit runner currently uses
  `representative-small-block-v0`.

Therefore the current ledger is a contract/evidence bridge, not a final
apples-to-apples performance benchmark.

### Layer 3: Memory Evidence

Memory evidence is recorded as evidence, not as a winner claim, until a row
selects a repeated apples-to-apples benchmark pack.

Current acceptable memory fields:

- C runner `peak_rss_bytes`;
- `.hako` requested/committed/live evidence exposed by the selected vertical
  slice;
- bridge fields that state whether the evidence exists.

Rows must not compare unrelated memory concepts as if they were identical.
For example, requested bytes, committed bytes, live bytes, and RSS are different
observations.

### Layer 4: Performance / Winner Claims

Performance and memory winner claims require a later row with:

- identical workload ids or an explicit workload equivalence map;
- repeated runs;
- warmup policy;
- summary statistic policy;
- environment capture;
- no provider/hook/replacement side effects unless that lane is explicitly
  opened.

Until that row exists, phase-295x may say:

```text
evidence is comparable
schema is stable
ledger is accepted
```

It must not say:

```text
hako_alloc is faster/slower than mimalloc
hako_alloc uses less/more memory than mimalloc
```

## Stop Line

Do not use the comparison lane as a back door for allocator-provider
activation or full native mimalloc reimplementation. If a row needs those
surfaces, split it into a later phase/lane with a new SSOT.
