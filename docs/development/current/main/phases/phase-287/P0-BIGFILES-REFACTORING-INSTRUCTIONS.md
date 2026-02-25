# Phase 287 P0: Big Files Refactoring 指示書（意味論不変 / 推定の削減）

**Date**: 2025-12-27  
**Status**: Complete ✅  
**Scope**: Rust 側の“でかいファイル”を分割して、推定（heuristics）依存を減らす  
**Non-goals**: 新機能、既定挙動変更、silent fallback 追加、env var 追加

---

## 目的（SSOT）

- “推定で決めている箇所” を減らし、境界・契約・入口を SSOT として明文化する。
- **意味論不変**で分割し、将来の拡張（Pattern6 generalize / merge 強化）に備える。

---

## 前提（現状の到達点）

- Pattern6 の事故（undef / 無限ループ）は SSOT へ固定済み:
  - latch 記録は `TailCallKind::BackEdge` のみ
  - entry-like は “JoinIR main の entry block のみ”
  - 二重 latch は `debug_assert!` で fail-fast
  - 入口: `docs/development/current/main/phases/phase-188.3/P2-REFACTORING-INSTRUCTIONS.md`

---

## でかいファイルの棚卸し（再現コマンド）

```bash
find src -name '*.rs' -print0 | xargs -0 wc -l | sort -nr | head -50
```

このセッションの観測（500行超え）:
- 16 個

---

## 優先順位（今すぐやる価値）

### 1) `merge/mod.rs`（1,555行）— 最優先

現状: merge coordinator + value remap + 契約検証 + header PHI 構築が 1 ファイルに同居。  
方針: “純粋寄り” から剥がして、`mod.rs` は orchestrator に寄せる。

実行用の詳細プラン:
- `docs/development/current/main/phases/phase-287/P0-MERGE_MOD_MODULARIZATION_PLAN.md`

結果（実装済み）:
- `merge/mod.rs`: 1,555 → 1,053 lines
- 新規モジュール: `entry_selector.rs`, `header_phi_prebuild.rs`, `value_remapper.rs`, `boundary_logging.rs`
- heuristic 排除: continuation 判定は `boundary.continuation_func_ids` を SSOT に採用（文字列一致を撤去）

#### 目標構造（案）

```
src/mir/builder/control_flow/joinir/merge/
├── mod.rs                      # orchestrator（公開APIと配線のみ）
├── entry_selector.rs           # loop header / entry block 選定（SSOT）
├── header_phi_prebuild.rs      # LoopHeaderPhiBuilder 呼び出し（SSOT）
├── boundary_logging.rs         # verbose/debug のみ（trace統一）
└── debug_assertions.rs         # merge 内の fail-fast / 契約検証（debug で固定）
```

注:
- Phase 287 P0 では `verification/` の新設ではなく、既存の “契約検証” を `debug_assertions.rs` に統合した（意味論不変）。
- 次（P2）で `contract_checks.rs` を facade 化し、契約単位のモジュール分割を行う（`contract_checks/` を新設予定）。

#### SSOT（ここで削る推定）

- loop header の推定を boundary に寄せる:
  - `JoinInlineBoundary.loop_header_func_name` を優先し、無い場合のみ legacy heuristic
- “log を常時出す” を禁止し、`trace.stderr_if(..., debug/verbose)` に統一

#### 受け入れ基準

- `cargo build --release` が通る
- `./tools/smokes/v2/run.sh --profile quick` が PASS
- 差分は “移動 + 入口統一” に限定（意味論不変）

---

### 2) `merge/instruction_rewriter.rs`（1,297行）— 今は“触らない”が正しい

現状: Scan → Plan → Apply の 3 段パイプラインが 1 ファイルで、局所的に複雑。  
方針: **Pattern6 の直後なので、いま大きく動かさない**（回帰コストが高い）。

#### ただし“安全にできる”こと（意味論不変）

- policy を 1 箇所へ集約して SSOT にする（既に一部完了）
  - latch 記録: `src/mir/builder/control_flow/joinir/merge/rewriter/latch_incoming_recorder.rs`
  - tail-call 分類: `src/mir/builder/control_flow/joinir/merge/tail_call_classifier.rs`
- “loop header 推定” を boundary SSOT に寄せる（既に実装済み）

#### 将来の分割計画（今すぐはやらない）

- `scanner.rs` / `planner/` / `applicator.rs` へ物理分割
- `RewriteContext` を SSOT にして、stage 間の引数を減らす

---

### 3) `patterns/ast_feature_extractor.rs`（1,148行）— 低難易度で効く

現状: 複数の “検出器” が同居。純粋関数なので物理分割が安全。  
方針: `pattern_recognizers/` を作って“1 recognizer = 1 質問”にする。

結果（実装済み）:
- `ast_feature_extractor.rs`: 1,148 → 135 lines（facade）
- `pattern_recognizers/`: 8 modules（1 module = 1 質問）

#### 目標構造（案）

```
src/mir/builder/control_flow/joinir/patterns/
├── ast_feature_extractor.rs          # facade（re-export と glue）
└── pattern_recognizers/
    ├── mod.rs
    ├── continue_break.rs
    ├── infinite_loop.rs
    ├── if_else_phi.rs
    ├── carrier_count.rs
    └── ...（既存の extracted recognizer と揃える）
```

#### 受け入れ基準

- 既存の public 関数シグネチャを維持（呼び出し側の差分を最小化）
- `cargo build --release` + quick PASS
- unit tests は “薄く” で良い（recognizer 単位で 1–2 個）

---

## 小テスト（1本だけ）で契約を固定する（推奨）

“bb番号や ValueId の固定” は不安定なので、**構造テスト**で固定する。

- latch 二重セット検知（debug）: `LoopHeaderPhiInfo::set_latch_incoming()` に `#[should_panic]`
- tail-call 分類の境界: `classify_tail_call()` の “entry-like でも target!=loop_step は LoopEntry ではない”

（MIR文字列の grep 固定は、ブロック番号が揺れやすいので最終手段）

---

## 検証手順（毎回）

```bash
cargo build --release
./target/release/hakorune --backend vm apps/tests/phase1883_nested_minimal.hako   # RC=9
./tools/smokes/v2/run.sh --profile quick
```
