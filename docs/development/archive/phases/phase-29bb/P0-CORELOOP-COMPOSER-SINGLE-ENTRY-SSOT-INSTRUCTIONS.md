---
Status: Ready
Scope: docs-only
---

# Phase 29bb P0: CoreLoopComposer single entry SSOT (docs-only)

## Objective

`plan/composer` の CoreLoop 合成を **単一入口**に固定するための SSOT を定義する。

以後、JoinIR 側・router 側・smoke 側は “v0/v1/v2” を意識しない（内部実装の詳細）。

## Context

現状の実装は `composer/` 配下に複数の “版” を持つ:

- `src/mir/builder/control_flow/plan/composer/coreloop_v0.rs`
- `src/mir/builder/control_flow/plan/composer/coreloop_v1.rs`
- `src/mir/builder/control_flow/plan/composer/coreloop_v2_nested_minimal.rs`
- 入口分岐は `src/mir/builder/control_flow/plan/composer/shadow_adopt.rs` に散在

## SSOT: Single entry contract

### API (SSOT)

`src/mir/builder/control_flow/plan/composer/mod.rs` から以下を公開する（名前は例）:

- `try_compose_core_loop_from_facts(...) -> Result<Option<CorePlan>, String>`

入力:

- `facts`（canonical facts を含む）
- `ctx`（必要なら strict/dev フラグ）
- `mode`（`Shadow` / `Release` の違い。タグの有無は呼び出し側ではなく observability SSOT に従う）

出力:

- `Ok(Some(core_plan))`: 合成できた（Call site は verify→lower のみ）
- `Ok(None)`: 対象外（Ok(None) は “明らかに非対象” のみ）
- `Err(...)`: strict/dev のみ fail-fast で観測（FlowBox freeze へ）

### Version selection rule (SSOT)

“v0/v1/v2” は外へ露出しない。内部の選択規則は **facts/features のみ**で決まる:

- `nested_loop=true` → nested-minimal 専用経路（現: v2）
- `value_join_needed=true` → value-join 経路（現: v1）
- それ以外 → no-join 経路（現: v0）

### Observability (SSOT)

- strict/dev のみ `flowbox/adopt` / `flowbox/freeze` を出す
- release 既定は恒常ログ不変（タグを出さない）
- `[plan/fallback:*]` は SSOT 外（Phase 29ba で撤去済み）

## Implementation notes (for P1+)

P1 では “中身を統合しない”。**単一入口の関数を追加して、内部で既存 v0/v1/v2 を呼ぶだけ**に留める。

P2 では `shadow_adopt.rs` の分岐を単一入口に寄せ、重複を減らす。

## Acceptance

- SSOT が README/Related から辿れる（迷子防止）
- Gate が緑:
  - `./tools/smokes/v2/run.sh --profile quick`
  - `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

