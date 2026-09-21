---
Status: design_stop__2026-09-21__CompositePackageRetentionAndRecipeBoundary
Task: MIR-CALL-PARSER-LOOPBREAK-COMPOSITE-SOURCE-PACKAGE-D0
Current execution row: MIR-CALL-PARSER-LOOPBREAK-COMPOSITE-SOURCE-PACKAGE-D0
Date: 2026-09-21
Parent: mir-call-parser-loopbreak-composite-forest-path-i0-2026-09-21.md
Implementation permission: false; package/Recipe/physical changes require this boundary decision
NextCard: MIR-CALL-PARSER-LOOPBREAK-COMPOSITE-SOURCE-PACKAGE-I3
---

# Parser composite LoopBreak package boundary D0

## Six-line brief

```text
Decision: choose one existing package/Recipe owner for the resolver-issued
  composite LoopBreak source product before any package admission or lowering.
Source authority + canonical issuer: the same resolved function input,
  conditional forest/exit product, and existing LoopBreak source projector;
  the normal semantic package may only co-seal and retain that product.
Non-authority: method names, selected-call filtering, AST/MIR rescans,
  LoopRouteContext, GenericLoop retry, VM/backend, fallback, or a second
  composite issuer.
Fail-fast boundary: exact package row, caller/source site, forest members,
  exit ledger, body inventory, Recipe item mapping, and terminal relation must
  be complete and one-shot before Cataloged/Selected or Builder effects.
Smallest next slice: decide the composite source-to-Recipe relation and the
  package loan shape for the finite parser root/child forest; no code yet.
Non-claims: no package receipt, Recipe item, physical MIR, production switch,
  old-edge deletion, VM parity, or warning cleanup.
```

## Finite acceptance boundary

The selected source product is the merged `ParserProgramBox.parse/2` root at
`parser_program_box.hako:81`, with resolver-issued children at `:131` and
`:182`. The source forest now retains exact paths through `IfThen`/`IfElse`,
the root/child parent indices, and the resolver exit ledger. The package row
must remain keyed by the existing batch slot, owner, function origin, and
exact source site; a method name or a selected target is not a key.

The package observer already owns complete per-declaration LoopBreak row
coverage and a one-shot loan. The unresolved question is the shape of the
candidate that can carry this composite product without dropping the nested
body inventory or reinterpreting the direct three-statement Recipe.

## Bounded decision tasks

| order | task | completion condition |
| --- | --- | --- |
| 1 | Package retention | decide whether the existing `LoopBreakSourcePackageLoanV1` can retain a typed direct/composite product in the same owner, with candidate/absence/residual states still explicit |
| 2 | Source-to-Recipe map | map every root/child body role, conditional path, resolver exit, and source call item to existing Recipe/JoinSig vocabulary, or name the exact missing owner; no line/name reconstruction |
| 3 | Physical consumer | verify whether the existing associated-source LoopBreak physical input can consume the composite Recipe without `LoopRouteContext` or Builder mutation; otherwise keep physical work out of I3 |
| 4 | Terminal and delete set | identify the named dependency terminal and the exclusive legacy caller/delete tuple; caller-zero is a later retirement condition, not an entry condition |
| 5 | Acceptance matrix | specify whole-package positive, supported-absence, foreign/duplicate/missing, residual, and selected parser dependency cases before implementation permission |

## Existing-owner census

* `normal_callable_semantic_package::loop_break_source` owns package-wide
  row coverage and the affine one-shot loan, but its current candidate type is
  the direct LoopBreak Facts product.
* `normal_callable_loop_source_facts::loop_break` owns direct topology and
  currently requires the three-statement body; it cannot silently absorb the
  parser forest by widening a predicate.
* `control_flow/plan/recipe_tree/loop_break_builder` and the associated-source
  physical adapter are downstream consumers. They may be extended only after
  the source-bound relation and exact item/exit mapping are accepted.
* `LoopRouteContext`, raw child entry, and the legacy composer remain
  non-authorities. They cannot be used to infer a composite Recipe or to
  manufacture a package candidate.

## Design-stop rule

Remain in `design_stop` if the existing package loan cannot carry the composite
product, if any source item or exit lacks a one-to-one Recipe relation, or if
the physical owner would need to rediscover source meaning. In that case the
successor must name one missing owner and a finite reopen trigger rather than
adding a second semantic receipt. Only after this D0 closes may I3 implement
the selected package/Recipe slice.
