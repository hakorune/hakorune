---
Status: Landed
Date: 2026-05-25
Scope: implement a narrow benchres.csv schema adapter for the hakmem corpus.
Related:
  - tools/allocator/hakmem_benchres_adapter.py
  - tools/allocator/hakmem_external_bench.py
---

# 295x-84 Hakmem Benchres Adapter

## Blocker

```text
MIMALLOC-COMPARISON-HAKMEM-BENCHRES-ADAPTER-295X-001
```

## Implementation

`tools/allocator/hakmem_benchres_adapter.py` reads a `mimalloc-bench`
`benchres.csv` file and emits phase-295x-style key-value evidence:

```text
output_contract=hakmem-external-benchres-adapter-v0
dataset_role=external-historical-benchmark-corpus
winner_claim=0
```

```bash
tools/allocator/hakmem_benchres_adapter.py \
  --in target/hakmem-bench/results/cfrac_sys_mimalloc.benchres.csv
```

The adapter normalizes:

```text
sys -> system
mi  -> mimalloc
tc  -> tcmalloc
```

and publishes:

```text
elapsed_ms
peak_rss_bytes
user_sec
sys_sec
major_faults
minor_faults
```

## Boundary

The adapter is still a historical/external corpus bridge. Its output is not a
phase-295x repeated measurement pack, and `winner_claim=0` remains mandatory.

## Selected Next

Select:

```text
MIMALLOC-COMPARISON-HAKMEM-BENCHRES-ADAPTER-CLOSEOUT-295X-001
```

The closeout should decide whether to add a `hakozuna_compare_*.log` adapter,
run a wider `mimalloc-bench` subset, or return to `.hako` workload porting.

## Verification

```bash
python3 -m py_compile tools/allocator/hakmem_benchres_adapter.py
tools/allocator/hakmem_benchres_adapter.py --in target/hakmem-bench/results/cfrac_sys_mimalloc.benchres.csv
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
