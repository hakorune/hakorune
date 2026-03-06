# Historical planner-payload residue SSOT

## Purpose

- historical planner-payload residue の整理計画

## Rules

- current runtime path は `Facts → Recipe → Verifier → Lower`
- planner_required では historical planner-payload lane を使わない（Recipe-first の一本道）
- planner-payload-shaped な routing 結果が再流入した場合は freeze:contract（silent fallback 禁止）
- この文書は historical cleanup ledger であり、新しい runtime 入口を定義しない

## Inventory

- runtime source から消えた historical planner-payload residue を監査し、再流入を防ぐ

Inventory command:

- rg -n "DomainPlan|DomainRecipe|domain_plan" src/mir/builder/control_flow/{plan,joinir} --glob '!*.md'

| Inventory | Status | Reason | Next |
|---|---|---|---|
| runtime source identifiers | done | `src/mir/builder/control_flow/{plan,joinir}/**` で 0 hit を維持 | active docs では historical note のみに限定 |

## Historical cleanup order

1) Pattern 系 wording cleanup（完了済み）
2) LoopScan / GenericLoop wording cleanup（完了済み）
3) active docs からの current-runtime 主語撤去（継続監査）
