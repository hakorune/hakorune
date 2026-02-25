# Phase 69-4.3: Trio 依存を LoopScopeShape に寄せる設計

## 目的

`loop_form_intake.rs` の Trio (LoopVarClassBox, LoopExitLivenessBox, LocalScopeInspectorBox) 依存を `LoopScopeShape` に置き換える設計を策定し、Phase 70 での実装準備を整える。

---

## 1. 現状分析: loop_form_intake.rs での Trio 使用パターン

### 1.1 intake_loop_form() 関数の役割

`intake_loop_form()` は LoopForm と MIR から以下を抽出する「入口箱」:

```rust
pub(crate) struct LoopFormIntake {
    pub pinned_ordered: Vec<String>,
    pub carrier_ordered: Vec<String>,
    pub header_snapshot: BTreeMap<String, ValueId>,
    pub exit_snapshots: Vec<(BasicBlockId, BTreeMap<String, ValueId>)>,
    pub exit_preds: Vec<BasicBlockId>,
}
```

### 1.2 Trio の使用箇所（L169-182）

```rust
// LocalScopeInspector に snapshot を登録して VarClass 判定の土台を整える
let mut inspector = LocalScopeInspectorBox::new();
inspector.record_snapshot(loop_form.header, &header_vals_mir);
for (bb, snap) in &exit_snapshots {
    inspector.record_snapshot(*bb, snap);
}

let all_names: Vec<String> = header_vals_mir.keys().cloned().collect();
let classified = var_classes.classify_all(
    &all_names,
    &pinned_hint,
    &carrier_hint,
    &inspector,
    &exit_preds,
);

let ordered_pinned: Vec<String> = classified
    .iter()
    .filter(|(_, c)| matches!(c, LoopVarClass::Pinned))
    .map(|(n, _)| n.clone())
    .collect::<BTreeSet<_>>()
    .into_iter()
    .collect();
let ordered_carriers: Vec<String> = classified
    .iter()
    .filter(|(_, c)| matches!(c, LoopVarClass::Carrier))
    .map(|(n, _)| n.clone())
    .collect::<BTreeSet<_>>()
    .into_iter()
    .collect();
```

### 1.3 Trio 3箱の責務

1. **LocalScopeInspectorBox**（変数定義追跡）
   - 役割: 各 basic block で定義されている変数を記録
   - 使用メソッド:
     - `record_snapshot(block, vars)`: snapshot を登録
     - `is_available_in_all(var, blocks)`: 全 block で利用可能か判定
     - `get_defining_blocks(var)`: 定義されている block 一覧を取得
     - `all_variables()`: 全変数名を取得

2. **LoopVarClassBox**（変数分類）
   - 役割: 変数を 4分類（Pinned/Carrier/BodyLocalExit/BodyLocalInternal）
   - 使用メソッド:
     - `classify_all(vars, pinned_hint, carrier_hint, inspector, exit_preds)`: 一括分類
     - `classify(var, ...)`: 単一変数分類

3. **LoopExitLivenessBox**（exit 後 liveness）
   - 役割: exit 後で使われる変数集合を計算
   - 使用メソッド:
     - `compute_live_at_exit(query, exit_block, header_vals, exit_snapshots)`: live 変数集合を返す
   - **現状**: 保守的近似（全変数を live とみなす）

---

## 2. LoopScopeShape カバー範囲

### 2.1 LoopScopeShape の構造

```rust
pub(crate) struct LoopScopeShape {
    pub header: BasicBlockId,
    pub body: BasicBlockId,
    pub latch: BasicBlockId,
    pub exit: BasicBlockId,
    pub pinned: BTreeSet<String>,
    pub carriers: BTreeSet<String>,
    pub body_locals: BTreeSet<String>,
    pub exit_live: BTreeSet<String>,
    pub progress_carrier: Option<String>,
    pub(crate) variable_definitions: BTreeMap<String, BTreeSet<BasicBlockId>>,
}
```

### 2.2 LoopScopeShape の提供 API

| API | 機能 | Trio 対応 |
|-----|-----|-----------|
| `classify(var)` | 変数を 4分類 | ✅ LoopVarClassBox.classify() |
| `classify_all(vars)` | 一括分類 | ✅ LoopVarClassBox.classify_all() |
| `needs_header_phi(var)` | header PHI 必要性判定 | ✅ LoopVarClass.needs_header_phi() |
| `needs_exit_phi(var)` | exit PHI 必要性判定 | ✅ LoopVarClass.needs_exit_phi() |
| `get_exit_live()` | exit 後 live 変数集合 | ✅ LoopExitLivenessBox.compute_live_at_exit() |
| `is_available_in_all(var, blocks)` | 全 block で利用可能か | ✅ LocalScopeInspectorBox.is_available_in_all() |
| `pinned_ordered()` | 順序付き pinned 一覧 | ✅ LoopFormIntake.pinned_ordered |
| `carriers_ordered()` | 順序付き carrier 一覧 | ✅ LoopFormIntake.carrier_ordered |
| `header_phi_vars()` | header PHI 対象変数 | ✅ pinned + carriers |
| `exit_phi_vars()` | exit PHI 対象変数 | ✅ exit_live |

### 2.3 Phase 48-4 完了の成果

- **Trio の内部化**: `LoopScopeShape::from_existing_boxes_legacy()` が Trio を使用
- **外部からの隠蔽**: 利用側は `LoopScopeShape::from_loop_form()` 経由で Trio を知らない
- **helper 関数の整備**:
  - `build_inspector()`: LocalScopeInspectorBox 構築
  - `classify_body_and_exit()`: LoopVarClassBox による分類
  - `merge_exit_live_from_box()`: LoopExitLivenessBox による exit_live 計算
  - `extract_variable_definitions()`: variable_definitions 抽出

---

## 3. Gap 分析

### 3.1 カバーできている機能

| 機能 | loop_form_intake.rs | LoopScopeShape | 状態 |
|------|---------------------|----------------|------|
| 変数分類（4分類） | ✅ classify_all() | ✅ classify() | **完全カバー** |
| pinned/carrier 判定 | ✅ hint → classify | ✅ BTreeSet 保持 | **完全カバー** |
| exit_live 計算 | ❌ 未使用 | ✅ get_exit_live() | **LoopScopeShape が優位** |
| 定義位置追跡 | ✅ inspector | ✅ variable_definitions | **完全カバー** |
| 順序付き一覧 | ✅ Vec 生成 | ✅ *_ordered() API | **完全カバー** |

### 3.2 カバーできていない機能

**結論**: なし！LoopScopeShape は Trio の全機能をカバーしている。

### 3.3 loop_form_intake.rs 固有の役割

`loop_form_intake.rs` が提供している機能で LoopScopeShape が直接提供していないもの:

1. **MIR パース**: preheader から変数名を推定（L31-88）
   - `s` (StringBox param), `n` (length), `i` (index) の抽出
   - `value_to_name` マッピング構築

2. **header φ 読み取り**: header PHI から snapshot 構築（L96-129）
   - pinned/carrier の hint 生成
   - incoming values の追跡

3. **exit preds 収集**: CFG から exit 前駆者を列挙（L147-153）

4. **LoopFormIntake 構造体**: 抽出結果を整形して返す

**重要な洞察**:
- これらは **LoopForm → LoopScopeShape への変換ロジック** であり、
- **LoopScopeShape::from_loop_form() が既に内部で実行している処理**！

---

## 4. 移行設計

### 4.1 現在の呼び出しフロー（Before）

```rust
// json_v0_bridge/lowering/loop_.rs（仮想例）
fn lower_loop_to_joinir(
    loop_form: &LoopForm,
    query: &impl MirQuery,
    mir_func: &MirFunction,
) -> Option<JoinIR> {
    // Step 1: Trio 箱を生成
    let var_classes = LoopVarClassBox::new();
    let exit_live_box = LoopExitLivenessBox::new();

    // Step 2: loop_form_intake を呼び出し
    let intake = intake_loop_form(
        loop_form,
        &var_classes,  // ← Trio 依存！
        query,
        mir_func,
    )?;

    // Step 3: LoopScopeShape を構築
    let scope = LoopScopeShape::from_existing_boxes(
        loop_form,
        &intake,
        &var_classes,      // ← Trio 依存！
        &exit_live_box,    // ← Trio 依存！
        query,
        func_name,
    )?;

    // Step 4: JoinIR lowering
    some_joinir_lowerer(&scope, ...)
}
```

### 4.2 Phase 70 後の呼び出しフロー（After）

```rust
// json_v0_bridge/lowering/loop_.rs（Phase 70版）
fn lower_loop_to_joinir(
    loop_form: &LoopForm,
    query: &impl MirQuery,
    mir_func: &MirFunction,
) -> Option<JoinIR> {
    // Step 1: LoopFormIntake を生成（Trio なし版）
    let intake = intake_loop_form_v2(
        loop_form,
        query,
        mir_func,
    )?;

    // Step 2: LoopScopeShape を構築（Trio 内部化）
    let scope = LoopScopeShape::from_loop_form(
        loop_form,
        &intake,
        query,
        func_name,
    )?;

    // Step 3: JoinIR lowering（変更なし）
    some_joinir_lowerer(&scope, ...)
}
```

### 4.3 intake_loop_form() の書き換え（核心部分）

#### Before（L169-182）

```rust
// LocalScopeInspector に snapshot を登録して VarClass 判定の土台を整える
let mut inspector = LocalScopeInspectorBox::new();
inspector.record_snapshot(loop_form.header, &header_vals_mir);
for (bb, snap) in &exit_snapshots {
    inspector.record_snapshot(*bb, snap);
}

let all_names: Vec<String> = header_vals_mir.keys().cloned().collect();
let classified = var_classes.classify_all(
    &all_names,
    &pinned_hint,
    &carrier_hint,
    &inspector,
    &exit_preds,
);

let ordered_pinned: Vec<String> = classified
    .iter()
    .filter(|(_, c)| matches!(c, LoopVarClass::Pinned))
    .map(|(n, _)| n.clone())
    .collect::<BTreeSet<_>>()
    .into_iter()
    .collect();
let ordered_carriers: Vec<String> = classified
    .iter()
    .filter(|(_, c)| matches!(c, LoopVarClass::Carrier))
    .map(|(n, _)| n.clone())
    .collect::<BTreeSet<_>>()
    .into_iter()
    .collect();
```

#### After（Phase 70 版）

```rust
// Phase 70: Trio 分類を削除し、pinned_hint/carrier_hint をそのまま返す
// 実際の分類は LoopScopeShape::from_loop_form() 内部で実施される

let ordered_pinned: Vec<String> = pinned_hint.into_iter().collect::<BTreeSet<_>>().into_iter().collect();
let ordered_carriers: Vec<String> = carrier_hint.into_iter().collect::<BTreeSet<_>>().into_iter().collect();
```

**削減効果**:
- **L169-182 の 14行を 2行に削減** → 85% 削減！
- Trio 依存を完全排除
- ロジック重複を解消（LoopScopeShape が SSOT に）

### 4.4 関数シグネチャの変更

#### Before

```rust
pub(crate) fn intake_loop_form(
    loop_form: &crate::mir::loop_form::LoopForm,
    var_classes: &LoopVarClassBox,  // ← 削除予定
    query: &impl MirQuery,
    mir_func: &MirFunction,
) -> Option<LoopFormIntake>
```

#### After

```rust
pub(crate) fn intake_loop_form_v2(
    loop_form: &crate::mir::loop_form::LoopForm,
    query: &impl MirQuery,
    mir_func: &MirFunction,
) -> Option<LoopFormIntake>
```

**変更内容**:
- `var_classes: &LoopVarClassBox` 引数を削除
- Trio を使わずに pinned/carrier hint を生成
- 実際の分類は呼び出し側が `LoopScopeShape::from_loop_form()` で実施

---

## 5. 拡張提案

### 5.1 現時点では拡張不要

LoopScopeShape は既に以下を提供している:

1. ✅ 変数分類（Pinned/Carrier/BodyLocalExit/BodyLocalInternal）
2. ✅ PHI 生成判定（header/exit）
3. ✅ 定義位置追跡（variable_definitions）
4. ✅ exit_live 計算（保守的近似）
5. ✅ 順序付き一覧（pinned_ordered/carriers_ordered）

**結論**: Phase 70 では既存 API のみで移行可能！

### 5.2 将来的な拡張候補（Phase 71+）

1. **精密 liveness 解析**（NYASH_EXIT_LIVE_ENABLE=1）
   - 現在: 保守的近似（全変数を live とみなす）
   - 将来: MIR スキャンによる精密解析（LoopFormOps 拡張後）

2. **Case-A 判定の統合**
   - 現在: `is_case_a_minimal_target()` が func_name で判定
   - 将来: LoopScopeShape に構造的判定を統合

3. **LoopFormIntake の統合**
   - 現在: LoopFormIntake と LoopScopeShape が分離
   - 将来: LoopScopeShape が LoopFormIntake の役割も吸収

---

## 6. 実装手順（Phase 70）

### 6.1 ステップ 1: intake_loop_form_v2() 実装

**ファイル**: `src/mir/join_ir/lowering/loop_form_intake.rs`

**変更内容**:
1. `intake_loop_form_v2()` を新規作成
2. `var_classes` 引数を削除
3. L169-182 の Trio 分類コードを削除
4. pinned_hint/carrier_hint をそのまま ordered_pinned/ordered_carriers に

**コード例**:

```rust
/// LoopForm + MIR から pinned/carrier hint を抽出する（Phase 70: Trio なし版）
pub(crate) fn intake_loop_form_v2(
    loop_form: &crate::mir::loop_form::LoopForm,
    query: &impl MirQuery,
    mir_func: &MirFunction,
) -> Option<LoopFormIntake> {
    // ... 既存の L31-167 までのコードは変更なし ...

    // Phase 70: Trio 分類を削除
    let ordered_pinned: Vec<String> = pinned_hint
        .into_iter()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    let ordered_carriers: Vec<String> = carrier_hint
        .into_iter()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();

    if ordered_pinned.is_empty() || ordered_carriers.is_empty() {
        return None;
    }

    Some(LoopFormIntake {
        pinned_ordered: ordered_pinned,
        carrier_ordered: ordered_carriers,
        header_snapshot: header_vals_mir,
        exit_snapshots,
        exit_preds,
    })
}
```

### 6.2 ステップ 2: 呼び出し側の移行

**対象ファイル**:
- `src/mir/join_ir/lowering/mod.rs`（もしあれば）
- json_v0_bridge の関連ファイル

**変更内容**:
1. `intake_loop_form()` → `intake_loop_form_v2()` に切り替え
2. Trio 生成コードを削除
3. `LoopScopeShape::from_existing_boxes()` → `LoopScopeShape::from_loop_form()` に変更

**コード例**:

```rust
// Before
let var_classes = LoopVarClassBox::new();
let exit_live_box = LoopExitLivenessBox::new();
let intake = intake_loop_form(loop_form, &var_classes, query, mir_func)?;
let scope = LoopScopeShape::from_existing_boxes(
    loop_form, &intake, &var_classes, &exit_live_box, query, func_name
)?;

// After
let intake = intake_loop_form_v2(loop_form, query, mir_func)?;
let scope = LoopScopeShape::from_loop_form(loop_form, &intake, query, func_name)?;
```

### 6.3 ステップ 3: テスト確認

**テスト対象**:
1. `loop_scope_shape/tests.rs` のテストが全て PASS
2. JoinIR lowering 系のテスト（json_v0_bridge 経由）
3. 既存の PHI 生成テスト（267/268 PASS 維持）

**確認コマンド**:

```bash
# 単体テスト
cargo test --release loop_scope_shape

# JoinIR lowering テスト
cargo test --release joinir

# 全テスト（退行チェック）
cargo test --release
```

### 6.4 ステップ 4: 旧関数の deprecation

**ファイル**: `src/mir/join_ir/lowering/loop_form_intake.rs`

**変更内容**:

```rust
/// 旧版（Phase 70 で deprecation 予定）
#[deprecated(since = "Phase 70", note = "Use intake_loop_form_v2() instead")]
pub(crate) fn intake_loop_form(
    loop_form: &crate::mir::loop_form::LoopForm,
    var_classes: &LoopVarClassBox,
    query: &impl MirQuery,
    mir_func: &MirFunction,
) -> Option<LoopFormIntake> {
    // 互換性のため一時的に残す
    intake_loop_form_v2(loop_form, query, mir_func)
}
```

### 6.5 ステップ 5: ドキュメント更新

**対象ファイル**:
- `src/mir/join_ir/lowering/loop_form_intake.rs` の冒頭コメント
- `src/mir/join_ir/lowering/loop_scope_shape/builder.rs` の Phase 48-6 コメント

**追加内容**:

```rust
//! # Phase 70: Trio 依存排除完了
//!
//! - intake_loop_form_v2(): Trio を使わずに hint のみを抽出
//! - LoopScopeShape::from_loop_form(): Trio を内部化して分類実施
//! - json_v0_bridge: Trio 依存ゼロ達成！
```

---

## 7. 移行の安全性保証

### 7.1 退行防止策

1. **段階的移行**: intake_loop_form_v2() を新規作成し、旧関数を deprecation
2. **既存テストの維持**: 267/268 PASS を維持
3. **互換レイヤー**: 旧関数から新関数を呼び出して動作検証

### 7.2 検証ポイント

| 項目 | 検証方法 | 期待結果 |
|------|---------|---------|
| PHI 生成 | 既存テスト実行 | 267/268 PASS 維持 |
| pinned/carrier 判定 | loop_scope_shape tests | 全 PASS |
| exit_live 計算 | JoinIR lowering テスト | 変化なし |
| MIR パース | preheader 変数抽出 | s/n/i 正常抽出 |

### 7.3 ロールバック計画

Phase 70 で問題が発生した場合:

1. `intake_loop_form_v2()` を削除
2. 旧 `intake_loop_form()` に戻す
3. Trio 依存を一時的に復活
4. 問題を調査して Phase 71 で再挑戦

---

## 8. 期待される効果

### 8.1 コード削減

| ファイル | Before | After | 削減率 |
|---------|--------|-------|-------|
| loop_form_intake.rs | 211行 | 197行 | **7% 削減** |
| json_v0_bridge 系 | Trio 生成コード | 削除 | **10-15行削減** |

### 8.2 設計改善

1. **責務の明確化**:
   - loop_form_intake: MIR パース + hint 生成
   - LoopScopeShape: 変数分類の SSOT

2. **依存の整理**:
   - json_v0_bridge: Trio を知らない
   - LoopScopeShape: Trio を内部化（builder.rs のみ）

3. **保守性向上**:
   - 分類ロジックの重複を解消
   - LoopScopeShape が唯一の分類実装に

### 8.3 Phase 48-6 設計の完成

Phase 48-6 の目標「Trio を builder.rs のみに封じ込める」が完全達成:

- ✅ json_v0_bridge: Trio 依存ゼロ
- ✅ loop_form_intake: Trio 呼び出しゼロ
- ✅ LoopScopeShape: Trio を内部化（builder.rs のみ知る）

---

## 9. 補足: LoopScopeShape::from_loop_form() の既存実装

Phase 48-2 で既に実装済み（`builder.rs` L64-81）:

```rust
/// Trio 引数なしで LoopScopeShape を構築（Trio 内部化）
pub(crate) fn from_loop_form(
    loop_form: &LoopForm,
    intake: &LoopFormIntake,
    query: &impl MirQuery,
    func_name: Option<&str>,
) -> Option<Self> {
    let var_classes = LoopVarClassBox::new();
    let exit_live_box = LoopExitLivenessBox::new();

    Self::from_existing_boxes(
        loop_form,
        intake,
        &var_classes,
        &exit_live_box,
        query,
        func_name,
    )
}
```

**重要な発見**:
- Trio の生成は **既に LoopScopeShape 内部で完結**！
- json_v0_bridge が Trio を渡す必要は **全くない**！
- Phase 70 は「呼び出し側を既存 API に変えるだけ」で完了！

---

## 10. まとめ

### 10.1 設計の核心

1. **loop_form_intake.rs**: Trio 分類を削除し、hint 抽出のみに専念
2. **LoopScopeShape::from_loop_form()**: 既存実装を活用（変更不要）
3. **json_v0_bridge**: Trio 生成コードを削除し、from_loop_form() を呼ぶだけ

### 10.2 Phase 70 の作業量

- **新規実装**: intake_loop_form_v2()（14行削減版）
- **呼び出し側修正**: Trio 生成コードの削除
- **テスト確認**: 既存テスト実行（267/268 PASS 維持）

**見積もり**: 1-2時間の実装 + 30分のテスト確認 = **合計 2.5時間**

### 10.3 Phase 70 完了後の状態

```
json_v0_bridge/
  └─ lowering/
      └─ loop_.rs
          ├─ intake_loop_form_v2() 呼び出し（Trio なし）
          └─ LoopScopeShape::from_loop_form() 呼び出し（Trio 内部化）

loop_scope_shape/
  └─ builder.rs
      ├─ from_loop_form() が Trio を内部生成
      └─ from_existing_boxes_legacy() が Trio を使用（Phase 48-6 境界）

loop_form_intake.rs
  ├─ intake_loop_form_v2() 実装（Trio なし、hint のみ）
  └─ intake_loop_form() deprecation（互換性維持）
```

**Phase 48-6 設計の完全実現**: Trio は builder.rs のみが知る層！

---

## Phase 70 実装 Checklist

- [ ] Step 1: intake_loop_form_v2() 実装（loop_form_intake.rs）
- [ ] Step 2: json_v0_bridge の呼び出し側修正
- [ ] Step 3: テスト確認（267/268 PASS 維持）
- [ ] Step 4: intake_loop_form() に deprecation マーク
- [ ] Step 5: ドキュメント更新（Phase 70 完了記録）
- [ ] Step 6: コミット（退行なし確認済み）

**実装準備完了！Phase 70 で Trio 依存ゼロを達成しよう！** 🚀
Status: Historical
