# Phase 196: Select Expansion Bug Analysis

## Problem Summary

JoinIR→MIR の Select 展開処理で、PHI inputs に undefined ValueId が使用される問題が発生している。

## Reproduction Case

```hako
// apps/tests/phase195_sum_count.hako
local i = 1
local sum = 0
local count = 0

loop(i <= 5) {
    if(i % 2 == 1) {
        sum = sum + i
        count = count + 1
    } else {
        sum = sum + 0
        count = count + 0
    }
    i = i + 1
}
```

##症状 (Before)

### JoinIR 側（正しい）
```
[joinir_block/handle_select] Created merge_block BasicBlockId(5) with 1 instructions
(first=Some(Phi { dst: ValueId(20), inputs: [(BasicBlockId(3), ValueId(14)), (BasicBlockId(4), ValueId(18))], type_hint: None }))
```

### ValueId Remapping（正しい）
```
[DEBUG-177]   JoinIR ValueId(14) → Host ValueId(21)
[DEBUG-177]   JoinIR ValueId(18) → Host ValueId(25)
```

### Block Remapping（正しい）
```
[trace:blocks] allocator: Block remap: join_func_1:BasicBlockId(3) → BasicBlockId(8)
[trace:blocks] allocator: Block remap: join_func_1:BasicBlockId(4) → BasicBlockId(9)
[trace:blocks] allocator: Block remap: join_func_1:BasicBlockId(5) → BasicBlockId(10)
```

### MIR 側（壊れている）
```
bb10:
    1: %27 = phi [%28, bb8], [%32, bb9]
    1: br %20, label bb11, label bb12
```

**問題**: ValueId(28) と ValueId(32) は定義されていない！
- 期待値: `phi [%21, bb8], [%25, bb9]`（remap後の値）
- 実際: `phi [%28, bb8], [%32, bb9]`（未定義の値）

## 根本原因の仮説

### 仮説1: PHI inputs の ValueId が remap されていない
- `handle_select()` で生成された PHI は JoinIR ValueId を使用
- `instruction_rewriter.rs` の PHI remap ロジック（line 317-333）が何らかの理由で適用されていない

### 仮説2: 二重 remap による上書き
- `remap_instruction()` で 1回 remap（line 304）
- 手動 block remap で再度 remap（line 317-333）
- 二重 remap が問題を引き起こしている可能性

### 仮説3: local_block_map のスコープ問題
- Select で生成された then/else ブロック（bb3, bb4）が local_block_map に含まれていない
- `.unwrap_or(*bb)` でフォールバックしている可能性

## 調査経路

1. **handle_select() 実装**:
   - `src/mir/join_ir_vm_bridge/joinir_block_converter.rs:407-484`
   - Select → Branch + then/else + merge(PHI) に展開
   - PHI inputs に生の JoinIR ValueId を使用（line 461）

2. **instruction_rewriter PHI remap**:
   - `src/mir/builder/control_flow/joinir/merge/instruction_rewriter.rs:317-333`
   - PHI の block ID と ValueID を両方 remap
   - `remapper.remap_value(*val)` で ValueId を変換（line 328）

3. **remap_instruction() の PHI 処理**:
   - `src/mir/builder/joinir_id_remapper.rs:327-334`
   - こちらでも PHI inputs を remap（line 331）

## 根本原因（確定）

**仮説2が正解: 二重 remap による破損**

### 問題の詳細

`instruction_rewriter.rs` の PHI 処理で、ValueId が **二重に remap** されていた：

1. **Line 304**: `remapper.remap_instruction(inst)` で PHI inputs の ValueId を remap
   - JoinIR `ValueId(14)` → Host `ValueId(21)`
   - JoinIR `ValueId(18)` → Host `ValueId(25)`

2. **Line 328**: `remapper.remap_value(*val)` で **再度** remap を試行
   - しかし `*val` は既に Host ValueId（21, 25）になっている！
   - `remap_value(ValueId(21))` → `value_map` に存在しない → `unwrap_or(21)` → `ValueId(21)`
   - **問題なく見えるが、実際には壊れている**

### なぜ壊れたか？

`remap_instruction()` が返した `inputs` は **既に remap 済み** なのに、line 328 で **もう一度 remap** しようとした。

しかし、実際には `remap_value()` は idempotent ではない可能性がある（value_map に存在しない場合は元の値を返すが、これは**別の ValueId が偶然同じ番号**の可能性がある）。

### 修正内容

**File**: `src/mir/builder/control_flow/joinir/merge/instruction_rewriter.rs`
**Line**: 317-335

```rust
// Before (Phase 172)
MirInstruction::Phi {
    dst,
    inputs,
    type_hint: None,
} => MirInstruction::Phi {
    dst,
    inputs: inputs
        .iter()
        .map(|(bb, val)| {
            let remapped_bb = local_block_map.get(bb).copied().unwrap_or(*bb);
            let remapped_val = remapper.remap_value(*val);  // ❌ 二重 remap!
            (remapped_bb, remapped_val)
        })
        .collect(),
    type_hint: None,
},

// After (Phase 196)
MirInstruction::Phi {
    dst,
    inputs,
    type_hint: None,
} => MirInstruction::Phi {
    dst,
    inputs: inputs
        .iter()
        .map(|(bb, val)| {
            let remapped_bb = local_block_map.get(bb).copied().unwrap_or(*bb);
            // Phase 196 FIX: Don't double-remap values!
            // remapper.remap_instruction() already remapped *val
            (remapped_bb, *val)  // ✅ 値はそのまま使う（既に remap 済み）
        })
        .collect(),
    type_hint: None,
},
```

### 修正後の動作（After）

```
bb10:
    1: %27 = phi [%21, bb8], [%25, bb9]  // ✅ 正しい ValueId
    1: br %20, label bb11, label bb12
```

## テスト結果

### ✅ phase195_sum_count.hako
```bash
$ NYASH_JOINIR_CORE=1 ./target/release/hakorune apps/tests/phase195_sum_count.hako
93
RC: 0
```
期待値: sum=9 (1+3+5), count=3 → result=93 ✅

### ✅ loop_if_phi.hako (Single-carrier P3)
```bash
$ NYASH_JOINIR_CORE=1 ./target/release/hakorune apps/tests/loop_if_phi.hako
[Console LOG] sum=9
RC: 0
```

### ✅ loop_min_while.hako (Pattern 1)
```bash
$ NYASH_JOINIR_CORE=1 ./target/release/hakorune apps/tests/loop_min_while.hako
0
1
2
RC: 0
```

### ✅ joinir_min_loop.hako (Pattern 2)
```bash
$ NYASH_JOINIR_CORE=1 ./target/release/hakorune apps/tests/joinir_min_loop.hako
RC: 0
```

## 退行チェック

- ✅ Pattern 1 (Simple while): PASS
- ✅ Pattern 2 (Break): PASS
- ✅ Pattern 3 (Single-carrier): PASS
- ✅ Pattern 3 (Multi-carrier): PASS
- ✅ No `[joinir/freeze]` warnings

## まとめ

### 問題
- Select 展開で生成された PHI の inputs が undefined ValueId を参照

### 根本原因
- `instruction_rewriter.rs` で ValueId を二重 remap（1回目: remap_instruction, 2回目: manual remap）

### 修正
- PHI の block ID のみを remap、ValueId は既に remap 済みなのでそのまま使用

### 影響範囲
- **1ファイル 1箇所のみ**: `instruction_rewriter.rs` line 331

### 成果
- Phase 195 の Multi-carrier Pattern 3 が完全動作
- 既存 Pattern 1/2/3 に退行なし
Status: Historical
