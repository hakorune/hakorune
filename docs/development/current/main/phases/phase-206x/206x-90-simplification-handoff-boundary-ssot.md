# 206x-90 Simplification Handoff Boundary SSOT

Status: SSOT
Date: 2026-04-12
Owner: `phase-206x`

## Purpose

- close lane `C2c` after the control-anchor operand contracts and legacy seed cleanup are landed
- keep the DCE / SimplifyCFG boundary explicit so structural control cleanup does not drift back into mainline DCE

## Decision

- mainline DCE owns:
  - liveness of control-anchor operands
  - seed ownership through `block.terminator` and reachable edge args
- later simplification bundle owns:
  - terminator deletion
  - block merge
  - branch/jump reshaping

## Why

- DCE is the liveness owner, not the CFG-rewrite owner
- pushing structural rewrites into the same lane would blur the handoff to `SimplifyCFG` / jump-threading

## Explicit Non-goals

- do not delete `Branch` / `Jump` / `Return`
- do not merge blocks
- do not rewrite terminators
- do not add `Debug` policy changes

## Immediate Follow-on

1. return to the next layer step
