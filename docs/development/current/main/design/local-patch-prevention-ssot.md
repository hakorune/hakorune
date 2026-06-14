---
Status: SSOT
Scope: Prevent repeated local patches from bypassing compiler ownership
Related:
  - docs/development/current/main/design/compiler-expressivity-first-policy.md
  - docs/development/current/main/design/ai-handoff-and-debug-contract.md
  - docs/development/current/main/design/phi-lifecycle-ssot.md
  - docs/development/current/main/phases/phase-293x/293x-1021-COREPLAN-PHI-BINDING-SSOT-001.md
---

# Local Patch Prevention

This SSOT fixes the failure mode where an implementation agent repairs the
nearest symptom instead of restoring the compiler ownership boundary.

## Decision

Repeated local patches are a BoxShape signal, not an invitation to add more
special cases.

```text
same failure class + 2 local patches
  -> stop the line
  -> write or update the boundary SSOT first
  -> add a guard or fixture that makes the invariant machine-checkable
  -> only then resume implementation
```

## Definitions

```text
local patch:
  A change that repairs one observed failing route without changing the owner
  boundary that made the failure possible.

same failure class:
  The same invariant is violated, even if the file, route, fixture, or error
  text differs.

structural audit:
  A docs-first pass that names the truth owner, forbidden owners, fail-fast
  tag or guard, and acceptance command before more implementation changes.
```

Examples of same failure class:

```text
PHI input not available in predecessor
  same class as:
    early PHI dst exposure
    direct PHI construction outside lifecycle owner
    variable_map used as early PHI truth

undefined value after nested loop
  same class as:
    BindingState not propagated
    LocalSSA asked to repair logical freshness
    preheader freshness copying arbitrary values

RecipeOnly route falls back to whole body
  same class as:
    policy selected item-order lowering
    implementation retried route-level ExitAllowed lowering
```

## Stop-Line Rules

### Rule 1: Two-strike stop

If the same failure class gets two local patches, stop implementation work.

Required before resuming:

```text
1. Current-state or phase card records the failure class.
2. SSOT names the truth owner and forbidden owners.
3. A guard, fixture, or smoke pins the invariant.
4. The next code change is scoped to that owner boundary.
```

### Rule 2: Docs-first for BoxShape fixes

For BoxShape failures, do not start with lowering code.

Order:

```text
SSOT boundary
  -> guard / fixture acceptance
  -> owner-local implementation
  -> verification
```

### Rule 3: Diagnose and implement separately

When available, split the work into:

```text
diagnostic role:
  inventory, owner map, invariant, failure-class classification

implementation role:
  code change inside the selected owner boundary
```

One person or agent may do both roles, but the diagnostic artifact must exist
before the implementation patch.

### Rule 4: Make the bad patch physically hard

Prefer type, visibility, or guard boundaries over comments.

Examples:

```text
PHI:
  direct PHI construction outside lifecycle owners is guarded.

RecipeOnly:
  route-level whole-body fallback is guarded.

Preheader freshness:
  block-id / PHI-pred remap only; arbitrary value capture is guarded.
```

## Current Guard

The current first machine guard is:

```bash
bash tools/checks/coreplan_phi_binding_boundary_guard.sh
```

It pins the `COREPLAN-PHI-BINDING-SSOT-001` row:

```text
nested_loop_preheader_hidden_value_capture=0
recipe_only_whole_body_fallback=0
phi_direct_emit_no_growth=1
```

## Non-goals

```text
Do not require a new shell guard for every small bug.
Do not block normal one-shot bug fixes.
Do not turn docs-first into large design ceremony.
Do not mix BoxCount acceptance expansion with BoxShape cleanup.
```

Use this SSOT only when the repair pressure is structural:

```text
same invariant breaks again
same ownership boundary is patched from multiple files
diagnostics are insufficient to name the owner
the tempting fix is a fallback, special case, or by-route copy
```

