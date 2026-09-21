---
Status: closeout__2026-09-21__AcceptedSameAuthorityIfBranchPathExtension
Task: MIR-CALL-PARSER-LOOPBREAK-COMPOSITE-FOREST-BOUNDARY-D0
Current execution row: MIR-CALL-PARSER-LOOPBREAK-COMPOSITE-FOREST-BOUNDARY-D0
Date: 2026-09-21
Parent: mir-call-parser-loopbreak-composite-source-owner-i2-2026-09-21.md
Implementation permission: false; design decision closed, successor implementation is separately bounded
NextCard: MIR-CALL-PARSER-LOOPBREAK-COMPOSITE-FOREST-PATH-I0
---

# Parser composite LoopBreak forest boundary D0

## Six-line brief

```text
Decision: decide whether the existing resolver loop forest/binder can represent the selected parser composite without a second authority, or retain a typed NoSafeSlice boundary.
Source authority + canonical issuer: the same FunctionSemanticResolverSessionV1 forest/exit ledger and existing loop-forest binder consumed by the I2 source projection.
Non-authority: AST/name/line rescans, path reconstruction, LoopCond reclassification, LoopRouteContext, generic retry, compatibility fallback, VM/backend, or a parser-specific forest issuer.
Fail-fast boundary: parser_program_box.hako:81 root forest lookup, nested :131/:182 descendants under IfThen/IfElse, exact parent relation, and every break/continue/return exit.
Smallest next slice: map the two observed reject forms to the existing resolver forest products and choose one same-authority extension or ParkedSealed__NoSafeSlice with a reopen trigger.
Non-claims: no Rust/Hako implementation, package/Recipe/physical publication, caller switch, old-edge deletion, backend parity, or warning cleanup.
```

## Observed boundary

The I2 source-only projection is green for its synthetic nested-loop product,
foreign-owner rejection, and supported merged `parse` loops. The selected
parser outer loop at `parser_program_box.hako:81` is still rejected before a
composite product can be issued:

* the root is reported as `ForestLookup` by the compiler wrapper, but the
  resolver forest construction reaches `UnsupportedAncestry(IfThen/IfElse)`
  while building the root's descendant set;
* nested loop members below conditional branches report
  `ForestBinding(UnsupportedAncestor)` because the portable loop path grammar
  has no `IfThen`/`IfElse` steps.

The I2 test records these as typed rejects. It does not turn them into an empty
candidate, fallback, or package absence. The missing evidence is therefore a
forest/path capability decision, not a Recipe or physical lowering bug.

## Bounded decision tasks

| order | task | completion condition |
| --- | --- | --- |
| 1 | Root forest lookup census | identify the wrapper distinction: root forest construction rejects the conditional descendant ancestry rather than lacking the root loop bundle |
| 2 | Conditional descendant census | map `:131` and `:182` as `LoopBody -> IfThen` source paths and compare them with the existing `IfSourcePathStepV1` vocabulary |
| 3 | Authority decision | accept one same-resolver forest/path extension: permit `IfThen`/`IfElse` ancestry and add matching `LoopSourcePathStepV1` variants; no second issuer or route |
| 4 | Follow-up boundary | create one implementation card for resolver forest ancestry plus portable loop-path/schema verification; package/Recipe/physical rows remain closed until it passes |

## Accepted Decision — same resolver forest/path extension

The root loop bundle is already in the resolver's loop-region index. The
rejection is caused by the closed ancestry grammar, which currently permits
only `ScopeBody` and `LoopBody` after a loop body. The parser's two nested
members require the exact paths `Body(22) -> LoopBody(8) -> IfThen(3)` and
`Body(22) -> LoopBody(9) -> IfThen(8)`. The existing `if_recipe_contract`
already uses typed `IfThenItem`/`IfElseItem` path steps, so the vocabulary can
be extended in the existing loop source-path owner without inventing a second
source authority.

The accepted bounded extension is:

1. allow `IfThen`/`IfElse` segments in resolver forest parent calculation;
2. add corresponding `LoopSourcePathStepV1` variants in the existing loop
   recipe source-binding schema and portable adapter;
3. update the source-binding verifier so branch steps preserve the required
   parent prefix and never permit body/loop skips; and
4. keep the existing resolver-issued member order, parent indices, exit ledger,
   and LoopBreak composite issuer unchanged outside this path capability.

This is a source-path BoxCount extension with one authority and one existing
consumer grammar. It does not authorize a Recipe item, package admission,
physical lowering, fallback, or production caller switch. The implementation
slice is owned by the successor card below.
