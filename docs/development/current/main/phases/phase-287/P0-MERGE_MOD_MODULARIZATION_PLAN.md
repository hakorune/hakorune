# Phase 287 P0: `merge/mod.rs` Modularization Plan（意味論不変）

**Date**: 2025-12-27  
**Status**: Historical（Implemented） ✅  
**Parent**: Phase 287 (Big Files Refactoring)  
**Goal**: `src/mir/builder/control_flow/joinir/merge/mod.rs` を“配線だけ”に寄せる（意味論不変）

---

## Note

この文書は実装前のプラン。Phase 287 P0 は既に完了しているので、現状の入口は以下:

- 入口（完了ログ）: `docs/development/current/main/phases/phase-287/P0-BIGFILES-REFACTORING-INSTRUCTIONS.md`
- 次（P2）: `docs/development/current/main/phases/phase-287/P2-CONTRACT_CHECKS-MODULARIZATION-INSTRUCTIONS.md`

## 前提（直近の SSOT）

- Pattern6 の merge/latch 事故は SSOT 化済み
  - latch 記録は `TailCallKind::BackEdge` のみ
  - entry-like は “JoinIR main の entry block のみ”
  - 二重 latch は `debug_assert!` で fail-fast
- loop header 推定は boundary SSOT を優先できる状態
  - `JoinInlineBoundary.loop_header_func_name`（明示）
  - ない場合のみ legacy heuristic（後方互換）

---

## 現状の問題

- `src/mir/builder/control_flow/joinir/merge/mod.rs` が巨大（責務が混在）
  - orchestrator / header PHI 構築 / entry 選定 / 値 remap / boundary ログ / 契約検証
- “推定（heuristics）” と “契約（SSOT）” が混ざると回帰が起きやすい

---

## 目標（Target State）

- `merge/mod.rs` は orchestrator のみ（公開 API + 配線）
- 純粋/半純粋ロジックを局所モジュールへ退避
- SSOT: `boundary.loop_header_func_name` 優先、fallback は “互換のためだけ”
- 意味論不変（挙動変更なし、silent fallback 追加なし）

---

## 提案ディレクトリ構造（案）

```
src/mir/builder/control_flow/joinir/merge/
├── mod.rs                       # orchestrator only
├── entry_selector.rs            # loop header func 選定（SSOT）
├── header_phi_prebuild.rs       # header PHI の事前構築（配線）
├── value_remapper.rs            # 小さな pure helper（必要なら）
├── boundary_logging.rs          # trace 統一（debug/verbose のみ）
└── verification/
    ├── mod.rs
    ├── phi_dst_checks.rs
    ├── carrier_checks.rs
    └── value_usage_checks.rs
```

注:
- “verification の SSOT” は既存 `merge/contract_checks.rs` と重なるので、移設するなら **入口を `verification/mod.rs` に統合**し、旧名は re-export で互換維持するのが安全。

---

## 実装フェーズ（Bottom-Up）

### Phase 1: `verification/` の抽出（低リスク）

- 対象: **pure function**（builder への副作用なし）だけを移す
- 既存 `contract_checks.rs` と役割衝突しないように、移設後も呼び出し点の責務を増やさない

検証:
- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`

### Phase 2: 小さな helper の抽出（低リスク）

- `value_remapper.rs` は “本当に重複があるなら” に限定（作りすぎない）
- 入口は `mod.rs` からのみ呼ぶ（依存拡散を避ける）

### Phase 3: `entry_selector.rs`（中リスク / SSOT）

SSOT:
1) `boundary.loop_header_func_name` があればそれを使う  
2) なければ legacy heuristic（`MAIN` と `boundary.continuation_func_ids` を除外して最初の関数）

注意（重要）:
- “k_exit の名前一致” で除外するのは NG（continuation は SSOT が `boundary.continuation_func_ids`）
- 対象は JoinIR function ではなく、merge が扱っている `MirModule.functions: BTreeMap<String, MirFunction>` の世界観に合わせる

検証:
- fixture: `./target/release/hakorune --backend vm apps/tests/phase1883_nested_minimal.hako`（RC=9）
- quick: `./tools/smokes/v2/run.sh --profile quick`

### Phase 4: `header_phi_prebuild.rs`（中リスク）

- ここは “pure” ではなく “配線（orchestrator補助）” と割り切る
- 入口で必要なもの（remapper / builder / boundary / loop_header_func_name など）を明示引数で受ける

### Phase 5: `boundary_logging.rs`（低リスク）

- `trace.stderr_if(..., debug/verbose)` へ統一
- “常時ログ” を禁止（quick のノイズ増加は避ける）

### Phase 6: `merge/mod.rs` の最終整理（低リスク）

- 公開 API と “段取り” だけ残す
- 変更は “移動 + 入口統一” に限定

---

## 検証（毎フェーズ）

```bash
cargo build --release
./target/release/hakorune --backend vm apps/tests/phase1883_nested_minimal.hako   # RC=9
./tools/smokes/v2/run.sh --profile quick
```

注:
- “0 warnings” は現状リポジトリ特性として非現実的なので、**0 errors** と **新規の恒常ログ無し** を受け入れ条件にする。

---

## リスクメモ

- `merge/mod.rs` と `merge/instruction_rewriter.rs` は両方で “entry 選定” を持ちがちなので、SSOT を二重にしない（可能なら selector を共用、ただし大きく動かさない）。
