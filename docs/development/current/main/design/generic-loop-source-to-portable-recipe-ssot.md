# Generic loop source -> portable Recipe SSOT

Status: `accepted and taskized; production activation remains 0`

Current row: `GENERIC-G0-SOURCE-TYPE-S0B`

This document fixes the complete Generic G0 path and its legacy retirement
boundary before implementation resumes. It is a design contract, not a
production or language-support claim. The consultation result and the closed
G0 admission window were accepted on 2026-08-06. Implementation may
resume only at the first row in the finite order below.

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
  -> loop_route_policy::generic_g0 policy observation
  -> complete five-row VerifiedGenericAdmissionWindowV1
  -> CanonicalLoopFamilySelectionV1 sole semantic family selector
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
| `Less`, positive `Add`, progression admission | `loop_route_policy::generic_g0` |
| exactly-one G0 admission winner | `loop_route_policy::family_selection`; test-only marker is promoted at S2 |
| Recipe keys and source/effect relation | Generic Recipe producer |
| portable structure | `LoopRecipeVerifierV1` |
| logical carrier, edge, Header/After binding | `LoopJoinSigElaboratorV1` |
| `BindingRef -> ValueId/PHI` | function-owned Binding SSA |
| CFG/BasicBlock allocation | `CanonicalCfgSessionV1` |
| physical return/completion | existing function completion + DraftSeal |
| module visibility | atomic module transaction |

The legacy 19-route evaluator in `loop_route_policy::policy` is migration
evidence, not this selector. At D0 there is no production canonical family
selector: `family_selection.rs` is test-only and cannot issue a real Selected
capability. S2 promotes that one boundary; it does not add a Generic-only
selector or teach the legacy raw-cursor evaluator a second selection policy.

## Closed G0 admission window

`GENERIC-G0-ADMISSION-WINDOW-D0` is closed by this Decision. Selection of G0
must compare it against exactly these four potentially competing migration
profiles; the five rows are the complete G0 admission window:

| Family row | Source-backed issuer | Current disposition path |
| --- | --- | --- |
| DirectAccum | `family_selection::direct_accum` from `VerifiedDirectAccumSingletonObservationV1` | S1 adapts the existing structural observation; legacy schedule receipt stays outside the set |
| NestedPredicate | `family_selection::nested_predicate` from `VerifiedNestedLoopSourceProjectionV1` | S1 adapts the exact bounded nested source projection |
| LoopTrueBreakContinue | `family_selection::loop_true` from `VerifiedLoopTrueBreakContinueSourceProjectionV1` | S1 adapts the exact explicit-branch source projection |
| LoopCondBreakContinue | `family_selection::loop_cond` source/policy observer | S1 adds the missing semantic observation before selection; `JOINIR-LOOP-M8-LOOPCOND-EXITS-S6D` later adds its Recipe cohort |
| Generic | `loop_route_policy::generic_g0` from the sealed G0 typed lease | Ready candidate or a typed Declined/Unresolved/Rejected disposition |

Each source unit in this window yields exactly one typed disposition per row:
`Candidate`, `Declined`, `Unresolved`, or `Rejected`. The observation-set
assembler owns completeness, owner/frame/mode equality, and duplicate-row
rejection. The selector alone owns Generic admission. It may issue
`Selected(Generic)` only for one exact Generic candidate plus four exact
Declined competitors. An overlap rejects; missing/unresolved rows remain
Unresolved.

These rows are not five semantic Loop kinds and not five Recipe variants. They
are migration/profile observations projected into one G0 overlap check; the
portable meaning remains the single recursive Recipe algebra. Five Declined
rows do not prove whole-unit `NoCandidate`. That constructor remains sealed
until `JOINIR-LOOP-M8-ALL19-CLOSEOUT-S6G` closes the complete all-route source-policy universe and its
whole-unit coverage proof.

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
L0 condition        = i < integer-literal expression site
L1 condition        = j < integer-literal expression site
outer step          = i = i + integer-literal expression site
inner step          = j = j + integer-literal expression site
tail                = one terminal return of exact BindingRef(j)
coverage            = every relevant source site exactly once
```

`GENERIC-G0-STRUCTURE-S0A` proves only this shape, ordering, identity, and
coverage. It makes no numeric ABI, candidate, selector, Recipe, or production
claim. S0A proves the literal sites and roles only; it does not prove their
sign, value, range, or positivity. S0C seals numeric facts and S1 owns the
positive-Add/Less admission decision.

S0A implementation receipt (2026-08-06): the natural-source projector at
`src/mir/compiler/generic_g0_projection/mod.rs` now issues one AST-free
observation into the sole structural issuer at
`src/mir/loop_structural_facts/generic_g0/mod.rs`. The sealed product is
move-only, checks exact body schedules, resolver-issued `BindingRefV1`
relations, owner/source/frame identity, and duplicate-free coverage. Focused
tests cover the canonical shape, AST immutability, reordered/extra/missing
schedule, wrong-binding, and foreign-frame rejects. The implementation has no
Recipe, policy, selector, Builder, MIR, retry, fallback, or production caller;
the shared caller-zero guard explicitly owns this projector boundary. S0B is
the next authorized row.

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

The capability chain is move-only and cumulative:

```text
S0A structure lease
  -> S0B bundle retains structure + adds source-type inventory
  -> S0C bundle retains S0B + adds numeric projection/lease
  -> S1 Generic observation owns the complete S0C bundle
  -> S2 Selected(Generic) moves that bundle unchanged
  -> S3 demand consumes selection and retains every site/role/BindingRef lease
  -> S4 consumes the demand once
```

No stage clones, reconstructs, or reissues an earlier lease. In particular,
S4 never recovers source/type/numeric authority from names, Recipe keys, or
test fixtures.

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
  unavailable in the G0 admission window; only the M8 all19 closeout may open it after complete
  all-route source-policy coverage and explicit Declined evidence
```

S1 seals the complete five-row G0 admission window. S2 removes the test-only
boundary from `family_selection`, replaces the marker `SelectedFamilyV1` with
a move-only selected Generic capability, and makes this the sole canonical G0
admission selector. It does not reuse the legacy 19-route evaluator. Selection
is semantic-only after S2: its production caller remains zero until the atomic
M10b cutover. There is no `NoCandidate`, order-priority winner, or different-
family fallback in this window.

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
| direct nested mutation bypasses | `nested_loop_depth1_route`, Generic nested adoption, located composer | disconnect every direct extract/normalize/mutate edge in M10b; delete Generic-only residue in R1 |
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
`nested_loop_depth1_route` and located/adoption direct mutation bypasses
old Generic selector rows used by that caller
```

M10b disconnects only direct mutation/reselection reached through the located
or nested adoption paths. It preserves the shared located source/provenance
handoff needed by M11. M11 later feeds that transport into the portable source
path and retires the source-erasing handoff/R4 fence; these are not the same
edge cohort.

After caller-zero proof, `GENERIC-LEGACY-DEAD-CODE-R1` physically removes the
dead Generic-only facts, extractors, composers, adapters, old-authority-only
assertions, and files. Portable parity fixtures, counterexamples, and the
retirement manifest remain as canonical evidence.
The shared `RecipeBody`/`RecipeBlock`, non-Generic routes, and common 19-route
policy remain until their own M11/M12 closeout.

The separate `src/mir/join_ir/lowering/**` authority and
`NYASH_JOINIR_LOWER_GENERIC` are outside Generic R1. They require caller-zero
evidence in M11/M12 or a named JoinModule retirement row; that row also updates
`docs/reference/environment-variables.md`.

The existing shared MirBuilder replacement manifest/guard is extended for the
caller-zero proof; no new per-row guard script is created.

## Finite shallow task order

```text
GENERIC-SOURCE-TO-PORTABLE-RECIPE-D0
  accepted; complete mapping, family universe, owners, cutover, and deletion boundary

GENERIC-G0-ADMISSION-WINDOW-D0
  closed here; five overlap rows, exact issuer/disposition contract, no NoCandidate

GENERIC-G0-STRUCTURE-S0A
  closed; natural-source structural/coverage witness landed; selection_open=false

GENERIC-G0-SOURCE-TYPE-S0B
  next; owner-branded parameter/result/literal/context inventory; no target policy

GENERIC-G0-NUMERIC-REPRESENTATION-S0C
  numeric_substrate seals representation/range plus existing return expectation

LOOP-JOINSIG-MODULE-SPLIT-R0
  behavior-neutral Refactor Series before adding nested logical authority

LOOP-RECIPE-PRODUCER-ID-S0
  portable producer provenance separated from legacy LoopRouteId

LOOP-JOINSIG-NESTED-SHADOW-S0
  one visible payload per binding; innermost recurrence carrier wins

LOOP-JOINSIG-AFTER-BINDING-S0
  logical Header/After binding identity and VerifiedLoopAfterBinding

LOOP-RECIPE-SOURCE-BOUND-CORE-S0
  common core/relation schema and verifier only; no Generic key instance

GENERIC-G0-CANDIDATE-S1
  Generic policy owns Less/positive-Add admission and one opaque observation

LOOP-FAMILY-DIRECT-OBSERVATION-S1
LOOP-FAMILY-NESTED-OBSERVATION-S1
LOOP-FAMILY-LOOPTRUE-OBSERVATION-S1
LOOP-FAMILY-LOOPCOND-OBSERVATION-S1
  four non-Generic semantic rows; typed disposition only, no Recipe or winner

GENERIC-G0-ADMISSION-SET-S1
  complete move-only five-row window; no winner authority

GENERIC-G0-SELECTOR-S2
  promote test-only family_selection into the sole canonical G0 admission selector

GENERIC-G0-DEMAND-S3
  role/site/BindingRef demand only; no Recipe key issuance

GENERIC-G0-RECIPE-S4
  sole key issuer; exact Recipe/JoinSig/core/effect/tail product

GENERIC-LEGACY-CORPUS-UNIVERSE-P0
  normalize active fast/selfhost/smoke/fixture cohorts without guessing the route

GENERIC-LEGACY-OBSERVATION-FRONT-G0
  prove the selected observation front reaches Loop before running the full census

GENERIC-LEGACY-ROUTE-OBSERVATION-P1
  record actual route/bypass/RC/output serially from the green front receipt

GENERIC-LEGACY-DISPOSITION-D0
  classify every old accepted fixture before narrowing or deletion

GENERIC-LEGACY-CROSS-FAMILY-DEPENDENCY-S0
  separate Generic-only deletion from shared/non-Generic canonicalizer callers

GENERIC-G0-PHYSICAL-PREFLIGHT-P0
  mutation-free common physical input plus Generic After envelope

LOOP-PHYSICALIZER-COMMON-OWNER-R0
  behavior-neutral 2-5 commit split; accepted shapes unchanged

LOOP-RECIPE-RECURSIVE-PHYSICALIZER-P0
  disconnected common CFG/Binding-SSA physicalizer through root After capability

GENERIC-G0-COMPLETION-P0
  hand root After value to existing completion/DraftSeal; no second Return writer

JOINIR-LOOP-M8-LOOPV0-RECURRENCE-S6A
JOINIR-LOOP-M8-LOOPV0-EXITS-JOINS-S6B
JOINIR-LOOP-M8-LOOPV0-SCANS-S6C
JOINIR-LOOP-M8-LOOPCOND-EXITS-S6D
JOINIR-LOOP-M8-GENERIC-RESIDUAL-S6E
JOINIR-LOOP-M8-ALL19-CLOSEOUT-S6G
  one portable producer+observation cohort per row, then same-selector all19 closeout

SELFHOST-LOOP-PORTABLE-WIRE-S7A
SELFHOST-LOOP-M8A-RECURRENCE-PARITY-S7B1
SELFHOST-LOOP-M8B-EXITS-JOINS-PARITY-S7B2
SELFHOST-LOOP-M8C-SCANS-PARITY-S7B3
SELFHOST-LOOP-M8D-LOOPCOND-PARITY-S7B4
SELFHOST-LOOP-M8E-GENERIC-PARITY-S7B5
SELFHOST-LOOP-PORTABLE-ALL19-PARITY-S7G
  .hako wire, one producer cohort per row, normalized all19 parity closeout

GENERIC-M10B-DELETION-MANIFEST-S0
  freeze exact selected old symbols/callers immediately before cutover

M10b-I0-R0
  one atomic production switch plus old mutating Generic/scheduler edge deletion

GENERIC-LEGACY-DEAD-CODE-R1
  caller-zero old authority removal; retain parity/counterexample evidence

M11-R1
  located/source-erasing handoff retirement

JOINIR-LOOP-LEGACY-DISPOSITION-R2A
JOINIR-LOOP-DUPLICATE-FACADE-RETIRE-R2B
JOINIR-LOOP-MUTATION-DISPATCH-RETIRE-R2C
JOINIR-LOOP-SOLE-AUTHORITY-CLOSEOUT-R2G
  exact disposition, duplicate facades, mutation dispatch, sole-authority closeout
```

No deeper `D4-S4-...` suffixes are added. Each row has one owner and one
acceptance claim. Source/guard files stay below 800 lines; workstream and
investigation documents stay at or below 1000 lines.

## Executable row contracts

Every ordinary row is one implementation-coupled commit. The two named R0/R2
Refactor Series may use 2-5 buildable commits but may not mix BoxShape with a
new accepted shape. A failed fast gate is stashed rather than committed.

### Source and portable logical foundation

| Row | Sole input -> output | Done | Stop / non-claim |
| --- | --- | --- | --- |
| `GENERIC-G0-STRUCTURE-S0A` | natural `ResolvedFunctionLoweringInputV1` -> move-only `VerifiedGenericStructuralFactsG0` | landed: exact positive plus reordered/extra/missing/wrong-binding/foreign-frame negatives; full source coverage; AST mutation zero; focused test and shared guard green | no type, numeric, policy, candidate, selector, Recipe, or production claim |
| `GENERIC-G0-SOURCE-TYPE-S0B` | S0A + callable source/header views -> inventory wrapped with S0A as `VerifiedGenericSourceBundleG0` | exact parameter/result/literal/context sites; missing/non-i64/foreign/coverage rejects | no literal representation, progression, or executable-return authority |
| `GENERIC-G0-NUMERIC-REPRESENTATION-S0C` | S0B bundle + exact target -> `VerifiedGenericTypedSourceBundleG0` with representation projection + one typed lease | natural unsuffixed literal positives; missing context is Unresolved; range/type contradiction is Rejected | do not duplicate numeric substrate or retag test AST |
| `LOOP-JOINSIG-MODULE-SPLIT-R0` | current `join_sig.rs` -> thin facade + model/visibility/port/flow modules | existing Recipe/JoinSig goldens byte-for-byte stable; all commits build; no acceptance delta | no nested-shadow or After feature in this series |
| `LOOP-RECIPE-PRODUCER-ID-S0` | current portable producers -> `LoopRecipeProducerIdV1` | portable schema imports `LoopRouteId` zero; current producers and normalized fixtures migrate; route parity moves to an external migration receipt | no selector or registry retirement |
| `LOOP-JOINSIG-NESTED-SHADOW-S0` | verified carriers + ancestry -> one visible payload per binding | C1/C2 same-binding, ancestor duplicate, sibling and foreign negatives; innermost wins in binding-key order | no PHI, After, or Generic special case |
| `LOOP-JOINSIG-AFTER-BINDING-S0` | verified edges/carriers -> `LoopJoinPortBindingV1` + `VerifiedLoopAfterBindingV1` | Header/After and owner/class/availability boundaries are exact | no physical ValueId/PHI or function Return |
| `LOOP-RECIPE-SOURCE-BOUND-CORE-S0` | Recipe + JoinSig + opaque source claim + relations -> `VerifiedLoopCoreProductV1` | caller-zero positive plus foreign/duplicate/uncovered/derived-role negatives | no real Generic keys, Builder, or physical IDs |

### Policy, selection, and G0 production

| Row | Sole input -> output | Done | Stop / non-claim |
| --- | --- | --- | --- |
| `GENERIC-G0-CANDIDATE-S1` | S0C typed lease + mode/profile/coverage -> `VerifiedGenericFamilyObservationG0` | exact Less/positive Add is Candidate; unsupported/symbolic is Unresolved; contradictory role/direction is Rejected | Generic policy may not select a winner or issue Recipe keys |
| `LOOP-FAMILY-DIRECT-OBSERVATION-S1` | DirectAccum structural observation + shared owner/window -> typed DirectAccum disposition | exact candidate/decline/unresolved/reject and foreign/mode boundaries fixed | no legacy schedule receipt, winner, or Recipe |
| `LOOP-FAMILY-NESTED-OBSERVATION-S1` | exact nested source projection + shared owner/window -> typed NestedPredicate disposition | G0 and non-nested shapes decline; exact bounded nested shape is a candidate | no physical Nested plan or route ID |
| `LOOP-FAMILY-LOOPTRUE-OBSERVATION-S1` | exact LoopTrue source projection + shared owner/window -> typed LoopTrue disposition | explicit branch candidate and shape/coverage declines/rejects fixed | no frozen-schedule admission or Recipe demand |
| `LOOP-FAMILY-LOOPCOND-OBSERVATION-S1` | shared source window + exact conditional-exit observer -> typed LoopCond disposition | missing issuer is eliminated; exact candidate/decline/unresolved/reject matrix fixed before S2 | no `JOINIR-LOOP-M8-LOOPCOND-EXITS-S6D` Recipe producer, physicalizer, or route selection |
| `GENERIC-G0-ADMISSION-SET-S1` | five owner-branded dispositions -> non-Clone `VerifiedGenericAdmissionWindowV1` | complete/missing/duplicate/foreign/mode cases fixed | no whole-unit NoCandidate, route order, cursor, winner, Recipe, or fallback |
| `GENERIC-G0-SELECTOR-S2` | complete admission window -> Selected(Generic) / Unresolved / Rejected | one Generic plus four Declined selects its move-only candidate; overlap rejects; five Declined is OutOfDomain/Unresolved; legacy schedule imports zero | semantic caller remains zero; no whole-unit NoCandidate, demand, Recipe, or production switch |
| `GENERIC-G0-DEMAND-S3` | `Selected(Generic(candidate))` -> `VerifiedGenericRecipeDemandG0` | selected capability consumed once; role/site/BindingRef lease exact; by-name repair zero | no Recipe key, ValueId, or PHI |
| `GENERIC-G0-RECIPE-S4` | S3 + common S0 services -> exact Recipe/relations/core/After/product | deterministic golden, three carrier rows, child-entry read, C1/C2, tail relation, and legacy provenance rejects | no physical MIR or production caller |

### Physical proof without production

| Row | Sole input -> output | Done | Stop / non-claim |
| --- | --- | --- | --- |
| `GENERIC-LEGACY-CORPUS-UNIVERSE-P0` | active phase29bq, selfhost subset, four Generic smokes, and Generic-named fixtures -> one normalized case universe | case/mode keys unique; compatibility stems alias a canonical case; names never imply a route | no runtime selection claim and no deletion |
| `GENERIC-LEGACY-OBSERVATION-FRONT-G0` | one case, then a small sample through the exact selected front -> green Loop-reached receipt | if failure occurs before Loop, name its real owner and open a separate repair row there | do not patch Generic, widen support, or declare census data from a failed front |
| `GENERIC-LEGACY-ROUTE-OBSERVATION-P1` | green front receipt + normalized universe -> serial route/bypass/RC/output observations | every required release/strict/planner-required run is observed; timeout or pre-Loop failure stays unclassified | no parallel full census and no manufactured result |
| `GENERIC-LEGACY-DISPOSITION-D0` | observed normalized universe -> checked disposition | accepted cases use only portable owner or accepted typed reject; nonaccepted cases may retain future evidence; unclassified accepted count zero | future evidence cannot retire currently accepted input; failed/unobserved cases block closeout |
| `GENERIC-LEGACY-CROSS-FAMILY-DEPENDENCY-S0` | Generic candidate files + repository caller graph -> Generic-only / neutralize-first / M11 / M12 or named JoinModule ownership | shared `UpdateCanon`, `RecipeBody/RecipeBlock`, located handoff, separate `join_ir/lowering`, and `NYASH_JOINIR_LOWER_GENERIC` are assigned outside R1 | do not wholesale-delete `generic_loop_canon/**`, a name-matched subtree, or its environment reference |
| `GENERIC-G0-PHYSICAL-PREFLIGHT-P0` | S4 + target + existing completion capability -> one-shot common physical input + Generic After envelope | mismatched Recipe/JoinSig/source/tail cannot be paired; Builder effects zero | no mutation or publication |
| `LOOP-PHYSICALIZER-COMMON-OWNER-R0` | current 792-line Accum owner -> common small services + thin Accum adapter | existing Accum MIR/fault/reuse/caller counts unchanged; each commit buildable | do not add recursive Generic acceptance in the refactor series |
| `LOOP-RECIPE-RECURSIVE-PHYSICALIZER-P0` | verified physical input + canonical CFG + function Binding SSA/PhiTxn -> disconnected candidate + root After capability | child PHI, post-child outer backedge, shadow, late-failure discard, fresh reuse | no Return, publication, production caller, route-local SSA, or fallback |
| `GENERIC-G0-COMPLETION-P0` | physical result + verified After effect -> existing explicit-return completion/DraftSeal | returned value is `L0.After/b1`; input/body temporaries reject | no direct Return writer and no LoopRecipe function tail |

### Coverage, cutover, and retirement

| Row | One acceptance claim | Mandatory closeout |
| --- | --- | --- |
| `JOINIR-LOOP-M8-*-S6A..S6E` | one producer cohort per stable row: LoopV0 recurrence; exits/joins; scans; LoopCond exits; Generic residual | each row emits its common Recipe/core product and typed source-policy observation; no new selector/verifier/CFG/PHI/physicalizer |
| `JOINIR-LOOP-M8-ALL19-CLOSEOUT-S6G` | all M8 observations -> `VerifiedLoopAllRouteObservationSetV1` + `WholeUnitLoopCoverageProofV1` -> the same `CanonicalLoopFamilySelectionV1` | all 19 routes are typed pre-effect decline or verified Recipe; accepted corpus parity and unclassified count zero; this row alone opens NoCandidate | implementation-coupled closeout, not docs-only; no second selector, route-ID/cursor selection, production caller, Option/retry/fallback |
| `SELFHOST-LOOP-PORTABLE-WIRE-S7A`, `SELFHOST-LOOP-M8*-PARITY-S7B1..S7B5`, `SELFHOST-LOOP-PORTABLE-ALL19-PARITY-S7G` | `.hako` portable wire, one stable producer cohort per row, then all19 normalized parity | selfhost quick, identity, no-hostbridge; no `.hako` physical/default claim |
| `GENERIC-M10B-DELETION-MANIFEST-S0` | freeze exact scheduler/debt/registry/composer/nested-bypass symbols and callers | checked manifest immediately precedes cutover; historical names are not authority |
| `M10b-I0-R0` | one `route_loop` caller -> one selected verified product -> one physicalizer -> one external commit | same commit deletes ordered retry, Generic debt/error-to-None, V0/V1 rows, selected mutating composer/body callers, nested `.ok()` and direct nested/located mutation bypasses; shared located source handoff remains for M11 |
| `GENERIC-LEGACY-DEAD-CODE-R1` | delete caller-zero Generic-only facts/extractors/composers/adapters/files | keep portable parity, counterexamples, and retirement manifest; do not wholesale-delete shared `generic_loop_canon` users |
| `M11-R1` | located source enters the same structural/Recipe path | delete covered source-erasing handoff and R4 shadows; no ingress widening |
| `JOINIR-LOOP-LEGACY-DISPOSITION-R2A` -> `JOINIR-LOOP-DUPLICATE-FACADE-RETIRE-R2B` -> `JOINIR-LOOP-MUTATION-DISPATCH-RETIRE-R2C` -> `JOINIR-LOOP-SOLE-AUTHORITY-CLOSEOUT-R2G` | classify `11/1/1/4/2`; delete duplicate facades; delete mutation-family dispatch; prove sole authorities | final counts: verifier/JoinSig/CFG/Binding-SSA/PhiTxn/physicalizer each one; family physical branches, retry, AST rematch, and adapter dispatch zero |

## Module homes and size boundary

```text
src/mir/compiler/generic_g0_projection/
  thin source navigation plus move-only S0A/S0B/S0C aggregate wrappers;
  no structure/type/numeric/policy decision

src/mir/loop_structural_facts/generic_g0/
  sole S0A structure verifier/issuer; VerifiedGenericStructuralFactsG0 only

src/mir/resolved_semantics/generic_g0/
  sole S0B source-type verifier/issuer; owner/site/BindingRef inventory only

src/mir/numeric_substrate/generic_g0/
  sole S0C representation/range/overflow verifier/issuer; typed lease only

src/mir/loop_route_policy/generic_g0/
  Less/positive-Add policy and move-only Generic candidate observation

src/mir/loop_route_policy/family_selection/
  four competing profile rows, Generic row, G0 admission window, sole G0 selector

src/mir/loop_recipe_contract/join_sig/
  model, visible payload, logical port binding, flow

src/mir/loop_recipe_contract/
  binding/effect relation, source-bound core verifier/product, producer ID

src/mir/loop_recipe_contract/generic_g0/
  selected-demand boundary, sole dense-key issuer, relations, After, aggregate, physical input

src/mir/builder/resolved_lowering/loop_recipe/
  topology, binding port, operation, recursive control, tail handoff, orchestration
```

Do not productionize or extend the old cfg(test) Generic carrier witnesses.
Do not append new authority to `join_sig.rs` (currently 731 lines), `verify.rs`
(716), or `loop_accum_physicalizer.rs` (792). Production files target 150-450
lines, orchestration/verifier files stay below 600 where practical, and every
source/check file remains at most 799 lines. Tests live beside their owner in
separate files. No new per-row shell guard is created; extend the shared
MirBuilder replacement manifest/guard.

## Checked legacy manifest

The implementation creates and preserves
`docs/development/current/main/design/fixtures/generic-loop-legacy-disposition-v1.tsv`.
It has one union
schema whose first column is `record_kind = case | edge`; there is no second
parser or implicit column shape. Case records carry case ID, canonical fixture,
corpus, mode, observation state, current acceptance, observed route,
nested-bypass flag, source surface, disposition, target owner, accepted
Decision, parity gate, and retention row. Edge records carry edge ID, path,
symbol, current role, production/test caller counts, first effect, cutover
action, retire row, and replacement owner. Nonapplicable columns use one
documented sentinel rather than an empty-field convention.

Allowed case dispositions are only:

```text
portable-owner
accepted-typed-reject
nonproduction-future-evidence
```

Observation state is exactly `unobserved | accepted | rejected |
failed-before-loop | timeout`. Cross-field validation is strict:

```text
current acceptance = accepted
  -> portable-owner | accepted-typed-reject only

nonproduction-future-evidence
  -> current acceptance != accepted

unobserved | failed-before-loop | timeout
  -> disposition cannot close and M10b remains blocked
```

The shared `mirbuilder_inplace_replacement_guard.sh` validates totality,
case/mode uniqueness, compatibility aliases, pre-cutover symbol existence,
post-cutover caller/deletion zero, accepted-case classification, and the line
cap. Missing paths or symbols fail; a filename containing `generic` is never a
selection or disposition fact.

## Gate profile

Ordinary rows run the owning focused tests first, then `git diff --check`,
`current_state_pointer_guard.sh`, the shared MirBuilder replacement guard, and
the relevant `generic_g0` / `loop_recipe_contract` library filters. P0 physical
rows and S6G/S7G/M10b/M11/R2G closeouts additionally run release build and
`dev_gate.sh quick`. The milestone closeouts add all19, phase29bq,
selfhost-parity, backend-parity, fault-injection, and fresh-reuse gates named by
their row. Repository-wide runtime census stays serial and starts with one
case, then a small sample, then the displayed full count.

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
workstream, and its owning README. Every S0A/S0B/S0C/S1/S2/S3/S4 landing also
updates `docs/reference/mir/generic-loop-stage-matrix.md` with its exact
caller-zero state and non-claims. A portable Recipe/JoinSig/core contract row
updates `docs/reference/mir/loop-recipe-contract.md` in the same commit even
while its production caller is zero.

Physical preflight, common-owner refactor, recursive physicalizer, and
completion rows update the loop Recipe reference and the exact applicable
`phi_policy.md` / `phi_invariants.md` caller-zero or behavior-neutral boundary
in their implementation commits; they must not wait for M10b to document what
has actually landed.

M10b updates the implemented default behavior in those references plus
`docs/reference/mir/phi_policy.md`, `docs/reference/mir/phi_invariants.md`, and
any affected diagnostic/environment reference in the same atomic commit. R1,
M11, and M12 remove stale legacy claims in their own implementation commits.
The final closeout audits grammar, diagnostics, default behavior, backend
parity, and legacy caller zero. Reference text never claims implementation
before its code row lands.

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
