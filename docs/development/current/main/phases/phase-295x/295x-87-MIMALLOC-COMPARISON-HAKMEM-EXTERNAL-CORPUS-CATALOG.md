---
Status: Landed
Date: 2026-05-25
Scope: catalog selected hakmem external artifacts and choose the next workload alignment target.
Related:
  - docs/development/current/main/phases/phase-295x/295x-hakmem-external-results-catalog-v0.toml
  - tools/allocator/hakmem_benchres_adapter.py
  - tools/allocator/hakmem_hakozuna_compare_log_adapter.py
---

# 295x-87 Hakmem External Corpus Catalog

## Blocker

```text
MIMALLOC-COMPARISON-HAKMEM-EXTERNAL-CORPUS-CATALOG-295X-001
```

## Catalog

The selected external corpus artifacts are listed in:

```text
docs/development/current/main/phases/phase-295x/295x-hakmem-external-results-catalog-v0.toml
```

The catalog records:

- clean/latest `mimalloc-bench` `benchres.csv` candidates;
- `hakozuna_compare_*` log-set candidates;
- `malloc-large` perf/strace secondary evidence;
- mandatory stop-line fields.

## Decision

Select:

```text
MIMALLOC-COMPARISON-MIMALLOC-BENCH-MALLOC-LARGE-WORKLOAD-CONTRACT-295X-001
```

as the next `.hako` port alignment row.

Rationale:

- `malloc-large` is allocator-focused and available in the extracted
  `mimalloc-bench` executable set;
- it is narrower than `larson` / `mstress` / thread-heavy or remote-free
  families;
- it helps align `.hako` huge-ish/page-source evidence with an external
  benchmark family without opening provider activation or process replacement.

## Stop Line

This row does not import historical corpus rows as current repeated
measurement evidence, compute speed/RSS winners, run heavy external packs, or
open provider/DLL/replacement/hook/global allocator seams, threads, atomics,
remote-free stress, or abandoned-heap stress.

