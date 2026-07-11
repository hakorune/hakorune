# 293x-914 CHECKS-ALLOCATOR-PROVIDER-SUBFAMILY-SPLIT Allocator Provider Subfamily Inventory Split

Status: landed
Date: 2026-05-20

## Decision

Split the allocator provider proof inventory into provider subfamily step files
while preserving the exact `allocator-wide` script order.

## Context

`293x-912` split the allocator-wide gate into top-level families. The provider
family remained the largest family inventory and mixed core provider surface,
activation, activation safety, activation decision, and diagnostics rows in one
file.

The allocator guard group already supports recursive `@include`, so provider
can be split without changing the public gate entry or execution semantics.

## Scope

- Keep `tools/checks/allocator/families/provider.steps` as the provider family
  entry.
- Split provider rows into:
  - `tools/checks/allocator/families/provider/core.steps`
  - `tools/checks/allocator/families/provider/activation.steps`
  - `tools/checks/allocator/families/provider/activation_safety.steps`
  - `tools/checks/allocator/families/provider/activation_decision.steps`
  - `tools/checks/allocator/families/provider/diagnostics.steps`
- Preserve exact `tools/checks/k2_wide_allocator_gate.sh --list` output.
- Update the check scripts index to document the provider subfamily owner.

## Non-Goals

- Do not migrate allocator-wide to `manifest_runner`.
- Do not change which guards allocator-wide runs.
- Do not change validation profiles or row cadence.
- Do not run the heavy allocator-wide gate as part of this cleanup.
- Do not change MIMAP-308A behavior.

## Required Evidence

```text
bash tools/checks/k2_wide_allocator_gate.sh --list
diff -u before-list after-list
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
