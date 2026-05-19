# 293x-900 MIMAP-297A Post Release-Applied Recycle Second-Release Diagnostic Row Selection

Status: landed
Date: 2026-05-20

## Decision

Select the next narrow allocator model row after MIMAP-296A second-release
diagnostic lands.

## Context

MIMAP-296A proves:

```text
modeled release-applied recycle row
  -> second release diagnostic attempt
  -> duplicate / stale reject
```

The next row should either close out this second-release diagnostic pack or
select the next scalar/model continuation. Keep raw pointer residence, real
arena backing release/recycle, segment-map mutation, atomics, OSVM/page-source,
providers, hooks, and `#[global_allocator]` closed unless a later row explicitly
opens them.

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Selected Row

`MIMAP-298A`:

```text
segment arena backing modeled allocation-ledger release-applied recycle second-release diagnostic closeout pack
```

Rationale:

- MIMAP-296A is a scalar/model diagnostic row with L3 evidence deferred.
- Closing the diagnostic immediately keeps the release-applied recycle
  duplicate/stale boundary from remaining as an open proof island.
- The closeout should bundle MIMAP-296A L2 with representative exact-MIR L3
  evidence while keeping lifecycle generation, real arena backing recycle,
  pointer residence, segment-map mutation, atomics, OSVM/page-source, providers,
  hooks, and `#[global_allocator]` closed.

Validation profile:

```text
closeout L3
```

## Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
