---
Status: design_stop__2026-09-21__ParserForestAuthorityBoundary
Task: MIR-CALL-PARSER-LOOPBREAK-COMPOSITE-FOREST-BOUNDARY-D0
Current execution row: MIR-CALL-PARSER-LOOPBREAK-COMPOSITE-FOREST-BOUNDARY-D0
Date: 2026-09-21
Parent: mir-call-parser-loopbreak-composite-source-owner-i2-2026-09-21.md
Implementation permission: false; source-authority and bounded-owner decision only
NextCard: MIR-CALL-PARSER-LOOPBREAK-COMPOSITE-SOURCE-PACKAGE-I3
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

* the root reports `ForestLookup` from the existing resolver forest index;
* nested loop members below conditional branches report
  `ForestBinding(UnsupportedAncestor)` at `IfThen`/`IfElse` depth.

The I2 test records these as typed rejects. It does not turn them into an empty
candidate, fallback, or package absence. The missing evidence is therefore a
forest-authority capability decision, not a Recipe or physical lowering bug.

## Bounded decision tasks

| order | task | completion condition |
| --- | --- | --- |
| 1 | Root forest lookup census | identify why the resolver loop source forest is absent for the exact parser `:81` source site and whether the absence is intentional or an issuer defect |
| 2 | Conditional descendant census | map `:131` and `:182` parent/ancestor segments to the existing forest binding contract without reconstructing paths from syntax |
| 3 | Authority decision | either accept a same-resolver forest/binder extension with explicit parent and exit relations, or retain `ParkedSealed__NoSafeSlice` with owner and reopen trigger |
| 4 | Follow-up boundary | if accepted, create a separate implementation card for only the forest product; if rejected, leave package/Recipe/physical rows closed behind the named terminal |

No package or Recipe task may start from the current typed rejects. A local
green synthetic projection cannot substitute for the parser root's missing
forest authority.
