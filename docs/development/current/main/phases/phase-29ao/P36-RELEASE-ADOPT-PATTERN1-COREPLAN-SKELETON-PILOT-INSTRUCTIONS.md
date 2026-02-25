---
Status: Ready
Scope: Stage-2 pilot（release既定でも CorePlan を採用する）を Pattern1 subset のみで開始する（仕様不変）
Related:
- docs/development/current/main/phases/phase-29ao/README.md
- docs/development/current/main/phases/phase-29ae/README.md
- docs/development/current/main/design/coreplan-migration-roadmap-ssot.md
- docs/development/current/main/design/coreplan-shadow-adopt-tag-coverage-ssot.md
---

# Phase 29ao P36: Stage-2 pilot — release adopt Pattern1 CorePlan skeleton (subset)

## 目的

- これまでの Stage-1（strict/dev の shadow adopt）で “CorePlan が正しい” を検出できる導線は整った。
- P36 は Stage-2 の最初の一歩として、**release既定でも** Pattern1 subset を `Facts → CorePlan(skeleton) → emit` に寄せる。
- ただし観測（タグ）・既定挙動（意味論/エラー文字列/恒常ログ）は不変を守る。

## 非目的

- Pattern2/3/5/6/7 の release flip（P37+）
- 新しい env var の追加
- by-name の分岐追加
- タグを release で出す（恒常ログ増加になるため禁止）

## 事前条件（SSOT）

- gate はこれ一本:
  - `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
- Pattern1 subset の安全ゲート（body=step-only）は既に固定済み:
  - `src/mir/builder/control_flow/plan/policies/pattern1_subset_policy.rs`

## 実装方針（安全順）

### Step 1: composer に “release adopt（タグ無し）” の入口を追加

- 対象:
  - `src/mir/builder/control_flow/plan/composer/shadow_adopt.rs`
- 追加する関数（例）:
  - `try_release_adopt_core_plan_for_pattern1(...) -> Result<Option<CorePlan>, String>`
- 条件:
  - `domain_plan` が Pattern1 であること
  - `outcome.facts` が存在し、facts 側も `pattern1_simplewhile` を持つこと
  - subset policy に通ること（extra stmt などは reject）
- 返り値:
  - 条件OKなら `Ok(Some(core_plan))`
  - 対象外なら `Ok(None)`

重要:
- **タグ（`[coreplan/shadow_adopt:*]`）は返さない**（release既定で出力しないため）。
- strict/dev での tag は既存の `try_shadow_adopt_core_plan()` を維持してよい（P35のcoverageで回帰固定済み）。

### Step 2: router で Pattern1 のみ release adopt を優先する

- 対象:
  - `src/mir/builder/control_flow/joinir/patterns/router.rs`
- 位置:
  - `lower_via_plan(builder, domain_plan, ctx)` の直前
- 形:
  - strict/dev: 既存どおり `try_shadow_adopt_core_plan()` を優先（タグ出力あり）
  - release: `try_release_adopt_core_plan_for_pattern1()` を試す（タグ出力なし）
  - どちらも `PlanVerifier::verify(&core_plan)` → `PlanLowerer::lower(...)`
  - 失敗時は既存どおり `lower_via_plan(...)` にフォールバック

Fail-Fast 方針:
- strict/dev: Pattern1 が選ばれたのに facts が欠ける / mismatch は `Err(...)` で落とす（silent fallback禁止）
- release: mismatch は `Ok(None)` で従来経路へ（既定挙動不変）

### Step 3: 回帰で“subset reject の負例”が release flip でも維持されることを確認

- 既存の負例スモーク:
  - `tools/smokes/v2/profiles/integration/joinir/phase29ao_pattern1_subset_reject_extra_stmt_vm.sh`
  - 期待: exit=3 かつ shadow-adopt tag 不在（P35で固定済み）
- P36は release flip なので、strict/dev 以外の別スモーク追加は不要（観測増加を避ける）

## 検証（必須）

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## コミット

- `git add -A`
- `git commit -m "phase29ao(p36): release adopt pattern1 coreplan skeleton pilot"`

## 成功条件

- release既定で挙動不変（quick 154/154 PASS）
- JoinIR gate（phase29ae pack）が緑
- strict/dev の tag gate と negative gate が崩れない
