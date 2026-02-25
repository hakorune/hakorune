# Refactoring 5.1: Pattern 3 Hardcoded ValueIds → ExitMeta化

**Date**: 2025-12-09
**Status**: 🚧 In Progress
**Estimated Time**: 3-4 hours
**Priority**: HIGH

---

## 📋 目標

Pattern 3 lowerer（`loop_with_if_phi_minimal.rs`）を Pattern 4 と同じ ExitMeta ベースのアーキテクチャに統一化する。これにより：

1. ✅ Hardcoded ValueIds 定数削除（`PATTERN3_K_EXIT_*_ID`）
2. ✅ Exit binding の動的生成
3. ✅ Multi-carrier support の完全化
4. ✅ Pattern 3/4 の共通化度向上

---

## 🔄 変更対象ファイル

### ファイル1: `src/mir/join_ir/lowering/loop_with_if_phi_minimal.rs`

**現在:**
```rust
pub(crate) fn lower_loop_with_if_phi_pattern(
    _scope: LoopScopeShape,
    join_value_space: &mut JoinValueSpace,
) -> Option<JoinModule>
```

**変更後:**
```rust
use crate::mir::join_ir::lowering::join_fragment_meta::JoinFragmentMeta;

pub(crate) fn lower_loop_with_if_phi_pattern(
    _scope: LoopScopeShape,
    join_value_space: &mut JoinValueSpace,
) -> Result<(JoinModule, JoinFragmentMeta), String>
```

**変更内容:**
1. 戻り値を `Option<JoinModule>` → `Result<(JoinModule, JoinFragmentMeta), String>` に変更
2. `JoinFragmentMeta` を構築して返す
3. ExitMeta を動的に生成

### ファイル2: `src/mir/builder/control_flow/joinir/patterns/pattern3_with_if_phi.rs`

**現在:**
```rust
const PATTERN3_K_EXIT_SUM_FINAL_ID: ValueId = ValueId(24);    // Hardcoded!
const PATTERN3_K_EXIT_COUNT_FINAL_ID: ValueId = ValueId(25);  // Hardcoded!

// Lines 118-164: has_count 条件分岐で exit_bindings を手動構築
let exit_bindings = if has_count {
    vec![
        LoopExitBinding {
            join_exit_value: PATTERN3_K_EXIT_SUM_FINAL_ID,    // ← Hardcoded!
            ...
        },
        ...
    ]
} else {
    // Dummy count hack
    ...
}
```

**変更後:**
```rust
// Hardcoded 定数削除（削除）

// Lines 300-314: ExitMeta から exit_bindings を動的生成
let exit_bindings = ExitMetaCollector::collect(
    self,
    &exit_meta,
    debug,
);

// Carrier validation（Pattern 4と同じ）
for carrier in &carrier_info.carriers {
    if !exit_bindings.iter().any(|b| b.carrier_name == carrier.name) {
        return Err(format!(
            "[cf_loop/pattern3] Carrier '{}' not found in exit bindings",
            carrier.name
        ));
    }
}
```

---

## 🔧 実装ステップ

### Step 1: `loop_with_if_phi_minimal.rs` の k_exit 関数を分析

**現在の k_exit 実装（lines 401-415）:**
```rust
let mut k_exit_func = JoinFunction::new(
    k_exit_id,
    "k_exit".to_string(),
    vec![sum_final, count_final],  // Phase 195: Multi-carrier
);

k_exit_func.body.push(JoinInst::Ret {
    value: Some(sum_final),
});
```

**分析:**
- `k_exit` の parameters: `[sum_final, count_final]` が exit PHI
- k_exit は `sum_final` を return（最初の carrier）
- ExitMeta として記録すべき情報:
  - `sum` → `sum_final` (ValueId(24))
  - `count` → `count_final` (ValueId(25))

### Step 2: ExitMeta 構築ロジック追加

lowerer の最後に以下を追加:

```rust
// Phase 213: Build ExitMeta for dynamic exit binding generation
use crate::mir::join_ir::lowering::join_fragment_meta::JoinFragmentMeta;
use crate::mir::join_ir::lowering::exit_meta::ExitMeta;
use std::collections::HashMap;

let mut exit_values = HashMap::new();

// Map carrier names to their k_exit parameter ValueIds
exit_values.insert("sum".to_string(), sum_final);
if has_count {
    exit_values.insert("count".to_string(), count_final);
}

let exit_meta = ExitMeta {
    exit_values,
    exit_func_id: k_exit_id,
};

let fragment_meta = JoinFragmentMeta {
    exit_meta,
    // その他フィールド
};

Ok((join_module, fragment_meta))
```

**注:** `has_count` フラグは必要。multi-carrier に対応するため。

### Step 3: `pattern3_with_if_phi.rs` での lowerer 呼び出し変更

**現在（lines 109-116）:**
```rust
let join_module = match lower_loop_with_if_phi_pattern(ctx.loop_scope, &mut join_value_space) {
    Some(module) => module,
    None => {
        trace::trace().debug("pattern3", "Pattern 3 lowerer returned None");
        return Ok(None);
    }
};
```

**変更後:**
```rust
let (join_module, exit_meta) = match lower_loop_with_if_phi_pattern(ctx.loop_scope, &mut join_value_space) {
    Ok(result) => result,
    Err(e) => {
        trace::trace().debug("pattern3", &format!("Pattern 3 lowerer failed: {}", e));
        return Err(format!("[cf_loop/pattern3] Lowering failed: {}", e));
    }
};

trace::trace().debug(
    "pattern3",
    &format!("ExitMeta: {} exit values", exit_meta.exit_values.len())
);
for (carrier_name, join_value) in &exit_meta.exit_values {
    trace::trace().debug(
        "pattern3",
        &format!("  {} → ValueId({})", carrier_name, join_value.0)
    );
}
```

### Step 4: Exit binding 動的生成（Hardcoded 定数削除）

**現在（lines 8-30, 118-164）:**
```rust
// Hardcoded constants
const PATTERN3_K_EXIT_SUM_FINAL_ID: ValueId = ValueId(24);
const PATTERN3_K_EXIT_COUNT_FINAL_ID: ValueId = ValueId(25);

// Manual exit_bindings construction with has_count branching
let exit_bindings = if has_count {
    vec![
        LoopExitBinding {
            carrier_name: "sum".to_string(),
            join_exit_value: PATTERN3_K_EXIT_SUM_FINAL_ID,  // ← Hardcoded!
            host_slot: sum_var_id,
        },
        LoopExitBinding {
            carrier_name: "count".to_string(),
            join_exit_value: PATTERN3_K_EXIT_COUNT_FINAL_ID, // ← Hardcoded!
            host_slot: count_var_id,
        }
    ]
} else {
    // Single-carrier hack
    ...
}
```

**削除/変更:**
```rust
// Hardcoded constants（削除）
// const PATTERN3_K_EXIT_SUM_FINAL_ID: ValueId = ValueId(24);
// const PATTERN3_K_EXIT_COUNT_FINAL_ID: ValueId = ValueId(25);

// Dynamic exit binding generation（追加）
use super::super::merge::exit_line::meta_collector::ExitMetaCollector;
let exit_bindings = ExitMetaCollector::collect(
    self,
    &exit_meta,
    debug,
);

// Carrier validation
for carrier in &carrier_info.carriers {
    if !exit_bindings.iter().any(|b| b.carrier_name == carrier.name) {
        return Err(format!(
            "[cf_loop/pattern3] Carrier '{}' not found in exit bindings",
            carrier.name
        ));
    }
}
```

### Step 5: Dummy Count Backward Compat 簡略化

**現在（lines 145-164）:**
```rust
if has_count {
    // Multi-carrier case
    (vec![...], vec![...], vec![...])
} else {
    // Single-carrier case with Dummy void
    let dummy_count_id = constant::emit_void(self);
    (vec![...], vec![..., dummy_count_id], vec![...])
}
```

**簡略化:**
```rust
// Phase 213: Always use multi-carrier structure
// Single-carrier tests will use dummy void internally
let join_inputs = vec![ValueId(0), ValueId(1), ValueId(2)];
let mut host_inputs = vec![ctx.loop_var_id, sum_var_id];

if has_count {
    host_inputs.push(carrier_count_var_id);
} else {
    // Use void dummy for backward compat
    host_inputs.push(constant::emit_void(self));
}
```

---

## 📊 Before/After Code Change Summary

### `loop_with_if_phi_minimal.rs`

| 項目 | Before | After | 削減 |
|------|--------|-------|------|
| 関数署名 | `Option<JoinModule>` | `Result<(JoinModule, JoinFragmentMeta)>` | - |
| k_exit 構築 | あり | あり（変更なし） | 0行 |
| ExitMeta 構築 | なし | 新規追加 | +20行 |
| **計** | 428行 | 448行 | +20行 |

### `pattern3_with_if_phi.rs`

| 項目 | Before | After | 削減 |
|------|--------|-------|------|
| Hardcoded 定数 | 2個（lines 8-30） | 削除 | -2行 |
| Manual exit binding | あり（lines 118-164） | ExitMetaCollector化 | -40行 |
| Dummy count hack | あり | 簡略化 | -5行 |
| Lowerer呼び出し | None/Some | Ok/Err | -5行 |
| ExitMeta debug | なし | 新規追加 | +10行 |
| **計** | 191行 | 149行 | **-42行（22%削減）** |

---

## ✅ テスト戦略

### Test 1: `loop_if_phi.hako` (既存 test)

**動作確認:**
```bash
./target/release/hakorune --dump-mir apps/tests/loop_if_phi.hako 2>&1 | grep -A 5 "k_exit"
```

**期待:** k_exit が sum/count を正しく処理（変わらず）

### Test 2: Multi-carrier tests（Phase 195）

**確認対象:**
- `test_pattern3_multi_carrier_sum_count`
- Carrier binding が動的に生成されることを確認

### Test 3: Cargo test

```bash
cargo test --release pattern3 2>&1 | tail -20
```

---

## 🚨 リスク & ミティゲーション

### リスク 1: ExitMeta 構築が複雑

**ミティゲーション:**
- Pattern 4 のコードをコピーペースト基準にする
- 最小限の変更に留める

### リスク 2: ExitMetaCollector の動作確認

**ミティゲーション:**
- Pattern 4 で既に使用済み（実績あり）
- Carrier validation で エラー検出

### リスク 3: Dummy count backward compat 破損

**ミティゲーション:**
- has_count フラグで分岐を保つ
- 古い単一キャリア テストは動作のまま

---

## 📝 Checklist

- [ ] Step 1: k_exit 実装分析完了
- [ ] Step 2: ExitMeta 構築ロジック追加
- [ ] Step 3: lowerer 呼び出し変更
- [ ] Step 4: Hardcoded 定数削除 + ExitMetaCollector 導入
- [ ] Step 5: Dummy count 簡略化
- [ ] Step 6: Build 成功確認
- [ ] Step 7: Test 実行確認
- [ ] Step 8: Commit & Document 更新
- [ ] Refactoring 5.1 COMPLETE ✅

---

## 📚 参考資料

- Pattern 4 実装: `src/mir/builder/control_flow/joinir/patterns/pattern4_with_continue.rs` (lines 300-380)
- ExitMetaCollector: `src/mir/builder/control_flow/joinir/merge/exit_line/meta_collector.rs`
- JoinFragmentMeta: `src/mir/join_ir/lowering/join_fragment_meta.rs`

