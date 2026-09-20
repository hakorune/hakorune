---
Status: fast__2026-09-20__ParserLoopBreakSourcePackage
Task: MIR-CALL-PARSER-LOOPBREAK-SOURCE-PACKAGE-I0
Date: 2026-09-20
Parent: mir-call-parser-loopbreak-source-consumer-d0-2026-09-20.md
Implementation permission: true for source projection, LoopBreak Facts co-seal, and package-wide candidate/absence coverage only
---

# Parser LoopBreak source package admission I0

## Six-line brief

```text
Decision: extend the existing callable source-Facts owner so the parser's
  complete semantic package records one source-aligned LoopBreak disposition
  for every resolver batch row before Cataloged/Selected.
Source authority + canonical issuer: resolver-issued forest/exit ledger and
  the existing LoopBreakFacts/Recipe planner outcome; the compiler projector
  issues direct source sites and the existing package issuer co-seals coverage.
Non-authority: selected-key membership, Dynamic admission, AST/MIR rescans,
  LoopRouteContext, legacy composer, compatibility retry, or physical IDs.
Fail-fast boundary: missing/foreign/duplicate/out-of-root source relations,
  absent direct topology, brand drift, or incomplete batch coverage stop before
  package publication and before Builder allocation.
Smallest next slice: issue the direct LoopBreak source shape, add the typed
  per-row disposition, and attach the complete coverage product to the package.
Non-claims: no physical LoopBreak lowering, production caller switch, old-edge
  deletion, backend parity, VM/AOT work, or whole-package source-to-EXE proof.
```

## Authority and owners

1. The compiler projector starts from one resolver-issued root
   `SourceStmtSiteV1` and exact `ResolvedFunctionLoweringInputV1::source()`
   children. It reuses the existing route-neutral forest/exit transport
   `VerifiedLoopCondBreakContinueSourceForestProjectionV1`; it adds only the
   direct LoopBreak source shape and verifies its relation to
   `LoopBreakSourceTopologyV1`.
2. `CallableGenericLoopSourceFactsIssuerV1` remains the sole per-callable
   source/Facts co-seal owner. Its new branch consumes the existing planner
   `PlanBuildOutcome`, direct terminality proof, source shape, forest/exit
   transport, exact source items, and catalog brand without issuing a second
   Recipe or JoinSig.
3. `issue_normal_callable_semantic_package_with_brand_catalog_v1` remains the
   sole package observer. It enumerates every `VerifiedResolvedCallableSemantic`
   batch row, including unselected rows, and stores one package-private
   candidate/absence row keyed by batch slot, owner, source identity, and the
   same catalog brand.

## Disposition contract

The per-row disposition is explicit and non-optional:

| disposition | meaning | package action |
| --- | --- | --- |
| `Candidate` | direct LoopBreak topology, resolver forest/exit relation, and terminality all match | retain move-only source product for the later physical consumer |
| `SupportedNonCandidate` | row is resolved and valid, but has no supported direct LoopBreak topology | retain typed absence; do not retry or infer from names |
| `Unresolved` | resolver/source evidence is missing or cannot be opened | reject the package with the named source error |
| `Rejected` | foreign, duplicate, out-of-root, brand-mismatched, or shape-mismatched evidence | reject before Cataloged/Selected |

`Option<LoopBreakCandidate>` is forbidden because it merges the last three
cases. `DynamicCallableAdmissionV1` is only a vocabulary precedent; its type
and recipe are not reused for LoopBreak.

## Focused implementation tasks

| order | task | completion condition |
| --- | --- | --- |
| 1 | Add the direct source projector beside the existing LoopCond/LoopTrue projectors | exact loop/condition/break-if/carrier-update/step sites and explicit break exit remain paired with the resolver forest and frame identity |
| 2 | Extend the existing Facts issuer | Generic, LoopCond, and LoopTrue paths remain unchanged; LoopBreak candidate and all typed declines/rejects are named |
| 3 | Issue package-wide coverage | every batch row is observed once; duplicate/missing/foreign row and catalog-brand drift reject before package publication |
| 4 | Add focused guards | direct positive parser row plus missing forest, foreign owner, duplicate site, out-of-root, ScopeBox, specialized-topology, and incomplete-batch negatives are green |
| 5 | Closeout evidence | record source sizes, classified reds, and the next physical-consumer card; do not claim source-to-MIR success |

## Explicit non-work

Do not call `loop_break_composer`, `route_loop_break_recipe`,
`LoopRouteContext`, or `lower_loop_v0_core` in this I0. Do not alter the
generic `RouteNotFrontSelected` fallback arm, the legacy raw child port, the
shared LoopBreak registry route, or any VM/AOT lane. The candidate-only old
edge is retired only after a later physical-consumer acceptance proves the
package candidate reaches its named terminal.

## Acceptance boundary

This I0 is complete only when the existing parser package issuer exposes a
brand-matched, complete candidate/absence product for all batch rows and all
negative cases fail before Builder effects. It does not advance the parser
package to source-to-MIR acceptance; that remains the next physical-consumer
row with its own caller, terminal, and exclusive delete-set.

## Progress receipt (2026-09-20)

The direct source projector is now present at
`src/mir/compiler/loop_break_source_projection.rs`. It observes only the
resolver-owned loop site, condition/break/assignment sites, and the paired
explicit-break exit record. Direct three-statement shape, ScopeBox, explicit
else, body arity, and exit-target rejection are covered by three focused tests
(3/3). No Builder, route execution, package publication, fallback, or legacy
edge changed in this slice. Package-wide candidate/absence coverage and the
Facts co-seal remain open and are the next implementation steps.
