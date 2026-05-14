---
Status: Landed
Date: 2026-05-14
Row: MIMAP-001
Scope: upstream mimalloc source pin for Hakorune blueprint work.
Related:
  - docs/development/current/main/design/mimalloc-hakorune-blueprint-task-breakdown-ssot.md
  - docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md
  - docs/development/current/main/investigations/mimalloc-upstream-pin.md
---

# MIMAP-001 Upstream Source Pin

## Summary

Pinned upstream mimalloc as an untracked local reference tree and recorded the
source-reading boundary for blueprint work.

## Outputs

- Added `.external/` to `.gitignore`.
- Checked out upstream mimalloc under `.external/upstream/mimalloc/`.
- Recorded the upstream URL, commit, describe value, license summary, local path,
  and initial source inventory window in
  `docs/development/current/main/investigations/mimalloc-upstream-pin.md`.

## Pin Snapshot

```text
upstream: https://github.com/microsoft/mimalloc.git
local: .external/upstream/mimalloc/
commit: fef6b0dd70f9d7fa0750b0d0b9fbb471203b94cd
describe: fef6b0d
commit date: 2026-04-29 15:46:57 -0700
license: MIT-style upstream license
```

## Stop Lines Kept

```text
no vendored mimalloc source
no copied C source in docs
no executable allocator behavior
no provider activation
no hooks / global allocator replacement
```

## Next

`MIMAP-002 source concept inventory` should read bounded upstream concept
families and classify each as near-transcription, lifecycle-rewrite,
substrate-gap, representation-gap, or deferred-unsafe.
