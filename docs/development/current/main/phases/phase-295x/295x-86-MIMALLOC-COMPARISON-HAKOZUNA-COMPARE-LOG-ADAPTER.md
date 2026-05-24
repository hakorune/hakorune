---
Status: Landed
Date: 2026-05-25
Scope: adapt hakmem hakozuna_compare logs into phase-295x-style evidence.
Related:
  - tools/allocator/hakmem_hakozuna_compare_log_adapter.py
  - /home/tomoaki/git/hakmem_20260525_extracted/hakmem/bench_results/hakozuna_compare_20260118_034633
---

# 295x-86 Hakozuna Compare Log Adapter

## Blocker

```text
MIMALLOC-COMPARISON-HAKOZUNA-COMPARE-LOG-ADAPTER-295X-001
```

## Implementation

`tools/allocator/hakmem_hakozuna_compare_log_adapter.py` reads one
`hakozuna_compare_*.log` file and emits:

```text
output_contract=hakmem-external-hakozuna-compare-log-adapter-v0
dataset_role=external-historical-benchmark-corpus
winner_claim=0
```

The adapter extracts:

```text
timestamp
git
label
iterations
working_set
declared_runs
run_count
throughput_min/median/max_ops_per_sec
elapsed_min/median/max_ms
peak_rss_min/median/max_bytes
```

and preserves the first run rows for schema inspection.

## Selected Next

Select:

```text
MIMALLOC-COMPARISON-HAKMEM-EXTERNAL-CORPUS-CATALOG-295X-001
```

The next row should catalog the representative historical artifacts and choose
the next workload alignment target.

## Stop Line

This row does not claim speed/RSS winners, import historical logs as current
repeated measurement evidence, run heavy benchmark packs, or open
provider/DLL/replacement/hook/global allocator seams.

## Verification

```bash
python3 -m py_compile tools/allocator/hakmem_hakozuna_compare_log_adapter.py
tools/allocator/hakmem_hakozuna_compare_log_adapter.py --in /home/tomoaki/git/hakmem_20260525_extracted/hakmem/bench_results/hakozuna_compare_20260118_034633/hakozuna_compare_20260118_034633_mimalloc_e165faccc.log
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

