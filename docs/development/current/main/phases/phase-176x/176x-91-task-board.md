# 176x-91 Task Board

Status: Landed
Date: 2026-04-12

## Goal

Land the first reachability-aware DCE widening slice.

## Tasks

- [x] lock the reachable-only DCE contract in docs
- [x] seed DCE liveness from reachable blocks only
- [x] keep backward liveness propagation reachable-only too
- [x] add focused unit guards for unreachable pure/effectful uses
- [x] keep existing reachable edge-arg guard green
- [x] keep quick gate green

## Exit

- reachability-aware DCE is landed without widening into full unreachable-block cleanup
