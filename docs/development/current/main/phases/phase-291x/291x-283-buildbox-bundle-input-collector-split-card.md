---
Status: Landed
Date: 2026-04-26
Scope: BuildBox bundle input collector split.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-281-buildbox-remaining-cleanup-order-card.md
  - lang/src/compiler/build/build_box.hako
  - lang/src/compiler/build/build_bundle_input_box.hako
  - lang/src/compiler/build/build_bundle_resolver_box.hako
---

# 291x-283: BuildBox Bundle Input Collector Split

## Goal

Keep `BuildBox` thin by moving bundle opts/env input collection into a
dedicated build-side input box.

The live resolver authority remains `BuildBundleResolverBox`. This card only
changes where the bundle input context is collected and normalized.

## Boundary

- `BuildBundleInputBox` owns:
  - bundle opts extraction
  - env alias/require input extraction
  - alias table parsing
  - require CSV parsing
  - bundle input presence check
- `BuildBundleResolverBox` owns:
  - duplicate validation at resolve time
  - require resolution
  - merged-prefix materialization
- `BuildBox` owns:
  - outer source-to-Program(JSON v0) sequencing
  - prepared scan-source result/error handoff
  - resolver invocation

## Non-Goals

- Do not change bundle CLI/env semantics.
- Do not reuse legacy `entry/bundle_resolver.hako`.
- Do not change parser fallback or Program(JSON v0) shape.
- Do not touch CoreMethodContract fallback rows.

## Acceptance

```bash
bash tools/smokes/v2/profiles/integration/stageb/stageb_bundle_duplicate_fail_vm.sh
SMOKES_ENABLE_STAGEB=1 bash tools/smokes/v2/profiles/integration/stageb/stageb_bundle_require_ok_vm.sh
bash tools/smokes/v2/profiles/integration/stageb/stageb_binop_vm.sh
HAKO_BUILD_TIMEOUT=20 bash tools/smokes/v2/profiles/quick/core/stageb_min_emit.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
```

Result: PASS.

Additional bundle coverage:

```bash
SMOKES_ENABLE_STAGEB=1 bash tools/smokes/v2/profiles/integration/stageb/stageb_bundle_require_fail_vm.sh
SMOKES_ENABLE_STAGEB=1 bash tools/smokes/v2/profiles/integration/stageb/stageb_bundle_require_multi_fail_vm.sh
SMOKES_ENABLE_STAGEB=1 bash tools/smokes/v2/profiles/integration/stageb/stageb_bundle_mix_emit_vm.sh
SMOKES_ENABLE_STAGEB=1 bash tools/smokes/v2/profiles/integration/stageb/stageb_bundle_alias_table_bad_vm.sh
```

Result: PASS.

Additional gate:

```bash
tools/checks/dev_gate.sh quick
```

Result: PASS.
