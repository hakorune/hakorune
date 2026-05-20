# 293x-967 MIMAP-351A Post Worker/TLS Pilot Row Selection

Status: landed
Date: 2026-05-21

## Decision

Select the next narrow allocator row after MIMAP-350A worker/TLS. The next row
must keep provider activation, host allocator replacement, hooks, and
`#[global_allocator]` inactive unless an explicit provider-facing ladder is
opened.

## Context

MIMAP-350A connected the MIMAP-349A OSVM/page-source report to the existing
internal worker/TLS cache seam and published bounded scalar worker/TLS facts.
The remaining large boundary is not source-level concurrency; it is the
provider/host integration ladder. That ladder remains closed by default.

## Candidate Next Rows

- provider activation inactive boundary inventory
- backend matcher no-growth closeout after worker/TLS
- allocator execution summary closeout pack before provider-facing work

## Stop Lines

- No provider activation, host allocator replacement, hooks, or
  `#[global_allocator]`.
- No source-level worker-local or concurrency surface.
- No backend `.inc` matcher by app, box, owner, or row name.
- No cross-function `Result` direct ABI or runtime sum materialization.

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

Selected MIMAP-352A provider inactive boundary inventory as the next narrow
allocator row after the worker/TLS pilot.
