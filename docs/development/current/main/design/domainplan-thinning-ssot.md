# DomainPlan Thinning SSOT

## Purpose

- Recipe-first の一本道化（DomainPlan/Normalizer を薄化）
- Residue plan: domainplan-residue-ssot.md

## Principles

- Recipe → Verifier → Lower が SSOT
- DomainPlan は label 層に限定
- Normalizer は RecipeComposer へ寄せる

## Order (SSOT)

1) break/if-phi/continue/infinite-early-exit routes は維持（legacy labels: Pattern2/3/4/5、既に recipe-only）
2) simple-while/bool-predicate-scan/accum-const-loop routes を削減対象として先に整理（legacy labels: Pattern1/8/9）
3) LoopScan/GenericLoop を最後に（影響範囲が大きい）

## Done

- LoopSimpleWhile（legacy label: Pattern1SimpleWhile）: planner_required で DomainPlan 経路を抑制（recipe-only）
- LoopCharMap（legacy label: Pattern1CharMap）: planner_required で DomainPlan 経路を抑制（recipe-only）
- LoopArrayJoin（legacy label: Pattern1ArrayJoin）: planner_required で DomainPlan 経路を抑制（recipe-only）
- BoolPredicateScan（legacy label: Pattern8BoolPredicateScan）: planner_required で DomainPlan 経路を抑制（recipe-only）
- AccumConstLoop（legacy label: Pattern9AccumConstLoop）: planner_required で DomainPlan 経路を抑制（recipe-only）
- LoopScanMethodsV0: planner_required で DomainPlan 経路を抑制（recipe-only）
- LoopScanMethodsBlockV0: planner_required で DomainPlan 経路を抑制（recipe-only）
- LoopScanPhiVarsV0: planner_required で DomainPlan 経路を抑制（recipe-only）
- LoopScanV0: planner_required で DomainPlan 経路を抑制（recipe-only）
- GenericLoopV1: planner_required で DomainPlan 経路を抑制（recipe-only）
- GenericLoopV0: planner_required で DomainPlan 経路を抑制（recipe-only）
