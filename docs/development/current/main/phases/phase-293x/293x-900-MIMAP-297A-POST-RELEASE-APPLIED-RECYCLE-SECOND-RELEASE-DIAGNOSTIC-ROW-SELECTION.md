# 293x-900 MIMAP-297A Post Release-Applied Recycle Second-Release Diagnostic Row Selection

Status: selected current
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

Pending: choose the next narrow allocator row after MIMAP-296A.
