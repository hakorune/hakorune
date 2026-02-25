# Phase 195-impl: Pattern3 複数キャリア対応（実装フェーズ）

**Status**: Ready for Implementation
**Date**: 2025-12-09
**Prerequisite**: Phase 195 design complete (phase195-pattern3-extension-design.md)

---

## 目的

**Pattern 3（If-Else PHI）で複数キャリア + 条件付き更新**を実装する。

**スコープ**:
- ✅ 複数キャリア（2-3個）の P3 処理
- ✅ ExitLine 拡張で対応（PhiGroupBox は作らない）
- ✅ if-sum パターン（sum + count）を通す
- ⏸️ _parse_string 簡易版（escaped のみ）は余力で

---

## Task 195-impl-1: Pattern3 lowerer の multi-carrier 対応

### 目標
Pattern3 lowerer を単一キャリア前提から**複数キャリア対応**に拡張する。

### 対象ファイル
- `src/mir/builder/control_flow/joinir/patterns/pattern3_with_if_phi.rs`

### 現状の実装（Phase 170-189）

**単一キャリア処理**:
```rust
// 既存: 単一キャリアのみ
fn lower_if_else_phi(
    if_node: &ASTNode,
    carrier_info: &CarrierInfo,
    // ...
) -> Result<ExitMeta, String> {
    // 1. Then/Else で carrier 更新式を取得
    let carrier_name = &carrier_info.carriers[0];  // ← 単一キャリア前提
    let then_update = extract_update(&if_node.then_branch, carrier_name)?;
    let else_update = extract_update(&if_node.else_branch, carrier_name)?;

    // 2. Then/Else ブロックで値を emit
    let then_value = emit_update(&then_update, ...)?;
    let else_value = emit_update(&else_update, ...)?;

    // 3. Merge 点で PHI 生成
    let phi_result = emit_phi(merge_block, then_value, else_value)?;

    // 4. ExitMeta に接続情報を載せる
    let exit_meta = ExitMeta {
        carrier_bindings: vec![(carrier_name.clone(), phi_result)],
        // ...
    };

    Ok(exit_meta)
}
```

### Phase 195-impl での拡張

**複数キャリア処理**:
```rust
// Phase 195: 複数キャリア対応
fn lower_if_else_phi(
    if_node: &ASTNode,
    carrier_info: &CarrierInfo,
    // ...
) -> Result<ExitMeta, String> {
    let mut carrier_bindings = Vec::new();

    // 1. 全キャリアについてループ処理
    for carrier_name in &carrier_info.carriers {
        // 2. Then/Else で carrier 更新式を取得
        let then_update = extract_update_or_unchanged(
            &if_node.then_branch,
            carrier_name,
            &previous_values,  // ← 更新なしの場合は前の値
        )?;
        let else_update = extract_update_or_unchanged(
            &if_node.else_branch,
            carrier_name,
            &previous_values,
        )?;

        // 3. Then/Else ブロックで値を emit
        let then_value = emit_update(&then_update, ...)?;
        let else_value = emit_update(&else_update, ...)?;

        // 4. Merge 点で PHI 生成（carrier ごとに1つ）
        let phi_result = emit_phi(merge_block, then_value, else_value)?;

        // 5. ExitMeta 用に保存
        carrier_bindings.push((carrier_name.clone(), phi_result));
    }

    // 6. ExitMeta に全キャリアの接続情報を載せる
    let exit_meta = ExitMeta {
        carrier_bindings,
        // ...
    };

    Ok(exit_meta)
}
```

### `extract_update_or_unchanged` 関数の追加

**目的**: 片方のブランチで更新がない場合、「前の値」を使用する。

```rust
fn extract_update_or_unchanged(
    branch: &[ASTNode],
    carrier_name: &str,
    previous_values: &HashMap<String, ValueId>,
) -> Result<UpdateExpr, String> {
    // Then/Else ブランチで carrier の Assign を探す
    if let Some(assign) = find_assign(branch, carrier_name) {
        // 更新あり: Assign の RHS を返す
        Ok(UpdateExpr::FromAST(assign.rhs.clone()))
    } else {
        // 更新なし: 前の値（ループ header の PHI param）を使用
        let prev_value = previous_values.get(carrier_name)
            .ok_or_else(|| format!("Carrier '{}' not in previous values", carrier_name))?;
        Ok(UpdateExpr::Unchanged(*prev_value))
    }
}

enum UpdateExpr {
    FromAST(Box<ASTNode>),   // 更新式あり
    Unchanged(ValueId),      // 更新なし（前の値使用）
}
```

### Fail-Fast 条件

**Phase 195-impl で弾くケース**:
1. **片方のブランチのみで carrier 定義**（明示的な更新なしも不可）:
   ```nyash
   if(cond) {
       sum = sum + i
       // count は更新なし（NG: 明示的に count = count が必要）
   } else {
       // sum も count も更新なし（NG）
   }
   ```
   → エラー: "Carrier 'count' not updated in both branches"

2. **複雑なネスト**:
   ```nyash
   if(cond1) {
       if(cond2) { sum = sum + i }
   }
   ```
   → エラー: "Nested if not supported in Phase 195"

### 実装手順

1. **`extract_update_or_unchanged` 関数追加** (~40 lines)
2. **`lower_if_else_phi` をループ処理に変更** (~30 lines 修正)
3. **`emit_update` で `UpdateExpr::Unchanged` 対応** (~10 lines)
4. **Fail-Fast エラーメッセージ追加** (~5 lines)

### ユニットテスト追加

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multi_carrier_if_else() {
        // CarrierInfo with 2 carriers: sum, count
        let carrier_info = CarrierInfo {
            carriers: vec!["sum".to_string(), "count".to_string()],
            updates: hashmap! {
                "sum" => ASTNode::BinOp { ... },
                "count" => ASTNode::BinOp { ... },
            },
        };

        // If-Else AST with updates for both carriers
        let if_node = create_if_else_ast(/* ... */);

        let result = lower_if_else_phi(&if_node, &carrier_info, ...);

        assert!(result.is_ok());
        let exit_meta = result.unwrap();
        assert_eq!(exit_meta.carrier_bindings.len(), 2);
    }

    #[test]
    fn test_unchanged_carrier_in_branch() {
        // If with update only in then branch
        let if_node = create_if_with_partial_update();

        // Should handle unchanged carrier (use previous value)
        let result = lower_if_else_phi(&if_node, &carrier_info, ...);

        assert!(result.is_ok());
        // Verify else branch uses previous value for unchanged carrier
    }
}
```

---

## Task 195-impl-2: ExitLine 拡張で複数キャリア PHI を扱う

### 目標
ExitLine 側で**複数 PHI を処理**する拡張を入れる。

### 対象ファイル
- `src/mir/builder/control_flow/joinir/exit_line/meta_collector.rs`
- `src/mir/builder/control_flow/joinir/exit_line/reconnector.rs`

### 現状の ExitLine（Phase 170-189）

**単一キャリア処理**:
```rust
// ExitMetaCollector
pub fn collect_exit_meta(
    pattern_result: &PatternResult,
) -> ExitMeta {
    ExitMeta {
        carrier_bindings: vec![
            (carrier_name, phi_dst)  // ← 単一キャリア
        ],
        // ...
    }
}

// ExitLineReconnector
pub fn reconnect(
    exit_meta: &ExitMeta,
    variable_map: &mut HashMap<String, ValueId>,
) {
    for (carrier_name, phi_dst) in &exit_meta.carrier_bindings {
        variable_map.insert(carrier_name.clone(), *phi_dst);
    }
}
```

### Phase 195-impl での拡張

**既に multi-carrier インフラあり**（CarrierVar.join_id, carrier_order）:
```rust
pub struct CarrierVar {
    pub name: String,
    pub join_id: ValueId,  // ← JoinIR 空間での ValueId
    // ...
}
```

**実装内容**:
1. **ExitMetaCollector**: 既に複数 carrier 対応（変更不要の可能性高い）
2. **ExitLineReconnector**: ループで全 carrier を処理（既存コードを確認）

### 実装確認手順

1. **ExitMetaCollector を確認**:
   ```rust
   // 既に複数 carrier_bindings を扱えているか確認
   pub fn collect_exit_meta(
       pattern_result: &PatternResult,
   ) -> ExitMeta {
       let mut carrier_bindings = Vec::new();

       // Pattern3 から渡ってくる全 carrier について
       for carrier_var in &pattern_result.carriers {
           carrier_bindings.push((
               carrier_var.name.clone(),
               carrier_var.join_id,
           ));
       }

       ExitMeta { carrier_bindings, ... }
   }
   ```

2. **ExitLineReconnector を確認**:
   ```rust
   // 既にループで全 carrier を処理しているか確認
   pub fn reconnect(
       exit_meta: &ExitMeta,
       variable_map: &mut HashMap<String, ValueId>,
   ) {
       for (carrier_name, phi_dst) in &exit_meta.carrier_bindings {
           // variable_map で carrier.name -> phi_dst を更新
           variable_map.insert(carrier_name.clone(), *phi_dst);
       }
   }
   ```

**想定**: 既存コードが既に複数対応している可能性が高い（Phase 170-189 の設計が良好）

**修正が必要な場合**: ループ処理を追加するだけ（~5 lines）

### ユニットテスト追加

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exit_line_multi_carrier() {
        let exit_meta = ExitMeta {
            carrier_bindings: vec![
                ("sum".to_string(), ValueId(10)),
                ("count".to_string(), ValueId(11)),
            ],
            // ...
        };

        let mut variable_map = HashMap::new();
        reconnect(&exit_meta, &mut variable_map);

        assert_eq!(variable_map.get("sum"), Some(&ValueId(10)));
        assert_eq!(variable_map.get("count"), Some(&ValueId(11)));
    }
}
```

---

## Task 195-impl-3: if-sum テスト（sum + count）で確認

### 目標
複数キャリア P3 が**実際に動作する**ことを E2E テストで確認。

### テストファイル

**ファイル**: `apps/tests/phase195_sum_count.hako`

```nyash
static box Main {
    main() {
        local sum = 0
        local count = 0
        local i = 0
        local len = 5

        loop(i < len) {
            if(i > 2) {
                sum = sum + i      // i=3,4 で加算
                count = count + 1
            }
            i = i + 1
        }

        // Expected: sum=7 (3+4), count=2
        local result = sum * 10 + count  // 72
        print(result)
        return 0
    }
}
```

### 実行手順

#### 1. ビルド
```bash
cargo build --release
```

#### 2. E2E 実行
```bash
# JoinIR 経路で実行
NYASH_JOINIR_CORE=1 ./target/release/hakorune apps/tests/phase195_sum_count.hako

# Expected output: 72
```

#### 3. Trace 確認
```bash
# JoinIR debug trace 有効化
NYASH_JOINIR_CORE=1 NYASH_JOINIR_DEBUG=1 ./target/release/hakorune apps/tests/phase195_sum_count.hako 2>&1 | grep "\[trace:joinir\]"

# Expected output (example):
# [trace:joinir] Pattern 3 applied: if-else with 2 carriers
# [trace:joinir] Carrier 'sum': PHI(%10, %11) -> %12
# [trace:joinir] Carrier 'count': PHI(%20, %21) -> %22

# CRITICAL: [joinir/freeze] が出ないこと
```

#### 4. 退行確認
```bash
# 既存の単一キャリア P3 テスト
./target/release/hakorune apps/tests/loop_if_phi.hako
# Expected: 既存の期待値

# Phase 190-194 テスト
./target/release/hakorune apps/tests/phase190_atoi_impl.hako
# Expected: 12

./target/release/hakorune apps/tests/phase191_body_local_atoi.hako
# Expected: 123

./target/release/hakorune apps/tests/phase193_init_method_call.hako
# Expected: RC:0
```

### 成功基準

- [ ] phase195_sum_count.hako が 72 を出力
- [ ] [joinir/freeze] が出ない（JoinIR 経路で動作）
- [ ] 既存テストが退行しない

---

## Task 195-impl-4: _parse_string 簡易版（escaped のみ）余力で

### 目標（オプション）
_parse_string の escaped フラグを P3 で扱う（buffer 連結は後回し）。

### テストファイル

**ファイル**: `apps/tests/phase195_flag_buffer.hako`

```nyash
static box Main {
    main() {
        local escaped = false
        local i = 0
        local len = 3

        loop(i < len) {
            if(i == 1) {
                escaped = true
            } else {
                escaped = false
            }
            i = i + 1
        }

        // Expected: escaped=false (最後に i=2 で false 代入)
        local result = 0
        if(escaped) {
            result = 1
        }
        print(result)  // Expected: 0
        return 0
    }
}
```

### 実行手順

```bash
# JoinIR 経路で実行
NYASH_JOINIR_CORE=1 ./target/release/hakorune apps/tests/phase195_flag_buffer.hako

# Expected output: 0
```

### 成功基準（余力で）

- [ ] phase195_flag_buffer.hako が 0 を出力
- [ ] P3 で BoolFlag（escaped）が動作
- [ ] buffer 連結は Phase 19x 後半で対応（今回は対象外）

---

## Task 195-impl-5: ドキュメント更新

### 1. phase195-pattern3-extension-design.md 更新

末尾に "Implementation Status" セクション追加:

```markdown
## Implementation Status

**完了日**: 2025-12-XX

### 実装サマリ

**対応パターン**:
- [x] 複数キャリア P3 処理（2-3個）
- [x] if-sum パターン（sum + count） - phase195_sum_count.hako
- [ ] _parse_string 簡易版（escaped のみ）- phase195_flag_buffer.hako（オプション）

### 実装内容

**ファイル変更**:
1. `pattern3_with_if_phi.rs` (+60 lines)
   - `extract_update_or_unchanged` 関数追加
   - `lower_if_else_phi` を複数キャリア対応に拡張
   - Fail-Fast 条件追加

2. `exit_line/meta_collector.rs`, `exit_line/reconnector.rs` (確認のみ)
   - 既に複数 carrier_bindings 対応済み（変更不要）

### E2E テスト結果

**phase195_sum_count.hako**:
```bash
NYASH_JOINIR_CORE=1 ./target/release/hakorune apps/tests/phase195_sum_count.hako
# Output: 72 ✅
```

**退行テスト**:
- loop_if_phi.hako: PASS ✅
- phase190-194 tests: ALL PASS ✅

### 技術的発見

1. **ExitLine は既に multi-carrier 対応**:
   - CarrierVar.join_id インフラが Phase 170-189 で整備済み
   - 新規実装不要、Pattern3 側の拡張のみで動作

2. **Unchanged carrier の扱い**:
   - 片方のブランチで更新なし → 前の値（ループ header PHI param）を使用
   - `UpdateExpr::Unchanged(ValueId)` で表現

3. **Fail-Fast 効果**:
   - 両ブランチで carrier 定義必須（明示的エラー）
   - ネストした if は Phase 196+ に延期

### 制限事項（Phase 195-impl）

- ❌ ネストした if: `if(c1) { if(c2) { ... } }`
- ❌ 3個以上の carrier（設計は対応済みだが、テスト未実施）
- ❌ LoopBodyLocal + MethodCall 混在は Phase 195+ 延期

### 次のステップ

**Phase 196+**: 候補
- Pattern 3 のネスト対応
- 3個以上の carrier での動作検証
- _parse_string 完全版（buffer 連結 + escaped flag 統合）

**Phase 200+**: ConditionEnv 拡張
- function-scoped variables サポート
- _parse_number, _atoi が動作可能に
```

### 2. CURRENT_TASK.md 更新

```markdown
## Phase 195-impl: Pattern3 複数キャリア対応（完了: 2025-12-XX）

**目的**: P3（If-Else PHI）で複数キャリア + 条件付き更新を実装

**実装内容**:
- ✅ Pattern3 lowerer 拡張（複数キャリアループ処理）
- ✅ ExitLine 確認（既に multi-carrier 対応済み）
- ✅ if-sum テスト成功（sum + count） - phase195_sum_count.hako → 72 ✅
- ⏸️ _parse_string 簡易版（escaped のみ）- 余力により実施

**成果**:
- P3 が「単一キャリア専用」から「複数キャリア対応」に昇格
- JsonParser/selfhost で if-in-loop パターンを拡張可能に
- 既存テスト退行なし

**技術的発見**:
- ExitLine は既に multi-carrier 対応（CarrierVar.join_id インフラ活用）
- Unchanged carrier は前の値（ループ header PHI param）使用
- Fail-Fast で複雑な分岐を明示的に弾く

**次のステップ**: Phase 196+（Pattern 3 ネスト対応）or Phase 200+（ConditionEnv 拡張）
```

### 3. joinir-architecture-overview.md 更新

Section 7.2 の Phase 195 を完了マークに更新:

```markdown
- [x] **Phase 195**: Pattern 3 拡張（複数キャリア対応）
  - P3 で 2-3 個の Carrier を同時処理可能に
  - ExitLine 拡張で複数 PHI 生成（既存インフラ活用）
  - if-sum パターン（sum+count）動作確認完了
  - JsonParser カバレッジ向上への基盤完成
```

---

## 成功基準

- [x] Pattern3 lowerer が複数キャリア対応（extract_update_or_unchanged 実装）
- [x] ExitLine が複数 PHI を処理（既存確認 or 拡張）
- [x] phase195_sum_count.hako が期待値（72）を出力
- [x] [joinir/freeze] が出ない（JoinIR 経路で動作）
- [x] 既存テスト（phase190-194, loop_if_phi）が退行しない
- [x] ドキュメント更新（Implementation Status, CURRENT_TASK, overview）

---

## 関連ファイル

### 実装対象
- `src/mir/builder/control_flow/joinir/patterns/pattern3_with_if_phi.rs`（主要実装）
- `src/mir/builder/control_flow/joinir/exit_line/meta_collector.rs`（確認のみ）
- `src/mir/builder/control_flow/joinir/exit_line/reconnector.rs`（確認のみ）

### テストファイル
- `apps/tests/phase195_sum_count.hako`（新規作成・必須）
- `apps/tests/phase195_flag_buffer.hako`（新規作成・オプション）
- `apps/tests/loop_if_phi.hako`（退行確認）

### ドキュメント
- `docs/development/current/main/phase195-pattern3-extension-design.md`（Implementation Status 追加）
- `docs/development/current/main/joinir-architecture-overview.md`（Phase 195 完了マーク）
- `CURRENT_TASK.md`（Phase 195-impl 完了記録）

---

## 設計原則（Phase 195-impl）

1. **既存インフラ活用**:
   - ExitLine の CarrierVar.join_id を再利用
   - 新規箱なし（PhiGroupBox は作らない）

2. **段階的実装**:
   - まず if-sum で基盤確認
   - 次に _parse_string 簡易版（余力で）

3. **Fail-Fast 継承**:
   - 両ブランチで carrier 定義必須
   - 複雑な分岐は明示的に弾く

4. **箱理論の実践**:
   - 設計書に基づく実装
   - 単一責任の原則維持
   - ドキュメント駆動開発
Status: Historical
