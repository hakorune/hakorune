---
Status: Landed
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

Close:

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

Current hot-loop cleanup status:

- remote-free minimum publish-only, collect-only, and publish+collect apps now
  allocate their pointer fixtures and page port outside the operation loop.
- loop bodies call facade fast wrappers instead of rebuilding port/page/inbox
  objects per iteration.
- `HakoAllocRemoteFreePageInbox.publish` reuses `pending_block_ids` slots when
  `pending_top` has been reset, so reused ports do not grow the pending array
  on every iteration.

No further remote-free minimum hot-loop cleanup is selected for this card unless
a fresh perf sample identifies a new owner.

## Mini-Agent Task Slices

Use this section as the restart checklist for smaller models. Pick exactly one
slice, keep the edit in the listed files, and commit only after the listed guard
passes.

### Slice 0 - Verify Current Remote-Free Minimum

Purpose: confirm the current remote-free minimum pack is green before opening a
new migration seam.

Files to read only:

- `apps/mimalloc-remote-free-minimum-benchmark-run-proof/*.hako`
- `lang/src/hako_alloc/memory/allocator_facade_box.hako`
- `lang/src/hako_alloc/memory/remote_free_page_integration_box.hako`

Commands:

```text
git status -sb
bash tools/checks/impl/phase295x_mimalloc_remote_free_minimum_benchmark_run_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Done when: all commands are green and no source change is needed.

### Slice 1 - Realloc/Aligned Facade Route Readability

Purpose: make the realloc/aligned path easier to port and optimize without
changing workload shape.

Allowed files:

- `apps/hako-alloc-mimalloc-comparison-realloc-aligned-exe-proof/main.hako`
- `lang/src/hako_alloc/memory/allocator_facade_box.hako`
- narrow realloc proof files only if the guard requires them

First commands:

```text
bash tools/checks/k2_wide_mimalloc_facade_realloc_grow_exe_guard.sh
bash tools/checks/k2_wide_mimalloc_facade_realloc_shrink_exe_guard.sh
bash tools/checks/k2_wide_mimalloc_realloc_alloc_copy_release_guard.sh
```

Allowed edit: move repeated success-path checks into facade-owned helpers, or
split long app-local checks into named proof helpers. Do not add a new workload.

Done when: the three commands above pass and the app still prints
`workload=representative-realloc-aligned-v0`.

### Slice 2 - Mixed-Small Workload Readability

Purpose: prepare the mixed-small comparison app for later body-timing without
changing operation semantics.

Allowed files:

- `apps/hako-alloc-mimalloc-comparison-mixed-small-exe-proof/main.hako`
- existing `lang/src/hako_alloc/memory/**` facade/helper owners used by the app

First commands:

```text
bash tools/checks/k2_wide_phase295x_mixed_size_evidence_run_guard.sh
```

Allowed edit: separate setup/body/verify/cleanup inside the `.hako` app, or
route repeated app-local checks through existing facade helpers.

Done when: the guard passes and the app still prints
`workload=representative-mixed-small-v0`.

### Slice 3 - Huge/OSVM Slice Readability

Purpose: keep the huge/OSVM comparison path readable before deeper porting.

Allowed files:

- `apps/hako-alloc-mimalloc-comparison-huge-osvm-slice-proof/main.hako`
- existing huge/page-source/page-model owners under `lang/src/hako_alloc/memory/`

First commands:

```text
bash tools/checks/k2_wide_hako_alloc_mimalloc_comparison_huge_osvm_slice_guard.sh
```

Allowed edit: reduce duplicated app-local checks or give repeated setup/body
blocks named helpers. Do not open provider activation, native allocator
replacement, or winner-claim seams.

Done when: the guard passes and the app still prints `workload=huge-osvm-v1`.

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

## Selected Next

Select:

```text
MIMALLOC-COMPARISON-MALLOC-LARGE-MEMORY-GAP-CLOSEOUT-295X-001
```

The next row should close the external `malloc-large` memory-gap attribution
pack and choose the next comparison seam.
