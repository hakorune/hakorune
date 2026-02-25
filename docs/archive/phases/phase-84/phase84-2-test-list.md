# Phase 84-2: Case D 残り 9件クイックリファレンス

## 一覧表

| # | Group | テスト名 | ValueId | ファイル | 行番号 |
|---|-------|---------|---------|---------|-------|
| 1 | C | `test_lowering_await_expression` | 2 | `src/mir/mod.rs` | 363 |
| 2 | A | `loop_with_continue_and_break_edge_copy_merge` | 56 | `src/tests/loop_continue_break_no_phi_tests.rs` | 21 |
| 3 | A | `nested_loop_with_multi_continue_break_edge_copy_merge` | 135 | `src/tests/loop_nested_no_phi_tests.rs` | 21 |
| 4 | A | `loop_inner_if_multilevel_edge_copy` | 74 | `src/tests/loop_nested_no_phi_tests.rs` | 224 |
| 5 | A | `loop_break_and_early_return_edge_copy` | 40 | `src/tests/loop_return_no_phi_tests.rs` | 22 |
| 6 | A | `vm_exec_break_inside_if` | 27 | `src/tests/mir_ctrlflow_break_continue.rs` | 47 |
| 7 | A | `loop_if_three_level_merge_edge_copy` | 75 | `src/tests/loop_return_no_phi_tests.rs` | 194 |
| 8 | B | `mir_stage1_cli_emit_program_min_exec_hits_type_error` | 7 | `src/tests/mir_stage1_cli_emit_program_min.rs` | 97 |
| 9 | B | `mir_stage1_cli_emit_program_min_compiles_and_verifies` | 7 | `src/tests/mir_stage1_cli_emit_program_min.rs` | 71 |

## グループ詳細

### GroupA: Loop 制御フロー PHI（7件）

**共通パターン**: Loop + continue/break

**テスト実行**:
```bash
# 個別テスト
cargo test --release --lib loop_with_continue_and_break_edge_copy_merge

# GroupA 全体
cargo test --release --lib loop_continue_break
cargo test --release --lib loop_nested
cargo test --release --lib loop_return
cargo test --release --lib vm_exec_break
```

**期待される解決策**: Edge Copy 追跡 PHI 型推論

### GroupB: 多段 PHI 型推論（2件）

**共通パターン**: static box + 複数 return 経路

**テスト実行**:
```bash
# GroupB 全体
cargo test --release --lib mir_stage1_cli_emit_program_min
```

**期待される解決策**: 再帰的 PHI 型推論

### GroupC: await 特殊パターン（1件）

**パターン**: await 式の MIR lowering

**テスト実行**:
```bash
cargo test --release --lib test_lowering_await_expression
```

**期待される解決策**: await 特殊ケース処理（暫定）

## 実行コマンド集

### 全 Case D 確認

```bash
NYASH_PHI_FALLBACK_DISABLED=1 cargo test --release --lib 2>&1 | grep -B5 "Case D"
```

### ValueId 一覧取得

```bash
NYASH_PHI_FALLBACK_DISABLED=1 cargo test --release --lib 2>&1 | grep "ValueId:" | sort | uniq
```

### グループ別実行

```bash
# GroupA のみ
cargo test --release --lib loop_ mir_ctrlflow_break

# GroupB のみ
cargo test --release --lib mir_stage1_cli

# GroupC のみ
cargo test --release --lib test_lowering_await
```

## デバッグ用環境変数

```bash
# MIR ダンプ
NYASH_DUMP_MIR=1 cargo test --release --lib <test_name>

# 詳細ログ
RUST_LOG=debug cargo test --release --lib <test_name>

# バックトレース
RUST_BACKTRACE=1 cargo test --release --lib <test_name>
```

## 次のステップ

1. **Phase 84-3**: GroupA の 7件を解決
   - `GenericTypeResolver::resolve_from_phi_with_copy_trace()` 実装
   - Edge Copy 追跡ロジック追加
   - 期待: 9件 → 2件

2. **Phase 84-4**: GroupB の 2件を解決
   - `GenericTypeResolver::resolve_from_phi_recursive()` 実装
   - 再帰的 PHI 型推論
   - 期待: 2件 → 1件

3. **Phase 84-5**: GroupC の 1件を解決
   - await 特殊ケース追加
   - 期待: 1件 → 0件

## 参考資料

- [詳細調査](./phase84-2-case-d-investigation.md)
- [サマリー](./phase84-2-summary.md)
- [CopyTypePropagator 実装](../../../src/mir/phi_core/copy_type_propagator.rs)
Status: Historical
