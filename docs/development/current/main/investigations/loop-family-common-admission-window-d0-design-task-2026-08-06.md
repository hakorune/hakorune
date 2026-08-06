---
Status: Design consultation required / implementation closed
Date: 2026-08-06
Decision: LOOP-FAMILY-COMMON-ADMISSION-WINDOW-D0
Authority: docs/development/current/main/design/loop-family-observation-policy-ssot.md
---

# Common Loop-family admission-window design task

## Purpose

Define the single cross-family boundary that may consume the already-landed
caller-zero observations for Generic G0, DirectAccum, NestedPredicate, and
LoopTrue. This is a design consultation only. No implementation, selector
caller, Recipe, Builder/MIR, physical lowering, retry/fallback, or legacy
retirement is authorized until the decision is worker-reviewed and recorded in
the design SSOT.

## Inputs that may be considered

Each family may contribute one sealed neutral disposition:

```text
Candidate
Declined
Unresolved
Rejected
```

The common boundary must consume the sealed source identity and provenance
already carried by each observation. It must not reread AST, re-resolve source,
inspect the legacy schedule/cursor/winner demand, or infer a route from a
family name. `Unresolved` and `Rejected` provenance must remain distinct.

## Questions to settle before implementation

1. Is the common product an overlap/admission window, a selector input, or two
   explicit products with one ownership boundary?
2. What exact precedence/overlap rule applies when more than one family is a
   `Candidate`?
3. Does any `Unresolved` block the whole request, or is there a sealed profile
   policy that decides this without retry/fallback?
4. Which disposition is publishable to the next Recipe stage, and what exact
   identity/provenance must it retain?
5. What is the smallest caller-zero witness and guard that proves the boundary
   without opening production selection?

## Stop lines

Do not implement or edit the legacy 19-route evaluator, `family_selection.rs`,
Recipe/JoinSig/BindingKey, Builder/MIR, physical route IDs, retry/fallback, or
production callers from this task. A worker-reviewed decision, a source-to-
neutral disposition matrix, and one bounded acceptance fixture are required
before the next implementation task is created.
