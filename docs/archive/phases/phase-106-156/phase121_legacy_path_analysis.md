# Phase 121: 旧 MIR/PHI 経路の特定

## 旧経路の定義

**旧 MIR/PHI 経路**とは：

- **JoinIR Lowering 前**の PHI 生成器を直接使用している経路
- **Phase 33 以前**の実装（If/Loop PHI 生成が直接 MirBuilder に組み込まれていた時期）
- Phase 33-10 で JoinIR If/Loop Lowering が実装されたが、MirBuilder への統合は未完

## 特定方法

### 1. ファイル名での特定

#### 旧経路候補

| ファイル | 状態 | 用途 |
|---------|------|------|
| `src/mir/builder/if_form.rs` | ✅ **現役** | 旧 If PHI 生成器（JoinIR 未統合） |
| `src/mir/builder/control_flow.rs` | ⚠️ **部分統合** | Loop PHI 生成器（Mainline のみ JoinIR） |
| `src/mir/loop_builder.rs` | ❓ **調査中** | 旧 Loop PHI 生成器（Phase 33-10 で削除済み？） |

#### JoinIR 経路

| ファイル | 状態 | 用途 |
|---------|------|------|
| `src/mir/join_ir/lowering/if_select.rs` | ✅ **実装済み** | JoinIR If Lowering |
| `src/mir/join_ir/lowering/loop_*.rs` | ✅ **実装済み** | JoinIR Loop Lowering |
| `src/mir/join_ir/frontend/mod.rs` | ✅ **実装済み** | AST → JoinIR 変換 |

### 2. 関数名での特定

#### 旧経路の特徴的関数

**If 文**（`src/mir/builder/if_form.rs`）:
```rust
// 旧 If PHI 生成器（Phase 121 時点で現役）
pub fn cf_if(
    &mut self,
    condition: &ASTNode,
    then_block: &ASTNode,
    else_block: Option<&ASTNode>,
) -> Result<ValueId, String> {
    // 旧 PHI 生成ロジック
    // - BasicBlock 直接操作
    // - PHI 命令を手動で挿入
    // - JoinIR Lowering を経由しない
}
```

**特徴**:
- `BasicBlock::phi()` を直接呼び出し
- `merge_phi_for_if()` 等のヘルパー関数使用
- JoinIR Lowering を経由しない

**Loop**（`src/mir/builder/control_flow.rs`）:
```rust
// Phase 49: 部分的に JoinIR 統合済み
pub fn cf_loop(
    &mut self,
    condition: &ASTNode,
    body: &ASTNode,
) -> Result<ValueId, String> {
    // Phase 49/80: Try JoinIR Frontend route for mainline targets
    if let Some(result) = self.try_cf_loop_joinir(&condition, &body)? {
        return Ok(result);
    }

    // Fallback: 旧 LoopBuilder
    self.cf_loop_legacy(condition, body)
}
```

**特徴**:
- `try_cf_loop_joinir()` で JoinIR 経由を試みる
- 失敗時は `cf_loop_legacy()` へフォールバック
- Mainline Targets のみ JoinIR 経由

#### JoinIR 経路の関数

**If Lowering**（`src/mir/join_ir/lowering/if_select.rs`）:
```rust
// JoinIR If Lowering（Phase 33-10 実装済み）
pub fn lower_if_to_mir(...) -> Result<ValueId, String> {
    // JoinIR ベースの If Lowering
    // - IfMerge/IfSelect 命令を生成
    // - PHI 命令を自動生成
}
```

**Loop Lowering**（`src/mir/join_ir/lowering/loop_*.rs`）:
```rust
// JoinIR Loop Lowering（Phase 33 実装済み）
pub fn lower_loop_to_mir(...) -> Result<ValueId, String> {
    // JoinIR ベースの Loop Lowering
    // - LoopForm を使用
    // - PHI 命令を自動生成
}
```

### 3. 環境変数での特定

#### 旧経路のフラグ

**Phase 121 調査結果**: 明示的な旧経路フラグは**存在しない**

**理由**:
- 旧経路がデフォルトのため、フラグで有効化する必要がない
- Phase 122 で `NYASH_LEGACY_PHI=1` を導入予定

#### JoinIR 経路のフラグ

| 環境変数 | 用途 | 実装箇所 |
|---------|-----|---------|
| `NYASH_JOINIR_STRICT=1` | フォールバック禁止（厳格モード） | `src/config/env.rs` |
| `NYASH_JOINIR_CORE=1` | JoinIR Core 有効化 | `src/config/env.rs` |
| `HAKO_JOINIR_PRINT_TOKENS_MAIN=1` | print_tokens を JoinIR 経由 | `src/mir/builder/control_flow.rs` |
| `HAKO_JOINIR_ARRAY_FILTER_MAIN=1` | ArrayExt.filter を JoinIR 経由 | `src/mir/builder/control_flow.rs` |

## hako_check での使用状況

### If 文の PHI 生成

**使用経路**: ❌ **旧経路**（`src/mir/builder/if_form.rs`）

**根拠**:

1. **ファイル**: `src/mir/builder/if_form.rs` が現役
2. **関数**: `cf_if()` が JoinIR Lowering を呼び出していない
3. **コード抜粋**:
```rust
// src/mir/builder/if_form.rs:L1-L50
pub fn cf_if(
    &mut self,
    condition: &ASTNode,
    then_block: &ASTNode,
    else_block: Option<&ASTNode>,
) -> Result<ValueId, String> {
    // 旧 PHI 生成ロジック（JoinIR Lowering 未使用）
    // ...
}
```

**問題点**:
- Phase 33-10 で JoinIR If Lowering が実装済み（`if_select.rs`）
- しかし MirBuilder の `cf_if()` はまだ JoinIR を呼び出していない
- hako_check で If 文を含むコードを解析する際、旧 PHI 生成器が使われる

### Loop の PHI 生成

**使用経路**: ⚠️ **混在**（Mainline Targets のみ JoinIR 経由）

**根拠**:

1. **ファイル**: `src/mir/builder/control_flow.rs`
2. **関数**: `cf_loop()` → `try_cf_loop_joinir()`
3. **コード抜粋**:
```rust
// src/mir/builder/control_flow.rs:L150-L200
pub fn cf_loop(
    &mut self,
    condition: &ASTNode,
    body: &ASTNode,
) -> Result<ValueId, String> {
    // Phase 49/80: Try JoinIR Frontend route for mainline targets
    if let Some(result) = self.try_cf_loop_joinir(&condition, &body)? {
        return Ok(result);
    }

    // Fallback: 旧 LoopBuilder
    self.cf_loop_legacy(condition, body)
}
```

**Mainline Targets** (JoinIR 経由):
- `JsonTokenizer.print_tokens/0`
- `ArrayExtBox.filter/2`

**その他の Loop** (旧経路):
- 上記以外のすべての Loop 文

## Phase 122+ での移行計画

### 移行必要箇所

#### 1. If 文の JoinIR 統合（最優先）

**ファイル**: `src/mir/builder/if_form.rs`

**現状**: ❌ 旧 PHI 生成器を使用中

**移行方法**:
```rust
// Phase 122: 環境変数で JoinIR 経路を選択可能に
pub fn cf_if(
    &mut self,
    condition: &ASTNode,
    then_block: &ASTNode,
    else_block: Option<&ASTNode>,
) -> Result<ValueId, String> {
    // Phase 122: 環境変数チェック
    if crate::config::env::env_bool("NYASH_HAKO_CHECK_JOINIR") {
        // JoinIR If Lowering を使用
        return self.cf_if_joinir(condition, then_block, else_block);
    }

    // 旧 PHI 生成器（互換性維持）
    self.cf_if_legacy(condition, then_block, else_block)
}

fn cf_if_joinir(
    &mut self,
    condition: &ASTNode,
    then_block: &ASTNode,
    else_block: Option<&ASTNode>,
) -> Result<ValueId, String> {
    // JoinIR If Lowering を呼び出す
    use crate::mir::join_ir::lowering::if_select::lower_if_to_mir;
    lower_if_to_mir(self, condition, then_block, else_block)
}
```

**実装ステップ**:
1. [ ] `cf_if_joinir()` 関数を追加（JoinIR Lowering 呼び出し）
2. [ ] `cf_if()` に環境変数チェックを追加
3. [ ] 既存ロジックを `cf_if_legacy()` に移動

#### 2. Loop の JoinIR 統合拡張（優先度中）

**ファイル**: `src/mir/builder/control_flow.rs`

**現状**: ⚠️ Mainline Targets のみ JoinIR 経由

**移行方法**:
```rust
// Phase 122: すべての Loop を JoinIR 経由に
pub fn cf_loop(
    &mut self,
    condition: &ASTNode,
    body: &ASTNode,
) -> Result<ValueId, String> {
    // Phase 122: 環境変数チェック
    if crate::config::env::env_bool("NYASH_HAKO_CHECK_JOINIR") {
        // すべての Loop を JoinIR 経由に
        if let Some(result) = self.try_cf_loop_joinir(&condition, &body)? {
            return Ok(result);
        }
    }

    // Phase 49: Mainline Targets のみ JoinIR 経由（既存ロジック）
    if let Some(result) = self.try_cf_loop_joinir(&condition, &body)? {
        return Ok(result);
    }

    // Fallback: 旧 LoopBuilder
    self.cf_loop_legacy(condition, body)
}
```

**実装ステップ**:
1. [ ] `try_cf_loop_joinir()` の Mainline Targets 制限を削除
2. [ ] 環境変数で制御するように変更
3. [ ] フォールバックロジックを整理

### 既に JoinIR 統合済み

#### 1. Loop Mainline Targets（Phase 49）

**ファイル**: `src/mir/builder/control_flow.rs`

**詳細**:
- ✅ `JsonTokenizer.print_tokens/0`: JoinIR 経由で動作確認済み
- ✅ `ArrayExtBox.filter/2`: JoinIR 経由で動作確認済み

**実装**:
```rust
// Phase 49: Mainline Integration
fn try_cf_loop_joinir(&mut self, condition: &ASTNode, body: &ASTNode) -> Result<Option<ValueId>, String> {
    let core_on = crate::config::env::joinir_core_enabled();

    // Mainline targets
    let mainline_targets = vec!["print_tokens", "filter"];

    if core_on && is_mainline_target(&func_name) {
        // JoinIR Frontend を使用
        return self.cf_loop_joinir_impl(condition, body, &func_name, debug);
    }

    Ok(None)
}
```

#### 2. JoinIR Frontend（Phase 49）

**ファイル**: `src/mir/join_ir/frontend/mod.rs`

**詳細**:
- ✅ `AstToJoinIrLowerer`: AST → JoinIR 変換
- ✅ JSON v0 → JoinIR 変換
- ✅ JoinModule 生成

## 旧経路と JoinIR 経路の比較

### If 文

| 項目 | 旧経路 | JoinIR 経路 |
|------|--------|------------|
| **ファイル** | `src/mir/builder/if_form.rs` | `src/mir/join_ir/lowering/if_select.rs` |
| **関数** | `cf_if()` | `lower_if_to_mir()` |
| **PHI 生成** | 手動（BasicBlock::phi()） | 自動（IfMerge/IfSelect） |
| **統合状況** | ❌ 現役使用中 | ✅ 実装済み（未統合） |
| **環境変数** | なし | `NYASH_HAKO_CHECK_JOINIR=1`（Phase 122） |

### Loop

| 項目 | 旧経路 | JoinIR 経路 |
|------|--------|------------|
| **ファイル** | `src/mir/builder/control_flow.rs` | `src/mir/join_ir/frontend/mod.rs` |
| **関数** | `cf_loop_legacy()` | `cf_loop_joinir_impl()` |
| **PHI 生成** | 手動（LoopBuilder） | 自動（LoopForm） |
| **統合状況** | ⚠️ フォールバック経路 | ⚠️ Mainline Targets のみ |
| **環境変数** | なし | `HAKO_JOINIR_*_MAIN=1` |

## コード削減見込み

### Phase 122 完了後

**削減箇所**:
- `src/mir/builder/if_form.rs`: 旧 PHI 生成ロジック削除可能（Phase 124）
- `src/mir/builder/control_flow.rs`: `cf_loop_legacy()` 削除可能（Phase 124）

**削減見込み**:
- `if_form.rs`: 約 200-300 行削減
- `control_flow.rs`: 約 100-150 行削減
- **合計**: 約 300-450 行削減

### Phase 124 完了後（旧経路完全削除）

**削減箇所**:
- 旧 PHI 生成ヘルパー関数削除
- フォールバックロジック削除
- 環境変数制御コード削除

**削減見込み**:
- **合計**: 約 500-600 行削減（MirBuilder 全体の約 5-10%）

## まとめ

hako_check 経路の旧 MIR/PHI 使用状況：

### ❌ **旧経路使用中**: 1箇所

1. **If 文の PHI 生成**（`src/mir/builder/if_form.rs`）
   - ファイル: `if_form.rs`
   - 関数: `cf_if()`
   - 根拠: JoinIR Lowering を呼び出していない

### ⚠️ **部分的統合**: 1箇所

1. **Loop の PHI 生成**（`src/mir/builder/control_flow.rs`）
   - ファイル: `control_flow.rs`
   - 関数: `cf_loop()` → `try_cf_loop_joinir()`
   - 根拠: Mainline Targets のみ JoinIR 経由、その他はフォールバック

### ✅ **JoinIR 統合済み**: 0箇所（完全統合は未完）

**Phase 122+ で段階的に JoinIR 統合を完了する。**

**最優先課題**: **If 文の JoinIR 統合**（`src/mir/builder/if_form.rs`）
Status: Historical
