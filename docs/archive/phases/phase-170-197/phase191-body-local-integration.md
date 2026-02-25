# Phase 191: Body-Local Init & Update Lowering Integration

**Status**: Ready for Implementation
**Date**: 2025-12-09
**Prerequisite**: Phase 190-impl complete (NumberAccumulation + PHI wiring fixed)

---

## 目的

Phase 184/186 で作った `LoopBodyLocalEnv`, `UpdateEnv`, `LoopBodyLocalInitLowerer` を
Pattern2/4 の中に本番統合して、`local digit = ...` などの body-local を JoinIR/MIR にきちんと落とす。

P2/P4 の既存ループ（`_atoi` など）を、body-local を含んだ形でも JoinIR で動かせるようにする。

---

## Task 191-1: 対象ケースの再確認（1本に絞る）

### 目標
「いま body-local が原因で JoinIR を OFF にしている代表ループ」を 1 本だけ選ぶ。

### 候補
```nyash
// _atoi 簡略版
local digit = s[i] - '0'
result = result * 10 + digit
```

または body-local を使う最小テスト:
```nyash
// phase191_body_local_atoi.hako
static box Main {
    main() {
        local result = 0
        local i = 0
        loop(i < 3) {
            local digit = i + 1    // body-local
            result = result * 10 + digit
            i = i + 1
        }
        print(result)  // Expected: 123
        return 0
    }
}
```

### 成果物
- 選定した代表ループのファイルパス
- なぜ現在 JoinIR で通らないかの説明（body-local init が emit されていない）

---

## Task 191-2: LoopBodyLocalInitLowerer の統合

### 対象ファイル
- `src/mir/builder/control_flow/joinir/patterns/pattern2_with_break.rs`
- 必要なら `pattern4_with_continue.rs` も

### 実装内容

ループ lowering の流れの中で:

1. ループ本体 AST を `LoopBodyLocalInitLowerer` に渡して、
   body-local の `local` 定義 (`digit = ...`) を JoinIR に emit しつつ、
   `LoopBodyLocalEnv` に `name -> join_id` を登録させる。

2. ここで emit するのは「init 分」だけ（Update 分は既に `CarrierUpdateEmitter` + `UpdateEnv` に任せる）。

### init emission の位置

- ループ body の **先頭**（carrier update より前）に出るようにする。
- JoinIR 側の ValueId 空間は、Phase 190-impl-D での衝突回避ルール (`body_local_start_offset`) に従う。

### コード変更例

```rust
// pattern2_with_break.rs 内

// Phase 191: Emit body-local initializations
let body_local_env = LoopBodyLocalInitLowerer::lower_init(
    &body_ast,
    &mut join_builder,
    &mut alloc_body_local_value,
)?;

// Phase 191: Create UpdateEnv with both condition and body-local
let update_env = UpdateEnv::new(condition_env.clone(), body_local_env.clone());

// Emit carrier updates using UpdateEnv
for (carrier_name, update_expr) in &carrier_info.updates {
    CarrierUpdateEmitter::emit_carrier_update_with_env(
        &mut join_builder,
        carrier_name,
        update_expr,
        &update_env,
    )?;
}
```

---

## Task 191-3: UpdateEnv 統合の確認

### 対象ファイル
- `src/mir/join_ir/lowering/carrier_update_emitter.rs`（すでに UpdateEnv 対応済み）

### 確認内容

選んだ代表ループで:
- `UpdateEnv` が:
  - `ConditionEnv`（LoopParam / OuterLocal）
  - `LoopBodyLocalEnv`（今回 lower した init の join_id）
  両方を見て `digit` や `temp` を正しく解決できているか確認。

### ユニットテスト追加

body-local `digit` を含む NumberAccumulation 更新を与えて、
JoinIR 側で Mul/Add 生成がエラーなく通ること:

```rust
#[test]
fn test_number_accumulation_with_body_local() {
    // UpdateEnv with body-local "digit" → ValueId(10)
    let mut body_local_env = LoopBodyLocalEnv::new();
    body_local_env.register("digit", ValueId(10));

    let condition_env = ConditionEnv::new();
    let update_env = UpdateEnv::new(condition_env, body_local_env);

    // Resolve "digit" should return ValueId(10)
    assert_eq!(update_env.resolve("digit"), Some(ValueId(10)));
}
```

---

## Task 191-4: E2E テスト

### テストケース

191-1 で選んだ Minimal `_atoi` ループを:
- body-local 版（`local digit = ...`）に戻して、
- `NYASH_JOINIR_CORE=1` で実行 → 期待値が返ることを確認。

```bash
# E2E テスト実行
./target/release/hakorune apps/tests/phase191_body_local_atoi.hako
# Expected output: 123
```

### 退行チェック

- 既存の int-only ループ（body-local なし）が引き続き動くこと
  - `phase190_atoi_impl.hako` → 12
  - `phase190_parse_number_impl.hako` → 123
- Trim/JsonParser の P5/P2 ラインに影響がないこと

---

## Task 191-5: ドキュメント更新

### 更新対象

1. **phase184-body-local-mir-lowering.md** / **phase186-body-local-init-lowering.md**:
   - 「Phase 191 で Pattern2/4 に統合完了」と追記
   - 代表ループ名と、init → UpdateEnv → PHI → ExitLine までの流れを 1 ケースだけ図解

2. **CURRENT_TASK.md**:
   - Phase 191: 完了マーク
   - 残タスク（もし他のループは後回しにするなら）を更新

3. **joinir-architecture-overview.md**:
   - Section 7.2 の残タスク「body-local 変数の init + update lowering」を完了マークに更新

---

## 成功基準

- [x] 代表ループ（body-local 版 `_atoi`）が JoinIR only で期待値を返す
- [x] 既存テスト（phase190_*.hako）が退行しない
- [x] UpdateEnv が body-local 変数を正しく解決できる
- [x] ドキュメントが更新されている

---

## 実装完了レポート (2025-12-09)

### 実装概要

Phase 191 が**完全成功**しました！ `LoopBodyLocalInitLowerer` を Pattern2 に統合し、body-local 変数の初期化式を JoinIR に正しく降下できるようになりました。

### 主な変更

1. **`loop_with_break_minimal.rs`**:
   - 関数シグネチャに `body_ast: &[ASTNode]` を追加
   - `body_local_env: Option<&mut LoopBodyLocalEnv>` に変更（mutable）
   - ループ body の先頭で `LoopBodyLocalInitLowerer` を呼び出し、初期化式を JoinIR に emit
   - Carrier update の前に body-local init を配置（正しい依存順序）

2. **`pattern2_with_break.rs`**:
   - `collect_body_local_variables` 呼び出しを削除（ValueId の二重割り当てを回避）
   - 空の `LoopBodyLocalEnv` を作成し、`LoopBodyLocalInitLowerer` に委譲
   - `lower_loop_with_break_minimal` に `_body` AST を渡すよう修正

3. **テストファイル**:
   - `apps/tests/phase191_body_local_atoi.hako` 新規作成
   - `local digit = i + 1` パターンで body-local 変数を使用
   - `result = result * 10 + digit` で NumberAccumulation パターン検証

4. **テスト修正**:
   - `tests/json_program_loop.rs` の `program_loop_body_local_exit` を修正
   - スコープ外の body-local 変数参照を削除（正しい動作に修正）

### 実行結果

```bash
$ ./target/release/hakorune apps/tests/phase191_body_local_atoi.hako
123  # ✅ 期待値通り

$ ./target/release/hakorune apps/tests/phase190_atoi_impl.hako
12   # ✅ 退行なし

$ ./target/release/hakorune apps/tests/phase190_parse_number_impl.hako
123  # ✅ 退行なし

$ cargo test --release json_program_loop
test json_loop_simple_verifies ... ok
test json_loop_body_local_exit_verifies ... ok
test json_loop_with_continue_verifies ... ok
# ✅ 全テストパス
```

### 技術的発見

1. **ValueId 二重割り当て問題**:
   - 旧実装: `collect_body_local_variables` が ValueId を事前割り当て → `LoopBodyLocalInitLowerer` がスキップ
   - 解決: 空の `LoopBodyLocalEnv` を渡し、`LoopBodyLocalInitLowerer` に完全委譲

2. **JoinIR ValueId 空間**:
   - JoinIR は独立した ValueId 空間を持つ（0 から開始）
   - Host ValueId へのマッピングは merge 段階で実施
   - `body_local_start_offset` は Host 空間の概念であり、JoinIR には不要

3. **UpdateEnv の優先順位**:
   - ConditionEnv（ループパラメータ・条件変数）が最優先
   - LoopBodyLocalEnv（body-local 変数）がフォールバック
   - この順序により、シャドウイングが正しく動作

### 制限事項

現在の実装では以下の初期化式のみサポート:
- 整数リテラル: `local x = 42`
- 変数参照: `local y = i`
- 二項演算: `local z = i + 1`, `local w = pos - start`

未サポート（Fail-Fast）:
- メソッド呼び出し: `local s = str.substring(...)`
- 文字列操作: `local t = s + "abc"`
- 複雑な式: ネストした呼び出し、非算術演算

### 次のステップ

Phase 191 で以下が達成されました:
- [x] Pattern 2 への body-local init lowering 統合
- [x] UpdateEnv による body-local 変数解決
- [x] NumberAccumulation パターンでの動作検証
- [x] 既存テストの退行なし

今後の拡張:
- Pattern 4 (continue) への同様の統合（必要に応じて）
- 文字列初期化式のサポート（Phase 188 の延長）
- メソッド呼び出し初期化式のサポート（Phase 192+）

---

## 関連ファイル

### インフラ（Phase 184 で実装済み）
- `src/mir/join_ir/lowering/loop_body_local_env.rs`
- `src/mir/join_ir/lowering/update_env.rs`
- `src/mir/join_ir/lowering/carrier_update_emitter.rs`

### 統合対象
- `src/mir/builder/control_flow/joinir/patterns/pattern2_with_break.rs`
- `src/mir/builder/control_flow/joinir/patterns/pattern4_with_continue.rs`

### テストファイル
- `apps/tests/phase191_body_local_atoi.hako`（新規作成）
- `apps/tests/phase190_atoi_impl.hako`（退行確認）
- `apps/tests/phase190_parse_number_impl.hako`（退行確認）
Status: Historical
