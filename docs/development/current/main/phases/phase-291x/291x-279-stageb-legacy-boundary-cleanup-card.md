---
Status: Landed
Date: 2026-04-26
Scope: Stage-B legacy boundary docs / bundle smoke helper cleanup.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-278-stageb-bundle-cli-facade-card.md
  - lang/src/compiler/build/README.md
  - lang/src/compiler/build/build_box.hako
  - lang/src/compiler/entry/stageb_body_extractor_box.hako
  - lang/src/compiler/entry/bundle_resolver.hako
  - tools/smokes/v2/lib/stageb_helpers.sh
---

# 291x-279: Stage-B Legacy Boundary Cleanup

## Goal

Make the post-`291x-278` Stage-B boundary readable before more cleanup.

The live Stage-B path is:

```text
compiler_stageb.hako
  -> StageBBuildOptionsBox.apply_args(args)
  -> StageBCompileAdapterBox.emit_program_json_v0(...)
  -> BuildBox.emit_program_json_v0(source, null)
```

`StageBBodyExtractorBox` and `BundleResolver` still exist, but they are
legacy compat / JoinIR fixture surfaces. They are not the live source-to-Program
authority.

## Ordered Cleanup

1. Update `CURRENT_TASK.md` with this short cleanup queue.
2. Update BuildBox docs so the owner-local bundle helper shape matches the
   implementation.
3. Mark `StageBBodyExtractorBox` / `BundleResolver` as legacy compat and
   JoinIR fixture surfaces.
4. Move repeated Stage-B bundle smoke VM environment setup into the shared
   Stage-B smoke helper.

## Non-Goals

- Do not delete `StageBBodyExtractorBox`; Rust JoinIR tests still reference it.
- Do not change bundle semantics.
- Do not reopen the residual `MapBox.has` fallback baseline.
- Do not add new environment variables.

## Acceptance

```bash
git status -sb
bash tools/smokes/v2/profiles/integration/stageb/stageb_bundle_duplicate_fail_vm.sh
SMOKES_ENABLE_STAGEB=1 bash tools/smokes/v2/profiles/integration/stageb/stageb_bundle_require_fail_vm.sh
SMOKES_ENABLE_STAGEB=1 bash tools/smokes/v2/profiles/integration/stageb/stageb_bundle_require_multi_fail_vm.sh
SMOKES_ENABLE_STAGEB=1 bash tools/smokes/v2/profiles/integration/stageb/stageb_bundle_alias_table_bad_vm.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
```

Result: PASS.

Additional helper coverage:

```bash
SMOKES_ENABLE_STAGEB=1 bash tools/smokes/v2/profiles/integration/stageb/stageb_bundle_require_ok_vm.sh
SMOKES_ENABLE_STAGEB=1 bash tools/smokes/v2/profiles/integration/stageb/stageb_bundle_mix_emit_vm.sh
```

Result: PASS.
