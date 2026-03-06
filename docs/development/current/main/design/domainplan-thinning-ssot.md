# Historical planner-payload thinning SSOT

## Purpose

- historical planner-payload-era wording / wrapper の cleanup 方針
- Residue plan: domainplan-residue-ssot.md

## Principles

- current runtime SSOT は `Recipe → Verifier → Lower`
- historical planner-payload wording は label/history 層に限定
- Normalizer は legacy wrapper として薄く保つ

## Order (SSOT)

1) break/if-phi/continue/infinite-early-exit routes は維持（legacy labels: Pattern2/3/4/5、既に recipe-only）
2) simple-while/bool-predicate-scan/accum-const-loop routes を削減対象として先に整理（legacy labels: Pattern1/8/9）
3) LoopScan/GenericLoop を最後に（影響範囲が大きい）

## Done (historical migration notes)

- LoopSimpleWhile（legacy label: LoopSimpleWhile）: planner_required で recipe-only 化済み
- LoopCharMap（legacy label: Pattern1CharMap）: planner_required で recipe-only 化済み
- LoopArrayJoin（legacy label: Pattern1ArrayJoin）: planner_required で recipe-only 化済み
- BoolPredicateScan（legacy label: Pattern8BoolPredicateScan）: planner_required で recipe-only 化済み
- AccumConstLoop（legacy label: Pattern9AccumConstLoop）: planner_required で recipe-only 化済み
- LoopScanMethodsV0: planner_required で recipe-only 化済み
- LoopScanMethodsBlockV0: planner_required で recipe-only 化済み
- LoopScanPhiVarsV0: planner_required で recipe-only 化済み
- LoopScanV0: planner_required で recipe-only 化済み
- GenericLoopV1: planner_required で recipe-only 化済み
- GenericLoopV0: planner_required で recipe-only 化済み
