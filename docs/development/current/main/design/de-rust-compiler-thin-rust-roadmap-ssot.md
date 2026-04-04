---
Status: Active (docs-first)
Scope: 脱Rust（compiler lane）として、Stage1 bridge の Rust 依存を薄くし、`.hako` compiler が Program/MIR を自力生成するまでの順序を固定する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/design/de-rust-full-rust-zero-roadmap-ssot.md
  - docs/development/current/main/design/selfhost-bootstrap-route-ssot.md
  - docs/development/current/main/design/de-rust-post-g1-runtime-plan-ssot.md
  - docs/development/current/main/design/de-rust-runtime-meaning-decision-red-inventory-ssot.md
  - docs/development/current/main/design/selfhost-parser-mirbuilder-migration-order-ssot.md
  - docs/development/current/main/phases/phase-29bq/29bq-115-selfhost-to-go-checklist.md
  - docs/development/current/main/phases/phase-29y/README.md
---

# De-Rust Compiler Thin-Rust Roadmap (SSOT)

## Goal

- Rust の責務を「ランナー/VM/LLVM 実行 + 最小ブリッジ」に縮退する。
- `.hako` compiler 経路で Program(JSON v0) と MIR(JSON v0) の両方を生成し、`hakorune` が `hakorune` を自己コンパイルできる状態を固定する。
- identity 証拠を Stage1 route のみで成立させる（stage0 fallback 依存を残さない）。

## Non-goals

- NyRT/runtime の全面 de-Rust（Phase 29y/29z ラインで継続）。
- 言語仕様拡張や parser grammar 拡張。
- fallback での通過（silent success）。

## Boundary note (2026-03-14)

- この文書は compiler lane の `thin-rust` 境界だけを定義する。
- `full Rust 0` の umbrella と runtime-zero / backend-zero の split tracking は
  `docs/development/current/main/design/de-rust-full-rust-zero-roadmap-ssot.md` を正本とする。
- backend-zero acceptance や readiness はこの文書では定義しない。

## Current boundary inventory (2026-02-11 snapshot, stage1-first)

| Boundary | Current owner | Evidence | Risk |
| --- | --- | --- | --- |
| Stage0 direct emit flags (`--emit-program-json-v0` / `--emit-mir-json`) | Rust compatibility lane（`--cli-mode stage0` 明示時のみ） | `tools/selfhost_identity_check.sh:48`, `tools/selfhost_identity_check.sh:145`, `tools/selfhost/lib/identity_routes.sh:172` | parser split-brain が残るため、daily evidence に混在させると ownership が曖昧化する |
| Stage1 `emit-program` | `.hako` Stage1 CLI + BuildBox/UsingResolver | `lang/src/runner/stage1_cli.hako:282`, `lang/src/runner/stage1_cli.hako:321` | `STAGE1_SOURCE_TEXT` 非依存時は FileBox 経路が残るため、環境差で失敗しうる |
| Stage1 `emit-mir` | `.hako` MirBuilder が MIR(JSON) を生成し、Rust bridge は受理/書き出しのみ | `lang/src/runner/stage1_cli.hako:121`, `src/runner/stage1_bridge/mod.rs:139`, `src/runner/stage1_bridge/mod.rs:169` | env-min 契約（`STAGE1_SOURCE_TEXT`=Program JSON）と subcmd 経路の parity 崩れが残リスク |
| Identity route（default=stage1） | stage1 env/subcmd を優先、`auto` は compat-only fallback（タグ付き） | `tools/selfhost/lib/identity_routes.sh:133`, `tools/selfhost/lib/identity_routes.sh:190`, `tools/selfhost_identity_check.sh:48`, `tools/selfhost_identity_check.sh:287` | `--cli-mode auto` は smoke で stage1 不具合を隠し得るため、full mode route guard が必須 |
| Selfhost Stage-A runtime route | Program(JSON) を Rust `json_v0_bridge` で MIR へ変換して実行 | `src/runner/selfhost.rs:357`, `src/runner/selfhost.rs:373`, `src/runner/selfhost.rs:397` | compiler ownership と runtime ownership が依然混在（thin-rust 最終到達前の残境界）。red inventory: `de-rust-runtime-meaning-decision-red-inventory-ssot.md` (`RDM-1`) |

## Target boundary (thin Rust)

- `.hako` 側 SSOT:
  - source -> Program(JSON v0): `lang/src/compiler/entry/compiler_stageb.hako`
  - Program(JSON v0) -> MIR(JSON v0): `.hako` mirbuilder entry（Stage1 CLI 経由）
- Rust 側 SSOT:
  - route orchestration / process management
  - JSON file execution (`--json-file`) と backend 実行（VM/LLVM/Cranelift）
  - verifier/passes/runtime safety

## Post-G1 Runtime Plan (accepted)

- runtime de-rust への移行順序・リスク・2週間計画は `docs/development/current/main/design/de-rust-post-g1-runtime-plan-ssot.md` を SSOT とする。
- この roadmap では compiler lane（D0-D4）の縮退を優先し、runtime lane（D5+ / Phase29y.1）は上記 SSOT に従って進める。

## Migration order (fixed)

### D0) Baseline lock (done before each task)

- `./tools/selfhost/run.sh --gate --planner-required 1 --max-cases 5 --jobs 4`
- `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`

### D1) `.hako` Stage1 CLI の `emit-mir` を本配線化

- 対象:
  - `lang/src/runner/stage1_cli.hako`
- 変更:
  - `Stage1Cli.emit_mir_json(...)` を “not wired” から `.hako` mirbuilder 呼び出しへ置換。
  - `emit mir-json` は Program ではなく MIR(JSON) を stdout へ出す。
- 受け入れ:
  - `tools/selfhost/compat/run_stage1_cli.sh emit mir-json apps/tests/hello_simple_llvm.hako` で `"functions"` を含む。
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_quick_suite_vm.sh --with-stage1` green。
- 進捗:
  - [x] D1-min1: `Stage1Cli.emit_mir_json(...)` を `.hako` mirbuilder 直結へ置換（2026-02-10）。
  - [x] 回帰確認: `phase29bq_hako_mirbuilder_quick_suite_vm.sh --with-stage1` green。

### D2) Rust `stage1_bridge` の emit-mir を thin wrapper 化

- 対象:
  - `src/runner/stage1_bridge/mod.rs`
- 変更:
  - `emit_mir` 時の Program(JSON) parse/lower（`json_v0_bridge`）を撤去。
  - child 出力が MIR(JSON) でない場合は fail-fast（契約タグ固定）。
- 受け入れ:
  - `hakorune --emit-mir-json <out> <src>`（Stage1 route）で MIR JSON が得られる。
  - D0 gate green。
- 進捗:
  - [x] D2-min1: emit-mir の Program(JSON) parse/lower を撤去し、child の MIR(JSON v0) を直接受理（2026-02-10）。
  - [x] 確認: `NYASH_USE_STAGE1_CLI=1 NYASH_STAGE1_MODE=emit-mir ./target/release/hakorune --emit-mir-json /tmp/d2_stage1_bridge_emit_mir.json apps/tests/hello_simple_llvm.hako` が `rc=0`。
  - [x] 回帰確認: `./tools/selfhost/run.sh --gate --planner-required 1 --max-cases 5 --jobs 4` green。

### D3) Identity route を stage1-first 契約へ固定（`auto` は compat-only）

- 対象:
  - `tools/selfhost/lib/identity_routes.sh`
  - `tools/selfhost_identity_check.sh`
- 変更:
  - stage1 env/subcmd route の `mir-json` で stage0 binary 再委譲を禁止（stage1 route 内で完結）。
  - full mode は Program/MIR の両方で stage1 route を必須化し、`auto` fallback は compat-only として隔離する。
- 受け入れ:
  - `tools/selfhost_identity_check.sh --mode full --skip-build --bin-stage1 <...> --bin-stage2 <...>` green（`--cli-mode` 省略 = stage1-first 既定）。
- 進捗:
  - [x] D3-min1: `tools/selfhost/lib/identity_routes.sh` mir-json env route から stage0 binary 再委譲を撤去（2026-02-11）。
  - [x] guard追加: `tools/selfhost_identity_check.sh` full mode で MIR 側も stage1 route 必須を固定。
  - [x] D3-min2a: stage1 env contract helper を SSOT 化（`tools/selfhost/lib/stage1_contract.sh`）し、`run_stage1_cli.sh` / `identity_routes.sh` で共通利用に統一（2026-02-11）。
  - [x] D3-min2b: contract smoke を 2ケース（emit-program positive / emit-mir invalid-path fail-fast）で追加し、`phase29bq_selfhost_stage1_contract_smoke_vm.sh` を green 固定（2026-02-11）。
  - [~] D3-min2c(trial): mir-json env route を helper経由 alias注入（`HAKO/NYASH/STAGE1`）+ `--from-program-json` まで拡張（2026-02-11）。
  - [x] D3-min3: prebuilt `stage1-cli` artifact で full identity を実行し、Program/MIR とも一致で green 固定（`--cli-mode` 省略 = stage1-first 既定, 2026-02-11）。
  - [x] D3-min10: `stage1_cli_env.hako` の mode alias を inline 正規化へ集約（`emit-program*` / `emit-mir*` の重複分岐を削減、2026-02-11）。
  - [x] D3-min11: `tools/selfhost/mainline/build_stage1.sh` の stage1-cli capability probe を `stage1_contract_exec_mode` 経由へ統一（2026-02-11）。
  - [x] D3-min12: helper-call mode 正規化で発生した `rc=97` 回帰の再導入を防ぐ guard を `phase29bq_selfhost_stage1_contract_smoke_vm.sh` に追加（alias mode 実行契約も同時固定、2026-02-11）。

### D4) Stage0 compatibility path の明示的縮退

- 対象:
  - `src/stage1/program_json_v0.rs`
  - `src/runner/emit.rs`
  - docs + scripts
- 変更:
  - stage0 direct flags は compatibility-only として隔離。
  - daily/milestone 運用入口は stage1 route のみを既定化。
- 受け入れ:
  - `CURRENT_TASK.md` / `10-Now.md` / selfhost scripts が stage1-first に整合。
  - stage0 route は `--cli-mode stage0` 明示時のみ使用。
- 進捗:
  - [x] D4-min1: `tools/selfhost_identity_check.sh` の既定 `--cli-mode` を `stage1` へ変更し、`auto` は compat-only 表記へ変更（2026-02-11）。
  - [x] D4-min2: `tools/selfhost/lib/identity_routes.sh` の auto fallback で stage0 へ落ちる際に `[identity/compat-fallback]` 診断タグを 1行出力するよう固定（2026-02-11）。
  - [x] D4-min3: `tools/selfhost_identity_check.sh` buildモードで `cli-mode=stage1/auto` 時は `stage1-cli` artifact を既定生成し、`stage0` 時のみ launcher artifact を生成するよう配線（stage1-cli は bootstrap compiler ではないため Stage2 build は default bootstrap で実行、2026-02-11）。
  - [x] D4-min4: `10-Now.md` / `phase-29bq/README.md` の daily 実行手順を stage1-first 既定（`cli-mode=stage1`, `stage1-cli(.stage2)`）へ同期し、`auto` は compat-only と明記（2026-02-11）。
  - [x] D4-min5: `29bq-90-selfhost-checklist.md` の identity 実行例を stage1-first 既定（`--cli-mode` 省略 + `stage1-cli(.stage2)` 既定パス）へ同期し、`auto` は互換診断用であることを明記（2026-02-11）。
  - [x] D4-min6: `29bq-115-selfhost-to-go-checklist.md` の identity コマンド例を stage1-first 既定（`--cli-mode` 省略 + `stage1-cli(.stage2)` 既定パス）へ同期し、旧 `--cli-mode stage1` 固定例を整理（2026-02-11）。
  - [x] D4-min7: `29bq-91-mirbuilder-migration-progress-checklist.md` の identity 証跡コマンドを stage1-first 既定（`--cli-mode` 省略 + `stage1-cli.stage2`）へ同期（2026-02-11）。
  - [x] D4-min8: `selfhost-bootstrap-route-ssot.md` / `coreplan-migration-roadmap-ssot.md` の identity 証跡コマンドを stage1-first 既定（`--cli-mode` 省略 + `stage1-cli.stage2`）へ同期（2026-02-11）。
  - [x] D4-min9: D3 節の identity 証跡表記を stage1-first 既定へ更新（`--cli-mode stage1` 固定記述を削除、2026-02-11）。
  - [x] D4-min10: `CURRENT_TASK.md` の Daily Checkpoint に残っていた identity 証跡コマンドを stage1-first 既定表記（`--cli-mode` 省略 + `stage1-cli.stage2`）へ同期（2026-02-11）。
  - [x] D4-min11: boundary inventory（旧 2026-02-10 snapshot）を stage1-first 現状へ更新し、owner/evidence/risk を再定義（2026-02-11）。
  - [x] D4-min12: D3 節の見出し/契約文を実装現状（default=stage1、`auto` は compat-only）と一致する表現へ更新（2026-02-11）。
  - [x] D4-min13: `10-Now.md` の daily 入口へ boundary inventory（2026-02-11 snapshot）参照を追記し、`CURRENT_TASK.md` と導線を同期（2026-02-11）。
  - [x] D4-min14: `29bq-90-selfhost-checklist.md` の daily run order（Identity節）に boundary inventory 参照ステップを追加し、入口を 3点（`CURRENT_TASK.md` / `10-Now.md` / checklist）で同期（2026-02-11）。
  - [x] D4-min15: `29bq-115-selfhost-to-go-checklist.md` の G1 closeout 手順に boundary inventory 参照を追加し、closeout 導線も stage1-first 境界認識へ同期（2026-02-11）。
  - [x] D4-min16: `29bq-91-mirbuilder-migration-progress-checklist.md` の snapshot 節へ boundary inventory 参照を追加し、進捗台帳の daily 入口を stage1-first 境界認識へ同期（2026-02-11）。
  - [x] D4-min17: `29bq-91` latest snapshot を 2026-02-11 に更新し、identity + daily gate（5-case）の最新実測を同期（2026-02-11）。
  - [x] D4-min18: `29bq-115` normalized snapshot を 2026-02-11 へ更新し、G1 identity 証跡を stage1-first 既定パス付き表記へ同期（2026-02-11）。
  - [x] D4-min19: de-rust lane docs の旧 snapshot/date 表記を棚卸しし、`CURRENT_TASK.md` の closeout snapshot 表記を 2026-02-11 へ同期（2026-02-11）。
  - [x] D4-min20: `10-Now.md` の `Identity default paths` 表記を `--cli-mode` 省略（stage1-first 既定）へ同期し、`--cli-mode stage1` 固定の残件を 1 件解消（2026-02-11）。
  - [x] D4-min21: `29bq-90-selfhost-checklist.md` の `既定パス` 表記を `--cli-mode` 省略（stage1-first 既定）へ同期（2026-02-11）。
  - [x] D4-min22: `29bq-91-mirbuilder-migration-progress-checklist.md` の 5-case daily gate 実測値を最新 run（`stageb_total_secs=18`, `avg_case_secs=3.60`）へ同期（2026-02-11）。
  - [x] D4-min23: `CURRENT_TASK.md` の Daily Checkpoint（2026-02-11）にある 5-case gate 実測値を最新 run（`stageb_total_secs=18`, `avg_case_secs=3.60`）へ同期（2026-02-11）。
  - [x] D4-min24: `CURRENT_TASK.md` の Daily Checkpoint（2026-02-11）に identity full PASS 証跡（`--mode full`）を 1 行追記し、`29bq-91` / `29bq-115` と証跡粒度を同期（2026-02-11）。
  - [x] D4-min25: compat route は explicit opt-in（`--allow-compat-route`）必須へ固定し、`legacy_main_readiness.sh` / `pre_promote_legacy_main_removal.sh` の既定を stage1-first（`stage1-cli(.stage2)`）へ同期。guard smoke `phase29bq_selfhost_identity_compat_route_guard_vm.sh` を追加して契約を pin（2026-02-16）。

### D5) Selfhost Stage-A runtime route の MIR-first 契約固定

- 対象:
  - `src/runner/selfhost.rs`
  - `src/runner/modes/common_util/selfhost/child.rs`
  - `tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_runtime_route_smoke_vm.sh`
  - `tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_runtime_route_program_reject_smoke_vm.sh`
- 変更:
  - stage-a route の child capture を Program/MIR 両観測に拡張し、pipe 目詰まりを避けるため temp-file capture に切替。
  - strict/dev(+planner_required) で Program(JSON v0) 入力を fail-fast（`[contract][runtime-route][expected=mir-json]` + exit 1）へ固定。
  - non-strict は `.hako` mirbuilder 優先で MIR 化を試み、未対応形は compat lane（Rust `json_v0_bridge`）へ限定フォールバック。
  - normal/fail の 2 本 gate を追加・固定。
- 進捗:
  - [x] D5-min1: Stage-A runtime route の MIR-first 契約固定（strict fail-fast + normal/fail gate）を実装（2026-02-11）。
  - [x] 検証: `bash tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_runtime_route_smoke_vm.sh` PASS（stage-a accepted tag 確認）。
  - [x] 検証: `bash tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_runtime_route_program_reject_smoke_vm.sh` PASS（strict+planner_required で rc=1 固定）。
  - [x] 回帰確認: `./tools/selfhost/run.sh --gate --planner-required 1 --max-cases 5 --jobs 4` PASS（`5/5`, `stageb_total_secs=19`, `avg_case_secs=3.80`）。

### D6) `.hako` VM backend 枠（Phase29z-S0a）

- 対象:
  - `src/runner/dispatch.rs`
  - `src/runner/modes/mod.rs`
  - `src/runner/modes/vm_hako.rs`
  - `src/cli/args.rs`
  - `tools/smokes/v2/profiles/integration/apps/archive/phase29z_vm_hako_backend_frame_vm.sh`
- 変更:
  - `--backend vm-hako` を dispatcher で受理し、専用 mode へ配線。
  - S0a は backend frame のみ実装し、実行は `[vm-hako/unimplemented] phase=s0a ... op=*` で fail-fast 固定（fallback なし）。
  - 契約固定用 smoke を追加し、`vm-hako` が「受理 + 非0終了 + 安定タグ」を満たすことを pin。
- 進捗:
  - [x] D6-min1: `vm-hako` backend frame + fail-fast tag を実装（2026-02-11）。
  - [x] D6-min2: S0b (`const/add/return`) を実装し、`phase29z_vm_hako_s0_const_add_return_parity_vm.sh` で parity 固定（`vm=42`, `vm-hako=42`）。
  - [x] D6-min3: S0c (`copy`) を実装し、`phase29z_vm_hako_s0_copy_add_return_parity_vm.sh` で parity 固定（`vm=42`, `vm-hako=42`）。
  - [x] D6-min4: S0d (`unop(neg)`) を実装し、`phase29z_vm_hako_s0_unary_neg_copy_add_parity_vm.sh` で parity 固定（`vm=42`, `vm-hako=42`）。
  - [x] D6-min5: S0e (`unop(not)`) を実装し、`phase29z_vm_hako_s0_unary_not_copy_add_parity_vm.sh` で parity 固定（`vm=1`, `vm-hako=1`）。
  - [x] D6-min6: S0f (`binop(sub)`) を subset-check に追加し、`phase29z_vm_hako_s0_copy_sub_return_parity_vm.sh` を追加。reject fixture を `mul` へ更新して `phase29z_vm_hako_backend_frame_vm.sh` の fail-fast 契約を維持。
  - [x] D6-min7: S0g (`binop(mul)`) を subset-check に追加し、`phase29z_vm_hako_s0_copy_mul_return_parity_vm.sh` を追加。reject fixture を `div` へ更新して `phase29z_vm_hako_backend_frame_vm.sh` の fail-fast 契約を維持。
  - [x] D6-min8: S0h (`binop(div)`) を subset-check に追加し、`phase29z_vm_hako_s0_copy_div_return_parity_vm.sh` を追加。reject fixture を `mod` へ更新して `phase29z_vm_hako_backend_frame_vm.sh` の fail-fast 契約を維持。
  - [x] D6-min9: S0i (`binop(mod)`) を subset-check に追加し、`phase29z_vm_hako_s0_copy_mod_return_parity_vm.sh` を追加。reject fixture を `compare` へ更新して `phase29z_vm_hako_backend_frame_vm.sh` の fail-fast 契約を維持。
  - [x] D6-min10: S0j (`compare(eq)`) を subset-check / `.hako` runner に追加し、`phase29z_vm_hako_s0_compare_eq_return_parity_vm.sh` を追加。reject fixture を `compare_lt` へ更新して `phase29z_vm_hako_backend_frame_vm.sh` の fail-fast 契約を維持。
  - [x] D6-min11: S0k (`compare(lt)`) を subset-check / `.hako` runner に追加し、`phase29z_vm_hako_s0_compare_lt_return_parity_vm.sh` を追加。reject fixture を `compare_ne` へ更新して `phase29z_vm_hako_backend_frame_vm.sh` の fail-fast 契約を維持。
  - [x] D6-min12: S0l (`branch/jump` 2block if最小) を subset-check / `.hako` runner に追加し、`phase29z_vm_hako_s0_branch_jump_if_return_parity_vm.sh` を追加。reject fixture は `compare_ne` を維持して `phase29z_vm_hako_backend_frame_vm.sh` の fail-fast 契約を継続。
  - [x] D6-min13: S0m (`Const(bool)`) を subset-check / `.hako` runner に追加し、`phase29z_vm_hako_s0_const_bool_branch_return_parity_vm.sh` を追加。backend frame を `phase=s0m` へ更新して `phase29z_vm_hako_backend_frame_vm.sh` の fail-fast 契約を継続。
  - [x] D6-min14: S1a (`newbox(StringBox)`) を subset-check に追加し、`phase29z_vm_hako_s1_newbox_string_return_parity_vm.sh` を追加。backend frame を `phase=s1a` へ更新して `phase29z_vm_hako_backend_frame_vm.sh` の fail-fast 契約を継続。
  - [x] D6-min15: S1b (`id/0` 呼び出し) を `boxcall(id)` / `call(id)` bridge で受理。`src/runner/modes/vm_hako.rs` を `compile -> subset -> payload -> driver` へ分割し、payload に `call0_const_map` / `boxcall0_const_map` を追加。`phase29z_vm_hako_s1_call_id0_return_parity_vm.sh` を追加し、backend frame を `phase=s1b` へ更新。
  - [x] D6-min16: S1c (`id(1-int)` call) を `call` bridge で受理。payload に `call1_arg0_map` / `boxcall1_arg0_map` を追加し、`phase29z_vm_hako_s1_call_id1_return_parity_vm.sh` を追加。backend frame を `phase=s1c` へ更新。
  - [x] D6-min17: S1d（method-call `id(1-int)`）を legacy `call(args=2)` bridge で受理。subset-check の reject を `call(args>2)` / `call(args2:*)` へ分割し、payload 側で `call(args=2)` を bridge `call(args=1)` へ正規化。`phase29z_vm_hako_s1_boxcall_id1_return_parity_vm.sh` を追加し、backend frame を `phase=s1d` へ更新。
  - [x] D6-min18: S1e（direct `boxcall(args=1)` `id(1-int)`）を fixture+gate で固定。`phase29z_vm_hako_s1_boxcall_id1_return_parity_vm.sh` に MIR 形契約（`boxcall(id,args=1)` present / dynamic `call(args=2,func=4294967295)` absent）を追加し、`--no-optimize` 依存を削除。backend frame / runner phase を `s1e` へ更新。
  - [x] D6-min19: S1f（`call(id0/id1-int)` bridge optimize-on）を fixture+gate で固定。`phase29z_vm_hako_s1_call_id0_return_parity_vm.sh` / `phase29z_vm_hako_s1_call_id1_return_parity_vm.sh` に MIR 形契約（`call(args=0/1)` present / dynamic `call(args=2,func=4294967295)` absent）を追加し、`--no-optimize` 依存を削除。backend frame / runner phase を `s1f` へ更新。
  - [x] D6-min20: S2a（`externcall(print 1-int)`）を fixture+gate で固定。subset-check は `externcall` / `mir_call(print)` / dynamic `call(dst=null,func=4294967295,args=1)` の最小許容に限定し、payload 正規化を `externcall(func=nyash.console.log,args=1)` へ統一。`.hako` runner に `externcall` / `mir_call` print 実行を追加。`phase29z_vm_hako_s2_externcall_print_i64_return_parity_vm.sh` を追加し、backend frame / runner phase を `s2a` へ更新。
  - [x] D6-min21: S2a cleanup（BoxShape）として print 形判定を `parse_print_arg_from_instruction` へ集約し、subset-check / payload 正規化の判定SSOTを一本化。`call(dst=null)` の print 受理は `args=1` のみに厳格化し、`.hako` 側 `mir_call(print)` 分岐は compat-only + 撤去条件をコメントで明記。
  - [x] D6-min22: S2b（`externcall(print 1-reg)` compare-origin bool 出力）を fixture+gate で固定。`phase29z_vm_hako_s2_externcall_print_bool_compare_return_parity_vm.sh` を追加し、`true/false` を `1/0` 正規化した stdout parity を契約化。backend frame / runner phase を `s2b` へ更新。
  - [x] D6-min23: S2c（externcall-only vm execution）として `.hako vm` の compat-only `mir_call(print)` 実行分岐を撤去。S2 fixture 群（int/bool compare）の externcall-only 契約で回帰がないことを確認し、backend frame / runner phase を `s2c` へ更新。
  - [x] D6-min24: S2d（strict-first legacy withdrawal）として Rust側 print canonicalizer の `mir_call(print)` 受理を strict-first で段階撤去。strict 時は `mir_call(legacy-disabled)` で fail-fast、non-strict は互換維持。backend frame / runner phase を `s2d` へ更新。
  - [x] D6-min25: S2e（full legacy withdrawal）として Rust側 print canonicalizer の `mir_call(print)` 受理を全モード撤去。`mir_call(legacy-removed)` で fail-fast し、`vm-hako` print入口を externcall-only 契約へ統一。backend frame / runner phase を `s2e` へ更新。
  - [x] D6-min26: S3a（`nop`）として `vm-hako` subset-check / `.hako runner` に `nop` を追加。Rust `mir_json_v0` loader も `op=nop` を受理するよう更新し、`phase29z_vm_hako_s3_nop_parity_vm.sh`（rust-vm core route vs hako-runner route）で parity を固定。backend frame / runner phase を `s3a` へ更新。
  - [x] D6-min27: S3b（`safepoint`）として `vm-hako` subset-check / `.hako runner` に `safepoint` を追加。Rust `mir_json_v0` loader も `op=safepoint` を受理するよう更新し、`phase29z_vm_hako_s3_safepoint_parity_vm.sh`（rust-vm core route vs hako-runner route）で parity を固定。backend frame / runner phase を `s3b` へ更新。
  - [x] D6-min28: S3c（`keepalive`）として `vm-hako` subset-check / `.hako runner` に `keepalive` を追加。Rust `mir_json_v0` loader も `op=keepalive` を受理するよう更新し、`phase29z_vm_hako_s3_keepalive_parity_vm.sh`（rust-vm core route vs hako-runner route）で parity を固定。backend frame / runner phase を `s3c` へ更新。
  - [x] D6-min29: S3d（`release_strong`）として `vm-hako` subset-check / `.hako runner` に `release_strong` を追加。Rust `mir_json_v0` loader も `op=release_strong` を受理するよう更新し、`phase29z_vm_hako_s3_release_strong_parity_vm.sh`（rust-vm core route vs hako-runner route）で parity を固定。backend frame / runner phase を `s3d` へ更新。
  - [x] D6-min30: S4a（`debug`）として `vm-hako` subset-check / `.hako runner` に `debug` を追加。Rust `mir_json_v0` loader も `op=debug` を受理するよう更新し、`phase29z_vm_hako_s4_debug_parity_vm.sh`（rust-vm core route vs hako-runner route）で parity を固定。backend frame / runner phase を `s4a` へ更新。
  - [x] D6-min31: S4b（`debug_log`）として `vm-hako` subset-check / `.hako runner` に `debug_log` を追加。Rust `mir_json_v0` loader も `op=debug_log` を受理するよう更新し、`phase29z_vm_hako_s4_debug_log_parity_vm.sh`（rust-vm core route vs hako-runner route）で parity を固定。backend frame / runner phase を `s4b` へ更新。
  - [x] D6-min32: S4c（`select`）として `vm-hako` subset-check / `.hako runner` に `select` を追加。Rust `mir_json_v0` loader も `op=select` を受理するよう更新し、`phase29z_vm_hako_s4_select_parity_vm.sh`（rust-vm core route vs hako-runner route）で parity を固定。backend frame / runner phase を `s4c` へ更新。
  - [x] D6-min33: S4d（`barrier`）として `vm-hako` subset-check / `.hako runner` に `barrier` を追加。Rust `mir_json_v0` loader も `op=barrier`（`kind=read|write`/`op_kind=Read|Write`）を受理するよう更新し、`phase29z_vm_hako_s4_barrier_parity_vm.sh`（rust-vm core route vs hako-runner route）で parity を固定。backend frame / runner phase を `s4d` へ更新。
  - [x] D6-clean-1: BoxShape cleanup として `lang/src/vm/boxes/mir_vm_s0.hako` の bool const 判定を helper 化（`_find_bool_const_value` / `_read_bool_literal`）し、空白あり JSON（`"type": "bool", "value": true|false`）を `phase29z_vm_hako_s4_bool_const_ws_parity_vm.sh` で契約化。
  - [x] D6-clean-2: BoxShape cleanup として `lang/src/vm/boxes/mir_vm_s0.hako` の scanner 再帰（`_scan_obj_end_rec` / `_scan_array_end_rec`）へ step-limit を追加し、深い payload は `[vm-hako/contract][scan-depth-exceeded kind=*]` で fail-fast 固定。`phase29z_vm_hako_s4_scanner_depth_guard_parity_vm.sh` で「rust-vm=42 / hako-runner=depth-guard(rc=1)」契約を pin。
  - [x] D6-clean-3: BoxShape cleanup として `src/runner/modes/vm_hako.rs` の shape 検証を helper 境界へ分離し、subset-check の責務（受理/拒否）を見通しよく固定。RDN-1 後の現行 pin は `subset_rejects_legacy_debug_log_even_with_non_reg_values` / `subset_rejects_select_missing_then_val`。
  - [x] D6-clean-4: BoxShape cleanup として S4 parity fixture を語彙最小へ再点検し、`vm_hako_json_parity_common.sh` に `VM_HAKO_PARITY_DENY_OPS`（comma list）契約を追加。RDN-1 後は legacy `debug_log` は unknown-op fail-fast として維持する。
  - [x] D6-min34: S5a（`load`）として `vm-hako` subset-check / `.hako runner` に `load` を追加。`mir_vm_s0.hako` は `mem` を別マップで保持し、`load(dst,ptr)` は `mem[ptr]`（未初期化=0）を返す契約で固定。backend frame / runner phase を `s5a` へ更新。
  - [x] D6-min35: S5b（`store`）として `vm-hako` subset-check / `.hako runner` に `store` を追加。`store(ptr,value)` は `mem[ptr] = reg[value]` 契約で固定し、`store -> load -> ret` 最小 fixture で parity を pin。backend frame / runner phase を `s5b` へ更新。
  - [x] D6-min36: S5c（`phi`）として `vm-hako` subset-check / `.hako runner` に `phi` を追加。`mir_vm_s0.hako` は block predecessor（`prev_bb`）に基づいて `incoming` から値を選択し、`pred=null` 時は first incoming を採用する最小契約で固定。backend frame / runner phase を `s5c` へ更新。
  - [x] D6-min37: S5d（`typeop`）として `vm-hako` subset-check / `.hako runner` / Rust `mir_json_v0` loader に `typeop` を追加。`operation=check|cast`（`is/as`, `op_kind` alias許容）+ `src/value` + `target_type/ty` の最小形を受理し、`cast(Integer)` parity を fixture+smoke で固定。backend frame / runner phase を `s5d` へ更新。
  - [x] D6-min38: S5e（`weakref`）として `vm-hako` subset-check / `.hako runner` / Rust `mir_json_v0` loader に `weak_new` / `weak_load` を追加。`weak_new(dst,box_val)` / `weak_load(dst,weak_ref)` の最小形を受理し、`newbox -> weak_new -> weak_load -> ret` parity を fixture+smoke で固定。backend frame / runner phase を `s5e` へ更新。
  - [x] D6-min39: S5f（`ref_new`）として `vm-hako` subset-check / `.hako runner` / Rust `mir_json_v0` loader / Rust VM interpreter に `ref_new` を追加。`ref_new(dst,box_val)` の最小形を受理し、`newbox -> ref_new -> ret` parity を fixture+smoke で固定。backend frame / runner phase を `s5f` へ更新。
  - [x] D6-min40: S5g（`future_new`）として `vm-hako` subset-check / `.hako runner` / Rust `mir_json_v0` loader に `future_new` を追加。`future_new(dst,value)` の最小形を受理し、`const -> future_new -> ret` parity を fixture+smoke で固定。backend frame / runner phase を `s5g` へ更新。
  - [x] D6-min41: S5h（`future_set`）として `vm-hako` subset-check / `.hako runner` / Rust `mir_json_v0` loader に `future_set` を追加。`future_set(future,value)` の最小形を受理し、`future_new -> future_set -> ret` parity を fixture+smoke で固定。backend frame / runner phase を `s5h` へ更新。
  - [x] D6-min42: S5i（`await`）として `vm-hako` subset-check / `.hako runner` / Rust `mir_json_v0` loader に `await` を追加。`await(dst,future)` の最小形を受理し、`future_new -> await -> ret` parity を fixture+smoke で固定。backend frame / runner phase を `s5i` へ更新。
  - [x] D6-min43: S5k（`array_get`）として `vm-hako` subset-check / `.hako runner` / Rust `mir_json_v0` loader に `array_get` を追加。`array_get(dst,array,index)` の最小形を受理し、`phase29z_vm_hako_s5_array_get_parity_vm.sh` で JSON route parity を固定。backend frame / runner phase を `s5k` へ更新。
  - [x] D6-min44: S5l（`array_set`）として `vm-hako` subset-check / `.hako runner` / Rust `mir_json_v0` loader に `array_set` を追加。`array_set(array,index,value)` の最小形を受理し、`phase29z_vm_hako_s5_array_set_parity_vm.sh` で JSON route parity を固定。backend frame / runner phase を `s5l` へ更新。
  - [x] D6-clean-11: `array_get/array_set` shim を register-slot 契約へ整列し、`index` は shape-only で扱う SSOT（`vm-hako-array-shim-contract-ssot.md`）を追加。backend frame の phase 契約は `vm_hako_phase.sh` から自動解決へ移行。
  - [x] D6-clean-5: `weak_new` precondition を fail-fast 化。`.hako` runner で register kind（`box|weak|scalar`）を追跡し、`weak_new(non-box)` は `[vm-hako/contract][weak_new-non-box]` で拒否。`phase29z_vm_hako_s5_weakref_non_box_reject_vm.sh` を追加して reject 契約を固定。
  - [x] D6-clean-6: BoxShape cleanup として `mir_vm_s0.hako` の `_exec_op` を `_exec_data_op` / `_exec_call_op` へ分割。語彙追加なしで責務を整理し、`weakref` parity を維持。
  - [x] D6-clean-7: Rust subset validator を `ensure_u64_fields` へ共通化し、`load/store/select/phi/typeop/weakref` の形検証重複を削減。
  - [x] D6-clean-8: alias 正規化 SSOT を `normalize_instruction_aliases` に集約。subset-check（`normalize_aliases_in_root`）と payload loader（`extract_main_payload_json`）で共通利用し、`operation/op_kind`, `kind/op_kind`, `src/value`, `target_type/ty` の drift を防止。
  - [x] D6-clean-9: `await(non-future)` reject を cross-route smoke で固定。`phase29z_vm_hako_s5_await_non_future_reject_vm.sh` を追加し、hako-runner は `[vm-hako/contract][await-non-future]` で fail-fast、rust-vm/hako-runner ともに `rc=1` を契約化。
  - [x] D6-clean-10: `new_closure` の現状挙動を cross-route probe smoke で固定。`phase29z_vm_hako_s5_newclosure_probe_vm.sh` を追加し、rust-vm は `unsupported op 'new_closure' in mir_json_v0 loader`、hako-runner は `[vm-hako/unimplemented op=new_closure]` で fail-fast、両routeとも `rc=1` を契約化。
  - [x] D6-clean-12: `NewClosure` の MIR JSON emit を canonical `mir_call(callee=Closure)` へ統一し、`backend_core_ops` allowlist/emitters を同期。併せて `vm_hako` driver + planner gate + newclosure probe の route を `NYASH_VM_HAKO_PREFER_STRICT_DEV=0` で rust-vm lane に固定し、strict/dev route drift を抑止。
  - [x] 検証: `cargo check -q --bin hakorune` PASS。
  - [x] 検証: `bash tools/smokes/v2/profiles/integration/apps/archive/phase29z_vm_hako_s1_call_id0_return_parity_vm.sh` PASS（`vm=7`, `vm-hako=7`）。
  - [x] 検証: `bash tools/smokes/v2/profiles/integration/apps/archive/phase29z_vm_hako_s1_call_id1_return_parity_vm.sh` PASS（`vm=9`, `vm-hako=9`）。
  - [x] 検証: `bash tools/smokes/v2/profiles/integration/apps/archive/phase29z_vm_hako_s1_boxcall_id1_return_parity_vm.sh` PASS（`vm=9`, `vm-hako=9`）。
  - [x] 検証: `bash tools/smokes/v2/profiles/integration/apps/archive/phase29z_vm_hako_s2_externcall_print_i64_return_parity_vm.sh` PASS（`vm=7`, `vm-hako=7`, `print=9`）。
  - [x] 検証: `bash tools/smokes/v2/profiles/integration/apps/archive/phase29z_vm_hako_backend_frame_vm.sh` PASS（`rc=1`）。
  - [x] 検証: `bash tools/smokes/v2/profiles/integration/apps/archive/phase29z_vm_hako_s1_call_id1_return_parity_vm.sh` PASS（post-D6-min21 recheck）。
  - [x] 検証: `bash tools/smokes/v2/profiles/integration/apps/archive/phase29z_vm_hako_s2_externcall_print_i64_return_parity_vm.sh` PASS（post-D6-min21 recheck）。
  - [x] 検証: `bash tools/smokes/v2/profiles/integration/apps/archive/phase29z_vm_hako_backend_frame_vm.sh` PASS（post-D6-min21 recheck）。
  - [x] 回帰確認: `./tools/selfhost/run.sh --gate --planner-required 1 --max-cases 5 --jobs 4` PASS（post-D6-min21: `5/5`, `stageb_total_secs=15`, `avg_case_secs=3.20`）。
  - [x] 検証: `bash tools/smokes/v2/profiles/integration/apps/archive/phase29z_vm_hako_s2_externcall_print_bool_compare_return_parity_vm.sh` PASS（`vm=7`, `vm-hako=7`, `bool01='1 0'`）。
  - [x] 検証: `bash tools/smokes/v2/profiles/integration/apps/archive/phase29z_vm_hako_backend_frame_vm.sh` PASS（post-D6-min22 recheck）。
  - [x] 回帰確認: `./tools/selfhost/run.sh --gate --planner-required 1 --max-cases 5 --jobs 4` PASS（post-D6-min22: `5/5`, `stageb_total_secs=20`, `avg_case_secs=4.00`）。
  - [x] 検証: `bash tools/smokes/v2/profiles/integration/apps/archive/phase29z_vm_hako_s2_externcall_print_i64_return_parity_vm.sh` PASS（post-D6-min23 recheck）。
  - [x] 検証: `bash tools/smokes/v2/profiles/integration/apps/archive/phase29z_vm_hako_s2_externcall_print_bool_compare_return_parity_vm.sh` PASS（post-D6-min23 recheck）。
  - [x] 検証: `bash tools/smokes/v2/profiles/integration/apps/archive/phase29z_vm_hako_backend_frame_vm.sh` PASS（post-D6-min23 recheck）。
  - [x] 回帰確認: `./tools/selfhost/run.sh --gate --planner-required 1 --max-cases 5 --jobs 4` PASS（post-D6-min23: `5/5`, `stageb_total_secs=20`, `avg_case_secs=4.00`）。
  - [x] 検証: `bash tools/smokes/v2/profiles/integration/apps/archive/phase29z_vm_hako_s2_externcall_print_i64_return_parity_vm.sh` PASS（post-D6-min24 recheck）。
  - [x] 検証: `bash tools/smokes/v2/profiles/integration/apps/archive/phase29z_vm_hako_s2_externcall_print_bool_compare_return_parity_vm.sh` PASS（post-D6-min24 recheck）。
  - [x] 検証: `bash tools/smokes/v2/profiles/integration/apps/archive/phase29z_vm_hako_backend_frame_vm.sh` PASS（post-D6-min24 recheck）。
  - [x] 回帰確認: `./tools/selfhost/run.sh --gate --planner-required 1 --max-cases 5 --jobs 4` PASS（post-D6-min24: `5/5`, `stageb_total_secs=20`, `avg_case_secs=4.00`）。
  - [x] 検証: `cargo check -q --bin hakorune` PASS（post-D6-min25）。
  - [x] 検証: `cargo test -q mir_call_print_ -- --nocapture` PASS（post-D6-min25）。
  - [x] 検証: `bash tools/smokes/v2/profiles/integration/apps/archive/phase29z_vm_hako_s2_externcall_print_i64_return_parity_vm.sh` PASS（post-D6-min25 recheck）。
  - [x] 検証: `bash tools/smokes/v2/profiles/integration/apps/archive/phase29z_vm_hako_s2_externcall_print_bool_compare_return_parity_vm.sh` PASS（post-D6-min25 recheck）。
  - [x] 検証: `bash tools/smokes/v2/profiles/integration/apps/archive/phase29z_vm_hako_backend_frame_vm.sh` PASS（post-D6-min25 recheck）。
  - [x] 回帰確認: `./tools/selfhost/run.sh --gate --planner-required 1 --max-cases 5 --jobs 4` PASS（post-D6-min25）。
  - [x] 検証: `cargo test -q subset_accepts_nop -- --nocapture` PASS（post-D6-min26）。
  - [x] 検証: `bash tools/smokes/v2/profiles/integration/apps/phase29z_vm_hako_s3_nop_parity_vm.sh` PASS（`rust-vm=42`, `hako-runner=42`）。
  - [x] 検証: `bash tools/smokes/v2/profiles/integration/apps/archive/phase29z_vm_hako_backend_frame_vm.sh` PASS（post-D6-min26 recheck）。
  - [x] 回帰確認: `./tools/selfhost/run.sh --gate --planner-required 1 --max-cases 5 --jobs 4` PASS（post-D6-min26: `5/5`, `stageb_total_secs=18`, `avg_case_secs=3.60`）。
  - [x] 検証: `cargo test -q subset_accepts_safepoint -- --nocapture` PASS（post-D6-min27）。
  - [x] 検証: `bash tools/smokes/v2/profiles/integration/apps/phase29z_vm_hako_s3_safepoint_parity_vm.sh` PASS（`rust-vm=42`, `hako-runner=42`）。
  - [x] 検証: `bash tools/smokes/v2/profiles/integration/apps/archive/phase29z_vm_hako_backend_frame_vm.sh` PASS（post-D6-min27 recheck）。
  - [x] 回帰確認: `./tools/selfhost/run.sh --gate --planner-required 1 --max-cases 5 --jobs 4` PASS（post-D6-min27）。
  - [x] 検証: `cargo test -q subset_accepts_keepalive -- --nocapture` PASS（post-D6-min28）。
  - [x] 検証: `bash tools/smokes/v2/profiles/integration/apps/phase29z_vm_hako_s3_keepalive_parity_vm.sh` PASS（`rust-vm=42`, `hako-runner=42`）。
  - [x] 検証: `bash tools/smokes/v2/profiles/integration/apps/archive/phase29z_vm_hako_backend_frame_vm.sh` PASS（post-D6-min28 recheck）。
  - [x] 回帰確認: `./tools/selfhost/run.sh --gate --planner-required 1 --max-cases 5 --jobs 4` PASS（post-D6-min28: `5/5`, `stageb_total_secs=22`, `avg_case_secs=4.40`）。
  - [x] 検証: `cargo test -q subset_accepts_release_strong -- --nocapture` PASS（post-D6-min29）。
  - [x] 検証: `bash tools/smokes/v2/profiles/integration/apps/phase29z_vm_hako_s3_release_strong_parity_vm.sh` PASS（`rust-vm=42`, `hako-runner=42`）。
  - [x] 検証: `bash tools/smokes/v2/profiles/integration/apps/archive/phase29z_vm_hako_backend_frame_vm.sh` PASS（post-D6-min29 recheck）。
  - [x] 回帰確認: `./tools/selfhost/run.sh --gate --planner-required 1 --max-cases 5 --jobs 4` PASS（post-D6-min29: `5/5`, `stageb_total_secs=20`, `avg_case_secs=4.00`）。
  - [x] 検証: `cargo test -q subset_accepts_debug -- --nocapture` PASS（post-D6-min30）。
  - [x] 検証: `bash tools/smokes/v2/profiles/integration/apps/phase29z_vm_hako_s4_debug_parity_vm.sh` PASS（`rust-vm=42`, `hako-runner=42`）。
  - [x] 検証: `bash tools/smokes/v2/profiles/integration/apps/archive/phase29z_vm_hako_backend_frame_vm.sh` PASS（post-D6-min30 recheck）。
  - [x] 回帰確認: `./tools/selfhost/run.sh --gate --planner-required 1 --max-cases 5 --jobs 4` PASS（post-D6-min30: `5/5`, `stageb_total_secs=24`, `avg_case_secs=4.80`）。
  - [x] 検証: `cargo test -q subset_accepts_debug_log -- --nocapture` PASS（post-D6-min31）。
  - [x] 検証: `bash tools/smokes/v2/profiles/integration/apps/phase29z_vm_hako_s4_debug_log_parity_vm.sh` PASS（`rust-vm=42`, `hako-runner=42`）。
  - [x] 検証: `bash tools/smokes/v2/profiles/integration/apps/archive/phase29z_vm_hako_backend_frame_vm.sh` PASS（post-D6-min31 recheck）。
  - [x] 回帰確認: `./tools/selfhost/run.sh --gate --planner-required 1 --max-cases 5 --jobs 4` PASS（post-D6-min31: `5/5`, `stageb_total_secs=11`, `avg_case_secs=2.20`）。
  - [x] 検証: `cargo test -q subset_accepts_select -- --nocapture` PASS（post-D6-min32）。
  - [x] 検証: `bash tools/smokes/v2/profiles/integration/apps/phase29z_vm_hako_s4_select_parity_vm.sh` PASS（`rust-vm=42`, `hako-runner=42`）。
  - [x] 検証: `bash tools/smokes/v2/profiles/integration/apps/archive/phase29z_vm_hako_backend_frame_vm.sh` PASS（post-D6-min32 recheck）。
  - [x] 回帰確認: `./tools/selfhost/run.sh --gate --planner-required 1 --max-cases 5 --jobs 4` PASS（post-D6-min32: `5/5`, `stageb_total_secs=20`, `avg_case_secs=4.00`）。
  - [x] 検証: `cargo test -q subset_accepts_barrier -- --nocapture` PASS（post-D6-min33）。
  - [x] 検証: `cargo check -q --bin hakorune` PASS（post-D6-min33）。
  - [x] 検証: `cargo build -q --release --bin hakorune` PASS（post-D6-min33）。
  - [x] 検証: `bash tools/smokes/v2/profiles/integration/apps/phase29z_vm_hako_s4_barrier_parity_vm.sh` PASS（`rust-vm=42`, `hako-runner=42`）。
  - [x] 検証: `bash tools/smokes/v2/profiles/integration/apps/phase29z_vm_hako_s4_bool_const_ws_parity_vm.sh` PASS（`rust-vm=42`, `hako-runner=42`）。
  - [x] 検証: `cargo test -q subset_ -- --nocapture` PASS（post-D6-clean-3; 16 tests）。
  - [x] 検証: `bash tools/smokes/v2/profiles/integration/apps/phase29z_vm_hako_s4_debug_parity_vm.sh` PASS（`rust-vm=42`, `hako-runner=42`）。
  - [x] 検証: `bash tools/smokes/v2/profiles/integration/apps/phase29z_vm_hako_s4_debug_log_parity_vm.sh` PASS（`rust-vm=42`, `hako-runner=42`）。
  - [x] 検証: `bash tools/smokes/v2/profiles/integration/apps/phase29z_vm_hako_s4_select_parity_vm.sh` PASS（`rust-vm=42`, `hako-runner=42`）。
  - [x] 検証: `bash tools/smokes/v2/profiles/integration/apps/phase29z_vm_hako_s4_barrier_parity_vm.sh` PASS（post-D6-clean-4; `rust-vm=42`, `hako-runner=42`）。
  - [x] 検証: `bash tools/smokes/v2/profiles/integration/apps/phase29z_vm_hako_s4_debug_parity_vm.sh` PASS（post-D6-clean-4; `rust-vm=42`, `hako-runner=42`）。
  - [x] 検証: `bash tools/smokes/v2/profiles/integration/apps/phase29z_vm_hako_s4_debug_log_parity_vm.sh` PASS（post-D6-clean-4; `rust-vm=42`, `hako-runner=42`）。
  - [x] 検証: `bash tools/smokes/v2/profiles/integration/apps/phase29z_vm_hako_s4_select_parity_vm.sh` PASS（post-D6-clean-4; `rust-vm=42`, `hako-runner=42`）。
  - [x] 検証: `bash tools/smokes/v2/profiles/integration/apps/phase29z_vm_hako_s4_bool_const_ws_parity_vm.sh` PASS（post-D6-clean-4; `rust-vm=42`, `hako-runner=42`）。
  - [x] 検証: `bash tools/smokes/v2/profiles/integration/apps/archive/phase29z_vm_hako_s4_scanner_depth_guard_parity_vm.sh` PASS（`rust-vm=42`, `hako-runner=1`, depth-guard contract）。
  - [x] 検証: `bash tools/smokes/v2/profiles/integration/apps/archive/phase29z_vm_hako_backend_frame_vm.sh` PASS（post-D6-clean-4 recheck; `phase=s4d`, `rc=1`）。
  - [x] 回帰確認: `./tools/selfhost/run.sh --gate --planner-required 1 --max-cases 5 --jobs 4` PASS（post-D6-clean-4: `5/5`, `stageb_total_secs=22`, `avg_case_secs=4.40`）。
  - [x] 検証: `cargo test -q vm_hako::tests::subset_accepts_load` PASS（post-D6-min34）。
  - [x] 検証: `cargo test -q vm_hako::tests::subset_rejects_load_missing_ptr` PASS（post-D6-min34）。
  - [x] 検証: `cargo build --release --bin hakorune` PASS（post-D6-min34）。
  - [x] 検証: `bash tools/smokes/v2/profiles/integration/apps/phase29z_vm_hako_s5_load_parity_vm.sh` PASS（`rust-vm=0`, `hako-runner=0`）。
  - [x] 検証: `bash tools/smokes/v2/profiles/integration/apps/archive/phase29z_vm_hako_backend_frame_vm.sh` PASS（post-D6-min34 recheck; `phase=s5a`, `rc=1`）。
  - [x] 回帰確認: `./tools/selfhost/run.sh --gate --planner-required 1 --max-cases 5 --jobs 4` PASS（post-D6-min34: `5/5`, `stageb_total_secs=20`, `avg_case_secs=4.00`）。
  - [x] 検証: `cargo test -q vm_hako::tests::subset_accepts_store` PASS（post-D6-min35）。
  - [x] 検証: `cargo test -q vm_hako::tests::subset_rejects_store_missing_value` PASS（post-D6-min35）。
  - [x] 検証: `cargo build --release --bin hakorune` PASS（post-D6-min35）。
  - [x] 検証: `bash tools/smokes/v2/profiles/integration/apps/phase29z_vm_hako_s5_store_parity_vm.sh` PASS（`rust-vm=42`, `hako-runner=42`）。
  - [x] 検証: `bash tools/smokes/v2/profiles/integration/apps/archive/phase29z_vm_hako_backend_frame_vm.sh` PASS（post-D6-min35 recheck; `phase=s5b`, `rc=1`）。
  - [x] 回帰確認: `./tools/selfhost/run.sh --gate --planner-required 1 --max-cases 5 --jobs 4` PASS（post-D6-min35: `5/5`, `stageb_total_secs=20`, `avg_case_secs=4.00`）。
  - [x] 検証: `cargo test -q vm_hako::tests::subset_accepts_phi` PASS（post-D6-min36）。
  - [x] 検証: `cargo test -q vm_hako::tests::subset_rejects_phi_missing_incoming` PASS（post-D6-min36）。
  - [x] 検証: `cargo build --release --bin hakorune` PASS（post-D6-min36）。
  - [x] 検証: `bash tools/smokes/v2/profiles/integration/apps/phase29z_vm_hako_s5_phi_parity_vm.sh` PASS（`rust-vm=42`, `hako-runner=42`）。
  - [x] 検証: `bash tools/smokes/v2/profiles/integration/apps/archive/phase29z_vm_hako_backend_frame_vm.sh` PASS（post-D6-min36 recheck; `phase=s5c`, `rc=1`）。
  - [x] 回帰確認: `./tools/selfhost/run.sh --gate --planner-required 1 --max-cases 5 --jobs 4` PASS（post-D6-min36: `5/5`, `stageb_total_secs=18`, `avg_case_secs=3.60`）。
  - [x] 検証: `cargo test -q vm_hako::tests::subset_accepts_typeop_check_integer -- --nocapture` PASS（post-D6-min37）。
  - [x] 検証: `cargo test -q vm_hako::tests::subset_rejects_typeop_missing_target_type -- --nocapture` PASS（post-D6-min37）。
  - [x] 検証: `cargo build --release --bin hakorune` PASS（post-D6-min37）。
  - [x] 検証: `bash tools/smokes/v2/profiles/integration/apps/phase29z_vm_hako_s5_typeop_parity_vm.sh` PASS（`rust-vm=42`, `hako-runner=42`）。
  - [x] 検証: `bash tools/smokes/v2/profiles/integration/apps/archive/phase29z_vm_hako_backend_frame_vm.sh` PASS（post-D6-min37 recheck; `phase=s5d`, `rc=1`）。
  - [x] 回帰確認: `./tools/selfhost/run.sh --gate --planner-required 1 --max-cases 5 --jobs 4` PASS（post-D6-min37: `5/5`, `stageb_total_secs=18`, `avg_case_secs=3.60`）。
  - [x] 検証: `cargo test -q vm_hako::tests::subset_accepts_weak_new_and_weak_load -- --nocapture` PASS（post-D6-min38）。
  - [x] 検証: `cargo test -q vm_hako::tests::subset_rejects_weak_new_missing_box_val -- --nocapture` PASS（post-D6-min38）。
  - [x] 検証: `cargo build --release --bin hakorune` PASS（post-D6-min38）。
  - [x] 検証: `bash tools/smokes/v2/profiles/integration/apps/phase29z_vm_hako_s5_weakref_parity_vm.sh` PASS（`rust-vm=42`, `hako-runner=42`）。
  - [x] 検証: `bash tools/smokes/v2/profiles/integration/apps/archive/phase29z_vm_hako_backend_frame_vm.sh` PASS（post-D6-min38 recheck; `phase=s5e`, `rc=1`）。
  - [x] 回帰確認: `./tools/selfhost/run.sh --gate --planner-required 1 --max-cases 5 --jobs 4` PASS（post-D6-min38: `5/5`, `stageb_total_secs=20`, `avg_case_secs=4.00`）。
  - [x] 検証: `cargo test -q vm_hako::tests:: -- --nocapture` PASS（post-D6-clean-7/8; 26 tests）。
  - [x] 検証: `bash tools/smokes/v2/profiles/integration/apps/phase29z_vm_hako_s5_weakref_parity_vm.sh` PASS（post-D6-clean-5/6 recheck; `rust-vm=42`, `hako-runner=42`）。
  - [x] 検証: `bash tools/smokes/v2/profiles/integration/apps/archive/phase29z_vm_hako_s5_weakref_non_box_reject_vm.sh` PASS（post-D6-clean-5; `rust-vm=1`, `hako-runner=1`）。
  - [x] 検証: `bash tools/smokes/v2/profiles/integration/apps/archive/phase29z_vm_hako_backend_frame_vm.sh` PASS（post-D6-clean-8 recheck; `phase=s5e`, `rc=1`）。
  - [x] 回帰確認: `./tools/selfhost/run.sh --gate --planner-required 1 --max-cases 5 --jobs 4` PASS（post-D6-clean-8: `5/5`, `stageb_total_secs=20`, `avg_case_secs=4.00`）。
  - [x] 検証: `cargo test -q vm_hako::tests:: -- --nocapture` PASS（post-D6-min39; 28 tests）。
  - [x] 検証: `cargo build --release --bin hakorune` PASS（post-D6-min39）。
  - [x] 検証: `bash tools/smokes/v2/profiles/integration/apps/phase29z_vm_hako_s5_refnew_parity_vm.sh` PASS（`rust-vm=42`, `hako-runner=42`）。
  - [x] 検証: `bash tools/smokes/v2/profiles/integration/apps/phase29z_vm_hako_s5_weakref_parity_vm.sh` PASS（post-D6-min39 recheck; `rust-vm=42`, `hako-runner=42`）。
  - [x] 検証: `bash tools/smokes/v2/profiles/integration/apps/archive/phase29z_vm_hako_s5_weakref_non_box_reject_vm.sh` PASS（post-D6-min39 recheck; `rust-vm=1`, `hako-runner=1`）。
  - [x] 検証: `bash tools/smokes/v2/profiles/integration/apps/archive/phase29z_vm_hako_backend_frame_vm.sh` PASS（post-D6-min39 recheck; `phase=s5f`, `rc=1`）。
  - [x] 回帰確認: `./tools/selfhost/run.sh --gate --planner-required 1 --max-cases 5 --jobs 4` PASS（post-D6-min39: `5/5`, `stageb_total_secs=18`, `avg_case_secs=3.80`）。
  - [x] 検証: `cargo test -q vm_hako::tests:: -- --nocapture` PASS（post-D6-min40; 30 tests）。
  - [x] 検証: `cargo build --release --bin hakorune` PASS（post-D6-min40）。
  - [x] 検証: `bash tools/smokes/v2/profiles/integration/apps/phase29z_vm_hako_s5_future_new_parity_vm.sh` PASS（`rust-vm=42`, `hako-runner=42`）。
  - [x] 検証: `bash tools/smokes/v2/profiles/integration/apps/phase29z_vm_hako_s5_refnew_parity_vm.sh` PASS（post-D6-min40 recheck; `rust-vm=42`, `hako-runner=42`）。
  - [x] 検証: `bash tools/smokes/v2/profiles/integration/apps/archive/phase29z_vm_hako_backend_frame_vm.sh` PASS（post-D6-min40 recheck; `phase=s5g`, `rc=1`）。
  - [x] 回帰確認: `./tools/selfhost/run.sh --gate --planner-required 1 --max-cases 5 --jobs 4` PASS（post-D6-min40: `5/5`, `stageb_total_secs=21`, `avg_case_secs=4.20`）。
  - [x] 検証: `cargo test -q vm_hako::tests:: -- --nocapture` PASS（post-D6-min41; 32 tests）。
  - [x] 検証: `cargo build --release --bin hakorune` PASS（post-D6-min41）。
  - [x] 検証: `bash tools/smokes/v2/profiles/integration/apps/phase29z_vm_hako_s5_future_set_parity_vm.sh` PASS（`rust-vm=42`, `hako-runner=42`）。
  - [x] 検証: `bash tools/smokes/v2/profiles/integration/apps/phase29z_vm_hako_s5_future_new_parity_vm.sh` PASS（post-D6-min41 recheck; `rust-vm=42`, `hako-runner=42`）。
  - [x] 検証: `bash tools/smokes/v2/profiles/integration/apps/archive/phase29z_vm_hako_backend_frame_vm.sh` PASS（post-D6-min41 recheck; `phase=s5h`, `rc=1`）。
  - [x] 回帰確認: `./tools/selfhost/run.sh --gate --planner-required 1 --max-cases 5 --jobs 4` PASS（post-D6-min41: `5/5`, `stageb_total_secs=21`, `avg_case_secs=4.20`）。
  - [x] 検証: `cargo test -q vm_hako::tests:: -- --nocapture` PASS（post-D6-min42; 34 tests）。
  - [x] 検証: `cargo build --release --bin hakorune` PASS（post-D6-min42）。
  - [x] 検証: `bash tools/smokes/v2/profiles/integration/apps/phase29z_vm_hako_s5_await_parity_vm.sh` PASS（`rust-vm=42`, `hako-runner=42`）。
  - [x] 検証: `bash tools/smokes/v2/profiles/integration/apps/phase29z_vm_hako_s5_future_set_parity_vm.sh` PASS（post-D6-min42 recheck; `rust-vm=42`, `hako-runner=42`）。
  - [x] 検証: `bash tools/smokes/v2/profiles/integration/apps/archive/phase29z_vm_hako_backend_frame_vm.sh` PASS（post-D6-min42 recheck; `phase=s5i`, `rc=1`）。
  - [x] 回帰確認: `./tools/selfhost/run.sh --gate --planner-required 1 --max-cases 5 --jobs 4` PASS（post-D6-min42: `5/5`, `stageb_total_secs=18`, `avg_case_secs=3.60`）。
  - [x] 検証: `bash tools/smokes/v2/profiles/integration/apps/phase29z_vm_hako_s5_await_non_future_reject_vm.sh` PASS（post-D6-clean-9; `rust-vm=1`, `hako-runner=1`, `await-non-future`）。
  - [x] 検証: `bash tools/smokes/v2/profiles/integration/apps/phase29z_vm_hako_s5_await_parity_vm.sh` PASS（post-D6-clean-9 recheck; `rust-vm=42`, `hako-runner=42`）。
  - [x] 検証: `bash tools/smokes/v2/profiles/integration/apps/archive/phase29z_vm_hako_backend_frame_vm.sh` PASS（post-D6-clean-9 recheck; `phase=s5i`, `rc=1`）。
  - [x] 回帰確認: `./tools/selfhost/run.sh --gate --planner-required 1 --max-cases 5 --jobs 4` PASS（post-D6-clean-9: `5/5`, `stageb_total_secs=22`, `avg_case_secs=4.40`）。
  - [x] 検証: `bash tools/smokes/v2/profiles/integration/apps/phase29z_vm_hako_s5_newclosure_probe_vm.sh` PASS（post-D6-clean-10; `rust-vm=1`, `hako-runner=1`, `new_closure` probe contract）。
  - [x] 検証: `bash tools/smokes/v2/profiles/integration/apps/phase29z_vm_hako_s5_await_parity_vm.sh` PASS（post-D6-clean-10 recheck; `rust-vm=42`, `hako-runner=42`）。
  - [x] 検証: `bash tools/smokes/v2/profiles/integration/apps/archive/phase29z_vm_hako_backend_frame_vm.sh` PASS（post-D6-clean-10 recheck; `phase=s5i`, `rc=1`）。
  - [x] 回帰確認: `./tools/selfhost/run.sh --gate --planner-required 1 --max-cases 5 --jobs 4` PASS（post-D6-clean-10: `5/5`, `stageb_total_secs=20`, `avg_case_secs=4.00`）。
  - [x] 検証: `cargo test -q mir_json_allowlist_accepts_new_closure` PASS（post-D6-clean-12）。
  - [x] 検証: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS（post-D6-clean-12）。
  - [x] 検証: `bash tools/smokes/v2/profiles/integration/apps/phase29x_llvm_only_daily_gate.sh` PASS（post-D6-clean-12, `nyash 1.0 features:llvm`）。
  - [x] 検証: `cargo test -q mcl0_is_noop_for_legacy_and_unified_callsites -- --nocapture` PASS（post-MCL-0）。
  - [x] 検証: `cargo test -q instruction_diet_ledger_counts_match_docs_ssot -- --nocapture` PASS（post-MCL-0）。
  - [x] 検証: `cargo test -q mir14_shape_is_fixed --test mir_instruction_set_sync -- --nocapture` PASS（post-MCL-0）。
  - [x] 検証: `cargo check -q --bin hakorune` PASS（post-MCL-0）。
  - [x] 回帰確認: `./tools/selfhost/run.sh --gate --planner-required 1 --max-cases 5 --jobs 4` PASS（post-MCL-0: `5/5`, `stageb_total_secs=20`, `avg_case_secs=4.00`）。
  - [x] 検証: `cargo test -q mcl1_rewrites_boxcall_to_method_callee -- --nocapture` PASS（post-MCL-1）。
  - [x] 検証: `cargo test -q mcl1_preserves_boxcall_used_values_contract -- --nocapture` PASS（post-MCL-1）。
  - [x] 検証: `cargo test -q instruction_diet_ledger_counts_match_docs_ssot -- --nocapture` PASS（post-MCL-1）。
  - [x] 検証: `cargo test -q mir14_shape_is_fixed --test mir_instruction_set_sync -- --nocapture` PASS（post-MCL-1）。
  - [x] 検証: `cargo check -q --bin hakorune` PASS（post-MCL-1）。
  - [x] 回帰確認: `./tools/selfhost/run.sh --gate --planner-required 1 --max-cases 5 --jobs 4` PASS（post-MCL-1: `5/5`, `stageb_total_secs=21`, `avg_case_secs=4.20`）。
  - [x] 検証: `cargo test -q mcl2_rewrites_externcall_to_extern_callee -- --nocapture` PASS（post-MCL-2）。
  - [x] 検証: `cargo test -q mcl2_preserves_externcall_used_values_contract -- --nocapture` PASS（post-MCL-2）。
  - [x] 検証: `cargo test -q instruction_diet_ledger_counts_match_docs_ssot -- --nocapture` PASS（post-MCL-2）。
  - [x] 検証: `cargo test -q mir14_shape_is_fixed --test mir_instruction_set_sync -- --nocapture` PASS（post-MCL-2）。
  - [x] 検証: `cargo check -q --bin hakorune` PASS（post-MCL-2）。
  - [x] 回帰確認: `./tools/selfhost/run.sh --gate --planner-required 1 --max-cases 5 --jobs 4` PASS（post-MCL-2: `5/5`, `stageb_total_secs=21`, `avg_case_secs=4.20`）。
  - [x] 検証: `cargo test -q mir_json_allowlist_rejects_legacy_callsite_shapes -- --nocapture` PASS（post-MCL-3）。
  - [x] 検証: `cargo test -q vm_allowlist_rejects_call_without_callee -- --nocapture` PASS（post-MCL-3）。
  - [x] 検証: `cargo test -q vm_preflight_rejects_legacy_call_without_callee_under_strict_gate -- --nocapture` PASS（post-MCL-3）。
  - [x] 検証: `cargo test -q instruction_diet_ledger_counts_match_docs_ssot -- --nocapture` PASS（post-MCL-3）。
  - [x] 検証: `cargo test -q mir14_shape_is_fixed --test mir_instruction_set_sync -- --nocapture` PASS（post-MCL-3）。
  - [x] 検証: `cargo check -q --bin hakorune` PASS（post-MCL-3）。
  - [x] 回帰確認: `./tools/selfhost/run.sh --gate --planner-required 1 --max-cases 5 --jobs 4` PASS（post-MCL-3: `5/5`, `stageb_total_secs=19`, `avg_case_secs=4.00`）。
  - [x] 検証: `cargo test -q mcl4_canonicalize_removes_legacy_callsite_instructions -- --nocapture` PASS（post-MCL-4）。
  - [x] 検証: `cargo test -q instruction_diet_ledger_counts_match_docs_ssot -- --nocapture` PASS（post-MCL-4）。
  - [x] 検証: `cargo test -q mir14_shape_is_fixed --test mir_instruction_set_sync -- --nocapture` PASS（post-MCL-4）。
  - [x] 検証: `cargo check -q --bin hakorune` PASS（post-MCL-4）。
  - [x] 回帰確認: `./tools/selfhost/run.sh --gate --planner-required 1 --max-cases 5 --jobs 4` PASS（post-MCL-4: `5/5`, `stageb_total_secs=18`, `avg_case_secs=3.60`）。

## D6 Lane Scope Snapshot (granular, 2026-02-12)

- 語彙母数（MIR kept tags）: 28（`src/mir/contracts/backend_core_ops.rs` の `MIR_INSTRUCTION_KEPT_TAGS`）
- `vm-hako` S5l まで実装済み（subset-check受理）:
  - `Const(i64/bool)`, `BinOp(add/sub/mul/div/mod)`, `Compare(eq/lt)`, `Branch/Jump`, `UnaryOp(neg/not)`, `Copy`, `Return`
  - `Call(callee=Global|Method|Extern)`（canonical callsite）
  - `NewBox(ArrayBox/StringBox/Main)`（限定）
  - `Safepoint`, `KeepAlive`, `ReleaseStrong`, `Debug`, `Select`, `Barrier`
  - `Load`, `Store`, `Phi`, `TypeOp`, `WeakRef`, `RefNew`
  - `FutureNew`, `FutureSet`, `Await`
  - `ArrayGet`, `ArraySet`（S5k/S5l, register-slot shim）
  - legacy JSON compatibility（語彙外）: `boxcall` / `externcall` / `mir_call(print)` は canonicalization/bridge のために subset で観測しうる
- retired legacy status:
  - `BoxCall/ExternCall/DebugLog/Nop` は `MirInstruction` enum から retire 済み（RCL-3 / RDN-0）
  - `vm-hako` subset は legacy `nop/debug_log` を unknown-op fail-fast で reject（RDN-1）
- 概算残（roadmap queue基準）: 約7（NCL lane を含む cleanup/統合タスク）
- 補足:
  - 上記は「MIR命令タグ + vm-hako互換入口」の整理。legacy JSON op は互換入口であり kept 語彙には数えない。
  - BoxCount は 1タスク1語彙で進め、必要な BoxShape clean-up は別タスクとして挟む（受理語彙は増やさない）。

### D6 Queue (active order)

1. [done] S4d: `barrier` を JSON route parity（rust-vm core route vs hako-runner route）で契約化し、`mir_json_v0` loader / subset / `.hako runner` の受理を追加する
2. [done] S4-clean-2: `mir_vm_s0` scanner 再帰を深さ制御つきに整理し、深い payload は depth-guard fail-fast 契約で固定する（BoxShape; 受理語彙は増やさない）
3. [done] S4-clean-3: `src/runner/modes/vm_hako.rs` の `debug/select` 形検証を小helperへ分離し、subset-check責務を見通しよく固定する（BoxShape; no new vocabulary）
4. [done] S4-clean-4: S4系 parity fixture を語彙最小へ再点検し、非目標 op 混入の fail-fast 契約（`VM_HAKO_PARITY_DENY_OPS`）を追加する（BoxShape; no new vocabulary）
5. [done] S5a: `load` を JSON route parity（rust-vm core route vs hako-runner route）で契約化し、`mir_json_v0` loader / subset / `.hako runner` の受理を追加する（BoxCount; 1 vocabulary）
6. [done] S5b: `store` を JSON route parity（rust-vm core route vs hako-runner route）で契約化し、`mir_json_v0` loader / subset / `.hako runner` の受理を追加する（BoxCount; 1 vocabulary）
7. [done] S5c: `phi` を JSON route parity（rust-vm core route vs hako-runner route）で契約化し、`mir_json_v0` loader / subset / `.hako runner` の受理を追加する（BoxCount; 1 vocabulary）
8. [done] S5d: `typeop` を JSON route parity（rust-vm core route vs hako-runner route）で契約化し、`mir_json_v0` loader / subset / `.hako runner` の受理を追加する（BoxCount; 1 vocabulary）
9. [done] S5e: `weakref` を JSON route parity（rust-vm core route vs hako-runner route）で契約化し、`mir_json_v0` loader / subset / `.hako runner` の受理を追加する（BoxCount; 1 vocabulary）
10. [done] S5e-clean-5: `weak_new(non-box)` reject 契約を追加し、`.hako` runner は `[vm-hako/contract][weak_new-non-box]` で fail-fast 固定する（BoxShape; no new vocabulary）
11. [done] S5e-clean-6: `mir_vm_s0` `_exec_op` を data/call の内部メソッドへ分割し、責務を整理する（BoxShape; no new vocabulary）
12. [done] S5e-clean-7: Rust subset validator を共通 helper（`ensure_u64_fields`）へ集約する（BoxShape; no new vocabulary）
13. [done] S5e-clean-8: alias 正規化（subset + payload loader）を `normalize_instruction_aliases` SSOT に統一する（BoxShape; no new vocabulary）
14. [done] S5f: `ref_new` を JSON route parity（rust-vm core route vs hako-runner route）で契約化し、`mir_json_v0` loader / subset / `.hako runner` / Rust VM interpreter の受理を追加する（BoxCount; 1 vocabulary）
15. [done] S5g: `future_new` を JSON route parity（rust-vm core route vs hako-runner route）で契約化し、`mir_json_v0` loader / subset / `.hako runner` の受理を追加する（BoxCount; 1 vocabulary）
16. [done] S5h: `future_set` を JSON route parity（rust-vm core route vs hako-runner route）で契約化し、`mir_json_v0` loader / subset / `.hako runner` の受理を追加する（BoxCount; 1 vocabulary）
17. [done] S5i: `await` を JSON route parity（rust-vm core route vs hako-runner route）で契約化し、`mir_json_v0` loader / subset / `.hako runner` の受理を追加する（BoxCount; 1 vocabulary）
18. [done] S5i-clean-9: `await(non-future)` を cross-route reject smoke（rust-vm/hako-runner）で固定し、`[vm-hako/contract][await-non-future]` fail-fast 契約を pin する（BoxShape; no new vocabulary）
19. [done] S5j-probe: `NewClosure` の rust-vm / hako-runner 現状挙動を probe し、受理/拒否の契約（stable tag + smoke）を先に固定する（BoxShape; no new vocabulary）
20. [done] MCL-0: canonicalization pass の入口追加（挙動不変; BoxShape lane）
21. [done] MCL-1: `BoxCall -> Call(callee=Method)` 変換（BoxShape lane; `BoxCall -> Call(callee=Method)`）
22. [done] MCL-2: `ExternCall -> Call(callee=Extern)` 変換（BoxShape lane; `ExternCall -> Call(callee=Extern)`）
23. [done] MCL-3: backend 入口 fail-fast（legacy call-site 残存 reject; BoxShape lane）
24. [done] MCL-4: docs/test 同期（ledger 契約維持; BoxShape lane）
25. [done] D6-min43: `ArrayGet` を `vm-hako` subset/runner/loader に 1語彙追加し、JSON route parity を固定（BoxCount lane）
26. [done] D6-min44: `ArraySet` を `vm-hako` subset/runner/loader に 1語彙追加し、JSON route parity を固定（BoxCount lane）
27. [done] RCL-0: post-canonical retire lane の実行順と fail-fast 契約を SSOT 化（docs-only lane）
28. [done] RCL-1: `.hako` mirbuilder（`lang/src/mir/builder/**`）の emit を `Call(callee=Method/Extern)` へ統一し、legacy `externcall/boxcall` 新規発行を停止（BoxCount lane）
29. [done] RCL-2: emit-side fail-fast（strict/dev の stage1 selfhost MIR 受け口で legacy emit を reject、安定タグ固定）
30. [done] RCL-3: `BoxCall/ExternCall` enum retire（parser/mirbuilder/runtime 全移行後）
31. [done] RCL-3-min1: `src/mir/ssot/extern_call.rs` を canonical 化し、新規 extern call emit を `Call(callee=Extern)` へ統一（legacy `ExternCall` 新規構築停止）
32. [done] RCL-3-min2: builder emit 境界で `BoxCall` を canonical `Call(callee=Method)` へ統一し、legacy `BoxCall` 新規構築を停止（`src/mir/ssot/method_call.rs` 追加、`module_lifecycle` の birth 観測同期）
33. [done] RCL-3-min3: enum retire（`MirInstruction::BoxCall/ExternCall` 削除 + backend/contracts/test 同期）
34. [done] RDN-0: `MirInstruction::DebugLog/Nop` enum retire（`DebugLog -> Debug` canonicalize、`nop` lower-away、backend/contracts/test 同期）
35. [done] RDN-1: `vm_hako` subset から legacy `nop/debug_log` 受理を撤去し、unknown-op fail-fast へ統一（diet ledger `kept=28/removed=16` に同期）
36. [done] NCL-0: `Call(callee=Closure)` を `NewClosure` へ canonicalize し、backend contracts で `call-closure-not-canonical` fail-fast を固定（BoxShape; no new vocabulary）
37. [done] NCL-1: `NewClosure` body を module metadata（`closure_bodies`）へ外出しし、`body_id` 参照の薄い命令形へ canonicalize（BoxShape; no new vocabulary）
38. [done] NCL-2: `Call(callee=Closure...)` の shape 判定を SSOT 化し、canonical 形（`dst=Some + args=[]`）のみ `NewClosure` へ正規化、非canonical 形は shape-specific fail-fast 理由へ固定（BoxShape; no new vocabulary）
39. [done] D6-clean-12: `NewClosure` emit canonicalization + route drift guard（`NYASH_VM_HAKO_PREFER_STRICT_DEV=0` pin）を適用し、phase29bq/compiler gate と vm-hako probe の実行レーンを固定（BoxShape; no new vocabulary）

### Post-canonical retire queue (queued; not active yet)

- activation gate:
  - 開始条件は充足（`D6-min44` 完了 + runtime lane 日次 gate 緑）。`RCL-3` は完了（min1/min2/min3）。
- fixed migration order:
  - 1) Rust 側 canonicalize で legacy 吸収（done: MCL-0..5）
  - 2) `.hako` mirbuilder の新規出力を canonical call-site へ移行
  - 3) 旧命令（`BoxCall`/`ExternCall`）を retire（enum/loader/backend）
- queue:
  - [done] RCL-0 (docs-only): retire 実行順と fail-fast 契約を SSOT 化（`mir-callsite-retire-lane-ssot.md`）
  - [done] RCL-1 (BoxCount): `.hako` mirbuilder の emit を `Call(callee=Method/Extern)` へ統一
  - [done] RCL-2 (BoxShape): strict/dev の stage1 selfhost MIR 受け口（`src/runner/modes/common_util/selfhost/json.rs`）で legacy emit-side reject（`[freeze:contract][callsite-retire:legacy-{boxcall|externcall}]`）
  - [done] RCL-3 (BoxShape): `BoxCall/ExternCall` enum retire（parser/mirbuilder/runtime 全移行後）
    - [done] min1: `extern_call` SSOT helper を canonical 化（new emit は `Call(callee=Extern)`）
    - [done] min2: `BoxCall` 生成経路を canonical 化（new emit は `Call(callee=Method)`）
    - [done] min3: enum retire（`MirInstruction::BoxCall/ExternCall` 削除 + backend/contracts/test 同期）
  - [done] RDN-0 (separate lane): `DebugLog/Nop` retire は callsite lane と分離（`MirInstruction` enum retire 済み）
  - [done] RDN-1 (separate lane): `vm_hako` subset から legacy `nop/debug_log` を撤去し、unknown-op fail-fast 契約へ統一
- NewClosure clean path (2-step):
  - [done] NCL-0: `NewClosure` は canonical 維持（`Call(callee=Closure)` は pass で `NewClosure` へ正規化し、境界で `call-closure-not-canonical` reject）
  - [done] NCL-1: closure body を外出し（`body_id -> module.metadata.closure_bodies`）して `NewClosure` を薄い命令へ縮退
  - [done] NCL-2: shape 契約を固定（canonical 形のみ rewrite、`call-closure-missing-dst` / `call-closure-runtime-args` を fail-fast 理由として pin）

### D6 Exit Criteria

- D6 完了条件:
  - S0-S5l の受理形が parity / reject 契約で固定されていること
  - post-canonical retire（RCL-3, RDN-0, RDN-1）が反映され、legacy 命令は enum kept 語彙に残っていないこと
- D6 後の入口:
  - selfhost checklist の failure-driven ループへ戻る（`29bq-90` PROBE→FIX→PROMOTE）

## Guardrails

- 1ブロッカー = 1受理形 = fixture+gate = 1コミット。
- BoxCount（受理追加）と BoxShape（責務整理）を混ぜない。
- fallback を増やさない。未対応形は fail-fast で原因側に寄せる。

## Next single task (recommended)

- selfhost checklist の failure-driven ループを継続し、freeze/reject 発生時のみ 1ブロッカーずつ PROBE→FIX→PROMOTE する。
