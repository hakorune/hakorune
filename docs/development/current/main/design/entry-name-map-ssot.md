Status: SSOT
Scope: Planner-first rule labels (human-readable)

# Entry Name Map (SSOT)

| RuleId (internal) | Display label |
| --- | --- |
| Pattern1SimpleWhile | LoopSimpleWhile |
| Pattern1CharMap | LoopCharMap |
| Pattern1ArrayJoin | LoopArrayJoin |
| Pattern2Break | LoopBreakRecipe |
| Pattern3IfPhi | IfPhiJoin |
| Pattern4Continue | LoopContinueOnly |
| Pattern5InfiniteEarlyExit | LoopTrueEarlyExit |
| Pattern6ScanWithInit | ScanWithInit |
| Pattern7SplitScan | SplitScan |
| Pattern8BoolPredicateScan | BoolPredicateScan |
| Pattern9AccumConstLoop | AccumConstLoop |
| LoopCondBreak | LoopExitIfBreakContinue |
| LoopCondContinueOnly | LoopContinueOnly |
| LoopCondContinueWithReturn | LoopContinueWithReturn |
| LoopCondReturnInBody | LoopReturnInBody |
| LoopTrueBreak | LoopTrueBreakContinue |
| GenericLoopV1 | LoopGenericRecipeV1 |
| GenericLoopV0 | LoopGenericFallbackV0 |

Notes:
- TSV / gate is kept in internal names.
- Display labels are for logs and human-facing explanations only.
