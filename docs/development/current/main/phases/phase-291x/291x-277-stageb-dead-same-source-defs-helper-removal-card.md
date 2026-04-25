---
Status: Landed
Date: 2026-04-26
Scope: Stage-B dead helper cleanup after BuildBox handoff.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-276-stageb-buildbox-handoff-adapter-card.md
  - lang/src/compiler/entry/stageb_same_source_defs_box.hako
---

# 291x-277: Stage-B Dead Same-Source Defs Helper Removal

## Problem

After `291x-276`, `compiler_stageb.hako` no longer calls
`StageBSameSourceDefsBox`. Source-to-Program enrichment is delegated through
`BuildBox.emit_program_json_v0(...)`, so the old Stage-B entry-local
same-source defs helper is no longer part of the live entry path.

Keeping the unused helper makes Stage-B look like it still has a second defs
scanner authority.

## Decision

Delete `lang/src/compiler/entry/stageb_same_source_defs_box.hako`.

Do not delete:

- `StageBBodyExtractorBox`: still used as a JoinIR/region test target.
- `StageBJsonBuilderBox`: still has direct-call alias and module snapshot
  references.
- `StageBKeywordExprStripBox`: still used by `StageBCompileAdapterBox`.

## Acceptance

- No live code reference to `StageBSameSourceDefsBox` remains.
- Stage-B binop smoke remains green.
- Stage-B quick minimal emit remains green.
- Current-state and CoreMethod no-growth guards remain green.

## Verification

```bash
rg -n "StageBSameSourceDefsBox|stageb_same_source_defs_box" lang src tools apps
bash tools/smokes/v2/profiles/integration/stageb/stageb_binop_vm.sh
HAKO_BUILD_TIMEOUT=20 bash tools/smokes/v2/profiles/quick/core/stageb_min_emit.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
```

Result: PASS. The `rg` command exits 1 because no live references remain.
