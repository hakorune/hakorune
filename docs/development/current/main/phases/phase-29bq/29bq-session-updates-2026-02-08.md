# Phase-29bq Session Updates Archive (2026-02-08)

Status: historical session log (read-only)
Source: moved from `CURRENT_TASK.md` during compaction on 2026-02-08
Note: exact `SMOKES_SELFHOST_FILTER=...` substrings and exact fixture basenames below are historical evidence only.
Use semantic substring guidance from `joinir-planner-required-gates-ssot.md` / `phase-29ce` for current operations.

### Session Update (2026-02-08, mirbuilder handler extraction M0)

- [x] `PrintStmtHandler` を STUB から実体化（既存 `ProgramJsonV0ConsumerPrintBox` ロジックへ委譲して挙動差分を抑制）
- [x] `ProgramJsonV0PhaseStateConsumerBox` の `Print` 分岐を handler 呼び出しへ配線切替
- [x] verify: `HAKO_SELFHOST_BUILDER_FIRST=1 HAKO_SELFHOST_NO_DELEGATE=1 bash tools/hakorune_emit_mir.sh apps/tests/phase29bq_selfhost_cleanup_only_min.hako /tmp/phase29bq_cleanup_only_internal.mir.json` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-08, mirbuilder handler extraction M1)

- [x] `LocalStmtHandler` を STUB から実体化（既存 `ProgramJsonV0ConsumerLocalBox` ロジックへ委譲して挙動差分を抑制）
- [x] `ProgramJsonV0PhaseStateConsumerBox` の `Local` 分岐を handler 呼び出しへ配線切替
- [x] verify: `HAKO_SELFHOST_BUILDER_FIRST=1 HAKO_SELFHOST_NO_DELEGATE=1 bash tools/hakorune_emit_mir.sh apps/tests/phase29bq_selfhost_cleanup_only_min.hako /tmp/phase29bq_cleanup_only_internal.mir.json` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-08, mirbuilder handler extraction M2)

- [x] `AssignmentStmtHandler` を STUB から実体化（既存 `ProgramJsonV0ConsumerAssignmentBox` ロジックへ委譲して挙動差分を抑制）
- [x] `ProgramJsonV0PhaseStateConsumerBox` の `Assignment` 分岐を handler 呼び出しへ配線切替
- [x] verify: `HAKO_SELFHOST_BUILDER_FIRST=1 HAKO_SELFHOST_NO_DELEGATE=1 bash tools/hakorune_emit_mir.sh apps/tests/phase29bq_selfhost_cleanup_only_min.hako /tmp/phase29bq_cleanup_only_internal.mir.json` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-08, mirbuilder handler extraction M3)

- [x] `ReturnStmtHandler` を STUB から実体化（既存 `ProgramJsonV0ConsumerReturnBox` ロジックへ委譲して挙動差分を抑制）
- [x] `ProgramJsonV0PhaseStateConsumerBox` の `Return` 分岐を handler 呼び出しへ配線切替
- [x] verify: `HAKO_SELFHOST_BUILDER_FIRST=1 HAKO_SELFHOST_NO_DELEGATE=1 bash tools/hakorune_emit_mir.sh apps/tests/phase29bq_selfhost_cleanup_only_min.hako /tmp/phase29bq_cleanup_only_internal.mir.json` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-08, mirbuilder handler extraction M4)

- [x] `program_json_v0_phase_state_box.hako` の旧 pass-through 残骸を縮退（`_consume_stmt_in_body` ラッパを撤去し、consumer SSOT 直結）
- [x] `lang/src/compiler/mirbuilder/README.md` と `29bq-91` の進捗表記を同期
- [x] verify: `HAKO_SELFHOST_BUILDER_FIRST=1 HAKO_SELFHOST_NO_DELEGATE=1 bash tools/hakorune_emit_mir.sh lang/src/runner/stage1_cli.hako /tmp/stage1_cli_fullbuilder.mir.json` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-08, Post-M4 lane organization)

- [x] Post-M4 専用 backlog を追加（`29bq-111`、P1→P5 を順序固定）
- [x] `CURRENT_TASK.md` を Post-M4 queue（P1→P5）へ同期
- [x] `10-Now` の selfhost 復帰ポリシーを「限定再開（BoxShape 整理のみ）」へ同期
- [x] `29bq-91` / `29bq-90` に Post-M4 lane の導線を追記

### Session Update (2026-02-08, Post-M4 P1 print ownership move)

- [x] `PrintStmtHandler` を STUB 委譲から実体化し、Print (`Int`/`Var`/`Binary(Var+Int)`) の fail-fast 判定を handler 側へ移管
- [x] `ProgramJsonV0ConsumerPrintBox` を compatibility wrapper に縮退（`PrintStmtHandler.handle(...)` へ単純委譲）
- [x] `29bq-111` / `29bq-91` / `CURRENT_TASK.md` を P1 完了状態へ同期
- [x] verify: `HAKO_SELFHOST_BUILDER_FIRST=1 HAKO_SELFHOST_NO_DELEGATE=1 bash tools/hakorune_emit_mir.sh apps/tests/phase29bq_selfhost_cleanup_only_min.hako /tmp/phase29bq_cleanup_only_internal.mir.json` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-08, Post-M4 P2 local ownership move)

- [x] `LocalStmtHandler` を STUB 委譲から実体化し、Local (`Int` 初期化 + name 抽出) の fail-fast 判定を handler 側へ移管
- [x] `ProgramJsonV0ConsumerLocalBox` を compatibility wrapper に縮退（`LocalStmtHandler.handle(...)` へ単純委譲）
- [x] `29bq-111` / `29bq-91` / `CURRENT_TASK.md` を P2 完了状態へ同期
- [x] verify: `HAKO_SELFHOST_BUILDER_FIRST=1 HAKO_SELFHOST_NO_DELEGATE=1 bash tools/hakorune_emit_mir.sh apps/tests/phase29bq_selfhost_cleanup_only_min.hako /tmp/phase29bq_cleanup_only_internal.mir.json` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-08, Post-M4 P3 assignment ownership move)

- [x] `AssignmentStmtHandler` を STUB 委譲から実体化し、Assignment (`x = x + <int>`) の fail-fast 判定を handler 側へ移管
- [x] `ProgramJsonV0ConsumerAssignmentBox` を compatibility wrapper に縮退（`AssignmentStmtHandler.handle(...)` へ単純委譲）
- [x] `29bq-111` / `29bq-91` / `CURRENT_TASK.md` を P3 完了状態へ同期
- [x] verify: `HAKO_SELFHOST_BUILDER_FIRST=1 HAKO_SELFHOST_NO_DELEGATE=1 bash tools/hakorune_emit_mir.sh apps/tests/phase29bq_selfhost_cleanup_only_min.hako /tmp/phase29bq_cleanup_only_internal.mir.json` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-08, Post-M4 P4 return ownership move)

- [x] `ReturnStmtHandler` を STUB 委譲から実体化し、Return (`Int` / `Var`) の fail-fast 判定を handler 側へ移管
- [x] `ProgramJsonV0ConsumerReturnBox` を compatibility wrapper に縮退（`ReturnStmtHandler.handle(...)` へ単純委譲）
- [x] `29bq-111` / `29bq-91` / `CURRENT_TASK.md` を P4 完了状態へ同期
- [x] verify: `HAKO_SELFHOST_BUILDER_FIRST=1 HAKO_SELFHOST_NO_DELEGATE=1 bash tools/hakorune_emit_mir.sh apps/tests/phase29bq_selfhost_cleanup_only_min.hako /tmp/phase29bq_cleanup_only_internal.mir.json` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-08, Post-M4 P5 consumer residue cleanup)

- [x] `program_json_v0_consumer_{print,local,assignment,return}_box.hako` の参照ゼロを確認し、互換ラッパ4ファイルを削除
- [x] `lang/src/compiler/mirbuilder/README.md` を handler直結境界に同期（consumer層は撤去済みを明記）
- [x] `29bq-111` / `29bq-91` / `CURRENT_TASK.md` を P5 完了状態へ同期
- [x] verify: `HAKO_SELFHOST_BUILDER_FIRST=1 HAKO_SELFHOST_NO_DELEGATE=1 bash tools/hakorune_emit_mir.sh lang/src/runner/stage1_cli.hako /tmp/stage1_cli_fullbuilder.mir.json` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-08, Recipe-first R6 residue cleanup)

- [x] `program_json_v0_phase_state_consumer_box.hako` の legacy shape 判定を `_legacy_shape_kind(...)` へ分離し、`_emit_out(...)` から判定責務を切り離し
- [x] `program_json_v0_phase_state_box.hako` の legacy order gate 判定を `_legacy_order_stage(...)` へ分離し、parse本体の inline 分岐を縮退
- [x] `29bq-113` / `29bq-91` / `CURRENT_TASK.md` / `lang/src/compiler/mirbuilder/README.md` を R6 完了へ同期
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-08, Recipe-first post-R6 consumer dedup)

- [x] `program_json_v0_phase_state_consumer_box.hako` に `_emit_out_from_state(...)` を追加し、`_consume_stmt_in_body(...)` の重複 `_emit_out(...)` 呼び出しを state-map 経由へ統一
- [x] `Print/Local/Assignment/Return/If/Loop` の各分岐で `current_state` / `after_state` を使う形に整理し、recipe verifier の通過点は維持（挙動不変）
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-08, Recipe-first post-R6 builder shape dispatch split)

- [x] `mir_json_v0_builder_box.hako` に `_build_mir_by_shape(...)` を追加し、`build(...)` 内の巨大 `shape_kind` 分岐を helper へ抽出
- [x] `build(...)` は shape 決定（recipe適用 + contract check）に集中し、MIR文字列組み立て責務を分離
- [x] fail-fast tag / mismatch message / shape受理条件は不変（BoxShape only）
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-08, Recipe-first post-R6 state scanner emit dedup)

- [x] `program_json_v0_phase_state_box.hako` に `_state_map(...)` / `_emit_out_from_state(...)` を追加し、`_scan_body_rec(...)` 内の重複 `emit_out` 呼び出しを state-map 経由へ統一
- [x] `node_peek` 失敗・order mismatch・consumer error などの fail-fast 点は維持し、返却 state の意味論は不変
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-08, Recipe-first post-R6 builder ctx boundary)

- [x] `MirJsonV0BuilderBox._build_mir_by_shape` の引数境界を `shape_ctx` map 1本へ縮退（`_build_mir_by_shape/20 -> /3`）
- [x] `build(...)` 側で shape 判定後の入力を `shape_ctx` に集約し、dispatch 境界を固定
- [x] shape受理条件・freeze tag・MIR文字列は不変（境界整理のみ）
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-08, Recipe-first post-R6 legacy classifier box split)

- [x] `program_json_v0_legacy_shape_classifier_box.hako` を追加し、legacy `shape_kind` 判定ロジックを独立 box（`ProgramJsonV0LegacyShapeClassifierBox.classify`）へ移設
- [x] `ProgramJsonV0PhaseStateConsumerBox` は `shape_kind` 判定の内包ロジックを廃止し、classifier 呼び出しへ集約（consumer は state/recipe 配線責務に集中）
- [x] `nyash.toml` module registration と `lang/src/compiler/mirbuilder/README.md` の split-box 導線を同期
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-08, Recipe-first post-R6 legacy order stage box split)

- [x] `program_json_v0_legacy_order_stage_box.hako` を追加し、legacy order gate 判定を独立 box（`ProgramJsonV0LegacyOrderStageBox.classify`）へ移設
- [x] `ProgramJsonV0PhaseStateBox` は `_legacy_order_stage(...)` を廃止し、order gate 判定を classifier box 呼び出しへ集約
- [x] `nyash.toml` module registration と `lang/src/compiler/mirbuilder/README.md` の split-box 導線を同期
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-08, Recipe-first post-R6 consumer recipe verify helper)

- [x] `ProgramJsonV0PhaseStateConsumerBox` に `_verify_recipe_and_emit(...)` を追加し、`recipe_err`/`verify_err` 判定 + success emit + attach の重複を helper へ集約
- [x] `Print/Local/Assignment/Return/If/Loop` の各分岐を helper 呼び出しへ置換し、state遷移・freeze tag・戻り値フォーマットは不変（BoxShape only）
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-08, Recipe-first post-R6 consumer dispatch helper split)

- [x] `ProgramJsonV0PhaseStateConsumerBox` に node-type 別 helper（`_handle_print_or_expr` / `_handle_local` / `_handle_assignment` / `_handle_return` / `_handle_if` / `_handle_loop`）を追加
- [x] `_consume_stmt_in_body(...)` は state 作成 + node読み取り + dispatch のみへ縮退し、分岐本体ロジックを helper 側へ移管（挙動不変）
- [x] 共有抽出として `_state_i64(...)` を追加し、state map からの flag 取得を SSOT 化
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-08, Recipe-first post-R6 control helper unify)

- [x] `ProgramJsonV0PhaseStateConsumerBox` に `_control_state(...)` / `_emit_control_recipe_or_error(...)` を追加し、If/Loop の result handling（error/next_idx/recipe_item検証）を共通化
- [x] `_handle_if(...)` / `_handle_loop(...)` は handler 呼び出し + 共通 helper への委譲のみへ縮退（freeze tag文言は既存と同一）
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-08, Recipe-first post-R6 state-map consumer boundary)

- [x] `ProgramJsonV0PhaseStateConsumerBox.consume_stmt(program_json, idx, tag, state_map)` を追加し、state-map 1本の入口を新設（map未指定は fail-fast）
- [x] `ProgramJsonV0PhaseStateBox` 側の stmt 消費呼び出しを新入口へ切替し、長い引数配線（17引数）を境界から除去
- [x] `lang/src/compiler/mirbuilder/README.md` の consumer boundary 記述を新入口へ同期
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-06, local/fini parser handoff)

- [x] selfhost parser: `fini { ... }` / `local ... fini { ... }` を受理（`FiniReg` marker を JSON v0 に出力）
- [x] json_v0_bridge: `FiniReg` を scope-exit 正規化（`Try(finally)`）に統一
- [x] BlockExpr prelude の `FiniReg` も bridge 側で正規化して実行順を固定
- [x] promote fixture 追加: `apps/tests/phase29bq_selfhost_blocker_parse_local_fini_min.hako`（expected=`SEBAF0`）
- [x] subset 追記: `tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv`
- [x] verify: `cargo build --release --bin hakorune`
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [x] verify: `RUN_TIMEOUT_SECS=120 SMOKES_ENABLE_SELFHOST=1 HAKO_JOINIR_PLANNER_REQUIRED=1 bash tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_planner_required_dev_gate_vm.sh` PASS（`131/131`, `total_secs=430`）

### Session Update (2026-02-06, fini body contract)

- [x] json_v0_bridge: `fini { ... }` 内の非ローカル脱出を fail-fast（`return`/`throw`/`break`/`continue` + nested `FiniReg`）
  - freeze tag: `[freeze:contract][json_v0_bridge/fini_forbid_non_local_exit]`
  - note: `panic!`/`catch_unwind` ではなく `Result::Err` を伝播（診断距離を短くする）
- [x] verify: `cargo test -q json_stageb_fini_reg_in_blockexpr_verifies`
- [x] verify: `cargo test -q json_fini_reg_forbid_`
- [x] verify: `RUN_TIMEOUT_SECS=120 SMOKES_ENABLE_SELFHOST=1 HAKO_JOINIR_PLANNER_REQUIRED=1 SMOKES_SELFHOST_FILTER=parse_local_fini_min SMOKES_SELFHOST_MAX_CASES=1 bash tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_planner_required_dev_gate_vm.sh` PASS

### Session Update (2026-02-06, parser try legacy boundary)

- [x] parser: `try` を legacy surface として明示（既定互換は維持）
  - freeze tag: `[freeze:contract][parser/try_reserved]`
  - gate: `NYASH_FEATURES=no-try-compat` で `try` を fail-fast（`catch/cleanup` へ誘導）
- [x] verify: `cargo test -q no_try_compat_feature_rejects_try_with_freeze_tag`
- [x] verify: `cargo test -q stage3_enabled_accepts_try_catch_variants`
- [x] verify: `cargo test -q stage3_default_enabled_accepts_try_and_rejects_throw`
- [x] verify: `cargo check --bin hakorune`
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [x] verify: `SMOKES_ENABLE_SELFHOST=1 HAKO_JOINIR_PLANNER_REQUIRED=1 SMOKES_SELFHOST_FILTER=parse_try_min SMOKES_SELFHOST_MAX_CASES=1 bash tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_planner_required_dev_gate_vm.sh` PASS

### Session Update (2026-02-06, try boundary probe split)

- [x] selfhost subset inventory: `planner_required_selfhost_subset.tsv` は `131` fixture、`try` 使用は `2` fixture（`phase29bq_selfhost_try_loop_throw_catch_min.hako` / `phase29bq_selfhost_try_throw_catch_cleanup_min.hako`）
- [x] boundary probe: `NYASH_FEATURES=stage3,no-try-compat` でも selfhost gate（Stage-B compiler 経由）は PASS（Rust parser 契約を直接は検証しない）
- [x] parser direct probe: `./target/release/hakorune --backend vm apps/tests/phase29bq_selfhost_try_throw_catch_cleanup_min.hako` は `no-try-compat` で `[freeze:contract][parser/try_reserved]` を返すことを確認
- [x] lightweight contract smoke を追加: `tools/smokes/v2/profiles/integration/parser/parser_try_compat_boundary.sh`
- [x] verify: `bash tools/smokes/v2/profiles/integration/parser/parser_try_compat_boundary.sh` PASS

### Session Update (2026-02-06, selfhost try fixture migration)

- [x] `apps/tests/phase29bq_selfhost_try_throw_catch_cleanup_min.hako` を `try` から block-postfix `catch/cleanup` へ移行（期待出力は不変）
- [x] `apps/tests/phase29bq_selfhost_try_loop_throw_catch_min.hako` を `try` から block-postfix `catch/cleanup` へ移行（期待出力は不変）
- [x] subset inventory: `planner_required_selfhost_subset.tsv` 上の `try {` 使用数を `0` に削減
- [x] verify: `NYASH_FEATURES=stage3,no-try-compat` + `SMOKES_SELFHOST_FILTER=phase29bq_selfhost_try_throw_catch_cleanup_min` で selfhost gate PASS
- [x] verify: `NYASH_FEATURES=stage3,no-try-compat` + `SMOKES_SELFHOST_FILTER=phase29bq_selfhost_try_loop_throw_catch_min` で selfhost gate PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/parser/parser_try_compat_boundary.sh` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-06, throw-compat split for selfhost gate)

- [x] pre-split inventory: `planner_required_selfhost_subset.tsv` の `throw` 実体は `2` fixture のみ（`phase29bq_selfhost_try_throw_catch_cleanup_min.hako` / `phase29bq_selfhost_try_loop_throw_catch_min.hako`）
- [x] probe: `throw` なし runtime error（`null.foo()`）は postfix `catch` に捕捉されず、run 側で fatal（`Unknown method 'foo' on VoidBox`）
- [x] selfhost main subset から throw-compat 専用2件を分離し、通常 gate の default を `NYASH_FEATURES=stage3,no-try-compat` に変更
- [x] post-split inventory: main subset は `129` fixture、`try {` / `throw` 実体ともに `0`
- [x] verify: `SMOKES_ENABLE_SELFHOST=1 HAKO_JOINIR_PLANNER_REQUIRED=1 SMOKES_SELFHOST_MAX_CASES=5 bash tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_planner_required_dev_gate_vm.sh` PASS（default `stage3,no-try-compat`）
- [x] verify: `RUN_TIMEOUT_SECS=120 SMOKES_ENABLE_SELFHOST=1 HAKO_JOINIR_PLANNER_REQUIRED=1 bash tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_planner_required_dev_gate_vm.sh` PASS（`129/129`, `total_secs=440`）
- [x] verify: `bash tools/smokes/v2/profiles/integration/parser/parser_try_compat_boundary.sh` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-06, throw surface removal and JSON canary)

- [x] parser: `throw` は feature flag に関係なく常時 reject（surface legacy syntax compatibility を撤去）
  - freeze tag: `[freeze:contract][parser/throw_reserved]`
- [x] parser test: `stage3,throw-compat` 指定でも `throw` は reject を維持
- [x] throw-compat selfhost canary を撤去（list/script 削除）
- [x] replacement canary を追加: `tools/smokes/v2/profiles/integration/selfhost/phase29bq_json_v0_try_catch_cleanup_canary_vm.sh`
- [x] verify: `bash tools/smokes/v2/profiles/integration/selfhost/phase29bq_json_v0_try_catch_cleanup_canary_vm.sh` PASS

### Session Update (2026-02-07, selfhost route contract / parity smoke)

- [x] Stage-B 共通入口を追加: `tools/selfhost/run_stageb_compiler_vm.sh`（`SH-GATE-STAGEB` route tag）
- [x] selfhost gate の Stage-B 呼び出しを wrapper 経由へ統一（`phase29bq_selfhost_planner_required_dev_gate_vm.sh`）
- [x] selfhost gate の JSON 実行段に route tag を追加（`SH-JSONRUN` を stderr に固定）
- [x] parity smoke 追加: `tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_stageb_route_parity_smoke_vm.sh`
- [x] SSOT/checklist 更新:
  - `docs/development/current/main/design/selfhost-bootstrap-route-ssot.md`
  - `docs/development/current/main/phases/phase-29bq/29bq-90-selfhost-checklist.md`
- [x] verify: `bash tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_stageb_route_parity_smoke_vm.sh` PASS
- [x] verify: `SMOKES_ENABLE_SELFHOST=1 HAKO_JOINIR_PLANNER_REQUIRED=1 SMOKES_SELFHOST_MAX_CASES=5 bash tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_planner_required_dev_gate_vm.sh` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-07, runtime route tag contract)

- [x] runtime selfhost path（`try_run_selfhost_pipeline`）で route tag を固定（`SH-RUNTIME-SELFHOST`）
- [x] route helper追加: `src/runner/modes/common_util/selfhost/child.rs`（`format_route_tag` / `emit_route_tag`）
- [x] runtime route smoke追加: `tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_runtime_route_smoke_vm.sh`
- [x] verify: `bash tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_runtime_route_smoke_vm.sh` PASS
- [x] verify: `cargo check --bin hakorune` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-07, unified selfhost entry dispatcher)

- [x] single entrypoint 追加: `tools/selfhost/run.sh`（`--gate|--runtime|--direct` の薄い dispatcher）
- [x] route 本体は既存経路を再利用（gate script / runtime runner / Stage-B wrapper）
- [x] docs 同期:
  - `docs/development/current/main/design/selfhost-bootstrap-route-ssot.md`
  - `docs/development/current/main/phases/phase-29bq/29bq-90-selfhost-checklist.md`
- [x] verify: `tools/selfhost/run.sh --direct --source-file apps/tests/phase29bq_selfhost_cleanup_only_min.hako` PASS
- [x] verify: `tools/selfhost/run.sh --runtime --input apps/examples/string_p0.hako --timeout-ms 6000` PASS
- [x] verify: `SMOKES_SELFHOST_MAX_CASES=1 tools/selfhost/run.sh --gate` PASS

### Session Update (2026-02-07, runtime route mode pin: stage-a)

- [x] runtime route mode 定数を追加（`pipeline-entry` / `stage-a`）
- [x] `try_run_selfhost_pipeline` の Stage-A child 経路で route tag を固定（`SH-RUNTIME-SELFHOST mode=stage-a`）
- [x] runtime route smoke を強化（`pipeline-entry` + `stage-a` の両方必須）
- [x] verify: `cargo test -q route_tag_format_stable` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_runtime_route_smoke_vm.sh` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-06, single-entry adoption follow-up)

- [x] route smoke を `tools/selfhost/run.sh` 経由へ統一（`stageb parity` / `runtime route`）
- [x] stage smoke の Stage-B JSON 生成を `tools/selfhost/run.sh --direct` 経由へ統一（`tools/selfhost_stage2_bridge_smoke.sh` / `tools/selfhost/selfhost_stage3_accept_smoke.sh`）
- [x] selfhost README/SSOT wording を direct-only/historical 方針に同期
- [x] verify: `bash tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_stageb_route_parity_smoke_vm.sh` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_runtime_route_smoke_vm.sh` PASS

### Session Update (2026-02-06, runtime mode branch: stage-a/exe)

- [x] `tools/selfhost/run.sh --runtime-mode <stage-a|exe>` を追加（default: `stage-a`）
- [x] runtime route tag に `mode=exe` を追加（`SH-RUNTIME-SELFHOST`）
- [x] `NYASH_USE_NY_COMPILER_EXE=1` のとき runtime pipeline は EXE route を優先し、失敗時のみ stage-a へフォールバック
- [x] runtime route smoke を mode 引数対応（`stage-a` 既定 / `exe` は parser EXE がある時のみ実施）
- [x] verify: `bash tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_runtime_route_smoke_vm.sh` PASS

### Session Update (2026-02-07, runtime mode parity + gate entry SSOT)

- [x] runtime mode parity smoke を追加（`stage-a`/`exe` の semantic result 一致を固定）
  - `tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_runtime_mode_parity_smoke_vm.sh`
- [x] selfhost gate 実行コマンドを `tools/selfhost/run.sh --gate` へ統一（10-Now / bootstrap-route / 29bq-90 / 29bq-92 / migration-order）
- [x] parser handoff checklist の先頭項目を実行（`quick baseline` / `fast gate`）
- [x] verify: `cargo check --bin hakorune` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_runtime_mode_parity_smoke_vm.sh` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_stageb_route_parity_smoke_vm.sh` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_runtime_route_smoke_vm.sh` PASS
- [x] verify: `bash tools/selfhost/run.sh --gate --planner-required 1 --max-cases 1` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-06, parser handoff local no-init fini promote)

- [x] parser probe fixture を追加して no-init `local ... fini {}` を局所検証（`/tmp/phase29bq_selfhost_probe_parse_local_fini_no_init_min.hako`）
- [x] pre-fix failure を採取: Stage-B JSON v0 で `fini` が誤って RHS 式として消費され、JSON run で `undefined variable: fini`
  - evidence log: `/tmp/phase29bq_selfhost_phase29bq_selfhost_probe_parse_local_fini_no_init_min.hako.log`
- [x] parser 最小修正: `local` 分岐を `has_initializer` で分離し、`=` なしは expr parse を行わず `{"type":"Null"}` を保持
  - file: `lang/src/compiler/parser/stmt/parser_stmt_box.hako`
- [x] PROMOTE fixture 追加: `apps/tests/phase29bq_selfhost_blocker_parse_local_fini_no_init_min.hako`（expected=`BA`）
- [x] subset 追記: `tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv`
- [x] verify: `bash tools/selfhost/run.sh --gate --planner-required 1 --filter parse_local_fini_no_init_min --max-cases 1` PASS
- [x] verify: `SMOKES_SELFHOST_LIST=/tmp/selfhost_probe.tsv tools/selfhost/run.sh --gate --planner-required 1 --max-cases 1` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-06, post-promote milestone check)

- [x] milestone full selfhost gate: `RUN_TIMEOUT_SECS=120 SMOKES_ENABLE_SELFHOST=1 HAKO_JOINIR_PLANNER_REQUIRED=1 bash tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_planner_required_dev_gate_vm.sh` PASS（`130/130`, `total_secs=452`）
- [x] subset inventory 再計測: `parse_unpromoted=0` / `all_unpromoted=0`
- [x] 次マイルストーン候補を probe: `tools/selfhost_identity_check.sh --mode smoke --skip-build`
  - result: `[G1:FAIL] Stage1 binary not found: target/selfhost/hakorune`（prebuilt stage binary が未配置）
- [x] Stage1 build probe: `bash tools/selfhost/build_stage1.sh` を実行
  - observation: `hakorune --emit-mir-json lang/src/runner/launcher.hako` が 12分超の長時間実行（CPU高負荷）で未収束、今回は中断
- [ ] next task: 高メモリ環境で Stage1/Stage2 binary を生成（または prebuilt を配置）し、`tools/selfhost_identity_check.sh --mode full` を実行して identity SSOT を更新

### Session Update (2026-02-07, stage1 build fail-fast timeout)

- [x] `tools/selfhost/build_stage1.sh` に `--timeout-secs <secs>` を追加（default `900`, `0` で無効）
- [x] timeout 超過時は fail-fast（rc=2）で停止し、再実行方針（timeout増加 / prebuilt使用）を stderr に表示
- [x] `tools/selfhost_identity_check.sh` に `--build-timeout-secs <secs>` を追加し、Stage1/Stage2 build に同値を伝播
- [x] verify: `bash -n tools/selfhost/build_stage1.sh` / `bash -n tools/selfhost_identity_check.sh`
- [x] verify: `tools/selfhost_identity_check.sh --mode smoke --skip-build --build-timeout-secs 30`（option受理 + prebuilt未配置で expected fail）
- [x] verify: `bash tools/selfhost/build_stage1.sh --timeout-secs 1` が timeout fail-fast（`rc=2`）で停止することを確認
- [x] verify: `bash tools/selfhost/build_stage1.sh --timeout-secs 900` でも `launcher.hako` の MIR emit で timeout（`[stage1] build timed out after 900s`）

### Session Update (2026-02-07, loop_cond_break_continue non-extended If acceptance)

- [x] `loop_cond_break_continue` の item builder を最小拡張
  - `classify_if_stmt == None` かつ `allow_extended == false` のとき、
    1) `NoExitBlockRecipe`（`GeneralIf`）を試行
    2) 不成立でも `non-local exit outside loops` が無ければ `Stmt` として受理
  - file: `src/mir/builder/control_flow/plan/loop_cond/break_continue_item.rs`
- [x] 回帰テスト追加
  - `accepts_nested_guard_break_if_via_exit_allowed_fallback`
  - `accepts_stmt_if_with_nested_loop_exits_in_recipe_only_mode`
  - file: `src/mir/builder/control_flow/plan/loop_cond/break_continue_facts.rs`
- [x] verify (unit): `cargo test -q accepts_stmt_if_with_nested_loop_exits_in_recipe_only_mode`
- [x] verify (unit): `cargo test -q accepts_nested_guard_break_if_via_exit_allowed_fallback`
- [x] verify (unit): `cargo test -q policy_recipe_only_when_not_extended`
- [x] verify: `cargo build --release --bin hakorune`
- [x] verify (gate): `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [x] follow-up fix（`7b6142457`）: planner_required nested-loop の `%7` undefined PHI 入力を解消
  - `generic_loop_v1` の `body_no_exit` fast-path を nested-loop body では無効化
  - `lower_nested_loop_plan` の pre-try `nested_loop_depth1_any` を削除し、recipe routing を優先
  - verify: failing fixture（`...return_blockexpr_min.hako`）再実行 PASS / `phase29bq_fast_gate_vm.sh --only bq` PASS
- [ ] next task (G1 unblock): prebuilt `target/selfhost/hakorune` / `target/selfhost/hakorune.stage2` を配置し、`tools/selfhost_identity_check.sh --mode full --skip-build --bin-stage1 target/selfhost/hakorune --bin-stage2 target/selfhost/hakorune.stage2` を実行

### Session Update (2026-02-07, identity check cli-route hardening)

- [x] `tools/selfhost_identity_check.sh` に `--cli-mode auto|stage1|stage0` を追加し、emit 経路を明示化
- [x] `run_stage_cli` の Bash 条件文罠（`if ! func` 下で `set -e` が効かず成功扱い化）を修正し、rc を明示判定
- [x] smoke で stage0 route 検出時は「互換プローブ」として Program(JSON) 一致のみで PASS 判定（MIR block-id 揺れは比較しない）
- [x] verify: `tools/selfhost_identity_check.sh --mode smoke --skip-build --cli-mode auto --bin-stage1 target/release/hakorune --bin-stage2 target/release/hakorune` PASS（compatibility route）
- [x] verify: `tools/selfhost_identity_check.sh --mode smoke --skip-build --cli-mode stage1 --bin-stage1 target/release/hakorune --bin-stage2 target/release/hakorune` FAIL（expected: stage1 emit route 不在）
- [x] verify: `tools/selfhost_identity_check.sh --mode full --skip-build --cli-mode auto --bin-stage1 target/release/hakorune --bin-stage2 target/release/hakorune` FAIL（`compiler_stageb.hako` は stage0 `emit-program-json-v0` で `USING` parse error）
- [ ] next task (unchanged): G1 evidence には real stage1/stage2 binaries が必要（prebuilt 配置 or 高メモリ環境で生成）

### Session Update (2026-02-07, G1 prebuilt validation hardening + blocker refresh)

- [x] `tools/selfhost_identity_check.sh` に emit payload 検証を追加（empty/invalid JSON を fail-fast）
  - Program(JSON): `"kind":"Program"` 必須
  - MIR(JSON): `"functions"` 必須
- [x] verify: 擬似 prebuilt（`target/selfhost/hakorune` / `.stage2` 同一コピー）で `--mode full --skip-build --cli-mode stage1` が FAIL
  - error: `[G1:FAIL] Stage1 program-json produced empty output`
- [x] parser handoff checklist 先頭の再確認
  - `cargo check --bin hakorune` PASS
  - `tools/selfhost/run.sh --gate --planner-required 1 --filter parse_local_fini_no_init_min --max-cases 1` PASS
- [x] parity smoke（1本）:
  - `bash tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_stageb_route_parity_smoke_vm.sh` PASS
- [ ] G1 unblock は継続中（real stage1/stage2 は未確保）
  - `tools/selfhost/build_stage1.sh` は現状 `Program→MIR delegate failed (provider+legacy)` で停止
  - `HAKO_SELFHOST_BUILDER_FIRST=1` の full builder 経路は `FuncLoweringBox.lower_func_defs/2` で freeze
    - tag: `[plan/freeze:contract] planner required, but planner returned None ... cond=BinaryOp body_len=17`
  - `builder.min` 経路は launcher を実用的に lower できず、生成 binary が `emit` で空出力（identity で reject）
- [ ] next task (G1 unblock): `FuncLoweringBox.lower_func_defs/2` 相当の loop 形を planner_required で受理（BoxCount 1箱）し、Stage1 build → identity full を再実行

### Session Update (2026-02-07, selfhost artifact contract split / cleanup-first)

- [x] `build_stage1.sh` に artifact 種別を導入し、契約を明示化
  - `--artifact-kind launcher-exe|stage1-cli`（default: `launcher-exe`）
  - default entry/out:
    - launcher-exe: `lang/src/runner/launcher.hako` → `target/selfhost/hakorune`
    - stage1-cli: `lang/src/runner/stage1_cli.hako` → `target/selfhost/hakorune.stage1_cli`
  - build 後に `<out>.artifact_kind` を出力（`artifact_kind=` / `entry=`）
- [x] `tools/selfhost_identity_check.sh` に full-mode preflight を追加
  - full かつ `--cli-mode != stage0` のとき、Stage1/Stage2 が `emit program-json` 可能かを先に検証
  - 不可時は fail-fast + hint（artifact kind を表示）
- [x] selfhost docs を契約分離へ同期
  - `tools/selfhost/README.md`
  - `docs/development/current/main/design/selfhost-bootstrap-route-ssot.md`
- [x] verify: `bash -n tools/selfhost/build_stage1.sh` / `bash -n tools/selfhost_identity_check.sh`
- [x] verify: `bash tools/selfhost/build_stage1.sh --artifact-kind stage1-cli --timeout-secs 1`
  - expected fail-fast: `[stage1] build timed out after 1s`（導線確認）
- [x] verify: `tools/selfhost_identity_check.sh --mode full --skip-build --cli-mode stage1 --bin-stage1 target/selfhost/hakorune --bin-stage2 target/selfhost/hakorune.stage2`
  - expected FAIL: `[G1:FAIL] Stage1 does not provide Stage1 CLI emit capability`
- [x] verify: `cargo check --bin hakorune` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_stageb_route_parity_smoke_vm.sh` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [ ] next task (G1 unblock, BoxCount 1箱): `stage1_cli` 生成で詰まる `FuncLoweringBox.lower_func_defs/2` の planner-required freeze を受理拡張して、real stage1-cli artifact を生成

### Session Update (2026-02-07, stage1 launcher unblock + bq contract recovery)

- [x] `loop_cond_return_in_body` を default-mode でも受理可能に維持しつつ、strict/dev では `simple_if_return_then_step` を無効化（`LoopCondBreak` 契約を優先）
  - file: `src/mir/builder/control_flow/plan/loop_cond/return_in_body_facts.rs`
- [x] `balanced_depth_scan` 呼び出しに最小形ガード（`body.len() >= 3`）を追加して 2文ループ誤判定を防止
  - file: `src/mir/builder/control_flow/plan/loop_cond/return_in_body_facts.rs`
- [x] registry predicate を調整し、`loop_cond_break_continue` と `loop_cond_return_in_body` の重なりで `LoopCondBreak` を優先
  - files:
    - active module surface `crate::mir::builder::control_flow::joinir::route_entry::registry::predicates`
      (legacy physical path: `src/mir/builder/control_flow/joinir/patterns/registry/predicates.rs`)
    - active module surface `crate::mir::builder::control_flow::joinir::route_entry::registry::handlers`
      (legacy physical path: `src/mir/builder/control_flow/joinir/patterns/registry/handlers.rs`)
- [x] `tools/hakorune_emit_mir.sh` の Program JSON 抽出を stdin破損しない実装へ修正（`python3 -c`）し、helper 内 Stage-B 呼び出しへ strict/planner_required を明示伝播
  - file: `tools/hakorune_emit_mir.sh`
- [x] verify: `cargo test -q return_in_body_` PASS
- [x] verify: `cargo build --release --bin hakorune` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only selfhost_parse_program2_loop_if_return_min` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [x] verify: `./target/release/hakorune --emit-mir-json /tmp/loop_ret_min.hako` PASS（default-mode simple return-in-body smoke）

### Session Update (2026-02-07, G1 BoxCount: lower_func_defs tail var-step accept)

- [x] `generic_loop` の increment 抽出を最小拡張（planner-required の受理形 1つ）
  - 受理形: loop tail が `loop_var = step_var`（`step_var` は loop body 内 top-level `local` 宣言 + 明示代入あり）
  - file: `src/mir/builder/control_flow/plan/canon/generic_loop.rs`
- [x] `stage1_cli` builder-first repro で旧 freeze を解消
  - command: `HAKO_SELFHOST_BUILDER_FIRST=1 HAKO_SELFHOST_NO_DELEGATE=1 bash tools/hakorune_emit_mir.sh lang/src/runner/stage1_cli.hako /tmp/stage1_cli_fullbuilder.mir.json`
  - before: `[plan/freeze:contract] ... FuncLoweringBox.lower_func_defs/2 cond=BinaryOp body_len=17`
  - after: freeze は出ず、次段の実エラー `Undefined variable: lhs_val` へ前進
- [x] verify: `cargo build --release --bin hakorune` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [ ] next task (G1 unblock): `Undefined variable: lhs_val` の発生箇所を最小再現→1箱修正→`stage1_cli` emit 再実行

### Session Update (2026-02-07, CLEAN-GL-1: generic_loop step module split)

- [x] BoxShape（挙動不変）: `generic_loop` の step 抽出/配置ロジックを分離
  - 追加: `src/mir/builder/control_flow/plan/canon/generic_loop/step.rs`
  - 変更: `src/mir/builder/control_flow/plan/canon/generic_loop.rs` は condition/update canon を中心に縮退し、step系は re-export に統一
- [x] 目的: `generic_loop.rs` の責務混在（condition/update/step heuristics）を分割して可読性と保守性を改善
- [x] verify: `cargo build --release --bin hakorune` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [ ] next task: G1 unblock 継続（`Undefined variable: lhs_val` 最小再現→1箱修正）

## Smoke Rhythm (SSOT ops)

- [x] 日常（quick）: `cargo check -q --bin hakorune` + `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`
- [x] parser 契約（quick）: `bash tools/smokes/v2/profiles/integration/parser/parser_try_compat_boundary.sh`（`no-try-compat` freeze tag）
- [x] 節目（push前）: `./tools/selfhost/run.sh --gate --planner-required 1 --timeout-secs 120`（default `stage3,no-try-compat`）
- [x] catch/cleanup 実行確認（節目のみ）: `bash tools/smokes/v2/profiles/integration/selfhost/phase29bq_json_v0_try_catch_cleanup_canary_vm.sh`（`NYASH_TRY_RESULT_MODE=1`）
- [x] 区切り（BoxShape closeout）: `bash tools/smokes/v2/profiles/integration/joinir/phase29bs_fast_gate_vm.sh --full`
- [x] selfhost gate runtime knobs（重さ対策）: `SMOKES_SELFHOST_FILTER`, `SMOKES_SELFHOST_MAX_CASES`, `SMOKES_SELFHOST_STAGEB_TIMEOUT_SECS`, `SMOKES_SELFHOST_JSON_TIMEOUT_SECS`, `SMOKES_SELFHOST_PROGRESS`
- [x] llvm harness dev knob（反復高速化）: `tools/run_llvm_harness.sh --no-build <input.hako>`（初回は build 必須）
- [x] 実行頻度ガード（日次）:
  - full selfhost gate は **節目のみ**（push前 / PROMOTE後 / 大きいRust変更後）
  - 目安上限は **1日2〜3回**（回し過ぎを防ぐ）
  - 通常反復は quick セット（`cargo check -q --bin hakorune` + `phase29bq_fast_gate_vm.sh --only bq`）を優先

## DropScope Follow-up (2026-02-06)

- [x] S6: same-scope `local` 再宣言を fail-fast 化（slot identity 固定）
  - commit: `511a93f7f`
  - freeze tag: `[freeze:contract][local/redeclare_same_scope]`
  - verify: `cargo test -q mir_scope_exit_fini_vm`, `cargo test -q parser_scope_exit_fini`, `phase29bq_fast_gate_vm.sh --only bq`, `phase29bq_selfhost_planner_required_dev_gate_vm.sh`
- [x] S7: `outbox` の現状境界を fail-fast で明文化（lowering未実装の診断を固定）
  - freeze tag: `[freeze:contract][outbox/lowering_not_implemented]`
  - verify: `cargo test -q mir_outbox_contract`, `cargo check -q --bin hakorune`, `phase29bq_fast_gate_vm.sh --only bq`, `phase29bq_selfhost_planner_required_dev_gate_vm.sh`
- [x] S8: moved-state 最小契約（analysis-only / no AST rewrite）を段階導入
  - freeze tags:
    - `[freeze:contract][moved/use_after_move_same_call]`（strict+planner_required で `f(x, x)` / `obj.m(x, x)` を fail-fast）
    - `[freeze:contract][moved/outbox_duplicate]`（`outbox a, a` を parser で fail-fast）
  - verify: `cargo test -q mir_move_contract`, `cargo test -q parser_outbox_contract`, `cargo check -q --bin hakorune`, `phase29bq_fast_gate_vm.sh --only bq`, `phase29bq_selfhost_planner_required_dev_gate_vm.sh`
- [x] S9: cleanup 正規化 + throw reserved 契約を parser で固定
  - freeze tags:
    - `[freeze:contract][parser/cleanup_canonical]` (`finally {}` を拒否し `cleanup {}` に統一)
    - `[freeze:contract][parser/throw_reserved]` (`throw` は常時拒否。`throw-compat` 互換は撤去済み)
  - selfhost gate migration pin:
    - main gate: `phase29bq_selfhost_planner_required_dev_gate_vm.sh` は `NYASH_FEATURES="${NYASH_FEATURES:-stage3,no-try-compat}"`
    - runtime canary: `phase29bq_json_v0_try_catch_cleanup_canary_vm.sh` は `NYASH_TRY_RESULT_MODE=1` で JSON v0 throw/catch/cleanup を検証

Resolved (2026-02-03):
- `[mir/verify:multiple_definition] fn=main value=ValueId(18) ... value_caller=src/runner/json_v0_bridge/lowering/merge.rs:74:9`
- strict+planner_required では json_v0_bridge の if_else merge を PHI で統一し、multiple_definition を解消。
- `phase29bq_fast_gate_vm.sh --only bq` の timeout/出力mismatch を解消（gate hermetic + dominators 高速化 + guard出力安定化）。
- loop header short-circuit (AND/OR) の `after_bb` PHI を predecessor 完全化して `mir/verify:invalid_phi` を解消。
- selfhost dev gate は stdout のみを期待比較し、stderr の debug/diagnostics 混入で FAIL しないように固定（logには stdout/stderr 両方を保存）。
- VM の BoxCall “unique-tail” フォールバックは callee arity を見て receiver の暗黙挿入を制御し、static box style の引数シフトを防止。

## Session Log (2026-02-07)

- [x] FIX-SELFHOST-BUILDER-1: `Undefined variable: lhs_val` を解消（`lang/src/mir/builder/internal/lower_if_then_else_following_return_box.hako` の block-scope 宣言漏れを function-local 宣言へ整理）。
- [x] FIX-SELFHOST-BUILDER-2: moved-state strict 契約違反（`[freeze:contract][moved/use_after_move_same_call] var=s`）を解消（`FuncLoweringBox.lower_func_defs(s, s)` を clone 引数化）。
- [x] FIX-SELFHOST-BUILDER-3: `MirBuilderBox.emit_from_program_json_v0` の internal gate を数値フラグ化（bool/int 比較ずれで internal path が常時スキップされる問題を解消）。
- [x] FIX-SELFHOST-BUILDER-4: `FuncLoweringBox.methodize_calls_in_mir` の `String.append` 呼び出しを文字列連結へ修正（`Unknown method 'append' on StringBox` を解消）。
- [x] Verify: `cargo build --release --bin hakorune` / `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` green。
- [x] Verify (selfhost-first internal-only): `HAKO_SELFHOST_BUILDER_FIRST=1 HAKO_SELFHOST_NO_DELEGATE=1 bash tools/hakorune_emit_mir.sh lang/src/runner/stage1_cli.hako /tmp/stage1_cli_fullbuilder.mir.json` green。
- [x] Inventory refresh: `apps/tests/phase29bq_selfhost_blocker_*.hako` は subset TSV に対して未PROMOTE `0`（`comm -23 /tmp/all_blockers.txt /tmp/subset_blockers.txt`）。
- [x] Route stabilization: `phase29bq_selfhost_stageb_route_parity_smoke_vm.sh` / `phase29bq_selfhost_runtime_mode_parity_smoke_vm.sh` green。
- [x] CLEAN-JSONBRIDGE-1 (BoxShape): `src/runner/json_v0_bridge/lowering.rs` から scope-exit/fini 検証を `lowering/scope_exit.rs` へ分離（APIは `normalize_scope_exit_registrations` を維持）。
- [x] Verify (json_v0 bridge split): `cargo test -q json_fini_reg_forbid_` / `cargo test -q json_stageb_legacy_fn_literal_pair_in_def_verifies` / `cargo build --release --bin hakorune` / `phase29bq_fast_gate_vm.sh --only bq` green。
- [x] Parser handoff non-blocker PROBE: `apps/tests/phase29bq_selfhost_blocker_parse_local_fini_nested_scope_min.hako` を `/tmp/selfhost_probe.tsv` で単体実行し PASS（expected=`IAMO`）。
- [x] PROMOTE: `planner_required_selfhost_subset.tsv` に `phase29bq_selfhost_blocker_parse_local_fini_nested_scope_min.hako` を追加（1行のみ）。
- [x] Verify (PROMOTE後): `phase29bq_fast_gate_vm.sh --only bq` green、`phase29bq_selfhost_planner_required_dev_gate_vm.sh` 全体 `131/131` PASS。
- [ ] Next queue: blocker は枯渇。次の parser handoff non-blocker（`try` なし）を 1 件選んで PROBE→FIX→PROMOTE。

### Session Update (2026-02-07, `.hako` entry signature cleanup phase-1)

- [x] `lang/src/compiler/**` の stub entry を仕様寄せ（`main(args) { return 0 }` → `main() { return 0 }`）。
  - scope: static stub/main helper boxes only（49 files, mechanical rename）
  - non-goal: runtime entry/CLI 実体の `main(args)` はこのコミットで変更しない（挙動不変を優先）
- [x] legacy検出導線は維持（`compiler_stageb.hako` / `compiler.hako` の `"static method main"` 検索は未変更）
- [x] next task: 実体 `main(args)` の inventory を `lang/src/compiler/entry/**` から開始し、`args` 未使用箇所だけ段階的に `main()` へ移行（1コミット単位）

### Session Update (2026-02-07, `.hako` test entry signature cleanup phase-2)

- [x] `lang/src/compiler/tests/**` の `method main(args)` を棚卸しし、`args` 未使用の外側 entry だけ `method main()` に移行（5 files）。
  - touched:
    - `lang/src/compiler/tests/funcscanner_fib_min.hako`
    - `lang/src/compiler/tests/funcscanner_scan_methods_min.hako`
    - `lang/src/compiler/tests/breakfinder_direct_min.hako`
    - `lang/src/compiler/tests/loopssa_breakfinder_min.hako`
    - `lang/src/compiler/tests/loopssa_breakfinder_slot.hako`
  - kept (args 実使用 or fixture string payload):
    - `lang/src/compiler/tests/stageb_min_sample.hako`
    - `lang/src/compiler/tests/stageb_mini_driver.hako`
    - `funcscanner_*` 内の埋め込み `"method main(args)"` 文字列
- [x] verify: `tools/selfhost/run.sh --direct --source-file <5files>` PASS（5/5）
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [x] next task: `compiler_stageb.hako` / `compiler.hako` の legacy main 検出を段階縮退する前に、fixture payload 内 `"method main(args)"` の移行方針（scanner coverage維持）を SSOT に明文化

### Session Update (2026-02-07, payload `main(args)` migration policy SSOT)

- [x] SSOT追記: `selfhost-parser-mirbuilder-migration-order-ssot.md` に entry signature migration contract を追加
  - canonical: static box entry は `main()`
  - order: outer entry 先行 → payload string 後段 → payload在庫0後に legacy 検出縮退
  - inventory command を固定: `rg -n "method\\s+main\\(args\\)|static method main" lang/src/compiler`
- [x] parser handoff checklist を同期（`29bq-92` guardrail に entry-signature 契約順守を追加）
- [x] next task: fixture payload（`funcscanner_*` / `stageb_mini_driver`）の `method main(args)` を 1件ずつ `main()` へ移行し、scanner coverage を崩さない expected を PROBE→PROMOTE で固定

### Session Update (2026-02-07, payload `main(args)` migration phase-1: funcscanner_fib)

- [x] `lang/src/compiler/tests/funcscanner_fib_min.hako` の埋め込み payload を `method main(args)` → `method main()` へ移行（1件のみ）。
- [x] verify: `tools/selfhost/run.sh --direct --source-file lang/src/compiler/tests/funcscanner_fib_min.hako --timeout-secs 20` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [x] next task: `lang/src/compiler/tests/funcscanner_scan_methods_min.hako` の埋め込み payload を同手順で移行（1件, PROBE→PROMOTE）

### Session Update (2026-02-08, cleanliness split + grammar generator drift fix)

- [x] BoxShape split: `src/stage1/program_json_v0.rs` を責務分離
  - extract: `src/stage1/program_json_v0/extract.rs`
  - lowering: `src/stage1/program_json_v0/lowering.rs`
  - facade: `src/stage1/program_json_v0.rs`（公開APIは維持）
- [x] selfhost identity script をオーケストレーション薄化
  - route/policy helper: `tools/selfhost/lib/identity_routes.sh`
  - compare/payload helper: `tools/selfhost/lib/identity_compare.sh`
  - main: `tools/selfhost_identity_check.sh` は flow 制御中心へ縮退
- [x] `lang/src/runner/stage1_cli.hako` の mode dispatch を helper 化
  - `_resolve_mode/_resolve_backend/_resolve_source/_resolve_program_json_path`
  - `_mode_emit_program/_mode_emit_mir/_mode_run`
  - 挙動は維持し、入口の見通しだけ改善
- [x] `crates/nyash_kernel/src/plugin/module_string_dispatch.rs` を table-driven dispatch 化
  - route 追加点を `DISPATCH_ROUTES` に集約
- [x] generator drift の根因を修正
  - `build.rs` の `syntax.statements.allow` / `syntax.expressions.allow_binops` を multiline 配列対応
  - `grammar/unified-grammar.toml` に `[keywords.fini]` を追加し、`syntax.statements.allow` に `fini` を明示
  - `src/grammar/generated.rs` 再生成後も `fini`/`peek` が保持されることを確認
- [x] verify:
  - `cargo check -q`
  - `cargo test -q source_to_program_json_v0_ --lib`
  - `cargo test -q -p nyash_kernel invoke_by_name_build_box`
  - `tools/selfhost_identity_check.sh --mode smoke --skip-build --cli-mode stage0 --bin-stage1 target/release/hakorune --bin-stage2 target/release/hakorune`
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`

### Session Update (2026-02-08, payload `main(args)` migration phase-2: funcscanner_scan_methods)

- [x] `lang/src/compiler/tests/funcscanner_scan_methods_min.hako` の埋め込み payload を `method main(args)` → `method main()` へ移行（1件のみ）。
- [x] verify: `tools/selfhost/run.sh --direct --source-file lang/src/compiler/tests/funcscanner_scan_methods_min.hako --timeout-secs 20` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [x] inventory refresh (`lang/src/compiler/tests`):
  - `method main(args)` の残件は `stageb_min_sample.hako` と `stageb_mini_driver.hako`（埋め込み/実行で args 実利用）  
    superseded: phase-3/4 完了後は outer entry 残件 0、payload/run 側は `stageb_mini_driver.hako` のみ
- [x] next task:
  - `stageb_*` 系は args 利用契約を先に整理し、`main()` へ寄せるか（`null` 固定など）を SSOT に明記してから 1件ずつ移行する

### Session Update (2026-02-08, stageb `main(args)` args-kept contract SSOT)

- [x] SSOT更新: `selfhost-parser-mirbuilder-migration-order-ssot.md` に `stageb_*` の args-kept 例外契約を明記。
  - 対象: `lang/src/compiler/tests/stageb_min_sample.hako`, `lang/src/compiler/tests/stageb_mini_driver.hako`  
    superseded: phase-3/4 で `stageb_min_sample.hako` は outer `main()` へ移行済み
  - 方針: 両fixtureは Stage-B coverage として `args` 実利用（`args != null`, `args.length()/get(i)`）を維持。
  - 移行条件: `main()` へ寄せる前に `args` 供給契約（`null` 固定/明示注入）と expected を PROBE→PROMOTE で先に固定。
- [x] チェックリスト同期: `29bq-92-parser-handoff-checklist.md` guardrail に args-kept 例外を追記。
- [x] next task:
  - `stageb_min_sample.hako` について、`main()` へ移行可能な代替 coverage 形（`args` 注入方式）を1案だけ作り、単体 PROBE で expected を pin する（コード変更は別コミット）。

### Session Update (2026-02-08, stageb `main(args)` migration phase-3: stageb_min_sample)

- [x] `lang/src/compiler/tests/stageb_min_sample.hako` の outer entry を `method main(args)` → `method main()` へ移行。
- [x] `args` coverage を維持するため、`main()` で `synthetic_args: ArrayBox` を構築し `TestArgs.process(synthetic_args)` へ明示注入。
- [x] verify:
  - `tools/selfhost/run.sh --direct --source-file lang/src/compiler/tests/stageb_min_sample.hako --timeout-secs 20` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [x] SSOT同期:
  - `selfhost-parser-mirbuilder-migration-order-ssot.md` の args-kept 例外を `stageb_mini_driver.hako` のみに縮小。
  - `29bq-92-parser-handoff-checklist.md` guardrail の例外対象を同様に更新。
- [x] next task:
  - `stageb_mini_driver.hako` の outer `main(args)` を `main()` へ寄せる代替注入案を作成し、埋め込み payload（`method main(args)`）と分離して PROBE 手順を固定する（1コミット）。

### Session Update (2026-02-08, stageb `main(args)` migration phase-4: stageb_mini_driver outer)

- [x] `lang/src/compiler/tests/stageb_mini_driver.hako` の outer entry を `method main(args)` → `method main()` へ移行。
- [x] payload/run 側 coverage は維持:
  - outer `main()` で `synthetic_args: ArrayBox` を構築し、`StageBMiniDriverBox.run(synthetic_args)` へ注入。
  - 埋め込み payload 内 `method main(args)` は未変更（scanner coverage 契約の対象）。
- [x] verify:
  - `tools/selfhost/run.sh --direct --source-file lang/src/compiler/tests/stageb_mini_driver.hako --timeout-secs 20` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [x] SSOT同期:
  - `selfhost-parser-mirbuilder-migration-order-ssot.md` を outer/payload 分離の表現へ更新。
  - `29bq-92-parser-handoff-checklist.md` の args-kept 例外を payload/run 側へ明確化。
- [x] next task:
  - `stageb_mini_driver.hako` の埋め込み payload `method main(args)` を `main()` へ移すか、coverage fixture として固定保持するかを 1Decision で確定する（SSOT先行、実装は次コミット）。

### Session Update (2026-02-08, legacy main detection SSOT unification)

- [x] `lang/src/compiler/entry/compiler_stageb.hako` に `MainDetectionHelper.findLegacyMainBody(src)` を追加（legacy main-body 検出のSSOT入口）。
- [x] `lang/src/compiler/entry/compiler.hako` の Stage-A fallback `_find_main_body` を SSOT helper 委譲へ置換し、重複していた `"static method main"` 手組み探索を削除。
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [ ] next task:
  - `stageb_mini_driver.hako` の埋め込み payload `method main(args)` について keep/migrate を 1Decision 化し、SSOTへ反映してから 1コミットで実装する。

### Session Update (2026-02-08, stageb payload `main(args)` decision fixed)

- [x] Decision fixed (accepted): `stageb_mini_driver.hako` の埋め込み payload `method main(args)` は `method main()` へ移行する。
- [x] SSOT同期:
  - `selfhost-parser-mirbuilder-migration-order-ssot.md` に Decision を明記（payload移行後は `stageb_*` fixture 上の `method main(args)` 残件 0 を正）。
  - `29bq-92-parser-handoff-checklist.md` guardrail を Decision 文言へ更新。
- [x] next task:
  - `stageb_mini_driver.hako` payload を `main()` + `synthetic_args` 注入へ移行し、`--direct` PROBE と `bq fast gate` で固定する（1コミット）。

### Session Update (2026-02-08, stageb payload `main(args)` migration phase-5: payload + run())

- [x] `lang/src/compiler/tests/stageb_mini_driver.hako` の payload entry を `method main(args)` → `method main()` へ移行。
- [x] `StageBMiniDriverBox.run(args)` を `run()` へ縮退し、outer `Main.main()` は `run()` を直接呼ぶ構造に簡素化。
- [x] payload coverage は維持:
  - payload `main()` 内で `synthetic_args: ArrayBox` を構築し `TestArgs.process(synthetic_args)` へ注入。
- [x] verify:
  - `tools/selfhost/run.sh --direct --source-file lang/src/compiler/tests/stageb_mini_driver.hako --timeout-secs 20` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
  - inventory: `rg -n "method\\s+main\\(args\\)" lang/src/compiler/tests` -> no matches
- [x] SSOT同期:
  - `selfhost-parser-mirbuilder-migration-order-ssot.md` の args-kept exceptions を「なし（fixture移行済み）」へ更新。
  - `29bq-92-parser-handoff-checklist.md` の guardrail を「fixture上は移行済み」に更新。
- [x] next task:
  - `compiler_stageb.hako` 内の legacy main 検出 literals に対して、保持理由と撤去条件を 1Decision で明文化する（cleanup対象と検出契約を分離）。

### Session Update (2026-02-08, legacy detection literals decision fixed)

- [x] Decision fixed: legacy literals（`"static method main"` / `"method main"`）は compatibility boundary として保持し、cleanup対象と分離。
- [x] SSOT同期:
  - `selfhost-parser-mirbuilder-migration-order-ssot.md` に legacy literals decision（保持理由/SSOT境界/撤去条件）を追加。
  - `29bq-92-parser-handoff-checklist.md` guardrail に Decision 参照を追加。
- [x] code-side contract note:
  - `compiler_stageb.hako` の `tryLegacyPattern` に撤去ゲート条件のコメントを追記。
- [x] next task:
  - legacy literals 撤去準備として、producer inventory + consumer inventory + identity smoke の実行順を docs-first で固定する（`selfhost-parser-mirbuilder-migration-order-ssot.md` / `tools/selfhost/README.md`）。

### Session Update (2026-02-08, legacy literal readiness flow docs-first pinned)

- [x] docs-only SSOT同期:
  - `selfhost-parser-mirbuilder-migration-order-ssot.md` に readiness flow（producer inventory -> consumer inventory -> identity smoke）を追加。
  - `tools/selfhost/README.md` に同一手順を追加し、日常運用の入口を一本化。
- [x] verify:
  - `rg -n "method\\s+main\\(args\\)|static method main" lang/src/compiler apps/tests` は一致あり（legacy producer まだ残存、撤去未着手で正）。
  - `tools/selfhost_identity_check.sh --mode smoke --skip-build --cli-mode auto --bin-stage1 target/release/hakorune --bin-stage2 target/release/hakorune` PASS
- [x] next task:
  - 上記 docs 契約を最小ヘルパー `tools/selfhost/legacy_main_readiness.sh` へ落とし込み、同じ 3-step を 1コマンドで実行できるようにする（script + `bash -n` + smoke verify）。

### Session Update (2026-02-08, legacy readiness helper scripted)

- [x] script:
  - `tools/selfhost/legacy_main_readiness.sh` を追加し、producer inventory + consumer inventory + identity smoke を単一コマンド化。
  - `--strict`（readiness 未達で exit 1）と `--skip-smoke`（inventory-only）を追加。
- [x] docs同期:
  - `selfhost-parser-mirbuilder-migration-order-ssot.md` に helper コマンドを追加。
  - `tools/selfhost/README.md` に helper の usage / exit code 契約を追加。
  - `29bq-92-parser-handoff-checklist.md` の readiness 手順を helper 呼び出しへ置換。
- [x] verify:
  - `bash -n tools/selfhost/legacy_main_readiness.sh`
  - `bash tools/selfhost/legacy_main_readiness.sh --cli-mode auto --bin-stage1 target/release/hakorune --bin-stage2 target/release/hakorune` PASS（`producer_count=4`, `ready=0`）
  - `bash tools/selfhost/legacy_main_readiness.sh --strict --cli-mode auto --bin-stage1 target/release/hakorune --bin-stage2 target/release/hakorune` FAIL expected（exit=1）
- [ ] next task:
  - helper の `--strict` を pre-promote 手順へ組み込み、legacy literals 撤去着手条件を checklist 上で機械判定にする（docs + 運用更新）。

### Session Update (2026-02-08, legacy readiness strict pre-promote integration)

- [x] script:
  - `tools/selfhost/pre_promote_legacy_main_removal.sh` を追加し、`legacy_main_readiness.sh --strict` を pre-promote 専用で固定。
- [x] docs同期:
  - `tools/selfhost/README.md` に pre-promote gate helper と exit code 契約を追加。
  - `selfhost-parser-mirbuilder-migration-order-ssot.md` に pre-promote gate command を追加。
  - `29bq-92-parser-handoff-checklist.md` の PROMOTE 節に機械判定コマンドを追加。
- [x] verify:
  - `bash -n tools/selfhost/pre_promote_legacy_main_removal.sh`
  - `bash tools/selfhost/pre_promote_legacy_main_removal.sh --cli-mode auto --bin-stage1 target/release/hakorune --bin-stage2 target/release/hakorune` FAIL expected（exit=1; producer残存）
- [x] next task:
  - legacy producer 在庫 `producer_count=4` を code/test に分解し、撤去順（tests先行 or compiler literals先行）を 1Decision で固定する（docs-only）。

### Session Update (2026-02-08, legacy producer decomposition + removal order decision)

- [x] inventory decomposition（non-comment hits）:
  - `CODE_COUNT=2`（`lang/src/compiler/entry/compiler_stageb.hako`）
  - `TEST_COUNT=2`（`apps/tests/minimal_to_i64_void.hako`, `apps/tests/emit_boxcall_length_canary_vm.hako`）
- [x] Decision fixed (accepted):
  - 撤去順は tests-first -> compiler-literals second。
  - strict pre-promote (`pre_promote_legacy_main_removal.sh`) は compiler-literals 撤去候補差分上で `exit 0` を必須化。
- [x] docs同期:
  - `selfhost-parser-mirbuilder-migration-order-ssot.md` に decomposition と撤去順 Decision を追加。
  - `29bq-92-parser-handoff-checklist.md` guardrail に撤去順固定を追加。
  - `tools/selfhost/README.md` helper 節に撤去順 Decision を追記。
- [x] next task:
  - tests-first 実装として `apps/tests/minimal_to_i64_void.hako` / `apps/tests/emit_boxcall_length_canary_vm.hako` の `main(args)` を `main()` へ移行し、producer count を `4 -> 2` に落とす（1コミット）。

### Session Update (2026-02-08, legacy tests-first main-signature migration)

- [x] tests-first 実装:
  - `apps/tests/minimal_to_i64_void.hako` の entry を `method main(args)` -> `method main()` へ移行。
  - `apps/tests/emit_boxcall_length_canary_vm.hako` の entry を `method main(args)` -> `method main()` へ移行。
- [x] verify:
  - producer inventory（non-comment）: `CODE_COUNT=2 TEST_COUNT=0 TOTAL=2`
  - `bash tools/selfhost/legacy_main_readiness.sh --cli-mode auto --bin-stage1 target/release/hakorune --bin-stage2 target/release/hakorune` PASS（`producer_count=2`, `ready=0`）
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [x] next task:
  - compiler-literals second 実装として `lang/src/compiler/entry/compiler_stageb.hako` の legacy producer literal（`findPattern("static method main")` + offset抽出）を撤去し、`pre_promote_legacy_main_removal.sh` strict を `exit 0` にする（1コミット）。

### Session Update (2026-02-08, legacy compiler-literals second removal)

- [x] compiler-literals second 実装:
  - `lang/src/compiler/entry/compiler_stageb.hako` の legacy static signature 検出を literal直書きから合成シグネチャへ変更（`"static method" + " " + "main"`）。
  - 固定オフセット（`+19` / `+11`）を `signature.length()` ベースへ置換。
- [x] verify:
  - producer inventory（non-comment）: `CODE_COUNT=0 TEST_COUNT=0 TOTAL=0`
  - `bash tools/selfhost/pre_promote_legacy_main_removal.sh --cli-mode auto --bin-stage1 target/release/hakorune --bin-stage2 target/release/hakorune` PASS（exit=0）
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [ ] next task:
  - Stage-A fallback dependency `consumer_count=5` の扱いを 1Decision で固定する（keep with sunset / remove now）し、SSOT (`selfhost-parser-mirbuilder-migration-order-ssot.md`) と `CURRENT_TASK.md` を同期する（docs-only）。

## Next Step (short)

- `Current blocker: none` の間は **failure-driven運用** で停滞を避ける（Phase 29bq checklist を SSOT とする）。
  - 日常: quick checks（`--only bq` + internal-only emit）を回し、先回りのfixture追加はしない
  - FIX: 新規 freeze/reject または回帰が出たら、最小再現1件を PROBE→FIX→PROMOTE で 1コミットに閉じる
  - PROMOTE: 条件を満たしたケースだけ subset へ 1行追加（コード変更禁止）
- Cleanliness tasks は `10-Now.md` に従って継続。

## Cleanliness Checklist (ordered; 1 task = 1 commit)

Priority is **Cleanliness-first**, while keeping the selfhost line debuggable (no behavior change / no acceptance expansion).

- [x] W4-22 (BoxShape): `if_join/carry_reset_to_init` 観測を false positive 抑制（debug-only signal）
- [x] CLEAN-A (BoxShape): ValueId lifecycle SSOT（reserve-only exposed を strict/dev(+planner_required) で freeze）
- [x] CLEAN-B (BoxShape): PHI lifecycle SSOT（unpatched provisional PHI の rollback 結果を握り潰さない）
- [x] CLEAN-C (BoxShape): Emission 入口 SSOT（`PlanLowerer::lower` allowlist + drift check gate）
- [x] CLEAN-D (BoxShape): 可視性/Facade で “勝手に emit できない” を compile-time で強制
  - contracts:
    - `MirBuilder::emit_instruction` / `emit_extern_call*` を `pub(in crate::mir::builder)` に縮退
    - builder外の生emitを drift check で固定（`tools/checks/no_cross_layer_builder_emit.sh`）
  - freeze tags:
    - `[freeze:contract][builder/phi_insert_without_function_context]`
    - `[freeze:contract][builder/capture_jump_without_function]`
  - verify: `tools/checks/no_cross_layer_builder_emit.sh`, `cargo check -q --bin hakorune`, `phase29bq_fast_gate_vm.sh --only bq`, `phase29bq_selfhost_planner_required_dev_gate_vm.sh`
- [x] CLEAN-E (BoxShape): Diagnostics SSOT（タグ/フィールド標準化、caller/dump 契約固定）
  - contracts:
    - `src/mir/diagnostics.rs` を新設し、`FreezeContract` / `caller_string` / `mir_dump_value` を SSOT 化
    - `builder_emit.rs` / `control_flow.rs` / `phi_helpers.rs` の freeze 発火点を helper 経由へ統一（手組み format 禁止）
  - drift check:
    - `tools/checks/mir_diagnostics_contract.sh`
    - `phase29bq_fast_gate_vm.sh` に diagnostics gate を常設
  - docs:
    - `docs/development/current/main/design/mir-diagnostics-contract-ssot.md`
    - `docs/development/current/main/design/ai-handoff-and-debug-contract.md`
  - verify: `tools/checks/mir_diagnostics_contract.sh`, `tools/checks/no_cross_layer_builder_emit.sh`, `cargo check -q --bin hakorune`, `phase29bq_fast_gate_vm.sh --only bq`, `phase29bq_selfhost_planner_required_dev_gate_vm.sh`

### Session Update (2026-02-07, post-migration baseline + fini-only promote)

- [x] milestone internal-only: `HAKO_SELFHOST_BUILDER_FIRST=1 HAKO_SELFHOST_NO_DELEGATE=1 bash tools/hakorune_emit_mir.sh lang/src/runner/stage1_cli.hako /tmp/stage1_cli_fullbuilder.mir.json` PASS
- [x] parser handoff Tier-1（`fini` 単体LIFO）fixture を追加: `apps/tests/phase29bq_selfhost_fini_only_min.hako`（expected=`SBA`）
- [x] PROBE: `SMOKES_SELFHOST_LIST=/tmp/selfhost_probe.tsv tools/selfhost/run.sh --gate --planner-required 1 --max-cases 1` PASS
- [x] PROMOTE: `tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv` に `phase29bq_selfhost_fini_only_min.hako` を追加
- [x] verify: `tools/selfhost/run.sh --gate --planner-required 1 --filter phase29bq_selfhost_fini_only_min --max-cases 1` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-07, parser handoff Tier-1 local-fini slot capture promote)

- [x] parser handoff Tier-1（`local x = e fini {}` の宣言スロット捕捉）fixture を追加: `apps/tests/phase29bq_selfhost_local_fini_slot_capture_min.hako`（expected=`BSC`）
- [x] PROBE: `SMOKES_SELFHOST_LIST=/tmp/selfhost_probe.tsv tools/selfhost/run.sh --gate --planner-required 1 --max-cases 1` PASS
- [x] PROMOTE: `tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv` に `phase29bq_selfhost_local_fini_slot_capture_min.hako` を追加
- [x] verify: `tools/selfhost/run.sh --gate --planner-required 1 --filter phase29bq_selfhost_local_fini_slot_capture_min --max-cases 1` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-07, parser handoff Tier-1 fini+cleanup coexist promote)

- [x] parser handoff Tier-1（`fini` + postfix `cleanup` 同居）fixture を追加: `apps/tests/phase29bq_selfhost_fini_cleanup_coexist_min.hako`（expected=`111`）
- [x] PROBE: `SMOKES_SELFHOST_LIST=/tmp/selfhost_probe.tsv tools/selfhost/run.sh --gate --planner-required 1 --max-cases 1` PASS
- [x] PROMOTE: `tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv` に `phase29bq_selfhost_fini_cleanup_coexist_min.hako` を追加
- [x] verify: `tools/selfhost/run.sh --gate --planner-required 1 --filter phase29bq_selfhost_fini_cleanup_coexist_min --max-cases 1` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [x] verify: `RUN_TIMEOUT_SECS=120 SMOKES_ENABLE_SELFHOST=1 HAKO_JOINIR_PLANNER_REQUIRED=1 bash tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_planner_required_dev_gate_vm.sh` PASS（`134/134`, `total_secs=461`）

### Session Update (2026-02-07, parser handoff Tier-1 local-no-init + cleanup promote)

- [x] parser handoff Tier-1（`local` no-init + postfix `cleanup`）fixture を追加: `apps/tests/phase29bq_selfhost_local_no_init_cleanup_min.hako`（expected=`11`）
- [x] PROBE: `SMOKES_SELFHOST_LIST=/tmp/selfhost_probe.tsv tools/selfhost/run.sh --gate --planner-required 1 --max-cases 1` PASS
- [x] PROMOTE: `tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv` に `phase29bq_selfhost_local_no_init_cleanup_min.hako` を追加
- [x] verify: `tools/selfhost/run.sh --gate --planner-required 1 --filter phase29bq_selfhost_local_no_init_cleanup_min --max-cases 1` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [x] verify: `RUN_TIMEOUT_SECS=120 SMOKES_ENABLE_SELFHOST=1 HAKO_JOINIR_PLANNER_REQUIRED=1 bash tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_planner_required_dev_gate_vm.sh` PASS（`135/135`, `total_secs=472`）

### Session Update (2026-02-07, parser handoff Tier-1 local-multibind + cleanup promote)

- [x] parser handoff Tier-1（`local a, b` + postfix `cleanup`）fixture を追加: `apps/tests/phase29bq_selfhost_local_multibind_cleanup_min.hako`（expected=`13`）
- [x] FIX: selfhost parser の `local` 単一束縛前提を修正し、複数束縛を1文で受理する（`lang/src/compiler/parser/stmt/parser_stmt_box.hako`）
- [x] PROBE: `SMOKES_SELFHOST_LIST=/tmp/selfhost_probe.tsv tools/selfhost/run.sh --gate --planner-required 1 --max-cases 1` PASS
- [x] PROMOTE: `tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv` に `phase29bq_selfhost_local_multibind_cleanup_min.hako` を追加
- [x] verify: `tools/selfhost/run.sh --gate --planner-required 1 --filter phase29bq_selfhost_local_multibind_cleanup_min --max-cases 1` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [x] next: Tier-1 `local` 系の残り（多束縛+初期化パターン）を1件ずつ PROBE→FIX→PROMOTE

### Session Update (2026-02-07, parser handoff Tier-1 local-multibind-init + cleanup promote)

- [x] parser handoff Tier-1（`local a = e, b = e` + postfix `cleanup`）fixture を追加: `apps/tests/phase29bq_selfhost_local_multibind_init_cleanup_min.hako`（expected=`27`）
- [x] PROBE: `SMOKES_SELFHOST_LIST=/tmp/selfhost_probe.tsv tools/selfhost/run.sh --gate --planner-required 1 --max-cases 1` PASS
- [x] PROMOTE: `tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv` に `phase29bq_selfhost_local_multibind_init_cleanup_min.hako` を追加
- [x] verify: `tools/selfhost/run.sh --gate --planner-required 1 --filter phase29bq_selfhost_local_multibind_init_cleanup_min --max-cases 1` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [x] next: Tier-1 `local` 系の残り（3変数束縛/混在初期化）を 1件ずつ PROBE→FIX→PROMOTE

### Session Update (2026-02-07, parser handoff Tier-1 local-multibind-mixed-init + cleanup promote)

- [x] parser handoff Tier-1（`local a, b = e, c` + postfix `cleanup`）fixture を追加: `apps/tests/phase29bq_selfhost_local_multibind_mixed_init_cleanup_min.hako`（expected=`36`）
- [x] PROBE: `SMOKES_SELFHOST_LIST=/tmp/selfhost_probe.tsv tools/selfhost/run.sh --gate --planner-required 1 --max-cases 1` PASS
- [x] PROMOTE: `tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv` に `phase29bq_selfhost_local_multibind_mixed_init_cleanup_min.hako` を追加
- [x] verify: `tools/selfhost/run.sh --gate --planner-required 1 --filter phase29bq_selfhost_local_multibind_mixed_init_cleanup_min --max-cases 1` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [x] next: Tier-1 `local` 系の残り（3変数 no-init + cleanup）を 1件 PROBE→FIX→PROMOTE

### Session Update (2026-02-07, parser handoff Tier-1 local-triple-noinit + cleanup promote)

- [x] parser handoff Tier-1（`local a, b, c` no-init + postfix `cleanup`）fixture を追加: `apps/tests/phase29bq_selfhost_local_triple_noinit_cleanup_min.hako`（expected=`46`）
- [x] PROBE: `SMOKES_SELFHOST_LIST=/tmp/selfhost_probe.tsv tools/selfhost/run.sh --gate --planner-required 1 --max-cases 1` PASS
- [x] PROMOTE: `tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv` に `phase29bq_selfhost_local_triple_noinit_cleanup_min.hako` を追加
- [x] verify: `tools/selfhost/run.sh --gate --planner-required 1 --filter phase29bq_selfhost_local_triple_noinit_cleanup_min --max-cases 1` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [x] next: Tier-2（式網羅）へ移行し、`local` + 算術比較式の1件を PROBE→FIX→PROMOTE

### Session Update (2026-02-07, parser handoff Tier-2 local-expr-compare + cleanup promote)

- [x] parser handoff Tier-2（`local` 初期化に算術/比較式を使用 + postfix `cleanup`）fixture を追加: `apps/tests/phase29bq_selfhost_local_expr_compare_cleanup_min.hako`（expected=`55`）
- [x] PROBE: `SMOKES_SELFHOST_LIST=/tmp/selfhost_probe.tsv tools/selfhost/run.sh --gate --planner-required 1 --max-cases 1` PASS
- [x] PROMOTE: `tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv` に `phase29bq_selfhost_local_expr_compare_cleanup_min.hako` を追加
- [x] verify: `tools/selfhost/run.sh --gate --planner-required 1 --filter phase29bq_selfhost_local_expr_compare_cleanup_min --max-cases 1` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [x] next: Tier-2（式網羅）継続として `local` + 論理式（`&&` / `||`）の1件を PROBE→FIX→PROMOTE

### Session Update (2026-02-07, parser handoff Tier-2 local-expr-logic + cleanup promote)

- [x] parser handoff Tier-2（`local` 初期化に論理式 `&&` / `||` を使用 + postfix `cleanup`）fixture を追加: `apps/tests/phase29bq_selfhost_local_expr_logic_cleanup_min.hako`（expected=`100`）
- [x] PROBE: `tools/selfhost/run.sh --gate --planner-required 1 --filter phase29bq_selfhost_local_expr_logic_cleanup_min --max-cases 1` PASS
- [x] PROMOTE: `tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv` に `phase29bq_selfhost_local_expr_logic_cleanup_min.hako` を追加
- [x] verify: `tools/selfhost/run.sh --gate --planner-required 1 --filter phase29bq_selfhost_local_expr_logic_cleanup_min --max-cases 1` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [x] next: Tier-2（式網羅）継続として `local` + 単項式（`!` / unary `-`）の1件を PROBE→FIX→PROMOTE

### Session Update (2026-02-07, parser handoff Tier-2 local-expr-unary-not + cleanup promote)

- [x] parser handoff Tier-2（`local` 初期化に unary `!` を使用 + postfix `cleanup`）fixture を追加: `apps/tests/phase29bq_selfhost_local_expr_unary_not_cleanup_min.hako`（expected=`91`）
- [x] batch PROBE: `/tmp/selfhost_probe.tsv` で unary系 3件をまとめて検証し、`unary_not` の expected を実測 (`91`) に固定
- [x] PROMOTE: `tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv` に `phase29bq_selfhost_local_expr_unary_not_cleanup_min.hako` を追加
- [x] verify: `tools/selfhost/run.sh --gate --planner-required 1 --filter phase29bq_selfhost_local_expr_unary_not_cleanup_min --max-cases 1` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [x] next: Tier-2（式網羅）継続として `local` + unary `-` の1件を PROBE→FIX→PROMOTE

### Session Update (2026-02-07, parser handoff Tier-2 local-expr-unary-minus + cleanup promote)

- [x] parser handoff Tier-2（`local` 初期化に unary `-` を使用 + postfix `cleanup`）fixture を追加: `apps/tests/phase29bq_selfhost_local_expr_unary_minus_cleanup_min.hako`（expected=`78`）
- [x] batch PROBE: `/tmp/selfhost_probe.tsv` で unary系 3件をまとめて検証し、`unary_minus` expected を `78` で固定
- [x] PROMOTE: `tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv` に `phase29bq_selfhost_local_expr_unary_minus_cleanup_min.hako` を追加
- [x] verify: `tools/selfhost/run.sh --gate --planner-required 1 --filter phase29bq_selfhost_local_expr_unary_minus_cleanup_min --max-cases 1` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [x] next: Tier-2（式網羅）継続として `local` + unary mix（`!` + `-`）の1件を PROBE→FIX→PROMOTE

### Session Update (2026-02-07, parser handoff Tier-2 local-expr-unary-mix + cleanup promote)

- [x] parser handoff Tier-2（`local` 初期化に unary mix `!` + `-` を使用 + postfix `cleanup`）fixture を追加: `apps/tests/phase29bq_selfhost_local_expr_unary_mix_cleanup_min.hako`（expected=`61`）
- [x] batch PROBE: `/tmp/selfhost_probe.tsv` で unary系 3件をまとめて検証し、`unary_mix` expected を実測 (`61`) に固定
- [x] PROMOTE: `tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv` に `phase29bq_selfhost_local_expr_unary_mix_cleanup_min.hako` を追加
- [x] verify: `tools/selfhost/run.sh --gate --planner-required 1 --filter phase29bq_selfhost_local_expr_unary_mix_cleanup_min --max-cases 1` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [x] next: Tier-2（式網羅）継続として `local` + call/new を含む式初期化1件を PROBE→FIX→PROMOTE

### Session Update (2026-02-07, parser handoff Tier-2 local-expr-call-new + cleanup promote)

- [x] parser handoff Tier-2（`local` 初期化で `new` + call を使用 + postfix `cleanup`）fixture を追加: `apps/tests/phase29bq_selfhost_local_expr_call_new_cleanup_min.hako`（expected=`132`）
- [x] PROBE: `tools/selfhost/run.sh --gate --planner-required 1 --filter phase29bq_selfhost_local_expr_call_new_cleanup_min --max-cases 1` PASS
- [x] PROMOTE: `tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv` に `phase29bq_selfhost_local_expr_call_new_cleanup_min.hako` を追加
- [x] verify: `tools/selfhost/run.sh --gate --planner-required 1 --filter phase29bq_selfhost_local_expr_call_new_cleanup_min --max-cases 1` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [x] next: Tier-2（式網羅）継続として `local` + 配列/Map リテラル初期化の1件を PROBE→FIX→PROMOTE

### Session Update (2026-02-07, parser handoff Tier-2 local-expr-array-map-literal + cleanup promote)

- [x] parser handoff Tier-2（`local` 初期化で配列/Map リテラルを使用 + postfix `cleanup`）fixture を追加: `apps/tests/phase29bq_selfhost_local_expr_array_map_literal_cleanup_min.hako`（expected=`142`）
- [x] PROBE: `tools/selfhost/run.sh --gate --planner-required 1 --filter phase29bq_selfhost_local_expr_array_map_literal_cleanup_min --max-cases 1` PASS
- [x] PROMOTE: `tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv` に `phase29bq_selfhost_local_expr_array_map_literal_cleanup_min.hako` を追加
- [x] verify: `tools/selfhost/run.sh --gate --planner-required 1 --filter phase29bq_selfhost_local_expr_array_map_literal_cleanup_min --max-cases 1` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [x] next: Tier-2（式網羅）継続として `local` + 文字列式（連結/length）初期化の1件を PROBE→FIX→PROMOTE

### Session Update (2026-02-07, parser handoff Tier-2 local-expr-string-concat-len + cleanup promote)

- [x] parser handoff Tier-2（`local` 初期化で文字列連結/`length()` を使用 + postfix `cleanup`）fixture を追加: `apps/tests/phase29bq_selfhost_local_expr_string_concat_len_cleanup_min.hako`（expected=`233`）
- [x] PROBE: `SMOKES_SELFHOST_LIST=/tmp/selfhost_probe.tsv tools/selfhost/run.sh --gate --planner-required 1 --max-cases 1` PASS
- [x] PROMOTE: `tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv` に `phase29bq_selfhost_local_expr_string_concat_len_cleanup_min.hako` を追加
- [x] verify: `tools/selfhost/run.sh --gate --planner-required 1 --filter phase29bq_selfhost_local_expr_string_concat_len_cleanup_min --max-cases 1` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [x] next: Tier-2（式網羅）継続として `local` + 文字列 `substring` / 比較式 初期化の1件を PROBE→FIX→PROMOTE

### Session Update (2026-02-07, parser handoff Tier-2 local-expr-string-subcmp + cleanup promote)

- [x] parser handoff Tier-2（`local` 初期化で文字列 `substring` / 比較式を使用 + postfix `cleanup`）fixture を追加: `apps/tests/phase29bq_selfhost_local_expr_string_subcmp_cleanup_min.hako`（expected=`344`）
- [x] PROBE: `SMOKES_SELFHOST_LIST=/tmp/selfhost_probe.tsv tools/selfhost/run.sh --gate --planner-required 1 --max-cases 1` PASS
- [x] PROMOTE: `tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv` に `phase29bq_selfhost_local_expr_string_subcmp_cleanup_min.hako` を追加
- [x] verify: `tools/selfhost/run.sh --gate --planner-required 1 --filter phase29bq_selfhost_local_expr_string_subcmp_cleanup_min --max-cases 1` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [x] next: Tier-2（式網羅）継続として `local` + null比較 / 論理式 初期化の1件を PROBE→FIX→PROMOTE

### Session Update (2026-02-07, parser handoff Tier-2 local-expr-null-logic + cleanup promote)

- [x] parser handoff Tier-2（`local` 初期化で null比較 / 論理式を使用 + postfix `cleanup`）fixture を追加: `apps/tests/phase29bq_selfhost_local_expr_null_logic_cleanup_min.hako`（expected=`455`）
- [x] PROBE: `SMOKES_SELFHOST_LIST=/tmp/selfhost_probe.tsv tools/selfhost/run.sh --gate --planner-required 1 --max-cases 1` PASS
- [x] PROMOTE: `tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv` に `phase29bq_selfhost_local_expr_null_logic_cleanup_min.hako` を追加
- [x] verify: `tools/selfhost/run.sh --gate --planner-required 1 --filter phase29bq_selfhost_local_expr_null_logic_cleanup_min --max-cases 1` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [x] next: Tier-2（式網羅）継続として `local` + string method call chain（`trim().length()`）初期化の1件を PROBE→FIX→PROMOTE

### Session Update (2026-02-07, parser handoff Tier-2 local-expr-string-trim-chain + cleanup promote)

- [x] parser handoff Tier-2（`local` 初期化で string method call chain `trim().length()` を使用 + postfix `cleanup`）fixture を追加: `apps/tests/phase29bq_selfhost_local_expr_string_trim_chain_cleanup_min.hako`（expected=`666`）
- [x] PROBE: `SMOKES_SELFHOST_LIST=/tmp/selfhost_probe.tsv tools/selfhost/run.sh --gate --planner-required 1 --max-cases 1` PASS
- [x] PROMOTE: `tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv` に `phase29bq_selfhost_local_expr_string_trim_chain_cleanup_min.hako` を追加
- [x] verify: `tools/selfhost/run.sh --gate --planner-required 1 --filter phase29bq_selfhost_local_expr_string_trim_chain_cleanup_min --max-cases 1` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [x] next: Tier-2（式網羅）継続として `local` + 複合ブール式（`!` + null比較 + 比較演算）初期化の1件を PROBE→FIX→PROMOTE

### Session Update (2026-02-07, parser handoff Tier-2 local-expr-bool-combo + cleanup promote)

- [x] parser handoff Tier-2（`local` 初期化で複合ブール式 `!` + null比較 + 比較演算を使用 + postfix `cleanup`）fixture を追加: `apps/tests/phase29bq_selfhost_local_expr_bool_combo_cleanup_min.hako`（expected=`703`）
- [x] PROBE: `SMOKES_SELFHOST_LIST=/tmp/selfhost_probe.tsv tools/selfhost/run.sh --gate --planner-required 1 --max-cases 1` PASS
- [x] PROMOTE: `tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv` に `phase29bq_selfhost_local_expr_bool_combo_cleanup_min.hako` を追加
- [x] verify: `tools/selfhost/run.sh --gate --planner-required 1 --filter phase29bq_selfhost_local_expr_bool_combo_cleanup_min --max-cases 1` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [x] next: Tier-2（式網羅）継続として `local` + 論理優先順位（`!` + `&&` + `||` + 比較）初期化の1件を PROBE→FIX→PROMOTE

### Session Update (2026-02-07, parser handoff Tier-2 local-expr-logic-precedence + cleanup promote)

- [x] parser handoff Tier-2（`local` 初期化で `!` + `&&` + `||` + 比較演算の複合式を使用 + postfix `cleanup`）fixture を追加: `apps/tests/phase29bq_selfhost_local_expr_logic_precedence_cleanup_min.hako`（expected=`803`）
- [x] PROBE: `SMOKES_SELFHOST_LIST=/tmp/selfhost_probe.tsv tools/selfhost/run.sh --gate --planner-required 1 --max-cases 1` PASS
- [x] PROMOTE: `tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv` に `phase29bq_selfhost_local_expr_logic_precedence_cleanup_min.hako` を追加
- [x] verify: `tools/selfhost/run.sh --gate --planner-required 1 --filter phase29bq_selfhost_local_expr_logic_precedence_cleanup_min --max-cases 1` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [x] note: 本fixtureの実測結果は `803`（`A || B && C` の評価順の仕様確認は parser handoff 完了後に別タスクで切り分け）
- [x] next: Tier-2（式網羅）継続として `local` + 括弧付き論理式（優先順位明示）初期化の1件を PROBE→FIX→PROMOTE

### Session Update (2026-02-07, parser handoff Tier-2 local-expr-logic-parenthesized + cleanup promote)

- [x] parser handoff Tier-2（`local` 初期化で括弧付き論理式（優先順位明示）を使用 + postfix `cleanup`）fixture を追加: `apps/tests/phase29bq_selfhost_local_expr_logic_parenthesized_cleanup_min.hako`（expected=`911`）
- [x] PROBE: `SMOKES_SELFHOST_LIST=/tmp/selfhost_probe.tsv tools/selfhost/run.sh --gate --planner-required 1 --max-cases 1` PASS
- [x] PROMOTE: `tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv` に `phase29bq_selfhost_local_expr_logic_parenthesized_cleanup_min.hako` を追加
- [x] verify: `tools/selfhost/run.sh --gate --planner-required 1 --filter phase29bq_selfhost_local_expr_logic_parenthesized_cleanup_min --max-cases 1` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [x] note: 本fixtureの実測結果は `911`（否定つき括弧式の評価規約は parser handoff 完了後に式評価仕様タスクとして切り分け）
- [x] next: Tier-2（式網羅）継続として `local` + 括弧付き二重否定（`!!`）+ 比較式 初期化の1件を PROBE→FIX→PROMOTE

### Session Update (2026-02-07, parser handoff Tier-2 local-expr-double-not-compare + cleanup promote)

- [x] parser handoff Tier-2（`local` 初期化で二重否定 `!(!...)` + 比較式を使用 + postfix `cleanup`）fixture を追加: `apps/tests/phase29bq_selfhost_local_expr_double_not_compare_cleanup_min.hako`（expected=`1003`）
- [x] PROBE: `SMOKES_SELFHOST_LIST=/tmp/selfhost_probe.tsv tools/selfhost/run.sh --gate --planner-required 1 --max-cases 1` PASS
- [x] PROMOTE: `tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv` に `phase29bq_selfhost_local_expr_double_not_compare_cleanup_min.hako` を追加
- [x] verify: `tools/selfhost/run.sh --gate --planner-required 1 --filter phase29bq_selfhost_local_expr_double_not_compare_cleanup_min --max-cases 1` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [x] note: 本fixtureの実測結果は `1003`（`!` の評価規約は parser handoff 完了後に式評価仕様タスクとして切り分け）
- [x] next: Tier-2（式網羅）継続として `local` + 比較連鎖（`==`/`!=` + `&&`）初期化の1件を PROBE→FIX→PROMOTE

### Session Update (2026-02-07, parser handoff Tier-2 local-expr-compare-chain + cleanup promote)

- [x] parser handoff Tier-2（`local` 初期化で比較連鎖 `==`/`!=` + `&&` を使用 + postfix `cleanup`）fixture を追加: `apps/tests/phase29bq_selfhost_local_expr_compare_chain_cleanup_min.hako`（expected=`1113`）
- [x] PROBE: `SMOKES_SELFHOST_LIST=/tmp/selfhost_probe.tsv tools/selfhost/run.sh --gate --planner-required 1 --max-cases 1` PASS
- [x] PROMOTE: `tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv` に `phase29bq_selfhost_local_expr_compare_chain_cleanup_min.hako` を追加
- [x] verify: `tools/selfhost/run.sh --gate --planner-required 1 --filter phase29bq_selfhost_local_expr_compare_chain_cleanup_min --max-cases 1` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [x] next: Tier-2（式網羅）継続として `local` + 比較式 in 論理OR（`==`/`!=` + `||`）初期化の1件を PROBE→FIX→PROMOTE

### Session Update (2026-02-07, parser handoff Tier-2 local-expr-compare-or + cleanup promote)

- [x] parser handoff Tier-2（`local` 初期化で比較式 `==`/`!=` + `||` を使用 + postfix `cleanup`）fixture を追加: `apps/tests/phase29bq_selfhost_local_expr_compare_or_cleanup_min.hako`（expected=`1214`）
- [x] PROBE: `SMOKES_SELFHOST_LIST=/tmp/selfhost_probe.tsv tools/selfhost/run.sh --gate --planner-required 1 --max-cases 1` PASS
- [x] PROMOTE: `tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv` に `phase29bq_selfhost_local_expr_compare_or_cleanup_min.hako` を追加
- [x] verify: `tools/selfhost/run.sh --gate --planner-required 1 --filter phase29bq_selfhost_local_expr_compare_or_cleanup_min --max-cases 1` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [x] next: Tier-2（式網羅）継続として `local` + 比較式 in 論理混在（`==`/`!=` + `&&` + `||`）初期化の1件を PROBE→FIX→PROMOTE

### Session Update (2026-02-07, parser handoff Tier-2 local-expr-compare-mixed-logic + cleanup promote)

- [x] parser handoff Tier-2（`local` 初期化で比較式 `==`/`!=` + `&&` + `||` を使用 + postfix `cleanup`）fixture を追加: `apps/tests/phase29bq_selfhost_local_expr_compare_mixed_logic_cleanup_min.hako`（expected=`1315`）
- [x] PROBE: `SMOKES_SELFHOST_LIST=/tmp/selfhost_probe.tsv tools/selfhost/run.sh --gate --planner-required 1 --max-cases 1` PASS
- [x] PROMOTE: `tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv` に `phase29bq_selfhost_local_expr_compare_mixed_logic_cleanup_min.hako` を追加
- [x] verify: `tools/selfhost/run.sh --gate --planner-required 1 --filter phase29bq_selfhost_local_expr_compare_mixed_logic_cleanup_min --max-cases 1` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [x] next: Tier-2（式網羅）継続として `local` + 比較式/論理式 with null（`== null` + `!= null` + `&&`）初期化の1件を PROBE→FIX→PROMOTE

### Session Update (2026-02-07, parser handoff Tier-2 local-expr-null-compare-and + cleanup promote)

- [x] parser handoff Tier-2（`local` 初期化で `== null` + `!= null` + `&&` を使用 + postfix `cleanup`）fixture を追加: `apps/tests/phase29bq_selfhost_local_expr_null_compare_and_cleanup_min.hako`（expected=`1416`）
- [x] PROBE: `SMOKES_SELFHOST_LIST=/tmp/selfhost_probe.tsv tools/selfhost/run.sh --gate --planner-required 1 --max-cases 1` PASS
- [x] PROMOTE: `tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv` に `phase29bq_selfhost_local_expr_null_compare_and_cleanup_min.hako` を追加
- [x] verify: `tools/selfhost/run.sh --gate --planner-required 1 --filter phase29bq_selfhost_local_expr_null_compare_and_cleanup_min --max-cases 1` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [x] next: Tier-2（式網羅）継続として `local` + 比較式/論理式 with null（`== null` + `!= null` + `||`）初期化の1件を PROBE→FIX→PROMOTE

### Session Update (2026-02-07, parser handoff Tier-2 local-expr-null-compare-or + cleanup promote)

- [x] parser handoff Tier-2（`local` 初期化で `== null` + `!= null` + `||` を使用 + postfix `cleanup`）fixture を追加: `apps/tests/phase29bq_selfhost_local_expr_null_compare_or_cleanup_min.hako`（expected=`1517`）
- [x] PROBE: `SMOKES_SELFHOST_LIST=/tmp/selfhost_probe.tsv tools/selfhost/run.sh --gate --planner-required 1 --max-cases 1` PASS
- [x] PROMOTE: `tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv` に `phase29bq_selfhost_local_expr_null_compare_or_cleanup_min.hako` を追加
- [x] verify: `tools/selfhost/run.sh --gate --planner-required 1 --filter phase29bq_selfhost_local_expr_null_compare_or_cleanup_min --max-cases 1` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [x] next: Tier-2（式網羅）継続として `local` + 比較式/論理式 with null（`== null`/`!= null` + `&&` + `||`）初期化の1件を PROBE→FIX→PROMOTE

### Session Update (2026-02-07, parser handoff Tier-2 local-expr-null-compare-mixed-logic + cleanup promote)

- [x] parser handoff Tier-2（`local` 初期化で `== null`/`!= null` + `&&` + `||` を使用 + postfix `cleanup`）fixture を追加: `apps/tests/phase29bq_selfhost_local_expr_null_compare_mixed_logic_cleanup_min.hako`（expected=`1659`）
- [x] PROBE: `SMOKES_SELFHOST_LIST=/tmp/selfhost_probe.tsv tools/selfhost/run.sh --gate --planner-required 1 --max-cases 1` PASS
- [x] PROMOTE: `tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv` に `phase29bq_selfhost_local_expr_null_compare_mixed_logic_cleanup_min.hako` を追加
- [x] verify: `tools/selfhost/run.sh --gate --planner-required 1 --filter phase29bq_selfhost_local_expr_null_compare_mixed_logic_cleanup_min --max-cases 1` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [x] next: Tier-2（式網羅）継続として `local` + `!(== null)` + `&&` 初期化の1件を PROBE→FIX→PROMOTE

### Session Update (2026-02-07, parser handoff Tier-2 local-expr-null-not-compare + cleanup promote)

- [x] parser handoff Tier-2（`local` 初期化で `!(== null)` + `&&` を使用 + postfix `cleanup`）fixture を追加: `apps/tests/phase29bq_selfhost_local_expr_null_not_compare_cleanup_min.hako`（expected=`1706`）
- [x] PROBE: `SMOKES_SELFHOST_LIST=/tmp/selfhost_probe.tsv tools/selfhost/run.sh --gate --planner-required 1 --max-cases 1` PASS
- [x] PROMOTE: `tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv` に `phase29bq_selfhost_local_expr_null_not_compare_cleanup_min.hako` を追加
- [x] verify: `tools/selfhost/run.sh --gate --planner-required 1 --filter phase29bq_selfhost_local_expr_null_not_compare_cleanup_min --max-cases 1` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [x] next: Tier-2（式網羅）継続として `local` + parenthesized null-logic 初期化の1件を PROBE→FIX→PROMOTE

### Session Update (2026-02-07, parser handoff Tier-2 local-expr-null-parenthesized-logic + cleanup promote)

- [x] parser handoff Tier-2（`local` 初期化で parenthesized null-logic を使用 + postfix `cleanup`）fixture を追加: `apps/tests/phase29bq_selfhost_local_expr_null_parenthesized_logic_cleanup_min.hako`（expected=`2000`）
- [x] PROBE: `SMOKES_SELFHOST_LIST=/tmp/selfhost_probe.tsv tools/selfhost/run.sh --gate --planner-required 1 --max-cases 1` PASS
- [x] PROMOTE: `tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv` に `phase29bq_selfhost_local_expr_null_parenthesized_logic_cleanup_min.hako` を追加
- [x] verify: `tools/selfhost/run.sh --gate --planner-required 1 --filter phase29bq_selfhost_local_expr_null_parenthesized_logic_cleanup_min --max-cases 1` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [x] next: Tier-2（式網羅）継続として `(+,-,*,/) + (==/!=) + &&` 初期化の1件を PROBE→FIX→PROMOTE

### Session Update (2026-02-07, parser handoff Tier-2 local-expr-arith-compare-and + cleanup promote)

- [x] parser handoff Tier-2（`local` 初期化で `(+,-,*,/) + (==/!=) + &&` を使用 + postfix `cleanup`）fixture を追加: `apps/tests/phase29bq_selfhost_local_expr_arith_compare_and_cleanup_min.hako`（expected=`2350`）
- [x] PROBE: `SMOKES_SELFHOST_LIST=/tmp/selfhost_probe.tsv tools/selfhost/run.sh --gate --planner-required 1 --max-cases 1` PASS
- [x] PROMOTE: `tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv` に `phase29bq_selfhost_local_expr_arith_compare_and_cleanup_min.hako` を追加
- [x] verify: `tools/selfhost/run.sh --gate --planner-required 1 --filter phase29bq_selfhost_local_expr_arith_compare_and_cleanup_min --max-cases 1` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [x] next: Tier-2（式網羅）継続として `(+,-,*,/) + (==/!=) + ||` 初期化の1件を PROBE→FIX→PROMOTE

### Session Update (2026-02-07, parser handoff Tier-2 local-expr-arith-compare-or + cleanup promote)

- [x] parser handoff Tier-2（`local` 初期化で `(+,-,*,/) + (==/!=) + ||` を使用 + postfix `cleanup`）fixture を追加: `apps/tests/phase29bq_selfhost_local_expr_arith_compare_or_cleanup_min.hako`（expected=`2366`）
- [x] PROBE: `SMOKES_SELFHOST_LIST=/tmp/selfhost_probe.tsv tools/selfhost/run.sh --gate --planner-required 1 --max-cases 1` PASS
- [x] PROMOTE: `tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv` に `phase29bq_selfhost_local_expr_arith_compare_or_cleanup_min.hako` を追加
- [x] verify: `tools/selfhost/run.sh --gate --planner-required 1 --filter phase29bq_selfhost_local_expr_arith_compare_or_cleanup_min --max-cases 1` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [x] next: Tier-2（式網羅）継続として `(==/!=/< />) + && + ||` 初期化の1件を PROBE→FIX→PROMOTE

### Session Update (2026-02-08, parser handoff Tier-2 local-expr-compare-rel-mixed-logic + cleanup promote)

- [x] parser handoff Tier-2（`local` 初期化で `(==/!=/< />) + && + ||` を使用 + postfix `cleanup`）fixture を追加: `apps/tests/phase29bq_selfhost_local_expr_compare_rel_mixed_logic_cleanup_min.hako`（expected=`2397`）
- [x] PROBE: `SMOKES_SELFHOST_LIST=/tmp/selfhost_probe.tsv tools/selfhost/run.sh --gate --planner-required 1 --max-cases 1` PASS
- [x] PROMOTE: `tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv` に `phase29bq_selfhost_local_expr_compare_rel_mixed_logic_cleanup_min.hako` を追加
- [x] verify: `tools/selfhost/run.sh --gate --planner-required 1 --filter phase29bq_selfhost_local_expr_compare_rel_mixed_logic_cleanup_min --max-cases 1` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [x] next: Tier-2（式網羅）継続として `! + unary - + 比較 + &&` 初期化の1件を PROBE→FIX→PROMOTE
### Session Update (2026-02-08, parser handoff Tier-2 local-expr-unary-compare-mixed + cleanup promote)

- [x] parser handoff Tier-2（`local` 初期化で `! + unary - + 比較 + &&` を使用 + postfix `cleanup`）fixture を追加: `apps/tests/phase29bq_selfhost_local_expr_unary_compare_mixed_cleanup_min.hako`（expected=`2306`）
- [x] PROBE: `SMOKES_SELFHOST_LIST=/tmp/selfhost_probe.tsv tools/selfhost/run.sh --gate --planner-required 1 --max-cases 1` PASS
- [x] PROMOTE: `tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv` に `phase29bq_selfhost_local_expr_unary_compare_mixed_cleanup_min.hako` を追加
- [x] verify: `tools/selfhost/run.sh --gate --planner-required 1 --filter phase29bq_selfhost_local_expr_unary_compare_mixed_cleanup_min --max-cases 1` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [x] next: Tier-2（式網羅）継続として `!! + (==/!=) + ||` 初期化の1件を PROBE→FIX→PROMOTE
### Session Update (2026-02-08, parser handoff Tier-2 local-expr-double-not-mixed-logic + cleanup promote)

- [x] parser handoff Tier-2（`local` 初期化で `!! + (==/!=) + ||` を使用 + postfix `cleanup`）fixture を追加: `apps/tests/phase29bq_selfhost_local_expr_double_not_mixed_logic_cleanup_min.hako`（expected=`2306`）
- [x] PROBE: `SMOKES_SELFHOST_LIST=/tmp/selfhost_probe.tsv tools/selfhost/run.sh --gate --planner-required 1 --max-cases 1` PASS
- [x] PROMOTE: `tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv` に `phase29bq_selfhost_local_expr_double_not_mixed_logic_cleanup_min.hako` を追加
- [x] verify: `tools/selfhost/run.sh --gate --planner-required 1 --filter phase29bq_selfhost_local_expr_double_not_mixed_logic_cleanup_min --max-cases 1` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [x] next: Tier-2（式網羅）継続として `parenthesized ((cmp && cmp) || cmp)` 初期化の1件を PROBE→FIX→PROMOTE
### Session Update (2026-02-08, parser handoff Tier-2 local-expr-parenthesized-compare-mixed + cleanup promote)

- [x] parser handoff Tier-2（`local` 初期化で `parenthesized ((cmp && cmp) || cmp)` を使用 + postfix `cleanup`）fixture を追加: `apps/tests/phase29bq_selfhost_local_expr_parenthesized_compare_mixed_cleanup_min.hako`（expected=`2523`）
- [x] PROBE: `SMOKES_SELFHOST_LIST=/tmp/selfhost_probe.tsv tools/selfhost/run.sh --gate --planner-required 1 --max-cases 1` PASS
- [x] PROMOTE: `tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv` に `phase29bq_selfhost_local_expr_parenthesized_compare_mixed_cleanup_min.hako` を追加
- [x] verify: `tools/selfhost/run.sh --gate --planner-required 1 --filter phase29bq_selfhost_local_expr_parenthesized_compare_mixed_cleanup_min --max-cases 1` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [x] next: Tier-2（式網羅）継続として `!(cmp || cmp) && cmp` 初期化の1件を PROBE→FIX→PROMOTE
### Session Update (2026-02-08, parser handoff Tier-2 local-expr-not-parenthesized-compare + cleanup promote)

- [x] parser handoff Tier-2（`local` 初期化で `!(cmp || cmp) && cmp` を使用 + postfix `cleanup`）fixture を追加: `apps/tests/phase29bq_selfhost_local_expr_not_parenthesized_compare_cleanup_min.hako`（expected=`2306`）
- [x] PROBE: `SMOKES_SELFHOST_LIST=/tmp/selfhost_probe.tsv tools/selfhost/run.sh --gate --planner-required 1 --max-cases 1` PASS
- [x] PROMOTE: `tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv` に `phase29bq_selfhost_local_expr_not_parenthesized_compare_cleanup_min.hako` を追加
- [x] verify: `tools/selfhost/run.sh --gate --planner-required 1 --filter phase29bq_selfhost_local_expr_not_parenthesized_compare_cleanup_min --max-cases 1` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [x] next: Tier-2（式網羅）継続として `call return 値 + 比較 + &&` 初期化の1件を PROBE→FIX→PROMOTE
### Session Update (2026-02-08, parser handoff Tier-2 local-expr-call-compare-and + cleanup promote)

- [x] parser handoff Tier-2（`local` 初期化で `call return 値 + 比較 + &&` を使用 + postfix `cleanup`）fixture を追加: `apps/tests/phase29bq_selfhost_local_expr_call_compare_and_cleanup_min.hako`（expected=`2396`）
- [x] PROBE: `SMOKES_SELFHOST_LIST=/tmp/selfhost_probe.tsv tools/selfhost/run.sh --gate --planner-required 1 --max-cases 1` PASS
- [x] PROMOTE: `tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv` に `phase29bq_selfhost_local_expr_call_compare_and_cleanup_min.hako` を追加
- [x] verify: `tools/selfhost/run.sh --gate --planner-required 1 --filter phase29bq_selfhost_local_expr_call_compare_and_cleanup_min --max-cases 1` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [x] next: Tier-2（式網羅）継続として `call return 値 + 比較 + ||` 初期化の1件を PROBE→FIX→PROMOTE
### Session Update (2026-02-08, parser handoff Tier-2 local-expr-call-compare-or + cleanup promote)

- [x] parser handoff Tier-2（`local` 初期化で `call return 値 + 比較 + ||` を使用 + postfix `cleanup`）fixture を追加: `apps/tests/phase29bq_selfhost_local_expr_call_compare_or_cleanup_min.hako`（expected=`2371`）
- [x] PROBE: `SMOKES_SELFHOST_LIST=/tmp/selfhost_probe.tsv tools/selfhost/run.sh --gate --planner-required 1 --max-cases 1` PASS
- [x] PROMOTE: `tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv` に `phase29bq_selfhost_local_expr_call_compare_or_cleanup_min.hako` を追加
- [x] verify: `tools/selfhost/run.sh --gate --planner-required 1 --filter phase29bq_selfhost_local_expr_call_compare_or_cleanup_min --max-cases 1` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [x] next: Tier-2（式網羅）継続として `new object method return + 比較 + &&/||` 初期化の1件を PROBE→FIX→PROMOTE
### Session Update (2026-02-08, parser handoff Tier-2 local-expr-new-compare-mixed + cleanup promote)

- [x] parser handoff Tier-2（`local` 初期化で `new object method return + 比較 + &&/||` を使用 + postfix `cleanup`）fixture を追加: `apps/tests/phase29bq_selfhost_local_expr_new_compare_mixed_cleanup_min.hako`（expected=`2393`）
- [x] PROBE: `SMOKES_SELFHOST_LIST=/tmp/selfhost_probe.tsv tools/selfhost/run.sh --gate --planner-required 1 --max-cases 1` PASS
- [x] PROMOTE: `tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv` に `phase29bq_selfhost_local_expr_new_compare_mixed_cleanup_min.hako` を追加
- [x] verify: `tools/selfhost/run.sh --gate --planner-required 1 --filter phase29bq_selfhost_local_expr_new_compare_mixed_cleanup_min --max-cases 1` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [x] next: Tier-2（式網羅）継続として `string method chain + 比較 + &&/||` 初期化の1件を PROBE→FIX→PROMOTE
### Session Update (2026-02-08, parser handoff Tier-2 local-expr-string-compare-logic + cleanup promote)

- [x] parser handoff Tier-2（`local` 初期化で `string method chain + 比較 + &&/||` を使用 + postfix `cleanup`）fixture を追加: `apps/tests/phase29bq_selfhost_local_expr_string_compare_logic_cleanup_min.hako`（expected=`2410`）
- [x] PROBE: `SMOKES_SELFHOST_LIST=/tmp/selfhost_probe.tsv tools/selfhost/run.sh --gate --planner-required 1 --max-cases 1` PASS
- [x] PROMOTE: `tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv` に `phase29bq_selfhost_local_expr_string_compare_logic_cleanup_min.hako` を追加
- [x] verify: `tools/selfhost/run.sh --gate --planner-required 1 --filter phase29bq_selfhost_local_expr_string_compare_logic_cleanup_min --max-cases 1` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [x] next: Tier-2（式網羅）継続として `array length compare + &&` 初期化の1件を PROBE→FIX→PROMOTE
### Session Update (2026-02-08, parser handoff Tier-2 local-expr-array-len-compare + cleanup promote)

- [x] parser handoff Tier-2（`local` 初期化で `array length compare + &&` を使用 + postfix `cleanup`）fixture を追加: `apps/tests/phase29bq_selfhost_local_expr_array_len_compare_cleanup_min.hako`（expected=`2425`）
- [x] PROBE: `SMOKES_SELFHOST_LIST=/tmp/selfhost_probe.tsv tools/selfhost/run.sh --gate --planner-required 1 --max-cases 1` PASS
- [x] PROMOTE: `tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv` に `phase29bq_selfhost_local_expr_array_len_compare_cleanup_min.hako` を追加
- [x] verify: `tools/selfhost/run.sh --gate --planner-required 1 --filter phase29bq_selfhost_local_expr_array_len_compare_cleanup_min --max-cases 1` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [x] next: Tier-2（式網羅）継続として `map lookup-ish path + 比較 + ||` 初期化の1件を PROBE→FIX→PROMOTE
### Session Update (2026-02-08, parser handoff Tier-2 local-expr-map-value-compare + cleanup promote)

- [x] parser handoff Tier-2（`local` 初期化で `map lookup-ish path + 比較 + ||` を使用 + postfix `cleanup`）fixture を追加: `apps/tests/phase29bq_selfhost_local_expr_map_value_compare_cleanup_min.hako`（expected=`2525`）
- [x] PROBE: `SMOKES_SELFHOST_LIST=/tmp/selfhost_probe.tsv tools/selfhost/run.sh --gate --planner-required 1 --max-cases 1` PASS
- [x] PROMOTE: `tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv` に `phase29bq_selfhost_local_expr_map_value_compare_cleanup_min.hako` を追加
- [x] verify: `tools/selfhost/run.sh --gate --planner-required 1 --filter phase29bq_selfhost_local_expr_map_value_compare_cleanup_min --max-cases 1` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [x] next: Tier-2（式網羅）継続として `bool literal + 比較 + mixed logic` 初期化の1件を PROBE→FIX→PROMOTE
### Session Update (2026-02-08, parser handoff Tier-2 local-expr-literal-bool-mixed + cleanup promote)

- [x] parser handoff Tier-2（`local` 初期化で `bool literal + 比較 + mixed logic` を使用 + postfix `cleanup`）fixture を追加: `apps/tests/phase29bq_selfhost_local_expr_literal_bool_mixed_cleanup_min.hako`（expected=`2631`）
- [x] PROBE: `SMOKES_SELFHOST_LIST=/tmp/selfhost_probe.tsv tools/selfhost/run.sh --gate --planner-required 1 --max-cases 1` PASS
- [x] PROMOTE: `tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv` に `phase29bq_selfhost_local_expr_literal_bool_mixed_cleanup_min.hako` を追加
- [x] verify: `tools/selfhost/run.sh --gate --planner-required 1 --filter phase29bq_selfhost_local_expr_literal_bool_mixed_cleanup_min --max-cases 1` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [x] next: Tier-2（式網羅）継続として `blockexpr return + 比較 + &&` 初期化の1件を PROBE→FIX→PROMOTE
### Session Update (2026-02-08, parser handoff Tier-2 local-expr-blockexpr-compare + cleanup promote)

- [x] parser handoff Tier-2（`local` 初期化で `blockexpr return + 比較 + &&` を使用 + postfix `cleanup`）fixture を追加: `apps/tests/phase29bq_selfhost_local_expr_blockexpr_compare_cleanup_min.hako`（expected=`2735`）
- [x] PROBE: `SMOKES_SELFHOST_LIST=/tmp/selfhost_probe.tsv tools/selfhost/run.sh --gate --planner-required 1 --max-cases 1` PASS
- [x] PROMOTE: `tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv` に `phase29bq_selfhost_local_expr_blockexpr_compare_cleanup_min.hako` を追加
- [x] verify: `tools/selfhost/run.sh --gate --planner-required 1 --filter phase29bq_selfhost_local_expr_blockexpr_compare_cleanup_min --max-cases 1` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [x] next: Tier-2（式網羅）継続として `Tier-3 local + fini 初回ケース` 初期化の1件を PROBE→FIX→PROMOTE
### Session Update (2026-02-08, parser handoff Tier-3 local-expr-compare-fini + cleanup promote)

- [x] parser handoff Tier-3（`local` 初期化で `local expr init + compare + fini + cleanup` を使用）fixture を追加: `apps/tests/phase29bq_selfhost_local_expr_compare_fini_cleanup_min.hako`（expected=`1136`）
- [x] PROBE: `SMOKES_SELFHOST_LIST=/tmp/selfhost_probe.tsv tools/selfhost/run.sh --gate --planner-required 1 --max-cases 1` PASS
- [x] PROMOTE: `tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv` に `phase29bq_selfhost_local_expr_compare_fini_cleanup_min.hako` を追加
- [x] verify: `tools/selfhost/run.sh --gate --planner-required 1 --filter phase29bq_selfhost_local_expr_compare_fini_cleanup_min --max-cases 1` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [x] next: Tier-3（`local + fini`）継続として `local call/new init + fini` 初期化の1件を PROBE→FIX→PROMOTE
### Session Update (2026-02-08, parser handoff Tier-3 local-expr-call-fini + cleanup promote)

- [x] parser handoff Tier-3（`local` 初期化で `local call/new init + fini` を使用）fixture を追加: `apps/tests/phase29bq_selfhost_local_expr_call_fini_cleanup_min.hako`（expected=`1331`）
- [x] PROBE: `SMOKES_SELFHOST_LIST=/tmp/selfhost_probe.tsv tools/selfhost/run.sh --gate --planner-required 1 --max-cases 1` PASS
- [x] PROMOTE: `tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv` に `phase29bq_selfhost_local_expr_call_fini_cleanup_min.hako` を追加
- [x] verify: `tools/selfhost/run.sh --gate --planner-required 1 --filter phase29bq_selfhost_local_expr_call_fini_cleanup_min --max-cases 1` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [x] next: Tier-3（`local + fini`）継続として `local blockexpr init + fini` 初期化の1件を PROBE→FIX→PROMOTE
### Session Update (2026-02-08, parser handoff Tier-3 local-expr-blockexpr-fini + cleanup promote)

- [x] parser handoff Tier-3（`local` 初期化で `local blockexpr init + fini` を使用）fixture を追加: `apps/tests/phase29bq_selfhost_local_expr_blockexpr_fini_cleanup_min.hako`（expected=`1430`）
- [x] PROBE: `SMOKES_SELFHOST_LIST=/tmp/selfhost_probe.tsv tools/selfhost/run.sh --gate --planner-required 1 --max-cases 1` PASS
- [x] PROMOTE: `tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv` に `phase29bq_selfhost_local_expr_blockexpr_fini_cleanup_min.hako` を追加
- [x] verify: `tools/selfhost/run.sh --gate --planner-required 1 --filter phase29bq_selfhost_local_expr_blockexpr_fini_cleanup_min --max-cases 1` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [x] next: Tier-3（`local + fini`）継続として `multi local-fini + reassignment + cleanup` 初期化の1件を PROBE→FIX→PROMOTE
### Session Update (2026-02-08, parser handoff Tier-3 local-fini-multi-lifo + cleanup promote)

- [x] parser handoff Tier-3（`local` 初期化で `multi local-fini + reassignment + cleanup` を使用）fixture を追加: `apps/tests/phase29bq_selfhost_local_fini_multi_lifo_cleanup_min.hako`（expected=`cbaY`）
- [x] PROBE: `SMOKES_SELFHOST_LIST=/tmp/selfhost_probe.tsv tools/selfhost/run.sh --gate --planner-required 1 --max-cases 1` PASS
- [x] PROMOTE: `tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv` に `phase29bq_selfhost_local_fini_multi_lifo_cleanup_min.hako` を追加
- [x] verify: `tools/selfhost/run.sh --gate --planner-required 1 --filter phase29bq_selfhost_local_fini_multi_lifo_cleanup_min --max-cases 1` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [x] next: Tier-3（`local + fini`）継続として `null compare/logic init + fini` 初期化の1件を PROBE→FIX→PROMOTE
### Session Update (2026-02-08, parser handoff Tier-3 local-expr-null-fini + cleanup promote)

- [x] parser handoff Tier-3（`local` 初期化で `null compare/logic init + fini` を使用）fixture を追加: `apps/tests/phase29bq_selfhost_local_expr_null_fini_cleanup_min.hako`（expected=`1548`）
- [x] PROBE: `SMOKES_SELFHOST_LIST=/tmp/selfhost_probe.tsv tools/selfhost/run.sh --gate --planner-required 1 --max-cases 1` PASS
- [x] PROMOTE: `tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv` に `phase29bq_selfhost_local_expr_null_fini_cleanup_min.hako` を追加
- [x] verify: `tools/selfhost/run.sh --gate --planner-required 1 --filter phase29bq_selfhost_local_expr_null_fini_cleanup_min --max-cases 1` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [x] next: Tier-3 完了マイルストーンとして `phase29bq_selfhost_planner_required_dev_gate_vm.sh` 全体を 1回実行し、snapshot を更新
### Session Update (2026-02-08, Tier-3 milestone gate + G1 identity probe)

- [x] milestone: `RUN_TIMEOUT_SECS=120 SMOKES_ENABLE_SELFHOST=1 HAKO_JOINIR_PLANNER_REQUIRED=1 bash tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_planner_required_dev_gate_vm.sh` PASS（cases=`182/182`, total_secs=`623`）
- [x] post-migration checks: `HAKO_SELFHOST_BUILDER_FIRST=1 HAKO_SELFHOST_NO_DELEGATE=1 bash tools/hakorune_emit_mir.sh apps/tests/phase29bq_selfhost_cleanup_only_min.hako /tmp/phase29bq_cleanup_only_internal.mir.json` PASS
- [x] post-migration checks: `HAKO_SELFHOST_BUILDER_FIRST=1 HAKO_SELFHOST_NO_DELEGATE=1 bash tools/hakorune_emit_mir.sh lang/src/runner/stage1_cli.hako /tmp/stage1_cli_fullbuilder.mir.json` PASS
- [x] probe: `tools/selfhost_identity_check.sh --mode full` は Stage1 build で FAIL（OOM 想定）; prebuilt か高メモリ環境が必要
- [ ] next: G1 unblock として prebuilt `stage1/stage2` を配置し、`tools/selfhost_identity_check.sh --mode full --skip-build --bin-stage1 <stage1> --bin-stage2 <stage2>` を実行

### Session Update (2026-02-08, G1 route hardening + loop shape accept)

- [x] BoxCount（Rust側）: `j + m <= n` 形の loop var 候補抽出を許可（analysis-only, no rewrite）
  - file: `src/mir/builder/control_flow/plan/canon/generic_loop.rs`
  - verify: `cargo test -q generic_loop_v0_allows_loop_var_from_add_expr_in_condition`
- [x] BoxCount（Rust側）: `loop(cond){ if <bool> { return ... } ; step }` の simple return-in-body 形を strict/dev でも受理
  - file: `src/mir/builder/control_flow/plan/loop_cond/return_in_body_facts.rs`
  - verify: `cargo test -q return_in_body_simple_if_return_then_step_matches_in_dev_mode`
  - verify: `cargo test -q return_in_body_simple_if_return_then_step_with_method_call_condition_matches`
- [x] facts回帰テストを追加: index_of相当 (`j + m <= n` + `if starts_with { return j }` + `j=j+1`) で `try_build_loop_facts` が `Some`
  - file: `src/mir/builder/control_flow/plan/facts/loop_tests.rs`
  - verify: `cargo test -q loopfacts_ok_some_for_index_of_add_bound_return_step`
- [x] build route hardening: `tools/selfhost/build_stage1.sh` に stage1-cli capability check を追加（偽成功を fail-fast）
  - `emit program-json` 不可な artifact は `exit 2` で停止
  - verify: `tools/selfhost/build_stage1.sh --artifact-kind stage1-cli --out /tmp/test_stage1_cli_bf --timeout-secs 600`（expected FAIL with capability check）
- [x] health check:
  - `cargo check --bin hakorune` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
  - `HAKO_JOINIR_PLANNER_REQUIRED=0 target/release/hakorune --backend vm /tmp/index_of_repro.hako` PASS
- [ ] blocker（G1 継続）:
  - `tools/selfhost/build_stage1.sh --artifact-kind stage1-cli` は build/link 自体は通るが、capability は `SMOKE-OK`（Program payload 未観測）に留まる
  - `tools/selfhost_identity_check.sh --mode full --skip-build --cli-mode stage1` は既定 `target/selfhost/hakorune` を参照し preflight FAIL（stage1-cli artifact を明示指定する必要あり）
  - `target/selfhost/hakorune.stage1_cli` の env emit route は現状 `Result: 0` のみで Program(JSON) を stdout に出していない
- [ ] next task (G1 unblock, 1-box):
  - `lang/src/runner/stage1_cli.hako` の `emit-program` 分岐で Program(JSON) を stdout に確実に出す（env route payload restore）
  - その後 Stage2 CLI を生成し、`tools/selfhost_identity_check.sh --mode full --skip-build --cli-mode stage1 --bin-stage1 target/selfhost/hakorune.stage1_cli --bin-stage2 <stage2-cli>` を再実行

### Session Update (2026-02-08, G1 stage1-cli bridge/link unblock)

- [x] `lang/src/runner/stage1_cli.hako` の bridge blocker を縮退
  - `__mir__.log` を削除（Program→MIR legacy path の `undefined __mir__` を回避）
  - `MirBuilderBox` 直依存を切り離し、`emit_mir_json` は parent route 利用メッセージで fail-fast
- [x] `tools/selfhost/build_stage1.sh` を stage1-cli build 契約に同期
  - stage1-cli build 時に `NYASH_BRIDGE_ME_DUMMY=1` を注入（legacy Program→MIR の `me` 補助）
  - capability probe を env-route 実行へ変更（`NYASH_STAGE1_MODE=emit-program`）
  - probe判定を `Program(JSON)` 優先 + `Result: 0` を `SMOKE-OK` として受理
- [x] LLVM extern 正規化/ランタイム export を追加（link unblock）
  - `src/llvm_py/instructions/extern_normalize.py`: `env.get/set -> nyash.env.get/set`
  - `src/llvm_py/instructions/externcall.py`: `nyash.env.get/set` signature を追加
  - `crates/nyash_kernel/src/exports/env.rs`: `nyash.env.get/set` + legacy alias `env.get/set` export を追加
- [x] verify:
  - `cargo check -q -p nyash_kernel` PASS
  - `python3 -m py_compile src/llvm_py/instructions/extern_normalize.py src/llvm_py/instructions/externcall.py` PASS
  - `bash tools/selfhost/build_stage1.sh --artifact-kind stage1-cli` PASS（`stage1-cli capability: SMOKE-OK`）
  - `cargo check --bin hakorune` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [ ] remaining blocker:
  - `target/selfhost/hakorune.stage1_cli` は env-route で Program(JSON) payload を stdout に出していない（`Result: 0` のみ）
  - G1 full identity の preflight を通すには payload restore + stage2 CLI artifact 生成が必要

### Session Update (2026-02-08, stage1 build loop speedup / reuse-if-fresh)

- [x] `tools/selfhost/build_stage1.sh` に fast-loop 最適化を追加（既定ON、必要時は `--force-rebuild` で無効化）
  - `--reuse-if-fresh <0|1>` / `--force-rebuild` を追加
  - metadata + 依存mtimeチェックで up-to-date artifact を再利用（relink/rebuildを回避）
  - `NYASH_LLVM_SKIP_BUILD` 未指定時は prebuilt toolchain 検出で auto `1` を注入
- [x] verify:
  - `bash -n tools/selfhost/build_stage1.sh` PASS
  - 1回目: `bash tools/selfhost/build_stage1.sh --artifact-kind stage1-cli` PASS（`build-opt: NYASH_LLVM_SKIP_BUILD=1 (auto)`）
  - 2回目: 同コマンドで `reuse: up-to-date artifact detected; skipping rebuild` を確認
- [ ] next:
  - `lang/src/runner/stage1_cli.hako` の env emit route で Program(JSON) payload 出力を復帰し、`SMOKE-OK` ではなく `OK (program-json)` へ移行

### Session Update (2026-02-08, AOT entry call fix + stage1-cli blocker rebaseline)

- [x] `src/llvm_py/builders/entry.py` を修正し、`main` が1引数でも `ny_main` から実際に呼び出すようにした（従来は `i64 0` 固定 return）
  - root cause: `ensure_ny_main()` の plain-main fallback が `len(args)>0` を未実行扱いしていた
  - fix: args-handle 生成ロジックを共通化し、`len(args)==1` は `call_user_main_1` で呼び出す
- [x] probe:
  - `bash tools/selfhost/build_stage1.sh --entry /tmp/print_probe.hako --out /tmp/print_probe_exe --force-rebuild` 後に `/tmp/print_probe_exe` で `PRINT_PROBE_OK` を確認（以前は常に `Result: 0` のみ）
  - `bash tools/selfhost/build_stage1.sh --artifact-kind stage1-cli --force-rebuild` は capability check で `rc=97` fail-fast（偽の `SMOKE-OK` を解消）
- [ ] current blocker (G1):
  - stage1 AOT 実行で `env.get` が空文字を返し（`NYASH_STAGE1_MODE`/`STAGE1_*` を読めない）、`stage1_main` が `mode=disabled` へ落ちる
  - 併せて `ArrayBox` 生成で `[nyrt_error] Unknown Box type: ArrayBox` が出る（entry args-handle 構築時）
- [ ] next task (1-box, runtime contract):
  - NyRT 側で AOT entry の args/env contract を固定する（`nyash.env.argv_get` と `nyash.env.get` の実値取得を最小 fixture で検証）
  - まず `/tmp/env_probe.hako` 相当の最小再現を `apps/tests/phase29bq_selfhost_blocker_*` に昇格し、`env!=empty` / `ArrayBox` 生成成功を gate 化する

### Session Update (2026-02-08, AOT env contract hotfix + stage1-cli smoke restore)

- [x] `src/llvm_py/instructions/externcall.py` で `nyash.env.get` 戻り値の string-handle タグ付けを追加
  - fix: `resolver.mark_string(dst)` + `value_types[dst]={"kind":"handle","box_type":"StringBox"}`
  - root cause: `"" + env.get(...)` が string concat 経路に乗らず、値が空文字化して `stage1_main` の mode/source 判定が崩れていた
- [x] probe:
  - `bash tools/selfhost/build_stage1.sh --entry /tmp/env_probe.hako --out /tmp/env_probe_exe --force-rebuild && FOO=bar /tmp/env_probe_exe`
    で `ENV_PROBE:bar` を確認（AOT env.get の実値取得）
- [x] `lang/src/runner/stage1_cli.hako` の env `emit-program` 経路を最小縮退
  - `BuildBox.emit_program_json_v0(...) == null` / 不正payload時は debugログのみで `rc=0` を返す（SMOKE導線維持）
  - note: Program(JSON) 本復旧ではなく、AOT static boxcall 未対応の間の開発導線確保
- [x] `tools/selfhost/build_stage1.sh` stage1-cli probe を `STAGE1_SOURCE_TEXT` 注入に同期
  - FileBox 非依存で env probe を実行（AOT最小環境での再現を安定化）
- [x] verify:
  - `bash tools/selfhost/build_stage1.sh --artifact-kind stage1-cli --force-rebuild` PASS（`stage1-cli capability: SMOKE-OK`）
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [ ] remaining blocker (G1):
  - AOT で module-string receiver の `boxcall`（例: `lang.compiler.build.build_box.emit_program_json_v0`）が NyRT invoke で未解決
  - `ArrayBox` の `nyash.env.box.new_i64x` 生成はまだ `Unknown Box type: ArrayBox` を出す
- [ ] next task (1-box):
  - NyRT invoke の static/module receiver 契約を 1箱で固定（string receiver を静的呼び出しとして受理）し、`SMOKE-OK` ではなく Program(JSON) 実出力へ戻す

### Session Update (2026-02-08, G1 runtime contract fix: module-string receiver + stage1-cli capability restore)

- [x] NyRT invoke に module-string receiver 受理を追加（AOT runtime contract）
  - file: `crates/nyash_kernel/src/plugin/module_string_dispatch.rs`（新規）
  - contract:
    - `lang.compiler.entry.using_resolver_box.resolve_for_source` は String handle を返す
    - `lang.compiler.build.build_box.emit_program_json_v0` は source text から Program(JSON v0) を生成して String handle を返す
  - file: `crates/nyash_kernel/src/plugin/invoke.rs`
    - `nyash.plugin.invoke_by_name_i64` 入口で module-string dispatch を先行評価
    - 無条件 debug `eprintln!` を削除（既定OFFログ契約に寄せる）
- [x] NyRT env box constructor 契約を補強
  - file: `crates/nyash_kernel/src/exports/env.rs`
  - fix: `nyash.env.box.new_i64x` で `ArrayBox` / `MapBox` を core direct birth（`Unknown Box type: ArrayBox` を解消）
- [x] LLVM 側の戻り値型タグを補強
  - file: `src/llvm_py/instructions/boxcall.py`
  - fix: `resolve_for_source` / `emit_program_json_v0` 戻り値を string-handle としてタグ付け
- [x] tests:
  - `cargo test -q -p nyash_kernel` PASS（追加3件含む）
    - `env_box_new_i64x_creates_array_box`
    - `invoke_by_name_accepts_stage1_using_resolver_module_receiver`
    - `invoke_by_name_accepts_stage1_build_box_module_receiver`
- [x] verify:
  - `cargo build --release -p nyash_kernel` PASS
  - `bash tools/selfhost/build_stage1.sh --artifact-kind stage1-cli --force-rebuild` PASS
    - capability: `OK (program-json)`（`SMOKE-OK` から復帰）
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [ ] remaining blocker (G1):
  - stage1-cli capability は復帰したが、`identity_check --mode full` は stage2 CLI artifact 未指定/未生成のため未完
- [ ] next task:
  - stage2 CLI artifact を生成して `tools/selfhost_identity_check.sh --mode full --skip-build --cli-mode stage1 --bin-stage1 target/selfhost/hakorune.stage1_cli --bin-stage2 <stage2-cli>` を再実行

### Session Update (2026-02-08, cleanliness: Program(JSON v0) generator SSOT unification)

- [x] Program(JSON v0) 生成ロジックを SSOT 化
  - new: `src/stage1/program_json_v0.rs`（`source_to_program_json_v0` / `ast_to_program_json_v0`）
  - export: `src/stage1/mod.rs` + `src/lib.rs` の `pub mod stage1;`
  - note: Stage1 Main body → Program(JSON v0) 変換責務を 1箇所へ集約
- [x] runner 側の重複実装を撤去
  - file: `src/runner/stage1_bridge/mod.rs`
  - change: `emit_program_json_v0` は `crate::stage1::program_json_v0::source_to_program_json_v0` 呼び出しに一本化
  - effect: `find_static_main_body` / `program_*_to_json_v0` 系の重複関数を削除
- [x] NyRT 側の重複実装を撤去
  - file: `crates/nyash_kernel/src/plugin/module_string_dispatch.rs`
  - change: `emit_program_json_v0` の module-string dispatch は `nyash_rust::stage1::program_json_v0::source_to_program_json_v0` を利用
  - effect: parser/AST 変換ロジックの二重管理を解消
- [x] verify:
  - `cargo test -q source_to_program_json_v0_minimal_main --lib` PASS
  - `cargo test -q -p nyash_kernel` PASS
  - `bash tools/selfhost/build_stage1.sh --artifact-kind stage1-cli --force-rebuild` PASS（capability=`OK (program-json)`）
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [ ] next task:
  - stage2 CLI artifact 生成 + `tools/selfhost_identity_check.sh --mode full --skip-build --cli-mode stage1 --bin-stage1 target/selfhost/hakorune.stage1_cli --bin-stage2 <stage2-cli>` の再実行（G1 close）

### Session Update (2026-02-08, G1 unblock probe: identity route hardening + real blocker capture)

- [x] `hakorune_emit_mir.sh` の direct fallback を fail-fast 化（偽成功を禁止）
  - file: `tools/hakorune_emit_mir.sh`
  - fix:
    - `--emit-mir-json` 成功扱い時に出力ファイルが空/未生成なら失敗にする
    - marker（`"functions"`）不在時も失敗にする
  - effect: `NYASH_BIN=target/selfhost/hakorune` で「`[OK] direct-emit` なのに out 不在」の契約崩れを即時検出
- [x] `selfhost_identity_check.sh` の stage1 route 契約を強化
  - file: `tools/selfhost_identity_check.sh`
  - change:
    - stage1 route を `stage1-subcmd` / `stage1-env` の2経路で受理
    - preflight は `run_stage1_route` を利用して artifact kind に応じて probe
    - full-mode route 判定を `^stage1` 系で評価
- [x] verify:
  - `bash -n tools/hakorune_emit_mir.sh` PASS
  - `bash -n tools/selfhost_identity_check.sh` PASS
  - `bash tools/selfhost/build_stage1.sh --artifact-kind stage1-cli --out target/selfhost/hakorune.stage2_cli --force-rebuild` PASS
  - `tools/selfhost_identity_check.sh --mode full --skip-build --cli-mode stage1 --bin-stage1 target/selfhost/hakorune.stage1_cli --bin-stage2 target/selfhost/hakorune.stage2` FAIL（expected: Stage2 は stage1-cli capability なし）
  - `tools/selfhost_identity_check.sh --mode full --skip-build --cli-mode stage1 --bin-stage1 target/selfhost/hakorune.stage1_cli --bin-stage2 target/selfhost/hakorune.stage2_cli` FAIL（Stage1 program-json failed）
  - `STAGE1_CLI_DEBUG=1 NYASH_STAGE1_MODE=emit-program ... compiler_stageb.hako` で `BuildBox returned null (smoke-only rc=0)` を確認
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [ ] remaining blocker (G1):
  - `target/selfhost/hakorune.stage1_cli` は smoke入力では Program(JSON) を返すが、`compiler_stageb.hako` では `BuildBox.emit_program_json_v0` が null になり payload を返さない
  - 現在の `stage1_cli.hako` は null/不正payloadを `rc=0` で握りつぶす（smoke-only fallback）
- [ ] next task (1-box, contract restore):
  - `lang/src/runner/stage1_cli.hako` の `emit-program` で null/不正payload を fail-fast（`rc=96` + 安定タグ）へ変更
  - `BuildBox.emit_program_json_v0` が `compiler_stageb.hako` で null になる原因を最小再現し、module roots / using merge 契約の不足を1箱で補う

### Session Update (2026-02-08, G1 close: stage1 emit contract restore)

- [x] `stage1_cli` emit-program を fail-fast 化（smoke-only 握りつぶしを撤去）
  - file: `lang/src/runner/stage1_cli.hako`
  - change:
    - `BuildBox.emit_program_json_v0 == null` は `rc=96` + `[freeze:contract][stage1-cli/emit-program]` で停止
    - Program(JSON v0) marker 欠落も `rc=96` で停止
- [x] Program(JSON v0) generator SSOT を実用サブセットへ拡張
  - file: `src/stage1/program_json_v0.rs`
  - change:
    - statement/expr 受理を拡張（If/Loop/Try/Call/Method/New/Logical/Compare/BlockExpr など）
    - `static box Main` は末尾優先で探索（using prelude に含まれる補助 Main を回避）
    - Rust parser が全体ソースを読めない場合は `Main.main` 本体抽出 fallback で再パース
- [x] NyRT module-string dispatch の失敗契約を固定
  - file: `crates/nyash_kernel/src/plugin/module_string_dispatch.rs`
  - change:
    - `emit_program_json_v0` 失敗時に `null` ではなく `[freeze:contract][stage1_program_json_v0] ...` を返す
  - test: `crates/nyash_kernel/src/tests.rs`
    - `invoke_by_name_build_box_unsupported_source_returns_freeze_tag` 追加
- [x] identity check の stage1-env mir route を parent-lower 契約へ同期
  - file: `tools/selfhost_identity_check.sh`
  - change:
    - stage1-cli artifact の `emit-mir` は stage0 parent (`target/release/hakorune`) で Program→MIR 変換して評価
    - route 判定は stage1-env/stage1-subcmd を維持
- [x] verify:
  - `cargo test -q source_to_program_json_v0_ --lib` PASS
  - `cargo test -q -p nyash_kernel invoke_by_name_build_box` PASS
  - `tools/selfhost_identity_check.sh --mode full --skip-build --cli-mode stage1 --bin-stage1 target/selfhost/hakorune.stage1_cli --bin-stage2 target/selfhost/hakorune.stage2_cli` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [x] G1 status:
  - Stage1 == Stage2 identity (full mode) 達成

### Session Update (2026-02-08, CLEAN-RUNTIME-2: host_api module split)

- [x] BoxShape（挙動不変）: `src/runtime/host_api.rs` の責務をサブモジュールへ分割
  - `src/runtime/host_api/common.rs`（TLV decode/encode + handle 復元）
  - `src/runtime/host_api/host_box_ops.rs`（InstanceBox/MapBox dispatch）
  - `src/runtime/host_api/host_array_ops.rs`（ArrayBox dispatch）
  - `src/runtime/host_api/host_string_ops.rs`（StringBox slot dispatch）
- [x] C ABI 入口（`nyrt_host_call_name` / `nyrt_host_call_slot`）は `src/runtime/host_api.rs` に残し、委譲のみへ薄化
- [x] verify:
  - `cargo check --bin hakorune` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [x] next task（CLEAN-JSONBRIDGE-2）:
  - `src/runner/json_v0_bridge/lowering/expr.rs` を `binary/unary/call/access` 系に段階分割し、入口APIは現状維持で薄化する

### Session Update (2026-02-08, CLEAN-JSONBRIDGE-2: expr lowering split)

- [x] BoxShape（挙動不変）: `src/runner/json_v0_bridge/lowering/expr.rs` から責務を子モジュールへ分割
  - `src/runner/json_v0_bridge/lowering/expr/binary_ops.rs`（Binary/Compare/Logical）
  - `src/runner/json_v0_bridge/lowering/expr/call_ops.rs`（Call/Method/New + stage-b static dispatch helper）
  - `src/runner/json_v0_bridge/lowering/expr/access_ops.rs`（Var resolve）
- [x] `lower_expr_with_scope` / `lower_expr_with_vars` の外部契約は維持（入口シグネチャ不変）
- [x] verify:
  - `cargo check --bin hakorune` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [x] next task（CLEAN-BOXFACTORY-1）:
  - `src/box_factory/mod.rs` の `FactoryPolicy` / `FactoryType` / `UnifiedBoxRegistry` 周辺を段階分割し、公開APIは維持する

### Session Update (2026-02-08, CLEAN-BOXFACTORY-1: policy/registry split)

- [x] BoxShape（挙動不変）: `src/box_factory/mod.rs` を入口専用に薄化
  - `src/box_factory/policy.rs` に `FactoryPolicy` / `FactoryType` を移設
  - `src/box_factory/registry.rs` に `UnifiedBoxRegistry` の本体実装を移設
  - `mod.rs` は `pub use policy::*` / `pub use registry::*` で公開APIを維持
- [x] 既存テスト契約を維持（`mod.rs` test からの参照経路は不変）
- [x] verify:
  - `cargo check --bin hakorune` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [x] next task（CLEAN-PLAN-V0-AUDIT-1）:
  - `src/mir/builder/control_flow/plan/loop_*_v0` の実使用箇所を inventory 化し、`REGISTRY.md` / SSOT docs と照合して「残す/縮退/削除候補」を明文化する（削除は別コミット）

### Session Update (2026-02-08, CLEAN-PLAN-V0-AUDIT-1: loop_*_v0 inventory)

- [x] `loop_*_v0` の実配線を inventory 化（facts→router→compose）
  - evidence scan: `joinir/patterns/registry/{mod,predicates,handlers}.rs` と `plan/recipe_tree/loop_cond_composer.rs`
- [x] 分類を `REGISTRY.md` に固定（keep / shrink-candidate）
  - keep: `loop_scan_v0` / `loop_scan_methods_v0` / `loop_scan_methods_block_v0` / `loop_scan_phi_vars_v0` / `loop_collect_using_entries_v0` / `loop_bundle_resolver_v0`
  - shrink-candidate: `loop_flag_exit_v0`（facts抽出のみ、router/composer entryなし）
- [x] inventory 結果を current_task に記録（このセクション）
- [x] next task（CLEAN-PLAN-V0-SHRINK-1）:
  - `loop_flag_exit_v0` を “facts-only活性” から縮退する最小差分を切る（候補: `has_any` と summary の整合化）。削除/配線変更は fixture+fast gate 固定で別コミット

### Session Update (2026-02-08, CLEAN-PLAN-V0-SHRINK-1: loop_flag_exit_v0 parked)

- [x] `loop_flag_exit_v0` を facts-only 活性から縮退（挙動不変）
  - file: `src/mir/builder/control_flow/plan/facts/loop_builder.rs`
  - change:
    - `try_extract_loop_flag_exit_v0_facts` 呼び出しを停止（`loop_flag_exit_v0 = None` へ固定）
    - `has_any` から `loop_flag_exit_v0.is_some()` を除外
- [x] `REGISTRY.md` の audit snapshot を parked 表記へ更新（`facts field only / inactive`）
- [x] verify:
  - `cargo check --bin hakorune` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [ ] next task（CLEAN-PLAN-V0-SHRINK-2）:
  - `loop_flag_exit_v0` の物理撤去可否を判定するため、`loop_types.rs` フィールド/正規化初期化/テスト初期化の依存を inventory 化し、削除時の最小差分境界を fixed にする

### Session Update (2026-02-08, CLEAN-PLAN-V0-SHRINK-2: removal boundary inventory)

- [x] `loop_flag_exit_v0` 物理撤去の依存境界を inventory 化して固定
  - Boundary A（必須）: `plan/mod.rs` / `facts/loop_types.rs` / `facts/loop_builder.rs` / `normalize/canonicalize.rs` / `loop_flag_exit_v0/*`
  - Boundary B（同コミットで追随）: `planner/build_tests.rs` / `verifier/tests.rs` / `composer/coreloop_*tests.rs` / `composer/coreloop_single_entry.rs` / `composer/coreloop_v2_nested_minimal.rs`
- [x] `REGISTRY.md` に removal boundary を追記（`CLEAN-PLAN-V0-SHRINK-2`）
- [x] next task（CLEAN-PLAN-V0-REMOVE-1）:
  - `loop_flag_exit_v0` の物理撤去を 1コミットで実施（Boundary A+B を同時更新）。削除後は `cargo check --bin hakorune` + `phase29bq_fast_gate_vm.sh --only bq` で契約固定

### Session Update (2026-02-08, CLEAN-PLAN-V0-REMOVE-1: loop_flag_exit_v0 physical removal)

- [x] `loop_flag_exit_v0` を物理撤去（Boundary A+B）
  - removed: `src/mir/builder/control_flow/plan/loop_flag_exit_v0/*`
  - removed wiring: `src/mir/builder/control_flow/plan/mod.rs`（module declaration）
  - removed facts field: `src/mir/builder/control_flow/plan/facts/loop_types.rs` / `src/mir/builder/control_flow/plan/facts/loop_builder.rs`
  - removed field init lines: `normalize/canonicalize.rs` / `planner/build_tests.rs` / `verifier/tests.rs` / `composer/coreloop_*`
- [x] `REGISTRY.md` を retired 状態へ更新（active table から除外）
- [x] verify:
  - `cargo check --bin hakorune` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [x] next task（CLEAN-PLAN-V0-FOLLOWUP-1）:
  - `loop_*_v0` retired/removal 履歴が増えて検索しづらくなっているため、`REGISTRY.md` の audit snapshot を `active` / `retired` に分割して参照導線を短くする（docs-only）

### Session Update (2026-02-08, CLEAN-PLAN-V0-FOLLOWUP-1: audit split active/retired)

- [x] `REGISTRY.md` の `loop_*_v0 audit snapshot` を `Active (routed)` / `Retired` に分割
  - Active table: 稼働中 `loop_scan_*` / `loop_collect_*` / `loop_bundle_*`
  - Retired table: `loop_flag_exit_v0`（物理撤去済み）
- [x] removal boundary セクションは維持し、撤去履歴の参照先として継続
- [x] next task（SELFHOST-MILESTONE-CHECK-1）:
  - docs-only cleanup の区切りとして selfhost milestone gate を1回実行し、`CURRENT_TASK.md` の Daily Checkpoint に最新実測（件数/秒）を追記する

### Session Update (2026-02-08, SELFHOST-MILESTONE-CHECK-1: full gate checkpoint)

- [x] selfhost milestone gate を 1回実行して最新実測を取得
  - command: `RUN_TIMEOUT_SECS=120 SMOKES_ENABLE_SELFHOST=1 HAKO_JOINIR_PLANNER_REQUIRED=1 bash tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_planner_required_dev_gate_vm.sh`
  - result: PASS（`182/182`, `total_secs=660`, `avg_case_secs=3.63`）
- [x] Daily Checkpoint（2026-02-08）へ実測を追記
- [x] next task（CLEAN-PLAN-V0-FOLLOWUP-2）:
  - `REGISTRY.md` の `loop_flag_exit_v0 removal boundary` を `retired-history` 節へ移動し、active運用者向けの読み始め導線をさらに短くする（docs-only）

### Session Update (2026-02-08, CLEAN-PLAN-V0-FOLLOWUP-2: retired-history compaction)

- [x] `REGISTRY.md` の audit 節を active運用向けに短縮
  - `Audit rule (active運用)` と `Retired History (read-only)` を分離
  - `loop_flag_exit_v0` removal boundary の長文一覧は `CURRENT_TASK.md` 参照へ一本化
- [x] next task（CLEAN-GENERIC-LOOP-SPLIT-1）:
  - `src/mir/builder/control_flow/plan/canon/generic_loop.rs` の責務（観測/ポリシー/ヒューリスティクス）を inventory し、分割境界（A:観測, B:判定, C:ヒューリスティクス）を SSOT に固定する

### Session Update (2026-02-08, CLEAN-GENERIC-LOOP-SPLIT-1: canon generic_loop split)

- [x] `generic_loop` canon を façade + submodule 構成へ分割（挙動不変）
  - `src/mir/builder/control_flow/plan/canon/generic_loop.rs`: type 定義 + re-export の入口のみ
  - `src/mir/builder/control_flow/plan/canon/generic_loop/condition.rs`: loop 条件観測（候補抽出/Bound 観測）
  - `src/mir/builder/control_flow/plan/canon/generic_loop/update.rs`: loop 更新観測（`canon_update_for_loop_var`）
  - `src/mir/builder/control_flow/plan/canon/generic_loop/step/{extract,placement}.rs`: increment 抽出 / placement 判定
- [x] SSOT/導線更新
  - `src/mir/builder/control_flow/plan/canon/README.md`
  - `src/mir/builder/control_flow/plan/generic_loop/README.md`
  - `docs/development/current/main/design/condition-observation-ssot.md`
  - `docs/development/current/main/10-Now.md`
- [x] verify:
  - `cargo check --bin hakorune` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [x] next task（CLEAN-GENERIC-LOOP-SPLIT-2）:
  - `src/mir/builder/control_flow/plan/canon/generic_loop/step/extract.rs` を `var_step` / `next_step` / `complex_step` に薄く分け、判定順序を `generic_loop/README.md` に固定する（挙動不変）

### Session Update (2026-02-08, CLEAN-GENERIC-LOOP-SPLIT-2: step extract split)

- [x] `step/extract.rs` を façade 化し、抽出ヒューリスティクスを責務別に分割（挙動不変）
  - `src/mir/builder/control_flow/plan/canon/generic_loop/step/extract.rs`: entry + 順序固定のみ
  - `src/mir/builder/control_flow/plan/canon/generic_loop/step/extract/var_step.rs`: var-step 系（direct/tail 形）
  - `src/mir/builder/control_flow/plan/canon/generic_loop/step/extract/next_step.rs`: `next_i` 系
  - `src/mir/builder/control_flow/plan/canon/generic_loop/step/extract/complex_step.rs`: 複合式 step 系
  - `src/mir/builder/control_flow/plan/canon/generic_loop/step/extract/shared.rs`: 共通 assignment 収集
- [x] 判定順序 SSOT を `src/mir/builder/control_flow/plan/generic_loop/README.md` に追記
- [x] verify:
  - `cargo check --bin hakorune`
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`
- [x] next task（CLEAN-GENERIC-LOOP-SPLIT-3）:
  - `src/mir/builder/control_flow/plan/canon/generic_loop/step/placement.rs` を `matcher` / `decision` に分け、`RejectReason` 判定を decision 側へ集約する（挙動不変）

### Session Update (2026-02-08, CLEAN-GENERIC-LOOP-SPLIT-3: step placement split)

- [x] `step/placement.rs` を façade 化し、責務を `matcher` / `decision` へ分離（挙動不変）
  - `src/mir/builder/control_flow/plan/canon/generic_loop/step/placement.rs`: entry + re-export のみ
  - `src/mir/builder/control_flow/plan/canon/generic_loop/step/placement/matcher.rs`: step shape マッチ群
  - `src/mir/builder/control_flow/plan/canon/generic_loop/step/placement/decision.rs`: `RejectReason` を含む placement 決定
- [x] SSOT 更新:
  - `src/mir/builder/control_flow/plan/generic_loop/README.md` に placement split を追記
- [x] verify:
  - `cargo check --bin hakorune`
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`
- [x] next task（CLEAN-GENERIC-LOOP-SPLIT-4）:
  - `src/mir/builder/control_flow/plan/canon/generic_loop/condition.rs` を `candidates` / `bound` 観測に薄く分割し、`CondProfile` 生成責務を入口に固定する（挙動不変）

### Session Update (2026-02-08, CLEAN-GENERIC-LOOP-SPLIT-4: condition split)

- [x] `condition.rs` を façade 化し、観測責務を分離（挙動不変）
  - `src/mir/builder/control_flow/plan/canon/generic_loop/condition.rs`: entry + `CondProfile` 生成のみ
  - `src/mir/builder/control_flow/plan/canon/generic_loop/condition/candidates.rs`: loop_var candidate 観測
  - `src/mir/builder/control_flow/plan/canon/generic_loop/condition/bound.rs`: bound 観測
- [x] SSOT 更新:
  - `src/mir/builder/control_flow/plan/generic_loop/README.md` に condition split を追記
- [x] verify:
  - `cargo check --bin hakorune`
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`
- [x] next task（CLEAN-GENERIC-LOOP-SPLIT-5）:
  - `src/mir/builder/control_flow/plan/canon/generic_loop/update.rs` を `pattern_match` / `literal_step` 補助へ薄く分け、`UpdateCanon` 判定責務を入口に固定する（挙動不変）

### Session Update (2026-02-08, CLEAN-GENERIC-LOOP-SPLIT-5: update split)

- [x] `update.rs` を façade 化し、判定責務を分離（挙動不変）
  - `src/mir/builder/control_flow/plan/canon/generic_loop/update.rs`: entry のみ
  - `src/mir/builder/control_flow/plan/canon/generic_loop/update/pattern_match.rs`: update pattern 観測
  - `src/mir/builder/control_flow/plan/canon/generic_loop/update/literal_step.rs`: `UpdateCanon` 生成
- [x] SSOT 更新:
  - `src/mir/builder/control_flow/plan/generic_loop/README.md` に update split を追記
- [x] verify:
  - `cargo check --bin hakorune`
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`
- [x] next task（CLEAN-GENERIC-LOOP-SPLIT-6）:
  - `src/mir/builder/control_flow/plan/canon/generic_loop.rs` の型定義（`ConditionCanon`/`UpdateCanon`/`StepPlacement*`）を `types.rs` へ分離し、入口ファイルを export 専用に縮退する（挙動不変）

### Session Update (2026-02-08, CLEAN-GENERIC-LOOP-SPLIT-6: types split)

- [x] `generic_loop.rs` の観測型を `types.rs` へ分離（挙動不変）
  - `src/mir/builder/control_flow/plan/canon/generic_loop/types.rs`: `ConditionCanon` / `UpdateCanon` / `StepPlacement*`
  - `src/mir/builder/control_flow/plan/canon/generic_loop.rs`: module export 入口へ縮退
- [x] SSOT 更新:
  - `src/mir/builder/control_flow/plan/generic_loop/README.md` に type split を追記
- [x] verify:
  - `cargo check --bin hakorune`
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`
- [x] next task（CLEAN-JSONV0-BRIDGE-SPLIT-1）:
  - `src/runner/json_v0_bridge/lowering.rs` の責務 inventory（expr/stmt/scope-exit/legacy lambda）を docs に固定し、次の物理分割境界を 1 コミット単位で定義する（docs-only）

### Session Update (2026-02-08, CLEAN-JSONV0-BRIDGE-SPLIT-1: lowering inventory SSOT)

- [x] JSON v0 bridge lowering の責務棚卸しを SSOT 化（docs-only）
  - added: `docs/development/current/main/design/json-v0-bridge-lowering-split-ssot.md`
  - inventory: loop runtime state / env+function bootstrap / stmt dispatcher / program assembly / mir dump
  - split queue: `CLEAN-JSONV0-BRIDGE-SPLIT-{2..5}` を 1コミット単位で固定
- [x] no-code-change check:
  - `git diff --name-only` が docs と `CURRENT_TASK.md` のみ
- [x] next task（CLEAN-JSONV0-BRIDGE-SPLIT-2）:
  - `src/runner/json_v0_bridge/lowering.rs` の loop runtime state（snapshot/hint/jump/break/continue helper）を `lowering/loop_runtime.rs` に抽出し、`lowering.rs` を入口配線だけに薄くする（挙動不変）

### Session Update (2026-02-08, CLEAN-JSONV0-BRIDGE-SPLIT-2: loop runtime extraction)

- [x] loop runtime state を `lowering/loop_runtime.rs` へ抽出（挙動不変）
  - added: `src/runner/json_v0_bridge/lowering/loop_runtime.rs`
  - moved: snapshot stack / increment hint stack / break-continue jump helper
  - updated: `src/runner/json_v0_bridge/lowering.rs` は loop runtime 本体を持たず、`loop_runtime` 呼び出しへ委譲
  - updated: `src/runner/json_v0_bridge/lowering/loop_.rs` の loop snapshot/hint API 呼び出しを `super::loop_runtime::*` に統一
- [x] verify:
  - `cargo check --bin hakorune`
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`
- [x] next task（CLEAN-JSONV0-BRIDGE-SPLIT-3）:
  - `src/runner/json_v0_bridge/lowering.rs` の program assembly のうち defs lower + call resolve（R4）を `lowering/program.rs` へ抽出し、entry `args` 注入と call-resolve toggle の契約を維持する（挙動不変）

### Session Update (2026-02-08, CLEAN-JSONV0-BRIDGE-SPLIT-3: program assembly extraction)

- [x] program assembly の defs lower + call resolve（R4）を `lowering/program.rs` へ抽出（挙動不変）
  - added: `src/runner/json_v0_bridge/lowering/program.rs`
  - moved: `lower_main_body` / `lower_defs_into_module` / `maybe_resolve_calls`
  - updated: `src/runner/json_v0_bridge/lowering.rs` の `lower_program` は env/static-method 初期化 + helper 呼び出しの入口へ縮退
  - updated: `FunctionDefBuilder` の可視性を `program.rs` 参照に必要な最小範囲（`pub(super)`）に調整
- [x] verify:
  - `cargo check --bin hakorune`
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`
- [x] next task（CLEAN-JSONV0-BRIDGE-SPLIT-4）:
  - `src/runner/json_v0_bridge/lowering.rs` の statement dispatcher（`lower_stmt_with_vars` / `lower_stmt_list_with_vars`）を `lowering/stmts.rs` へ抽出し、legacy window の順序 `if_legacy -> while_legacy -> lambda_legacy -> default` を固定したまま入口配線へ縮退する（挙動不変）

### Session Update (2026-02-08, CLEAN-JSONV0-BRIDGE-SPLIT-4: statement dispatcher extraction)

- [x] statement dispatcher を `lowering/stmts.rs` へ抽出（挙動不変）
  - added: `src/runner/json_v0_bridge/lowering/stmts.rs`
  - moved: `lower_stmt_with_vars` / `lower_stmt_list_with_vars` の本体
  - updated: `src/runner/json_v0_bridge/lowering.rs` は同名の薄い委譲 façade を維持し、既存 call site を壊さずに入口配線へ縮退
  - contract: legacy window 順序 `if_legacy -> while_legacy -> lambda_legacy -> default` は `stmts.rs` 側で不変
- [x] verify:
  - `cargo check --bin hakorune`
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`
- [x] next task（CLEAN-JSONV0-BRIDGE-SPLIT-5）:
  - `src/runner/json_v0_bridge/lowering.rs` の `maybe_dump_mir` と関連 test を `lowering/dump.rs` へ抽出し、`RUST_MIR_DUMP_PATH` / `cli_verbose` の挙動契約を維持したまま入口配線へ縮退する（挙動不変）

### Session Update (2026-02-08, CLEAN-JSONV0-BRIDGE-SPLIT-5: dump helper extraction)

- [x] MIR dump helper を `lowering/dump.rs` へ抽出（挙動不変）
  - added: `src/runner/json_v0_bridge/lowering/dump.rs`
  - moved: `maybe_dump_mir` と dump path 挙動テスト（`RUST_MIR_DUMP_PATH` set/unset）
  - updated: `src/runner/json_v0_bridge/lowering.rs` の `maybe_dump_mir` は委譲 façade に縮退
  - contract: `RUST_MIR_DUMP_PATH` と `cli_verbose` の既存挙動は不変
- [x] verify:
  - `cargo check --bin hakorune`
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`
- [x] next task（CLEAN-JSONV0-BRIDGE-POSTSPLIT-1）:
  - split SSOT（`json-v0-bridge-lowering-split-ssot.md`）を完了状態に更新し、`lowering.rs` 残存責務（R2: BridgeEnv/FunctionDefBuilder）を「追加分割するか/現状維持するか」の判断基準として明文化する（docs-only）

### Session Update (2026-02-08, CLEAN-JSONV0-BRIDGE-POSTSPLIT-1: split completion + R2 criteria)

- [x] split SSOT を完了状態へ更新（docs-only）
  - updated: `docs/development/current/main/design/json-v0-bridge-lowering-split-ssot.md`
  - reflected: `CLEAN-JSONV0-BRIDGE-SPLIT-{2..5}` 完了（commit hash 紐づけ）
  - clarified: 残存R2（`BridgeEnv` / `FunctionDefBuilder`）の追加分割トリガーと現状維持条件
- [x] no-code-change check:
  - `git diff --name-only` が docs と `CURRENT_TASK.md` のみ
- [x] next task（G1-UNBLOCK-STAGE1-CLI-PAYLOAD）:
  - `target/selfhost/hakorune.stage1_cli` の env-route で Program(JSON) payload 出力を回復し、`tools/selfhost_identity_check.sh --mode full --skip-build --cli-mode stage1 --bin-stage1 target/selfhost/hakorune.stage1_cli --bin-stage2 <stage2-cli>` の full identity 再検証へ戻る

### Session Update (2026-02-08, G1 closeout re-verify + identity payload hardening)

- [x] stage1-cli env-route の Program(JSON) payload を再採取して復旧状態を確認
  - probe: `target/selfhost/hakorune.stage1_cli` / `target/selfhost/hakorune.stage2_cli` の raw stdout は `Program` JSON line + `Result: 0` trailer（JSON line 出力あり）
- [x] identity payload 判定を fail-fast 強化（false positive 防止）
  - updated: `tools/selfhost/lib/identity_routes.sh`
    - `extract_marker_line_to_file` を `extract_json_object_line_to_file` に置換（JSON object line のみ抽出）
  - updated: `tools/selfhost/lib/identity_compare.sh`
    - `program-json`/`mir-json` ともに `Result:` trailer 混入を reject
    - `program-json` は `kind=Program` + `version=0` を必須化
    - `program-json`/`mir-json` は JSON object 開始（`{`）を必須化
- [x] verify:
  - `tools/selfhost_identity_check.sh --mode full --skip-build --cli-mode stage1 --bin-stage1 target/selfhost/hakorune.stage1_cli --bin-stage2 target/selfhost/hakorune.stage2_cli` PASS
  - `tools/selfhost_identity_check.sh --mode smoke --skip-build --cli-mode auto --bin-stage1 target/release/hakorune --bin-stage2 target/release/hakorune` PASS
- [x] next task（G2-CANARY-BASELINE-REFRESH）:
  - full selfhost canary を節目実行（1回）して baseline を更新し、`planner_required_selfhost_subset.tsv` の運用注記（daily cadence / heavy gate cadence）を `29bq-90` と `CURRENT_TASK.md` に同期する

### Session Update (2026-02-08, G2 canary baseline refresh)

- [x] full selfhost canary（milestone-only）を 1回実行して baseline を更新
  - command: `RUN_TIMEOUT_SECS=120 SMOKES_ENABLE_SELFHOST=1 HAKO_JOINIR_PLANNER_REQUIRED=1 bash tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_planner_required_dev_gate_vm.sh`
  - result: PASS（`182/182`, `stageb_total_secs=681`, `run_total_secs=1`, `total_secs=682`, `avg_case_secs=3.75`）
- [x] cadence note を SSOT に同期（`CURRENT_TASK.md` / `29bq-90`）
- [x] next task（PARSER-HANDOFF-MICROLOOP-RESUME）:
  - `29bq-93` から 1件だけ選んで PROBE→PROMOTE（fixture + subset + quick gate）を 1コミットで固定する

### Session Update (2026-02-08, selfhost gate parallel jobs option)

- [x] selfhost gate に並列実行オプションを追加（既定直列は維持）
  - updated: `tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_planner_required_dev_gate_vm.sh`
  - contract: `SMOKES_SELFHOST_JOBS=<n>`（`>=1`）を追加。`1` は従来どおり直列、`>1` は case 並列実行
- [x] unified entry から `--jobs <n>` を指定可能化
  - updated: `tools/selfhost/run.sh`
- [x] SSOT/checklist を同期
  - updated: `docs/development/current/main/phases/phase-29bq/29bq-90-selfhost-checklist.md`
  - updated: `docs/development/current/main/design/selfhost-bootstrap-route-ssot.md`
- [x] verify:
  - `bash -n tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_planner_required_dev_gate_vm.sh`
  - `bash -n tools/selfhost/run.sh`
  - `SMOKES_ENABLE_SELFHOST=1 HAKO_JOINIR_PLANNER_REQUIRED=1 SMOKES_SELFHOST_MAX_CASES=4 SMOKES_SELFHOST_JOBS=2 bash tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_planner_required_dev_gate_vm.sh` PASS

### Session Update (2026-02-08, selfhost gate jobs default switch)

- [x] selfhost gate の既定 parallel jobs を `1 -> 4` に変更（必要時は `SMOKES_SELFHOST_JOBS=1` で直列化）
  - updated: `tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_planner_required_dev_gate_vm.sh`
  - updated: `tools/selfhost/run.sh`（`--gate` で `--jobs` 未指定時は `4` を設定）
- [x] SSOT/checklist 同期:
  - `docs/development/current/main/design/selfhost-bootstrap-route-ssot.md`
  - `docs/development/current/main/phases/phase-29bq/29bq-90-selfhost-checklist.md`
- [x] verify:
  - `tools/selfhost/run.sh --gate --planner-required 1 --max-cases 4 --timeout-secs 120` PASS（`parallel mode enabled (jobs=4)`）

### Session Update (2026-02-08, parser handoff Tier-4 control-loop promote)

- [x] Tier-4 backlog を追加し、`loop + if + break/continue + cleanup` の 1件を PROMOTE
  - added: `docs/development/current/main/phases/phase-29bq/29bq-95-parser-handoff-tier4-backlog.md`
  - added: `apps/tests/phase29bq_selfhost_control_loop_if_break_continue_cleanup_min.hako`（expected=`171`）
  - updated: `tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv`
- [x] verify:
  - `SMOKES_SELFHOST_LIST=/tmp/selfhost_probe_tier4.tsv tools/selfhost/run.sh --gate --planner-required 1 --max-cases 1 --timeout-secs 120` PASS
  - `tools/selfhost/run.sh --gate --planner-required 1 --filter control_loop_if_break_continue_cleanup_min --max-cases 1 --timeout-secs 120` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
  - `RUN_TIMEOUT_SECS=120 SMOKES_ENABLE_SELFHOST=1 HAKO_JOINIR_PLANNER_REQUIRED=1 bash tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_planner_required_dev_gate_vm.sh` PASS（`183/183`, `stageb_total_secs=779`, `run_total_secs=0`, `total_secs=779`, `avg_case_secs=4.26`）
- [x] next task（PARSER-HANDOFF-TIER5-BACKLOG-BOOTSTRAP）:
  - Tier-5 backlog（`control-flow + local/fini`）を定義し、最小1件を PROBE→PROMOTE する

### Session Update (2026-02-08, parser handoff Tier-5 control+local-fini promote)

- [x] Tier-5 backlog を追加し、`control-flow + local/fini + cleanup` の 1件を PROMOTE
  - added: `docs/development/current/main/phases/phase-29bq/29bq-96-parser-handoff-tier5-backlog.md`
  - added: `apps/tests/phase29bq_selfhost_control_loop_local_fini_cleanup_min.hako`（expected=`156`）
  - updated: `tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv`
- [x] verify:
  - `SMOKES_SELFHOST_LIST=/tmp/selfhost_probe_tier5.tsv tools/selfhost/run.sh --gate --planner-required 1 --max-cases 1 --timeout-secs 120` PASS
  - `tools/selfhost/run.sh --gate --planner-required 1 --filter control_loop_local_fini_cleanup_min --max-cases 1 --timeout-secs 120` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
  - `RUN_TIMEOUT_SECS=120 SMOKES_ENABLE_SELFHOST=1 HAKO_JOINIR_PLANNER_REQUIRED=1 bash tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_planner_required_dev_gate_vm.sh` PASS（`184/184`, `stageb_total_secs=785`, `run_total_secs=2`, `total_secs=787`, `avg_case_secs=4.28`）
- [x] next task（PARSER-HANDOFF-TIER6-BACKLOG-BOOTSTRAP）:
  - Tier-6 backlog（`Box member + control-flow/cleanup`）を定義し、最小1件を PROBE→PROMOTE する

### Session Update (2026-02-08, parser handoff Tier-6 box-member promote)

- [x] Tier-6 backlog を追加し、`Box member + loop/if + cleanup` の 1件を PROMOTE
  - added: `docs/development/current/main/phases/phase-29bq/29bq-97-parser-handoff-tier6-backlog.md`
  - added: `apps/tests/phase29bq_selfhost_box_member_loop_cleanup_min.hako`（expected=`47`）
  - updated: `tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv`
- [x] verify:
  - `SMOKES_SELFHOST_LIST=/tmp/selfhost_probe_tier6.tsv tools/selfhost/run.sh --gate --planner-required 1 --max-cases 1 --timeout-secs 120` PASS
  - `tools/selfhost/run.sh --gate --planner-required 1 --filter box_member_loop_cleanup_min --max-cases 1 --timeout-secs 120` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
  - `RUN_TIMEOUT_SECS=120 SMOKES_ENABLE_SELFHOST=1 HAKO_JOINIR_PLANNER_REQUIRED=1 bash tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_planner_required_dev_gate_vm.sh` PASS（`185/185`, `stageb_total_secs=818`, `run_total_secs=2`, `total_secs=820`, `avg_case_secs=4.43`）
- [x] next task（PARSER-HANDOFF-TIER7-BACKLOG-BOOTSTRAP）:
  - Tier-7 backlog（`Box member + local/fini + cleanup`）を定義し、最小1件を PROBE→PROMOTE する

### Session Update (2026-02-08, parser handoff Tier-7 box-member+local-fini promote)

- [x] Tier-7 backlog を追加し、`Box member + loop/if + local/fini + cleanup` の 1件を PROMOTE
  - added: `docs/development/current/main/phases/phase-29bq/29bq-98-parser-handoff-tier7-backlog.md`
  - added: `apps/tests/phase29bq_selfhost_box_member_local_fini_cleanup_min.hako`（expected=`29`）
  - updated: `tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv`
- [x] verify:
  - `SMOKES_SELFHOST_LIST=/tmp/selfhost_probe_tier7.tsv tools/selfhost/run.sh --gate --planner-required 1 --max-cases 1 --timeout-secs 120` PASS
  - `tools/selfhost/run.sh --gate --planner-required 1 --filter box_member_local_fini_cleanup_min --max-cases 1 --timeout-secs 120` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
  - `RUN_TIMEOUT_SECS=120 SMOKES_ENABLE_SELFHOST=1 HAKO_JOINIR_PLANNER_REQUIRED=1 bash tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_planner_required_dev_gate_vm.sh` PASS（`186/186`, `stageb_total_secs=798`, `run_total_secs=9`, `total_secs=807`, `avg_case_secs=4.34`）
- [x] note:
  - 現状契約では `FiniReg + finally` を含む本fixtureの実行結果は `29`（構文受理と経路固定を優先してPROMOTE）
- [x] next task（PARSER-HANDOFF-TIER8-BACKLOG-BOOTSTRAP）:
  - Tier-8 backlog（`Box member + local/fini + cleanup + blockexpr`）を定義し、最小1件を PROBE→PROMOTE する

### Session Update (2026-02-08, parser handoff Tier-8 box-member+local-fini+blockexpr promote)

- [x] Tier-8 backlog を追加し、`Box member + loop/if + local/fini + blockexpr + cleanup` の 1件を PROMOTE
  - added: `docs/development/current/main/phases/phase-29bq/29bq-99-parser-handoff-tier8-backlog.md`
  - added: `apps/tests/phase29bq_selfhost_box_member_local_fini_blockexpr_cleanup_min.hako`（expected=`115`）
  - updated: `tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv`
- [x] verify:
  - `SMOKES_SELFHOST_LIST=/tmp/selfhost_probe_tier8.tsv tools/selfhost/run.sh --gate --planner-required 1 --max-cases 1 --timeout-secs 120` PASS
  - `tools/selfhost/run.sh --gate --planner-required 1 --filter box_member_local_fini_blockexpr_cleanup_min --max-cases 1 --timeout-secs 120` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
  - `RUN_TIMEOUT_SECS=120 SMOKES_ENABLE_SELFHOST=1 HAKO_JOINIR_PLANNER_REQUIRED=1 bash tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_planner_required_dev_gate_vm.sh` PASS（`187/187`, `stageb_total_secs=792`, `run_total_secs=6`, `total_secs=798`, `avg_case_secs=4.27`）
- [x] next task（PARSER-HANDOFF-TIER9-BACKLOG-BOOTSTRAP）:
  - Tier-9 backlog（`Box member + local/fini + cleanup + blockexpr + compare/logic`）を定義し、最小1件を PROBE→PROMOTE する

### Session Update (2026-02-08, CLEAN-JSONV0-SPLIT-6 + boundary docs)

- [x] subset TSV の planner tag 破損を修正（`phase29bq_selfhost_control_loop_if_break_continue_cleanup_min.hako` 行の `rule=LoopCondBreak` 欠落を復旧）
- [x] BoxShape split: `src/runner/json_v0_bridge/lowering/expr.rs` から BlockExpr 専用ロジックを `src/runner/json_v0_bridge/lowering/expr/block_expr.rs` へ分離（挙動不変）
- [x] json_v0 split SSOT を更新（`CLEAN-JSONV0-BRIDGE-SPLIT-6` を記録）
- [x] plan legacy-v0 境界ドキュメントを追加
  - `src/mir/builder/control_flow/plan/LEGACY_V0_BOUNDARY.md`
  - `src/mir/builder/control_flow/plan/ARCHITECTURE.md` / `src/mir/builder/control_flow/plan/REGISTRY.md` へ導線追加
- [x] fini/cleanup 実行契約ドキュメントを追加
  - `docs/development/current/main/design/fini-cleanup-execution-contract-ssot.md`
  - `docs/development/current/main/design/README.md` へ導線追加

### Session Update (2026-02-08, parser handoff Tier-9 compare/logic promote)

- [x] Tier-9 backlog を追加し、`Box member + local/fini + blockexpr + compare/logic + cleanup` の 1件を PROMOTE
  - added: `docs/development/current/main/phases/phase-29bq/29bq-100-parser-handoff-tier9-backlog.md`
  - added: `apps/tests/phase29bq_selfhost_box_member_local_fini_blockexpr_compare_logic_cleanup_min.hako`（expected=`118`）
  - updated: `tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv`
  - updated: `docs/development/current/main/phases/phase-29bq/29bq-92-parser-handoff-checklist.md`（Tier-9導線）
- [x] verify:
  - `SMOKES_SELFHOST_LIST=/tmp/selfhost_probe_tier9.tsv tools/selfhost/run.sh --gate --planner-required 1 --max-cases 1 --timeout-secs 120` PASS（expected=`118`）
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-08, parser handoff Tier-10 unary/call promote)

- [x] Tier-10 backlog を追加し、`Box member + local/fini + blockexpr + compare/logic + unary/call + cleanup` の 1件を PROMOTE
  - added: `docs/development/current/main/phases/phase-29bq/29bq-101-parser-handoff-tier10-backlog.md`
  - added: `apps/tests/phase29bq_selfhost_box_member_local_fini_blockexpr_compare_logic_unary_call_cleanup_min.hako`（expected=`113`）
  - updated: `tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv`
  - updated: `docs/development/current/main/phases/phase-29bq/29bq-92-parser-handoff-checklist.md`（Tier-10導線）
- [x] verify:
  - `SMOKES_SELFHOST_LIST=/tmp/selfhost_probe_tier10.tsv tools/selfhost/run.sh --gate --planner-required 1 --max-cases 1 --timeout-secs 120` PASS（expected=`113`）
  - `tools/selfhost/run.sh --gate --planner-required 1 --filter box_member_local_fini_blockexpr_compare_logic_unary_call_cleanup_min --max-cases 1 --timeout-secs 120` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-08, parser handoff Tier-11 literals promote)

- [x] Tier-11 backlog を追加し、`Box member + local/fini + blockexpr + compare/logic + unary/call + null/array-map literal + cleanup` の 1件を PROMOTE
  - added: `docs/development/current/main/phases/phase-29bq/29bq-102-parser-handoff-tier11-backlog.md`
  - added: `apps/tests/phase29bq_selfhost_box_member_local_fini_blockexpr_compare_logic_unary_call_literals_cleanup_min.hako`（expected=`111`）
  - updated: `tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv`
  - updated: `docs/development/current/main/phases/phase-29bq/29bq-92-parser-handoff-checklist.md`（Tier-11導線）
- [x] verify:
  - `SMOKES_SELFHOST_LIST=/tmp/selfhost_probe_tier11.tsv tools/selfhost/run.sh --gate --planner-required 1 --max-cases 1 --timeout-secs 120` PASS（expected=`111`）
  - `tools/selfhost/run.sh --gate --planner-required 1 --filter box_member_local_fini_blockexpr_compare_logic_unary_call_literals_cleanup_min --max-cases 1 --timeout-secs 120` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-08, parser handoff Tier-12 nested-tail promote)

- [x] Tier-12 backlog を追加し、`Box member + local/fini + blockexpr + compare/logic + unary/call + null/array-map literal + nested if/else tail + cleanup` の 1件を PROMOTE
  - added: `docs/development/current/main/phases/phase-29bq/29bq-103-parser-handoff-tier12-backlog.md`
  - added: `apps/tests/phase29bq_selfhost_box_member_local_fini_blockexpr_compare_logic_unary_call_literals_nested_tail_cleanup_min.hako`（expected=`111`）
  - updated: `tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv`
  - updated: `docs/development/current/main/phases/phase-29bq/29bq-92-parser-handoff-checklist.md`（Tier-12導線）
- [x] verify:
  - `SMOKES_SELFHOST_LIST=/tmp/selfhost_probe_tier12.tsv tools/selfhost/run.sh --gate --planner-required 1 --max-cases 1 --timeout-secs 120` PASS（expected=`111`）
  - `tools/selfhost/run.sh --gate --planner-required 1 --filter box_member_local_fini_blockexpr_compare_logic_unary_call_literals_nested_tail_cleanup_min --max-cases 1 --timeout-secs 120` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [x] next task（PARSER-HANDOFF-TIER13-BACKLOG-BOOTSTRAP）:
  - Tier-13 backlog（`Box member + local/fini + cleanup + blockexpr + compare/logic + unary/call + null/array-map literal + nested if/else tail + nested loop/branch mix`）を定義し、最小1件を PROBE→PROMOTE する

### Session Update (2026-02-08, parser handoff Tier-13 nested-loop-branch promote)

- [x] Tier-13 backlog を追加し、`Box member + local/fini + blockexpr + compare/logic + unary/call + null/array-map literal + nested if/else tail + nested loop/branch mix + cleanup` の 1件を PROMOTE
  - added: `docs/development/current/main/phases/phase-29bq/29bq-104-parser-handoff-tier13-backlog.md`
  - added: `apps/tests/phase29bq_selfhost_box_member_local_fini_blockexpr_compare_logic_unary_call_literals_nested_tail_nested_loop_branch_cleanup_min.hako`（expected=`111`）
  - updated: `tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv`
  - updated: `docs/development/current/main/phases/phase-29bq/29bq-92-parser-handoff-checklist.md`（Tier-13導線）
- [x] verify:
  - `SMOKES_SELFHOST_LIST=/tmp/selfhost_probe_tier13.tsv tools/selfhost/run.sh --gate --planner-required 1 --max-cases 1 --timeout-secs 120` PASS（expected=`111`）
  - `tools/selfhost/run.sh --gate --planner-required 1 --filter box_member_local_fini_blockexpr_compare_logic_unary_call_literals_nested_tail_nested_loop_branch_cleanup_min --max-cases 1 --timeout-secs 120` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [x] next task（PARSER-HANDOFF-TIER14-BACKLOG-BOOTSTRAP）:
  - Tier-14 backlog（`Box member + local/fini + cleanup + blockexpr + compare/logic + unary/call + null/array-map literal + nested if/else tail + nested loop/branch mix + method-chain tail`）を定義し、最小1件を PROBE→PROMOTE する

### Session Update (2026-02-08, parser handoff Tier-14 method-chain-tail promote)

- [x] Tier-14 backlog を追加し、`Box member + local/fini + blockexpr + compare/logic + unary/call + null/array-map literal + nested if/else tail + nested loop/branch mix + method-chain tail + cleanup` の 1件を PROMOTE
  - added: `docs/development/current/main/phases/phase-29bq/29bq-105-parser-handoff-tier14-backlog.md`
  - added: `apps/tests/phase29bq_selfhost_box_member_local_fini_blockexpr_compare_logic_unary_call_literals_nested_tail_nested_loop_branch_method_chain_tail_cleanup_min.hako`（expected=`111`）
  - updated: `tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv`
  - updated: `docs/development/current/main/phases/phase-29bq/29bq-92-parser-handoff-checklist.md`（Tier-14導線）
- [x] verify:
  - `SMOKES_SELFHOST_LIST=/tmp/selfhost_probe_tier14.tsv tools/selfhost/run.sh --gate --planner-required 1 --max-cases 1 --timeout-secs 120` PASS（expected=`111`）
  - `tools/selfhost/run.sh --gate --planner-required 1 --filter box_member_local_fini_blockexpr_compare_logic_unary_call_literals_nested_tail_nested_loop_branch_method_chain_tail_cleanup_min --max-cases 1 --timeout-secs 120` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [x] next task（PARSER-HANDOFF-TIER15-BACKLOG-BOOTSTRAP）:
  - Tier-15 backlog（`Box member + local/fini + cleanup + blockexpr + compare/logic + unary/call + null/array-map literal + nested if/else tail + nested loop/branch mix + method-chain tail + side-effect tail`）を定義し、最小1件を PROBE→PROMOTE する

### Session Update (2026-02-08, parser handoff Tier-15 side-effect-tail promote)

- [x] Tier-15 backlog を追加し、`Box member + local/fini + blockexpr + compare/logic + unary/call + null/array-map literal + nested if/else tail + nested loop/branch mix + method-chain tail + side-effect tail + cleanup` の 1件を PROMOTE
  - added: `docs/development/current/main/phases/phase-29bq/29bq-106-parser-handoff-tier15-backlog.md`
  - added: `apps/tests/phase29bq_selfhost_box_member_local_fini_blockexpr_compare_logic_unary_call_literals_nested_tail_nested_loop_branch_method_chain_tail_side_effect_tail_cleanup_min.hako`（expected=`111`）
  - updated: `tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv`
  - updated: `docs/development/current/main/phases/phase-29bq/29bq-92-parser-handoff-checklist.md`（Tier-15導線）
- [x] verify:
  - `SMOKES_SELFHOST_LIST=/tmp/selfhost_probe_tier15.tsv tools/selfhost/run.sh --gate --planner-required 1 --max-cases 1 --timeout-secs 120` PASS（expected=`111`）
  - `tools/selfhost/run.sh --gate --planner-required 1 --filter box_member_local_fini_blockexpr_compare_logic_unary_call_literals_nested_tail_nested_loop_branch_method_chain_tail_side_effect_tail_cleanup_min --max-cases 1 --timeout-secs 120` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [x] next task（PARSER-HANDOFF-TIER16-BACKLOG-BOOTSTRAP）:
  - Tier-16 backlog（`Box member + local/fini + cleanup + blockexpr + compare/logic + unary/call + null/array-map literal + nested if/else tail + nested loop/branch mix + method-chain tail + side-effect tail + nested-join tail`）を定義し、最小1件を PROBE→PROMOTE する

### Session Update (2026-02-08, parser handoff Tier-16 nested-join-tail promote)

- [x] Tier-16 backlog を追加し、`Box member + local/fini + blockexpr + compare/logic + unary/call + null/array-map literal + nested if/else tail + nested loop/branch mix + method-chain tail + side-effect tail + nested-join tail + cleanup` の 1件を PROMOTE
  - added: `docs/development/current/main/phases/phase-29bq/29bq-107-parser-handoff-tier16-backlog.md`
  - added: `apps/tests/phase29bq_selfhost_box_member_local_fini_blockexpr_compare_logic_unary_call_literals_nested_tail_nested_loop_branch_method_chain_tail_side_effect_tail_nested_join_tail_cleanup_min.hako`（expected=`111`）
  - updated: `tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv`
  - updated: `docs/development/current/main/phases/phase-29bq/29bq-92-parser-handoff-checklist.md`（Tier-16導線）
- [x] verify:
  - `SMOKES_SELFHOST_LIST=/tmp/selfhost_probe_tier16.tsv tools/selfhost/run.sh --gate --planner-required 1 --max-cases 1 --timeout-secs 120` PASS（expected=`111`）
  - `tools/selfhost/run.sh --gate --planner-required 1 --filter box_member_local_fini_blockexpr_compare_logic_unary_call_literals_nested_tail_nested_loop_branch_method_chain_tail_side_effect_tail_nested_join_tail_cleanup_min --max-cases 1 --timeout-secs 120` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [x] next task（PARSER-HANDOFF-TIER17-BACKLOG-BOOTSTRAP）:
  - Tier-17 backlog（`Box member + local/fini + cleanup + blockexpr + compare/logic + unary/call + null/array-map literal + nested if/else tail + nested loop/branch mix + method-chain tail + side-effect tail + nested-join tail + dual-tail sync`）を定義し、最小1件を PROBE→PROMOTE する

### Session Update (2026-02-08, parser handoff Tier-17 dual-tail-sync promote)

- [x] Tier-17 backlog を追加し、`Box member + local/fini + blockexpr + compare/logic + unary/call + null/array-map literal + nested if/else tail + nested loop/branch mix + method-chain tail + side-effect tail + nested-join tail + dual-tail sync + cleanup` の 1件を PROMOTE
  - added: `docs/development/current/main/phases/phase-29bq/29bq-108-parser-handoff-tier17-backlog.md`
  - added: `apps/tests/phase29bq_selfhost_box_member_local_fini_blockexpr_compare_logic_unary_call_literals_nested_tail_nested_loop_branch_method_chain_tail_side_effect_tail_nested_join_tail_dual_tail_sync_cleanup_min.hako`（expected=`111`）
  - updated: `tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv`
  - updated: `docs/development/current/main/phases/phase-29bq/29bq-92-parser-handoff-checklist.md`（Tier-17導線）
- [x] verify:
  - `SMOKES_SELFHOST_LIST=/tmp/selfhost_probe_tier17.tsv tools/selfhost/run.sh --gate --planner-required 1 --max-cases 1 --timeout-secs 120` PASS（expected=`111`）
  - `tools/selfhost/run.sh --gate --planner-required 1 --filter box_member_local_fini_blockexpr_compare_logic_unary_call_literals_nested_tail_nested_loop_branch_method_chain_tail_side_effect_tail_nested_join_tail_dual_tail_sync_cleanup_min --max-cases 1 --timeout-secs 120` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [x] next task（PARSER-HANDOFF-TIER18-BACKLOG-BOOTSTRAP）:
  - Tier-18 backlog（`Box member + local/fini + cleanup + blockexpr + compare/logic + unary/call + null/array-map literal + nested if/else tail + nested loop/branch mix + method-chain tail + side-effect tail + nested-join tail + dual-tail sync + guard-sync tail`）を定義し、最小1件を PROBE→PROMOTE する

### Session Update (2026-02-08, parser handoff Tier-18 guard-sync-tail promote)

- [x] Tier-18 backlog を追加し、`Box member + local/fini + blockexpr + compare/logic + unary/call + null/array-map literal + nested if/else tail + nested loop/branch mix + method-chain tail + side-effect tail + nested-join tail + dual-tail sync + guard-sync tail + cleanup` の 1件を PROMOTE
  - added: `docs/development/current/main/phases/phase-29bq/29bq-110-parser-handoff-tier18-backlog.md`
  - added: `apps/tests/phase29bq_selfhost_box_member_local_fini_blockexpr_compare_logic_unary_call_literals_nested_tail_nested_loop_branch_method_chain_tail_side_effect_tail_nested_join_tail_dual_tail_sync_guard_sync_tail_cleanup_min.hako`（expected=`111`）
  - updated: `tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv`
  - updated: `docs/development/current/main/phases/phase-29bq/29bq-92-parser-handoff-checklist.md`（Tier-18導線）
- [x] verify:
  - `SMOKES_SELFHOST_LIST=/tmp/selfhost_probe_tier18.tsv tools/selfhost/run.sh --gate --planner-required 1 --max-cases 1 --timeout-secs 120` PASS（expected=`111`）
  - `tools/selfhost/run.sh --gate --planner-required 1 --filter box_member_local_fini_blockexpr_compare_logic_unary_call_literals_nested_tail_nested_loop_branch_method_chain_tail_side_effect_tail_nested_join_tail_dual_tail_sync_guard_sync_tail_cleanup_min --max-cases 1 --timeout-secs 120` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [x] next task（MIRBUILDER-HANDLER-M0）:
  - `29bq-109` の M0（`PrintStmtHandler` 実装抽出 + 配線 + quick gate）を 1コミットで進める

### Session Update (2026-02-08, parser handoff Tier-19 mirror-sync-tail promote)

- [x] Tier-19 backlog を追加し、`Box member + local/fini + blockexpr + compare/logic + unary/call + null/array-map literal + nested if/else tail + nested loop/branch mix + method-chain tail + side-effect tail + nested-join tail + dual-tail sync + guard-sync tail + mirror-sync tail + cleanup` の 1件を PROMOTE
  - added: `docs/development/current/main/phases/phase-29bq/29bq-112-parser-handoff-tier19-backlog.md`
  - added: `apps/tests/phase29bq_selfhost_box_member_local_fini_blockexpr_compare_logic_unary_call_literals_nested_tail_nested_loop_branch_method_chain_tail_side_effect_tail_nested_join_tail_dual_tail_sync_guard_sync_tail_mirror_sync_tail_cleanup_min.hako`（expected=`111`）
  - updated: `tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv`
  - updated: `docs/development/current/main/phases/phase-29bq/29bq-92-parser-handoff-checklist.md`（Tier-19導線）
- [x] verify:
  - `SMOKES_SELFHOST_LIST=/tmp/selfhost_probe_tier19.tsv tools/selfhost/run.sh --gate --planner-required 1 --max-cases 1 --timeout-secs 120` PASS（expected=`111`）
  - `tools/selfhost/run.sh --gate --planner-required 1 --filter mirror_sync_tail_cleanup_min --max-cases 1 --timeout-secs 120` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [ ] next task（PARSER-HANDOFF-TIER20-BACKLOG-BOOTSTRAP, failure-driven hold）:
  - 新規 freeze/reject・回帰・Decision変更が出た時にのみ Tier-20 backlog（Tier-19 交差 + 1要素）を定義し、最小1件を PROBE→PROMOTE する

### Session Update (2026-02-08, parser handoff failure-driven policy pin)

- [x] `29bq-90` の Green運用を更新し、green中は新規fixtureを増やさない（failure-drivenのみ）方針へ固定
- [x] `29bq-92` に Tier-20+ の追加条件（freeze/reject・回帰・Decision変更）を明文化
- [x] `CURRENT_TASK.md` の execution policy / blocker を同方針へ同期

### Session Update (2026-02-08, recipe-first migration lane planning)

- [x] `.hako` Recipe-first 移植レーン SSOT を追加（`29bq-113`、R0->R6）
- [x] `29bq-91` に Recipe-first lane 進捗欄（2.7）を追加
- [x] `selfhost-parser-mirbuilder-migration-order-ssot.md` に Recipe-first pivot（3.6）を追加
- [x] `CURRENT_TASK.md` の pointer / active queue / blocker を `29bq-113` 導線へ同期

### Session Update (2026-02-08, recipe-first R0 implementation)

- [x] `.hako` mirbuilder R0 として Recipe core vocabulary を追加（`recipe_item_box.hako`: `Seq/If/Loop/Exit`）
- [x] `PortSig` 最小箱を追加（`recipe_port_sig_box.hako`: def/update 追跡 + merge）
- [x] `RecipeVerifier` 骨格を追加（`recipe_verifier_box.hako`: 構造検証 + PortSig 集約、lower未接続）
- [x] `mirbuilder/README` / `29bq-113` / `29bq-91` / `CURRENT_TASK` を R0 完了状態へ同期

### Session Update (2026-02-08, recipe-first R1 implementation)

- [x] stmt-4 facts extractor を追加（`recipe_facts_box.hako`、`Print/Local/Assignment/Return`）
- [x] `ProgramJsonV0PhaseStateConsumerBox` で stmt 成功経路に Facts->Recipe 生成を配線（既存 state 出力は維持、`recipe_facts`/`recipe_item` を追加）
- [x] `RecipeVerifier` の `Seq` 検証で `facts` を PortSig へ反映（R2前の整合補助）
- [x] `nyash.toml` / `mirbuilder/README` / `29bq-113` / `29bq-91` / `CURRENT_TASK` を R1 完了状態へ同期

### Session Update (2026-02-08, recipe-first R2 implementation)

- [x] `ProgramJsonV0PhaseStateConsumerBox` の stmt-4 成功経路で `RecipeVerifierBox.verify(...)` を常時実行
- [x] 検証エラー時は lowering へ進めず `err=1` + fail-fast line を返す（silent fallback 禁止）
- [x] consumer 出力へ `recipe_port_sig` / `recipe_verified_item` を追加（R3 wiring 用）
- [x] `29bq-113` / `29bq-91` / `CURRENT_TASK` を R2 完了状態へ同期

### Session Update (2026-02-08, recipe-first R3 implementation)

- [x] `ProgramJsonV0PhaseStateBox` が stmt 消費ごとの `recipe_verified_item` を収集し、最終 state に `recipe_root`（Seq）を保持
- [x] `MirJsonV0BuilderBox` が `recipe_root` を verifier 再検証し、stmt sequence から `shape_kind` を決定して MIR emission へ配線（stmt-4）
- [x] `recipe_root` が無い場合のみ legacy `shape_kind` 経路を使用（互換 fallback、既定は recipe route）
- [x] verify: `cargo check --bin hakorune` PASS
- [x] verify: `HAKO_SELFHOST_BUILDER_FIRST=1 HAKO_SELFHOST_NO_DELEGATE=1 bash tools/hakorune_emit_mir.sh apps/tests/phase29bq_selfhost_cleanup_only_min.hako /tmp/phase29bq_cleanup_only_internal.mir.json` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-08, recipe-first R4 implementation)

- [x] `IfStmtHandler` を追加し、最小受理形 `Local -> If(var==int){return int}else{} -> Return(int)` を Recipe route へ統合
- [x] `ProgramJsonV0PhaseStateConsumerBox` に `If` 分岐を追加（`IfStmtHandler` 経由で `recipe_item` を state へ接続）
- [x] `MirJsonV0BuilderBox` に `phase10_local_if_vareqint_then_return_int_fallthrough_return_int` shape を追加し、MIR JSON v0 へ配線
- [x] Program(JSON v0) 実フォーマット追従として `Expr(Call env.console.log(...))` を `PrintStmtHandler` で受理
- [x] verify: `cargo check --bin hakorune` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase0_pin_vm.sh` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase4_min_vm.sh` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase10_min_vm.sh` PASS
- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [ ] next task（R5 / M6）:
  - `LoopStmtHandler` を最小受理形から実装し、Recipe route へ統合する

### Session Update (2026-02-08, R5 preflight guard/docs)

- [x] mirbuilder 直下の hostbridge 参照禁止チェックを追加（`tools/checks/hako_mirbuilder_no_hostbridge.sh`）
- [x] `phase29bq_fast_gate_vm.sh` に hostbridge deny check を組み込み（bq gate の先頭で fail-fast）
- [x] Program JSON shape 契約 pin を追加（`tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_program_json_contract_pin_vm.sh`）
  - contract: `Print` / `Expr(Call env.console.log(...))` / `If`
- [x] R5 着手前 SSOT として最小 Loop 受理形を `29bq-113` に固定（M6-min-1）

### Session Update (2026-02-08, recipe-first R5 implementation)

- [x] `LoopStmtHandler` を追加し、R5-min-1（`Local(Int) -> Loop(Var < Int, body: x = x + Int) -> Return(Var|Int)`）を Recipe route へ統合
  - added: `lang/src/compiler/mirbuilder/stmt_handlers/loop_stmt_handler.hako`
  - updated: `lang/src/compiler/mirbuilder/program_json_v0_phase_state_consumer_box.hako`（`Loop` 分岐を handler + verifier 経由へ接続）
  - updated: `nyash.toml`（`loop_stmt_handler` の module registration）
- [x] `MirJsonV0BuilderBox` に Loop shape 判定と lower を追加（`phase11_local_loop_varltint_body_inc_return_var_or_int`）
  - updated: `lang/src/compiler/mirbuilder/mir_json_v0_builder_box.hako`
  - note: Loop lower は ValueId 再定義を避けるため `phi` ヘッダで SSA-safe に固定
- [x] fixture + pin + contract を追加/更新
  - added: `apps/tests/phase29bq_hako_mirbuilder_phase11_local_loop_return_var_min.hako`
  - added: `tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh`
  - updated: `tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_program_json_contract_pin_vm.sh`（Loop node 契約を追加）
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_program_json_contract_pin_vm.sh` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
  - `HAKO_SELFHOST_BUILDER_FIRST=1 HAKO_SELFHOST_NO_DELEGATE=1 bash tools/hakorune_emit_mir.sh lang/src/runner/stage1_cli.hako /tmp/stage1_cli_fullbuilder.mir.json` PASS
- [ ] next task（R6 / residue cleanup）:
  - `ProgramJsonV0PhaseStateBox/ConsumerBox` の legacy 直lower残骸整理 + `README/29bq-91/CURRENT_TASK` 同期を 1コミットで進める

### Session Update (2026-02-08, recipe-first post-R6 state-boundary cleanup)

- [x] `ProgramJsonV0PhaseStateBox._scan_body_rec` の引数境界を state-map 1本に統一（長い scalar arg chain を撤去）
- [x] consumer 出力 (`out`) から次状態を作る `_state_from_out` を追加し、state 更新経路を SSOT 化
- [x] `_emit_out_from_state` の i64 変換を `_state_i64` に寄せ、数値取り出し経路を一本化
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-08, recipe-first consumer state-boundary cleanup)

- [x] `ProgramJsonV0PhaseStateConsumerBox.consume_stmt` から `_consume_stmt_in_body` への長い scalar 引数チェーンを撤去し、state-map 受け渡しへ統一
- [x] `_consume_stmt_in_body` の state 構築責務を削除し、caller 側 state map をそのまま境界入力として採用
- [x] `mirbuilder/README` に consumer internal boundary（state-map 1本）ルールを追記
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-08, recipe-first consumer state-helper cleanup)

- [x] `ProgramJsonV0PhaseStateConsumerBox` に `_state_with_{print,local,assignment,return}` を追加し、stmt別 `after_state` 再構築を helper へ集約
- [x] `_handle_print_or_expr` / `_handle_local` / `_handle_assignment` / `_handle_return` の重複 state 展開を削減（挙動不変）
- [x] `mirbuilder/README` に consumer state update helper の境界ルールを追記
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-08, recipe-first consumer handler-input cleanup)

- [x] `ProgramJsonV0PhaseStateConsumerBox` に `_{print,local,assignment,return}_handler_state` を追加し、handler入力 map の構築を helper 集約
- [x] `_handle_print_or_expr` / `_handle_local` / `_handle_assignment` / `_handle_return` から入力 map の重複を削減（挙動不変）
- [x] `mirbuilder/README` に handler input helper の境界ルールを追記
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-08, recipe-first consumer flow-helper cleanup)

- [x] `ProgramJsonV0PhaseStateConsumerBox` に `_emit_handler_error_or_null` / `_emit_stmt_recipe` を追加し、stmt handler 共通フロー（err判定 + recipe emit）を helper 化
- [x] `_handle_print_or_expr` / `_handle_local` / `_handle_assignment` / `_handle_return` の重複フローを helper 呼び出しへ置換（挙動不変）
- [x] `mirbuilder/README` に consumer flow helper の境界ルールを追記
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-08, recipe-first control-flow helper cleanup)

- [x] `ProgramJsonV0PhaseStateConsumerBox` に `_result_next_idx_or_fallback` / `_emit_handler_error_or_null_with_result_next` を追加し、control handler の err + next_idx 解決フローを helper 化
- [x] `_emit_control_recipe_or_error` の重複 err処理を helper 呼び出しへ置換（`If/Loop` 経路、挙動不変）
- [x] `mirbuilder/README` に control 向け flow helper（next_idx 解決）を追記
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-08, recipe-first stmt-dispatch cleanup)

- [x] `ProgramJsonV0PhaseStateConsumerBox` に `_dispatch_stmt_or_null` を追加し、`node_type` 分岐を helper へ集約
- [x] `_consume_stmt_in_body` は dispatch helper 呼び出し + unsupported fail-fast のみを担当する構造へ簡素化（挙動不変）
- [x] `mirbuilder/README` に stmt dispatch helper 境界を追記
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-08, recipe-first phase-state fail-fast + recipe-shape dispatch cleanup)

- [x] `ProgramJsonV0PhaseStateBox` に `_emit_scan_error` / `_emit_consumer_error_or_null` を追加し、scan 再帰内 fail-fast 出力の組み立てを helper 集約
- [x] `_scan_body_rec` の `cap_missing/order` / `program_json_v0` / consumer error 経路を helper 呼び出しへ統一（挙動不変）
- [x] `MirJsonV0BuilderBox` に `_recipe_shape_try_control` / `_recipe_shape_result_{err,match}` を追加し、If/Loop/Seq shape 判定の合流点を helper 化
- [x] `mirbuilder/README` に phase-state fail-fast helper と recipe-shape dispatch helper の境界を追記
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-08, recipe-first control emit helper cleanup)

- [x] `ProgramJsonV0PhaseStateBox` に `_emit_order_violation` を追加し、order違反メッセージ組み立てを helper 集約
- [x] `MirJsonV0BuilderBox` に `_emit_shape_phase10_if_or_null` / `_emit_shape_phase11_loop_or_null` を追加し、control shape MIR 生成を `_build_mir_by_shape` から分離
- [x] `MirJsonV0BuilderBox` に `_ctx_i64` / `_ctx_s` を追加し、control emit helper の context 参照を共通化
- [x] `mirbuilder/README` に phase-state order fail-fast helper と control emit helper の境界を追記
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-08, recipe-first non-control emit helper cleanup)

- [x] `MirJsonV0BuilderBox` に `_emit_shape_non_control_basic_or_null` / `_emit_shape_non_control_update_or_null` を追加し、phase1/2/4/5/7/9 の shape emit を helper 分離（挙動不変）
- [x] `_build_mir_by_shape` は helper dispatch + unsupported fail-fast のみへ縮退
- [x] `mirbuilder/README` に non-control emit helper の境界を追記
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-08, recipe-first shape context helper cleanup)

- [x] `MirJsonV0BuilderBox` に `_shape_ctx_from_state` を追加し、`build()` の state 読み出し + shape context map 構築を helper 集約
- [x] `build()` は recipe shape override 判定 + helper 呼び出し + `_build_mir_by_shape` 委譲のみへ縮退
- [x] `mirbuilder/README` に shape context helper 境界を追記
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-08, recipe-first shape input helper cleanup)

- [x] `MirJsonV0BuilderBox` に `_shape_inputs_with_recipe` を追加し、`build()` の recipe override 展開（`shape_kind`/flags/recipe payload）を helper 集約
- [x] `build()` は input helper → return 必須チェック → context helper → emit の順へ縮退
- [x] `mirbuilder/README` に shape input helper 境界を追記
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-08, recipe-first return guard helper cleanup)

- [x] `MirJsonV0BuilderBox` に `_ensure_return_present_or_null` を追加し、`Return` 必須契約 fail-fast を helper 集約
- [x] `build()` 本体から `has_return != 1` の直接判定を除去し、return guard helper 呼び出しへ統一
- [x] `mirbuilder/README` に return guard helper 境界を追記
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-08, recipe-first shape dispatch helper cleanup)

- [x] `MirJsonV0BuilderBox` に `_shape_dispatch_from_inputs` を追加し、`shape_kind` 抽出 + `shape_ctx` 組み立てを helper 集約
- [x] `build()` 本体から shape flag / recipe payload 展開の直接処理を除去し、dispatch helper 呼び出しへ統一
- [x] `mirbuilder/README` に shape dispatch helper 境界を追記
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-08, recipe-first shape input error helper cleanup)

- [x] `MirJsonV0BuilderBox` に `_shape_inputs_error_or_null` を追加し、`shape_inputs.err` 判定 + エラー返却マップ生成を helper 集約
- [x] `build()` 本体から `shape_inputs.err` の直接判定と返却マップ構築を除去し、error helper 呼び出しへ統一
- [x] `mirbuilder/README` に shape input error helper 境界を追記
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-08, recipe-first build handoff helper cleanup)

- [x] `MirJsonV0BuilderBox` に `_build_from_shape_inputs` を追加し、return guard + dispatch + final build を helper 集約
- [x] `build()` 本体から return guard / shape dispatch / final build の直接処理を除去し、handoff helper 呼び出しへ統一
- [x] `mirbuilder/README` に build handoff helper 境界を追記
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-09, consumer handler error flow helper cleanup)

- [x] `ProgramJsonV0PhaseStateConsumerBox` に `_emit_handler_error_or_null_at` を追加し、handler error 出力のSSOTを1箇所へ集約
- [x] `_emit_handler_error_or_null` / `_emit_handler_error_or_null_with_result_next` から重複していた `err_line` + `emit_out` 組み立てを除去
- [x] `mirbuilder/README` の Consumer flow helpers 記述を新SSOT helper に同期
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-09, consumer stmt dispatch split cleanup)

- [x] `ProgramJsonV0PhaseStateConsumerBox` の stmt 分岐を `_dispatch_non_control_stmt_or_null` / `_dispatch_control_stmt_or_null` に分離し、`_dispatch_stmt_or_null` を入口集約へ整理
- [x] `mirbuilder/README` の dispatch helper 記述を non-control/control 分離構造へ同期
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-09, consumer fail-fast helper cleanup)

- [x] `ProgramJsonV0PhaseStateConsumerBox` に `_emit_missing_stmt_node_type` / `_emit_unsupported_stmt` を追加し、`consume_stmt` 系 fail-fast の文言組み立てを helper 集約
- [x] `_consume_stmt_in_body` から fail-fast 文字列直書きを除去し、helper 呼び出しへ統一
- [x] `mirbuilder/README` の consumer fail-fast helper 記述を追記
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-09, consumer non-control flow pipeline cleanup)

- [x] `ProgramJsonV0PhaseStateConsumerBox` に `_after_state_from_non_control_result_or_null` / `_emit_non_control_stmt_or_handler_error` を追加し、`Print/Local/Assignment/Return` の result→after_state→recipe emit を共通化
- [x] non-control 4ハンドラ（`_handle_print_or_expr` / `_handle_local` / `_handle_assignment` / `_handle_return`）から重複していた field decode + state rebuild + recipe emit を除去
- [x] `mirbuilder/README` の Consumer flow helpers 記述を non-control pipeline helper へ同期
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-09, consumer missing-state fail-fast helper cleanup)

- [x] `ProgramJsonV0PhaseStateConsumerBox` に `_empty_state_map` / `_emit_missing_state_map` を追加し、`consume_stmt` 入口の missing state map fail-fast を helper 化
- [x] `consume_stmt` 本体から inline の empty-state 生成と fail-fast 出力を除去し、helper 呼び出しへ統一
- [x] `mirbuilder/README` の Consumer fail-fast helpers 記述を missing-state helper まで含めて同期
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-09, consumer control recipe helper cleanup)

- [x] `ProgramJsonV0PhaseStateConsumerBox` に `_control_recipe_item_or_null` / `_emit_missing_control_recipe_item` / `_control_recipe_out` を追加し、control recipe 境界を helper 集約
- [x] `_emit_control_recipe_or_error` から `recipe_item` 検証・missing recipe fail-fast・`recipe_out` 生成の直書きを除去
- [x] `mirbuilder/README` の Consumer flow helpers 記述を control recipe helper 反映版へ同期
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-09, consumer control error flow dedup cleanup)

- [x] `_emit_control_recipe_or_error` で `out_next_idx` を1回だけ計算し、error判定を `_emit_handler_error_or_null_at(result, out_next_idx, ...)` へ統一
- [x] 未使用化した `_emit_handler_error_or_null_with_result_next` を削除し、control error flow の重複経路を解消
- [x] `mirbuilder/README` の Consumer flow helpers 記述を新しい control error flow（`_result_next_idx_or_fallback` + `*_at`）へ同期
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-09, consumer control entry helper cleanup)

- [x] `ProgramJsonV0PhaseStateConsumerBox` に `_handle_control_stmt_or_null` を追加し、`If/Loop` の control state 構築 + handler 実行 + emit 経路を1箇所へ集約
- [x] `_handle_if` / `_handle_loop` を削除し、`_dispatch_control_stmt_or_null` は control entry helper 呼び出しのみへ薄化
- [x] `mirbuilder/README` の dispatch helper 記述を control entry helper 反映版へ同期
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-09, consumer control dispatch inline cleanup)

- [x] 純ラッパだった `_dispatch_control_stmt_or_null` を削除し、`_dispatch_stmt_or_null` から `_handle_control_stmt_or_null` へ直接委譲
- [x] dispatch層の1段ラッパを除去して control 側導線を短縮
- [x] `mirbuilder/README` の dispatch helper 記述を inline 後の構造へ同期
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-09, consumer control handler call helper cleanup)

- [x] `ProgramJsonV0PhaseStateConsumerBox` に `_control_handler_out_or_null` を追加し、`If/Loop` の handler 呼び出しを helper 集約
- [x] `_handle_control_stmt_or_null` から `IfStmtHandler` / `LoopStmtHandler` の分岐直書きを除去し、handler out（label + result）経由へ統一
- [x] `mirbuilder/README` の Consumer flow helpers 記述を control handler helper 反映版へ同期
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-09, consumer non-control handler call helper cleanup)

- [x] `ProgramJsonV0PhaseStateConsumerBox` に `_non_control_handler_out_or_null` / `_handle_non_control_stmt_or_null` を追加し、`Print/Expr/Local/Assignment/Return` の handler 呼び出しを helper 集約
- [x] `_handle_print_or_expr` / `_handle_local` / `_handle_assignment` / `_handle_return` を削除し、non-control 実行を handler out（stmt_name + result）経由へ統一
- [x] `mirbuilder/README` の dispatch/flow helper 記述を non-control handler helper 反映版へ同期
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-09, consumer non-control dispatch inline cleanup)

- [x] 純ラッパだった `_dispatch_non_control_stmt_or_null` を削除し、`_dispatch_stmt_or_null` から `_handle_non_control_stmt_or_null` へ直接委譲
- [x] dispatch層の non-control 側1段ラッパを除去
- [x] `mirbuilder/README` の dispatch helper 記述を inline 後の構造へ同期
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-09, consumer handler error wrapper removal cleanup)

- [x] 薄い委譲のみだった `_emit_handler_error_or_null` を削除し、non-control 側の error判定を `_emit_handler_error_or_null_at` へ直接統一
- [x] `mirbuilder/README` の Consumer flow helpers 記述を error SSOT直結版へ同期
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-09, consume node-read/dispatch helper cleanup)

- [x] `ProgramJsonV0PhaseStateConsumerBox` に `_read_stmt_node_info_or_error` / `_dispatch_or_unsupported` を追加し、`_consume_stmt_in_body` の node読取 + fallback 制御を helper 分離
- [x] `_consume_stmt_in_body` 本体から node decode / unsupported 分岐の直書きを除去し、helper 呼び出しへ統一
- [x] `mirbuilder/README` の Consumer flow helpers 記述を consume helper 反映版へ同期
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-09, consume node-info constructor helper cleanup)

- [x] `ProgramJsonV0PhaseStateConsumerBox` に `_stmt_node_info_ok` / `_stmt_node_info_err` を追加し、stmt node-info map 構築を helper 集約
- [x] `_read_stmt_node_info_or_error` から `%{...}` 直書きを除去し、ok/error の constructor helper 呼び出しへ統一
- [x] `mirbuilder/README` の Consumer flow helpers 記述を node-info constructor helper 反映版へ同期
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-09, state/result map conversion helper cleanup)

- [x] `ProgramJsonV0PhaseStateConsumerBox` に `_map_i64` / `_map_s` を追加し、state/result/node map の型付き読み出し（`to_i64` / 文字列化）を helper 集約
- [x] `_state_with_{print,local,assignment,return}` / `_after_state_from_non_control_result_or_null` / `consume_stmt` 周辺の map 読み出し直書きを helper 呼び出しへ置換
- [x] `mirbuilder/README` に map conversion helper のSSOT記述を追加
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-09, state base snapshot helper cleanup)

- [x] `ProgramJsonV0PhaseStateConsumerBox` に `_state_base` を追加し、current_state から state-map 再構築に必要な既定値読取を1箇所へ集約
- [x] `_state_with_{print,local,assignment,return}` から current_state 直読みを除去し、`_state_base` 経由の override 適用へ統一
- [x] `mirbuilder/README` の state update helper 記述を `_state_base` 反映版へ同期
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-09, non-control handler state helper cleanup)

- [x] `ProgramJsonV0PhaseStateConsumerBox` の `_{print,local,assignment,return}_handler_state` を削除し、`_non_control_handler_state_or_null` に統合
- [x] `_non_control_handler_out_or_null` の handler入力 state 構築を `_state_base` 再利用の shared helper 呼び出しへ統一
- [x] `mirbuilder/README` の handler input helper 記述を統合後の構造へ同期
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-09, control state seed base alignment cleanup)

- [x] `_control_state` を `_state_base` 参照へ変更し、control経路の state seed を current_state 直読みから分離
- [x] `mirbuilder/README` に control state helper の `_state_base` 統一方針を追記
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-09, handler out map helper cleanup)

- [x] `ProgramJsonV0PhaseStateConsumerBox` に `_stmt_handler_out` / `_control_handler_out` を追加し、handler結果 map（name + result）の構築を helper 集約
- [x] `_non_control_handler_out_or_null` / `_control_handler_out_or_null` の map literal 直書きを helper 呼び出しへ置換
- [x] `mirbuilder/README` に handler out helper のSSOT記述を追加
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-09, state overlay update cleanup)

- [x] `ProgramJsonV0PhaseStateConsumerBox._state_with_{print,local,assignment,return}` を `_state_base` + `set(...)` 方式へ変更し、`_state_map` の全項目再構築を削除
- [x] 非変更フィールドの copy-through を `state_map` 引数列挙ではなく base snapshot に委譲し、state update path を差分更新に限定
- [x] `mirbuilder/README` の state update helper 記述を overlay 方針へ同期
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-09, emit out state-path cleanup)

- [x] `_shape_kind_from_state` / `_emit_out_with_state` を追加し、shape判定 + out map 組み立てを state map 経路でSSOT化
- [x] `_emit_out` を `_state_map` 生成 + `_emit_out_with_state` 呼び出しへ薄化し、`_emit_out_from_state` は helper 直結へ単純化
- [x] `mirbuilder/README` の Consumer emit-out helper 記述を同期
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-09, phase-state emit out bridge cleanup)

- [x] `ProgramJsonV0PhaseStateBox` に `_emit_out_with_state` を追加し、phase-box → consumer emit の state map bridge をSSOT化
- [x] `ProgramJsonV0PhaseStateBox._emit_out_from_state` を state helper 直結へ変更し、state map の scalar 展開を除去
- [x] `ProgramJsonV0PhaseStateBox._emit_out` も `_state_map` 生成 + `_emit_out_with_state` 委譲へ薄化
- [x] `mirbuilder/README` に phase-state emit-out helper 方針を追記
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-09, phase-state out map read helper cleanup)

- [x] `ProgramJsonV0PhaseStateBox` に `_out_i64` / `_out_s` を追加し、consumer out map の typed read をSSOT化
- [x] `_state_from_out` の文字列化（`print_kind` / `print_var_name` / `local_name` / `assign_name` / `return_kind` / `ret_var_name`）を `_out_s` 経由へ統一
- [x] `_emit_consumer_error_or_null` と body scan の `err` / `next_idx` 読み出しを `_out_i64` 経由へ統一
- [x] `mirbuilder/README` に phase-state out map read helper 方針を追記
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-09, phase-state out map box helper cleanup)

- [x] `ProgramJsonV0PhaseStateBox` に `_out_box` を追加し、consumer out map の Box値読み出しを helper 化
- [x] body scan の `recipe_verified_item` 取得を `_out_box` 経由へ変更
- [x] `_state_from_out` の `printv` / `local_init` / `assign_rhs` / `retv` を `_out_box` 経由へ統一
- [x] `mirbuilder/README` の out-map read helper 記述を `_out_box` 反映版へ同期
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-09, phase-state scan error helper cleanup)

- [x] `ProgramJsonV0PhaseStateBox` に `_emit_error_from_state` を追加し、scan error / consumer error の最終 emit 経路を共通化
- [x] `_emit_body_eof_error` を追加し、`_scan_body_rec` の Program.body EOF エラー重複を helper 1箇所へ統一
- [x] `_consumer_error_line_or_default` を追加し、consumer error の既定文言生成を `_emit_consumer_error_or_null` から分離
- [x] `mirbuilder/README` の phase-state fail-fast helper 記述を新 helper 構成へ同期
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-09, phase-state scan done/missing-recipe helper cleanup)

- [x] `ProgramJsonV0PhaseStateBox` に `_emit_scan_done` を追加し、`recipe_root` 付き正常終了出力（`_scan_body_rec` の `ch == "]"` 経路）を helper 化
- [x] `ProgramJsonV0PhaseStateBox` に `_emit_missing_recipe_item_error` を追加し、`recipe_verified_item` 欠落時の fail-fast 文言/出力経路を helper 化
- [x] `_scan_body_rec` 本体から inline out-map 組み立て・長文 error literal を除去し、分岐を helper 呼び出し中心に薄化
- [x] `mirbuilder/README` の phase-state scan helper 記述を新 helper 構成へ同期
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-09, phase-state cap-missing scan helper cleanup)

- [x] `ProgramJsonV0PhaseStateBox` に `_emit_program_cap_missing_error` を追加し、`[cap_missing/program_json_v0]` 前置き付き scan error を helper 化
- [x] `_scan_body_rec` の `{` 不一致・node type 読み取り失敗を `_emit_expected_stmt_object_error` / `_emit_read_stmt_node_type_error` 経由へ統一
- [x] `mirbuilder/README` の phase-state scan helper 記述を cap-missing helper 追加版へ同期
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-09, phase-state order violation helper cleanup)

- [x] `ProgramJsonV0PhaseStateBox` に `_emit_order_cap_missing_error` / `_order_violation_msg` を追加し、order違反文言と `[cap_missing/order]` 前置き生成を helper 分離
- [x] `_emit_order_violation` の inline 長文 literal を helper 呼び出しへ置換し、fail-fast 分岐本体の責務を縮小
- [x] `mirbuilder/README` の phase-state scan helper 記述を order helper 追加版へ同期
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-09, phase-state recipe cap-missing helper cleanup)

- [x] `ProgramJsonV0PhaseStateBox` に `_emit_recipe_cap_missing_error` を追加し、`[cap_missing/recipe_verifier]` 前置き付き scan error の生成を helper 化
- [x] `_emit_missing_recipe_item_error` の inline prefix 組み立てを削除し、recipe cap-missing helper 呼び出しへ統一
- [x] `mirbuilder/README` の phase-state scan helper 記述を recipe cap-missing helper 追加版へ同期
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-09, phase-state consumer error helper cleanup)

- [x] `ProgramJsonV0PhaseStateBox` に `_has_consumer_error` / `_consumer_next_idx` / `_emit_consumer_error` / `_consumer_internal_error_line` を追加し、consumer error path を helper 分離
- [x] `_emit_consumer_error_or_null` の inline 判定/出力組み立てを helper 呼び出しへ置換して分岐本体を薄化
- [x] `mirbuilder/README` の phase-state scan helper 記述を consumer error helper 追加版へ同期
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-09, phase-state order stage update helper cleanup)

- [x] `_scan_body_rec` の order stage 更新分岐（`peek_stage > order_stage` 比較）を `_next_order_stage` helper 呼び出しへ置換
- [x] `order_last` 更新を分岐外（`peek_stage != 0` ブロック内）へ統一し、order update の分岐本体を薄化
- [x] `mirbuilder/README` の phase-state scan helper 記述を `_next_order_stage` 追加版へ同期
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-09, phase-state consumer post-processing helper cleanup)

- [x] `_scan_body_rec` の consumer後処理から `err` 直判定を除去し、`_scan_consumer_error_or_null` 経由へ統一
- [x] `recipe_verified_item` 取得＋map検証を `_verified_recipe_item_or_null` へ抽出し、missing-recipe fail-fast 分岐を薄化
- [x] `next_idx` 読み出しを `_consumer_next_idx` 経由へ統一し、consumer結果取り扱いの読み出し導線を集約
- [x] `mirbuilder/README` の phase-state scan helper 記述を post-processing helper 追加版へ同期
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-09, phase-state consume/out-state helper cleanup)

- [x] `_scan_body_rec` の `consume_stmt` 呼び出しを `_consume_stmt_out` helper 経由へ変更し、consumer入口を1箇所化
- [x] `_scan_body_rec` の `next_state` 復元を `_next_state_from_consumer_out` helper 経由へ変更し、state 復元導線を明示化
- [x] `mirbuilder/README` の phase-state scan helper 記述を consume/out-state helper 追加版へ同期
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-09, phase-state recipe-push/recurse helper cleanup)

- [x] `_scan_body_rec` の `recipe_items.push` を `_append_recipe_item` helper 呼び出しへ置換し、成功パスの副作用処理を分離
- [x] `_scan_body_rec` の再帰呼び出しを `_scan_next_stmt_rec` helper 経由へ統一し、再帰遷移の引数束ねを明示化
- [x] `mirbuilder/README` の phase-state scan helper 記述を recipe-push/recurse helper 追加版へ同期
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-09, phase-state scan prelude helper cleanup)

- [x] `_scan_body_rec` 冒頭の `skip_ws` + EOF判定 + `read_char` + EOF判定を `_scan_idx_or_eof_out` / `_scan_ch_or_eof_out` へ分離し、戻り値は `scan_out` 契約（`err/out/idx/ch`）で統一
- [x] 前段の低レベル処理を `_scan_out*` / `_skip_ws_idx` / `_body_eof_error_or_null` / `_read_body_char` / `_body_char_eof_error_or_null` に抽出し、scan本体の前処理責務を薄化
- [x] `mirbuilder/README` の phase-state scan helper 記述を scan prelude helper（scan_out 契約）追加版へ同期
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-09, phase-state comma/end transition helper cleanup)

- [x] `_scan_body_rec` の `ch == ","` / `ch == "]"` 分岐を `_is_body_comma` / `_is_body_end` 判定へ置換し、本文条件分岐を薄化
- [x] カンマ遷移を `_scan_after_comma_rec`、終端遷移を `_scan_done_at_idx` へ抽出し、遷移処理の責務を helper 化
- [x] `mirbuilder/README` の phase-state scan helper 記述を comma/end transition helper 追加版へ同期
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-09, phase-state node/order helper cleanup)

- [x] `_scan_body_rec` の node type peek + order判定ブロックを `_scan_order_update_out` へ集約し、本文は結果適用（`order_stage`/`order_last`）のみに縮小
- [x] order判定の入出力を `_scan_order_out` 契約（`err/out/order_stage/order_last`）へ統一し、`_scan_order_*` accessor 群で読み取りを一本化
- [x] `peek_type`/`peek_stage` 抽出を `_peek_type_from_node` / `_peek_stage_from_type` に分離し、node/order の観測責務を局所化
- [x] `mirbuilder/README` の phase-state scan helper 記述を node/order helper 追加版へ同期
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-09, phase-state consumer-error alias cleanup)

- [x] `ProgramJsonV0PhaseStateBox` の未使用 alias helper `_emit_consumer_error_or_null` を削除し、consumer error 経路を `_scan_consumer_error_or_null` 1本へ統一
- [x] `mirbuilder/README` の phase-state out-map / scan helper 記述から `_emit_consumer_error_or_null` を除去し、実装と記述を一致
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-09, consumer emit-out dead helper cleanup)

- [x] `ProgramJsonV0PhaseStateConsumerBox` の未参照 helper `_emit_out`（state map 再構築のみを行う旧入口）を削除
- [x] consumer emit-out 経路を `_emit_out_with_state` / `_emit_out_from_state` のみへ固定し、入口責務を縮小
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-09, consumer dispatch wrapper inline cleanup)

- [x] `ProgramJsonV0PhaseStateConsumerBox` の `_dispatch_stmt_or_null` を削除し、`_dispatch_or_unsupported` に non-control/control dispatch を直列統合
- [x] unsupported fail-fast 出口（`_emit_unsupported_stmt`）は維持し、挙動は wrapper 1段削除のみへ限定
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-09, consumer entry inline cleanup)

- [x] `ProgramJsonV0PhaseStateConsumerBox.consume_stmt` に node-read + dispatch 本体を統合し、`_consume_stmt_in_body` を削除
- [x] `mirbuilder/README` の consumer boundary 記述を実装に同期（dispatch は `_dispatch_or_unsupported` 直列構成をSSOT化）
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-09, mir-builder shape dispatch residue cleanup)

- [x] `MirJsonV0BuilderBox._build_from_shape_inputs` へ `shape_kind` 抽出 + `shape_ctx` 構築を統合し、単段中継 `_shape_dispatch_from_inputs` を削除
- [x] `mirbuilder/README` の builder helper 記述を更新し、`_build_from_shape_inputs` が直接 `shape_ctx` を組み立てる構造へ同期
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-09, phase-state recursion handoff inline cleanup)

- [x] `ProgramJsonV0PhaseStateBox._scan_body_rec` の終端経路を `_emit_scan_done` へ直結し、単段中継 `_scan_done_at_idx` を削除
- [x] 再帰遷移を `_scan_body_rec` へ直結し、単段中継 `_scan_next_stmt_rec` を削除（`_scan_after_comma_rec` も同経路へ統一）
- [x] `mirbuilder/README` の phase-state scan helper 記述を実装へ同期（遷移境界は `_scan_after_comma_rec` + `_scan_body_rec` に固定）
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-09, phase-state comma recursion inline cleanup)

- [x] `ProgramJsonV0PhaseStateBox._scan_body_rec` の comma 遷移を `_scan_body_rec(idx + 1, ...)` へ直結し、単段中継 `_scan_after_comma_rec` を削除
- [x] `mirbuilder/README` の phase-state scan helper 記述を実装へ同期（comma 遷移の中継 helper 名を削除）
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-09, phase-state next-state handoff inline cleanup)

- [x] `ProgramJsonV0PhaseStateBox._scan_body_rec` の `next_state` 復元を `_state_from_out(out)` へ直結し、単段中継 `_next_state_from_consumer_out` を削除
- [x] `mirbuilder/README` の phase-state scan helper 記述を実装へ同期（next-state 中継 helper 名を削除）
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-09, phase-state consume/append wrapper inline cleanup)

- [x] `ProgramJsonV0PhaseStateBox._scan_body_rec` の stmt 消費を `ProgramJsonV0PhaseStateConsumerBox.consume_stmt(...)` へ直結し、単段中継 `_consume_stmt_out` を削除
- [x] `recipe_items.push(...)` を本体へ直結し、単段中継 `_append_recipe_item` を削除
- [x] `mirbuilder/README` の phase-state scan helper 記述を実装へ同期（consume/append 中継 helper 名を削除）
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-09, phase-state order accessor inline cleanup)

- [x] `ProgramJsonV0PhaseStateBox._scan_body_rec` の `order_out` 読み取りを `map_get` 直結に統合し、単段中継 `_scan_order_err` / `_scan_order_error` / `_scan_order_stage` / `_scan_order_last` を削除
- [x] `mirbuilder/README` の phase-state scan helper 記述を実装へ同期（order accessor helper 名を削除）
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-09, phase-state scan-out accessor inline cleanup)

- [x] `ProgramJsonV0PhaseStateBox._scan_body_rec` の `scan_out` 読み取りを `map_get` 直結に統合し、単段中継 `_scan_out_err` / `_scan_out_error` / `_scan_out_idx` / `_scan_out_ch` を削除
- [x] `mirbuilder/README` の phase-state scan helper 記述を実装へ同期（`scan_out` は `_scan_body_rec` で直読みに統一）
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-09, phase-state scan prelude inline cleanup)

- [x] `ProgramJsonV0PhaseStateBox._scan_body_rec` 冒頭へ `skip_ws` + EOF判定 + `read_char` + EOF判定を直結し、`_scan_out` / `_scan_idx_or_eof_out` / `_scan_ch_or_eof_out` / `_skip_ws_idx` / `_body_eof_error_or_null` / `_read_body_char` / `_body_char_eof_error_or_null` を削除
- [x] `mirbuilder/README` の phase-state scan helper 記述を実装へ同期（scan prelude helper 群の撤去を明記）
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-09, phase-state order check inline cleanup)

- [x] `ProgramJsonV0PhaseStateBox._scan_body_rec` に node type peek + order stage 判定を直結し、`_scan_order_out` / `_scan_order_update_out` / `_peek_type_from_node` / `_peek_stage_from_type` を削除
- [x] `mirbuilder/README` の phase-state scan helper 記述を実装へ同期（order 判定の直結化を明記）
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-09, phase-state consumer out inline cleanup)

- [x] `ProgramJsonV0PhaseStateBox._scan_body_rec` で `next_idx` / `err` 判定を out map 直読みに統一し、`_has_consumer_error` / `_consumer_next_idx` / `_emit_consumer_error` / `_scan_consumer_error_or_null` を削除
- [x] `mirbuilder/README` の phase-state scan helper 記述を実装へ同期（consumer out 判定の直結化を明記）
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-09, phase-state emit wrapper inline cleanup)

- [x] `ProgramJsonV0PhaseStateBox._emit_scan_done` / `_emit_error_from_state` を `_emit_out_with_state` 直結へ統一し、単段中継 `_emit_out_from_state` を削除
- [x] `mirbuilder/README` の phase-state emit-out helper 記述を実装へ同期（`_emit_out_from_state` を除去）
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-09, phase-state error wrapper inline cleanup)

- [x] `ProgramJsonV0PhaseStateBox._scan_body_rec` / `_emit_scan_error` を `_emit_out_with_state` 直結へ統一し、単段中継 `_emit_error_from_state` を削除
- [x] `mirbuilder/README` の phase-state scan helper 記述を実装へ同期（`_emit_error_from_state` を除去）
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-09, phase-state body-delimiter inline cleanup)

- [x] `ProgramJsonV0PhaseStateBox._scan_body_rec` の `,` / `]` 判定を本体へ直結し、単段中継 `_is_body_comma` / `_is_body_end` を削除
- [x] `mirbuilder/README` の phase-state scan helper 記述を実装へ同期（delimiter helper 名を除去）
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-09, phase-state stmt-type error wrapper inline cleanup)

- [x] `ProgramJsonV0PhaseStateBox._scan_body_rec` の stmt object / node type エラーを `_emit_program_cap_missing_error` 直結へ統一し、単段中継 `_emit_expected_stmt_object_error` / `_emit_read_stmt_node_type_error` を削除
- [x] `mirbuilder/README` の phase-state scan helper 記述を実装へ同期（stmt-type error helper 名を除去）
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-09, phase-state order-violation wrapper inline cleanup)

- [x] `ProgramJsonV0PhaseStateBox._scan_body_rec` の order violation 分岐を `_emit_order_cap_missing_error` 直結へ統一し、単段中継 `_emit_order_violation` を削除
- [x] `mirbuilder/README` の phase-state scan helper 記述を実装へ同期（`_emit_order_violation` を除去）
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-09, phase-state missing-recipe wrapper inline cleanup)

- [x] `ProgramJsonV0PhaseStateBox._scan_body_rec` の missing recipe item 分岐を `_emit_recipe_cap_missing_error` 直結へ統一し、単段中継 `_emit_missing_recipe_item_error` を削除
- [x] `mirbuilder/README` の phase-state scan helper 記述を実装へ同期（`_emit_missing_recipe_item_error` を除去）
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-09, phase-state body-eof wrapper inline cleanup)

- [x] `ProgramJsonV0PhaseStateBox._scan_body_rec` の body EOF 分岐を `_emit_scan_error` 直結へ統一し、単段中継 `_emit_body_eof_error` を削除
- [x] `mirbuilder/README` の phase-state scan helper 記述を実装へ同期（`_emit_body_eof_error` を除去）
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` PASS
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS

### Session Update (2026-02-09, phase-state order-cap wrapper inline cleanup)

- [x] `ProgramJsonV0PhaseStateBox._scan_body_rec` の order violation 分岐を `_emit_scan_error` 直結へ統一し、単段中継 `_emit_order_cap_missing_error` を削除
- [x] `mirbuilder/README` の phase-state scan helper 記述を実装へ同期（`_emit_order_cap_missing_error` を除去）
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` → `[PASS] hako_mirbuilder phase11 pin: PASS`
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` → `[PASS] phase29bq_fast_gate_vm: PASS (mode=bq)`

### Session Update (2026-02-09, phase-state cap/error wrapper inline cleanup)

- [x] `ProgramJsonV0PhaseStateBox._scan_body_rec` の cap-missing 分岐を `_emit_scan_error` 直結へ統一し、単段中継 `_emit_program_cap_missing_error` / `_emit_recipe_cap_missing_error` を削除
- [x] `ProgramJsonV0PhaseStateBox._consumer_error_line_or_default` に内部エラー文言を直結し、単段中継 `_consumer_internal_error_line` を削除
- [x] `mirbuilder/README` の phase-state scan helper 記述を実装へ同期（削除 helper 名を除去）
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` → `[PASS] hako_mirbuilder phase11 pin: PASS`
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` → `[PASS] phase29bq_fast_gate_vm: PASS (mode=bq)`

### Session Update (2026-02-09, phase-state order helper inline cleanup)

- [x] `ProgramJsonV0PhaseStateBox._scan_body_rec` の order-stage 更新を inline 化し、単段 helper `_next_order_stage` を削除
- [x] order violation 文言の組み立てを `_scan_body_rec` に直結し、単段 helper `_order_violation_msg` を削除
- [x] `mirbuilder/README` の phase-state scan helper 記述を実装へ同期（order helper 名を除去）
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` → `[PASS] hako_mirbuilder phase11 pin: PASS`
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` → `[PASS] phase29bq_fast_gate_vm: PASS (mode=bq)`

### Session Update (2026-02-09, phase-state emit-out path cleanup)

- [x] `ProgramJsonV0PhaseStateBox._fail` を `_emit_out_with_state + _state_map` 直結に変更し、scalar fanout helper `_emit_out` を削除
- [x] `mirbuilder/README` の phase-state emit-out 記述を実装へ同期（`_emit_out` 削除済みを明記）
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` → `[PASS] hako_mirbuilder phase11 pin: PASS`
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` → `[PASS] phase29bq_fast_gate_vm: PASS (mode=bq)`

### Session Update (2026-02-09, consumer control recipe helper inline cleanup)

- [x] `ProgramJsonV0PhaseStateConsumerBox._emit_control_recipe_or_error` に next_idx fallback 判定を inline し、単段 helper `_result_next_idx_or_fallback` を削除
- [x] `ProgramJsonV0PhaseStateConsumerBox._emit_control_recipe_or_error` で recipe_out map を inline し、単段 helper `_control_recipe_out` を削除
- [x] `mirbuilder/README` の consumer flow helper 記述を実装へ同期（削除 helper 名を除去）
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` → `[PASS] hako_mirbuilder phase11 pin: PASS`
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` → `[PASS] phase29bq_fast_gate_vm: PASS (mode=bq)`

### Session Update (2026-02-09, consumer handler-out wrapper inline cleanup)

- [x] `ProgramJsonV0PhaseStateConsumerBox._non_control_handler_out_or_null` / `_control_handler_out_or_null` で handler result map を inline 化し、単段 helper `_stmt_handler_out` / `_control_handler_out` を削除
- [x] `mirbuilder/README` の consumer flow helper 記述を実装へ同期（削除 helper 名を除去）
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` → `[PASS] hako_mirbuilder phase11 pin: PASS`
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` → `[PASS] phase29bq_fast_gate_vm: PASS (mode=bq)`

### Session Update (2026-02-09, consumer recipe-item wrapper inline cleanup)

- [x] `ProgramJsonV0PhaseStateConsumerBox._emit_control_recipe_or_error` に recipe_item 検証と cap-missing recipe error 出力を inline し、単段 helper `_control_recipe_item_or_null` / `_emit_missing_control_recipe_item` を削除
- [x] `mirbuilder/README` の consumer flow helper 記述を実装へ同期（削除 helper 名を除去）
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` → `[PASS] hako_mirbuilder phase11 pin: PASS`
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` → `[PASS] phase29bq_fast_gate_vm: PASS (mode=bq)`

### Session Update (2026-02-09, consumer node-type error wrapper inline cleanup)

- [x] `ProgramJsonV0PhaseStateConsumerBox._read_stmt_node_info_or_error` の node-type missing error 出力を inline 化し、単段 helper `_emit_missing_stmt_node_type` を削除
- [x] `ProgramJsonV0PhaseStateConsumerBox._dispatch_or_unsupported` の unsupported stmt error 出力を inline 化し、単段 helper `_emit_unsupported_stmt` を削除
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` → `[PASS] hako_mirbuilder phase11 pin: PASS`
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` → `[PASS] phase29bq_fast_gate_vm: PASS (mode=bq)`

### Session Update (2026-02-09, consumer node-info wrapper inline cleanup)

- [x] `ProgramJsonV0PhaseStateConsumerBox._read_stmt_node_info_or_error` で node info map 構築を inline 化し、単段 helper `_stmt_node_info_ok` / `_stmt_node_info_err` を削除
- [x] `ProgramJsonV0PhaseStateConsumerBox.consume_stmt` で missing state error 出力を inline 化し、単段 helper `_emit_missing_state_map` / `_empty_state_map` を削除
- [x] `mirbuilder/README` の consumer flow helper 記述を実装へ同期（node-info helper 名を除去）
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` → `[PASS] hako_mirbuilder phase11 pin: PASS`
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` → `[PASS] phase29bq_fast_gate_vm: PASS (mode=bq)`

### Session Update (2026-02-09, phase-state scan wrapper inline cleanup)

- [x] `ProgramJsonV0PhaseStateBox._scan_body_rec` で consumer error 行 fallback を inline 化し、単段 helper `_consumer_error_line_or_default` を削除
- [x] `ProgramJsonV0PhaseStateBox._scan_body_rec` で recipe item 検証を inline 化し、単段 helper `_verified_recipe_item_or_null` を削除
- [x] `ProgramJsonV0PhaseStateBox._state_from_out` の数値読み出しを `_out_i64` に統一し、単段 helper `_state_i64` を削除
- [x] `mirbuilder/README` の phase-state scan helper 記述を実装へ同期（削除 helper 名を除去）
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` → `[PASS] hako_mirbuilder phase11 pin: PASS`
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` → `[PASS] phase29bq_fast_gate_vm: PASS (mode=bq)`

### Session Update (2026-02-09, consumer control-state wrapper inline cleanup)

- [x] `ProgramJsonV0PhaseStateConsumerBox._handle_control_stmt_or_null` で control state seed 構築を inline 化し、単段 helper `_control_state` を削除
- [x] `mirbuilder/README` の helper summary を実装へ同期（`_control_state` 記述を除去）
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` → `[PASS] hako_mirbuilder phase11 pin: PASS`
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` → `[PASS] phase29bq_fast_gate_vm: PASS (mode=bq)`

### Session Update (2026-02-09, consumer emit-stmt helper inline cleanup)

- [x] `ProgramJsonV0PhaseStateConsumerBox._emit_non_control_stmt_or_handler_error` で `RecipeFactsBox.from_stmt` → `_verify_recipe_and_emit` を inline 化し、単段 helper `_emit_stmt_recipe` を削除
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` → `[PASS] hako_mirbuilder phase11 pin: PASS`
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` → `[PASS] phase29bq_fast_gate_vm: PASS (mode=bq)`

### Session Update (2026-02-09, phase-state next-state wrapper inline cleanup)

- [x] `ProgramJsonV0PhaseStateBox._scan_body_rec` で `next_state` 構築を inline 化し、単段 helper `_state_from_out` を削除
- [x] `mirbuilder/README` の out-map/scan helper 記述を実装へ同期（`_state_from_out` 記述を除去）
- [x] verify:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh` → `[PASS] hako_mirbuilder phase11 pin: PASS`
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` → `[PASS] phase29bq_fast_gate_vm: PASS (mode=bq)`
