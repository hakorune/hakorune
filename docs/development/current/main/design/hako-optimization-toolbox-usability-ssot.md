---
Status: SSOT
Date: 2026-05-27
Scope: optimization toolbox entry points for hako_check, MIR shape, mimalloc exact-EXE measurement, and row guards.
Related:
  - AGENTS.md
  - tools/hako_check/README.md
  - docs/development/current/main/design/hako-check-mir-observation-boundary-ssot.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md
  - docs/tools/check-scripts-index.md
---

# Hako Optimization Toolbox Usability SSOT

## Purpose

This document is the quick map for optimization work that uses `hako_check`,
MIR shape tools, exact-EXE measurement, and row guards.

The goal is not to make `hako_check` an optimizer. The goal is to make the
tool chain easy to follow:

```text
measure -> source surface -> MIR shape -> owner selection -> guard surface -> implementation -> measurement
```

## Quick Entry

Start here when an optimization task asks "where is the cost?" or "which tool
should I use next?"

```text
1. git status -sb
2. bash tools/checks/current_state_pointer_guard.sh
3. Read CURRENT_STATE.toml current_blocker_token
4. Read the active card in phase-296x if the lane is mimalloc/perf parity
5. Choose the smallest tool surface below
```

Do not start by reading broad source trees unless the active card is already a
source-level implementation row.

## Tool Surfaces

### Source Surface

Owner:

```text
tools/hako_check/perf_surface_inventory.py
tools/hako_check/README.md
```

Use when the question is:

```text
Which .hako method has source-level risk?
Is the risk method calls, field access, ArrayBox access, linear scan, or allocation-like source?
```

Contract:

```text
output_contract=hako-check-perf-surface-v1
```

Stop line:

```text
hako_check observes source only.
It does not rewrite .hako, analyze actual lowered MIR cost, or select benchmark winners.
```

### MIR Shape Surface

Owner:

```text
tools/mir_check/method_shape_report.py
tools/allocator/hako_source_mir_shape_join.py
tools/allocator/hako_mimalloc_small_alloc_mir_shape_deep_dive.py
tools/allocator/hako_mimalloc_small_alloc_phi_copy_lowering_probe.py
```

Use when the question is:

```text
Does the source risk actually lower into calls, fields, arrays, PHIs, copies, branches, or returns?
Is the current owner still .hako source shape, or has it moved into MIR builder lowering?
```

Contracts:

```text
output_contract=hako-mir-method-shape-v0
output_contract=hako-source-mir-shape-join-v1
output_contract=hako-mimalloc-small-alloc-mir-shape-deep-dive-v0
output_contract=hako-mimalloc-small-alloc-phi-copy-lowering-probe-v0
```

Stop line:

```text
MIR shape tools classify lowered shape.
They do not implement MIR builder changes.
```

### Measurement Surface

Owner:

```text
tools/allocator/hako_exe_memory_runner.sh
tools/allocator/hako_mimalloc_*measurement*.py
```

Use when the question is:

```text
Did the implementation change exact-EXE behavior or timing?
Are provider activation, replacement, hooks, globals, and winner claims still closed?
```

Required closure fields for this lane:

```text
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

### Row Guard Surface

Owner:

```text
tools/checks/k2_wide_phase296x_*.sh
docs/tools/check-scripts-index.md
```

Use when the question is:

```text
Is the current row's card, taskboard, CURRENT_STATE pointer, tool contract, and stop line coherent?
```

Every new optimization row that adds a durable tool should add:

```text
1. tool under tools/allocator, tools/hako_check, or tools/mir_check
2. guard under tools/checks
3. check-scripts-index entry
4. active card evidence
```

## Phaselet Pattern

For optimization tool usability, prefer small phaselets:

```text
tool-contract
tool-implementation
guard-surface
first-consumer
closeout
```

For performance keeper work, prefer:

```text
measurement
owner-selection
guard-surface
implementation
post-measurement
next-owner-refresh
```

Do not combine `.hako` keeper work with MIR builder owner selection in the same
row. If two non-keepers occur in the same owner family, stop the line and move
to shape/owner diagnostics.

## Current Mimalloc Lessons

The row112-118 slice established the useful pattern:

```text
1. rollback measurement proved the accepted baseline still held
2. source/MIR refresh rejected two non-keepers
3. MIR shape deep-dive showed phi/copy dominance
4. lowering probe classified local copy churn
5. owner selection picked materialize_vars_single_pred_at_entry
6. guard surface fixed before/after shape and exact-EXE measurement
7. implementation removed single-incoming PHIs while keeping measurement claims conservative
```

Result:

```text
objectLifecycleSmallAlloc MIR instructions: 247 -> 191
single_incoming_phi_count: 61 -> 0
exact-EXE median: 620ms -> 620ms
winner_claim=0
```

Interpretation:

```text
This was a real MIR shape cleanup, not yet a performance win.
The next owner should be selected from remaining multi-return/copy evidence.
```

## Boundary Rules

- `hako_check` is source observation only.
- MIR tools are lowered-shape observation only.
- Measurement tools provide evidence; they do not justify broad rewrites alone.
- Guard surfaces come before implementation when touching MIR builder behavior.
- Provider activation, process allocator replacement, hooks, globals, and winner
  claims stay closed unless a separate decision row opens them.
