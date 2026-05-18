# 293x-787 MIMAP-264A Segment Arena Backing Modeled Source Accounting Inventory

Status: selected current
Date: 2026-05-19

## Decision

Add scalar/model source-backed arena accounting after the modeled source bridge
closeout.

## Context

MIMAP-260A records modeled source bridge facts from an accepted modeled
arena-slot report, and MIMAP-262A closed the source bridge family. The next row
should account for source capacity and committed bytes in model space before
opening real arena backing allocation.

## Scope

- Observe accepted modeled source bridge reports.
- Publish scalar/model accounting facts for source capacity, committed bytes,
  uncommitted bytes, slot capacity, and slot padded bytes.
- Reject missing/rejected source bridge reports, invalid source token, invalid
  capacity/commit geometry, and closed-substrate requirements.
- Keep the row inventory-only.

## Stop Lines

- No real raw pointer residence.
- No pointer-derived lookup or dereference.
- No real arena backing allocation.
- No real segment-map mutation.
- No real segment allocation/free execution.
- No atomic bitmap execution.
- No OSVM/page-source execution.
- No TLS, worker-local, worker scheduling, or source-level concurrency.
- No provider activation, host allocator replacement, hooks, or
  `#[global_allocator]`.
- No cross-function `Result` direct ABI or runtime sum materialization.
- No backend `.inc` matcher by app or owner name.

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
