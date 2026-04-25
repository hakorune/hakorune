---
Status: Landed
Date: 2026-04-26
Scope: Rebaseline phase-291x after the CoreMethodContract `.inc` classifier baseline reached zero.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/README.md
  - docs/development/current/main/phases/phase-291x/291x-287-mir-call-maphas-sentinel-retirement-card.md
  - docs/development/current/main/design/stage2-hako-owner-vs-inc-thin-shim-ssot.md
  - lang/src/runtime/meta/README.md
  - lang/src/runtime/meta/mir_call_route_policy_box.hako
  - tools/checks/core_method_contract_inc_no_growth_guard.sh
---

# 291x-288 Post `.inc` Zero-Baseline Rebaseline Card

## Goal

Close the vague "next cleanup selection pending" state after `291x-287`.

This card is docs-only. It does not change compiler behavior, MIR metadata,
fixtures, smokes, or native shims.

## Current Truth

`291x-287` closed the CoreMethodContract `.inc` method/box classifier lane:

```text
core_method_contract_inc_no_growth_guard:
  classifiers = 0
  rows = 0
```

This means the guarded CoreMethodContract `.inc` mirror-pruning lane is closed.
It does not mean every compiler-clean task is done.

## Remaining Cleanup Shape

Do not reopen the old `.inc` mirror-prune rows unless a new owner-path change
creates fresh evidence. The former `291x-255` and `291x-275` inventories are
historical task-order records; they were superseded by the `291x-286` and
`291x-287` closeout work.

The next cleanup should move from residual `.inc` classifier deletion to owner
truth cleanup:

```text
current closed lane:
  CoreMethodContract guarded .inc method/box classifiers

next lane:
  runtime/meta mir_call route-policy owner audit
```

The first target is:

```text
lang/src/runtime/meta/mir_call_route_policy_box.hako
```

It still contains a broad route vocabulary keyed by `box_name`, `method_name`,
and `recv_family`. That may be either:

- an active `.hako` semantic table that should be made manifest/CoreMethod
  backed, or
- a stale transitional owner vocabulary that should be documented and retired
  carefully.

Do not delete or rename it in the first implementation slice. It is registered
from `lang/src/runtime/meta/hako_module.toml` and mentioned by runtime/meta docs
and design docs, so the first slice must establish whether it is still an
active owner path.

## Next Ordered Tasks

1. `291x-289` route-policy meta owner audit
   - Inventory references to `MirCallRoutePolicy`.
   - Determine whether generated/selfhost snapshots or module loading require
     the file.
   - Update `lang/src/runtime/meta/README.md` and the stage2 owner-vs-shim SSOT
     to state active vs transitional ownership.
   - No behavior change.

2. `291x-290` route-policy cleanup implementation
   - If active: narrow the table toward generated CoreMethod/manifest
     consumption instead of ad hoc by-name classification.
   - If stale: retire or quarantine it with module/snapshot-safe steps.
   - Keep `.inc` classifier baseline at zero.

3. Future cleanup selection
   - Only after the route-policy owner audit is closed.
   - Keep BoxShape cleanup separate from any CoreOp/LoweringTier feature work.

## Boundaries

- Do not reintroduce `.inc` method/box-name classifiers.
- Do not add hot lowering or perf probes in this lane.
- Do not mix Stage-B adapter thinning with runtime/meta route-policy cleanup.
- Do not treat string comparisons in unguarded helper/analyzer `.inc` files as
  equivalent to the now-closed CoreMethodContract mirror-pruning lane. They
  need their own owner audit and guard scope before deletion.

## Acceptance

```bash
git status -sb
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
```
