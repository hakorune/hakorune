---
Status: design stop
Date: 2026-07-26
Decision: NORMAL-SOURCE-PLAN0-D0 (pending consultation)
---

# NORMAL-SOURCE-PLAN0-D0

The bounded MirBuilder core is complete at the accepted scope: canonical
function exit, Script result, entry/result projection, atomic Raw publication,
the supported Raw VM lane, and one explicit no-import normal-file lane. The
next step must not add a second compiler or silently widen that lane.

This card is a design stop. No parser, Builder, backend, or default-route code
is authorized until the source-plan authority is answered and recorded as an
accepted decision.

## Questions that must be closed

1. **Classification authority** — Is a normal source unit classified once by a
   source-owned sealed plan (`ScalarRoot` / `CallableModule`), or may the
   front door select a route directly from the requested profile? The answer
   must identify the single owner and forbid reclassification after handoff.
2. **Profile relation** — Is `NormalFileNoImportVmReferenceV1` a permanent
   narrow reference profile, with a separate future
   `NormalFileCanonicalCoreVmReferenceV1`, or may the existing profile grow?
3. **Function capability handoff** — Which existing
   `SealedFunctionExitContractV1` / `PreparedFunctionDraftSealV1` products are
   admitted for Main and ordinary callables, and which source facts remain a
   typed pre-Builder rejection?
4. **Callable plan boundary** — Does one sealed callable catalog produce the
   exact child lowering plan, with no name lookup, fallback, or second body
   classifier? Define the owner for helper order, call graph, and entry relation.
5. **Script relation** — Does Script tail classification remain the existing
   source-owned `RawScriptBodyRecipeV1`, or is a new normal-only classifier
   proposed? A second `ValueId`/AST authority is forbidden.
6. **Failure and reuse law** — Where do unsupported annotations, imports,
   dynamic carriers, or heterogeneous results reject, and how is the original
   source/profile owner retained for `discard(self)` and later compiler reuse?
7. **Promotion boundary** — What exact evidence is required before a future
   canonical-core profile can replace any normal caller? Default routing,
   `compile_with_source`, imports, JSON, REPL, LLVM/native, and legacy caller
   retirement remain non-claims here.

## Fixed non-authority

```text
last Builder ValueId
module/function symbol scan
CLI backend string after typed selection
Legacy compile retry
AST rewrite or source text replacement
process status as source-result classification
```

## Required decision product

```rust
struct SealedNormalSourcePlanDecisionV1 {
    profile_relation: ...,
    source_classifier_owner: ...,
    callable_plan_owner: ...,
    script_result_owner: ...,
    failure_retention_owner: ...,
    next_executable_row: ...,
}
```

The product is documentation-only until accepted. After acceptance, the first
implementation row must be one bounded semantic slice with a fixture and a
structural guard; no default cutover is implied.
