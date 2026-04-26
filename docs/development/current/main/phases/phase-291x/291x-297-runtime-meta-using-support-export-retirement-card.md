---
Status: Landed
Date: 2026-04-26
Scope: Retire unused `UsingResolver` / `UsingDecision` runtime/meta support exports after owner audit.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-296-runtime-meta-using-support-owner-audit-card.md
  - lang/src/runtime/meta/README.md
  - lang/src/runtime/meta/hako_module.toml
  - src/runner/stage1_bridge/embedded_stage1_modules_snapshot.json
---

# 291x-297 Runtime/Meta Using Support Export Retirement Card

## Goal

Remove the stale `UsingResolver` / `UsingDecision` support exports after
`291x-296` showed that they are not active compiler owner paths.

This is a BoxShape cleanup slice. It does not change Stage1/Pipeline using
resolution behavior.

## Implementation

Removed:

```text
lang/src/runtime/meta/using_resolver.hako
lang/src/runtime/meta/using_decision.hako
lang/src/runtime/meta/hako_module.toml: UsingResolver / UsingDecision exports
```

Refreshed:

```text
src/runner/stage1_bridge/embedded_stage1_modules_snapshot.json
```

Kept intact:

```text
lang/src/runtime/meta/json_shape_parser.hako
lang/src/compiler/entry/using_resolver_box.hako
lang/src/compiler/pipeline_v2/using_resolver_box.hako
```

## Boundary

- Do not delete `JsonShapeToMap`; JoinIR bridge tests name its function
  directly.
- Do not touch Stage1/Pipeline using resolver boxes.
- Do not change module resolution behavior.
- Do not re-add placeholder meta resolver stubs under `runtime/meta`.

## Acceptance

```bash
bash tools/selfhost/refresh_stage1_module_env_snapshot.sh
bash tools/checks/module_registry_hygiene_guard.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
cargo test -q embedded_snapshot_matches_registry_doc
git diff --check
tools/checks/dev_gate.sh quick
```

## Next

Audit `JsonShapeToMap` separately. Keep it only if the JoinIR bridge / fixture
path is still an active owner path; otherwise move or retire it in a dedicated
slice.
