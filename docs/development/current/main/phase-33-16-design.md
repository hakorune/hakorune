# Phase 33-16: Loop Header PHI SSOT 設計書

## 現状

Phase 33-15 で SSA-undef エラーを修正したが、対処療法として exit_phi_inputs と carrier_inputs 収集をスキップした。
結果、joinir_min_loop.hako のループ結果値が正しくない（期待値2、実際0）。

## 根本原因

JoinIR の関数パラメータ（i_param, i_exit）は MIR 空間で SSA 定義を持たない。

```
JoinIR:
  fn loop_step(i_param):   // ← i_param は関数パラメータ
    ...
    Jump(k_exit, [i_param]) // ← exit 時に i_param を渡す
    ...
    i_next = i_param + 1
    Call(loop_step, [i_next]) // ← tail call

  fn k_exit(i_exit):       // ← i_exit も関数パラメータ
    Return(i_exit)
```

MIR にインライン化すると:
- `i_param` は BoundaryInjector Copy でエントリー時に設定される
- tail call は `Copy { dst: i_param_remapped, src: i_next_remapped }` に変換される
- **しかし exit path では i_param_remapped に有効な値がない！**

## 解決策: LoopHeaderPhiBuilder

ループヘッダーブロックに PHI を配置して、carrier の「現在値」を追跡する。

### 目標構造

```
host_entry:
  i_init = 0
  Jump → loop_header

loop_header:
  i_phi = PHI [(host_entry, i_init), (latch, i_next)]  ← NEW!
  exit_cond = !(i_phi < 3)
  Branch exit_cond → exit, body

body:
  break_cond = (i_phi >= 2)
  Branch break_cond → exit, continue

continue:
  i_next = i_phi + 1
  Jump → loop_header  // latch edge

exit:
  // i_phi がループ終了時の正しい値！
```

### 実装計画

#### 1. LoopHeaderPhiBuilder (完了)

`src/mir/builder/control_flow/joinir/merge/loop_header_phi_builder.rs` に実装済み。

```rust
pub struct LoopHeaderPhiInfo {
    pub header_block: BasicBlockId,
    pub carrier_phis: BTreeMap<String, CarrierPhiEntry>,
    pub expr_result_phi: Option<ValueId>,
}
```

#### 2. merge パイプラインへの組み込み (TODO)

`merge/mod.rs` の merge_joinir_mir_blocks() を修正:

```rust
// Phase 3: Remap ValueIds
remap_values(builder, &used_values, &mut remapper, debug)?;

// Phase 3.5: Build loop header PHIs (NEW!)
let loop_header_info = if boundary.is_some() && is_loop_pattern {
    loop_header_phi_builder::LoopHeaderPhiBuilder::build(
        builder,
        header_block_id,      // ← Pattern lowerer から渡す必要あり
        entry_block_id,
        loop_var_name,
        loop_var_init,
        &[], // carriers
        expr_result_is_loop_var,
        debug,
    )?
} else {
    LoopHeaderPhiInfo::empty(BasicBlockId(0))
};

// Phase 4: Merge blocks and rewrite instructions
// instruction_rewriter に loop_header_info を渡す
let merge_result = instruction_rewriter::merge_and_rewrite(
    builder,
    mir_module,
    &mut remapper,
    &value_to_func_name,
    &function_params,
    &mut loop_header_info,  // latch_incoming を設定するため mut
    boundary,
    debug,
)?;

// Phase 4.5: Finalize loop header PHIs (NEW!)
if boundary.is_some() && loop_header_info.carrier_phis.len() > 0 {
    loop_header_phi_builder::LoopHeaderPhiBuilder::finalize(
        builder,
        &loop_header_info,
        debug,
    )?;
}
```

#### 3. instruction_rewriter 修正 (TODO)

Phase 33-15 の skip ロジックを削除し、以下を実装:

**Tail call 処理時** (276-335行):
```rust
// tail call の args を latch_incoming として記録
for (i, arg_val_remapped) in args.iter().enumerate() {
    // carrier 名を特定（loop_var_name or carrier_name）
    let carrier_name = get_carrier_name_for_index(i);
    loop_header_info.set_latch_incoming(
        carrier_name,
        new_block_id,      // latch block
        *arg_val_remapped, // 次のイテレーション値
    );
}
```

**Exit path 処理時** (350-435行):
```rust
// exit_phi_inputs に header PHI の dst を使う
if let Some(phi_dst) = loop_header_info.get_carrier_phi(loop_var_name) {
    exit_phi_inputs.push((new_block_id, phi_dst));
}

// carrier_inputs にも header PHI の dst を使う
for (carrier_name, entry) in &loop_header_info.carrier_phis {
    carrier_inputs.entry(carrier_name.clone())
        .or_insert_with(Vec::new)
        .push((new_block_id, entry.phi_dst));
}
```

#### 4. JoinInlineBoundary 拡張 (TODO)

header_block 情報を Pattern lowerer から渡す:

```rust
pub struct JoinInlineBoundary {
    // ... existing fields ...

    /// Phase 33-16: Loop header block ID (for LoopHeaderPhiBuilder)
    /// Set by Pattern lowerers that need header PHI generation.
    pub loop_header_block: Option<BasicBlockId>,
}
```

#### 5. Pattern 2 lowerer 修正 (TODO)

`pattern2_with_break.rs` で header_block を設定:

```rust
boundary.loop_header_block = Some(loop_step_entry_block);
```

### テスト戦略

| テスト | 期待値 | 検証ポイント |
|--------|--------|-------------|
| joinir_min_loop.hako | `return i` → 2 | expr_result PHI が正しい値を持つ |
| trim 系テスト | 回帰なし | carrier PHI が正しく動作 |
| SSA-undef | エラーなし | 定義済み ValueId のみ参照 |

## 実装の複雑さ

**中程度**。主な変更点:
1. merge/mod.rs: Phase 3.5 と 4.5 追加
2. instruction_rewriter.rs: latch_incoming 記録と PHI dst 参照
3. Pattern lowerers: header_block 情報の追加

## 次のステップ

1. merge/mod.rs に Phase 3.5/4.5 フックを追加
2. instruction_rewriter に loop_header_info 引数を追加
3. Pattern 2 lowerer で header_block を設定
4. テストで検証
