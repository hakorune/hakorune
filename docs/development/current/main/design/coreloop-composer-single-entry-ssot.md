---
Status: Retired
Scope: CoreLoopComposer single entry (feature-driven)
Related:
- docs/development/current/main/design/coreplan-flowbox-interface-ssot.md
- docs/development/current/main/design/coreplan-skeleton-feature-model.md
- docs/development/current/main/design/coreloop-composer-v0-v1-boundary-ssot.md
- docs/development/current/main/phases/archive/phase-29bb/README.md
---

# CoreLoopComposer single entry SSOT

Retired: 291x-754 removed the cfg-test-only `coreloop_v0` / `coreloop_v1` /
`coreloop_single_entry` composer shelf. This document is retained as historical
context for the retired version-selection design. Current route composition
should use active Facts/Recipe/Composer owners and `recipe_tree/*_composer.rs`;
do not revive this single-entry shelf.

目的: `coreloop_v0/v1/v2` を外に露出せず、**facts/features 駆動の単一入口**で CoreLoop 合成を行う。

## Single entry API (SSOT)

`src/mir/builder/control_flow/plan/composer/mod.rs` から公開する。

- `try_compose_core_loop_from_facts(...) -> Result<Option<CorePlan>, String>`

入力:

- canonical facts（features を含む）
- strict/dev と release の mode（タグ出力は呼び出し側ではなく observability SSOT に従う）

出力:

- `Ok(Some(core_plan))`: 合成成功
- `Ok(None)`: **明らかに非対象**（Ok(None) はここに限定する）
- `Err(...)`: strict/dev のみ fail-fast（FlowBox freeze へ）

## Version selection rule (SSOT)

内部の “v0/v1/v2” 選択は **facts/features のみ**で決める。by-name 分岐は禁止。

- `nested_loop == true` → nested-minimal 経路（現: v2）
- `value_join_needed == true` → value-join 経路（現: v1）
- それ以外 → no-join 経路（現: v0）

## Observability (SSOT)

- strict/dev のみ `flowbox/adopt` / `flowbox/freeze`
- release 既定は恒常ログ不変（タグなし）
- emit/merge は FlowBox タグ生成のために CFG/Facts を再解析しない

## Invariants (SSOT)

- 入口は **単一**（router/shadow_adopt からはこの API だけを呼ぶ）
- facts/features 以外の “外部情報” で版を切り替えない

## Gate (SSOT)

- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
