Status: VerificationReport, Historical

# Phase 84-3: PhiTypeResolver 実装完了報告書

**日付**: 2025-12-02
**作業者**: Claude
**レビュー者**: User (tomoaki)

## エグゼクティブサマリー

Phase 84-3 で PhiTypeResolver を実装した結果、**Case D を 9件 → 4件に削減**しました（**56%削減達成**）。

特に、Loop 制御フロー関連の複雑な PHI パターン（GroupA）を **5件完全解決**し、箱理論に基づく型推論システムの有効性を実証しました。

## 成果指標

| 指標 | 目標 | 実績 | 達成率 |
|-----|------|------|--------|
| Case D 削減件数 | 5件以上 | 5件 | 100% |
| Case D 残存件数 | 5件以下 | 4件 | 120%（目標超過） |
| 削減率 | 40%以上 | 56% | 140%（目標超過） |
| GroupA 解決率 | 80%以上 | 100% | 125%（目標超過） |

## 削減詳細

### Phase 84-3 で解決された 5件

| # | テスト名 | ValueId | パターン | 検証結果 |
|---|---------|---------|---------|---------|
| 1 | `loop_with_continue_and_break_edge_copy_merge` | ValueId(56) | Loop + continue/break | ✅ 解決 |
| 2 | `nested_loop_with_multi_continue_break_edge_copy_merge` | ValueId(135) | Nested loop | ✅ 解決 |
| 3 | `loop_inner_if_multilevel_edge_copy` | ValueId(74) | Loop + 多段 if | ✅ 解決 |
| 4 | `loop_break_and_early_return_edge_copy` | ValueId(40) | Loop + early return | ✅ 解決 |
| 5 | `vm_exec_break_inside_if` | ValueId(27) | Loop + if-break | ✅ 解決確認済み |

**検証コマンド**:
```bash
NYASH_PHI_FALLBACK_DISABLED=1 cargo test --release --lib vm_exec_break_inside_if
# 結果: test ... ok ✅
```

### 残り 4件の分類

| # | テスト名 | ValueId | パターン | 解決方法 |
|---|---------|---------|---------|---------|
| 1 | `test_lowering_await_expression` | ValueId(2) | await 特殊構文 | Phase 84-4-C |
| 2 | `mir_lowering_of_qmark_propagate` | ValueId(7) | QMark 特殊構文 | Phase 84-4-B |
| 3 | `mir_stage1_cli_emit_program_min_compiles_and_verifies` | ValueId(7) | Stage1Cli 型推論 | Phase 84-4-B |
| 4 | `mir_stage1_cli_emit_program_min_exec_hits_type_error` | ValueId(7) | Stage1Cli 型推論 | Phase 84-4-B |

## 技術的成果

### PhiTypeResolver の実装

**ファイル**: `src/mir/phi_core/phi_type_resolver.rs`（新規作成）

**責務**: PHI + Copy グラフを辿って、安全に型を決められるときだけ MirType を返す

**コア機能**:

```rust
pub struct PhiTypeResolver<'f> {
    func: &'f MirFunction,
    value_types: &'f BTreeMap<ValueId, MirType>,
}

impl<'f> PhiTypeResolver<'f> {
    pub fn resolve(&self, root: ValueId) -> Option<MirType> {
        // DFS でグラフ探索
        // Copy → src へ進む
        // Phi → incoming 値へ進む
        // base 定義 → value_types から型取得
        // 全ての base 型が一致すれば返す
    }
}
```

**安全装置**:
- visited セットで循環検出（無限ループ防止）
- 探索上限（max_visits）でタイムアウト防止
- Unknown/Void の除外による型安全性確保

### 解決メカニズム

**GroupA（Loop 制御フロー）の典型的パターン**:

```
Block1 (loop_header):
  %sum_header = PHI [%sum_init, %sum_loop, %sum_updated]

Block2 (break):
  %sum_final = Copy %sum_header  ← value_types に IntegerBox 登録済み
  Jump after_loop

Block3 (continue):
  %sum_loop = Copy %sum_header   ← value_types に IntegerBox 登録済み
  Jump loop_header

Block4 (after_loop):
  %56 = PHI [%sum_final]         ← PhiTypeResolver で型推論成功！
  Return %56
```

**PhiTypeResolver の探索経路**:
```
%56 (PHI) → %sum_final (Copy) → %sum_header (PHI) → %sum_init (Const/IntegerBox)
                                                    → %sum_loop (Copy) → ...
                                                    → %sum_updated (BinOp/IntegerBox)
```

**結果**: 全ての base 型が IntegerBox → %56 の型は IntegerBox と推論成功

## 残存 4件の根本原因

**統一された問題**: 「base 定義（BoxCall/Await）の戻り値型が value_types に未登録」

### なぜ PhiTypeResolver で解決できないか

PhiTypeResolver の設計原則:
- **責務**: 既に登録された型を「伝播」する
- **制約**: base 定義の型が未登録の場合は None を返す（正しい動作）

BoxCall/Await 命令の問題:
- lowering 時に戻り値型を value_types に登録していない
- PhiTypeResolver が探索しても型情報が存在しない

**解決策**: BoxCall/Await の lowering 時に型情報を登録する（Phase 84-4）

## Phase 84-4 への推奨

### 実装優先度

1. **Phase 84-4-A**: dev フォールバック（0.5日）
   - 目的: 開発環境の即座のアンブロック
   - 環境変数: `NYASH_PHI_DEV_FALLBACK=1`
   - 対象: 全 4件（暫定）

2. **Phase 84-4-B**: BoxCall 型情報登録（1-2日）
   - 目的: 根本解決
   - 実装箇所: `src/mir/builder/builder_calls.rs`
   - 対象: 3件（GroupB 2件 + GroupD 1件）

3. **Phase 84-4-C**: Await 型情報特殊処理（0.5日）
   - 目的: 暫定解決
   - 実装箇所: `src/mir/builder/stmts.rs`
   - 対象: 1件（GroupC）

### 期待成果

```bash
# Phase 84-4-A 完了後
NYASH_PHI_DEV_FALLBACK=1 NYASH_PHI_FALLBACK_DISABLED=1 cargo test --release --lib
# 期待: Case D = 0件（dev 環境のみ）

# Phase 84-4-B 完了後
NYASH_PHI_FALLBACK_DISABLED=1 cargo test --release --lib 2>&1 | grep "Case D" | wc -l
# 期待: 1件（await のみ残存）

# Phase 84-4-C 完了後
NYASH_PHI_FALLBACK_DISABLED=1 cargo test --release --lib 2>&1 | grep "Case D" | wc -l
# 期待: 0件（完全解決）
```

## Phase 82-84 の累積成果

| Phase | 実装内容 | 削減件数 | 残存件数 | 削減率 | 累積削減率 |
|-------|---------|---------|---------|--------|-----------|
| Phase 82 | フォールバック検出 | - | 12件 | - | - |
| Phase 84-2 | CopyTypePropagator | 3件 | 9件 | 25% | 25% |
| **Phase 84-3** | **PhiTypeResolver** | **5件** | **4件** | **56%** | **67%** |
| Phase 84-4（目標） | BoxCall/Await 型登録 | 4件 | 0件 | 100% | 100% |

## 箱理論の実現

Phase 84-3 により、型推論システムの箱化が明確化されました:

```
[型生成レイヤー] - 型を作る
  ├─ emit_const()          ✅ 実装済み
  ├─ emit_box_call()       🎯 Phase 84-4-B で型登録追加
  └─ build_await_expression() 🎯 Phase 84-4-C で型登録追加

[型伝播レイヤー] - 型を広げる
  ├─ CopyTypePropagator    ✅ Phase 84-2 実装済み
  └─ PhiTypeResolver       ✅ Phase 84-3 実装済み

[統合レイヤー] - 全体を調整
  └─ GenericTypeResolver   ✅ 既存実装

[レガシー] - 削除予定
  └─ if_phi.rs フォールバック 🗑️ Phase 84-5 で削除
```

## リスクと軽減策

### リスク1: GroupD（QMark）が新規出現

**リスク**: PhiTypeResolver 実装の副作用で、以前は隠蔽されていた型推論の欠陥が顕在化

**軽減策**:
- ✅ 根本原因を特定済み（BoxCall 型情報の未登録）
- ✅ Phase 84-4-B で根本解決予定

**ポジティブな側面**: 以前は偶然動いていた部分を明示的に修正できる

### リスク2: dev フォールバックの濫用

**リスク**: Phase 84-4-A の dev フォールバックが常用され、根本解決が遅延

**軽減策**:
- ✅ 環境変数による明示的制御（`NYASH_PHI_DEV_FALLBACK=1`）
- ✅ production 環境（CI）では依然として厳格
- ✅ 警告ログで問題箇所を明示

### リスク3: Phase 84-4 の実装時間超過

**リスク**: BoxCall 型情報登録が予想より複雑で 1-2日を超過

**軽減策**:
- ✅ ビルトイン Box のハードコード型情報で最小実装
- ✅ Phase 26-A の slot_registry 統合は将来拡張として分離
- ✅ Unknown 型での暫定登録も許容

## 今後のマイルストーン

### Phase 84-4（予定: 2-3日）

**目標**: BoxCall/Await 型情報登録による根本解決

**成果物**:
- `src/mir/builder/lifecycle.rs` - dev フォールバック追加
- `src/mir/builder/builder_calls.rs` - BoxCall 型登録追加
- `src/mir/builder/stmts.rs` - Await 型登録追加

**完了条件**:
```bash
NYASH_PHI_FALLBACK_DISABLED=1 cargo test --release --lib 2>&1 | grep "Case D" | wc -l
# 期待: 0
```

### Phase 84-5（予定: 1日）

**目標**: if_phi.rs レガシーフォールバック完全削除

**成果物**:
- `src/mir/join_ir/lowering/if_phi.rs` 削除（約 300行）
- `GenericTypeResolver` の if_phi 呼び出し削除
- `lifecycle.rs` の Case D 処理削除

**完了条件**:
```bash
# if_phi.rs が存在しない
ls src/mir/join_ir/lowering/if_phi.rs
# 期待: No such file or directory

# レガシーフォールバック呼び出しが存在しない
grep -r "infer_type_from_phi_fallback" src/
# 期待: 出力なし
```

## 謝辞

Phase 84-3 の成功は、以下の要因によるものです:

1. **ChatGPT Pro の設計**: PHI + Copy グラフ型推論という明確な責務分離
2. **箱理論の適用**: 単一責務の徹底による保守性向上
3. **段階的実装**: Phase 84-2 の CopyTypePropagator という土台
4. **詳細な調査**: Phase 84-1/2 の失敗パターン分析

## まとめ

**Phase 84-3 の成果**:
- ✅ PhiTypeResolver 実装完了（新規ファイル作成）
- ✅ 56%削減達成（9件 → 4件）
- ✅ GroupA（Loop 制御フロー）100%解決
- ✅ 箱理論に基づく型推論システムの明確化

**残り 4件の本質**:
- 全て「BoxCall/Await の型情報未登録」という同一問題
- PhiTypeResolver の責務外（設計上正しい制約）

**Phase 84-4 への期待**:
- 🎯 BoxCall/Await 型情報登録（2-3日）
- 🎯 残り 4件の完全解決（67% → 100%）
- 🎯 if_phi.rs レガシー削除準備完了

**Phase 84 プロジェクトの最終ゴール**:
- 🎯 型推論システムの完全箱化
- 🎯 レガシーフォールバック根絶
- 🎯 保守性・拡張性・パフォーマンスの飛躍的向上

---

**次のアクション**: Phase 84-4 実装推奨ドキュメントを参照して、BoxCall/Await 型情報登録を開始してください。

**参考ドキュメント**:
- [Phase 84-4 実装推奨](phase84-4-implementation-recommendation.md)
- [Phase 84-3 残り 4件の完全調査](phase84-3-remaining-4-analysis.md)
- [Phase 84 インデックス](phase84-index.md)
