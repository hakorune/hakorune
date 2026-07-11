---
Status: Landed
Date: 2026-04-26
Scope: JoinIR route detector unused legacy module export inventory
Related:
  - src/mir/loop_route_detection/mod.rs
  - src/mir/loop_route_detection/legacy/mod.rs
  - src/mir/loop_route_detection/legacy/loop_body_carrier_promoter.rs
  - src/mir/loop_route_detection/legacy/loop_body_cond_promoter.rs
  - src/mir/loop_route_detection/legacy/loop_body_digitpos_promoter.rs
---

# 291x-357: JoinIR Route Detector Unused Module Export Inventory

## Goal

Inventory which legacy modules still need parent-module compatibility exports
from `crate::mir::loop_route_detection`.

This is BoxShape-only. Do not prune modules or migrate imports in this card.

## External Caller Modules

These parent-module exports still have non-legacy external callers:

```text
break_condition_analyzer
function_scope_capture
loop_body_carrier_promoter
loop_body_cond_promoter
loop_condition_scope
mutable_accumulator_analyzer
pinned_local_analyzer
trim_loop_helper
```

Keep those exports for now.

## Legacy-Internal Only Modules

These parent-module exports have no non-legacy external callers in the searched
paths:

```text
condition_var_analyzer
loop_body_digitpos_promoter
digitpos_detector
trim_detector
```

Observed internal ownership:

```text
loop_condition_scope -> super::condition_var_analyzer
loop_body_cond_promoter -> loop_body_digitpos_promoter
loop_body_digitpos_promoter -> digitpos_detector
loop_body_carrier_promoter -> trim_detector
```

`condition_var_analyzer` already uses a legacy-internal `super::` path from its
caller. The other three still have parent-module import paths inside legacy
code.

## Decision

Prune only after legacy-internal imports use owner-local paths instead of the
parent compatibility surface.

That keeps the parent module as external compatibility surface only, not an
internal routing bus for legacy modules.

## Next Cleanup

Migrate these legacy-internal imports:

```text
crate::mir::loop_route_detection::loop_body_digitpos_promoter
crate::mir::loop_route_detection::digitpos_detector
crate::mir::loop_route_detection::trim_detector
```

to legacy-owner-local paths.

Do not prune parent exports in the same card.

## Non-Goals

- No parent module export deletion.
- No detector logic change.
- No route classifier behavior change.
- No caller migration outside legacy internals.

## Validation

```bash
cargo check -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
