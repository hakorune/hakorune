---
Status: Active
Date: 2026-05-27
Scope: taskboard for phase-296x mimalloc benchmark contract lane.
Related:
  - docs/development/current/main/design/mimalloc-benchmark-dll-roadmap-ssot.md
  - docs/development/current/main/phases/phase-296x/README.md
---

# 296x-90 Mimalloc Benchmark Taskboard

## Rule

Benchmark contract work comes before DLL/provider work. Do not use this phase
to activate a provider, replace the process allocator, install hooks, or make
winner claims.

## Current Truth

- Phase-295x landed the first `.hako` mimalloc comparison/remote-free pass.
- The external `hakmem` corpus exists at:

```text
/home/tomoaki/git/hakmem_20260525_extracted/hakmem
```

- The corpus includes benchmark binaries, source, historical `benchres.csv`,
  `hakozuna_compare` logs, perf data, and strace data.
- Phase-296x should make those assets usable through stable Hakorune-side
  contracts before DLL/provider work begins.

## Current Blocker

```text
MIMALLOC-DLL-LOAD-ONLY-SELECTION-296X-001:
  Select load-only DLL metadata smoke after benchmark contracts are stable.
```

## Queue

| Order | Row | Status | Boundary |
| --- | --- | --- | --- |
| 0 | `MIMALLOC-BENCHMARK-LANE-LOCK-296X-001` | Landed | Open the benchmark contract lane and keep DLL/provider/replacement seams closed. |
| 1 | `MIMALLOC-BENCHMARK-HAKMEM-ASSET-INVENTORY-296X-001` | Landed | Inventory the external hakmem corpus and select first adapter rows. |
| 2 | `MIMALLOC-BENCHMARK-RESULT-CONTRACT-296X-001` | Landed | Define the stable benchmark result vocabulary before parsing external logs. |
| 3 | `MIMALLOC-BENCHMARK-HAKMEM-BENCHRES-ADAPTER-296X-001` | Landed | Convert selected `benchres.csv` rows to Hakorune benchmark evidence with winner claims closed. |
| 4 | `MIMALLOC-BENCHMARK-HAKOZUNA-COMPARE-LOG-ADAPTER-296X-001` | Landed | Convert selected `hakozuna_compare` logs to Hakorune benchmark evidence. |
| 5 | `MIMALLOC-BENCHMARK-EXACT-EXE-HARNESS-PILOT-296X-001` | Landed | Run one already-landed `.hako` workload through a benchmark harness using the accepted result contract. |
| 6 | `MIMALLOC-BENCHMARK-EXTERNAL-CORPUS-CLOSEOUT-296X-001` | Landed | Close corpus adapter bring-up and select the first real repeated measurement row. |
| 7 | `MIMALLOC-BENCHMARK-EXACT-EXE-REPEATED-MEASUREMENT-296X-001` | Landed | Run the selected same workload with process-repeat timing and repeated samples. |
| 8 | `MIMALLOC-DLL-LOAD-ONLY-SELECTION-296X-001` | Current | Select load-only DLL metadata smoke after benchmark contracts are stable. |

## Mini-Agent Restart Queue

Pick exactly one slice.

### Slice 1 - Hakmem Asset Inventory

Purpose: make the external corpus readable from Hakorune docs without parsing
everything yet.

Allowed files:

- `docs/development/current/main/phases/phase-296x/296x-01-MIMALLOC-BENCHMARK-HAKMEM-ASSET-INVENTORY.md`
- optional tiny helper under `tools/allocator/` if the inventory row chooses to
  generate a report
- the row guard for the inventory row

Read-only external path:

```text
/home/tomoaki/git/hakmem_20260525_extracted/hakmem
```

Done when the row lists selected artifacts, rejects unsafe artifacts, and
chooses exactly one next adapter.

### Slice 2 - Result Contract

Purpose: define the stable output vocabulary before any parser is written.

Allowed files:

- a new `296x-02-*` card
- `docs/development/current/main/design/mimalloc-benchmark-dll-roadmap-ssot.md`
- a row guard

Required stop line:

```text
winner_claim=0
provider_active=0
replacement_active=0
global_allocator=0
```

### Slice 3 - Benchres Adapter

Purpose: parse one selected `benchres.csv` corpus family into the accepted
result contract.

Allowed files:

- `tools/allocator/*benchres*`
- a focused test/guard
- the active row card

Do not edit external corpus files.

### Slice 4 - Hakozuna Compare Adapter

Purpose: parse one selected `hakozuna_compare_*.log` family into the accepted
result contract.

Allowed files:

- `tools/allocator/*hakozuna_compare*`
- a focused test/guard
- the active row card

Do not open DLL/provider work.

### Slice 5 - Exact-EXE Repeated Measurement

Purpose: run the selected same workload with real repeated measurement policy.

Required policy:

```text
sample_count=3
warmup_count=1
operation_repeat=128
winner_claim=0
```

Do not open DLL/provider work.

### Slice 6 - DLL Load-Only Selection

Purpose: only after adapter closeout, select a load-only DLL metadata smoke.

This slice is not eligible until rows 1-7 are landed.
