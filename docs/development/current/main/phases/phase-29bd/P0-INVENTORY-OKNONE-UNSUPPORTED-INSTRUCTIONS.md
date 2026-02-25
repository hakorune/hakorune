# Phase 29bd P0: Inventory Ok(None) / Unsupported (docs-first)

## Goal

CorePlan purity Stage-2 の前提として、Ok(None) と unsupported/unstructured の発火点を棚卸しし、
strict/dev で「許容」か「Freeze すべき」かを SSOT 化する。release 既定の挙動は不変。

## Scope

対象は Plan/Composer/Normalizer/emit/verifier/edgecfg の fallback・unsupported 入口。
pattern-name 依存や by-name 回避は禁止。

## Steps

1) 検索（候補の列挙）

- Ok(None) 入口:
  - `rg -n "Ok\(None\)" src/mir/builder/control_flow/plan src/mir/join_ir` 
- unsupported/unstructured/fallback の語彙:
  - `rg -n "unsupported|unstructured|fallback" src/mir/builder/control_flow/plan src/mir/join_ir`
- Freeze 入口:
  - `rg -n "Freeze::|freeze" src/mir/builder/control_flow/plan src/mir/join_ir`
- edgecfg/verifier:
  - `rg -n "verify|contract" src/mir/builder/control_flow/plan src/mir/join_ir`

2) SSOT表を README に追記

`docs/development/current/main/phases/phase-29bd/README.md` に次の形式で追記する:

```
| Location | Condition | Current behavior | Strict/Dev policy | Notes |
| --- | --- | --- | --- | --- |
| <path:line> | <shape/feature> | Ok(None) | Freeze(contract) | gate対象 | 
```

- Strict/Dev policy は必ず以下のどれか:
  - Allow (tag only)
  - Freeze(contract)
  - Freeze(unsupported)
- gate対象のものは「Freeze(contract)」を優先

3) Next 更新

- `docs/development/current/main/10-Now.md`
- `docs/development/current/main/30-Backlog.md`
- `docs/development/current/main/design/coreplan-migration-roadmap-ssot.md`

P0 完了後は Next を P1 に更新する。

## Verification (optional)

- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## Commit

- `git add -A`
- `git commit -m "docs(phase29bd): start purity stage2 (fallback to zero)"`
