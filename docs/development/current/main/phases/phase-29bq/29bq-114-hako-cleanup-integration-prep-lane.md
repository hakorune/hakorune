---
Status: Active (C0-C3 done, M7-min-1/2/3/4 done; failure-driven monitor)
Scope: `.hako` mirbuilder cleanup integration の契約固定 + 最小受理形を段階実装するレーン
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/phases/phase-29bq/README.md
  - docs/development/current/main/phases/phase-29bq/29bq-91-mirbuilder-migration-progress-checklist.md
  - docs/development/current/main/phases/phase-29bq/29bq-113-hako-recipe-first-migration-lane.md
  - docs/development/current/main/design/fini-cleanup-execution-contract-ssot.md
---

# Phase 29bq — `.hako` Cleanup Integration Prep Lane (C0-C3)

目的:
- `cleanup` 統合を「実装先行」ではなく「契約先行」で進める。
- 受理形/拒否形/境界責務を先に固定し、BoxCount と BoxShape の混線を防ぐ。

非目的:
- 新しい表面構文の導入
- try/throw の再導入
- fallback で通す暫定実装

前提（固定）:
- R0-R6（Recipe Tree / Facts / Verifier / Lower）は完了済み。
- R5 minimal Loop 契約では `cleanup` 混在は reject。
- C0-C3 完了後は `M7-min-*` を 1受理形ずつ実装し、毎回 pin/gate で境界を固定する。

## Ordered Queue (docs-only first)

### C0: Cleanup integration minimum contract
- [x] 最小受理形を1つ定義（non-loop の postfix cleanup 1形）
- [x] reject 形を明記（nested exit / loop+cleanup 混在など）
- [x] freeze tag を固定（`[freeze:contract][hako_mirbuilder][cap_missing/... ]`）
- [x] 非目標を明記（今回やらない形）

C0 contract（fixed）:
- 最小受理形（このレーンの起点）:
  - non-loop postfix cleanup のみ。
  - 参照 fixture: `apps/tests/selfhost_cleanup_only_min.hako`
  - 期待する最小流れ: `Local -> Block -> postfix cleanup -> Return`
- reject 形（R5最小契約の維持）:
  - `loop + cleanup` 混在（同一スコープ）。
  - cleanup body 内の non-local exit（`return` / `break` / `continue` / `throw`）。
  - cleanup 統合を前提にした nested-control widening（M7前は受理拡張しない）。
- freeze tag（固定）:
  - 共通 prefix: `[freeze:contract][hako_mirbuilder]`
  - cleanup 未対応/未受理: `[cap_missing/stmt:Cleanup]`
  - loop 契約違反（loop+cleanup 混在を含む）: `[cap_missing/stmt:Loop]`
  - fini body の non-local exit（bridge 側契約）: `[freeze:contract][json_v0_bridge/fini_forbid_non_local_exit]`
- 非目標（C0ではやらない）:
  - try/throw の再導入、表面構文拡張。
  - loop cleanup widening（`break/continue/cleanup` 混在受理）。
  - execution order の再定義（`fini-cleanup-execution-contract-ssot` を優先）。

### C1: Acceptance pin contract
- [x] fixture/pin の最小セットを定義（accept 1 + reject 1）
- [x] quick verify コマンドを固定
- [x] full selfhost canary は節目のみ実行ルールを明記

C1 contract（fixed）:
- accept pin（runtime/gate 側の基準）:
  - fixture: `apps/tests/selfhost_cleanup_only_min.hako`
  - command:
    - `SMOKES_ENABLE_SELFHOST=1 HAKO_JOINIR_PLANNER_REQUIRED=1 ./tools/selfhost/run.sh --gate --planner-required 1 --filter cleanup_only_min --max-cases 1`
  - pass 条件: gate exit=0 かつ case PASS（`Expected 11` 互換）。
- reject pin（C1 baseline / pre-M7 snapshot）:
  - fixture: `apps/tests/selfhost_cleanup_only_min.hako`
  - command（2段）:
    - `./target/release/hakorune --emit-program-json-v0 /tmp/phase29bq_c1_cleanup_try.json apps/tests/selfhost_cleanup_only_min.hako`
    - `HAKO_PROGRAM_JSON_FILE=/tmp/phase29bq_c1_cleanup_try.json ./target/release/hakorune --backend vm lang/src/compiler/mirbuilder/emit_mir_json_v0_from_program_json_v0.hako > /tmp/phase29bq_c1_cleanup_try.mir.json`
  - pass 条件: exit=2 かつ `/tmp/phase29bq_c1_cleanup_try.mir.json` に
    - `[freeze:contract][hako_mirbuilder][cap_missing/stmt:Try]`
    - を含む（cleanup 統合前の fail-fast 境界）。
- accept pin（post-M7-min-1 / `.hako` mirbuilder 経路）:
  - fixture: `apps/tests/selfhost_cleanup_only_min.hako`
  - command:
    - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_cleanup_try_min_vm.sh`
  - pass 条件: exit=0 かつ stdout=`11`。
- accept pin（post-M7-min-3 / `.hako` mirbuilder 経路）:
  - command:
    - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_cleanup_try_finally_local_min_vm.sh`
  - pass 条件: `finally=[Local 1stmt]` 変形ケースが exit=0 かつ stdout=`13`。
- reject 境界（current）:
  - non-minimal `Try(cleanup)` 形（multi stmt / catches 非空 / loop+cleanup）は fail-fast（`[cap_missing/stmt:Try]` または `[cap_missing/stmt:Loop]`）。
- reject pin（post-M7-min-2 / `.hako` mirbuilder 経路）:
  - command:
    - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_cleanup_try_reject_nonminimal_vm.sh`
  - pass 条件: 3ケース（multi stmt / catches 非空 / loop+cleanup）すべて reject tag で失敗する。
- reject pin（post-M7-min-4 / `.hako` mirbuilder 経路）:
  - command:
    - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_cleanup_try_finally_local_var_mismatch_reject_vm.sh`
  - pass 条件: `Try` body 更新変数と `finally Local` 更新変数が不一致のケースが
    - `[freeze:contract][hako_mirbuilder][cap_missing/stmt:Try] Try finally Local must update the same var as Try body`
    - で fail-fast する。
- daily quick verify（C1時点）:
  - `cargo check --bin hakorune`
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_cleanup_try_min_vm.sh`
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`
  - 上記 pins の再現
- full selfhost canary rhythm:
  - `RUN_TIMEOUT_SECS=120 SMOKES_ENABLE_SELFHOST=1 HAKO_JOINIR_PLANNER_REQUIRED=1 bash tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_planner_required_dev_gate_vm.sh`
  - は節目実行のみ（1日の上限は 2〜3 回）。

### C2: Handler/Recipe boundary map
- [x] consumer dispatch 境界（どこで cleanup を受けるか）を1箇所に固定
- [x] Facts / Verifier / Lower の責務分割を1図で固定
- [x] `README` と `29bq-91` の対応箇所を先に指定

C2 boundary map（fixed）:
- single dispatch entry（cleanup の受け口）:
  - `ProgramJsonV0PhaseStateConsumerBox.consume_stmt(...)`
  - `ProgramJsonV0PhaseStateConsumerBox._dispatch_or_unsupported(...)`
  - ここ以外で `Try/cleanup` を個別に分岐しない（入口一本化）。
- node routing contract（M7-min-1 後の境界）:
  - non-control: `Print/Local/Assignment/Return` -> 既存 stmt handlers
  - control: `If/Loop` -> 既存 control handlers
  - cleanup source: Program(JSON v0) の `StmtV0::Try`（postfix cleanup 由来）
  - `M7-min-1` で `Try` を cleanup 専用 handler へ集約し、consumer 側で意味論を持たない
  - current: non-loop 最小形のみ受理。その他 `Try` 形は fail-fast（`[cap_missing/stmt:Try]`）で固定
- Facts / Verifier / Lower split（責務図）:
  - `stmt_handlers/*`: shape の読み出しと Recipe item 生成のみ（MIR emit禁止）
  - `recipe/recipe_facts_box.hako`: cleanup 受理条件の抽出（観測のみ）
  - `recipe/recipe_verifier_box.hako`: cleanup 契約検証（non-local exit 禁止など）を fail-fast で固定
  - `mir_json_v0_builder_box.hako`: verifier 済み recipe のみを MIR JSON v0 に下ろす
  - `program_json_v0_phase_state_box.hako`: scan/order のみ（cleanup 意味論を持たない）
- pointer sync（対応箇所）:
  - `lang/src/compiler/mirbuilder/README.md` の「Cleanup integration boundary (C2 prep)」
  - `docs/development/current/main/phases/phase-29bq/29bq-91-mirbuilder-migration-progress-checklist.md` の「2.8 Cleanup integration prep (C2 boundary pin)」

### C3: Go/No-Go decision
- [x] C0-C2 が揃ったら「実装開始条件」を明記
- [x] 1コミット=1受理形（fixture+gate同梱）を再確認
- [x] failure-driven 運用と矛盾しないことを確認

C3 decision（fixed）:
- Go（M7実装を開始してよい条件）:
  - C0/C1/C2 が docs 上ですべて固定済み。
  - C1 baseline accept/reject pin が再現可能（accept: gate PASS / reject: `cap_missing/stmt:Try`）。
  - daily quick verify（`cargo check --bin hakorune` + `phase29bq_fast_gate_vm.sh --only bq`）が green。
- No-Go（実装開始を止める条件）:
  - accept/reject pin のどちらかが再現しない。
  - freeze tag が不安定（文言や prefix の揺れが発生）。
  - cleanup 実装タスクに loop widening / grammar変更 / fallback が混入した。
- 実装単位ルール（再確認）:
  - 1コミット = 1受理形 + fixture/pin + quick gate。
  - BoxShape（整理）と BoxCount（受理拡張）を同一コミットに混ぜない。
  - full selfhost canary は節目のみ（1日 2〜3 回上限）。
- M7-min-1 result（実装済み）:
  - commit: `192173aca`
  - scope: Program(JSON v0) `StmtV0::Try`（postfix cleanup由来）の non-loop 最小形受理
  - acceptance pin: `phase29bq_hako_mirbuilder_cleanup_try_min_vm.sh`（PASS）
- M7-min-2 result（実装済み）:
  - commit: this change-set（M7-min-2）
  - scope: non-minimal `Try(cleanup)` 形（multi stmt / catches 非空 / loop+cleanup）の reject pin 固定
  - reject pin: `phase29bq_hako_mirbuilder_cleanup_try_reject_nonminimal_vm.sh`（PASS）
- M7-min-3 result（実装済み）:
  - commit: this change-set（M7-min-3）
  - scope: non-loop cleanup で `finally=[Local 1stmt]` を1形だけ受理
  - acceptance pin: `phase29bq_hako_mirbuilder_cleanup_try_finally_local_min_vm.sh`（PASS）
- M7-min-4 result（実装済み）:
  - commit: this change-set（M7-min-4）
  - scope: `Try` body と `finally Local` の更新変数不一致を reject pin 化して同一変数契約を固定
  - reject pin: `phase29bq_hako_mirbuilder_cleanup_try_finally_local_var_mismatch_reject_vm.sh`（PASS）
- M7 next operation（failure-driven）:
  - 新規 freeze/reject か契約変更が出るまで、M7 の受理拡張は追加しない。
  - 回帰時は `StmtV0::Try` 受理拡張を戻し、C1 baseline reject pin（`cap_missing/stmt:Try`）へ即復帰する。

## Exit Criteria

- C0-C3 がすべて完了
- `M7-min-1` が pin/gate 付きで実装・固定されている
- `M7-min-2` が reject pin/gate 付きで実装・固定されている
- `M7-min-3` が accept pin/gate 付きで実装・固定されている
- `M7-min-4` が reject pin/gate 付きで実装・固定されている
- `CURRENT_TASK.md` / `29bq-113` / 本ドキュメントで記述矛盾がない
- 次タスクが failure-driven 運用（新規回帰時のみ M7 拡張）として固定されている
