---
Status: Current
Date: 2026-05-28
Scope: implement the selected direct cached-page known-live release keeper.
Blocker: RELEASE-DIRECT-CACHED-PAGE-KNOWN-LIVE-RELEASE-IMPLEMENTATION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-147-PAGE-ARRAY-KEEPER-SELECTION.md
---

# 296x-148 Release Direct Cached Page Known-Live Release Implementation

## Purpose

Apply one .hako page-array keeper by using `releaseLocalKnownLive` only on the
already-proven direct cached-page release path. Generic release fallback and
`releaseLocal` safety checks must remain unchanged.

## Required Output

```text
output_contract=release-direct-cached-page-known-live-release-implementation-v0
input_contract=page-array-keeper-selection-v0
keeper_applied
generic_release_fallback_preserved
exact_exe_proof_ok
expected_array_get_removed
summary=ok
```
