---
Status: SSOT
Decision: provisional
Date: 2026-03-02
Scope: JoinIR 拡張を Rust reference route と `.hako` mainline route の同一契約で進める。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/design/joinir-design-map.md
  - docs/development/current/main/design/joinir-port-task-pack-ssot.md
  - docs/development/current/main/design/joinir-planner-required-gates-ssot.md
  - docs/development/current/main/design/build-lane-separation-ssot.md
  - docs/development/current/main/design/ai-handoff-and-debug-contract.md
---

# JoinIR Extension Dual-Route Contract (SSOT)

## Purpose

- JoinIR 拡張を docs-first + gate-first で進め、Rust/`.hako` の意味差分を早期検出する。
- `.hako` mainline を実装主導にし、Rust は reference/host の最小差分に限定する。
- unsupported shape の silent fallback を禁止し、必ず fail-fast で停止する。

## Route Terms

- `route-hako-mainline`:
  - `.hako` JoinIR 実装を使う本流。
  - `NYASH_VM_USE_FALLBACK=0` を必須にする。
- `route-rust-reference`:
  - Rust JoinIR 実装。parity 比較対象と切り分け用途。
- `route-compat-fallback`:
  - 禁止。hook/shape 未対応は fail-fast に固定する。

## Required Route Report (stable)

- JoinIR 拡張時のログは dev-only（default OFF）で最低 1 行を出す。
  - `[route/joinir] vm=<rust|hako> kernel=<rust|hako> joinir=<rust|hako> fallback=<0|1> emit=<direct|stageb-delegate>`
- gate は `fallback=1` を検出したら FAIL とする。
- stable tag の運用は `ai-handoff-and-debug-contract.md` を正本にする。

## Fixed Order (JIR-EXT-00..05)

### JIR-EXT-00: Contract Update (docs-first)

- 追加する受理形 1 件を文書化する（shape / input contract / fail-fast tag / 非対応境界）。
- 同時に fixture 名と smoke 名を固定する。
- 受理形追加（BoxCount）と構造整理（BoxShape）を同コミットで混ぜない。

#### Active Seed (shape-01)

- shape id: `JIR-EXT-SHAPE-01`
- target fixture:
  - `apps/tests/phase29bq_selfhost_blocker_phi_injector_collect_phi_vars_nested_loop_no_exit_var_step_min.hako`
- current boundary (GREEN for planner path):
  - rust reference route: `18` and `RC: 0`
  - hako mainline route: `18` and `RC: 0`（lane tag=`vm-hako`）
- scope:
  - Planner-required で `Pattern1` を通過した後、`main` の loop-cond lowering で `None -> freeze` になる shape を受理対象にする。
- non-goal:
  - fallback 追加で通さない（`NYASH_VM_USE_FALLBACK=0` 固定）。

### JIR-EXT-01: Gate Seed / Lock

- 新 fixture を gate に追加し、先に RED を確認した後に GREEN lock へ昇格する。
- 必須 gate:
  - `tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`
  - planner-required gate（`joinir-planner-required-gates-ssot.md` の SSOT entry）
- parity 比較が必要な場合は rust/hako の同入力差分を固定する。

#### Active Gate Lock (ext-red mode)

- command:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only ext-red`
- implementation:
  - `tools/smokes/v2/profiles/integration/joinir/phase29c0_joinir_ext_shape01_red_seed_vm.sh`
- lock policy:
  1. rust reference route（`NYASH_VM_HAKO_PREFER_STRICT_DEV=0`）で GREEN (`18`, `RC:0`) を確認する。
  2. hako mainline route（`NYASH_VM_HAKO_PREFER_STRICT_DEV=1`）で GREEN (`18`, `RC:0`) と lane tag=`vm-hako` を確認する。
  3. 両 route で non-zero は拒否する。
  4. 両方で route report と lane tag を確認し、fallback drift を拒否する。

### JIR-EXT-02: `.hako` Mainline Implementation

- まず `.hako` 側に受理形を実装する（mainline-first）。
- 未対応形は reject/freeze タグで fail-fast を維持する。
- workaround 目的の AST rewrite は禁止（analysis-only view を使う）。

### JIR-EXT-03: Rust Reference Alignment (minimal)

- Rust は parity 比較で必要な最小差分のみ実装する。
- host 側は意味論の主導権を持たない（reference / portability / debug 補助に限定）。

### JIR-EXT-04: Parity Lock

- 同一 fixture で rust/hako の結果とタグを比較し、差分ゼロを確認する。
- route report の `fallback=0` を必須化する。

### JIR-EXT-05: Promote

- `CURRENT_TASK.md` / `10-Now.md` / 必要なら phase README を同期更新する。
- done 条件:
  1. 追加 fixture が fast gate で green
  2. planner-required gate が green
  3. fallback なし（route report / tags で確認）

## Commit Unit Rule

- `1 blocker = 1 shape = 1 fixture = 1 smoke = 1 commit` を固定する。
- fast gate FAIL 状態で `cases.tsv` 追加や別 shape の混入をしない。

## Current Baseline (2026-03-02)

- `build_stage1` の mainline emit route は `stageb-delegate` が既定。
- `hakorune --emit-mir-json` direct route は JoinIR freeze 切り分け用途（debug mainline）として運用する。
- helper-only 経路の結果は JoinIR 昇格判定に使わない。

## Reopen Rule

- lane A が monitor-only でも、次のいずれかで reopen する。
  1. `[joinir/freeze]` または `[plan/freeze:*]` が再発
  2. parity gate で rust/hako 差分が再発
  3. route report で `fallback=1` を検出
- reopen 時は JIR-EXT-00 から再開する。
