---
Status: Landed
Date: 2026-05-27
Scope: open the post-port mimalloc benchmark contract lane and keep DLL/provider work closed.
Blocker: MIMALLOC-BENCHMARK-LANE-LOCK-296X-001
Related:
  - docs/development/current/main/design/mimalloc-benchmark-dll-roadmap-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md
---

# 296x-00 Mimalloc Benchmark Lane Lock

## Decision

Close:

```text
MIMALLOC-BENCHMARK-LANE-LOCK-296X-001
```

Open phase-296x as a benchmark contract lane after the first `.hako` mimalloc
port pass.

The lane starts from the external corpus:

```text
/home/tomoaki/git/hakmem_20260525_extracted/hakmem
```

and turns existing benchmark assets into stable Hakorune-side contracts before
DLL/provider work begins.

## Stop Line

This row does not open:

- provider package / DLL generation;
- provider activation and provider API execution;
- process allocator replacement, hooks, backend matchers, or
  `#[global_allocator]`;
- performance or memory winner claims.

## Selected Next

Select:

```text
MIMALLOC-BENCHMARK-HAKMEM-ASSET-INVENTORY-296X-001
```

The next row should inventory the external `hakmem` corpus and choose the first
adapter row.
