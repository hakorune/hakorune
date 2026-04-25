---
Status: Landed
Date: 2026-04-26
Scope: Retire the unused `MirCallNeedPolicy` runtime/meta export after owner audit.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-291-mir-call-need-policy-owner-audit-card.md
  - docs/development/current/main/design/stage2-hako-owner-vs-inc-thin-shim-ssot.md
  - lang/src/runtime/meta/README.md
  - lang/src/runtime/meta/hako_module.toml
  - lang/c-abi/shims/README.md
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_need_policy.inc
  - src/runner/stage1_bridge/embedded_stage1_modules_snapshot.json
---

# 291x-292 MIR-Call Need Policy Export Retirement Card

## Goal

Remove the stale `MirCallNeedPolicy` `.hako` export after `291x-291` showed
that it is not an active executable owner path.

This is a BoxShape cleanup slice. It does not change native need-policy
lowering, declaration emission, or `mir_call` behavior.

## Implementation

Removed:

```text
lang/src/runtime/meta/mir_call_need_policy_box.hako
lang/src/runtime/meta/hako_module.toml: MirCallNeedPolicy export
```

Refreshed:

```text
src/runner/stage1_bridge/embedded_stage1_modules_snapshot.json
```

Kept intact:

```text
lang/c-abi/shims/hako_llvmc_ffi_mir_call_need_policy.inc
```

The native need-policy consumer remains the current executable owner until a
generated manifest or typed LoweringPlan owns the need flags and feeds the
prepass.

## Boundary

- Do not re-add a `.hako` by-name need table.
- Do not treat this as native shim thinning; the native need consumer is still
  required.
- Do not merge this with `MirCallSurfacePolicy`; that table still needs a
  separate owner audit.
- Do not add any `.inc` method/box-name classifier rows.

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

Open the next owner audit:

```text
MirCallSurfacePolicy owner audit
```

Do not delete it by analogy; verify active `.hako`/Rust/native consumer paths
first.
