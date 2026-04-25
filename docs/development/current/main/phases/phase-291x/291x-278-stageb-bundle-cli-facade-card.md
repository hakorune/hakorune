---
Status: Landed
Date: 2026-04-26
Scope: Stage-B bundle CLI facade / BuildBox env handoff.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-276-stageb-buildbox-handoff-adapter-card.md
  - lang/src/compiler/entry/stageb_build_options_box.hako
  - lang/src/compiler/entry/stageb_compile_adapter_box.hako
  - lang/src/compiler/build/build_box.hako
---

# 291x-278: Stage-B Bundle CLI Facade

## Problem

`291x-276` made `compiler_stageb.hako` a BuildBox handoff adapter, but the
legacy Stage-B bundle CLI surface was still stranded in the old
`StageBBodyExtractorBox` path.

The stranded options are:

- `--bundle-src <code>`
- `--bundle-mod <Name:code>`
- `--require-mod <Name>`

## Decision

Keep these options as a Stage-B compat CLI facade, but do not put bundle
semantics back into `compiler_stageb.hako`.

```text
compiler_stageb.hako
  -> StageBBuildOptionsBox.apply_args(args)
  -> StageBCompileAdapterBox.emit_program_json_v0(...)
  -> BuildBox.emit_program_json_v0(source, null)
```

`StageBBuildOptionsBox` owns only CLI token packaging:

- `--bundle-src` -> existing `HAKO_BUNDLE_ALIAS_TABLE` env surface with a
  synthetic entry name
- `--bundle-mod` -> existing `HAKO_BUNDLE_ALIAS_TABLE` env surface
- `--require-mod` -> existing `HAKO_REQUIRE_MODS` env surface

No new environment variables are introduced. `BuildBox` remains the authority
for validation, duplicate checks, require checks, bundle/env alias merging, and
Program(JSON v0) generation.

## Acceptance

- Stage-B bundle CLI options reach BuildBox through its existing env contract.
- `compiler_stageb.hako` remains a thin driver; it does not parse bundle token
  details inline.
- Duplicate and missing bundle tags stay stable for the existing smoke surface.
- Stage-B binop and minimal emit remain green.

## Verification

```bash
bash tools/smokes/v2/profiles/integration/stageb/stageb_bundle_duplicate_fail_vm.sh
SMOKES_ENABLE_STAGEB=1 bash tools/smokes/v2/profiles/integration/stageb/stageb_bundle_require_fail_vm.sh
SMOKES_ENABLE_STAGEB=1 bash tools/smokes/v2/profiles/integration/stageb/stageb_bundle_require_multi_fail_vm.sh
SMOKES_ENABLE_STAGEB=1 bash tools/smokes/v2/profiles/integration/stageb/stageb_bundle_require_ok_vm.sh
bash tools/smokes/v2/profiles/integration/stageb/stageb_binop_vm.sh
HAKO_BUILD_TIMEOUT=20 bash tools/smokes/v2/profiles/quick/core/stageb_min_emit.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
```

Result: PASS.

## Notes

- The first attempt passed a MapBox opts object across the Stage-B adapter
  boundary. VM-Hako treated that path poorly (`String.get`), so this card uses
  BuildBox's existing env contract instead.
- No new env variables were added.
- The bundle smoke scripts now use the same non-strict JoinIR settings as the
  Stage-B helper path, because these scripts are CLI compatibility checks, not
  planner-required JoinIR acceptance checks.
