# Phase 287: Developer Hygiene（big files / smoke / normalizer）

**Status**: Complete (P0-P8)
**Date**: 2025-12-27
**Previous**: Phase 286 (Plan Line完全運用化)

## 概要（SSOT）

Phase 287 は「開発導線の整備（意味論不変）」を優先して、巨大ファイルの責務分離（big files refactoring）と、既存の hygiene（smoke/normalizer）を扱う。

## 2025-12-27 Update: Big Files Refactoring（P0）✅

`merge/mod.rs` を modularize（意味論不変）し、SSOT（boundary-first / continuation SSOT）を強化した。

- 入口: `docs/development/current/main/phases/phase-287/P0-BIGFILES-REFACTORING-INSTRUCTIONS.md`

## 2025-12-27 Update: AST Feature Extractor modularization（P1）✅

`ast_feature_extractor.rs` を facade にして、`pattern_recognizers/` 配下へ recognizer 単位で分割した（意味論不変）。

- 入口: `docs/development/current/main/phases/phase-287/P1-AST_FEATURE_EXTRACTOR-INSTRUCTIONS.md`
- 次（P2）: `docs/development/current/main/phases/phase-287/P2-CONTRACT_CHECKS-MODULARIZATION-INSTRUCTIONS.md`（完了）

---

## 2025-12-27 Update: Contract Checks modularization（P2）✅

- `contract_checks.rs` を facade 化して、契約検証を “1 module = 1 契約” に分割した（意味論不変）。
  - 指示書: `docs/development/current/main/phases/phase-287/P2-CONTRACT_CHECKS-MODULARIZATION-INSTRUCTIONS.md`

---

## 2025-12-27 Update: Instruction Rewriter stage split（P3）✅

- `merge/instruction_rewriter.rs` を “Scan/Plan/Apply” の stage 単位に分割し、facade（orchestrator）へ縮退した（意味論不変）。
  - 指示書: `docs/development/current/main/phases/phase-287/P3-INSTRUCTION_REWRITER-MODULARIZATION-INSTRUCTIONS.md`

---

## 2025-12-27 Update: Plan stage modularization（P4）✅

- `rewriter/stages/plan.rs` を facade 化し、`rewriter/stages/plan/` 配下へ責務単位で分割した（意味論不変）。
  - 指示書: `docs/development/current/main/phases/phase-287/P4-PLAN_STAGE-MODULARIZATION-INSTRUCTIONS.md`

---

## 2025-12-27 Update: Stages facade（P5）✅

- pipeline 関数（scan/plan/apply）を `stages/mod.rs` から re-export し、呼び出し側 import を単一入口へ統一した（意味論不変）。
  - 指示書: `docs/development/current/main/phases/phase-287/P5-STAGES-VISIBILITY-FACADE-INSTRUCTIONS.md`

---

## 2025-12-27 Update: Remove scan stage（P6）✅

- Scan stage を削除し、pipeline を 2-stage（Plan→Apply）へ単純化した（意味論不変）。
  - 指示書: `docs/development/current/main/phases/phase-287/P6-SCAN_PLAN-INTEGRATION-INSTRUCTIONS.md`

---

## 2025-12-27 Update: Rewriter scaffolding cleanup（P7）✅

- pipeline の古い scaffolding（`apply_box.rs` / `parameter_binding_box.rs` / `tail_call_detector_box.rs`）を削除し、SSOT を `rewriter/stages/*` に寄せた（意味論不変）。
  - 指示書: `docs/development/current/main/phases/phase-287/P7-REWRITER-BOX-SCAFFOLDING-CLEANUP-INSTRUCTIONS.md`

---

## 2025-12-27 Update: Rewriter README / guard（P8）✅

- `rewriter/README.md` を追加し、責務境界と SSOT（Plan→Apply）を明文化した（docs-only）。
  - 指示書: `docs/development/current/main/phases/phase-287/P8-REWRITER-README-GUARD-INSTRUCTIONS.md`
  - 成果物: `src/mir/builder/control_flow/joinir/merge/rewriter/README.md`

---

## 2025-12-27 Update: Phase closeout（P9）✅

- Phase 287 完了状態を docs に反映し、Now/Backlog を次フェーズへ切り替えた（docs-only）。
  - 指示書: `docs/development/current/main/phases/phase-287/P9-PHASE-CLOSEOUT-INSTRUCTIONS.md`

---

## Summary

**Phase 287 完了項目（P0-P8）**:
- ✅ P0: `merge/mod.rs` modularization (1,555 → 1,053 lines)
- ✅ P1: `ast_feature_extractor.rs` facade化 (1,148 → 135 lines)
- ✅ P2: `contract_checks.rs` facade化 & 契約単位分割
- ✅ P3: `instruction_rewriter.rs` stage分割 (Scan/Plan/Apply)
- ✅ P4: `rewriter/stages/plan.rs` facade化 (741 → 120 lines)
- ✅ P5: `stages/mod.rs` facade & re-export統一
- ✅ P6: Scan stage削除 (Plan→Apply 2-stage pipeline)
- ✅ P7: 未使用Box雛形削除 (apply_box, tail_call_detector_box, parameter_binding_box)
- ✅ P8: `rewriter/README.md` 追加 (責務境界明文化)
- ✅ P9: Phase closeout (docs更新)

**検証結果**: quick 154/154 PASS維持、意味論不変

**Next**: None (Phase 287 complete)

## Legacy / Historical (2025-12-26 plan)

以下は「Phase 287 を hygiene として計画していた時期のログ」。今後の候補として残すが、P0/P1（big files）とは別系統。

### Legacy docs（smoke quick）

- P1（legacy）: quick 軽量化（~45s 目標）: `docs/development/current/main/phases/phase-287/P1-INSTRUCTIONS.md`
- P2（legacy, optional）: quick をさらに 45s へ寄せる: `docs/development/current/main/phases/phase-287/P2-INSTRUCTIONS.md`

## Phase 286 完了作業（historical）

### ✅ Legacy Pattern5 削除（488行）

**削除理由**: Plan line 完全運用化により、legacy Pattern5 は完全にデッドコード化

**削除ファイル**:
- historical path token: `pattern5_infinite_early_exit.rs` under the old `joinir/patterns/` lane (488行)

**関連削除**:
- `router.rs` の `LOOP_PATTERNS` テーブルから Pattern5 エントリ削除
- `mod.rs` から `pub mod pattern5_infinite_early_exit;` 削除
- `router.rs` のドキュメント更新（Pattern5 → Pattern4 優先順位へ）

**影響範囲**:
- Pattern5 は Plan line の `extract_pattern5_plan()` 経由で処理されるため、機能退行なし
- `LOOP_PATTERNS` テーブルの優先順位: Pattern5 → Pattern4 → Pattern8... から Pattern4 → Pattern8... に変更

### ✅ Warning クリーンアップ

**実行コマンド**:
```bash
cargo fix --lib -p nyash-rust --allow-dirty
```

**修正内容**:
- 1件の自動修正（`normalizer.rs`）
- 未使用 import などを自動修正

### ✅ ビルド＆テスト確認

**ビルド結果**:
```bash
cargo build --release
# → 成功（130 warnings、エラーなし）
```

**テスト結果**:
```bash
tools/smokes/v2/run.sh --profile quick
# → 154/154 PASS ✅
```

**退行なし**: quick smoke 154/154 PASS を維持

## 削除前後の統計

### コード削減

| 項目 | 削減数 |
|------|--------|
| ソースファイル | 1ファイル (pattern5_infinite_early_exit.rs) |
| 総削減行数 | 488行 |
| router.rs エントリ | 5行（Pattern5 エントリ） |
| mod.rs 宣言 | 1行 |

### Pattern優先順位の変更

**削除前**（Phase 131-11+）:
```
Pattern5 (most specific) → Pattern4 → Pattern3 → Pattern1 → Pattern2
```

**削除後**（Phase 286+）:
```
Pattern4 → Pattern8 → Pattern9 → Pattern3 → Pattern1 → Pattern2
```

**注**: Pattern5/6/7 は Plan line 経由で処理（`PLAN_EXTRACTORS` テーブル）

## Legacy backlog (post-2025-12-27)

### (legacy) normalizer.rs 分割計画

**現状**: `src/mir/builder/control_flow/plan/normalizer.rs` が大きすぎる（推定 1,500+ 行）

**分割案**:

#### 1. Pattern5 正規化ロジック分離（430行）
- `normalizer/pattern5.rs` - Pattern5 専用正規化
- Pattern5 の複雑な構造展開ロジックを独立モジュール化

#### 2. Helper 関数共通化（700行）
- `normalizer/helpers.rs` - 共通ヘルパー関数
- 複数パターンで使用される変換ロジック

#### 3. コアロジック残存（300行）
- `normalizer.rs` - PlanNormalizer struct & normalize() エントリーポイント
- パターン横断的な正規化インターフェース

**期待効果**:
- メンテナンス性向上
- テスト分離（Pattern5 正規化のみをユニットテスト）
- 責任分離（SRP原則）

### (legacy) LOOP_PATTERNS テーブル完全削除

**背景**: 全Pattern が Plan line 経由になれば、`LOOP_PATTERNS` テーブルは不要

**前提条件**:
- Pattern1-4 の Plan line 移行完了（Phase 286 でPattern1-4はPlan line化済み）
- Pattern8-9 の Plan line 移行完了（Phase 286 でPlan line化済み）

**削除対象**:
- `router.rs` の `LOOP_PATTERNS` static テーブル（40行程度）
- `LoopPatternEntry` struct（不要化）

**残存**:
- `PLAN_EXTRACTORS` テーブルのみ（SSOT）

**期待効果**:
- 二重管理解消（LOOP_PATTERNS vs PLAN_EXTRACTORS）
- ルーティングロジック一本化

## 実装方針

### フェーズ分割

1. **Phase 286（完了）**: Legacy Pattern5 削除 + Warning クリーンアップ
2. **Phase 287-P0（保留）**: normalizer.rs 分割（P0優先度は低い）
3. **Phase 287-P1（将来）**: legacy JoinIR table 削除（全route family の Plan line 化後）

### 段階的移行

- 急がない：normalizer.rs 分割は緊急度低（機能的に問題なし）
- 機会を待つ：P1 はPattern1-4完全移行のタイミングで実施

## 関連ドキュメント

- [Phase 286 計画](../phase-286/README.md) - Plan Line完全運用化
- [Plan Line アーキテクチャ](../../design/plan-line-architecture.md) - Extractor → Normalizer → Verifier → Lowerer
- [Pattern移行ログ](../phase-273/README.md) - Pattern6/7 Plan line移行（Phase 273）

## 検証項目

### ✅ Phase 286 完了検証

- [x] Pattern5 ファイル削除（488行）
- [x] router.rs の LOOP_PATTERNS から Pattern5 エントリ削除
- [x] mod.rs から Pattern5 宣言削除
- [x] `cargo fix` 実行（Warning クリーンアップ）
- [x] `cargo build --release` 成功（0エラー）
- [x] quick smoke 154/154 PASS
- [x] Phase 287 ドキュメント作成

### 📋 Phase 287-P0/P1 検証（将来）

- [ ] normalizer.rs 分割完了（P0）
  - [ ] pattern5.rs 独立（430行）
  - [ ] helpers.rs 共通化（700行）
  - [ ] normalizer.rs 縮小（300行）
- [ ] LOOP_PATTERNS テーブル削除（P1）
  - [ ] 全Pattern Plan line経由確認
  - [ ] `LOOP_PATTERNS` static削除
  - [ ] legacy route-entry struct 削除

## 備考

**注意**: normalizer.rs の分割は行わない（本Phase完了時点では保留）

**理由**:
- 機能的に問題なし（現在の構造で動作）
- 緊急度低（開発速度への影響なし）
- 別Phase対応が適切（Phase 287-P0として計画のみ）
