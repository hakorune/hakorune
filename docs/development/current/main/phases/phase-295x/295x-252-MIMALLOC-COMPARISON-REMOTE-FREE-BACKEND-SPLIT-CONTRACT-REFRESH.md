---
Status: Current
Date: 2026-05-26
Scope: refresh backend-split comparison contract on existing remote-free minimum benchmark workloads.
Blocker: MIMALLOC-COMPARISON-REMOTE-FREE-BACKEND-SPLIT-CONTRACT-REFRESH-295X-002
Related:
  - docs/development/current/main/phases/phase-295x/295x-251-MIMALLOC-COMPARISON-REMOTE-FREE-BACKEND-SPLIT-SELECTION.md
  - apps/mimalloc-remote-free-minimum-benchmark-run-proof/main.hako
  - apps/mimalloc-remote-free-minimum-benchmark-run-proof/remote_free_publish_only.hako
  - apps/mimalloc-remote-free-minimum-benchmark-run-proof/remote_free_collect_only.hako
  - apps/mimalloc-remote-free-minimum-benchmark-run-proof/remote_free_publish_collect_cycle.hako
---

# 295x-252 Remote-Free Backend-Split Contract Refresh

## Decision

Open:

```text
MIMALLOC-COMPARISON-REMOTE-FREE-BACKEND-SPLIT-CONTRACT-REFRESH-295X-002
```

Refresh the comparison contract so backend split can be observed on the same
remote-free minimum benchmark workload pack without changing semantic workload
shape.

## Implementation Contract

Keep the existing `.hako` workload surfaces and fixed policy:

```text
operation_repeat=128
warmup_count=1
sample_count=5
```

and add backend-split contract lines over the same pack:

```text
backend_split_scope=remote-free-minimum-v0
backend_split_family=split-observation
summary=ok
```

No workload expansion, no per-workload winner claim, and no native C split in
this row.

## Perf / ASM / MIR Anchor (2026-05-26)

Recent high-repeat measurements and profiling over the existing remote-free
minimum pack found that the publish+collect cycle path stays the slowest
variant and is dominated by field access churn in the inbox/report path.

Observed highlights:

```text
remote_free_publish_collect_cycle (repeat64): median 110.6ms
remote_free_collect_only (repeat64): median 86.5ms
hot symbols (remote cycle): field_get_hii / field_set_hii / field_get_u64 / field_set_u64
```

MIR shape also shows dense field access and call traffic around
`HakoAllocRemoteFreePageInbox.publish` and `collectPending`.

Selected next implementation slice (implementation-first, no new workload/docs
expansion): optimize `.hako` inbox hot path by caching repeated `me.*` reads and
performing single writeback where possible in:

- `lang/src/hako_alloc/memory/remote_free_page_integration_box.hako`
  - `HakoAllocRemoteFreePageInbox.publish`
  - `HakoAllocRemoteFreePageInbox.collectPending`

Follow-up measurement (same repeat64 bundle, 2026-05-26) after the above slice:

```text
remote_free_publish_collect_cycle median: 110.6ms -> 60.245ms (-45.5%)
remote_free_collect_only median: 86.5ms -> 56.109ms (-35.1%)
```

Current next small seam:

- `lang/src/hako_alloc/memory/remote_free_page_integration_box.hako`
  - `HakoAllocRemoteFreePageExerciseReport.ok` local cache pass to reduce
    repeated `field_get_*` traffic during the compound check.

## Ownership Freeze (2026-05-26)

Current ownership judgment for remaining hotspot:

```text
split estimate: .hako 75% / mirbuilder 25%
```

Evidence basis:

- key hot methods show near 1:1 mapping between source `me.*` reads/writes and
  MIR `field_get/set`
- no dominant sign of builder-introduced extra field access in the current
  remote-free path

Action tracks:

- Track A (now): keep reducing success-path `me.*` reads/writes in
  `lang/src/hako_alloc/memory/**`
- Track B (later, smallest compiler seam): evaluate conservative same-block
  `FieldGet` CSE with invalidation on `FieldSet` / impure call / branch

Track-B first cut (2026-05-26) landed:

- `src/mir/passes/cse.rs` now performs conservative same-block `FieldGet`
  forwarding (`FieldGet -> Copy`) and invalidates cached entries on side-effect
  instructions.

## Guard / Validation Contract

Use existing guards:

```text
bash tools/checks/impl/phase295x_mimalloc_remote_free_minimum_benchmark_run_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Stop Line

Do not open provider activation, DLL/shared-library packaging,
replacement/hooks/global allocator seams, thread/TLS/atomic expansions, or
native C/mimalloc winner claims.
