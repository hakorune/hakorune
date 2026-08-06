# Generic loop source -> portable Recipe SSOT

Status: `proposed design; consultation stop`

Current row: `GENERIC-SOURCE-TO-PORTABLE-RECIPE-D0`

This document fixes the complete Generic G0 path and its legacy retirement
boundary before implementation resumes. It is a design contract, not a
production or language-support claim. The next code row remains blocked until
the user accepts this consultation result.

## Decision

The first bounded Generic profile is one explicit nested two-loop function:

```hako
function f(i: i64, j: i64): i64 {
  loop(i < 3) {
    loop(j < 3) {
      j = j + 1
    }
    i = i + 1
  }
  return j
}
```

G0 accepts only exact `i64` parameters and an explicit `: i64` result. An
omitted result annotation is not inferred from `return j`; it is
`Unresolved(MissingReturnAnnotation)`.

Plain integer literals are contextually projected from the exact typed
`BindingRef` at their source site. Typed integer suffixes remain outside
Language v1. Tests must not rewrite `Integer` AST nodes into `TypedInteger` and
then claim natural-source evidence.

The portable body/control vocabulary is sufficient for G0, but the current
logical recurrence contract is not. Before the Generic Recipe producer is
implemented, the common portable layer must gain:

```text
1. nested carrier shadow
2. logical Header/After binding identity
3. source-bound verified core product
4. producer provenance independent of legacy route IDs
```

The post-loop `return j` stays outside `LoopRecipeV1`. `LoopExit::Return`
continues to mean a return occurring inside a loop.

The two existing Recipe names remain deliberately separate:

| Product | Status | Authority |
| --- | --- | --- |
| Builder `RecipeBody` / `RecipeBlock` | legacy Generic transport | AST-bearing composer/lowerer input; never portable evidence |
| `LoopRecipeV1` / verified common products | canonical target | AST-free recursive loop semantics |

G0 admits only exact `Less` conditions and positive exact-`i64` `Add` steps.
Calls, methods, new, print, fields, collections, captures, shadowing, if,
break/continue, extra loops/statements/tails, symbolic deltas/bounds, other
operators, foreign sites/frames, and incomplete coverage are outside G0. A
future profile must add an explicit fact/policy/Recipe row; schema vocabulary
alone never widens admission.

## One-way final pipeline

```text
VerifiedResolvedFunctionV1
+ resolver source-site inventory / loop forest / BindingRef map
  -> Generic G0 source projector
  -> VerifiedGenericStructuralFactsG0
  -> VerifiedGenericSourceTypeInventoryG0
  -> VerifiedGenericNumericRepresentationProjectionG0
  -> VerifiedGenericPolicyG0
  -> opaque family observation
  -> LoopRoutePolicyV1 sole canonical family selector
  -> SelectedGenericCandidateG0
  -> VerifiedGenericRecipeDemandG0
  -> Generic portable Recipe producer
  -> VerifiedLoopCoreProductV1
       - VerifiedLoopRecipeV1
       - VerifiedLoopJoinSigV1
       - verified source claim
       - verified BindingKey <-> BindingRef/effect relation
  -> VerifiedGenericAfterEffectG0
  -> VerifiedGenericRecipeProductG0
  -> Builder-free physical preflight
  -> common recursive Recipe physicalizer
       - CanonicalCfgSessionV1
       - function-owned Binding SSA / PhiTxn
  -> existing function-completion owner
  -> DraftSeal prepare
  -> infallible commit
  -> atomic module publication
```

There is no Generic-specific second MirBuilder, route-local PHI builder,
source reconstruction, ordered retry, or fallback.

## Authority map

| Decision | Sole owner |
| --- | --- |
| source owner/site/scope/forest/frame and `BindingRef` | `resolved_semantics` |
| bounded G0 grammar observation and exact coverage | Generic source projector / structural facts |
| parameter/result/literal source type inventory | resolver/source type bridge |
| exact numeric representation, range, sign, overflow | `numeric_substrate` |
| exact source result spelling expectation | existing `ExactTrivialReturnAbiV1` |
| `Less`, positive `Add`, progression admission | `loop_route_policy::generic` |
| exactly-one family winner | existing all-family `LoopRoutePolicyV1` selector owner |
| Recipe keys and source/effect relation | Generic Recipe producer |
| portable structure | `LoopRecipeVerifierV1` |
| logical carrier, edge, Header/After binding | `LoopJoinSigElaboratorV1` |
| `BindingRef -> ValueId/PHI` | function-owned Binding SSA |
| CFG/BasicBlock allocation | `CanonicalCfgSessionV1` |
| physical return/completion | existing function completion + DraftSeal |
| module visibility | atomic module transaction |

`FunctionSyntaxViewV1` remains body-only. `ExactTrivialReturnAbiV1` remains the
exact source-spelling expectation owner. `ReturnExitContract` remains the
ordinary executable return authority. Neither is duplicated inside Generic or
inferred from its tail.

## Source products

The structural witness is AST-free after issuance and proves exactly:

```text
function body       = [L0, Return(j)]
outer body          = [L1, update(i)]
inner body          = [update(j)]
L0 condition        = i < positive integer literal
L1 condition        = j < positive integer literal
outer step          = i = i + positive integer literal
inner step          = j = j + positive integer literal
tail                = one terminal return of exact BindingRef(j)
coverage            = every relevant source site exactly once
```

`GENERIC-G0-STRUCTURE-S0A` proves only this shape, ordering, identity, and
coverage. It makes no numeric ABI, candidate, selector, Recipe, or production
claim.

The resolver/source bridge then issues one move-only product:

```text
VerifiedGenericSourceTypeInventoryG0 {
  owner / origin / source-kind
  exact parameter declaration sites and BindingRefs
  exact return annotation site and source spelling
  exact literal OwnedExprSite rows
  literal role and contextual BindingRef
}
```

`numeric_substrate` consumes it exactly once with the target profile and
issues:

```text
VerifiedGenericNumericRepresentationProjectionG0 {
  exact parameter and literal representation/range rows with source provenance
  return_expectation: existing ExactTrivialReturnAbiV1
}

VerifiedGenericTypedNumericFactLeaseG0
```

The policy layer consumes only the typed lease. It never sees AST, invents a
candidate, or allocates Recipe keys.

## Exact portable G0 mapping

Recipe-local dense identities are deterministic:

```text
bindings:
  b0 = source BindingRef(i)
  b1 = source BindingRef(j)

loops:
  L0 = root
  L1 = child of L0

inputs:
  v0 = initial i
  v1 = initial j
```

Operation order is:

```text
L0 condition:
  ReadBinding(b0)                    -> v2
  ConstI64(c0)                       -> v3
  CompareI64(Less, v2, v3)          -> v4

L0 body:
  ReadBinding(b1)                    -> v5
  Loop(L1)                           // child entry uses v5

L1 condition:
  ReadBinding(b1)                    -> v6
  ConstI64(c1)                       -> v7
  CompareI64(Less, v6, v7)          -> v8

L1 body:
  ReadBinding(b1)                    -> v9
  ConstI64(d1)                       -> v10
  BinaryI64(Add, v9, v10)           -> v11
  WriteBinding(b1, v11)

L0 body, after L1:
  ReadBinding(b0)                    -> v12
  ConstI64(d0)                       -> v13
  BinaryI64(Add, v12, v13)          -> v14
  WriteBinding(b0, v14)
```

There are two source bindings but three recurrence carrier rows:

```text
C0 = (owner L0, binding b0, entry v0)
C1 = (owner L0, binding b1, entry v1)
C2 = (owner L1, binding b1, entry v5)
```

`v5` is derived Recipe glue, not a synthetic source statement or AST rewrite.
It transfers the current outer `j` into the child recurrence on every outer
iteration. Omitting C1 loses the child result at the outer loop; omitting C2
loses the child header PHI; using `v1` directly for C2 resets the child on each
outer iteration.

## Common portable closure required before G0 Recipe

### Nested carrier shadow

The current `visible_payloads` may publish both ancestor `j` and child `j`.
The corrected common law is:

```text
visible payload contains each binding exactly once
the innermost carrier shadows an ancestor carrier for that binding
after shadowing, rows are ordered by binding key
```

This is a common JoinSig rule, never a `GenericG0` name check.

### Logical port binding and After capability

Incoming edge payload values remain Recipe operation values. A header PHI or
After merge result is a different logical identity and must not be forged as
`v1` or `v11`.

The common JoinSig product therefore exposes a logical port-binding row
equivalent to:

```text
LoopJoinPortBindingV1 {
  loop_key
  port: Header | After
  binding
  class
}
```

and issues:

```text
VerifiedLoopAfterBindingV1 {
  root_loop: L0
  binding: b1
  class: I64
  logical source: L0.After/b1
}
```

The physicalizer maps that logical identity through the sole Binding SSA. The
Generic layer consumes the capability; it does not reinterpret JoinSig.

### Source-bound verified core

The common contract defines and verifies, before physical work:

```text
VerifiedLoopRecipeBindingRelationV1
  Recipe binding key <-> exact source BindingRef/value class/declaration role

VerifiedLoopBindingEffectRelationV1
  Recipe item/derived role <-> Recipe binding/source BindingRef/source site

VerifiedLoopCoreProductV1
  verified Recipe + JoinSig + opaque source claim + both relations
```

`DerivedCarrierEntry` is an explicit effect role for `v5`. Labels and source
names are diagnostic only; they are never mapping authority. The common S0 row
adds only schema/verifier vocabulary and caller-zero synthetic validation. The
real G0 keys and relation instances are issued exactly once by the Generic
producer in S4.

### Producer provenance

Portable provenance is separated from the legacy scheduler's `LoopRouteId`.
The canonical producer identity is a diagnostic/product enum such as:

```text
LoopRecipeProducerIdV1::GenericG0
```

It is not a family-selection authority. A canonical G0 product must never
claim `GenericLoopV0` or `GenericLoopV1` provenance.

## Function tail

The Generic wrapper adds:

```text
VerifiedGenericAfterEffectG0 {
  exact terminal Return source site
  exact source BindingRef(j)
  recipe binding b1
  VerifiedLoopAfterBindingV1(L0.After, b1)
  existing ExactTrivialReturnAbiV1 source expectation
  existing executable return/completion contract
  no-trailing-source coverage receipt
}
```

The physicalizer reads the `L0.After/b1` Binding SSA value and hands it to the
existing explicit-return completion path. DraftSeal remains the only physical
Return writer. `LoopRecipeV1` is not widened with a function tail.

## Outcome algebra

```text
Ready
  every required source, type, range, policy, coverage, and provenance fact is sealed

Unresolved
  required information is absent or opaque: missing/unknown type annotation,
  missing approved literal context, unknown target, or unavailable capability

Rejected
  known contradiction: foreign identity/site, non-i64 result, mismatch,
  out-of-range/non-positive literal, malformed shape, duplicate winner,
  unavailable After binding, or uncovered source/effect

NoCandidate
  whole-unit selector result only when coverage is complete and every family
  explicitly Declined; never a partial Generic observation or retry signal
```

S2 adds one caller-zero Generic observation row to the existing sole
all-family policy selector; it does not create a Generic selector or a second
winner authority. Selection is semantic-only after S2. Production selection
remains zero until the atomic M10b cutover. There is no order-priority winner
or different-family fallback.

## Legacy authority and retirement

The live old path is:

```text
route_loop
  -> old Generic facts/extractors
  -> ordered route preflight/selection
  -> Generic V0/V1 registry handler
  -> AST-bearing RecipeBody composer/skeleton
  -> Builder allocation/mutation
  -> verifier/lower failure converted to post-effect retry debt
  -> possible second route on an already-mutated Builder
```

The highest-risk authority is not merely the parser or an enum. It is the
combination of early Builder mutation plus retry/error-to-`None`. Cutover must
remove those edges atomically.

The retirement ledger is grouped by responsibility, not by filename count:

| Cohort | Representative current paths | Disposition |
| --- | --- | --- |
| old AST/facts authority | `plan/generic_loop/**`, `facts/canon/generic_loop/**`, `generic_loop_canon/**`, `mir/policies/generic_loop_*` | replace with resolver/structural/type/policy products; delete after parity |
| ordered route authority | `joinir/route_entry/registry/**`, Generic V0/V1 route rows | disconnect in M10b; delete Generic-only rows in R1 |
| mutating composer/lowerer | `recipe_tree/generic_loop_composer.rs`, `skeletons/generic_loop.rs`, `features/generic_loop_*`, `features/generic_loop_body/**` | remove production callers in M10b; delete dead files in R1 |
| retry/fallback | `execution_witness.rs`, `legacy_receipt.rs`, nested handoff/adoption helpers | delete selected Generic debt, `.ok()`, continuation, and retry edges in M10b |
| recent test-only source evidence | `resolved_semantics/generic_resolved_carrier_*`, `loop_structural_facts/generic_resolved_carrier_*` | migrate useful fixtures into G0 products, then retire the superseded named witnesses |
| shared infrastructure | `RecipeBody`/`RecipeBlock`, non-Generic route policy and physicalizers | retain through Generic R1; reconsider only in M11/M12 |

Before cutover, every legacy Generic fixture is classified in a checked
manifest. Every fixture accepted by current production must be exactly one of:

```text
implemented by G0
implemented by another portable producer
explicitly rejected by an accepted language/profile Decision
```

“Retained for a named future portable profile” is allowed only for evidence
that current production does not accept. It cannot justify removing support
from an already accepted source.

G0 must not silently retire the legacy route's broader calls, print, if,
locals, exits, or effect surfaces.

M10b switches one named production caller to the verified portable product and
in the same commit removes every selected old mutating authority:

```text
Generic V0/V1 registry handlers and predicates
Generic post-effect retry debt / error-to-None conversion
legacy Generic receipt and continuation edges
Generic composer/skeleton/pipeline/body lowerer callers
nested Generic `.ok()`/retry edges
old Generic selector rows used by that caller
```

After caller-zero proof, `GENERIC-LEGACY-DEAD-CODE-R1` physically removes the
dead Generic-only facts, extractors, composers, adapters, old-authority-only
assertions, and files. Portable parity fixtures, counterexamples, and the
retirement manifest remain as canonical evidence.
The shared `RecipeBody`/`RecipeBlock`, non-Generic routes, and common 19-route
policy remain until their own M11/M12 closeout.

The existing shared MirBuilder replacement manifest/guard is extended for the
caller-zero proof; no new per-row guard script is created.

## Finite shallow task order

```text
GENERIC-SOURCE-TO-PORTABLE-RECIPE-D0
  this worker-reviewed consultation proposal; implementation remains unauthorized

GENERIC-G0-STRUCTURE-S0A
  natural-source structural/coverage witness only; selection_open=false

GENERIC-G0-SOURCE-TYPE-S0B
  owner-branded parameter/result/literal/context inventory; no target policy

GENERIC-G0-NUMERIC-REPRESENTATION-S0C
  numeric_substrate seals representation/range plus existing return expectation

LOOP-JOINSIG-NESTED-SHADOW-S0
  one visible payload per binding; innermost recurrence carrier wins

LOOP-JOINSIG-AFTER-BINDING-S0
  logical Header/After binding identity and VerifiedLoopAfterBinding

LOOP-RECIPE-SOURCE-BOUND-CORE-S0
  common core/relation schema and verifier only; no Generic key instance

LOOP-RECIPE-PRODUCER-ID-S0
  portable producer provenance separated from legacy LoopRouteId

GENERIC-G0-CANDIDATE-S1
  opaque move-only candidate; no selector caller

GENERIC-G0-SELECTOR-S2
  Generic row in the sole all-family selector; no second winner owner

GENERIC-G0-DEMAND-S3
  role/site/BindingRef demand only; no Recipe key issuance

GENERIC-G0-RECIPE-S4
  sole key issuer; exact Recipe/JoinSig/core/effect/tail product

GENERIC-LEGACY-CORPUS-DISPOSITION-P0
  classify every old accepted fixture before narrowing or deletion

GENERIC-G0-PHYSICAL-PREFLIGHT-P0
  mutation-free physical input/completion/topology capability

LOOP-RECIPE-RECURSIVE-PHYSICALIZER-P0
  disconnected common CFG/Binding-SSA physicalizer and tail-handoff parity

M8-ALL19
  close all portable producers required by the shared pipeline

M9-SELFHOST-PARITY
  normalized portable Recipe/product parity; no production cutover

M10b-I0-R0
  one atomic production switch plus old mutating Generic/scheduler edge deletion

GENERIC-LEGACY-DEAD-CODE-R1
  caller-zero old authority removal; retain parity/counterexample evidence

M11-R1
  located/source-erasing handoff retirement

M12-R2
  shared legacy family adapter/route provenance closeout
```

No deeper `D4-S4-...` suffixes are added. Each row has one owner and one
acceptance claim. Source/guard files stay below 800 lines; workstream and
investigation documents stay at or below 1000 lines.

## Required evidence

The positive minimum includes exact normalized G0 Recipe, C1/C2 same-binding
recurrence, duplicate-free child payload, child header PHI, root backedge
carrying post-child `j`, `VerifiedLoopAfterBinding(b1)`, and a tail that uses
the After value rather than an input/body temporary.

Negative evidence includes missing root/child carriers, missing/early child
entry, duplicate payload, foreign owner/frame/BindingRef, shadowed same-name
different binding, missing/nonterminal/wrong tail, unavailable After binding,
wrong class, wrong body order, non-positive/range-invalid literals, incomplete
coverage, and legacy V0/V1 provenance on a G0 product.

Each implementation commit updates `CURRENT_STATE.toml`, this SSOT, the active
workstream, and its owning README. At the corresponding production activation
and retirement row it also updates the exact `docs/reference/**` documents in
the same commit. The final reference closeout audits grammar, diagnostics,
default behavior, backend parity, and legacy caller zero.

## Stop lines

```text
AST rewrite or typed-literal test forgery                    = 0
name-based BindingRef/type reconstruction                    = 0
Generic key issuance before S4                              = 0
physical ValueId/PHI outside function-owned Binding SSA     = 0
Generic-specific recurrence/After special case              = 0
function tail inside LoopRecipeV1                            = 0
legacy Generic V0/V1 provenance on canonical G0             = 0
Builder mutation before complete verified physical input    = 0
retry/fallback after selection or mutation                  = 0
production caller before M10b                               = 0
silent legacy feature retirement                            = 0
public reference support claim before activation            = 0
```
