---
Status: Ready
Scope: code+tests+docs（仕様不変）
Related:
  - docs/development/current/main/phases/phase-29ao/README.md
  - tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh
  - src/mir/builder/control_flow/joinir/patterns/router.rs
  - src/mir/builder/control_flow/plan/composer/shadow_adopt.rs
---

# Phase 29ao P31: shadow adopt の判定/Fail-Fast/タグを composer に集約（router を薄くする）

Date: 2025-12-30  
Status: Ready for execution  
Goal: `router.rs` に散っている shadow adopt の分岐（Pattern1/2/3/5/6/7 の facts検証 + subset gate + tag 出力）を `plan/composer` 側の SSOT 入口に集約し、router を “呼ぶだけ” の薄い orchestrator に縮退する。挙動（strict/dev の fail-fast・タグ・ログ）は不変。

## 背景

- P30 で Facts→CorePlan の入口自体は composer に集約できたが、router にはまだ
  - patternごとの `if strict_or_dev && matches!(...)` 連鎖
  - `facts missing/mismatch/compose rejected` のエラーパス
  - subset gate（Pattern2/6 の `outcome.plan` 条件）
  - タグ出力（P28/P29）
  が残っている。
- ここが残ると、今後の拡張（Pattern4/8/9 や feature合成の導線追加）のたびに router が肥大化し、SSOT が分散する。

## 非目的

- shadow adopt の範囲拡張（P31は構造整理のみ）
- エラーメッセージ変更
- タグ名変更（既存の `[coreplan/shadow_adopt:*]` を維持）
- 新しい env var 追加

## 実装方針（構造で解く）

### 1) composer/shadow_adopt に “判定 + compose + tag” の単一入口を追加

対象:
- `src/mir/builder/control_flow/plan/composer/shadow_adopt.rs`

追加（例）:
```rust
pub(in crate::mir::builder) struct ShadowAdoptOutcome {
    pub core_plan: CorePlan,
    pub tag: &'static str, // e.g. "[coreplan/shadow_adopt:pattern7_split_scan]"
}

pub(in crate::mir::builder) fn try_shadow_adopt_core_plan(
    builder: &mut MirBuilder,
    ctx: &LoopPatternContext,
    strict_or_dev: bool,
    domain_plan: &DomainPlan,
    outcome: &PlanBuildOutcome,
) -> Result<Option<ShadowAdoptOutcome>, String>;
```

責務:
- strict/dev でしか shadow adopt しない（strict_or_dev=false なら常に Ok(None)）
- Pattern2/6 の subset gate はここに集約（`outcome.plan` が planner 由来かどうかで判定）
- `facts missing/mismatch/compose rejected` の fail-fast をここに集約（メッセージは現状の router と一致させる）
- 成功時に返す `tag` は既存タグをそのまま返す（出力は router 側で `eprintln!("{}", tag)` に統一）

### 2) router は “1回呼ぶだけ” に縮退

対象:
- `src/mir/builder/control_flow/joinir/patterns/router.rs`

変更:
- `if strict_or_dev { ... }` の pattern 連鎖を削除し、代わりに 1 箇所だけ:
  - `if let Some(adopt) = composer::try_shadow_adopt_core_plan(...) ? { PlanVerifier::verify; eprintln!(tag); return PlanLowerer::lower(...); }`
- adopt しない場合は従来通り `lower_via_plan(builder, domain_plan, ctx)` に落とす

### 3) テスト（回帰ゲートで固定）

P28/P29 により、回帰ゲートの smoke が “タグ必須” になっているので、このリファクタは gate で必ず踏まれて検証される。

必須:
- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## docs 更新

- `docs/development/current/main/phases/phase-29ao/README.md`（P31 追加、Next 更新）
- `docs/development/current/main/10-Now.md` / `docs/development/current/main/30-Backlog.md` / `CURRENT_TASK.md`
- `docs/development/current/main/design/coreplan-migration-roadmap-ssot.md`（Current/Next 更新）

## コミット

- `git add -A`
- `git commit -m "phase29ao(p31): ssot shadow adopt routing in composer"`
