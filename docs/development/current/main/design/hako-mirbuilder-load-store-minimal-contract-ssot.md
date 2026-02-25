---
Status: SSOT
Scope: `.hako` mirbuilder の Load/Store 受理を docs-first で固定する（B1）
Decision: accepted (design-first before B2 implementation)
Updated: 2026-02-10
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/phases/phase-29bq/29bq-115-selfhost-to-go-checklist.md
  - docs/development/current/main/phases/phase-29bq/29bq-91-mirbuilder-migration-progress-checklist.md
  - lang/src/compiler/mirbuilder/README.md
  - src/runner/mir_json_v0.rs
---

# .hako MirBuilder Load/Store Minimal Contract (B1 SSOT)

## Goal

`MIW7` の次段として、Load/Store を **先回り実装せず** failure-driven で増やすための最小契約を固定する。

- 1受理形 = 1fixture = 1pin = 1commit
- 実装前に shape / freeze tag / pin command を固定する

## Non-goals

- `MIW8-10`（Call/NewBox/BoxCall の値バリエーション）を先に増やすこと
- nested control（break/continue 複合、深い if/loop）
- cleanup/fini の追加拡張
- `try`/`throw` の受理追加

## Precondition (hard gate)

`.hako` mirbuilder route は `--mir-json-file` で MIR JSON v0 を実行する。  
現状の v0 loader (`src/runner/mir_json_v0.rs`) は `load/store` を未受理。

したがって B2 は次の順序で進める:

1. `LS0` loader readiness: v0 loader で `load/store` を受理できる状態を先に固定
2. `LS1` Load minimal shape
3. `LS2` Store minimal shape

## Freeze tag contract

`.hako` mirbuilder 側の拒否は既存契約を維持:

- prefix: `[freeze:contract][hako_mirbuilder]`
- missing capability: `[cap_missing/stmt:*]` / `[cap_missing/expr:*]` / `[cap_missing/recipe_shape]`

## Minimal acceptance shapes

## LS1: Load minimal (phase19)

Program(JSON v0) の最小形:

- `Local(Int)` -> `Local(Var)` -> `Return(Var)`
- 例: `local x = 7; local y = x; return y`

受理条件:

- `Local(Var)` は 1段参照のみ（`Var` of existing local）
- binary/call/boxcall/newbox は混在させない
- return は `Var` のみ

pin target:

- fixture: `apps/tests/phase29bq_hako_mirbuilder_phase19_load_local_var_min.hako`
- smoke: `tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase19_load_local_var_min_vm.sh`
- expected: `rc=7`, stdout empty

## LS2: Store minimal (phase20)

Program(JSON v0) の最小形:

- `Local(Int)` -> `Assignment(Int)` -> `Return(Var)`
- 例: `local x = 1; x = 9; return x`

受理条件:

- assignment rhs は `Int` のみ（LS2時点）
- update var は既存 local に限定
- `Binary` などの rhs 拡張は後段

pin target:

- fixture: `apps/tests/phase29bq_hako_mirbuilder_phase20_store_assignment_int_min.hako`
- smoke: `tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase20_store_assignment_int_min_vm.sh`
- expected: `rc=9`, stdout empty

## Execution order (must)

1. `LS0`: v0 loader readiness pin（handwritten MIR JSON で load/store を直接検証）
2. `LS1`: Load minimal を 1commit で実装
3. `LS2`: Store minimal を 1commit で実装
4. 各ステップで `quick_suite` + `--only bq` を実行して green 固定

## Required checks per step

```bash
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_quick_suite_vm.sh
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq
```

## Done definition (B1/B2)

- B1: このSSOTが `CURRENT_TASK.md` / `29bq-115` / `29bq-91` / mirbuilder README と同期されている
- B2-LS1: phase19 fixture+pin が green
- B2-LS2: phase20 fixture+pin が green

