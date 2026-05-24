---
Status: Landed
Date: 2026-05-25
Scope: select a schema adapter for hakmem benchres artifacts.
Related:
  - docs/development/current/main/phases/phase-295x/295x-82-MIMALLOC-COMPARISON-HAKMEM-EXTERNAL-BENCH-BRIDGE.md
  - tools/allocator/hakmem_benchres_adapter.py
---

# 295x-83 Hakmem Schema Adapter Selection

## Blocker

```text
MIMALLOC-COMPARISON-HAKMEM-SCHEMA-ADAPTER-SELECTION-295X-001
```

## Decision

Select a narrow `benchres.csv` adapter before importing any historical
`hakmem` result as current phase evidence.

The first adapter reads whitespace-delimited `mimalloc-bench` `benchres.csv`
rows and emits key-value evidence with:

```text
output_contract=hakmem-external-benchres-adapter-v0
dataset_role=external-historical-benchmark-corpus
winner_claim=0
```

This adapter is for schema discovery and workload alignment only.

## Selected Next

Select:

```text
MIMALLOC-COMPARISON-HAKMEM-BENCHRES-ADAPTER-295X-001
```

## Stop Line

This row does not import historical `hakmem` CSV/log rows as phase-295x
repeated measurement evidence, compute winners, run heavy benchmark packs, or
open provider/DLL/replacement/hook/global allocator seams.

