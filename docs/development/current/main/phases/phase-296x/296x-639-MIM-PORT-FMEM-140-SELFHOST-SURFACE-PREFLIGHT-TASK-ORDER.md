---
Status: Done
Date: 2026-06-08
Scope: MIM-PORT-FMEM-140.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/workstreams/mimalloc-current.md
  - docs/development/current/main/design/fastmem-verified-direct-default-retirement-ssot.md
  - docs/development/current/main/design/stage0-stage1-feature-responsibility-split-ssot.md
  - docs/development/current/main/design/language-minimal-surface-ssot.md
  - src/mir/builder/exprs.rs
  - src/tests/mir_outbox_contract.rs
---

# 296x-639 MIM-PORT-FMEM-140 Selfhost Surface Preflight Task Order

## Purpose

Record the post-review task order before opening more implementation work.
The review correctly separates the remaining selfhost-preflight surface gaps
from broad language features that should stay docs/check-only before
selfhosting.

The FastMemory dedicated lowerer part of the review was based on an older
state. In the current tree, `MIRBUILDER-FMEM-000..015` has already landed
through `296x-638`; the transitional lowerer is now a thin region-entry and
obligation shell. Do not reopen that lane unless new evidence appears.

## Decision

```text
before selfhost, implement:
  SELFHOST-SURFACE-000:
    selfhost-surface-check for pending / transport / deferred surface use

  OUTBOX-0:
    narrow outbox lowering for parser-accepted outbox declarations

before selfhost, do not implement:
  uses broad checker
  requires / ensures / invariant runtime insertion
  transition legality checker
  broad type alias semantics
  PackedArray<T> full lowering
  fixed-width numeric runtime lanes
  Channel / task_scope / lock / source worker_local
  unsafe {}

fastmem lowerer status:
  MIRBUILDER-FMEM-000..015 landed through 296x-638
  do not restart lowerer retirement from the older f3c878 review snapshot
```

## SELFHOST-SURFACE-000

Add a check/report row that prevents Stage1 selfhost sources from relying on
pending, transport-only, deferred, or prohibited semantics.

```text
shape:
  report/check only
  no new runtime behavior
  no new broad parser acceptance
  no pending surface promotion by implication

report/check fields:
  selfhost_pending_surface_use_count=0
  selfhost_transport_surface_semantic_use_count=0
  selfhost_guarded_surface_use_count
  selfhost_forbidden_surface_use_count=0

acceptance:
  live / guarded rows are visible
  pending / transport-only semantic use fails fast
  deferred / prohibited syntax remains fail-fast
```

## OUTBOX-0

Close the current parser-accepted / MIR-missing gap for `outbox` without
opening a rich ownership checker.

```text
shape:
  outbox x[, y]
    -> local Void binding
    -> function metadata outbox binding evidence
    -> return transfer-intent metadata when returned

non_goals:
  outbox x = expr
  Rust-style moved-state checker
  broad escape analysis
  fini / lifecycle rewrite
  weak/outbox unification

acceptance:
  outbox_lowering=1
  outbox_binding_count>0
  outbox_init_expr_supported=0
  outbox_transfer_return_metadata=1
  outbox_rich_move_checker=0
```

## Order

```text
1. SELFHOST-SURFACE-000
   reason:
     make forbidden/pending Stage1 surface use observable before adding more
     lowering behavior

2. OUTBOX-0
   reason:
     close the one known parser-accepted / MIR-missing surface with a narrow
     metadata-bearing binding

3. Post-outbox selfhost gate refresh
   reason:
     re-run the selfhost profile with outbox no longer expected to freeze as
     unimplemented

4. Return to mimalloc FastMemory body work or AtomicRemoteHead lane
   reason:
     only after the selfhost surface contract is explicit
```

## Verification

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Closeout

```text
next: SELFHOST-SURFACE-000 selfhost surface check
```
