---
Status: Landed
Date: 2026-04-26
Scope: BuildBox bundle resolver BoxShape split.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-279-stageb-legacy-boundary-cleanup-card.md
  - lang/src/compiler/build/README.md
  - lang/src/compiler/build/build_box.hako
  - lang/src/compiler/build/build_bundle_resolver_box.hako
  - lang/src/compiler/hako_module.toml
---

# 291x-280: BuildBox Bundle Resolver Split

## Goal

Keep `BuildBox` as the live source-to-Program(JSON v0) authority while moving
bundle merge/require validation out of the BuildBox entry file.

The target shape is:

```text
BuildBox._prepare_scan_src(...)
  -> collect bundle/env inputs
  -> BuildBundleResolverBox.resolve(...)
  -> scan_src materialization
```

`BuildBox` keeps input collection and source-to-Program sequencing.
`BuildBundleResolverBox` owns only bundle merge, duplicate checks, require
checks, env alias lookup, and recursion guard.

## Non-Goals

- Do not change bundle CLI/env semantics.
- Do not reuse the legacy `entry/bundle_resolver.hako`; it remains a compat /
  JoinIR fixture surface.
- Do not change parser/body/defs/import injection authority.
- Do not reopen CoreMethodContract fallback rows.

## Acceptance

```bash
bash tools/smokes/v2/profiles/integration/stageb/stageb_bundle_duplicate_fail_vm.sh
SMOKES_ENABLE_STAGEB=1 bash tools/smokes/v2/profiles/integration/stageb/stageb_bundle_require_fail_vm.sh
SMOKES_ENABLE_STAGEB=1 bash tools/smokes/v2/profiles/integration/stageb/stageb_bundle_require_multi_fail_vm.sh
SMOKES_ENABLE_STAGEB=1 bash tools/smokes/v2/profiles/integration/stageb/stageb_bundle_require_ok_vm.sh
SMOKES_ENABLE_STAGEB=1 bash tools/smokes/v2/profiles/integration/stageb/stageb_bundle_mix_emit_vm.sh
bash tools/smokes/v2/profiles/integration/stageb/stageb_binop_vm.sh
HAKO_BUILD_TIMEOUT=20 bash tools/smokes/v2/profiles/quick/core/stageb_min_emit.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
```

Result: PASS.

Additional gate:

```bash
tools/checks/dev_gate.sh quick
```

Result: PASS.
