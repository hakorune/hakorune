---
Status: SSOT
Decision: accepted
Date: 2026-05-10
Scope: `phase-293x` allocator replacement hook boundary before any process allocator replacement implementation.
Related:
  - docs/development/current/main/design/hako-alloc-policy-state-contract-ssot.md
  - docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md
  - docs/development/current/main/phases/phase-293x/293x-097-M45-PRODUCTION-ALLOCATOR-PORT-ENTRY-PLAN.md
  - docs/development/current/main/phases/phase-293x/293x-103-M51-PRODUCTION-ALLOCATOR-PORT-CLOSEOUT-GUARD.md
  - lang/src/hako_alloc/README.md
---

# Allocator Replacement Hook Boundary (SSOT)

## Goal

Fix the boundary for a future allocator replacement hook without activating the
hook. M52 is a docs/guard row only.

The problem this SSOT prevents is a broad shortcut where `.hako`, `.inc`, Rust
runtime code, or an environment variable independently decides to replace the
process allocator.

## Decision

Allocator replacement is allowed only as a future named row after this boundary.

Current accepted state:

```text
hako_alloc:
  owns allocator policy/control shape

runtime/substrate:
  owns raw capability facades

runtime/kernel/native metal:
  owns final allocation bodies and hook install mechanics

MIR/manifest/HookPlan:
  will own backend-readable hook facts when a future row activates them

.inc:
  reads emitted facts only
  does not infer allocator replacement from names
```

The hook is not active in M52.

## Hook Meaning

Future "allocator replacement hook" means an explicit runtime/kernel seam that
can route allocation requests through a verified allocator policy and substrate
body.

It does not mean:

```text
#[global_allocator] is added now.
HakoAllocProductionFacade installs itself.
apps opt in by name.
.inc matches HakoAlloc* names.
an environment variable silently swaps allocators.
pointer fetch_add becomes active.
OSVM unreserve/release becomes active.
native pointer attrs become active.
```

## Owner Split

### `hako_alloc`

Owns:

- size-class and page policy;
- local page allocation/free policy;
- remote-free policy decisions;
- page-source policy decisions;
- facade-level accounting and validation.

Does not own:

- process-global hook install;
- direct libc/syscall allocator bodies;
- native pointer attrs;
- backend route emission.

### Runtime / Kernel / Native Metal

Owns:

- final allocation/free/realloc bodies;
- hook install/uninstall mechanics in a future explicit row;
- bootstrap allocation needed before any hook is active;
- reentrancy guard mechanics in a future explicit row.

Does not own:

- allocator policy duplication;
- app/facade-name routing;
- hidden environment-variable opt-in.

### MIR / Manifest / HookPlan

Future active rows must put backend-readable facts here first.

Minimum future vocabulary:

```text
HookPlan:
  hook_id
  entrypoints
  allocation_classes
  owned_policy_box
  substrate_routes
  reentrancy_mode
  no_alloc/no_safepoint requirements
  fail_fast_diagnostic
```

The backend must consume facts, not source names.

### `.inc`

Allowed:

- emit already-decided runtime calls or link declarations;
- fail-fast when a required HookPlan fact is absent.

Forbidden:

- matching `HakoAllocProductionFacade`;
- matching allocator app names;
- discovering hook policy by scanning `.hako`;
- deciding process allocator ownership.

## Stop Line

M52 keeps these inactive:

- process allocator replacement;
- `#[global_allocator]`;
- `hako_alloc` hook install function;
- backend allocator replacement route;
- hidden allocator hook environment variable;
- pointer `fetch_add`;
- OSVM unreserve/release;
- noalias / nonnull / dereferenceable widening.

Any future row that needs one of these must name the row, fixture/guard,
diagnostic, and rollback condition.

## Future Order

Recommended next rows:

1. `M53 allocator HookPlan vocabulary lock`
   - docs/manifest vocabulary only;
   - no runtime hook body.
2. `M54 allocator hook runtime dry-run guard`
   - runtime/kernel hook shape exists but cannot replace the process allocator;
   - diagnostics only, default inactive.
3. `M55 allocator hook activation proof`
   - explicit opt-in gate;
   - verified HookPlan fact required;
   - no `.inc` name inference.

This order may be split further, but it must not be collapsed into a single
allocator replacement implementation commit.

## Gate

```bash
bash tools/checks/k2_wide_allocator_replacement_hook_boundary_guard.sh
bash tools/checks/k2_wide_production_allocator_port_closeout_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
