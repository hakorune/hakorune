---
Status: Complete
Date: 2026-05-12
Scope: M187 exact usize for size-class policy.
Related:
  - lang/src/hako_alloc/memory/size_class_box.hako
  - apps/mimalloc-size-class-usize-policy-proof/main.hako
  - docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md
---

# 293x-198 M187 Size-Class usize Policy

## Goal

Add `usize` input facades to `SizeClassBox` while preserving the existing
signed sentinel returns.

M187 adds:

```text
size_to_bin_usize(size: usize)
good_size_usize(size: usize)
bin_size_usize(bin: usize)
accepts_usize(size: usize)
```

## Stop Line

M187 does not migrate stored fields, change `HakoAllocPageModel`, change
page-map/realloc/aligned/huge owners, or reinterpret oversized failure
sentinels as `usize`.

`good_size_usize(...)` and `bin_size_usize(...)` still return `-1` for invalid
or oversized cases through the existing signed result lane.

## Proof

```bash
bash tools/checks/k2_wide_mimalloc_size_class_usize_policy_guard.sh
bash tools/checks/k2_wide_mimalloc_size_class_policy_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
