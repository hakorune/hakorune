---
Status: Active
Decision: provisional
Date: 2026-02-19
Scope: lane A（Compiler Meaning）の JoinIR 移植を 1 blocker = 1 fixture = 1 smoke = 1 commit で進めるための固定順序。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/design/de-rust-lane-map-ssot.md
  - docs/development/current/main/design/de-rust-compiler-thin-rust-roadmap-ssot.md
  - docs/development/current/main/design/joinir-planner-required-gates-ssot.md
  - docs/development/current/main/design/compiler-expressivity-first-policy.md
  - docs/development/current/main/design/ai-handoff-and-debug-contract.md
---

# JoinIR Port Task Pack (lane A SSOT)

## Purpose

- JoinIR 移植を場当たりで進めず、固定順序で小さく前進する。
- lane B/C（pipeline/runtime）との混線を避け、lane A（意味決定）だけに責務を閉じる。

## Entry Rule

- 既定は monitor-only（blocker=`none`）。
- 次のいずれかで lane A blocker 化し、このタスクパックを起動する。
  1. `phase29bq_fast_gate_vm.sh --only bq` が FAIL
  2. planner-required gate が FAIL
  3. `[joinir/freeze]` / `[plan/reject]` 系タグが mainline で再発
- active override:
  - 現在値は `CURRENT_TASK.md` の compiler lane block（`phase-29bq / <task|none>`）を唯一入力にする。
  - mirror 同期は `bash tools/selfhost/sync_lane_a_state.sh` -> `bash tools/checks/phase29bq_joinir_port_sync_guard.sh` を固定経路とする。

## Fixed Order (JIR-PORT-00..07)

### JIR-PORT-00: Boundary Lock (docs-first)

- 目的:
  - Rust JoinIR と `.hako` JoinIR の責務境界を先に固定し、同時修正を禁止する。
- 実施:
  - 入力・出力契約、fail-fast タグ、撤去条件を docs に先行記述。
  - fallback/silent pass 禁止を明記。
- 受け入れ:
  - SSOT 参照だけで「どこまでが lane A か」を説明できること。

#### JIR-PORT-00 Boundary Contract (fixed)

- lane scope:
  1. lane A（JoinIR/Planner/CorePlan の意味決定）だけを変更対象にする。
  2. lane B（parser/mirbuilder/stage1 導線）と lane C（runtime/vm capability）は同コミットで触らない。
- route ownership:
  1. Rust JoinIR route は parity baseline（比較対象）として扱う。
  2. `.hako` JoinIR route は移植対象として扱う。
  3. `.hako` route 未対応形は fallback せず fail-fast する。
- fail-fast tags（stable）:
  1. planner reject: `[plan/reject]` / `[plan/reject_detail]`
  2. planner invariant freeze: `[plan/freeze:*]`
  3. JoinIR lowering freeze: `[joinir/freeze]` または `[joinir/<phase>/<pattern>/contract]`
- prohibited:
  1. silent pass / implicit fallback / lane 混在パッチを禁止する。
  2. `1 blocker = 1 fixture = 1 smoke = 1 commit` を破る変更を禁止する。
- exit criteria（JIR-PORT-00 完了条件）:
  1. `CURRENT_TASK.md` と `10-Now.md` に lane A active blocker を同期済み。
  2. next task が `JIR-PORT-01 (Parity Probe)` に固定されている。

### JIR-PORT-01: Parity Probe

- 目的:
  - 同一入力で Rust route / `.hako` route の差分を観測固定する。
- 実施:
  - 最小 fixture 1件 + parity smoke 1本を追加。
  - 出力比較は canonical 化後に実施（順序揺れは比較対象から除外）。
- 受け入れ:
  - PASS時は parity 緑、FAIL時は差分位置が stable tag で特定できること。
- fixed fixture/smoke:
  - fixture: `apps/tests/phase29bq_joinir_port01_if_merge_min.hako`
  - smoke: `tools/smokes/v2/profiles/integration/joinir/phase29bq_joinir_port01_parity_probe_vm.sh`

### JIR-PORT-02: if/merge Minimal Port

- 目的:
  - `if + merge` の最小受理形を `.hako` へ移植。
- 実施:
  - 1受理形だけ追加（BoxCount）。
  - 未対応形は strict/dev で freeze を返す。
- 受け入れ:
  - 対象 fixture が `.hako` route で緑、非対象は fail-fast 維持。

### JIR-PORT-03: loop Minimal Port

- 目的:
  - `loop` 系を 1形ずつ移植（continue/break を分離）。
- 実施:
  - 1コミットで 1形のみ追加。
  - 早期終了/条件付き書き換え等の境界は reject/freeze で明示固定。
- 受け入れ:
  - 追加形の fixture/smoke 緑、既知非対応はタグ付きで落ちること。
- fixed fixture/smoke:
  - fixture: `apps/tests/phase29bq_joinir_port03_loop_local_return_var_min.hako`
  - smoke: `tools/smokes/v2/profiles/integration/joinir/phase29bq_joinir_port03_loop_minimal_vm.sh`

### JIR-PORT-04: PHI / Exit Invariant Lock

- 目的:
  - PHI/exit 系の不変条件を parity + verifier で固定。
- 実施:
  - PHI/exit 向け fixture を最小追加し、gate へ昇格。
  - Rust route は compat-only へ縮退する条件を明記。
- 受け入れ:
  - verifier と parity の両方で green 固定。
- fixed fixture/smoke:
  - fixture: `apps/tests/phase29bq_joinir_port04_phi_exit_invariant_min.hako`
  - smoke: `tools/smokes/v2/profiles/integration/joinir/phase29bq_joinir_port04_phi_exit_invariant_lock_vm.sh`

### JIR-PORT-05: Promotion Boundary Lock

- 目的:
  - `JIR-PORT-04` の固定結果を lane A の常設運用境界へ昇格する。
- 実施:
  - `CURRENT_TASK.md` / `10-Now.md` / lane map / task pack の active/next を同期。
  - `phase29bq_fast_gate_vm.sh --only bq` の日次運用に `port04` lock が含まれる状態を固定。
- 受け入れ:
  - sync guard と fast gate（bq）がともに green 固定。

### JIR-PORT-06: Monitor-only Boundary Lock

- 目的:
  - lane A の blocker が `none` の平常運用で、境界契約が崩れないことを固定する。
- 実施:
  - lane A の状態を monitor-only へ戻し、復帰条件（fast gate / planner-required fail 時のみ再起動）を明示する。
  - `CURRENT_TASK.md` / `10-Now.md` / lane map / task pack の状態を `done=06, active=none, next=none` へ同期する。
- 受け入れ:
  - sync guard が green かつ lane A が monitor-only 契約（blocker=`none`）で運用されること。

### JIR-PORT-07: Expression Parity Seed (unary+compare+logic)

- 目的:
  - loop/if の移植後に残る式レイヤ（unary/compare/logic）の最小 parity seed を lane A で固定する。
- 実施:
  - `1 blocker = 1 fixture = 1 smoke = 1 commit` で、最小1件の expression seed fixture を追加する。
  - fail-fast で未対応形を明示し、fallback を追加しない。
  - `CURRENT_TASK.md` / `10-Now.md` / lane map / task pack を `done=06, active=07, next=none` へ同期する。
- 受け入れ:
  - seed fixture が `.hako` route で green、非対応は stable tag で freeze/reject になること。
  - sync guard と daily gate が green を維持すること。

## Operation Rules (must keep)

- `1 blocker = 1 fixture = 1 smoke = 1 commit`
- BoxCount（受理追加）と BoxShape（構造整理）を同コミットで混ぜない。
- gate FAIL 状態で cases.tsv を増やさない（先に復旧）。
- debug tag は 1行・stable・default OFF。
- lane A state 更新は `CURRENT_TASK.md` を唯一入力にし、`bash tools/selfhost/sync_lane_a_state.sh` -> `bash tools/checks/phase29bq_joinir_port_sync_guard.sh` の順で mirror 同期を固定する。

## Daily / Milestone Commands

- daily:
  - `bash tools/selfhost/run_lane_a_daily.sh`
- milestone:
  - `./tools/selfhost/run.sh --gate --planner-required 1 --max-cases 5 --jobs 4`

## Current State (2026-02-19)

- status mirror SSOT: `CURRENT_TASK.md`（この文書は lane A 専用 mirror）
- JIR-PORT-00: done（Boundary Lock, docs-first）
- JIR-PORT-01: done（Parity Probe）
- JIR-PORT-02: done（if/merge minimal port）
- JIR-PORT-03: done（loop minimal port）
- JIR-PORT-04: done（PHI / Exit invariant lock）
- JIR-PORT-05: done（promotion boundary lock）
- JIR-PORT-06: done（monitor-only boundary lock）
- JIR-PORT-07: done（expression parity seed lock: unary+compare+logic）
- lane A blocker: `none`（monitor-only）
- next: `none`（tail active）
