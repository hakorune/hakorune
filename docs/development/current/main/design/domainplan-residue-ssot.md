# DomainPlan Residue SSOT

## Purpose

- DomainPlan 残骸の整理計画

## Rules

- planner_required では DomainPlan 経路は使わない（Recipe-first の一本道）
- planner_required では DomainPlan は一律禁止（例外なし）
- DomainPlan が “routing 結果” として残っている場合は freeze:contract（silent fallback 禁止）
- Planner 内部で一時的に DomainPlan を構築してもよいが、planner_required では **必ず抑制（recipe-only）**する

## Inventory

- DomainPlan の残存 variant を列挙し、削除候補 / 互換維持 / 後回し に分類する

Inventory command:

- rg -n "DomainPlan::" src/mir/builder/control_flow/plan

| Variant | Status | Reason | Next |
|---|---|---|---|
| LoopCondContinueWithReturn | keep | planner skeleton / recipe-only sentinel | suppress in planner_required (already) / consider removing by moving outcome to recipe-only plan-only |

## Entry Order

1) Pattern 系（完了済み/薄化済み）
2) LoopScan / GenericLoop（完了済み）
3) それ以外（残存分）
