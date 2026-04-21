---
Status: Active
Date: 2026-04-22
Scope: phase-290x `ArrayBox` surface canonicalization の taskboard。docs-first で phase を固定し、その後の implementation cards を小さく切る。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/phases/phase-290x/README.md
  - docs/development/current/main/phases/phase-290x/290x-90-arraybox-surface-canonicalization-design-brief.md
  - docs/development/current/main/phases/phase-290x/290x-92-arraybox-surface-inventory-ledger.md
  - docs/development/current/main/phases/phase-137x/README.md
---

# Phase 290x ArrayBox Surface Task Board

## North Star

```text
surface contract
  -> one canonical catalog

execution dispatch
  -> one canonical invoke seam

exposure state
  -> explicit runtime/std/docs/smoke visibility
```

## Rules

- docs-first before refactor
- app lane remains primary
- phase-137x stays observe-only unless app work is actually blocked
- no “method added in 5 places by memory” workflow
- `apps/std/array.hako` is sugar, not the truth owner

## Docs Slice

| Card | Status | Goal |
| --- | --- | --- |
| `290x-0a` | done in this phase setup | create phase front + brief + taskboard + inventory ledger |
| `290x-0b` | done in this phase setup | update current/restart/workstream pointers to phase-290x |
| `290x-0c` | done in this phase setup | lock `length()` canonical / `size()` alias decision in docs |

## Implementation Slice Order

| Card | Status | Goal |
| --- | --- | --- |
| `290x-1a` | done | add `src/boxes/array/surface_catalog.rs` and define stable method rows for `length/size/get/set/push/pop/slice/remove/insert` |
| `290x-1b` | done | define `ArrayMethodId` and normalize canonical name/alias lookup through the catalog |
| `290x-1c` | done | add `ArrayBox::invoke_surface(...)` and route the first stable methods through it |
| `290x-1d` | done | convert thin consumers (`type_registry`, `method_resolution`, `effects_analyzer`) to read the catalog |
| `290x-1e` | done | thin VM dispatch (`boxes_array.rs`, `calls/method/dispatch.rs`) onto the invoke seam |
| `290x-1f` | done | add one stable Array surface smoke anchored on the catalog |

## Implementation Slice Snapshot

- landed code-side authoring point: `src/boxes/array/surface_catalog.rs`
- stable dispatch id: `ArrayMethodId`
- stable runtime seam: `ArrayBox::invoke_surface(...)`
- thin catalog readers now include:
  - `src/runtime/type_registry.rs`
  - `src/mir/builder/calls/method_resolution.rs`
  - `src/mir/builder/calls/effects_analyzer.rs`
  - `src/backend/mir_interpreter/handlers/boxes_array.rs`
  - `src/backend/mir_interpreter/handlers/calls/method.rs`
  - `src/backend/mir_interpreter/handlers/calls/method/dispatch.rs`
- `length` remains canonical; `size` is compatibility alias; `len` is retained as legacy slot alias, not a new canonical name
- stable smoke:
  - `tools/smokes/v2/profiles/integration/apps/phase290x_arraybox_surface_catalog_vm.sh`
  - covers catalog unit lock, `ArrayBox::invoke_surface(...)`, and VM visible-owner routes
  - `slice` return semantics are pinned through `invoke_surface`; direct source follow-up calls still lower through a `RuntimeDataBox` union receiver and are deferred as a separate return-type topic

## Current Stable Surface Target

The first cataloged stable methods are:

- `length`
- `size` (alias)
- `get`
- `set`
- `push`
- `pop`
- `slice`
- `remove`
- `insert`

## Explicitly Deferred

- `lastIndexOf(needle, start_pos)` two-arg runtime gap
- static-box receiver diagnostics cleanup
- wider collection API redesign
- non-cataloged extended methods (`clear/contains/indexOf/join/sort/reverse`)
- perf reopen work under phase-137x

## Exit Condition For Phase 290x

This phase is ready to close when:

1. catalog is the clear surface authoring point
2. dispatch truth no longer drifts across multiple hand-maintained lists
3. exposure state is explicit enough to distinguish:
   - implemented only
   - surfaced
   - smoke-pinned
4. app work no longer needs to rediscover ArrayBox truth by repo-wide search
