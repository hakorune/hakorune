# DomainPlan Thinning SSOT

## Purpose

- Recipe-first の一本道化（DomainPlan/Normalizer を薄化）
- Residue plan: domainplan-residue-ssot.md

## Principles

- Recipe → Verifier → Lower が SSOT
- DomainPlan は label 層に限定
- Normalizer は RecipeComposer へ寄せる

## Order (SSOT)

1) Pattern2/3/4/5 は維持（既に recipe-only）
2) Pattern1/8/9 を削減対象として先に整理
3) LoopScan/GenericLoop を最後に（影響範囲が大きい）

## Done

- Pattern1SimpleWhile: planner_required で DomainPlan 経路を抑制（recipe-only）
- Pattern1CharMap: planner_required で DomainPlan 経路を抑制（recipe-only）
- Pattern1ArrayJoin: planner_required で DomainPlan 経路を抑制（recipe-only）
- Pattern8BoolPredicateScan: planner_required で DomainPlan 経路を抑制（recipe-only）
- Pattern9AccumConstLoop: planner_required で DomainPlan 経路を抑制（recipe-only）
- LoopScanMethodsV0: planner_required で DomainPlan 経路を抑制（recipe-only）
- LoopScanMethodsBlockV0: planner_required で DomainPlan 経路を抑制（recipe-only）
- LoopScanPhiVarsV0: planner_required で DomainPlan 経路を抑制（recipe-only）
- LoopScanV0: planner_required で DomainPlan 経路を抑制（recipe-only）
- GenericLoopV1: planner_required で DomainPlan 経路を抑制（recipe-only）
- GenericLoopV0: planner_required で DomainPlan 経路を抑制（recipe-only）
