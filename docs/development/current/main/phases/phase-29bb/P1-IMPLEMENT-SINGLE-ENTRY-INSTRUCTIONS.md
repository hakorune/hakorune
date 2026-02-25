---
Status: Ready
Scope: code + docs
---

# Phase 29bb P1: Implement CoreLoopComposer single entry

## Goal

`coreloop_v0/v1/v2_*` を外へ露出しない **単一入口 API** を code 側に実装し、以後の採用（shadow/release）分岐は
この入口を経由する形へ移行できる足場を用意する。

この P1 では「中身を統合」しない。既存の v0/v1/v2 実装を **そのまま呼ぶだけ**。

## SSOT

- `docs/development/current/main/design/coreloop-composer-single-entry-ssot.md`

## Non-goals

- subset 拡張（facts/extractors/planner の拡張）
- 観測語彙の変更（FlowBox schema を維持）
- strict/dev と release の意味論差分の変更（挙動不変）

## Implementation

### Step 1: 単一入口モジュールを追加

追加（推奨）:

- `src/mir/builder/control_flow/plan/composer/coreloop_single_entry.rs`

実装する関数（名前は SSOT に合わせる）:

- `pub(in crate::mir::builder) fn try_compose_core_loop_from_facts(`
  - `builder: &mut MirBuilder,`
  - `facts: &CanonicalLoopFacts,`
  - `ctx: &LoopPatternContext,`
  - `) -> Result<Option<CorePlan>, String>`

選択規則（SSOT通り、facts/features のみで決める）:

1) `facts.nested_loop == true`:
   - `coreloop_v2_nested_minimal::try_compose_core_loop_v2_nested_minimal(...)`
2) `facts.value_join_needed == true`:
   - v1 側（存在する facts に応じて）を試す
   - 例: `pattern2_break/pattern3_ifphi/pattern5_infinite_early_exit/split_scan/scan_with_init`
3) else:
   - v0 側（存在する facts に応じて）を試す
   - 例: `scan_with_init/split_scan/pattern1_simplewhile`

注意:

- v1 の `scan_with_init` は現状 reject 固定（既存挙動維持）。
- 返り値の `Ok(None)` は「明らかに非対象」のみ（呼び出し側の gate 判定は別 SSOT）。

### Step 2: composer/mod.rs から公開する

更新:

- `src/mir/builder/control_flow/plan/composer/mod.rs`

やること:

- `pub(super) mod coreloop_single_entry;`
- `pub(in crate::mir::builder) use coreloop_single_entry::try_compose_core_loop_from_facts;`

### Step 3: shadow_adopt 側は “温存” しつつ P2 に備える

この P1 では `shadow_adopt.rs` の分岐統合はしない（P2）。
ただし、`try_compose_core_loop_from_facts` の unit test を追加して
v0/v1/v2 の選択が SSOT 通りであることだけを固定する（accept/reject の境界）。

## Tests (must)

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## Docs updates

- `docs/development/current/main/phases/phase-29bb/README.md`
  - P0 ✅ / P1 in-progress を反映
  - P1 指示書リンクを追加
- `docs/development/current/main/10-Now.md`
  - Next を `Phase 29bb P1` へ更新

## Commit

- `git add -A`
- `git commit -m "phase29bb(p1): add coreloop composer single entry"`

