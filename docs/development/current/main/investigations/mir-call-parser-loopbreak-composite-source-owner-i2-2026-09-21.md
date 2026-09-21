---
Status: closeout__2026-09-21__SourceProductAndTypedParserForestBoundary
Task: MIR-CALL-PARSER-LOOPBREAK-COMPOSITE-SOURCE-OWNER-I2
Current execution row: MIR-CALL-PARSER-LOOPBREAK-COMPOSITE-SOURCE-OWNER-I2
Date: 2026-09-21
Parent: mir-call-parser-loopbreak-composite-source-product-i1-2026-09-21.md
Implementation permission: true for one source-only composite projection product and focused guards; no package/Recipe/physical/cutover change
NextCard: MIR-CALL-PARSER-LOOPBREAK-COMPOSITE-FOREST-BOUNDARY-D0
---

# Parser composite LoopBreak source-owner projection I2

## Six-line brief

```text
Decision: extend the existing LoopBreak source owner with a typed composite
  projection product; keep direct and composite candidates explicitly typed.
Source authority + canonical issuer: one ResolvedFunctionLoweringInputV1,
  its resolver loop forest/exit ledger, and the existing LoopBreak source owner.
Non-authority: LoopCond reclassification, LoopRouteContext, legacy routes,
  AST/name/line rescans, Recipe keys, BodyId/StmtRef, physical IDs, fallback,
  retry, package filtering, or backend parity.
Fail-fast boundary: foreign root, missing/duplicate/out-of-root body role,
  unsupported body child, missing/duplicate exit, or exit-target mismatch.
Smallest next slice: issue the parser root :81 composite condition/body/
  forest/exit projection while retaining nested :131/:182 roles, then verify it.
Non-claims: no Recipe/package/physical publication, production switch, old-edge
  deletion, VM work, warning cleanup, or whole parser acceptance.
```

## Accepted same-owner extension

The I1 audit found no independent existing owner that can retain the parser
composite body and resolver exit ledger. The accepted decision is therefore a
same-owner extension: the existing LoopBreak source projection/Facts owner
remains the sole owner for this family, and its source issuer may classify one
resolved invocation as a direct or typed composite source projection. The
projection retains the resolver root, condition, ordered body inventory, the
existing nested forest when that forest is issuable, and every exit record as
one source product. No later pass may reconstruct those relations from names,
AST rescans, or line numbers.

The direct three-statement projection keeps its existing acceptance and
physical path. The composite product is a new typed source shape in the same
owner; it does not make the existing direct Recipe accept arbitrary bodies.
The next package row must choose the existing package retention slot and the
next Recipe row must reuse the existing exit-only block Recipe vocabulary.

## Authority chain and boundaries

The source input and resolver forest are the only semantic authorities. The
existing LoopBreak body-inventory helper issues ordered source roles and nested
loop membership. The existing LoopCond forest projection may be co-sealed as
the resolver exit ledger, but LoopCond admission and its one-body route remain
outside this family. A future Recipe producer may call
`try_build_exit_allowed_block_recipe`; I2 must not mint `RecipeItem`, `StmtRef`,
`BodyId`, target identities, or physical MIR IDs.

The associated-source physical spine remains the intended downstream consumer
after a composite Recipe exists. I2 does not connect that consumer, alter the
legacy `LoopRouteContext` route, add a fallback, or change package admission.

## Finite acceptance matrix

| input / witness | I2 result | required evidence |
| --- | --- | --- |
| parser root `:81` | typed forest-boundary rejection | existing forest issuer reports `ForestLookup`; no composite product is fabricated |
| nested loop sites `:131` and `:182` | nested inventory entries | source order and parent relation remain resolver-owned |
| resolver exits at `:85`, `:95`, `:104`, `:109`, `:127`, `:142`, `:161`, `:165`, `:186`, `:188`, `:194` | one co-sealed exit ledger | no duplicate, foreign, or out-of-root exit record |
| foreign root or source unit | typed rejection before effects | owner/source identity check |
| duplicate, missing, or out-of-root role | typed rejection before effects | body-inventory guard |
| unsupported child (`ScopeBox`, `TryCatch`, `TaskScope`, or opaque route) | typed rejection before effects | explicit source-body reject |
| existing direct three-statement LoopBreak | unchanged direct projection | existing focused direct matrix remains green |

The parser-row sites are a finite acceptance boundary, not a claim that the
full parser package or executable path is complete. If the current focused
fixture cannot expose the ten exit records directly, record that as missing
evidence and keep package/physical claims open rather than manufacturing a
receipt.

## I2 work order

1. Reuse the existing body-inventory issuer without a second source scanner;
   expose only the narrow crate-visible seam needed by the composite module.
2. Add a typed composite projection module that co-seals root/condition/body,
   the existing resolver forest, and the ordered body inventory.
3. Add focused positive and negative guards for direct compatibility, nested
   body retention, foreign roots, and the parser's typed forest boundary.
4. Run the focused projection test once with the repository's existing warning
   baseline. Do not launch another Cargo process concurrently.
5. Close I2 with source-product evidence and hand the exact retained fields
   plus the parser forest blocker to the next design card.

No production caller, package admission, physical lowering, or old-edge delete
is part of this row. Those actions require their own source-to-Recipe and
caller-zero evidence.

## Implementation and evidence — 2026-09-21

The same-owner module `src/mir/compiler/loop_break_composite_source_projection.rs`
now co-seals the root/condition/body, ordered body inventory, existing forest
product, and exit ledger. It exposes no Recipe, package, physical, or backend
identity. The body inventory seam is crate-visible only; the direct projection
shape is unchanged.

Focused command:

```text
CARGO_BUILD_JOBS=4 cargo test --profile quick --lib loop_break_composite_source_projection
```

Result: **3 passed, 0 failed**, with the repository warning baseline of 560
warnings. The synthetic nested-loop/root-exit product and foreign-root guard
are green. The merged parser check exercises supported `parse` loops, while
the selected outer `parser_program_box.hako:81` shape remains a named typed
boundary: `ForestLookup` for the root and `ForestBinding(UnsupportedAncestor)`
for conditional nested loop members. This is evidence for the next design
slice, not package or source-to-MIR acceptance.

The next card must decide whether the existing resolver forest/binder can be
extended in the same authority to represent those conditional descendants and
the missing root forest, or whether the parser composite remains
`ParkedSealed__NoSafeSlice`. No package/Recipe work is admitted until that
decision is accepted.
