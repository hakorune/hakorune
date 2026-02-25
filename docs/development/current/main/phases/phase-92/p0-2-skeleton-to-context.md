# Phase 92 P0-2: Skeleton Integration to LoopPatternContext (Option A)

Status: Historical  
Scope: Phase 92 P0-2 の試行配線ログ（Option A）。現行の入口は `docs/development/current/main/phases/phase-92/README.md`。  
Related:
- `docs/development/current/main/phases/phase-92/README.md`

## 概要

ConditionalStep情報をPattern2 lowererに渡すため、LoopPatternContextにSkeletonフィールドを追加しました。

## Update（Phase 92 P1）

Phase 92 P1 で境界を整理し、routing 層から skeleton を取り除いた。
本ドキュメントは「P0-2 で一度試した配線（Option A）」の記録として残し、現行の入口は `docs/development/current/main/phases/phase-92/README.md` を SSOT とする。

## SSOT原則

**中心的なルール**: Canonicalizerが一度検出したConditionalStep情報を、lowering側で再検出しない。Skeleton経由でUpdate情報を渡す。

## 実装内容

### 1. router.rs - LoopPatternContextの拡張

**追加フィールド**:
```rust
/// Phase 92 P0-2: Optional LoopSkeleton from canonicalizer
/// This provides ConditionalStep information for Pattern2 lowering.
/// None if canonicalizer hasn't run yet (backward compatibility).
/// SSOT Principle: Avoid re-detecting ConditionalStep in lowering phase.
pub skeleton: Option<&'a LoopSkeleton>,
```

**新規メソッド**:
```rust
/// Phase 92 P0-2: Set skeleton (for canonicalizer integration)
pub(crate) fn with_skeleton(mut self, skeleton: &'a LoopSkeleton) -> Self {
    self.skeleton = Some(skeleton);
    self
}
```

**変更内容**:
- `skeleton: None`をデフォルト値として`new()`に追加
- 後方互換性を保つため、Optionalとして実装

### 2. parity_checker.rs - Skeletonの取得と返却

**関数シグネチャ変更**:
```rust
// Before:
pub(super) fn verify_router_parity(...) -> Result<(), String>

// After (Phase 92 P0-2):
pub(super) fn verify_router_parity(...)
    -> (Result<(), String>, Option<LoopSkeleton>)
```

**変更理由**:
- 既に`canonicalize_loop_expr()`を呼び出していた
- Skeletonを破棄せず、呼び出し側に返すことで再利用可能に
- パリティチェックとSkeleton取得の2つの責務を同時に実行

### 3. routing.rs - Skeletonのコンテキストへの設定

**実装パターン**:
```rust
// Phase 92 P0-2: Get skeleton from canonicalizer for Option A
let skeleton_holder: Option<crate::mir::loop_canonicalizer::LoopSkeleton>;
if crate::config::env::joinir_dev_enabled() {
    let (result, skeleton_opt) = self.verify_router_parity(condition, body, func_name, &ctx);
    result?;
    skeleton_holder = skeleton_opt;
    if skeleton_holder.is_some() {
        // Set skeleton reference in context (must use holder lifetime)
        ctx.skeleton = skeleton_holder.as_ref();
    }
} else {
    skeleton_holder = None;
}
```

**設計ポイント**:
- `skeleton_holder`でライフタイムを延長
- `ctx.skeleton = skeleton_holder.as_ref()`で参照を設定
- `joinir_dev_enabled()`時のみ有効（パフォーマンス考慮）

### 4. pattern2_with_break.rs - ConditionalStepの検出

**can_lower()への追加**:
```rust
// Phase 92 P0-2: Check skeleton for ConditionalStep support
if let Some(skeleton) = ctx.skeleton {
    use crate::mir::loop_canonicalizer::UpdateKind;

    // Count ConditionalStep carriers
    let conditional_step_count = skeleton.carriers.iter()
        .filter(|c| matches!(c.update_kind, UpdateKind::ConditionalStep { .. }))
        .count();

    if conditional_step_count > 0 {
        if ctx.debug {
            trace::trace().debug(
                "pattern2/can_lower",
                &format!(
                    "Phase 92 P0-2: Found {} ConditionalStep carriers in skeleton",
                    conditional_step_count
                ),
            );
        }

        // Phase 92 P0-2: ConditionalStep support enabled
        // Pattern2 can handle these via if-else JoinIR generation
        // TODO: Implement actual lowering in cf_loop_pattern2_with_break_impl
    }
}
```

**実装戦略**:
- まず検出ロジックのみ実装（TODOマーク付き）
- 実際のloweringは次のフェーズで実装
- デバッグトレース追加で動作確認可能

## 箱化モジュール化の原則

### 責任分離

| モジュール | 責任 |
|-----------|------|
| **loop_canonicalizer** | LoopSkeleton生成、ConditionalStep検出 |
| **parity_checker** | パリティ検証、Skeleton取得 |
| **routing** | Skeletonのコンテキスト設定 |
| **pattern2_with_break** | ConditionalStepのlowering（将来実装） |

### Fail-Fast原則

- **未対応ケース**: TODOコメントで明示
- **エラーケース**: 既存のエラーハンドリングを維持
- **検証**: can_lower()で早期チェック

## 後方互換性

### 既存パターンへの影響

- **Pattern 1-5**: `ctx.skeleton`は`None`のまま、既存動作に影響なし
- **Pattern 2**: Skeletonがある場合のみ追加機能が有効化
- **テスト**: 全20テスト中18テスト成功（2テスト無視）

### 段階的導入

1. **Phase 92 P0-2** (このフェーズ): Skeleton配線のみ
2. **Phase 92 P1**: ConditionalStep lowering実装
3. **Phase 92 P2**: carrier_update_emitter統合

## ビルド結果

```bash
cargo check --release
# ✅ 成功 (警告のみ、エラーなし)

cargo test --release --lib pattern2
# ✅ 18 passed; 0 failed; 2 ignored
```

## 次のステップ (Phase 92 P1)

### ConditionalStep Lowering実装

1. **carrier_update_emitter.rs**:
   - `UpdateKind::ConditionalStep`のマッチング追加
   - if-else形式のJoinIR生成

2. **loop_update_analyzer.rs**:
   - ConditionalStep用の処理追加（必要に応じて）

3. **pattern2_with_break.rs**:
   - TODO実装
   - Skeletonからのthen_delta/else_delta読み取り
   - JoinIR if-else構造の生成

### テストケース追加

- `test_escape_sequence_loop()`: `i += 2` vs `i += 1`
- `test_conditional_delta_carriers()`: 複数キャリア対応

## ファイル一覧

実装で変更されたファイル:
- `/home/tomoaki/git/hakorune-selfhost/src/mir/builder/control_flow/joinir/patterns/router.rs`
- `/home/tomoaki/git/hakorune-selfhost/src/mir/builder/control_flow/joinir/parity_checker.rs`
- `/home/tomoaki/git/hakorune-selfhost/src/mir/builder/control_flow/joinir/routing.rs`
- `/home/tomoaki/git/hakorune-selfhost/src/mir/builder/control_flow/joinir/patterns/pattern2_with_break.rs`

## 参照

- **Skeleton定義**: `/home/tomoaki/git/hakorune-selfhost/src/mir/loop_canonicalizer/skeleton_types.rs`
- **UpdateKind Contract**: skeleton_types.rs L63-109 (ConditionalStepのコメント)
