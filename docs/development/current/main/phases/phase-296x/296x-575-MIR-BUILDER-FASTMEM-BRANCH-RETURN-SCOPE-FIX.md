---
Status: Done
Date: 2026-06-07
Scope: MIRBuilder FastMemory branch-local return acceptance.
Related:
  - docs/development/current/main/phases/phase-296x/296x-574-MIM-PORT-FMEM-076-PAGE-LOCAL-ALLOC-ROUTE-CFG-PREFLIGHT.md
  - lang/src/hako_alloc/memory/page_meta_page_local_alloc_route_cfg_preflight_box.hako
  - src/mir/builder/vars/lexical_scope.rs
---

# 296x-575 MIRBuilder FastMemory Branch Return Scope Fix

## Purpose

Accept a narrow FastMemory branch shape where each branch exits with `return`,
without violating the MIRBuilder lexical-scope stack contract.

This is a builder acceptance fix, not a FastMemory lowering row.

## Observed Failure

The following source shape currently freezes during MIR build:

```hako
fastmem PageMapV0 {
    if same_owner {
        return local_result
    } else {
        return free_result
    }
}
```

Observed error:

```text
[freeze:contract][lexical_scope/unbalanced_pop]
```

The same route can be represented today by assigning inside the branch and
returning once after the branch. MIM-PORT-FMEM-076 uses that smaller shape so
the route-CFG preflight can stay focused.

## Required Boundaries

```text
MIRBuilder / lexical scope fix only
no new FastMemory MemOp
no new LLVM lowering
no allocation route execution claim
no page-local alloc route producer pilot
no LayoutRef phi/join rule unless the failing shape requires it explicitly
no product activation / hook / global allocator / winner claim change
```

## Acceptance Sketch

```text
fixture with branch-local return builds MIR without lexical_scope/unbalanced_pop
existing branch CFG fixture still builds
MIM-PORT-FMEM-076 preflight fixture still builds
no new report claims are opened by this fix
fastmem source syntax smoke remains green
current state pointer guard passes
git diff --check passes
```

## Result

The current MIRBuilder accepts the narrow branch-local return shape without a
code-side lexical-scope change. This card pins that behavior with a dedicated
`hako_alloc` fixture and source-syntax smoke coverage so later FastMemory route
work cannot regress back to `lexical_scope/unbalanced_pop`.

## Verification

```bash
bash tools/hako_check/fastmem_source_syntax_smoke.sh
bash tools/hako_check/fastmem_check_smoke.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Non-goals

```text
opening page-local allocation route CFG lowering
implementing path-sensitive allocation branch execution
retiring the diagnostic Python-template C bridge
changing FastMemory report/check producer sequencing
```
