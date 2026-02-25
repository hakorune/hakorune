# Phase 287 P6: Scan plan 統合（docs-first, 意味論不変の範囲で）

**Date**: 2025-12-27  
**Status**: Completed ✅  
**Scope**: `merge/instruction_rewriter.rs` の Stage 1（scan）を削除して pipeline を 2-stage（Plan→Apply）へ単純化する（意味論不変）。  
**Non-goals**: 挙動変更、ログ恒常増加、silent fallback 追加、テスト専用の暫定コード追加

---

## 背景（問題点）

現状の pipeline は “Scan → Plan → Apply” という構造だが、`RewritePlan` が Stage 2 で実質未使用になっており、同じ検出（tail call / return など）を Stage 2 側で再走査している。

この状態は以下の問題を生む:
- “Scan の存在意義” が曖昧で、将来の保守時に二重修正が起きやすい
- Stage 境界の責務が不明瞭になりやすい（read-only のはずが drift しやすい）

---

## 目的（SSOT）

- Scan の役割を SSOT として固定する（使うなら “何に使うか”、捨てるなら “なぜ捨てるか”）。
- 意味論不変を守ったまま、重複ロジックを解消するための “安全な最短ルート” を決める。

---

## 選択肢（決めること）

### Option A: Scan plan を Stage 2 に反映（維持）

- `RewritePlan` を Stage 2 の入力として使い、検出を 1 回に寄せる。
- 受け入れ条件:
  - Stage 2 で “scan と同等の検出” を再計算しない
  - out-of-scope は `Ok(None)` ではなく、現状通りの Fail-Fast 契約を維持（fallback 禁止）

### Option B: Stage 1 を削除（単純化）

- Scan を消し、Stage 2 が唯一の “解析 + 変換” 入口になる。
- 受け入れ条件:
  - debug/log の恒常出力差を出さない（既存 `debug` フラグに従う）
  - pipeline の見通し（責務）を docs と module 構造で補強する

**Decision（2025-12-27）**: Option B を採用する。

理由:
- Stage 2（Plan）は boundary/local map/PHI/tailcall/terminator を扱っており、Scan を“本当に使う”には Scan 側に情報を増殖させやすい。
- Scan が未使用のまま残るのは二重修正の温床なので、先に削除して 2-stage へ収束させるのが最も構造的に安全。

---

## 手順（docs-first）

1. `RewritePlan` の intended contract を 10 行程度で記述（何を “決めた” と見なすか）
2. Option A/B を選び、採用理由と不採用理由を 1 段落ずつ書く
3. 実装は “移動 + 入口統一” に限定する（if 増殖や新条件を入れない）

---

## 検証（受け入れ基準）

```bash
cargo build --release
./tools/smokes/v2/run.sh --profile quick
./target/release/hakorune --backend vm apps/tests/phase1883_nested_minimal.hako   # RC=9
```

受け入れ:
- Build: 0 errors
- quick: 154/154 PASS
- Pattern6: RC=9 維持
- 恒常ログ増加なし
