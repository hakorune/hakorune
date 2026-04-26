---
Status: Landed
Date: 2026-04-27
Scope: Stage-B adapter thinning inventory
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-437-next-lane-selection-card.md
  - docs/development/current/main/design/selfhost-authority-facade-compat-inventory-ssot.md
  - lang/src/compiler/entry/compiler_stageb.hako
  - lang/src/compiler/build/build_box.hako
---

# 291x-438: Stage-B Adapter Thinning Inventory

## Goal

Inventory the live `compiler_stageb.hako` responsibility surface before code
edits.

This is a BoxShape inventory. No behavior changed.

## Live Read

`compiler_stageb.hako` is already much thinner than the older inventory text
implies:

- no direct `ParserBox` import
- no body extraction implementation
- no same-source defs/import scan implementation
- no Program(JSON v0) fragment injection implementation
- no parser authority beside `BuildBox`

The live authority chain is:

```text
compiler_stageb.hako
  -> StageBCompileAdapterBox.emit_program_json_v0(...)
  -> BuildBox.emit_program_json_v0(source, null)
  -> BuildBox / helper boxes own source -> Program(JSON v0)
```

## Responsibility Table

| Surface | Current owner | Classification | Next action |
| --- | --- | --- | --- |
| `StageBDriverBox.compile(...)` | entry facade | keep in adapter | thin public compile API only |
| args/source resolution | `StageBArgsBox` | already extracted | keep out of entry |
| bundle/require CLI token packaging | `StageBBuildOptionsBox` | already extracted | keep out of entry; BuildBox owns validation/merge semantics |
| depth guard / trace markers | `StageBDriverGuardBox` | already extracted | keep as guard helper |
| source -> Program(JSON v0) | `BuildBox` | authority owner | keep as the only authority |
| keyword expr strip | `StageBCompileAdapterBox` | compat post-processing | keep quarantined; do not grow |
| null/freeze/list output handling | `compiler_stageb.hako` | entry-local output boundary | extract to a small output/result helper |
| `_starts_with` / `_is_freeze_tag` | `compiler_stageb.hako` | output helper residue | move with output/result helper or replace with shared helper |
| `HAKO_STAGEB_FUNCSCAN_TEST` block | `compiler_stageb.hako` | disabled dev harness residue | prune or quarantine; do not keep in hot entry |
| body extraction / parser / fragment injection | `BuildBox` family | no longer in entry | update stale docs wording |

## Next Cleanup Slice

Use `291x-439-stageb-output-boundary-helper` as the next code slice:

- add a small Stage-B output/result helper
- move null/freeze/list emission and `_starts_with`/freeze-tag checks out of
  `compiler_stageb.hako`
- keep `StageBDriverBox.main` as orchestration only
- do not move `BuildBox` authority or parser/body logic

The disabled FuncScan harness should be a follow-up slice after the output
boundary is thinner.

## Guards

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
