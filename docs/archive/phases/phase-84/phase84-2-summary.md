# Phase 84-2: CopyTypePropagator 実装完了サマリー

## 成果

### 削減結果

```
Before Phase 84-2: 12件の Case D 失敗
After Phase 84-2:   9件の Case D 失敗

削減数: 3件 (25%削減)
```

### 実装内容

**新規作成ファイル**:
- `src/mir/phi_core/copy_type_propagator.rs` (125行)

**主要機能**:
```rust
pub struct CopyTypePropagator;

impl CopyTypePropagator {
    pub fn propagate(
        function: &MirFunction,
        types: &mut BTreeMap<ValueId, MirType>,
    ) {
        // Copy 命令を走査して型を伝播
        for inst in all_copy_instructions(function) {
            if let Some(src_type) = types.get(&src) {
                types.insert(dst, src_type.clone());
            }
        }
    }
}
```

**統合箇所**:
- `src/mir/builder/lifecycle.rs:371` - infer_type_from_phi() の直前に呼び出し

## 残り 9件の分類

### GroupA: Loop 制御フロー PHI（7件）

**パターン**: Loop + continue/break による Edge Copy 合流

**典型的なコード**:
```hako
i = 0
sum = 0
loop(i < 5) {
  i = i + 1
  if (i == 3) { break }    // → edge copy
  if (i % 2 == 0) { continue } // → edge copy
  sum = sum + i
}
return sum // ← PHI の型が未解決
```

**問題**: PHI の incoming 値が Edge Copy の dst で、src の型を遡れない

**テスト一覧**:
1. `loop_with_continue_and_break_edge_copy_merge` - ValueId(56)
2. `nested_loop_with_multi_continue_break_edge_copy_merge` - ValueId(135)
3. `loop_inner_if_multilevel_edge_copy` - ValueId(74)
4. `loop_break_and_early_return_edge_copy` - ValueId(40)
5. `vm_exec_break_inside_if` - ValueId(27)
6. `loop_if_three_level_merge_edge_copy` - ValueId(75)
7. （GroupA 合計 7件）

### GroupB: 多段 PHI 型推論（2件）

**パターン**: 複数の PHI 命令が連鎖

**典型的なコード**:
```hako
static box Stage1Cli {
  stage1_main(args) {
    if cond1 { return 96 }
    if cond2 { return 96 }
    return 0
  }
}

static box Main {
  main(args) {
    return Stage1Cli.stage1_main(args) // ← 多段 PHI
  }
}
```

**問題**: PHI の incoming 値が別の PHI で、再帰的に解決できない

**テスト一覧**:
1. `mir_stage1_cli_emit_program_min_exec_hits_type_error` - ValueId(7)
2. `mir_stage1_cli_emit_program_min_compiles_and_verifies` - ValueId(7)

### GroupC: await 特殊パターン（1件）

**パターン**: await 式の MIR lowering

**コード**:
```rust
let ast = ASTNode::AwaitExpression {
    expression: Box::new(ASTNode::Literal {
        value: LiteralValue::Integer(1),
        ...
    }),
    ...
};
```

**問題**: await の型推論が未実装（非同期システム未完成）

**テスト**:
1. `test_lowering_await_expression` - ValueId(2)

## Phase 84-3 実装推奨

### 目標: GroupA の 7件を解決

**新機能**: Edge Copy 追跡 PHI 型推論

**実装方針**:
```rust
// GenericTypeResolver に追加
pub fn resolve_from_phi_with_copy_trace(
    function: &MirFunction,
    ret_val: ValueId,
    types: &BTreeMap<ValueId, MirType>,
) -> Option<MirType> {
    // PHI の incoming 値から Copy を遡る
    for (_, incoming_val) in phi_inputs {
        // 1. 直接型取得を試みる
        if let Some(ty) = types.get(incoming_val) {
            continue;
        }

        // 2. Copy 命令を遡る
        if let Some(src) = find_copy_src(function, incoming_val) {
            if let Some(ty) = types.get(&src) {
                // src の型を使用
            }
        }

        // 3. 多段 Copy を再帰的に追跡
        if let Some(ty) = trace_copy_chain(function, incoming_val, types) {
            // チェーンを遡った型を使用
        }
    }
}
```

**期待効果**:
- 9件 → 2件に削減（GroupB + GroupC のみ残存）
- Loop 制御フローの型推論が完全動作

### 実装ファイル

- `src/mir/join_ir/lowering/generic_type_resolver.rs` - 新関数追加
- `src/mir/builder/lifecycle.rs` - 新関数呼び出し統合

### テスト検証

```bash
# Phase 84-3 完了確認
NYASH_PHI_FALLBACK_DISABLED=1 cargo test --release --lib 2>&1 | grep "Case D" | wc -l
# 期待: 2 (GroupB のみ)
```

## Phase 84-4/5 展望

### Phase 84-4: 多段 PHI 型推論

**目標**: GroupB の 2件を解決

**実装**: `resolve_from_phi_recursive()` で PHI チェーンを再帰的に追跡

### Phase 84-5: await 暫定対応

**目標**: GroupC の 1件を解決

**実装**: lifecycle.rs に await 特殊ケース追加（暫定）

## 完了条件

```bash
# 全 Case D 解決
NYASH_PHI_FALLBACK_DISABLED=1 cargo test --release --lib 2>&1 | grep "Case D"
# 期待: 出力なし（0件）

# 最終確認
cargo test --release --lib
# 期待: test result: ok
```

## タイムライン

- **Phase 84-2**: 完了 ✅ (12件 → 9件)
- **Phase 84-3**: 推定 1-2日 (9件 → 2件)
- **Phase 84-4**: 推定 1-2日 (2件 → 1件)
- **Phase 84-5**: 推定 30分 (1件 → 0件)

**合計**: 2-4日で Case D 完全解決見込み

## 参考資料

- [Phase 84-2 詳細調査](./phase84-2-case-d-investigation.md)
- [CopyTypePropagator 実装](../../../src/mir/phi_core/copy_type_propagator.rs)
- [GenericTypeResolver](../../../src/mir/join_ir/lowering/generic_type_resolver.rs)
- [lifecycle.rs 統合箇所](../../../src/mir/builder/lifecycle.rs:371)
Status: Historical
