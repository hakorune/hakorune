---
Status: Accepted shallow implementation task / caller-zero witness
Date: 2026-08-06
Decision: GENERIC-SELECTION-OPEN-D0-I0-R0
Authority: docs/development/current/main/design/generic-loop-source-to-portable-recipe-ssot.md
---

# Generic selection open: candidate envelope witness

## Purpose

Open the existing `GENERIC-SELECTION-OPEN-D0` gate with one bounded,
`cfg(test)`-only resolver-to-candidate envelope. This is an evidence product,
not Generic production selection. It must make the source-to-selection
boundary concrete without adding another D4 suffix.

## Source authority

The issuer receives one `VerifiedResolvedFunctionV1`, one borrowed
`FunctionSyntaxViewV1`, and one move-only `GenericSourceLeaseV1`. The lease is
the sole authority for owner/origin/source-kind, exact loop sites, forest/frame
identity, and role-level `BindingRef` claims. The source view is borrowed only
while issuing typed facts; no AST or source lifetime may remain in the result.

The issuer may consume the existing carrier proof, V2 Condition/Step role
catalog, and V3 syntax facts. Body effect and Coverage/Exit are separate typed
proofs issued in this cell from the same source view and resolver inventory.
Policy mode/coverage, row normalization, selector choice, Recipe demand, and
physical identity remain owned by their existing modules.

## Product

`VerifiedGenericCandidateEnvelopeV1` is move-only and atomically owns:

```text
carrier/condition/step typed shape facts
body effect proof for the exact nested G0 fixture
complete loop-window and post-loop Return proof
the original resolver lease brand through the shape chain
```

The first natural fixture is the existing `generic_both(i, j)` source:

```text
outer Less loop
  inner Less loop
    j = j + 1
  i = i + 1
return j after the outer loop
```

The proof records exact sites and `BindingRef`s, not names or route IDs. It
rejects missing/foreign inventory, wrong loop/body order, extra statements,
non-binding assignment targets, unsupported exits, and a return that is not
outside the root loop. It does not issue a family row or a winner.

## Non-authority and forbidden work

This cell must not read legacy schedules/cursors, `family_selection.rs`, old
Generic facts, route IDs, `RecipeBody`, `LoopRecipeV1`, `JoinSig`, Builder,
MIR, `ValueId`, or production callers. It must not re-resolve names, rebuild
AST nodes, clone/split/re-pair leases, retry, fallback, or manufacture
`NoCandidate`. The existing five-row selector remains caller-zero.

## Acceptance

1. The natural fixture issues one envelope with all five proof families
   represented by typed, AST-free products.
2. The envelope retains exact owner/origin/forest/frame and all role
   `BindingRef`s after the source borrow ends.
3. A shadowing or malformed-body fixture rejects before envelope publication.
4. The consumed lease/envelope cannot be issued or consumed twice.
5. No selector, demand, Recipe, Builder/MIR, production, retry, or fallback
   caller is added; `NoCandidate` remains outside this window gate.
6. New source/test files stay below 800 lines and the shared guards are green.
7. The implementation commit updates the loop SSOT, reference matrix/module
   README, workstream/current mirrors, and states that the post-implementation
   reference documentation must be refreshed in the same commit.

## Finish line and next row

This task closes with a caller-zero envelope witness and focused tests. The
next row is a separate Generic policy/selector handoff review. Only after a
real `Selected(Generic)` exists may `VerifiedGenericRecipeDemandV1`, Recipe
keys, physical cutover, and legacy retirement be opened.
