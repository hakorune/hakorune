# `lang/src/compiler/mirbuilder/` (Phase-0)

Responsibility (Phase-0):
- Convert **Stage-B Program(JSON v0)** → **MIR JSON v0**.
- This layer is an **I/O + contract** boundary for starting the `.hako` MIR Builder migration.

Non-goals (Phase-0):
- SSA / AST rewrite / optimization. Do not add “workaround rewrites”.
- Silent fallback. Fail-fast with a stable tag.

Fail-fast tag (SSOT):
- All failures in this directory must start with: `[freeze:contract][hako_mirbuilder]`

Phase-1 (vocabulary rollout, 1 blocker = 1 commit):
- Vocabulary order (SSOT for Phase-1 rollout):
  - Literal/Const → Return → Local → Assignment → Call → Print → BlockExpr → If → Loop
- Unsupported nodes must fail-fast (no silent fallback):
  - Prefix: `[freeze:contract][hako_mirbuilder][cap_missing/<kind>] ...`
  - Example: `[freeze:contract][hako_mirbuilder][cap_missing/stmt:If] unsupported stmt in Program(JSON v0)`

Pinned fixture (Phase-0):
- `apps/tests/phase29bq_blockexpr_basic_min.hako`

Entry (Phase-0):
- `lang/src/compiler/mirbuilder/emit_mir_json_v0.hako`
  - Input (Phase-0): env `HAKO_PROGRAM_JSON_FILE` (preferred) or `HAKO_PROGRAM_JSON`, containing **Program(JSON v0)** or **AST JSON (roundtrip)**.
  - Output: prints **MIR JSON v0** to stdout (single line).

Quick manual run (Phase-0 pin):
- Emit AST JSON: `./target/release/hakorune --emit-ast-json /tmp/p.json apps/tests/phase29bq_blockexpr_basic_min.hako`
- Emit MIR JSON v0 (via `.hako` entry): `HAKO_PROGRAM_JSON_FILE=/tmp/p.json ./target/release/hakorune --backend vm lang/src/compiler/mirbuilder/emit_mir_json_v0.hako > /tmp/mir.json`
- Execute MIR JSON: `./target/release/hakorune --mir-json-file /tmp/mir.json`

Pinned fixture (Phase-1):
- `apps/tests/phase29bq_hako_mirbuilder_phase1_literal_return_min.hako`

Phase-1 pin note:
- Phase-1 uses `--emit-program-json-v0` as the input (Program JSON v0), not `--emit-ast-json`.

Phase-0/1/2 pin note (JoinIR / planner-required):
- The pin smokes run with `HAKO_JOINIR_PLANNER_REQUIRED=1` to avoid “fallback drift” when `.hako` code uses loops.

Pin semantics (stdout / rc SSOT):
- `--mir-json-file` uses the MIR `ret` value as the process exit code (e.g. Phase-4 rc=0, Phase-5 rc=7).

Pin script executable bit (SSOT):
- If you add a new pin script under `tools/smokes/**`, set the git executable bit: `git update-index --chmod=+x <script>`.

Program(JSON v0) scanner (Phase-3 SSOT):
- `lang/src/compiler/mirbuilder/program_json_v0_scanner_box.hako`
- Rule: call sites must not do ad-hoc string scans; extend the scanner SSOT instead.
- `ProgramJsonV0PhaseStateBox._scan_body_rec` keeps a state-map boundary (no long scalar arg chains) and delegates stmt decode to the consumer entry.

Program(JSON v0) → MIR JSON v0 (split boxes):
- Entry wrapper: `lang/src/compiler/mirbuilder/emit_mir_json_v0_from_program_json_v0.hako`
- Parser/state: `lang/src/compiler/mirbuilder/program_json_v0_phase_state_box.hako`
- State consumer SSOT: `lang/src/compiler/mirbuilder/program_json_v0_phase_state_consumer_box.hako`
- Consumer entry boundary: `ProgramJsonV0PhaseStateConsumerBox.consume_stmt(program_json, idx, tag, state_map)`
- Consumer stmt dispatch helper: `_dispatch_or_unsupported` に non-control (`_handle_non_control_stmt_or_null`) と control (`_handle_control_stmt_or_null`) を直列化し、unknown は fail-fast に収束
- Consumer fail-fast helpers: `_emit_missing_state_map` / `_emit_missing_stmt_node_type` / `_emit_unsupported_stmt` で `consume_stmt` 経路のエラー文言組み立てを集約（`_empty_state_map` で missing-state 既定値をSSOT化）
- Consumer state update helpers: `_state_base` が current_state snapshot をSSOT化し、`_state_with_{print,local,assignment,return}` は `_state_map` 再構築ではなく `set(...)` オーバーレイで差分更新のみを担う（copy/paste state fanout 禁止）
- Consumer map read helpers: `_map_i64` / `_map_s` で state/result/node map の型付き読み出しをSSOT化（`to_i64` / 文字列化の重複を抑止）
- Consumer emit-out helpers: `_shape_kind_from_state` / `_emit_out_with_state` で shape 判定と out map 組み立てを state map 経路へ統一し、`_emit_out_from_state` の scalar fanout を除去
- Consumer handler input helpers: `_non_control_handler_state_or_null` が `_state_base` から handler入力 map（Print/Local/Assignment/Return）を構築し、状態読み出し重複を局所化
- Consumer flow helpers: `consume_stmt` 本体は `_read_stmt_node_info_or_error` + `_dispatch_or_unsupported` に分離し、node info map 構築は `_read_stmt_node_info_or_error` 内へ inline 集約。non-control は `_non_control_handler_out_or_null` で handler呼び出しと結果 map（`stmt_name` + `result`）を集約し、`_handle_non_control_stmt_or_null` + `_after_state_from_non_control_result_or_null` + `_emit_non_control_stmt_or_handler_error` へ収束。error出力は `_emit_handler_error_or_null_at` をSSOTに統一し、control は `_control_handler_out_or_null` で handler呼び出しと結果 map（`handler_label` + `result`）を集約した上で `_emit_handler_error_or_null_at` と inline recipe_item 検証で recipe境界を集約（err判定 + recipe emit + next_idx解決）
- Phase state emit-out helper: `ProgramJsonV0PhaseStateBox._emit_out_with_state` が consumer の `_emit_out_with_state` bridge を担う単一出口（phase側の `_emit_out` は削除済み）
- Phase state out-map read helpers: `ProgramJsonV0PhaseStateBox._out_i64` / `_out_s` / `_out_box` で consumer out map 読み出しを型付き化し、`next_state` 構築と recipe item 受け渡しの重複参照を集約
- Phase state scan helpers: `ProgramJsonV0PhaseStateBox._emit_scan_done` で `recipe_root` 付き成功出力を集約し、scan前段（skip-ws→EOF→char read→EOF）は `_scan_body_rec` 冒頭へ直結して map ラップ層を廃止。遷移判定は `ch == ","` / `ch == "]"` + `_scan_body_rec` 再帰へ直結し、node/order 判定も `_scan_body_rec` 内へ直結（`read_node_type_at` + `LegacyOrderStage.classify` + order-stage inline update）し、consumer err/next_idx 判定も `_scan_body_rec` で out map を直接読む形へ統一。fail-fast 側は `_emit_out_with_state` を共通出口に `_emit_scan_error` + inline（err_line fallback / recipe_item 検証 / next_state 構築）で error 出力組み立てを集約
- Legacy fallback classifier: `lang/src/compiler/mirbuilder/program_json_v0_legacy_shape_classifier_box.hako`
- Legacy order stage gate: `lang/src/compiler/mirbuilder/program_json_v0_legacy_order_stage_box.hako`
- Statement handlers: `lang/src/compiler/mirbuilder/stmt_handlers/{print,local,assignment,return,if,loop}_stmt_handler.hako`
- MIR JSON builder entry: `lang/src/compiler/mirbuilder/mir_json_v0_builder_box.hako`（thin handoff）
- MIR JSON shape helper: `lang/src/compiler/mirbuilder/mir_json_v0_shape_box.hako`
- MIR JSON emit helper: `lang/src/compiler/mirbuilder/mir_json_v0_emit_box.hako`

Recipe-first status (R0-R6):
- Recipe item vocabulary: `lang/src/compiler/mirbuilder/recipe/recipe_item_box.hako`
- PortSig container: `lang/src/compiler/mirbuilder/recipe/recipe_port_sig_box.hako`
- Verifier skeleton: `lang/src/compiler/mirbuilder/recipe/recipe_verifier_box.hako`
- R1 facts extractor: `lang/src/compiler/mirbuilder/recipe/recipe_facts_box.hako`（`Print/Local/Assignment/Return`）
- R2 verifier wiring: `ProgramJsonV0PhaseStateConsumerBox` success paths always run verifier (`recipe_port_sig` / `recipe_verified_item` in state map, structure-verification mode).
- R3 lower wiring: `ProgramJsonV0PhaseStateBox` collects `recipe_root`; `MirJsonV0BuilderBox` verifies recipe root and selects `shape_kind` from recipe stmt sequence before MIR emission.
- R4 if wiring: `IfStmtHandler` + `MirJsonV0BuilderBox` (`phase10_local_if_vareqint_then_return_int_fallthrough_return_int`) で最小 If 受理を pin 化。
- R5 loop wiring: `LoopStmtHandler` + `MirJsonV0BuilderBox` (`phase11_local_loop_varltint_body_inc_return_var_or_int`) で最小 Loop 受理を pin 化（SSA-safe phi header）。
- R6 residue cleanup: legacy fallback classifier/order gate を helper 分離（`ProgramJsonV0LegacyShapeClassifierBox.classify` / `ProgramJsonV0LegacyOrderStageBox.classify`）し、Recipe-first 主経路との境界を明示。
- CS2 split (shape/emit):
  - `MirJsonV0ShapeBox.shape_inputs_with_recipe` が recipe verify + shape 判定（If/Loop/Seq）と recipe payload 展開を担当。
  - `MirJsonV0EmitBox.build_from_shape_inputs` が return guard + state→shape_ctx 構築 + MIR emit（control/non-control）を担当。
  - `MirJsonV0BuilderBox.build` は entry 互換を維持した thin wrapper（shape→emit の handoff のみ）として固定。
- Legacy fallback: when `recipe_root` is absent, `MirJsonV0BuilderBox` keeps existing `shape_kind` route for compatibility.

Cleanup integration boundary (C2/M7):
- SSOT lane: `docs/development/current/main/phases/phase-29bq/29bq-114-hako-cleanup-integration-prep-lane.md`
- Single dispatch entry:
  - `ProgramJsonV0PhaseStateConsumerBox.consume_stmt(...)`
  - `ProgramJsonV0PhaseStateConsumerBox._dispatch_or_unsupported(...)`
- Responsibility split:
  - consumer/handlers: node routing + recipe construction only
  - facts/verifier: cleanup acceptance and invariant checks
  - builder: verified recipe lowering only
- Current contract (post-M7-min-4):
  - Program(JSON v0) `StmtV0::Try` accepts non-loop minimal postfix cleanup only.
  - `finally` supports:
    - `Expr(Call env.console.log(var))` (M7-min-1)
    - `Local` assignment (`x = x + Int`) when Try body updates the same var (M7-min-3)
  - Pin: `tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_cleanup_try_min_vm.sh`
  - Pin (finally Local): `tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_cleanup_try_finally_local_min_vm.sh`
  - Non-minimal `Try(cleanup)` remains fail-fast (`[freeze:contract][hako_mirbuilder][cap_missing/stmt:Try]`).
- Reject boundary pin (M7-min-2):
  - `tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_cleanup_try_reject_nonminimal_vm.sh`
  - Cases: multi stmt / catches non-empty / loop+cleanup.
- Reject boundary pin (M7-min-4):
  - `tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_cleanup_try_finally_local_var_mismatch_reject_vm.sh`
  - Case: var-mismatch between Try body update and `finally Local` update.
- Next task:
  - failure-driven only: add new M7 pin/acceptance shape only when a new freeze/reject or contract change appears.

Phase-29bq handler split status:
- `program_json_v0_phase_state_box.hako` owns scan/order recursion only.
- Statement-shape field extraction is delegated directly to stmt handlers (`Print`/`Local`/`Assignment`/`Return`).
- Legacy compatibility consumer wrappers were removed in Post-M4 P5 (no runtime references).

module registration (SSOT):
- New `.hako` boxes should be exported from the owning `*/hako_module.toml` and listed in `hako.toml` `[modules.workspace].members`.
- `hako.toml` / `nyash.toml` `[modules]` direct entries are override/compat only (do not grow this table by default).

Guard rails (CI / fast gate):
- No hostbridge reference under mirbuilder:
  - `tools/checks/hako_mirbuilder_no_hostbridge.sh`
- Program(JSON v0) shape contract pin:
  - `tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_program_json_contract_pin_vm.sh`
  - Contract shapes: `Print`, `Expr(Call env.console.log(...))`, `If`, `Loop` (R5-min).
- Loop pin smoke:
  - `tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh`
- MIR instruction pin smoke (minimal widening):
  - `tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase12_return_newbox_min_vm.sh`
  - Contract: `Return(NewBox)` accepts `new <BoxType>()` only (`args=[]`).
  - `tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase13_return_call_id0_min_vm.sh`
  - Contract: `Return(Call)` accepts `id()` only (`name=id`, `args=[]`).
  - `tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase14_return_boxcall_stringbox_length_abc_min_vm.sh`
  - Contract: `Return(BoxCall)` accepts `new StringBox("abc").length()` only.
  - `tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase15_return_call_id1_int9_min_vm.sh`
  - Contract: `Return(Call)` one-arg widening accepts `id(9)` only (`name=id`, `args=[Int(9)]`).
  - `tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase16_return_newbox_stringbox_abc_min_vm.sh`
  - Contract: `Return(NewBox)` one-arg widening accepts `new StringBox("abc")` only.
  - `tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase17_return_boxcall_stringbox_indexof_b_abc_min_vm.sh`
  - Contract: `Return(BoxCall)` one-arg widening accepts `new StringBox("abc").indexOf("b")` only.
  - `tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase18_return_call_id1_int7_min_vm.sh`
  - Contract: `Return(Call)` one-arg widening accepts `id(7)` only (`name=id`, `args=[Int(7)]`).
  - `tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase19_load_local_var_min_vm.sh`
  - Contract: Load minimal accepts `Local(Int)>Local(Var)>Return(Var)` and emits `load/store` in MIR JSON v0.
  - Next lane SSOT (Load/Store docs-first):
    - `docs/development/current/main/design/hako-mirbuilder-load-store-minimal-contract-ssot.md`
    - fixed order: `LS0(v0 loader readiness)` -> `LS1(Load minimal)` -> `LS2(Store minimal)`（current next: `LS2`）
- Quick suite helper (daily/milestone):
  - quick: `tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_quick_suite_vm.sh`
  - milestone: `tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_quick_suite_vm.sh --with-stage1`
  - quick + bq: `tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_quick_suite_vm.sh --with-stage1 --with-bq`

Box naming (SSOT):
- A `static box FooBox { ... }` must be referenced as `FooBox` (the static box type name is part of the runtime contract).
- Rule: `using ... as FooBox` is allowed, but `using ... as Alias` is forbidden (do not rename box types via alias).
- Rationale (incident): aliasing can generate `NewBox Alias` at runtime and fail with `Unknown Box type: Alias`.
