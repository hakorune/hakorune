# Phase 84-2: Case D 失敗パターン詳細

## GroupA: Loop 制御フロー PHI（7件）

### パターン A-1: continue + break

**テスト**: `loop_with_continue_and_break_edge_copy_merge`
**ValueId**: 56

**コード構造**:
```hako
i = 0
sum = 0
loop(i < 5) {
  i = i + 1
  if (i == 3) { break }       // ← break 経路
  if (i % 2 == 0) { continue } // ← continue 経路
  sum = sum + i                // ← normal 経路
}
return sum // ← ValueId(56) の型が未解決
```

**MIR 構造（推測）**:
```
Block_LoopHeader:
  %sum_phi = PHI [%sum_init, %sum_continue, %sum_normal]
  %i_phi = PHI [%i_init, %i_continue, %i_normal]
  ...

Block_Break:
  %sum_break = Copy %sum_phi  ← value_types に型登録済み
  Jump after_loop

Block_Continue:
  %sum_continue = Copy %sum_phi ← value_types に型登録済み
  Jump loop_header

Block_Normal:
  %sum_normal = BinOp Add %sum_phi %i_phi
  Jump loop_header

Block_AfterLoop:
  %56 = PHI [%sum_break]      ← incoming の型が未登録！
  Return %56
```

**問題の本質**:
- PHI(56) の incoming 値は `%sum_break`
- `%sum_break` は Copy の dst で、value_types に未登録
- Copy の src `%sum_phi` は型登録済みだが、GenericTypeResolver は追跡しない

**解決策**:
```rust
// GenericTypeResolver::resolve_from_phi_with_copy_trace()
for (_, incoming_val) in phi_inputs {
    // Copy を遡る
    if let Some(src) = find_copy_src(function, incoming_val) {
        if let Some(ty) = types.get(&src) {
            return Some(ty.clone());
        }
    }
}
```

### パターン A-2: 多段 continue/break

**テスト**: `nested_loop_with_multi_continue_break_edge_copy_merge`
**ValueId**: 135

**コード構造**:
```hako
i = 0
sum = 0
loop(i < 10) {
  i = i + 1
  if (i == 2 || i == 4) { continue }   // ← continue 経路1
  if (i == 7) {
    if (1 == 1) { break }              // ← break 経路（ネスト）
  }
  if ((i % 3) == 0) { continue }       // ← continue 経路2
  sum = sum + i                        // ← normal 経路
}
return sum // ← ValueId(135)
```

**特徴**:
- 複数の continue 経路
- ネストした if 内の break
- PHI の incoming 値が多い（4-5個）

### パターン A-3: Loop + 多段 if

**テスト**: `loop_inner_if_multilevel_edge_copy`
**ValueId**: 74

**コード構造**:
```hako
j = 0
acc = 0
loop(j < 6) {
  j = j + 1
  if (j < 3) {
    if (j % 2 == 0) { continue }     // ← 2段 if + continue
    acc = acc + 10
  } else {
    if (j == 5) { break }            // ← 2段 if + break
    acc = acc + 1
  }
}
return acc // ← ValueId(74)
```

**特徴**:
- then/else の両方に制御フロー
- 多段ネスト if
- 変数更新が複数経路に分散

### パターン A-4: Loop + early return

**テスト**: `loop_break_and_early_return_edge_copy`
**ValueId**: 40

**コード構造**:
```hako
i = 0
acc = 0
loop(i < 6) {
  i = i + 1
  if (i == 5) { break }              // ← break 経路
  if (i == 3) { return acc }         // ← early return 経路
  acc = acc + i
}
return acc // ← ValueId(40)
```

**特徴**:
- break と early return の混在
- 関数終了が複数経路
- return 型推論が複雑

### パターン A-5: 単純 if-break

**テスト**: `vm_exec_break_inside_if`
**ValueId**: 27

**コード構造**:
```hako
local i = 0
loop(i < 10) {
  if (i == 3) { break }              // ← if 内 break
  i = i + 1
}
return i // ← ValueId(27)
```

**特徴**:
- 最もシンプルな if-break パターン
- これが解決できればベースケース成功

### パターン A-6: 3段ネスト if

**テスト**: `loop_if_three_level_merge_edge_copy`
**ValueId**: 75

**コード構造**:
```hako
x = 0
i = 0
loop(i < 7) {
  i = i + 1
  if (i % 2 == 0) {
    if (i == 4) { continue }         // ← 3段目 continue
    x = x + 2
  } else {
    if (i == 5) { break }            // ← 3段目 break
    x = x + 1
  }
}
return x // ← ValueId(75)
```

**特徴**:
- 3段ネスト制御フロー
- then/else の両方に制御フロー
- 変数更新の分岐が複雑

## GroupB: 多段 PHI 型推論（2件）

### パターン B-1: static box + 複数 return

**テスト**: `mir_stage1_cli_emit_program_min_*`
**ValueId**: 7

**コード構造**:
```hako
static box Stage1Cli {
  emit_program_json(source) {
    if source == null || source == "" { return null }  // ← return 経路1
    return "{prog:" + source + "}"                     // ← return 経路2
  }

  stage1_main(args) {
    if args == null { args = new ArrayBox() }
    local src = env.get("STAGE1_SOURCE")
    if src == null || src == "" { return 96 }          // ← return 経路3

    local prog = me.emit_program_json(src)
    if prog == null { return 96 }                      // ← return 経路4
    print(prog)
    return 0                                           // ← return 経路5
  }
}

static box Main {
  main(args) {
    env.set("STAGE1_SOURCE", "apps/tests/stage1_using_minimal.hako")
    return Stage1Cli.stage1_main(args) // ← ValueId(7)
  }
}
```

**MIR 構造（推測）**:
```
Function: Stage1Cli.emit_program_json
  Block1:
    %1 = PHI [null, string_concat_result]
    Return %1

Function: Stage1Cli.stage1_main
  Block1:
    %2 = Call Stage1Cli.emit_program_json
  Block2:
    %3 = PHI [%2, const_96, const_0]
    Return %3

Function: Main.main
  Block1:
    %4 = Call Stage1Cli.stage1_main
  Block2:
    %7 = PHI [%4]  ← %4 の型が未解決（多段 PHI）
    Return %7
```

**問題の本質**:
- PHI(7) の incoming 値は PHI(3) の結果
- PHI(3) の incoming 値は PHI(1) の結果
- **3段 PHI チェーン** が発生

**解決策**:
```rust
// GenericTypeResolver::resolve_from_phi_recursive()
pub fn resolve_from_phi_recursive(
    function: &MirFunction,
    ret_val: ValueId,
    types: &BTreeMap<ValueId, MirType>,
    visited: &mut HashSet<ValueId>,
) -> Option<MirType> {
    if visited.contains(&ret_val) {
        return None; // 循環検出
    }
    visited.insert(ret_val);

    // PHI の incoming 値を再帰的に解決
    for (_, incoming_val) in phi_inputs {
        if let Some(ty) = resolve_from_phi_recursive(
            function, *incoming_val, types, visited
        ) {
            return Some(ty);
        }
    }
    None
}
```

## GroupC: await 特殊パターン（1件）

### パターン C-1: await 式

**テスト**: `test_lowering_await_expression`
**ValueId**: 2

**コード構造**:
```rust
// Rust AST 生成
let ast = ASTNode::AwaitExpression {
    expression: Box::new(ASTNode::Literal {
        value: LiteralValue::Integer(1),
        span: crate::ast::Span::unknown(),
    }),
    span: crate::ast::Span::unknown(),
};
```

**MIR 構造（推測）**:
```
Block1:
  %1 = Const Integer(1)
  %2 = Await %1           ← await 命令の戻り値
  Return %2
```

**問題の本質**:
- await 式の型推論が未実装
- 非同期システム（Safepoint/Checkpoint）が Phase 67+ 実装予定
- 現在は MIR13 migration pending

**暫定対応**:
```rust
// lifecycle.rs に特殊ケース追加
if is_await_expression {
    // await の戻り値型は Unknown で許容
    return MirType::Unknown;
}
```

**長期対応（Phase 67+）**:
- async/await システム完全実装
- type_hint による await 型推論
- Safepoint/Checkpoint 命令統合

## 解決優先度

### 優先度1: GroupA（7件）

**理由**:
- 最も頻出するパターン
- Loop 制御フローは実用コードで必須
- Edge Copy 追跡で一気に解決可能

**期待効果**: 9件 → 2件（78%削減）

### 優先度2: GroupB（2件）

**理由**:
- static box は Stage1Cli で使用中
- 多段メソッド呼び出しも実用的
- 再帰的 PHI 推論で解決可能

**期待効果**: 2件 → 1件（50%削減）

### 優先度3: GroupC（1件）

**理由**:
- await は実験的機能
- 本格実装は Phase 67+ 予定
- 暫定対応で十分

**期待効果**: 1件 → 0件（100%削減）

## 実装チェックリスト

### Phase 84-3: Edge Copy 追跡

- [ ] `GenericTypeResolver::resolve_from_phi_with_copy_trace()` 実装
- [ ] `find_copy_src()` ヘルパー関数実装
- [ ] `trace_copy_chain()` ヘルパー関数実装
- [ ] lifecycle.rs 統合
- [ ] テスト実行: GroupA の 7件を確認

### Phase 84-4: 多段 PHI 推論

- [ ] `GenericTypeResolver::resolve_from_phi_recursive()` 実装
- [ ] 循環検出ロジック実装
- [ ] lifecycle.rs 統合
- [ ] テスト実行: GroupB の 2件を確認

### Phase 84-5: await 暫定対応

- [ ] lifecycle.rs に await 特殊ケース追加
- [ ] テスト実行: GroupC の 1件を確認

## 完了確認

```bash
# Phase 84-3 完了
NYASH_PHI_FALLBACK_DISABLED=1 cargo test --release --lib 2>&1 | grep "Case D" | wc -l
# 期待: 2

# Phase 84-4 完了
NYASH_PHI_FALLBACK_DISABLED=1 cargo test --release --lib 2>&1 | grep "Case D" | wc -l
# 期待: 1

# Phase 84-5 完了
NYASH_PHI_FALLBACK_DISABLED=1 cargo test --release --lib 2>&1 | grep "Case D"
# 期待: 出力なし（0件）

# 最終確認
cargo test --release --lib
# 期待: test result: ok
```
Status: Historical
