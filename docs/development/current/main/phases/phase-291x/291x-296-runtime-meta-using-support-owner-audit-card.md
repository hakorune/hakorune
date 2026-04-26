---
Status: Landed
Date: 2026-04-26
Scope: Audit `UsingResolver` / `UsingDecision` runtime/meta support exports before changing module exports.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-295-runtime-meta-live-table-inventory-card.md
  - lang/src/runtime/meta/README.md
  - lang/src/runtime/meta/hako_module.toml
  - lang/src/runtime/meta/using_resolver.hako
  - lang/src/runtime/meta/using_decision.hako
  - src/runner/stage1_bridge/embedded_stage1_modules_snapshot.json
---

# 291x-296 Runtime/Meta Using Support Owner Audit Card

## Goal

Decide whether `lang/src/runtime/meta/using_resolver.hako` and
`lang/src/runtime/meta/using_decision.hako` are active owner paths or stale
support vocabulary before export cleanup.

This card is an owner audit. It does not change module exports, snapshots, or
compiler behavior.

## Evidence

Repository search found no external active caller of:

```text
selfhost.meta.UsingResolver
selfhost.meta.UsingDecision
UsingDecision.decide(...)
```

The only active code reference to `selfhost.meta.UsingResolver` is internal to
the support pair:

```text
lang/src/runtime/meta/using_decision.hako
```

Other `UsingResolver` search hits are Stage1 / Pipeline using resolver boxes or
comments. They are separate compiler paths:

```text
lang/src/compiler/entry/using_resolver_box.hako
lang/src/compiler/pipeline_v2/using_resolver_box.hako
```

The current non-doc references to the support exports are:

```text
lang/src/runtime/meta/hako_module.toml
src/runner/stage1_bridge/embedded_stage1_modules_snapshot.json
lang/src/runtime/meta/using_resolver.hako
lang/src/runtime/meta/using_decision.hako
```

## Decision

`UsingResolver` and `UsingDecision` under `runtime/meta` are registered
transitional support utilities, not active compiler owner paths.

They may remain registered only as short-lived cleanup debt. Do not route
Stage1/Pipeline using resolution through these stubs; those paths already have
their own owners.

## Next Implementation

Open `291x-297` as the implementation slice:

```text
UsingResolver / UsingDecision export quarantine / retirement
```

Preferred order:

1. Remove both stale exports if no caller is introduced.
2. Delete both support files together; `UsingDecision` depends on
   `UsingResolver`.
3. Refresh `src/runner/stage1_bridge/embedded_stage1_modules_snapshot.json`.
4. Keep `JsonShapeToMap` separate because JoinIR bridge tests name its
   function directly.
5. Run module-registry and current-state guards.

If removal exposes a real consumer, stop and move the pair to the appropriate
support namespace instead of preserving them as compiler semantic tables.

## Boundaries

- Do not delete `JsonShapeToMap` in this slice.
- Do not touch Stage1/Pipeline using resolver boxes.
- Do not change module resolution behavior.
- Do not treat comments mentioning Stage1 `UsingResolver` as evidence for this
  `selfhost.meta` support pair.

## Acceptance

```bash
rg -n "selfhost\\.meta\\.UsingResolver|selfhost\\.meta\\.UsingDecision|using_resolver\\.hako|using_decision\\.hako|UsingDecision\\b|UsingResolver\\b" lang src tools crates apps --glob '!target/**' --glob '!*.json'
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
```
