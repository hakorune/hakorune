---
Status: Landed
Date: 2026-05-28
Scope: classify duplicate reason global calls in objectLifecycleSmallAlloc failure return blocks.
Blocker: HAKO-MIMALLOC-SMALL-ALLOC-DUPLICATE-REASON-CALL-PROBE-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-120-HAKO-MIMALLOC-SMALL-ALLOC-RETURN-BLOCK-LOCAL-SSA-COPY-PROBE.md
---

# 296x-121 Hako Mimalloc Small Alloc Duplicate Reason Call Probe

## Purpose

Row120 found that failure return blocks call the same reason global twice:

```text
reason_call_count=10
duplicate_reason_call_count=5
next_action=reason_call_probe
```

Classify whether the next keeper should bind the reason once in `.hako`, add a
MIR call-CSE probe, or introduce a narrow reason-singleton lowering rule.

## Required Output

```text
output_contract=hako-mimalloc-small-alloc-duplicate-reason-call-probe-v0
input_contract=hako-mimalloc-small-alloc-return-block-local-ssa-copy-probe-v0
selected_owner=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
duplicate_reason_call_count=5
reason_call_count=10
failure_return_block_count=5
next_action=hako_reason_bind_probe|mir_call_cse_probe|reason_singleton_lowering_probe|stop_line
summary=ok
```

## Stop Line

Do not implement a `.hako` keeper or MIR builder change in this row.

## Evidence

Command:

```bash
tmp=$(mktemp -d /tmp/hakorune_row121_actual.XXXXXX)
NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 target/release/hakorune \
  --backend mir \
  --emit-mir-json "$tmp/app.mir.json" \
  apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako >/dev/null
python3 tools/allocator/hako_mimalloc_small_alloc_duplicate_reason_call_probe.py \
  --mir-json "$tmp/app.mir.json" \
  --source lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako \
  --out "$tmp/report.out"
cat "$tmp/report.out"
```

Report:

```text
output_contract=hako-mimalloc-small-alloc-duplicate-reason-call-probe-v0
input_contract=hako-mimalloc-small-alloc-return-block-local-ssa-copy-probe-v0
selected_owner=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
source_reason_call_count=5
reason_call_count=10
reason_effect_io_count=10
duplicate_reason_call_count=5
duplicate_unused_reason_call_count=5
failure_return_block_count=5
selected_reason=nested_reason_call_duplicated_with_unused_first_result_and_io_effect
next_action=hako_reason_bind_probe
next_diagnostic=small_alloc_hako_reason_bind_probe
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_hako_mimalloc_small_alloc_duplicate_reason_call_probe_guard.sh
```

## Decision

Source has five reason calls in `objectLifecycleSmallAlloc`, but MIR has ten.
Each failure return block duplicates the same reason global call, and the first
result is unused. Because the duplicate calls carry `IO` effects in MIR, do not
start with generic MIR call CSE. Next row should probe the narrower `.hako`
reason-local binding shape.
