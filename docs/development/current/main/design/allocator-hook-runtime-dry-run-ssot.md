---
Status: SSOT
Decision: accepted
Date: 2026-05-10
Scope: allocator hook runtime dry-run boundary before any process allocator replacement.
Related:
  - docs/development/current/main/design/allocator-replacement-hook-boundary-ssot.md
  - docs/development/current/main/design/allocator-hook-plan-v0-ssot.md
  - docs/development/current/main/design/allocator-hook-plan-v0.toml
---

# Allocator Hook Runtime Dry-Run Boundary (SSOT)

## Goal

Define the next runtime/kernel seam without activating allocator replacement.

M54 is a design/guard row. It keeps runtime hook work in a dry-run shape until a
future row proves activation.

## Decision

A future runtime dry-run hook may exist only as an explicit, default-inactive
diagnostic seam.

Current M54 state:

```text
HookPlan vocabulary:
  reserved

runtime hook install:
  absent

process allocator replacement:
  absent

environment toggle:
  absent

.inc hook inference:
  absent
```

## Dry-Run Meaning

Future dry-run means:

```text
runtime/kernel can validate a HookPlan-shaped request
runtime/kernel can report what would be installed
runtime/kernel must not replace malloc/free/realloc
runtime/kernel must not become allocator policy owner
```

It does not mean:

```text
#[global_allocator]
process allocator replacement
hidden opt-in environment variable
.inc name matching
app/facade/policy-name routing
native pointer attr widening
```

## Required Future Dry-Run Shape

When implemented, the dry-run seam must:

- require an explicit HookPlan fact;
- return a diagnostic-only result;
- fail-fast with `[allocator-hook/dry-run-missing-plan]` when the fact is absent;
- be default inactive;
- avoid environment-variable opt-in until a separate documented row names it;
- avoid any `#[global_allocator]` or process allocator replacement.

## Owner Split

### Runtime / Kernel

Owns future dry-run validation mechanics and diagnostics.

Does not own allocator policy.

### `hako_alloc`

Owns allocator policy and facade state.

Does not install hooks.

### MIR / Manifest

Owns future HookPlan facts.

### `.inc`

May emit only facts decided elsewhere. It must not infer dry-run eligibility
from names.

## Stop Line

M54 keeps these inactive:

- runtime hook install/uninstall body;
- process allocator replacement;
- `#[global_allocator]`;
- hook environment toggles;
- `.inc` hook/facade/policy name matching;
- pointer `fetch_add`;
- OSVM unreserve/release;
- native pointer attr widening.

## Gate

```bash
bash tools/checks/k2_wide_allocator_hook_runtime_dry_run_guard.sh
bash tools/checks/k2_wide_allocator_hook_plan_vocab_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
