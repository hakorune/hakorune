---
Status: selected__design_stop__parser_loop_forest_exit_coseal__2026-09-19
Task: MIR-CALL-PARSER-LOOP-FOREST-EXIT-COSEAL-D0
Date: 2026-09-19
Parent: mir-call-parser-loop-source-admission-d0-2026-09-19.md
Implementation permission: false; define the missing source product only
Classification: BoxCount; one finite parser-loop forest and exit contract
---

# Parser loop forest and exit co-seal D0

## Six-line brief

```text
Decision: define whether the existing callable-loop source authority can co-seal the finite ParserProgramBox.parse/2 loop forest, binding schedule, and resolved exits as one product.
Source authority + canonical issuer: resolver-owned CallableSemanticSourceLedgerView (`resolved_loop_source_context`/`resolved_loop_source_forest`/`resolved_exits`) consumed by the existing callable-loop source issuer; no second issuer.
Non-authority: LoopBreak AST facts, parser names/lines, AST/MIR rescans, route-local coordinates, VM/compatibility fallback, and independently paired exit or binding receipts.
Fail-fast boundary: exact outer loop site, ordered forest parent indices, owner/frame identity, every break/return transfer target, and source binding coverage must co-seal before Recipe or physical lowering.
Smallest next slice: map those source products to the existing portable LoopRecipe source binding and JoinSig/physical boundary, then name the first missing consumer field.
Non-claims: no parser implementation, static catalog/publication, resolver If support, legacy route re-entry, production switch, or compatibility retirement.
```

## Finite source contract

```text
owner       = ParserProgramBox.parse/2
root loop   = loop(cont_prog == 1)
children    = direct condition/body plus nested static_semis and loop(true)
exits       = resolved break/return records, each targeting its sealed loop/function region
bindings    = condition reads and body rebinds from the existing callable handoff
target      = ParserStringUtilsBox.starts_with/3 (consumer evidence only)
```

The parent source-admission card established that the loop itself is the
blocker. This card designs only the missing co-seal; it must not infer any
relation from the target call or from Hako source line numbers.

## Existing products and gaps

| Product | Reusable evidence | Missing for this shape |
| --- | --- | --- |
| `CallableLoopReadyBodyOnlyProductV1` | owner, loop site, direct condition/body binding rows, affine pre-effect claim | nested loop forest and resolved break/return transfer set |
| `VerifiedLoopSourceForestBindingV1` | resolver-issued source paths and parent-index validation against a portable Recipe | no parser-loop Recipe/JoinSig consumes this forest today |
| `ResolvedExitRecordV1` | exact origin and control-transfer target for each exit | no callable-loop source product retains exits with the binding schedule |
| `GenericLoopV1SemanticRecipeV1` | existing source Recipe and physical adapter | rejects `NestedLoopOutsideFirstCohort` and cannot carry this exit contract |
| `LoopBreakFacts` | legacy three-statement break facts | parser body is stateful/nested and its AST facts are not source authority |

The product must remain move-only and owner-branded. It cannot expose a
reacquirable `(forest, exits, bindings)` tuple or let a later consumer pair
independently issued rows.

## Ordered design tasks

| Order | Task | Completion condition |
| --- | --- | --- |
| 1 | Co-seal shape | Define the one product's fields, ownership, frame, forest order, and exit coverage; include direct and nested loop identities. |
| 2 | Recipe mapping | Show which existing `LoopRecipeSourceBindingV1`, `LoopJoinSigElaboratorV1`, and continuation contracts consume each field; missing mapping stays a typed gap. |
| 3 | Physical boundary | Inspect the existing loop physical adapter and name whether it can consume the extended Recipe without a second owner; otherwise retain `NoSafeSlice`. |
| 4 | Negative matrix | Pin foreign owner/frame, forest parent drift, missing exit, wrong transfer target, duplicate site, binding-site drift, and nested coverage rejection. |
| 5 | Decision/next card | Choose one authority extension or retain the typed terminal. If extension is accepted, create a separate implementation I0 with focused guards; no code in this D0. |

## Acceptance and non-claims

Acceptance is a source-to-Recipe authority matrix with one named physical
consumer or a typed `NoSafeSlice`. The static parent remains before
Cataloged/Selected until this matrix and its negative evidence are complete.
No new parser issuer, fallback, AST rewrite, MIR scan, VM route, or production
caller switch is authorized here.
