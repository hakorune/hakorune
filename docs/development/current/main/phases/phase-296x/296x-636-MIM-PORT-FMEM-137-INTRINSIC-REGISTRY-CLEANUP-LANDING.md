---
Status: Done
Date: 2026-06-08
Scope: MIM-PORT-FMEM-137.
Related:
  - docs/development/current/main/design/fastmem-verified-direct-default-retirement-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-630-MIM-PORT-FMEM-131-FASTMEM-DEDICATED-LOWERER-REMAINING-TASK-ORDER.md
  - src/mir/builder/fastmem/calls.rs
  - src/mir/builder/fastmem/tests/branch.rs
  - src/mir/builder/fastmem/tests/memops.rs
  - tools/hako_check/fastmem_source_syntax_smoke.sh
---

# 296x-636 MIM-PORT-FMEM-137 Intrinsic Registry Cleanup Landing

## Purpose

Record that fastmem `mem.*` call lowering now routes through a small intrinsic
registry descriptor instead of scattered string matching, while keeping all
intrinsic behavior and errors stable.

## Implementation

```text
fastmem intrinsic calls:
  look up intrinsic descriptor from registry
  validate arity from registry metadata
  lower through shared descriptor dispatch

inventory / check:
  fastmem_dedicated_method_call_lowering_count remains visible
  fastmem_forbidden_call_count remains visible
  intrinsic behavior remains unchanged
```

The intrinsic route still owns the same allowed vocabulary, but the hardcoded
string-match branches are now centralized behind a registry descriptor.

## Verification

```bash
cargo test -q fastmem_source --lib
bash tools/hako_check/fastmem_capability_inventory_smoke.sh
bash tools/hako_check/fastmem_check_smoke.sh
bash tools/checks/current_state_pointer_guard.sh
```

## Landed

```text
FastMemory intrinsic call lowering now uses a small registry descriptor,
keeping intrinsic behavior stable while removing scattered string matching.
```

## Closeout

```text
next: MIRBUILDER-FMEM-014 branch condition gate generalization
```
