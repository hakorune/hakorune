---
Status: SSOT
Scope: Type system responsibilities (compiler-first / fail-fast / no rewrite)
Related:
- docs/reference/language/types.md
- docs/development/current/main/design/compiler-expressivity-first-policy.md
- docs/development/current/main/design/lego-composability-policy.md
- docs/development/current/main/design/coreplan-skeleton-feature-model.md
- docs/development/current/main/design/planner-entry-guards-ssot.md
---

# Type System Policy (SSOT)

目的: “型”の責務を 1 箇所に押し込み、AI/開発者が **MirType に全部を背負わせて事故る**のを防ぐ。

前提:
- Everything is Box / 動的言語（実行時チェック + fail-fast）
- no rewrite（analysis-only view で観測する）
- compiler-first（selfhost は入口、目的ではない）

## 1. 3つに分ける（責務の境界）

### 1.1 `MirType`（コード生成のための表現）

- 目的: MIR/バックエンド（VM/LLVM/WASM）で “値” を表現するための最小情報。
- `Unknown` を許す（表現上の都合）。
- **禁止**: `MirType` で “言語仕様の真偽判定/演算子意味/例外” を決めない。

### 1.2 `RuntimeTypeTag`（実行時SSOT）

- 目的: **fail-fast の唯一の真実**（truthiness / binop / compare / cast など）。
- `Unknown` を “許容” しない（未定義は TypeError で落とす）。
- SSOT（言語仕様）は `docs/reference/language/types.md`。
  - truthiness（`to_bool_vm`）/ `String + <any>` / `Void/Null` の扱いはここで固定する。

### 1.3 `TypeView`（解析・観測専用）

- 目的: Facts/Canon/Planner の **観測**（analysis-only）。
- `Unknown` を許す。
- **禁止**: TypeView による AST rewrite、評価順変更、最適化判断（挙動変更）をしない。
- 許容: `Freeze` の "理由" を良くする、`[plan/reject]` の handoff を選ぶための補助（挙動は変えない）。

#### TypeView 設計詳細（T3 確定）

**責務**:
- MirType / ConstValue / value_types から「観測用の型情報」を作成
- Planner / Canon / Facts が「この値は何っぽいか」を知るための入口
- 最適化ヒント（routing, callee resolution, dead code 推定）に使用可能

**構造案**（Phase 15+ で実装）:
```rust
pub enum TypeView {
    Primitive(PrimitiveKind),  // Integer, Float, Bool, String, Void
    Box(String),               // "ArrayBox", "MapBox", etc.
    Future(Box<TypeView>),
    WeakRef,
    Unknown,
}
```

**禁止事項（型で守る）**:
1. ❌ AST rewrite の判断に使用しない
2. ❌ 評価順序の変更判断に使用しない
3. ❌ TypeError / fail-fast の判断に使用しない（それは RuntimeTypeTag の責務）
4. ❌ 実行結果を変える最適化判断に使用しない

**許容事項**:
1. ✅ `[plan/reject]` の理由を改善する（ログ品質向上）
2. ✅ `Freeze` の理由を詳細化する（診断向上）
3. ✅ callee routing の **ヒント** として使用（Known-only rewrite など）
4. ✅ dead code 推定の **ヒント** として使用（DCE pass での参考情報）

**導入タイミング**:
- Phase 15+: 必要になった箇所から段階的に導入
- 広域置換はしない（MirType はそのまま維持、TypeView は観測専用として並存）

## 2. “型”に関するSSOTの優先順位

1. 言語仕様（実行時の真実）: `docs/reference/language/types.md`
2. 実装の入口（観測/ガード）: `docs/development/current/main/design/planner-entry-guards-ssot.md`
3. コード生成の表現（MirType）は 1) を **変えない**

## 3. 運用ルール（AI向けチェックボックス）

型に触る変更をする前に、必ずチェック:

- [ ] これは `MirType` の変更？ `RuntimeTypeTag` の変更？ `TypeView` の変更？（主語を 1 つに固定）
- [ ] 言語仕様SSOT（`docs/reference/language/types.md`）に **矛盾**がない
- [ ] “挙動変更” が混ざるなら、必ず **strict/dev 限定**かつ SSOT に明記（既定挙動は不変）
- [ ] `Ok(None)` 黙殺を増やさない（必要なら `[plan/reject]` に理由を出す）
- [ ] fast gate が緑（FAIL のまま commit しない）

## 4. 当面の方針（compiler-first）

- 既定は "完全動的 + fail-fast" を維持する。
- 型注釈（将来）は **解析・診断用途**が主。既定でコード生成に影響させない。
  - 影響させるなら、専用フラグ + SSOT + gate を必須にする。

## 5. 現状（Phase 15 T4 完了時点）

### 5.1 実装状況

| 層 | 存在 | 場所 | 状態 |
|----|------|------|------|
| MirType | ✅ | `src/mir/types.rs` | コード生成/表現で使用（OK）|
| RuntimeTypeTag | ✅ | `src/backend/runtime_type_tag.rs` | **T2 で導入済み**（VMValue の粗分類） |
| RuntimeTypeSpec | ✅ | `src/backend/runtime_type_spec.rs` | **T4 で導入済み**（意味論 SSOT） |
| TypeView | ❌ | なし | **未実装**（T3 で設計、Phase 15+ で必要時に導入） |

### 5.2 意味論リーク箇所（要除去）

| ファイル | 問題 | 移設先 |
|---------|------|--------|
| (なし) | - | T4 で完了 |

### 5.3 正しく実装されている箇所

| ファイル | 内容 | 状態 |
|---------|------|------|
| `src/backend/abi_util.rs` | `to_bool_vm`, `eq_vm`, `tag_of_vm` | ✅ VMValue 直接使用 |
| `src/backend/runtime_type_tag.rs` | `tag_from_vmvalue`, `tag_to_str` | ✅ VMValue 直接使用 |
| `src/backend/runtime_type_spec.rs` | `matches_spec`, `spec_from_mir_type` | ✅ MirType 遮断済み |
| `src/backend/mir_interpreter/handlers/arithmetic.rs` | `handle_binop`, `eval_binop`, `eval_cmp` | ✅ VMValue 直接使用 |

## 6. 移行ロードマップ（T0-T4）

### Phase 14（docs のみ）✅ 完了

- **T0**: ✅ SSOT 文書確定（このファイル + `types.md` に実装アンカー追記）
- **T1**: ✅ MirType 棚卸し（分類表を作成、意味論リーク箇所を明記）
- **T3**: ✅ TypeView 設計記録（責務/禁止事項のみ、コード変更なし）

### Phase 15+（実装）✅ 完了

- **T2**: ✅ `src/backend/runtime_type_tag.rs` 新規作成（RuntimeTypeTag enum + tag_to_str）
  - `abi_util.rs` の `tag_of_vm` を `tag_to_str(tag_from_vmvalue(v))` に変更（SSOT集約）
- **T4**: ✅ `src/backend/runtime_type_spec.rs` 新規作成（意味論ロジック分離）
  - `type_ops.rs` から `matches_mir_type()` を削除、`spec_from_mir_type()` + `matches_spec()` に変更
  - MirType 依存を完全遮断（意味論ロジックは MirType に一切依存しない）
  - 挙動は `types.md` と完全一致を維持（挙動変更なし）

### 完了条件

- [x] MirType で「意味論判定」している箇所が 0 件
- [x] RuntimeTypeSpec が「実行時の型の意味論」の唯一の入口
- [x] RuntimeTypeTag が「VMValue の粗分類」の入口（意味論 SSOT ではない）
- [ ] TypeView が「観測専用」で責務明確化（Phase 15+ で必要時に導入）

## 7. 型追加時のチェックリスト

新しい型を追加する際は、以下の主語を確認:

- [ ] **RuntimeTypeSpec**: 意味論判定が必要？ → `runtime_type_spec.rs` に variant 追加
- [ ] **RuntimeTypeTag**: VMValue の粗分類が必要？ → `runtime_type_tag.rs` に variant 追加
- [ ] **MirType**: コード生成/表現が必要？ → `mir/types.rs` に variant 追加
- [ ] **TypeView**: 観測用途が必要？ → 将来実装時に追加（現状なし）
- [ ] **types.md**: 言語仕様 SSOT に追記
- [ ] **Gate**: 挙動変更なら gate 追加

