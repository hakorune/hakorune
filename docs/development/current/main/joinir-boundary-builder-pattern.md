# JoinInlineBoundaryBuilder 使用パターン（Phase 201）

## Pattern2 の Canonical 形式

Pattern2 で確立した Builder 使用パターンを他の Pattern に適用する際の基準。

### Builder 構築順序

```rust
let boundary = JoinInlineBoundaryBuilder::new()
    .with_inputs(join_inputs, host_inputs)
    .with_condition_bindings(condition_bindings)
    .with_exit_bindings(exit_bindings)
    .with_loop_var_name(Some(loop_var_name.clone()))
    .with_expr_result(fragment_meta.expr_result)
    .build();
```

### 各メソッドの意味

1. **with_inputs(join_inputs, host_inputs)**
   - ループパラメータの橋渡し
   - join_inputs: JoinIR 側の ValueId リスト
   - host_inputs: MIR 側の ValueId リスト

2. **with_condition_bindings(bindings)**
   - 条件式専用変数の橋渡し
   - ConditionBinding のリスト（変数名 + JoinIR ValueId のペア）

3. **with_exit_bindings(bindings)**
   - キャリア出口の橋渡し
   - (変数名, JoinIR ValueId) のペア

4. **with_loop_var_name(name)**
   - ループ変数名（LoopHeader PHI 生成用）
   - Some(変数名) または None

5. **with_expr_result(expr)**
   - ループが式として返す値
   - Some(ValueId) または None

### フィールド直書き禁止

- ❌ `boundary.loop_var_name = ...;` 等の直接代入は禁止
- ✅ Builder の fluent API のみ使用

### Pattern別の使用パターン

#### Pattern2 (with break)
```rust
let boundary = JoinInlineBoundaryBuilder::new()
    .with_inputs(vec![ValueId(0)], vec![loop_var_id])
    .with_condition_bindings(condition_bindings)
    .with_exit_bindings(exit_bindings)
    .with_loop_var_name(Some(loop_var_name.clone()))
    .with_expr_result(fragment_meta.expr_result)
    .build();
```

#### Pattern3 (with if-phi)
```rust
let boundary = JoinInlineBoundaryBuilder::new()
    .with_inputs(
        vec![ValueId(0), ValueId(1)],      // i, sum
        vec![loop_var_id, sum_var_id],
    )
    .with_exit_bindings(vec![
        ("sum".to_string(), ValueId(18))
    ])
    .with_loop_var_name(Some(loop_var_name.clone()))
    .build();
```

#### Pattern4 (with continue)
```rust
let boundary = JoinInlineBoundaryBuilder::new()
    .with_inputs(join_inputs, host_inputs)  // Dynamic carrier count
    .with_exit_bindings(exit_bindings)
    .with_loop_var_name(Some(loop_var_name.clone()))
    .build();
```

## Phase 201 目標

- ✅ Pattern2/3/4 全てで Builder 使用に統一
- ✅ フィールド直書き（`boundary.field = value;`）完全排除
- ✅ 境界情報の組み立てを 1 箇所（Builder）に集約
- ✅ テスト全 PASS（挙動不変）

## 実装ファイル

- Builder 本体: `src/mir/join_ir/lowering/inline_boundary_builder.rs`
- Pattern2: `src/mir/builder/control_flow/joinir/patterns/pattern2_with_break.rs`
- Pattern3: `src/mir/builder/control_flow/joinir/patterns/pattern3_with_if_phi.rs`
- Pattern4: `src/mir/builder/control_flow/joinir/patterns/pattern4_with_continue.rs`
