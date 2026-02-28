---
Status: Active
Decision: accepted
Date: 2026-02-28
Scope: `.hako` 完結（Rust thin wrapper化）へ向けた runtime C ABI cutover の実行順を固定する。
Related:
  - docs/development/current/main/design/de-rust-lane-map-ssot.md
  - docs/development/current/main/design/de-rust-post-g1-runtime-plan-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-214-runtime-rust-thin-to-zero-execution-path-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-215-runtime-execution-path-observability-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-216-runtime-v0-abi-slice-lock-ssot.md
  - docs/development/current/main/phases/phase-29y/10-ABI-SSOT.md
  - docs/reference/abi/ABI_BOUNDARY_MATRIX.md
  - docs/reference/abi/nyrt_c_abi_v0.md
  - docs/development/current/main/design/optimization-portability-classification-ssot.md
---

# Hako Runtime C ABI Cutover Order (SSOT)

## Goal

- 最終形を `.hako` 主体に寄せる。
- Rust は runtime 意味論の実体ではなく、thin wrapper と検証導線へ縮退する。
- 既存契約（borrowed/owned, fail-fast, gate）を壊さずに移行する。
- done 判定は `execution-path-zero`（mainline/CI既定で Rust runtime/plugin loader 非依存）で固定する。

## Non-goals

- 一括置換（big-bang rewrite）
- 第3の新規ABI面の追加
- selfhost 前に広域機能追加を混ぜること

## Boundary Lock (must keep)

1. Canonical ABI は 2 面のみを維持する。
   - Core C ABI
   - TypeBox ABI v2
2. 関数ABIは `args borrowed / return owned` を維持する。
3. `GC` と `poll` は独立スイッチとして維持する。
4. 失敗は fail-fast タグで停止し、silent fallback を禁止する。

## Precondition (before cutover work)

先に LLVM-HOT-20 の打ち止めを固定する。

1. `cargo check --bin hakorune`
2. `tools/checks/dev_gate.sh quick`
3. `PERF_GATE_BENCH_COMPARE_ENV_CHECK=1 PERF_GATE_AOT_SKIP_BUILD_CHECK=1 PERF_GATE_AOT_AUTO_SAFEPOINT_ENV_CHECK=1 PERF_GATE_KILO_PARITY_LOCK_CHECK=1 bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh`
4. `bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small 1 3`

## Fixed Execution Order (1 blocker = 1 contract = 1 commit)

### Step 0: Docs boundary sync (docs-only)

- ABI 境界・所有権・エラー契約を 1 枚で再確認し、実装に入る前の参照先を固定する。
- 受け入れ:
  - `ABI_BOUNDARY_MATRIX.md`
  - `10-ABI-SSOT.md`
  - 本文書
  の 3 点で矛盾がない。

### Step 1: V0 ABI slice lock (docs + tests)

- まず 3 関数だけを cutover 対象に固定する。
  - `string_len`
  - `array_get_i64`
  - `array_set_i64`
- 各関数で以下を明記:
  - 入力型
  - 所有権
  - 失敗時契約
  - `.hako` 側の同等口
- 受け入れ:
  - ABI 仕様の関数表が 3 関数で確定
  - 既存テストに対応 contract 名が追加される
  - lock: `29cc-216-runtime-v0-abi-slice-lock-ssot.md`

### Step 2: Rust thin wrapper lock (small code)

- Rust 側は「ABI 入出力整形 + fail-fast」だけに責務を限定する。
- runtime 意味論（文字列検索/配列語彙の実体）は増やさない。
- 受け入れ:
  - wrapper 単体テスト
  - `cargo check --bin hakorune`

### Step 3: `.hako` adapter lock (small code)

- `.hako` 側に同形 adapter を作り、3 関数だけ直結する。
- VM low-level の配線は
  `lang/src/runtime/collections/array_core_box.hako`（array系）と
  `lang/src/runtime/collections/string_core_box.hako`（string_len）を正本入口とし、
  `lang/src/vm/boxes/mir_call_v1_handler.hako` からのみ呼ぶ。
- Rust fallback を暗黙利用しない（strict で fail-fast）。
- 受け入れ:
  - adapter fixture smoke（3 関数）
  - `string_len` は adapter route 契約（registry + handler tag + core box）で固定
  - lane C quick gate 緑

### Step 4: One-route cutover (kilo monitor)

- `kilo` の1経路だけ `.hako` adapter 優先へ切り替える。
- 性能目標ではなく、契約一致を優先する。
- 受け入れ:
  - `kilo` parity lock 緑
  - 回帰時は即 rollback 可能（1コミット戻し）

### Step 5: Expand by one vocabulary at a time

- 以降は 1 語彙ずつ同じ型で拡張する。
- 1コミットに複数語彙を混ぜない。

## Optional External Review Checkpoint

- 外部相談（例: ChatGPT Pro）は Step 1 完了時に 1 回だけ行う。
- 相談対象は「ABI関数表と所有権契約」のレビューに限定する。
- 最適化一般論の再相談は、このレーンでは行わない。

## Done (for this SSOT)

- Step 0/1 の docs 固定が完了し、次実装が 1 本に絞れている。
- 実装は Step 2 から順番に着手する。
- source-zero（Rust 実装完全撤去）は別フェーズで扱う。
