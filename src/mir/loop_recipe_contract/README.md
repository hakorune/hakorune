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
  visible ancestor-carrier payloads, and the accepted M7-S2-A LoopTrue
  explicit-else branch row. The branch row records direct then-`break` and
  else-`continue` exits without creating physical CFG/PHI obligations. Its
  verified product is non-`Clone` and has no production caller. Full
  dominance/predecessor, binding merges, implicit fallthrough, and wider
  nested-exit closure remain later slices.
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
Tail/ABI/Completion. The next operation row must first define an item-keyed,
exact source/effect product because current effect ordinals alone are
ambiguous across nested loops. No operation MIR or production caller is
claimed here.

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
family production caller. M7-S2-A closes only one caller-zero logical branch
shape; physical consumers, binding merges, implicit fallthrough, and broader
branch/merge obligations remain explicitly out of scope. Do not attempt this
convergence during D5 caller-zero physical-input work; it is a post-cutover
refactor gate.
