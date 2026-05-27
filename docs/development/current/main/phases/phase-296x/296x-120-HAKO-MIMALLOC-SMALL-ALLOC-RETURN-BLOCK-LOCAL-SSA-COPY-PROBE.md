---
Status: Landed
Date: 2026-05-28
Scope: classify local SSA copy materialization inside objectLifecycleSmallAlloc return blocks.
Blocker: HAKO-MIMALLOC-SMALL-ALLOC-RETURN-BLOCK-LOCAL-SSA-COPY-PROBE-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-119-HAKO-MIMALLOC-SMALL-ALLOC-MULTI-RETURN-COPY-PROBE.md
---

# 296x-120 Hako Mimalloc Small Alloc Return Block Local SSA Copy Probe

## Purpose

Row119 showed return blocks contain copy pressure, but not copies into the
return value itself:

```text
return_block_copy_count=23
copy_to_return_value_count=0
next_action=local_ssa_copy_probe
```

Classify whether these copies come from receiver materialization, argument
materialization, or duplicate reason-object calls before selecting another
MIR-builder change.

## Required Output

```text
output_contract=hako-mimalloc-small-alloc-return-block-local-ssa-copy-probe-v0
input_contract=hako-mimalloc-small-alloc-multi-return-copy-probe-v0
selected_owner=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
return_block_copy_count=23
receiver_copy_count
arg_copy_count
duplicate_reason_call_count
next_action=receiver_materialization_probe|arg_materialization_probe|reason_call_probe|stop_line
summary=ok
```

## Stop Line

Do not implement a MIR builder change in this row.

## Evidence

Command:

```bash
tmp=$(mktemp -d /tmp/hakorune_row120_actual.XXXXXX)
NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 target/release/hakorune \
  --backend mir \
  --emit-mir-json "$tmp/app.mir.json" \
  apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako >/dev/null
python3 tools/allocator/hako_mimalloc_small_alloc_return_block_local_ssa_copy_probe.py \
  --mir-json "$tmp/app.mir.json" \
  --out "$tmp/report.out"
cat "$tmp/report.out"
```

Report:

```text
output_contract=hako-mimalloc-small-alloc-return-block-local-ssa-copy-probe-v0
input_contract=hako-mimalloc-small-alloc-multi-return-copy-probe-v0
selected_owner=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
return_block_count=6
return_block_copy_count=23
receiver_copy_count=7
arg_copy_count=9
reason_call_count=10
duplicate_reason_call_count=5
selected_reason=failure_return_blocks_duplicate_reason_global_calls
next_action=reason_call_probe
next_diagnostic=small_alloc_duplicate_reason_call_probe
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_hako_mimalloc_small_alloc_return_block_local_ssa_copy_probe_guard.sh
```

## Decision

The return-block copies are not primarily return-value copies. Failure return
blocks duplicate reason global calls, with two calls per failure reason shape.
Next row: classify whether this should be fixed in `.hako` by binding the reason
once, in MIR by call CSE, or by a reason singleton lowering rule.
