# 182x-91: task board

Status: Landed

## Goal

Land unreachable block pruning as the next CFG-cleanup slice after the current DCE liveness cuts.

## Tasks

- [x] document the pruning contract and non-goals
- [x] add a MIR-side unreachable block pruning helper
- [x] wire DCE to prune unreachable blocks after liveness elimination
- [x] add focused regression coverage for unreachable block deletion
- [x] update current pointers and phase indexes

## Exit

- unreachable block pruning is landed
- broader effect-sensitive / no-dst DCE remains backlog
