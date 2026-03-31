---
Status: SSOT
Scope: selfhost lane の file-level responsibility inventory を固定し、authority / adapter / facade / compat / shell split の移行順を 1 枚で管理する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/design/selfhost-compiler-structure-ssot.md
  - docs/development/current/main/design/selfhost-bootstrap-route-ssot.md
  - docs/development/current/main/design/json-v0-route-map-ssot.md
  - docs/development/current/main/phases/phase-29ci/README.md
  - lang/src/compiler/README.md
  - lang/src/runner/README.md
  - tools/selfhost/README.md
---

# Selfhost Authority / Facade / Compat Inventory SSOT

## Goal

selfhost lane で混線しやすい 3 軸

- stage
- artifact
- responsibility

を file-level で読み分ける。

この文書は route vocabulary や stage vocabulary の親 SSOTではない。
ここでは `どのファイルが何者で、次にどう縮めるか` だけを固定する。

## Vocabulary

- `keep authority`
  - current owner として残す
- `shrink to adapter`
  - authority を持たず、入力整形と handoff のみに縮める
- `shrink to facade`
  - CLI/orchestration のみ残し、pipeline detail は外へ逃がす
- `same-file cluster keep`
  - 現状は同一ファイル cluster を維持する
- `strategy shell`
  - build/reuse/bootstrap strategy を選ぶ shell
- `contract shell`
  - env / proof / capability contract を pin する shell
- `thin entry stub`
  - bootstrap artifact entry only; no CLI or pipeline policy
- `compat quarantine`
  - explicit compat keep / retire target
- `legacy naming keep`
  - 旧名や互換名を明示 keep として残す

## File Inventory

| File / Surface | Current target role | Bucket | Current read | Next action |
| --- | --- | --- | --- | --- |
| `lang/src/compiler/build/build_box.hako` | `source -> Program(JSON v0)` authority | `keep authority` | `BuildBox.emit_program_json_v0(...)` is the current source-to-Program authority | keep as the single authority owner |
| `lang/src/compiler/entry/compiler_stageb.hako` | Stage-B emit entry | `shrink to adapter` | Stage-B entry currently still carries parse/body/defs/import shaping residue | shrink toward input adaptation + `BuildBox` handoff only |
| `lang/src/runner/launcher.hako` | stage1 CLI facade/orchestration | `shrink to facade` | now routes build/emit through `LauncherCompileFacadeBox` and payload-contract helpers; direct Program(JSON) wrapper boxes are gone | keep shrinking toward CLI/request dispatch only |
| `lang/src/runner/stage1_cli_env.hako` | stage1 env-entry authority cluster | `same-file cluster keep` | owner-local small boxes are already split, but same-file for now | defer file split until authority/facade cleanup proves a blocker |
| `lang/src/runner/launcher_native_entry.hako` | launcher bootstrap entry stub | `thin entry stub` | run-only bootstrap shell for `build_stage1.sh` launcher-exe; carries no CLI policy | keep thin; logical owner stays in `launcher.hako` |
| `lang/src/runner/stage1_cli_env_entry.hako` | stage1-cli bootstrap entry stub | `thin entry stub` | run-only bootstrap shell for `build_stage1.sh` stage1-cli; carries no CLI policy | keep thin; logical owner stays in `stage1_cli_env.hako` |
| `tools/selfhost/build_stage1.sh` | bootstrap strategy owner | `strategy shell` | chooses artifact/bootstrap/reuse/fallback strategy | keep strategy here and avoid compiler-authority growth |
| `tools/selfhost/lib/stage1_contract.sh` | stage1 shell contract owner | `contract shell` | owns env inject / emit proof / capability verify | keep shell contract centralized here |
| `Program(JSON v0)` route/surfaces | bootstrap-only compat family | `compat quarantine` | compat/bootstrap keep + retire target | keep quarantined; do not reopen as mainline |
| `nyash` legacy naming residue | old binary/package wording | `legacy naming keep` | compat naming residue still exists | defer cleanup until after structure cleanup |

## Fixed Migration Order

1. authority unification
   - shrink `compiler_stageb.hako` toward Stage-B adapter shape
   - keep `BuildBox.emit_program_json_v0(...)` as the only `source -> Program(JSON v0)` authority
2. launcher facade extraction
   - shrink `launcher.hako` toward CLI facade/orchestration shape
3. conditional `stage1_cli_env.hako` split
   - only if steps 1/2 leave a real blocker
4. shell strategy/contract split
   - keep `build_stage1.sh` vs `stage1_contract.sh` reading explicit
5. compat physical isolation
   - move compat families only after authority/facade cleanup is stable
6. naming cleanup last
   - `nyash` / `hakorune` legacy naming is not part of the current structure wave

## Accepted Now

- `BuildBox.emit_program_json_v0(...)` is the current sole `source -> Program(JSON v0)` authority
- `compiler_stageb.hako` should shrink toward Stage-B entry/adapter shape
- `launcher.hako` should shrink toward CLI facade/orchestration shape
- `build_stage1.sh` should be read as `strategy shell`
- `stage1_contract.sh` should be read as `contract shell`

## Deferred On Purpose

- forced file split of `stage1_cli_env.hako`
- physical `compat/` directory migration
- package / crate / bin naming cleanup

## Non-Goals

- do not redefine stage vocabulary here
- do not reopen `Program(JSON v0)` as a mainline artifact family
- do not mix file-level cleanup with new compiler acceptance shapes
