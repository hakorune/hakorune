# Phase 33-18: continue+if/else ループパターン設計フェーズ

**Goal**: 「if (cond) { … } else { continue }」型のループを JoinIR で扱う方法を箱理論ベースで設計する

---

## Task 33-18-1: continue+if/else パターンのインベントリ

### 検出方法
- `rg "continue" apps/tests/ tools/selfhost/ --glob "*.hako"`

### パターン一覧表

| ファイル | ループ条件 | continue位置 | if構造 | carrier数 | 更新式 |
|---------|-----------|-------------|--------|----------|--------|
| **Pattern A: if (cond) { continue } - then側continue** |||||||
| `loop_continue_pattern4.hako` | `i < 10` | then | `if (i % 2 == 0) { continue }` | 2 (i, sum) | `i = i + 1`, `sum = sum + i` |
| `test_pattern4_simple_continue.hako` | `i < n` | then | `if is_even == 1 { continue }` | 3 (i, sum, is_even) | `i = i + 1`, `sum = sum + i` |
| `parser_box_minimal.hako:skip_ws` | `i < n` | then | `if ch == " " \|\| ... { continue }` | 1 (i) | `i = i + 1` |
| `llvm_phi_mix.hako` | `i < 10` | then | `if (i == 2 \|\| i == 4) { continue }` | 2 (i, sum) | 条件付き更新 |
| `llvm_stage3_break_continue.hako` | `i < 10` | then | `if (i < 5) { continue }` | 1 (i) | `i = i + 1` |
| **Pattern B: if (cond) { ... } else { continue } - else側continue** |||||||
| `loop_if_phi_continue.hako` | `i < 6` | else | `if (i % 2 == 0) { i++; printed++; continue } else { i+=2 }` | 2 (i, printed) | 両分岐で更新 |
| `失敗テスト（mirbuilder...）` | `i < 5` | else | `if (i != M) { sum += i } else { continue }` | 3 (i, s, M) | then側のみ更新 |
| **Pattern C: 複雑パターン（nested/mixed）** |||||||
| `loopform_continue_break_scan.hako` | `true` | then | continue + break 混在 | 2 (i, sum) | 複数分岐 |
| `try_finally_continue_inner_loop.hako` | `j < 3` | then | `if (j == 1) { mark = 1; continue }` | 2 (j, mark) | try/finally内 |
| `nested_loop_inner_continue_isolated.hako` | `j < 3` | then | `if (j == 1) { continue }` | 1 (j) | 内側ループ |

### パターン分類

#### Pattern A: then側continue（単純）
```nyash
loop(cond) {
    if (skip_condition) {
        i = i + 1
        continue
    }
    // main processing
    i = i + 1
}
```
- **特徴**: continue が条件成立時に実行される「スキップ」パターン
- **既存対応**: Pattern4 で処理可能な形式
- **問題なし**: 現在動作している

#### Pattern B: else側continue（問題あり）
```nyash
loop(cond) {
    if (process_condition) {
        // main processing
    } else {
        continue
    }
    i = i + 1
}
```
- **特徴**: continue が条件不成立時に実行される
- **論理的同等**: `if (!process_condition) { continue } else { ... }` と等価
- **問題**: 現在 JoinIR では対応できず失敗する
- **失敗例**: `mirbuilder_loop_varvar_ne_else_continue_desc_core_exec_canary_vm`

---

## Task 33-18-2: LoopFeatures / PatternKind から見た分類

### 現在の classify() ロジック

```rust
pub fn classify(features: &LoopFeatures) -> LoopPatternKind {
    // Pattern 4: Continue (highest priority)
    if features.has_continue {
        return LoopPatternKind::Pattern4Continue;
    }
    // ...
}
```

**問題点**: `has_continue == true` だけで Pattern4 に分類するが、
- Pattern B（else側continue）は if-else 構造を持つ
- `has_if_else_phi == true` と `has_continue == true` が同時に成立する可能性
- 現在のロジックでは continue 優先のため、Pattern4 に分類されるが lowering できない

### 設計案

#### 案 A: Pattern4 に統合（BoolExprLowerer で正規化）

**アイデア**:
- `if (!cond) { ... } else { continue }` を `if (cond) { continue } else { ... }` に変換
- BoolExprLowerer に「条件反転 + 分岐入れ替え」ロジックを追加
- Pattern4 lowerer はそのまま使える

**メリット**:
- 新しい Pattern を追加しなくて良い
- 既存の Pattern4 lowerer を再利用
- 箱の数が増えない

**デメリット**:
- BoolExprLowerer の責務が増える
- 反転ロジックが複雑になる可能性

#### 案 B: 新規 Pattern5 として独立

**アイデア**:
- `Pattern5ContinueIfElse` を新設
- `has_continue && has_if_else_phi` の組み合わせを検出
- 専用 lowerer を実装

**メリット**:
- 責務が明確に分離
- Pattern4 と独立して実装・テスト可能

**デメリット**:
- 新しい箱が増える
- 重複コードが発生する可能性

### 選択基準

| 基準 | 案 A (統合) | 案 B (新設) |
|-----|------------|------------|
| 箱の数 | 増えない | +1 (Pattern5) |
| 既存コード変更 | BoolExprLowerer | classify() のみ |
| 実装難易度 | 中（反転ロジック） | 中（新規lowerer） |
| テスト容易性 | 既存テスト再利用 | 新規テスト必要 |

**推奨**: **案 A（Pattern4 統合）**
- 理由: `if (cond) { continue }` と `if (!cond) { ... } else { continue }` は論理的に同型
- 「continue がどちらの分岐にあっても、最終的に同じ CFG 骨格になる」ことを活用

---

## Task 33-18-3: JoinIR 箱との責務マッピング

### 既存箱との関係

| 箱 | 現在の責務 | Pattern B での役割 |
|---|-----------|------------------|
| **LoopFeatures** | break/continue/if_else_phi 検出 | 変更なし（情報収集のみ） |
| **classify()** | Pattern 1-4 振り分け | 案Aなら変更なし |
| **BoolExprLowerer** | 条件式の SSA 化 | **拡張**: continue 分岐の正規化 |
| **Pattern4 lowerer** | continue ブロック生成 | 変更なし |
| **Header PHI** | ループヘッダの PHI 生成 | 変更なし |
| **ExitLine** | carrier / expr 出口処理 | 変更なし |

### 変更が必要な箇所

1. **BoolExprLowerer** (or 新規 ContinueBranchNormalizer Box)
   - `if (cond) { ... } else { continue }` を検出
   - `if (!cond) { continue } else { ... }` に変換
   - 変換後の AST を Pattern4 lowerer に渡す

2. **router.rs** (optional)
   - else 側 continue の検出を追加
   - BoolExprLowerer への委譲を追加

### joinir-architecture-overview.md への追記案

```markdown
### Continue パターンの分類ルール (Phase 33-18)

- Pattern4_WithContinue は以下の条件で適用:
  - `has_continue == true` AND `has_break == false`
  - continue の位置（then/else）は問わない（正規化で吸収）

- else 側 continue の処理:
  - BoolExprLowerer で条件反転 → then 側 continue 形式に正規化
  - 正規化後は通常の Pattern4 として処理
```

---

## Task 33-18-4: 完了条件と次フェーズへの橋渡し

### Phase 33-18 完了条件チェックリスト

- [x] continue+if/else パターンのインベントリが docs に揃っている
- [x] Pattern4 に畳めるか／Pattern5 新設かの方針が決まっている（案 A: 統合）
- [x] JoinIR の箱たち（Features / BoolExprLowerer / Header PHI / ExitLine）のどこを触るかが決まっている
- [ ] 実装フェーズ（33-19）のタスクリストが 3〜5 個に落ちている

### 実装フェーズ (Phase 33-19) タスクリスト案

1. **Task 33-19-1**: ContinueBranchNormalizer Box 作成
   - else 側 continue を then 側に移動する AST 変換
   - 単体テスト付き

2. **Task 33-19-2**: router.rs への統合
   - Pattern B 検出時に正規化を呼び出す
   - 正規化後 Pattern4 lowerer に委譲

3. **Task 33-19-3**: 失敗テスト修正
   - `mirbuilder_loop_varvar_ne_else_continue_desc_core_exec_canary_vm` が PASS になることを確認

4. **Task 33-19-4**: 追加スモークテスト
   - Pattern B の各バリエーション（単一carrier、複数carrier）

5. **Task 33-19-5**: ドキュメント更新
   - joinir-architecture-overview.md に正式追記

---

## 備考

- 失敗テストの直接原因は「JoinIR does not support this pattern」エラー
- LoopBuilder は既に削除されているため、JoinIR での対応が必須
- CFG reachability の問題も別途あり（Rust CLI 経由では MIR 生成されるが reachable=false）

**作成日**: 2025-12-07
**Phase**: 33-18 (Design Only)
Status: Historical
