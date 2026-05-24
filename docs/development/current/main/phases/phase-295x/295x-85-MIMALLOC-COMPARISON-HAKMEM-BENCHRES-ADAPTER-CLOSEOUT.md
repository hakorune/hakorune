---
Status: Landed
Date: 2026-05-25
Scope: close the hakmem benchres adapter and select the hakozuna_compare log adapter.
Related:
  - docs/development/current/main/phases/phase-295x/295x-84-MIMALLOC-COMPARISON-HAKMEM-BENCHRES-ADAPTER.md
  - tools/allocator/hakmem_hakozuna_compare_log_adapter.py
---

# 295x-85 Hakmem Benchres Adapter Closeout

## Blocker

```text
MIMALLOC-COMPARISON-HAKMEM-BENCHRES-ADAPTER-CLOSEOUT-295X-001
```

## Closeout

The `benchres.csv` adapter is accepted as a narrow schema-discovery bridge for
`mimalloc-bench` historical output:

```text
output_contract=hakmem-external-benchres-adapter-v0
dataset_role=external-historical-benchmark-corpus
winner_claim=0
```

## Selected Next

Select:

```text
MIMALLOC-COMPARISON-HAKOZUNA-COMPARE-LOG-ADAPTER-295X-001
```

`hakozuna_compare_*.log` files contain repeated run-level throughput/RSS
evidence and should be adapted before choosing an external workload alignment
catalog.

## Stop Line

This row does not import historical rows as phase-295x repeated measurement
evidence, compute winners, run heavy benchmark packs, or open
provider/DLL/replacement/hook/global allocator seams.

