---
Status: Active
Scope: `.hako` MirBuilder を Rust の薄型 Recipe-first（Facts -> Recipe -> Verifier -> Lower）へ寄せる実行順序 SSOT。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/phases/phase-29bq/29bq-90-selfhost-checklist.md
  - docs/development/current/main/phases/phase-29bq/29bq-91-mirbuilder-migration-progress-checklist.md
  - docs/development/current/main/phases/phase-29bq/29bq-111-hako-mirbuilder-post-m4-lane.md
  - docs/development/current/main/design/selfhost-parser-mirbuilder-migration-order-ssot.md
---

# Phase 29bq — `.hako` Recipe-first Migration Lane (R0-R6)

目的:
- `.hako` 側 mirbuilder を「簡易直lower」から「薄型 Recipe-first」へ段階移行する。
- Rust 側との設計二重化を止め、検証点を Verifier へ集約する。

非目的:
- grammar 拡張（no-try/no-throw 方針を維持）。
- fallback で通す運用（strict/dev + planner_required で fail-fast）。
- 1コミットに複数責務を混在させること。

固定ルール:
- 1コミット = 1タスク（R0-R6の1項目）+ quick verify。
- fixture 追加は failure-driven（新規 freeze/reject / 回帰 / Decision変更）時のみ。
- 各タスクで `HAKO_SELFHOST_BUILDER_FIRST=1 HAKO_SELFHOST_NO_DELEGATE=1 ...hakorune_emit_mir.sh` + `phase29bq_fast_gate_vm.sh --only bq` を必須にする。

## 0) 依存順マップ（移植候補の再配置）

Box一覧は「行数」ではなく「Recipe-first依存順」で実行する:

1. 基盤: `ProgramJsonV0ScannerBox` / `ProgramJsonV0StateBox` / `HeaderEmitBox`
2. 既存 stmt handlers: `LocalStmtHandler` / `ReturnStmtHandler` / `PrintStmtHandler` / `AssignmentStmtHandler`（Post-M4で実体化済み）
3. 制御構文: `IfStmtHandler` / `LoopStmtHandler`（R5-min-1 実装済み）
4. 統合: `ProgramJsonV0PhaseStateBox` / `ProgramJsonV0PhaseStateConsumerBox` / `MirJsonV0BuilderBox`

## 1) Ordered Queue (must follow)

### R0: Recipe core vocabulary（データ構造のみ）

- [x] `RecipeItem` 相当（`Seq`/`If`/`Loop`/`Exit`）の `.hako` 表現を追加する。
- [x] `PortSig`（必要最小の定義/更新追跡）を追加する。
- [x] `RecipeVerifier` 骨格を追加（まだ lowering には接続しない）。

### R1: Facts extraction（stmt 4種から開始）

- [x] `Print`/`Local`/`Assignment`/`Return` の Facts 抽出を追加する。
- [x] `ProgramJsonV0PhaseStateConsumerBox` の stmt 経路で Facts->Recipe を生成する（既存出力は維持）。
- [x] 失敗時は `[freeze:contract][hako_mirbuilder]` で fail-fast する。

### R2: Verifier always-on（検証契約の固定）

- [x] Facts から生成した Recipe を Verifier で常時検証する。
- [x] 検証エラーは lowering へ進めず fail-fast する（silent fallback禁止）。

### R3: Lower wiring（機械配線へ寄せる）

- [x] `Print`/`Local`/`Assignment`/`Return` を Recipe経由 lowering に切替える。
- [x] `MirJsonV0BuilderBox` の入出力契約は変更しない（挙動不変）。
- [x] 旧直lower分岐は残してもよいが、責務を README に明記する。

### R4: If integration（M5）

- [x] `IfStmtHandler` を実装し、Facts->Recipe->Verifier->Lower で受理する。
- [x] 最小受理形（`Local -> If(var==int){return int}else{} -> Return(int)`）を Phase-10 pin で固定する。
- [x] Program(JSON v0) の `Expr(Call env.console.log(...))` を print 受理へ接続し、phase4 回帰を防ぐ。
- [x] quick verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase0_pin_vm.sh`
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase4_min_vm.sh`
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase10_min_vm.sh`
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`

### R5: Loop integration（M6）

- [x] 先に最小受理形を固定する（no-try/no-throw, failure-driven）。
- [x] `LoopStmtHandler` を実装し、Facts->Recipe->Verifier->Lower で受理する。
- [x] `stage1_cli` internal-only emit で契約を固定する。
- [x] quick verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh`
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_program_json_contract_pin_vm.sh`
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`
  - `HAKO_SELFHOST_BUILDER_FIRST=1 HAKO_SELFHOST_NO_DELEGATE=1 bash tools/hakorune_emit_mir.sh lang/src/runner/stage1_cli.hako /tmp/stage1_cli_fullbuilder.mir.json`

R5 minimal Loop contract (implementation SSOT):
- accepted shape (M6-min-1):
  - `Local(Int init)` -> `Loop(cond: Compare(Var < Int))` -> body contains exactly one `Assignment(Var = Binary(Var + Int))` -> `Return(Var|Int)`
- acceptance conditions:
  - `Loop` before first `Local` is rejected (`[cap_missing/stmt:Loop]`).
  - Nested `Loop` / `If-in-Loop` / `break` / `continue` / `cleanup` mixing are rejected in R5-min-1.
  - cond operators are limited to `<` in R5-min-1 (widening is R5 follow-up).
- widening policy:
  - Extend one shape at a time with fixture + pin + quick gate.
  - Do not mix Loop widening with unrelated cleanup/refactor in one commit.

### R6: Residue cleanup（統合と縮退）

- [x] `ProgramJsonV0PhaseStateBox/ConsumerBox` の legacy 直lower残骸を整理する。
  - `ProgramJsonV0PhaseStateConsumerBox`: legacy `shape_kind` 判定を `_legacy_shape_kind(...)` へ分離。
  - `ProgramJsonV0PhaseStateBox`: legacy order gate 判定を `_legacy_order_stage(...)` へ分離。
- [x] `lang/src/compiler/mirbuilder/README.md` / `29bq-91` / `CURRENT_TASK.md` を同期する。
- [x] Tier backlog docs は追加拡張せず、`failure-driven hold` 運用を維持する。

## Status Clarification (2026-02-09)

この lane（R0-R6）について、外部レビューで出やすい認識差をここで固定する。

- 実装済み（この lane の成果）:
  - Recipe Tree vocabulary: `lang/src/compiler/mirbuilder/recipe/recipe_item_box.hako`
  - Facts extraction: `lang/src/compiler/mirbuilder/recipe/recipe_facts_box.hako`
  - Verifier: `lang/src/compiler/mirbuilder/recipe/recipe_verifier_box.hako`
  - Lower wiring（Recipe -> MIR）: `lang/src/compiler/mirbuilder/mir_json_v0_builder_box.hako`
- したがって「Recipe/Verifier/Lower が未実装」は古い状態で、現SSOTでは該当しない。
- `cleanup` 統合はこの lane のスコープ外。R5 最小 Loop 契約では `cleanup` 混在を reject する。
- cleanup 統合に進む場合は、実装先行ではなく docs-first の準備レーン（`29bq-114`）で契約固定してから着手する。

## 2) Acceptance (per task)

必須:
- `HAKO_SELFHOST_BUILDER_FIRST=1 HAKO_SELFHOST_NO_DELEGATE=1 bash tools/hakorune_emit_mir.sh apps/tests/phase29bq_selfhost_cleanup_only_min.hako /tmp/phase29bq_cleanup_only_internal.mir.json`
- `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`

節目（R5/R6）:
- `HAKO_SELFHOST_BUILDER_FIRST=1 HAKO_SELFHOST_NO_DELEGATE=1 bash tools/hakorune_emit_mir.sh lang/src/runner/stage1_cli.hako /tmp/stage1_cli_fullbuilder.mir.json`

## 3) Next Selection Rule

R0-R6 は完了。以後は failure-driven で運用する:
1. 日常は quick verify のみ（`hakorune_emit_mir.sh` + `phase29bq_fast_gate_vm.sh --only bq`）。
2. 新規 freeze/reject または既存 green の回帰が出た時だけ、最小1件の PROBE→FIX→PROMOTE を実施する。
3. grammar 拡張は保留し、residue cleanup は BoxShape（挙動不変）に限定する。
4. cleanup 統合は `29bq-114` の C0-C3（docs-only）を満たすまで code lane に入れない。
