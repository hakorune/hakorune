---
Status: Landed
Date: 2026-04-27
Scope: Stage-B adapter thinning closeout
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-438-stageb-adapter-thinning-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-439-stageb-output-boundary-helper-card.md
  - docs/development/current/main/phases/phase-291x/291x-440-stageb-disabled-funcscan-harness-cleanup-card.md
  - docs/development/current/main/design/selfhost-authority-facade-compat-inventory-ssot.md
  - lang/src/compiler/entry/compiler_stageb.hako
---

# 291x-441: Stage-B Adapter Thinning Closeout

## Goal

Close the Stage-B adapter thinning burst and prevent open-ended entry cleanup.

This is a closeout card. No behavior changed.

## Closed In This Burst

- `291x-438` inventoried the live Stage-B entry surface.
- `291x-439` extracted Program(JSON v0) output/error boundary handling into
  `StageBOutputBox`.
- `291x-440` removed the disabled `HAKO_STAGEB_FUNCSCAN_TEST` no-op harness.

## Final Boundary

`compiler_stageb.hako` now remains adapter-shaped:

```text
StageBDriverBox.main(args)
  -> StageBDriverGuardBox
  -> StageBArgsBox / StageBBuildOptionsBox
  -> StageBCompileAdapterBox
  -> StageBOutputBox
```

The source -> Program(JSON v0) authority remains:

```text
StageBCompileAdapterBox
  -> BuildBox.emit_program_json_v0(source, null)
```

## Confirmed Absent From Entry

- direct `ParserBox` import
- body extraction implementation
- same-source defs/import scan implementation
- Program(JSON v0) fragment injection implementation
- disabled FuncScan success detour

## Deferred To New Lanes

- CoreMethodContract -> CoreOp / LoweringPlan migration
- `.inc` generated enum/table consumer migration
- MapGet return-shape metadata / proof / lowering work
- launcher/stage1 facade cleanup
- historical roadmap/archive FuncScanner wording cleanup

## Decision

Stop the Stage-B adapter thinning burst here. The next card should choose the
next compiler-cleanliness lane instead of continuing small Stage-B entry edits
by default.

## Guards

```bash
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
```

Result: PASS.
