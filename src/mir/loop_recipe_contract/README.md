# Portable Loop Recipe Contract

This directory owns the Builder-free, selfhost-portable semantic contract for
Loop lowering.

## Authority

- `LoopRecipeArtifactV1` owns schema version, a required source wire claim,
  `LoopRecipeProducerIdV1` receipt, and one `LoopRecipeV1`.
- The source wire claim names one declared-function body by compilation-unit
  and function ordinals, then maps the ordered Loop arena exactly 1:1 to unique
  typed paths.
- `LoopRecipeV1` owns one closed recursive control algebra represented on the
  wire as ordered arenas with recipe-local keys. It contains no source or route
  authority.
- `LoopBindingKeyV1` is issued only by the Recipe producer. Resolver/neutral
  facts and Binding SSA must not mint it. The producer co-seals each key with
  its source-backed `BindingRefV1` relation before a canonical plan can be
  verified.
- The semantic Loop node is the single long-term loop shape: `Always` is a
  degenerate predicate, and a loop with no explicit `break`/`continue`/`return`
  is the same frame with fewer exit rows. Nested loops recurse through the
  same `LoopRecipeItemV1::Loop` node; they are not a second semantic family.
  `LoopV0`, `LoopTrue`, and `LoopCond` names belong to current producers or
  legacy physical adapters, not to the portable semantic SSOT.
- The legacy count of 19 is an ingress/coverage count only. All accepted rows
  normalize into this same recursive algebra; they must not create 19 Recipe
  variants, completed-source-pattern enums, verifier branches, or
  physicalizers. M7/M8 are adapter-coverage migrations, not Recipe-kind growth.
- External/pre-loop values are named explicitly by `inputs`; every other value
  has exactly one operation result.
- A carrier entry must be available before its owning Loop is entered. The
  caller-zero `LoopJoinSigElaboratorV1` elaborates bounded Accum edges,
  visible ancestor-carrier payloads, the accepted M7-S2-A explicit-else
  branch row, and the shared M7-S2-B one-sided exit/fallthrough row. A branch
  row has two typed arm dispositions: `Exit` or `Fallthrough`. An omitted
  source `else` is represented by logical `Fallthrough`; no source node or AST
  rewrite is synthesized. Terminal arms retain their own visible payload and
  do not participate in a binding merge. The verified product is non-`Clone`
  and has no production caller. Full dominance/predecessor, PHI materialization,
  and wider nested-exit closure remain later slices.
- `LoopRecipeVerifierV1` consumes only `LoopRecipeV1`. It cannot select or retry
  a route and cannot inspect source ownership or the producer receipt.
- `LoopRecipeVerifierV1` owns structural recipe preconditions; the JoinSig
  elaborator owns logical dataflow/edge rows. Do not merge these authorities.
- Artifact verification proves only the wire claim's internal structure:
  canonical source-key order, exact coverage, unique paths, root entry through
  `body_item`, and direct-child path grammar. It does not prove that the named
  function or AST sites exist, nor that they produced this recipe.
- `StructurallyVerifiedLoopRecipeSourceClaimV1` is therefore an internal,
  non-`Clone` validation capability. Its wire DTO remains intentionally
  `Clone`; neither type is source authority.

## Current M8 boundary

G0 I1 is a closed caller-zero physical canary. The S6A variable-accum design is
accepted: one atomic AST-free Facts product will project two external inputs,
two Recipe bindings, eight Core binding effects, and eleven item-source rows
into the existing Recipe/JoinSig/Core owners. The selected
`LOOP-INPUT-SOURCE-RELATION-SET-R0` is now landed: callable's singular
initialized-local input relation is a one-row instance of the common
exact-coverage `VerifiedLoopInitializedLocalInputSourceSetV1`; Generic
parameter inputs stay separate. S6A source observation and its
provenance-only producer are closed as a caller-zero row, including the normal
`Main.main` resolver ingress, typed C/D/U/R envelope, duplicate-role rejection,
and source-site coherence negatives; no selector or physical consumer is
opened.

The common set co-seals declaration (including its statement site), initializer,
`BindingRefV1`, Recipe input value, and class against the already sealed Recipe
carrier and Core binding relation. It exposes the complete row slice only; no
first/filter/ordinal reconstruction API exists. Empty, duplicate, foreign-owner,
missing-carrier, class, declaration, and binding mismatches fail before Builder
effects.

S6A keeps the same one-owner rule for the recurrence source map. Private
condition/update/step observations never become separate verified products;
`VerifiedVariableAccumRecurrenceFactsV1` is the only neutral handoff. Its
producer consumes a Candidate once and projects exactly 2 input, 2 binding, 8
Core-effect, and 11 item-source relations into the existing owners. Reads,
constants, binary expressions, assignment targets, carrier entries, and
initializer declarations use their exact owned source anchors; Recipe ordinals
are not source lookup keys. Duplicate roles and incoherent source sites fail
before the aggregate is issued, while missing evidence remains Unresolved.
`NoSafeSlice` is a development stop, not a source disposition, and no S6A
selector or physicalizer is opened here.

The producer is `variable_accum_recurrence_producer.rs`. It consumes one Facts
aggregate and seals the existing Recipe/JoinSig/Core/input/effect owners with
`2` input, `2` binding, `8` Core-effect, and `11` item-source rows. It accepts
exact `I64` bound/delta literals from Facts and does not inspect AST or
reclassify the source family.

The audited `loop_simple_while_inline_explicit_step_min.hako` fixture remains
Declined by the existing SimpleWhile, DirectAccum, and Generic G0 observers.
No route relabel, DirectAccum widening, new Recipe kind, selector/physicalizer,
Builder/MIR effect, M9 parity, production route, retry/fallback, or legacy
deletion is open. The S6A closeout updates this README and the matching
`docs/reference/**` receipt; M10b cutover requires a final update.

## M8 S6B variable-accum break/fallthrough producer

The caller-zero `VariableAccumBreakV1` producer consumes one atomic Facts
candidate for `apps/tests/loop_break_plan_subset_min.hako` and deterministically
projects it into the existing `LoopRecipeV1`, `LoopJoinSigV1`, source-bound
Core, initialized-local input set, and operation/effect product. It does not
inspect AST, select a route, or create physical identities. The normalized
Recipe keeps the conditional terminal arm separate from the implicit normal
fallthrough arm; the terminal payload is not merged with the normal update.

The sealed receipt is intentionally explicit: 2 Recipe bindings, 2 external
inputs, 3 logical blocks, 20 item rows, 17 values, 2 carriers, and 1 break
exit. The source-bound relation counts are 2 inputs, 2 bindings, 10 Core
effects, and 18 operation-source rows. The branch `If` and `Break` source
anchors are retained in `VariableAccumBreakControlSourceReceiptV1`; they are
not forced into the operation/effect table. Producer provenance is
`variable_accum_break_v1` and is diagnostic only.

Focused positive/negative tests cover exact source projection, deterministic
Recipe/JoinSig/Core sealing, independent `Exit`/`Fallthrough` arms, incomplete
coverage (`Unresolved`), unsupported/explicit-else shapes (`Declined`), and
foreign owner (`Rejected`). S6B remains Builder-free and caller-zero: no
physicalizer, completion, selector, retry/fallback, production caller, or
legacy retirement is opened.

## M8 S6C scans design stop

Scan source is not yet a portable Recipe cohort. `LoopRecipeV1` currently has
only numeric operations and `I64`/`Bool`/`Unit` values, while the smallest
forward `ScanWithInit` shape requires typed text values, resolver-bound calls,
text comparison, and a distinct callable tail. The legacy scan composers are
AST-reconstructing compatibility owners and are not reusable source authority.

The typed call/value architecture is now accepted, but implementation starts
with the behavior-neutral `LOOP-RECIPE-OPERATION-SHAPE-SPLIT-R0` row because
`operation_physical_demand.rs` is 781 lines and `verify.rs` is 725 lines. No
schema growth is allowed until the responsibility split leaves every changed
Rust source file below the 760-line design trigger. This is BoxShape-only and
must not add an operation, value class, fixture, selector, or physical route.

The later typed cohort uses an explicit `LoopRecipeV2` boundary with logical
`I64|Bool|Unit|Text`, a Recipe-local CallSlot, and `TextEq(Text, Text) -> Bool`.
The Recipe contains no method/Box names, MIR IDs, resolver capability, Home,
effect, ABI, or runtime lookup. A separate resolver-issued instance-call
target and source-bound call relation own those facts. `Handle`, `Any`,
`Opaque`, Array/collection, and nominal Box values are not admitted. Implement
`ScanWithInit` first after these rows; keep `SplitScan`, `CharMap`, `ArrayJoin`,
and `BoolPredicateScan` separate. `NoSafeSlice` is a development state, not a
source disposition. Every landed typed schema/observer/producer commit must
update `docs/reference/mir/loop-recipe-contract.md`, this README, and the
source Facts README with its focused tests.

The V2 `Dynamic` value extension is landed as a caller-zero structural wire
receipt. It is never selector-refined to `Text`/`I64` and never
means runtime `Unknown`, Home, or ABI. One external item/source relation must
bind each Dynamic CallSlot to the exact borrowed target/envelope. The wire
does not store that target or envelope.

The unchanged module envelope catalog contains seven Dynamic rows: three are
Loop-owned and four are valid non-Loop calls. The first bounded
`skip_while/4` relation selects two and retains all seven. Missing membership
authority widens the compiler rather than narrowing the source. Fault stays
outside Recipe values, exits, JoinSig, Completion, and Return.

`LOOP-V2-OPERAND-DEFINITION-GUARD-R0` is landed. One common verifier path now
rejects known-but-not-yet-defined CallSlot, numeric, TextEq, WriteBinding, If,
and Return operands without adding an accepted source shape. The later order
is Dynamic value I0, source-value
relation, CallSlot/envelope co-seal, explicit Dynamic operations, exact local
source/scope closure, V2 JoinSig/Fault authority, and only then neutral local
Home Flow. V1 remains unchanged.

The exact local source/scope closure is now landed. The existing Dynamic source
issuer proves the iteration-local declaration belongs to the sealed Loop-body
scope and that its resolver use set is exactly one I7 read with no rebind or
nested capture. The atomic envelope exposes only a borrowed V10/I6/I7 view;
it adds no Recipe binding/carrier, Home state, cleanup plan, or physical port.
The next design boundary is V2 Dynamic JoinSig and its separation from Fault.

The focused golden covers type-consistent Dynamic input, binding, carrier,
CallSlot normal result, WriteBinding, and Return domains. It does not issue the
external source-value or target/envelope relation. That relation is the next
design stop and must retain all seven module envelope rows while selecting
only exact callable/Loop source sites.

The source-value relation premise audit is `NoSafeSlice` as an executable row:
there is no complete V2 Recipe producer, and the unchanged source still needs
Dynamic Add/Less. A two-CallSlot key product would be partial truth. The order
is corrected to Dynamic operations, full unchanged-source V2 producer, then a
single atomic source/Recipe/envelope co-seal. The per-Recipe co-seal will
borrow—not consume—the complete seven-row module envelope catalog.

The Dynamic operation D0 and schema/verifier I0 are landed. V2 adds only
`DynamicAdd` and `DynamicLess`:

```text
DynamicAdd:  Dynamic x I64 -> Dynamic
DynamicLess: Dynamic x Dynamic -> Bool
             Dynamic x I64     -> Bool
```

Existing `ConstI64` rows own the exact `1`, `0`, and second `1`; no
`ConstDynamic`, embedded delta, selector refinement, or reuse of one literal
source anchor is allowed. The unchanged source has two Add and two Less rows.
The older profile-specific P1 proof owns only the root Less and step Add and
must not be promoted as complete authority. Normal results follow the language
operator contract; TypeError/Fault defines no Recipe value or lexical control
edge. I0 changes only V2 schema/verifier/tests and leaves V1, source co-seal,
physical lowering, provider/runtime execution, and production selection
closed.

The focused golden contains two Add, two Less, and three distinct
`ConstI64(1/0/1)` rows; substring Add is temporary, inner Less consumes the
prior CallSlot result, and only StepAdd feeds `WriteBinding`. Eight focused
Dynamic-operation tests and all twenty-six V2 schema tests pass. The next row
was a design stop for one complete unchanged-source V2 producer; no partial
call-only producer or durable source-to-key product may be opened.

That top-down audit found a compiler-side BoxShape gap, not a source gap. The
complete source is expressible as one Loop, three blocks, one induction
binding, four inputs, one carrier, eighteen values, seventeen items, one
inner Return, and two CallSlots. However, V2 does not yet seal V1-equivalent
root/block ownership, exact block/Loop/Exit use, recursive preorder, terminal
Exit, or artifact source-binding structure. `LOOP-V2-CONTROL-STRUCTURE-GUARD-R0`
must restore those neutral checks before the full candidate producer. The
fixture remains unchanged; no new accepted operation or source shape is part
of R0.

The later producer retains `ch` as a source-local value relation over the
substring CallSlot result rather than inventing a Recipe binding/carrier. The
inner Return is the sole Recipe Exit; the outer Return remains Callable Tail,
and the existing two-site Completion stays intact for a later exact
partition. The detailed mapping and negative matrix are owned by
`loop-v2-dynamic-full-producer-task-2026-08-10.md`.

`LOOP-V2-CONTROL-STRUCTURE-GUARD-R0` is now landed. V2 uses a separate
`typed_schema_v2_structure.rs` owner for root/parent, block/If ownership,
exact block/Loop/Exit use, terminal Exit, target ancestry, and recursive
preorder. Artifact verification also consumes the common structural
source-path verifier and stores a non-Clone verified source claim. The
resolved-source adapter can bind its non-Clone Loop root directly to a
verified V2 root; callers cannot provide a key or coordinates. R0 is
BoxShape-only and adds no accepted source shape. The complete producer I0 is
now open.

## Generic G0 S4 producer

`generic_g0/` owns the caller-zero S4 aggregate producer. It consumes one
`VerifiedGenericRecipeDemandG0`, binds the resolved source forest exactly once,
and emits one deterministic `GenericG0` Recipe artifact plus the common
source-bound Core and Generic After envelope. The exact portable mapping is
the SSOT in `docs/development/current/main/design/generic-loop-source-to-portable-recipe-ssot.md`:
two bindings, two nested loops, fifteen values, three carrier rows, and ten
source/effect relations. This subtree is `cfg(test)` only until a later,
explicit production-caller row; it has no Builder/MIR, physical, completion,
retry/fallback, or legacy-deletion authority. The implementation receipt is
in `docs/reference/mir/generic-loop-stage-matrix.md`.

## Callable single-loop co-seal (caller-zero)

The selected callable single-loop profile is not a second Recipe family. Its
`cfg(test)` producer consumes the resolver/MAP source product once, builds the
same recursive `LoopRecipeV1`, and delegates structural verification, JoinSig,
and source-bound Core sealing to the common owners. It publishes the common
`VerifiedLoopRecipeCoSealV1` together with separate callable Prelude/Tail
source contracts. The Tail retains its exact terminal statement site and
prefix binding; it cannot fuse with the logical Loop After binding.

This row owns no Builder/MIR, physical IDs, ABI/Completion, selector,
physicalizer, retry, fallback, production route, or legacy deletion. The
test-only producer id `callable_single_loop_v1` is provenance only and is not a
legacy route alias. Physical preparation remains a later explicit row.

## Topology physicalizer boundary (caller-zero)

The bounded P0 canary adds a neutral move-only
`VerifiedLoopPhysicalBoundaryV1` projection for the existing Core and logical
After capability. It is consumed only by the test-only topology physicalizer;
it does not create a Recipe/JoinSig/CFG/SSA/PHI owner or expose callable
Tail/ABI/Completion. The current operation row is the passive
`LOOP-RECIPE-OPERATION-EFFECT-S0` product. It uses item-keyed source evidence
and exact Core effect relations because effect ordinals alone are ambiguous
across nested loops. The product emits no operation MIR or production caller.

### Operation/effect product S0

`VerifiedLoopOperationEffectProductV1` is the sole passive owner for the
operation/effect join after D0. It moves `VerifiedLoopCoreProductV1` exactly
once and stores only profile-issued item/anchor evidence. Recipe operation
kind, operands, binding relations, and effect relations are read through the
moved Core; no parallel catalog is allowed. Every Recipe `Operation` item is
covered exactly once, with exact block/loop placement and owner-branded source
anchors. `ReadBinding`/`WriteBinding` require the matching Core relation;
pure operations reject fabricated binding evidence.

The focused nested fixture covers 19 operation items plus duplicate, missing,
foreign-owner, wrong-placement, and pure-operation binding-evidence rejects.
This is still a caller-zero contract cell: no Builder/MIR, selector, retry,
fallback, Return, DraftSeal, or production route is opened.

### ReadBinding leaf-emitter I0

The complete prepared operation program now projects all `ReadBinding` rows
without exposing a first/select/filter extraction API. Each row retains the
Recipe operation, `Expr(OwnedExprSiteV1)` source, `SourceRead` effect anchor,
binding, owner, class, and logical placement. `DerivedCarrierEntry` is not a
read leaf and is rejected as an unavailable carrier seed. This contract layer
remains Builder-free; canonical BindingSSA/PHI claims and physical value
receipts belong only to the private lowering leaf.

The bounded leaf uses an explicit `PreheaderSeed` or `CanonicalLive` entry
requirement and returns immutable logical/physical placement evidence. Typed
validation happens before claim; any claim/read/type/receipt failure is a
terminal unpublished-function failure owned by the outer session transaction.
The production replacement row, other operations, carrier seeds, full Loop
physicalization, and legacy route retirement remain closed.

The callable adapter row is now closed separately. It consumes the callable
source relations once, checks the transient operation view against the sealed
Recipe, derives Core placement/effect evidence, and issues this neutral
product while retaining Prelude/Tail/context in a thin profile wrapper. The
Generic G0 anchor row is now also closed. The producer issues the explicit
15-row item-to-anchor ledger before source facts leave its boundary; item 3 is
the existing child-entry `DerivedCarrierEntry` for carrier 2. Item 4, C0/C1
carriers, and Generic tail reads remain outside this product.

Cross-profile parity is closed as a diagnostic-only receipt. Callable has seven
rows and Generic G0 has fifteen, but the receipt compares neither counts nor
source order; the common verifier owns the shared reject family. Reviewed
Decision B closes with one complete private move-only
`VerifiedLoopOperationPhysicalDemandV1` bundling the moved resolver semantic
context, operation/effect evidence, common continuation, and a key-only private
index. Its `prepare_all` implementation derives the complete Recipe-order
schedule for Callable's seven rows and Generic G0's fifteen rows with zero
Builder/MIR effect. It exposes no first/select/filter operation API. Leaf
emission and production operation activation remain closed.

The normal callable prepared-ingress row now consumes this full demand through
one `PreparedCallableLoopIngressV1::prepare_full_demand` entry. It checks the
source/context, input, Prelude, and Tail owner relation once, retains the
callable transport in a thin profile wrapper, and leaves the common program as
the sole Recipe-order/coverage owner. This is still Builder-free; physical
session opening, ABI/Completion handoff, selector, Generic G0 parity, and
legacy retirement remain later rows.

## Recursive segment plan R1

`PreparedLoopOperationProgramV1::prepare_physical_layout` is the closed R1
Builder-free boundary. It consumes the complete Recipe-order program and
derives `PreparedLoopPhysicalLayoutV1`; it never selects one operation or
reinterprets source. The layout records exact item coverage, segment-local
operation order, and nested child-entry -> parent-resume transfers. For the
G0 counterexample, root B1 is split into pre-child and resume segments, so the
derived order is `[0,1,2,3,5,6,7,8,9,10,11,12,13,14,15]` with five segments.

R1 remains contract-only: no Builder/MIR IDs, CFG/SSA/PHI mutation, physical
After writer, G0 emission, selector, retry/fallback, or legacy route is
opened. R2 is the next segment-aware canonical block cutover and must update
this README and `docs/reference/**` in the same implementation commit.

The neutral `VerifiedLoopSemanticContextV1` and
`VerifiedLoopContinuationContractV1` wrappers are transport-only. Callable and
Generic G0 move existing resolver/JoinSig evidence into them exactly once; no
After reissue, clone, source-name lookup, or route-local context is allowed.

The accepted V2 Dynamic design does not copy this V1 flow owner. V1 and V2
provide private borrowed Recipe views to one common JoinSig engine, while
their typed seals preserve each exact schema value class. V2 adds no
Dynamic-profile transfer owner: its only new logical case is a one-sided If
whose terminal arm targets `FunctionExit` and whose other arm falls through.
The exact source/Recipe aggregate later invokes the engine and requests After
internally; callers never pass JoinSig or Continuation into that co-seal.
`V10/ch`, Dynamic Fault, Callable Tail, Completion, Home, and physical layout
remain outside this module boundary until their separately named rows land.

`LOOP-JOINSIG-NEUTRAL-ENGINE-R0` is landed. The existing V1 elaborator now
projects its verified Recipe through `join_sig/recipe_view.rs` into one common
class-generic flow engine. The view is borrowed, unpublished, non-serialized,
and exhaustive over V1 operations/items/exits. Existing V1 aliases, normalized
edge/branch/payload/port order, accepted shapes, After capabilities, and branch
Return rejection are unchanged. No V2 adapter, Dynamic class, FunctionExit
branch target, source co-seal, or physical dependency is present in this row.
Focused evidence is 31 passing `join_sig` tests and a green library check; the
largest touched source file is 634 lines.

## Generic legacy corpus inventory

The pre-production legacy inventory is intentionally outside this portable
contract. `generic-loop-legacy-disposition-v1.tsv` is a checked, 25-column
case/edge union consumed only by the shared replacement guard; its 389 P0 case
records are unobserved future evidence. This directory must not select a route,
interpret fixture names, or open a production Recipe caller from that manifest.
The next observation-front task is tracked in
`docs/development/current/main/investigations/generic-legacy-observation-front-g0-task-2026-08-07.md`.

The G0 receipt is separate from this portable contract:
`docs/development/current/main/design/fixtures/generic-legacy-observation-front-g0-v1.json`.
It records one direct VM invocation only. The current receipt is a named
pre-Loop failure in the `raw_expression_dispatch/mod.rs` BinaryOp arm while
lowering the prelude `StringifyOperator.apply/1` second `if` condition; it does not
open a Generic route, Recipe caller, Builder/MIR path, or disposition.

## JoinSig module map

The `join_sig/` directory is the single logical JoinSig owner. Its facade keeps
the historical `join_sig::*` API stable while the child modules split
responsibilities without adding a semantic route:

| Module | Responsibility | Non-authority |
| --- | --- | --- |
| `join_sig/mod.rs` | module declarations and compatibility re-exports | no construction or elaboration policy |
| `join_sig/recipe_view.rs` | private borrowed schema projection into the one flow engine | no stored Recipe, public Plan, source admission, or physical meaning |
| `join_sig/model.rs` | logical ports, edges, payloads, branch rows, rejection algebra, opaque verified wrapper | no raw constructor outside the facade/issuer boundary |
| `join_sig/port.rs` | logical exit-to-port edge projection | no physical CFG/PHI or MIR IDs |
| `join_sig/visibility.rs` | carrier seeding and visible payload projection | no source/AST inspection or route choice |
| `join_sig/flow.rs` | the sole logical elaborator and recursive dataflow owner | no Builder, physical lowering, retry, or publication |
| `join_sig_branch.rs` | existing direct branch-row helper | no second exit-edge owner |

### Nested carrier shadow

`visible_payloads` projects one target loop's visible carriers from the
verified Recipe parent chain. It walks from the target toward the root and
keeps the first `LoopBindingKeyV1` it sees, so the innermost recurrence carrier
shadows an ancestor carrier. It then emits exactly one row per binding in
binding-key order, using the current logical binding-to-value map.

Sibling carriers are outside the target lineage and remain invisible. Unknown
loop owners and duplicate carriers are rejected by `LoopRecipeVerifierV1`
before this projection; JoinSig does not reclassify them. The rule is common
to every nested Recipe and has no Generic, source-name, After, PHI, physical-ID,
or selector special case. The focused contract tests live in
`join_sig_nested_shadow_tests.rs`.

### Header/After binding identity

After all loop rows are elaborated, `join_sig/port.rs` compares every incoming
edge for `Header` and `After`. Each port must have the same duplicate-free
binding set on all of its incoming edges, with one consistent value class per
binding. The resulting `LoopJoinPortBindingV1` rows are sorted by
`(loop_key, port, binding)` and deliberately omit edge values.

`VerifiedLoopJoinSigV1::require_after_binding` is the sole issuer of the
opaque, non-`Clone` `VerifiedLoopAfterBindingV1` capability. A loop without an
incoming `After` edge is valid but cannot issue that capability. Wrong
owner/binding, expected-class mismatch, duplicate payload, set mismatch, and
class mismatch are typed rejects; source `BindingRef`, PHI, `ValueId`, Return,
Generic selection, and physical lowering remain outside this product.

This row is a behavior-neutral structural split. The verified JoinSig wrapper
is constructed only by the elaborator; callers continue to use the facade.

## Current Generic design stop

The proposed G0 target is documented in
`docs/development/current/main/design/generic-loop-source-to-portable-recipe-ssot.md`.
It exposes a common-contract gap, not a Generic exception: one source binding
may be a carrier of both an ancestor and a child Loop. Before the Generic
producer exists, JoinSig must shadow visible carrier payloads by binding with
the innermost owner winning; the common logical Header/After identity product
is now closed. The source-bound verified core must still retain the opaque
source claim plus exact Recipe-key/`BindingRef`/effect relations.

### Source-bound core S0

`source_bound_core.rs` is the common caller-zero co-seal boundary. It consumes
an already verified Recipe artifact, an already verified JoinSig, one resolver
`FunctionOwnerIdV1`, and unsealed binding/effect relation DTOs. Its sole issuer
checks exact Recipe-key coverage, one-to-one source `BindingRefV1` ownership and
value class, source-only declaration provenance, typed effect roles, and the
JoinSig/Recipe pair before returning one move-only `VerifiedLoopCoreProductV1`.

`DerivedCarrierEntry` is anchored by a typed loop statement path plus the
Recipe-local `LoopCarrierKeyV1`; labels, fabricated expression sites, and
source-name lookup are not accepted. The product exposes no AST, selector,
Generic key issuance, Builder/MIR, physical identity, retry, or production
caller. Real Generic relation instances remain the sole responsibility of the
future S4 producer.

`LoopRecipeProvenanceV1` now carries `producer_id: LoopRecipeProducerIdV1`.
The old `producer_route` JSON key is rejected; schema V1 has no compatibility
alias because this contract is still caller-zero and pre-production. The
legacy scheduler/policy/registry keeps `LoopRouteId`, while a test-only
`LegacyRouteParityReceiptV1` records the three current profile mappings and
the legacy-only Generic V0/V1 rows. The portable schema and producers import
no `LoopRouteId`, and no selector, registry, verifier, normalizer, or
physicalizer dispatches on producer ID. There is still no Generic Recipe
producer or production consumer, and a post-loop function tail must not be
inserted into `LoopRecipeV1`.

## LoopTrue S2 producer

`produce_loop_true_break_continue_recipe_v1` is the caller-zero S2 producer
for the sealed `LoopTrueBreakContinue` policy brand. It consumes one
`VerifiedLoopTrueBreakContinuePolicyDemandV1`, retains its policy receipt, and
projects the sealed source shape into the existing envelope:

```text
policy demand
  -> fixed LoopTrue RecipeV1
  -> source-bound artifact verification
  -> VerifiedLoopJoinSigV1
```

The exact envelope is one `Always` loop with three blocks, one I64 binding and
carrier, four values, one `ReadBinding`, one bound `ConstI64`, one `Equal`
comparison, one explicit-else `If`, and direct owner-targeted `Break`/`Continue`
exits. The producer is deterministic and has no AST inspection, route switch,
retry/fallback, physical CFG/PHI, or Builder effect. The result is a verified
logical product only; it does not claim a production caller or physical
adoption.

## Loop JoinSig mixed-fallthrough S2-B

The shared `LoopJoinSigElaboratorV1` now records conditional arms with one
stable shape:

```text
LoopJoinBranchV1
  owner_loop
  if_item
  condition
  then_arm: Exit(LoopJoinBranchExitV1) | Fallthrough { payload }
  else_arm: Exit(LoopJoinBranchExitV1) | Fallthrough { payload }
```

An omitted source `else` is an implicit logical `Fallthrough`; no AST node or
synthetic source branch is created. A terminal arm captures the visible
payload at its own exit site. A terminal arm and a normal arm therefore do not
merge binding state. The normal arm continues with its state, while the loop
row receives the terminal `Break`/`Continue` edge and a normal `Backedge`.
Two normal arms must have equal binding/value state and are rejected otherwise.
The existing explicit `Break`-then/`Continue`-else pair remains valid.

This is a caller-zero logical JoinSig contract only. `LoopRecipeV1` is
unchanged; source observation, Recipe production, physical CFG/PHI, Builder,
selector, retry/fallback, production activation, and legacy deletion remain
outside this row. The exact implementation receipt is in
`docs/reference/mir/loop-recipe-contract.md` under
`LOOP-JOINSIG-MIXED-FALLTHROUGH-D0`.

## Typed Dynamic JoinSig V2 receipt

`LOOP-RECIPE-V2-JOINSIG-DYNAMIC-I0` connects the complete verified V2 Recipe
to the same private JoinSig flow engine through a borrowed V2 adapter. The
adapter exhaustively projects every V2 operation's def/use relation and keeps
`LoopValueClassV2::Dynamic`; it never converts V2 into V1 or walks control a
second time.

Branch targets are a separate typed family from value classes. V1 retains
Loop-only targets and still rejects a Return arm. V2 uses
`LoopJoinBranchExitTargetV2::{Loop, FunctionExit}` and accepts only
Break/Continue-to-Loop or Return-to-FunctionExit. The unchanged Dynamic Recipe
seals exactly five edges, one Return/fallthrough branch, and Header/After
bindings for `B0:Dynamic`. Carrier-derived payloads contain `V1` on entry and
Return, `V17` on backedge, and never contain the iteration-local `V10/ch` or
the Return operand `V14`. The V2 Recipe verifier also rejects either body-local
value as the root carrier entry, so the exclusion is sealed before JoinSig
issuance rather than maintained by convention.

This is still caller-zero logical evidence. Source/Recipe/JoinSig/After
co-seal, Continuation, Completion consumption, Fault/Home, physical layout,
Builder/MIR/CFG/PHI, production selection, retry, and fallback remain outside
this receipt.

## V2 root-carrier Join closure

The semantic-program row removes the split V2 issuance surface. A verified V2
Recipe now enters `issue_sole_root_carrier_join_closure_v2`, which derives its
root and requires exactly one root-owned carrier before invoking the private
V2 adapter and raw After lookup. The result keeps the JoinSig and matching
After in one non-`Clone` `VerifiedLoopJoinClosureV2` with no `into_parts`.

The compiler profile supplies no Loop key, binding key, class, JoinSig, or
After. `LoopJoinSigElaboratorV2` and a raw V2 After alias are not re-exported
through the production facade. This subtree still does not import the Dynamic
compiler profile; the profile consumes the safe combined closure instead.

## Forbidden dependencies

This subtree must not import AST nodes, `MirBuilder`, `CorePlan`, physical
`ValueId`/`BasicBlockId`, `Frag`, route composers, callbacks, retry, or legacy
mutation-family policy.

The control tree is the sole source of connectivity. Logical CFG/JoinSig and
physical MIR are later elaborations; they are not duplicated in this wire
contract.

Arena rows and recursive traversal both use canonical preorder. Artifact source
paths use only the closed steps `body_item`, `scope_body_item`, and
`loop_body_item`. A root path starts with exactly one `body_item`; later steps
may describe outer `scope_body_item`/`loop_body_item` ancestry. A semantic child
is exactly its parent's path plus one `loop_body_item` and zero or more
`scope_body_item` steps. A second `body_item` or `loop_body_item` cannot skip an
intermediate semantic Loop.

Normalization has three deliberate views: full artifact, source-bound
(source + semantics, without route receipt), and semantic-only (without source
or route). Schema V1 is still caller-zero and pre-production, so this is a V1
contract correction with no compatibility adapter or V2 alias.

## Extension rule

Start with the Accum-ready operation vocabulary. Add one typed operation only
when a route migration supplies a counterexample and fixtures. Never add opaque
AST/statement payloads or legacy-emitter escape hatches.

## Post-cutover convergence gate

After the portable producer has one production caller and the canonical
session is the physical lifecycle owner, the remaining family adapters are a
temporary implementation detail. The cleanup target is:

```text
frame producers (LoopV0 / LoopTrue / LoopCond)
  -> one general frame adapter (condition + typed exit rows)
Nested
  -> recursive use of the same frame adapter
Generic
  -> classified and removed; no post-effect retry
```

The gate is semantic and evidence-based, not a rename: all fixtures must have
the same verified Recipe/JoinSig winner, CFG/PHI/value parity, and no legacy
family production caller. M7-S2-A and the shared M7-S2-B contract close only
bounded caller-zero logical branch shapes; physical consumers, PHI material,
and broader nested branch/merge obligations remain explicitly out of scope.
Do not attempt this convergence during D5 caller-zero physical-input work; it
is a post-cutover refactor gate.

## Operation shape split R0

The behavior-neutral R0 split keeps the public `loop_recipe_contract` facade
and all semantic owners unchanged while separating the operation-demand
facade into row types and recursive schedule construction, and separating the
verifier facade from canonical key/lookup helpers. The split is structural;
it adds no Recipe vocabulary, selector, physical ID, Builder effect, or
production caller. Each changed Rust file remains below the 760-line design
trigger (800-line hard boundary).

The focused demand, verifier, and structural tests are green. One existing
`source_bound_core_tests` assertion remains a known baseline red and reproduces
at parent `e00a374803`; it is recorded in the active R0 card and is not repaired
by this BoxShape series. Typed Text/Call schema growth remains a separate
BoxCount row. When that later row changes the language contract, its reference
page and this module README must be updated in the same implementation commit.

## Explicit typed Recipe V2 schema — `LOOP-RECIPE-V2-TYPED-SCHEMA-CALLSLOT-I0`

The typed vocabulary is a separate wire, not an in-place widening of V1.
`schema_v2.rs` owns `LoopRecipeArtifactV2`, logical `Text`, `CallSlot`, and
`TextEq`; `typed_schema_v2.rs` owns only structural verification. A CallSlot
contains recipe-local optional receiver/result keys and ordered argument keys.
It contains no method/Box name, resolver target, Home/effect/ABI contract,
MIR/physical ID, or runtime lookup string. The resolver target and source-bound
call relation are later rows.

The first V2 verifier checks schema version, canonical logical keys, referenced
values, duplicate definitions, numeric domains, and `TextEq(Text, Text) ->
Bool`. Its focused closeout keeps duplicate-definition and wrong-result-class
fixtures independent, so the verifier contract remains unchanged. It performs
no source lookup, Builder/MIR/CFG/PHI lowering, Tail/
Completion handling, scan observation, selector, fallback, or production
activation. `Text` is a logical class only; representation and ownership stay
in the source-bound contract. V1 remains a separate accepted wire and is not
decoded as V2.
