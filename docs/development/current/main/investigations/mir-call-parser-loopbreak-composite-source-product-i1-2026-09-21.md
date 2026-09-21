---
Status: design_stop__2026-09-21__CompositeSourceProductConsumerBoundary
Task: MIR-CALL-PARSER-LOOPBREAK-COMPOSITE-SOURCE-PRODUCT-I1
Current execution row: MIR-CALL-PARSER-LOOPBREAK-COMPOSITE-SOURCE-PRODUCT-I1
Date: 2026-09-21
Parent: mir-call-parser-loopbreak-composite-source-product-i0-2026-09-21.md
Implementation permission: false; select the existing owner that will retain the composite inventory before any code change
NextCard: MIR-CALL-PARSER-LOOPBREAK-COMPOSITE-SOURCE-PRODUCT-I1
---

# Parser composite LoopBreak source-product consumer I1

## Six-line brief

```text
Decision: choose one existing source/Facts owner to retain the composite
  LoopBreak body inventory and its resolver exit ledger.
Source authority + canonical issuer: the same resolved parser function input,
  resolver loop forest, and resolved exit ledger used by I0.
Non-authority: AST/name/line rescans, LoopRouteContext inference, generic
  located-body retries, new Recipe keys, physical IDs, fallback, or VM routes.
Fail-fast boundary: missing consumer, foreign root, dropped nested body, or
  exit/site cardinality mismatch is a NoSafeSlice rather than a new receipt.
Smallest next slice: audit the existing LoopBreak Facts/Recipe owner and the
  source physical adapter for a single exact composite inventory handoff.
Non-claims: no composite Recipe, package admission, production switch, or
  legacy-edge deletion until the owner and full source-to-Recipe mapping close.
```

I0 proved the source-only inventory guard on the existing direct projection.
The parser fixture's root `:81` and nested loops `:131`/`:182` still have no
selected consumer that can retain the ordered body roles and resolver exits
through Facts into the existing Recipe/physical owner. This is a design
boundary, not a permission to add a parallel product or to revive LoopCond
fallback.

The next worker audit must name the canonical consumer, its retained relation,
and the exact source-to-Recipe correspondence before implementation resumes.
