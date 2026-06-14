---
Status: Landed
Date: 2026-06-15
Task: MIMALLOC-COMPILER-LOWERING-OPTIMIZATION-CHECKPOINT-001
Scope: Checkpoint the mimalloc compiler-lowering optimization run after the
  CFG-stable receiver operand rewrite keeper closed the large body-timing gap.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-296x/296x-700-MIMALLOC-BODY-TIMING-NEXT-OWNER-SELECTION-001.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
---

# MIMALLOC-COMPILER-LOWERING-OPTIMIZATION-CHECKPOINT-001

## Purpose

296x-700 selected a pause rather than another compiler implementation owner:

```text
current_body_elapsed_ratio=1.865
fresh_gap_owner=hako_runtime_baseline
fresh_gap_confidence=low
selected_next_owner=pause_compiler_lowering_optimization
```

This row records the checkpoint and stop line for the current mimalloc
compiler-lowering optimization run.

## Required Output

```text
output_contract=hako-mimalloc-compiler-lowering-optimization-checkpoint-v0
source_evidence=296x-700
compiler_lowering_optimization_pause=1
receiver_operand_copy_chain_owner_closed=1
stable_body_elapsed_ratio=1.790
fresh_body_elapsed_ratio=1.865
winner_keeper=cfg_stable_dominance_guarded_receiver_operand_rewrite
next_compiler_owner_selected=0
startup_lane_reopened=0
source_hako_changed=0
summary=ok
```

## Stop Line

```text
do not continue patching call operands
do not reopen startup optimization
do not select compiler implementation work without fresh high-confidence owner
```

## Acceptance

```text
mimalloc_compiler_lowering_optimization_checkpoint_landed=1
source_evidence=296x-700
compiler_lowering_optimization_pause=1
summary=ok
```

## Result

```text
output_contract=hako-mimalloc-compiler-lowering-optimization-checkpoint-v0
source_evidence=296x-700
compiler_lowering_optimization_pause=1
receiver_operand_copy_chain_owner_closed=1
stable_body_elapsed_ratio=1.790
fresh_body_elapsed_ratio=1.865
winner_keeper=cfg_stable_dominance_guarded_receiver_operand_rewrite
next_compiler_owner_selected=0
startup_lane_reopened=0
source_hako_changed=0
summary=ok
```

Interpretation:

```text
The active compiler-lowering optimization run has a clean checkpoint. The
current body timing gap is too small and too mixed to justify another compiler
implementation row without a new high-confidence owner probe.
```
