---
Status: Completed
Decision: accepted
Date: 2026-05-10
Scope: M66 allocator provider task breakdown.
Related:
  - docs/development/current/main/design/allocator-provider-current-task-breakdown-ssot.md
  - docs/development/current/main/design/allocator-provider-manifest-v0-ssot.md
  - tools/checks/k2_wide_allocator_provider_task_breakdown_guard.sh
---

# 293x-118 M66 Allocator Provider Task Breakdown

## Goal

Make the current allocator provider / replacement hook lane readable before
adding the provider manifest parser.

## Result

M66 adds a dedicated task-breakdown SSOT that:

- summarizes M52-M65 completion;
- separates lifecycle, `hako_alloc` policy/state, provider diagnostics, and
  native metal provider layers;
- reserves the M67-M75 task ladder;
- repeats the activation stop line;
- documents guard hygiene for past-card guards.

M66 also applies that guard-hygiene rule to two stale past-card guards:

- M33 no longer pins M34 as `next-card`;
- M51 no longer treats diagnostic-only allocator replacement wording as active
  process allocator replacement.

## Non-Goals

This card does not add:

- runtime provider parser;
- provider registry;
- provider selection;
- hook activation;
- process allocator replacement;
- `.inc` provider/hook name matching.

## Verification

```bash
bash tools/checks/k2_wide_allocator_provider_task_breakdown_guard.sh
bash tools/checks/k2_wide_allocator_provider_manifest_vocab_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
