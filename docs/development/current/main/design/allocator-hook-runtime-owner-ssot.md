---
Status: SSOT
Decision: accepted
Date: 2026-05-10
Scope: allocator hook runtime owner row before dry-run implementation.
Related:
  - docs/development/current/main/design/allocator-hook-runtime-dry-run-ssot.md
  - docs/development/current/main/design/allocator-hook-activation-proof-ssot.md
---

# Allocator Hook Runtime Owner (SSOT)

## Goal

Name the runtime owner before any allocator hook dry-run code is added.

M56 is a docs/guard owner row. It keeps implementation absent.

## Decision

Future allocator hook dry-run mechanics belong to a runtime-owned seam, not to
`hako_alloc`, `.inc`, apps, or environment variables.

Reserved future owner:

```text
src/runtime/allocator_hook_dry_run.rs
```

Reserved future API shape:

```text
validate_allocator_hook_dry_run(plan, proof) -> diagnostic result
```

Current M56 state:

```text
runtime owner:
  named

runtime dry-run code:
  diagnostic-only in src/runtime/allocator_hook_dry_run.rs

hook activation:
  absent

process allocator replacement:
  absent
```

## Owner Rules

### Runtime Owner

May own future dry-run validation mechanics:

- HookPlan presence check;
- activation proof presence check;
- diagnostic result construction;
- fail-fast tag ownership for missing plan/proof.

Must not own:

- allocator policy;
- `.hako` facade matching;
- process allocator replacement;
- environment variable opt-in.

### `hako_alloc`

Keeps allocator policy/facade state only.

### `.inc`

Must not become the runtime owner. It may only emit facts decided elsewhere in a
future active row.

## Stop Line

M56/M57 keep these inactive:

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
bash tools/checks/k2_wide_allocator_hook_runtime_owner_guard.sh
bash tools/checks/k2_wide_allocator_hook_activation_proof_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
