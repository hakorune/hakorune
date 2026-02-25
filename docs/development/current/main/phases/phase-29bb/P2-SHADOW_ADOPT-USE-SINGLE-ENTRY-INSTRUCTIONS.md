---
Status: Ready
Scope: code + docs
---

# Phase 29bb P2: Converge `shadow_adopt.rs` onto the composer single entry

## Goal

`src/mir/builder/control_flow/plan/composer/shadow_adopt.rs` に残っている

- pattern ごとの重複
- v0/v1/v2 の選択ロジック（`value_join_needed` / `nested_loop`）の重複

を削減し、**composer 側の単一入口**（SSOT）に寄せる。

この P2 は「挙動不変の構造整理」だけを行う。

## SSOT

- Single entry contract: `docs/development/current/main/design/coreloop-composer-single-entry-ssot.md`
- Phase: `docs/development/current/main/phases/phase-29bb/README.md`
- Gate: `docs/development/current/main/phases/phase-29ae/README.md`

## Non-goals

- subset 拡張（facts/extractors/planner の拡張）
- 新しい env var の追加
- strict/dev の観測スキーマ変更（FlowBox schema を維持）
- 既存のエラー文字列・ログの変更（strict/dev 含めできるだけ保持）

## Implementation steps

### Step 1: composer 側に「DomainPlan を固定して合成する」薄い入口を追加

対象:

- `src/mir/builder/control_flow/plan/composer/coreloop_single_entry.rs`

追加（例）:

- `pub(super) fn try_compose_core_loop_for_domain_plan(`
  - `builder: &mut MirBuilder,`
  - `facts: &CanonicalLoopFacts,`
  - `ctx: &LoopPatternContext,`
  - `domain_plan: &DomainPlan,`
  - `) -> Result<Option<CorePlan>, String>`

要件:

- domain_plan の variant を **固定**したうえで合成する
  - 例: `DomainPlan::ScanWithInit(_)` のときは scan 経路のみ
  - `Ok(Some(_))` を返すのは “その domain_plan に対応する CorePlan” のみ
  - それ以外は `Ok(None)`（誤採用禁止）
- v0/v1/v2 の選択は facts/features のみ（SSOT）
  - `nested_loop` → nested-minimal（現 v2）を優先
  - `value_join_needed` → v1
  - else → v0
- 既存の `coreloop_v0/*`, `coreloop_v1/*`, `coreloop_v2_nested_minimal/*` を **そのまま呼ぶだけ**（内部の再設計はしない）

注意:

- `is_integer` は LoopFacts の専用 facts/normalizer 経路があるので、P2 では対象外（既存の専用関数を維持）

### Step 2: `shadow_adopt.rs` から v0/v1/v2 直接参照を撤去

対象:

- `src/mir/builder/control_flow/plan/composer/shadow_adopt.rs`

やること:

- 既存の helper（例: `try_compose_core_loop_scan_with_init` / `try_compose_core_loop_split_scan`）を削除
- Pattern6/7/2/3/5 の合成で v0/v1 を直接呼んでいる箇所を、
  Step 1 の `try_compose_core_loop_for_domain_plan(...)` 呼び出しに置換
- strict/dev の fail-fast 文言（`"... strict/dev adopt failed: ..."`）はできるだけ同一に保つ

期待する成果:

- `shadow_adopt.rs` が “pattern 固有の gate と、composer 単一入口呼び出し” だけになる
- v0/v1/v2 の選択・呼び分けは composer 側へ集約される

### Step 3: docs/phase tracking 更新

更新:

- `docs/development/current/main/phases/phase-29bb/README.md`
  - P2 を追加（リンク含む）
- `docs/development/current/main/10-Now.md`
  - Next を `Phase 29bb P2` に更新

## Acceptance

- Build:
  - `cargo build --release`
- Smokes (SSOT):
  - `./tools/smokes/v2/run.sh --profile quick`
  - `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
- FlowBox tags / gate は維持（strict/dev only、release 恒常ログ不変）

## Commit

- `git add -A`
- `git commit -m "phase29bb(p2): converge shadow_adopt onto single entry"`

