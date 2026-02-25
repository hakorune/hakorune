# Phase 287 P1: `ast_feature_extractor.rs` 分割指示書（意味論不変）

**Date**: 2025-12-27  
**Status**: Complete ✅  
**Scope**: `src/mir/builder/control_flow/joinir/patterns/ast_feature_extractor.rs`（1,148行）を“recognizer単位”に分割  
**Non-goals**: ルーティング条件の変更、検出仕様の変更、silent fallback 追加、`merge/instruction_rewriter.rs` の大改造

実装:
- commit: `de1cd1fea`
- Phase hub: `docs/development/current/main/phases/phase-287/README.md`

---

## 目的（SSOT）

- “推定（heuristics）で決めているところ”を増やさず、既存検出の **契約を保ったまま** 構造（フォルダ/モジュール）で責務分離する。
- `ast_feature_extractor.rs` を **facade**（re-export + glue）へ寄せて、個別検出は “1ファイル=1質問” にする。

---

## 背景（現状）

- `ast_feature_extractor.rs` は純粋関数中心だが、複数の検出器が同居して巨大化している。
- `escape_pattern_recognizer.rs` のように、既に分割できることは確認済み。

---

## 目標の構造（案）

```
src/mir/builder/control_flow/joinir/patterns/
├── ast_feature_extractor.rs               # facade（公開関数の入口）
└── pattern_recognizers/                  # NEW
    ├── mod.rs
    ├── continue_break.rs                 # continue/break/return の単純探索
    ├── infinite_loop.rs                  # condition==true の判定
    ├── if_else_phi.rs                    # if-else phi 検出
    ├── carrier_count.rs                  # carrier count（代入ベース）
    ├── parse_number.rs                   # parse_number 系の検出
    ├── parse_string.rs                   # parse_string 系の検出
    └── skip_whitespace.rs                # skip_ws 系の検出
```

注:
- 既存の `escape_pattern_recognizer.rs` はそのままでも良い（P1で無理に統合しない）。
- “recognizer名” は **検出の質問** を表す（例: `detect_infinite_loop`、`detect_continue_pattern` など）。

---

## 実績（実装済み）

- `ast_feature_extractor.rs` は facade 化され、既存 import の互換を維持したまま `pattern_recognizers/` へ分割済み。
- 恒常ログ増加なし、build/quick/Pattern6 の回帰なし（意味論不変）。

---

## 進め方（安全な順序）

### Step 1: facade化の準備（最小差分）

- `pattern_recognizers/` を追加し、`mod.rs` を置く（空でもよい）。
- `ast_feature_extractor.rs` から、新フォルダの関数を `pub(crate)` で re-export できる形にする。

### Step 2: “依存が少ない” 検出器から移す

優先（低依存）:
- `detect_continue_in_body` / `detect_break_in_body` / `detect_return_in_body`
- `detect_infinite_loop`

ルール:
- 公開関数シグネチャは維持（呼び出し側の差分最小）。
- `pub(crate)` API の入口は **ast_feature_extractor.rs に残す**（外部参照の破壊を避ける）。

### Step 3: 中依存（helper多め）を移す

- if-else phi 系
- carrier count 系

### Step 4: 個別パターン recognizer を移す（必要なら）

- parse_number / parse_string / skip_whitespace など
- 既に抽出済みの recognizer と重複する場合は、P1では **統合しない**（P2以降で整理）。

---

## テスト（仕様固定）

P1 は意味論不変が主目的なので “薄く” で良い:

- 既存の unit tests があるなら移設に合わせて位置だけ更新
- 新規追加するなら、各 recognizer に 1本まで（代表ケースのみ）

---

## 検証手順（受け入れ基準）

```bash
cargo build --release
./target/release/hakorune --backend vm apps/tests/phase1883_nested_minimal.hako   # RC=9
./tools/smokes/v2/run.sh --profile quick
```

受け入れ:
- ビルド 0 errors
- quick 154/154 PASS
- Pattern6 RC=9 維持
- 恒常ログ増加なし
