---
Status: design_stop__2026-09-21__ParserLoopBreakSourcePhysical
Task: MIR-CALL-PARSER-LOOPBREAK-SOURCE-PHYSICAL-I0
Current execution row: MIR-CALL-PARSER-LOOPBREAK-SOURCE-PHYSICAL-I0
Date: 2026-09-21
Parent: mir-call-parser-loopbreak-source-transport-i0-2026-09-20.md
Implementation permission: false until the source-bound physical owner and delete set are accepted
NextCard: none
---

# Parser LoopBreak source physical I0

## Six-line brief

```text
Decision: design one source-bound LoopBreak adapter that reuses the existing
  loop_v0 physical owner after exact source/Facts/Recipe co-seal.
Source authority + canonical issuer: the existing LoopBreak source Facts and
  Recipe owners, joined with the selected callable's resolver source relations.
Non-authority: synthetic AST/StmtRef construction, name remapping, a second
  LoopBreak Recipe, LoopRouteContext inference, legacy composer, or fallback.
Fail-fast boundary: named pre-effect rejection for missing, foreign, duplicate,
  out-of-root, order-mismatched, or non-direct source rows.
Smallest next slice: inventory the existing source-parts/loop_v0 consumer,
  exact LoopBreak source site and exit/forest relation, and finite old-edge set.
Non-claims: no route execution, publication, backend parity, VM work, or delete.
```

## Design tasks

| order | task | acceptance evidence |
| --- | --- | --- |
| 1 | Identify the existing physical owner | one method receives the transported candidate and reuses `CallableLoopSourceParts`/`lower_raw_loop_v0`; no new physical owner |
| 2 | Close source relation | candidate site, break/update/step items, exit/forest relation, package brand, and selected callable owner are co-sealed before effects |
| 3 | Define pre-effect rejects | missing, foreign, duplicate, out-of-root, order mismatch, and specialized/non-direct topology each have named rejection rows |
| 4 | Enumerate production callers and old edges | finite caller/terminal inventory and exclusive delete tuple; no caller-zero shortcut |
| 5 | Decide implementation boundary | accepted bounded slice or `NoSafeSlice` with missing issuer/owner named; no synthetic receipt or fallback |

## Existing transport handoff

`MIR-CALL-PARSER-LOOPBREAK-SOURCE-TRANSPORT-I0` already retains the package
product through install and moves it once into the selected lowering state.
That product is a transport input only. This card must consume the exact
source candidate rather than re-scan the AST or reconstruct a route from a
selected key.

The existing physical core is reusable only after source alignment is proven.
The legacy `loop_break_composer` and `route_loop_break_recipe` are not source
consumers. `LoopRouteContext`, synthetic `StmtRef`, and unconditional fallback
remain outside the authority chain.

## Required design-stop evidence

The card cannot enter `fast` until the same owner names the source issuer,
physical consumer, pre-effect terminal, and deletion set. Focused evidence must
cover one direct candidate, one typed absence, one specialized rejection, and
foreign/duplicate/out-of-root relations. A local transport green is not
source-to-MIR acceptance.

## Non-claims

The parser `starts_with/3` tuple remains stopped before physical lowering and
publication. No Cataloged/Selected claim, backend parity claim, source-to-MIR
claim, or legacy retirement claim is made here.
