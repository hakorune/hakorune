# 204x-90 Control-Anchor Operand Liveness SSOT

Status: SSOT
Date: 2026-04-12
Owner: `phase-204x`

## Purpose

- land the first narrow code cut inside lane C2
- distinguish operand liveness from control-structure simplification

## Decision

- mainline DCE keeps values live when they are used by:
  - `Return.value`
  - `Branch.cond`
  - reachable edge args
- this ownership is explicit control-anchor operand seeding, not generic pure pruning

## Explicit Non-goals

- do not delete `Branch` / `Jump` / `Return`
- do not merge blocks
- do not fold branches or rewrite CFG
- do not mix this with `Debug` stripping

## Immediate Follow-on

1. `C2b` legacy in-instruction-list control-anchor seed cleanup
2. `C2c` simplification-handoff wording lock
