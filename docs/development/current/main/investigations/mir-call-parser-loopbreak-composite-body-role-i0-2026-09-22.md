---
Status: fast__2026-09-22__CompositeLoopBreakBodyRoleProjection
Task: MIR-CALL-PARSER-LOOPBREAK-COMPOSITE-BODY-ROLE-I0
Date: 2026-09-22
Parent: mir-call-parser-loopbreak-composite-source-reopen-d1-2026-09-22.md
Implementation permission: true for the same-owner source body-role product only
NextCard: MIR-CALL-PARSER-LOOPBREAK-COMPOSITE-SOURCE-PACKAGE-I1
---

# Parser composite LoopBreak body-role projection I0

## Six-line brief

```text
Decision: issue one resolver-branded, AST-free body-role tree for the
  composite LoopBreak root before package or Recipe admission.
Source authority + canonical issuer: the existing
  issue_loop_break_composite_source_projection_v1 input, its resolver forest,
  source view, region index, and exit ledger.
Non-authority: names/line numbers, flattened AST rescans, LoopRouteContext,
  generic route retry, planner guesses, Recipe keys, JoinSig keys, and MIR.
Fail-fast boundary: foreign/duplicate sites, skipped body members, missing
  branch regions, unbound exits, child-frame mismatch, or uncovered selected
  source-call items produce typed rejection.
Smallest next slice: add the role tree and positive/negative projection tests;
  package retention and Recipe construction remain separate successors.
Non-claims: no package candidate, source-to-MIR publication, physical lowering,
  production caller switch, fallback, or old-edge deletion.
```

## Accepted source-to-role contract

The same-owner composite projection owns the root site, loop condition/body
sites, forest preorder, and all resolver exits. I0 extends that product with a
recursive role tree. It retains only source sites, resolver region/transfer
records, forest member indices, and frame keys; it issues no Recipe or physical
identity.

The finite role vocabulary is:

| role | required evidence | downstream meaning |
| --- | --- | --- |
| `Statement` | exact statement site | one existing `RecipeItem::Stmt` |
| `If` | exact condition site, `if_region_bundle`, then/else role trees, and an exit contract | one existing `RecipeItem::IfV2` when the tree contains a branch exit or nested loop |
| `Loop` | exact loop site, forest member index/parent, condition site, body tree, frame key | one existing `RecipeItem::LoopV0` |
| `Exit` | exact statement site plus resolver `ResolvedExitRecordV1` | one existing `RecipeItem::Exit` |

An `If` with no nested control role may remain an opaque `Statement`; an `If`
with an exit or nested loop must be explicit. Unsupported `ScopeBox`,
`BuildGate`, `TaskScope`, `ContextScope`, `FastMemRegion`, and `TryCatch`
shapes are typed absence for this I0. This keeps the accepted shape finite and
prevents the Recipe producer from rediscovering control meaning.

The exit-to-branch relation is resolver-owned: use each exit's sealed
`source_region`, walk `VerifiedResolvedFunctionV1.region().parent()`, and
match the first `IfThen`/`IfElse` region issued by `if_region_bundle`. Reject a
foreign owner, an exit outside the root forest, an ambiguous branch, or a
transfer whose target loop is not the enclosing forest member.

Selected method-call items remain a separate exact source-item inventory. I0
must prove that each selected call site is contained by exactly one role-tree
subtree and that no role subtree drops a selected item. The target/publication
probe is transported unchanged; target-to-Recipe mapping belongs to I1.

## Focused acceptance matrix

| case | expected result |
| --- | --- |
| parser root `:81` with resolver children `:131` and `:182` | one same-owner role tree with forest preorder and child frames retained |
| nested `If` with explicit break/continue | explicit `If` and `Exit` roles with resolver branch relation |
| nested child loop | `Loop` role keyed by the existing forest member/frame, no route inference |
| foreign root or foreign forest | typed `ForeignOwner` rejection |
| duplicate/missing body role or dropped child | typed coverage rejection |
| exit outside root, wrong target, or unresolved branch | typed exit-relation rejection |
| selected source call outside or twice inside the role tree | typed source-item coverage rejection |

I0 closes only when the positive parser projection and every negative row are
green. The package loan, Recipe producer, physical adapter, and
`GenericLoopV1NotSelected` cutover remain later rows in D1.

## Implementation checkpoint — 2026-09-22

The source product is implemented in
`src/mir/compiler/loop_break_composite_body_role.rs` and is co-sealed by the
existing composite source projection. The focused projection matrix is 3/3:
the synthetic nested root, the foreign-root rejection, and the merged parser
projection all pass. `cargo check --profile quick --lib` also passes with the
I147 warning baseline of 1,672 library warnings.

This does not close I0 yet. The current tests observe the positive nested role
tree and the upstream foreign-owner stop; direct guards for duplicate/missing
body coverage, unsupported shapes, and exit relation failures still need to be
added before the negative rows above can be claimed.
