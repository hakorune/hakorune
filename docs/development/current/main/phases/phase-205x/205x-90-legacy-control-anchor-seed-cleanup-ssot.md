# 205x-90 Legacy Control-Anchor Seed Cleanup SSOT

Status: SSOT
Date: 2026-04-12
Owner: `phase-205x`

## Purpose

- close lane `C2b` after the `C2a` operand contracts are fixed
- remove legacy control-anchor seeding that no longer matches `BasicBlock` ownership

## Decision

- mainline DCE no longer seeds `Branch` / `Jump` / `Return` operands from `block.instructions`
- control-anchor operand liveness is now owned only by:
  - `block.terminator`
  - reachable edge args

## Why

- `BasicBlock::add_instruction` routes control instructions into `block.terminator`
- keeping a second instruction-list seed path would preserve stale legacy ownership
- this cut narrows seed ownership without changing control-shape behavior

## Explicit Non-goals

- do not delete `Branch` / `Jump` / `Return`
- do not change edge-arg liveness
- do not merge blocks or simplify CFG

## Immediate Follow-on

1. `C2c` simplification-handoff wording lock
2. return to the next layer step
