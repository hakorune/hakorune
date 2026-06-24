---
Status: Landed
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
lightweight_exact_exe_proof_ok
expected_array_get_removed
summary=ok
```

## Evidence

```text
output_contract=release-direct-cached-page-known-live-release-implementation-v0
input_contract=page-array-keeper-selection-v0
proof_scope=lightweight_known_live_release_smoke
keeper_applied=1
generic_release_fallback_preserved=1
lightweight_exact_exe_proof_ok=1
allocation_count=64
free_count=64
release_known_page_fast_path_count=64
release_known_page_fallback_count=0
expected_array_get_removed_at_full_repeat=524288
full_repeat_measurement_executed=0
winner_claim=0
replacement_active=0
selected_next=post_known_live_release_measurement
summary=ok
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_release_direct_cached_page_known_live_release_implementation_guard.sh
```
